//! # Cox–Ross–Rubinstein (CRR) binomial tree
//!
//! Discrete multiperiod tree for **European and American** equity-style options
//! under continuous rates and continuous dividend yield (same \((S,K,T,r,q,\sigma)\)
//! language as BSM).
//!
//! ---
//!
//! ## Trading perspective
//!
//! | Question | Tree answer |
//! |----------|-------------|
//! | Fair value with **early exercise**? | American put (and call when \(q>0\)) |
//! | How far from European BSM? | Early-exercise premium = American − European tree |
//! | What IV is the **mid** implying (American)? | [`american_implied_vol`] |
//! | Teaching / audit? | [`crr_solution`] + node table for small \(N\) |
//!
//! European CRR → BSM as \(N\to\infty\) (within model assumptions).
//!
//! ---
//!
//! ## Engineering perspective
//!
//! - Hot path: [`crr_price`] with fixed [`CrrParams`] (steps often 100–500).  
//! - Greeks: tree Δ/Γ from first-step nodes; vega via bump (finite difference).  
//! - Teaching: [`crr_solution::print_table`] prints **layer 0…min(N, max_print)** node values.  
//! - Cost: \(O(N^2)\) nodes — not nanosecond; use closed form for European when valid.  
//! - Risk-neutral \(p^*\) must lie in \([0,1]\); otherwise [`crr_price`] returns
//!   [`FinanceError::Unsolvable`] (do not silently clamp).
//!
//! ## CRR mechanics
//!
//! ```text
//! Δt = T/N
//! u  = exp(σ √Δt),  d = 1/u
//! p* = (e^{(r−q)Δt} − d) / (u − d)     risk-neutral up probability
//! disc = e^{−r Δt}
//!
//! Terminal: max(S u^j d^{N−j} − K, 0)  (call) / put dual
//! Backward: cont = disc [ p* V_up + (1−p*) V_down ]
//! American: V = max(exercise, cont)
//! ```
//!
//! ## Word problem
//!
//! > S=K=100, T=1, r=5%, q=0, σ=20%, N=100. American put vs European put?
//!
//! American ≥ European; early-exercise premium often small but positive for puts.
//!
//! ```
//! use finance_solution::derivatives::{
//!     crr_price, CrrParams, ExerciseStyle, OptionType,
//! };
//! let p = CrrParams::new(100.0, 100.0, 1.0, 0.05, 0.0, 0.20, 100, ExerciseStyle::American);
//! let am = crr_price(p, OptionType::Put).unwrap();
//! let eu = crr_price(p.with_style(ExerciseStyle::European), OptionType::Put).unwrap();
//! assert!(am + 1e-12 >= eu);
//! ```

use crate::derivatives::types::OptionType;
use crate::util::error::{require_finite, FinanceError, FinanceResult};
use crate::{columns_with_strings, print_table_locale_opt};

/// European vs American exercise at each node.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ExerciseStyle {
    European,
    American,
}

impl ExerciseStyle {
    pub fn is_american(self) -> bool {
        matches!(self, ExerciseStyle::American)
    }
}

/// CRR tree inputs (equity-style continuous \(q\)).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CrrParams {
    pub spot: f64,
    pub strike: f64,
    pub time_years: f64,
    pub rate: f64,
    pub dividend_yield: f64,
    pub vol: f64,
    /// Number of time steps \(N \ge 1\).
    pub steps: usize,
    pub style: ExerciseStyle,
}

impl CrrParams {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        spot: f64,
        strike: f64,
        time_years: f64,
        rate: f64,
        dividend_yield: f64,
        vol: f64,
        steps: usize,
        style: ExerciseStyle,
    ) -> Self {
        Self {
            spot,
            strike,
            time_years,
            rate,
            dividend_yield,
            vol,
            steps,
            style,
        }
    }

    pub fn with_style(mut self, style: ExerciseStyle) -> Self {
        self.style = style;
        self
    }

    /// ATM one-year fixture.
    pub const fn atm_one_year(
        spot: f64,
        rate: f64,
        vol: f64,
        steps: usize,
        style: ExerciseStyle,
    ) -> Self {
        Self::new(spot, spot, 1.0, rate, 0.0, vol, steps, style)
    }
}

/// Tree first-order risk (Δ/Γ from nodes; vega bumped).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CrrGreeks {
    pub delta: f64,
    pub gamma: f64,
    /// Finite-difference ∂V/∂σ per +1.0 absolute vol.
    pub vega: f64,
}

impl CrrGreeks {
    #[inline]
    pub fn vega_per_vol_point(self) -> f64 {
        self.vega / 100.0
    }
}

/// One node for teaching tables.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CrrNode {
    pub step: usize,
    pub up_moves: usize,
    pub stock: f64,
    pub option: f64,
    pub exercise: f64,
    pub continuation: f64,
    /// True if American and exercise > continuation (within eps).
    pub early_exercise: bool,
}

/// Full teaching solution.
#[derive(Clone, Debug)]
pub struct CrrSolution {
    pub option_type: OptionType,
    pub params: CrrParams,
    pub price: f64,
    pub greeks: CrrGreeks,
    /// All nodes when `steps <= print_cap` at build; else empty (use price only).
    pub nodes: Vec<CrrNode>,
    formula: String,
    symbolic_formula: String,
}

impl CrrSolution {
    pub fn formula(&self) -> &str {
        &self.formula
    }
    pub fn symbolic_formula(&self) -> &str {
        &self.symbolic_formula
    }

    /// Print node table (empty if tree was large and nodes not retained).
    pub fn print_table(&self) {
        self.print_table_locale_opt(None, None);
    }

    pub fn print_table_locale(&self, locale: &num_format::Locale, precision: usize) {
        self.print_table_locale_opt(Some(locale), Some(precision));
    }

    fn print_table_locale_opt(
        &self,
        locale: Option<&num_format::Locale>,
        precision: Option<usize>,
    ) {
        if self.nodes.is_empty() {
            println!(
                "(no node table: steps={} > capture cap; price={:.6})",
                self.params.steps, self.price
            );
            return;
        }
        let columns = columns_with_strings(&[
            ("step", "i", true),
            ("ups", "i", true),
            ("stock", "f", true),
            ("option", "f", true),
            ("exercise", "f", true),
            ("cont", "f", true),
            ("early", "s", true),
        ]);
        let data = self
            .nodes
            .iter()
            .map(|n| {
                vec![
                    n.step.to_string(),
                    n.up_moves.to_string(),
                    n.stock.to_string(),
                    n.option.to_string(),
                    n.exercise.to_string(),
                    n.continuation.to_string(),
                    if n.early_exercise { "Y" } else { "" }.to_string(),
                ]
            })
            .collect();
        print_table_locale_opt(&columns, data, locale, precision);
    }
}

/// Validated CRR pack.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ValidatedCrr {
    params: CrrParams,
}

impl ValidatedCrr {
    pub fn new(params: CrrParams) -> FinanceResult<Self> {
        validate_crr_params(params)?;
        Ok(Self { params })
    }

    pub fn params(self) -> CrrParams {
        self.params
    }

    pub fn price(self, option_type: OptionType) -> FinanceResult<f64> {
        crr_price(self.params, option_type)
    }

    pub fn greeks(self, option_type: OptionType) -> FinanceResult<CrrGreeks> {
        crr_greeks(self.params, option_type)
    }
}

/// CRR option price.
pub fn crr_price(params: CrrParams, option_type: OptionType) -> FinanceResult<f64> {
    validate_crr_params(params)?;
    ensure_risk_neutral_prob(params)?;
    Ok(price_unchecked(params, option_type).0)
}

/// Tree Δ/Γ + FD vega.
pub fn crr_greeks(params: CrrParams, option_type: OptionType) -> FinanceResult<CrrGreeks> {
    validate_crr_params(params)?;
    ensure_risk_neutral_prob(params)?;
    Ok(greeks_unchecked(params, option_type))
}

/// Teaching solution; retains nodes when `steps <= 12` (readable table).
pub fn crr_solution(params: CrrParams, option_type: OptionType) -> FinanceResult<CrrSolution> {
    validate_crr_params(params)?;
    ensure_risk_neutral_prob(params)?;
    let capture = params.steps <= 12;
    let (price, nodes) = if capture {
        let (px, nd) = price_with_nodes(params, option_type, true);
        (px, nd)
    } else {
        (price_unchecked(params, option_type).0, Vec::new())
    };
    let greeks = greeks_unchecked(params, option_type);
    let style = match params.style {
        ExerciseStyle::European => "European",
        ExerciseStyle::American => "American",
    };
    let formula = format!(
        "{option_type} CRR {style} S={} K={} T={} r={} q={} σ={} N={} → {:.6}",
        params.spot,
        params.strike,
        params.time_years,
        params.rate,
        params.dividend_yield,
        params.vol,
        params.steps,
        price
    );
    let symbolic =
        "u=e^{σ√Δt}, d=1/u, p*=(e^{(r-q)Δt}-d)/(u-d); V=max(exercise, disc·E*[V]) American"
            .to_string();
    Ok(CrrSolution {
        option_type,
        params,
        price,
        greeks,
        nodes,
        formula,
        symbolic_formula: symbolic,
    })
}

/// American (or European) implied vol via Newton on CRR price + FD vega.
///
/// Uses the same Newton + bisection pattern as European closed-form IV.
pub fn tree_implied_vol(
    params: CrrParams,
    option_type: OptionType,
    market_price: f64,
) -> FinanceResult<f64> {
    validate_crr_params(params)?;
    ensure_risk_neutral_prob(params)?;
    // Note: IV path varies σ; intermediate probes may briefly leave the p* interval and
    // use a defensive clamp inside the tree. Extreme market_price fails the root finder.
    require_finite("market_price", market_price)?;
    if market_price < 0.0 {
        return Err(FinanceError::Unsolvable {
            message: "market_price must be non-negative",
        });
    }
    if params.time_years == 0.0 {
        return Err(FinanceError::Unsolvable {
            message: "implied vol undefined at expiry (T=0)",
        });
    }
    // Intrinsic floor for American (and European at T>0 can be above discounted intrinsic)
    let intrinsic = match option_type {
        OptionType::Call => (params.spot - params.strike).max(0.0),
        OptionType::Put => (params.strike - params.spot).max(0.0),
    };
    if market_price + 1e-12 < intrinsic && params.style.is_american() {
        return Err(FinanceError::Unsolvable {
            message: "market_price below American intrinsic floor",
        });
    }

    crate::derivatives::implied_vol::solve_implied_vol(
        market_price,
        |sigma| {
            let mut p = params;
            p.vol = sigma;
            price_unchecked(p, option_type).0
        },
        |sigma| {
            let mut p = params;
            p.vol = sigma;
            greeks_unchecked(p, option_type).vega
        },
    )
}

/// Alias: American IV when style is American (any style works).
pub fn american_implied_vol(
    params: CrrParams,
    option_type: OptionType,
    market_price: f64,
) -> FinanceResult<f64> {
    tree_implied_vol(params, option_type, market_price)
}

fn validate_crr_params(p: CrrParams) -> FinanceResult<()> {
    require_finite("spot", p.spot)?;
    require_finite("strike", p.strike)?;
    require_finite("time_years", p.time_years)?;
    require_finite("rate", p.rate)?;
    require_finite("dividend_yield", p.dividend_yield)?;
    require_finite("vol", p.vol)?;
    if p.spot <= 0.0 || p.strike <= 0.0 {
        return Err(FinanceError::InvalidCashflow {
            message: "spot and strike must be strictly positive",
        });
    }
    if p.time_years < 0.0 {
        return Err(FinanceError::Unsolvable {
            message: "time_years must be non-negative",
        });
    }
    if p.vol < 0.0 {
        return Err(FinanceError::Unsolvable {
            message: "vol must be non-negative",
        });
    }
    if p.steps == 0 {
        return Err(FinanceError::Unsolvable {
            message: "CRR steps must be >= 1",
        });
    }
    Ok(())
}

/// Risk-neutral up probability must lie in \[0, 1\] for the CRR step size.
fn ensure_risk_neutral_prob(p: CrrParams) -> FinanceResult<()> {
    if p.time_years == 0.0 || p.vol == 0.0 {
        return Ok(());
    }
    let dt = p.time_years / p.steps as f64;
    let u = (p.vol * dt.sqrt()).exp();
    let d = 1.0 / u;
    let a = ((p.rate - p.dividend_yield) * dt).exp();
    let denom = u - d;
    if denom <= 0.0 {
        return Err(FinanceError::Unsolvable {
            message: "CRR up/down factors degenerate (check vol and steps)",
        });
    }
    let p_star = (a - d) / denom;
    if !(0.0..=1.0).contains(&p_star) {
        return Err(FinanceError::Unsolvable {
            message: "CRR risk-neutral probability outside [0, 1]; reduce steps or check r,q,σ,T",
        });
    }
    Ok(())
}

/// Tree layers used for price and finite-difference-free Δ/Γ.
struct TreeResult {
    price: f64,
    /// Option values at step 1: (0 ups / down node, 1 up).
    step1: Option<(f64, f64)>,
    /// Option values at step 2: (0, 1, 2 ups).
    step2: Option<(f64, f64, f64)>,
    /// CRR up factor for this tree (needed for stock spacing in Δ/Γ).
    u: f64,
    d: f64,
    nodes: Vec<CrrNode>,
}

/// Returns (price, dummy triple for call sites that only need price.0).
fn price_unchecked(p: CrrParams, option_type: OptionType) -> (f64, (f64, f64, f64)) {
    let tr = tree_core_full(p, option_type, false);
    let trip = match tr.step1 {
        Some((dn, up)) => (up, tr.price, dn),
        None => (tr.price, tr.price, tr.price),
    };
    (tr.price, trip)
}

fn price_with_nodes(p: CrrParams, option_type: OptionType, capture: bool) -> (f64, Vec<CrrNode>) {
    let tr = tree_core_full(p, option_type, capture);
    (tr.price, tr.nodes)
}

fn greeks_unchecked(p: CrrParams, option_type: OptionType) -> CrrGreeks {
    if p.time_years == 0.0 || p.steps == 0 {
        let delta = match option_type {
            OptionType::Call => {
                if p.spot > p.strike {
                    1.0
                } else if p.spot < p.strike {
                    0.0
                } else {
                    0.5
                }
            }
            OptionType::Put => {
                if p.spot < p.strike {
                    -1.0
                } else if p.spot > p.strike {
                    0.0
                } else {
                    -0.5
                }
            }
        };
        return CrrGreeks {
            delta,
            gamma: 0.0,
            vega: 0.0,
        };
    }

    let tr = tree_core_full(p, option_type, false);
    let (delta, gamma) = match tr.step1 {
        Some((v_dn, v_up)) if (tr.u - tr.d).abs() > 1e-14 => {
            let s_up = p.spot * tr.u;
            let s_dn = p.spot * tr.d;
            let delta = (v_up - v_dn) / (s_up - s_dn);
            let gamma = match tr.step2 {
                Some((v_dd, v_ud, v_uu)) if p.steps >= 2 => {
                    let s_uu = p.spot * tr.u * tr.u;
                    let s_ud = p.spot * tr.u * tr.d;
                    let s_dd = p.spot * tr.d * tr.d;
                    if (s_uu - s_ud).abs() > 1e-14 && (s_ud - s_dd).abs() > 1e-14 {
                        let d_u = (v_uu - v_ud) / (s_uu - s_ud);
                        let d_d = (v_ud - v_dd) / (s_ud - s_dd);
                        (d_u - d_d) / (0.5 * (s_uu - s_dd))
                    } else {
                        0.0
                    }
                }
                _ => 0.0,
            };
            (delta, gamma)
        }
        _ => (0.0, 0.0),
    };

    let h = (p.vol * 0.01).max(1e-4);
    let mut p_up = p;
    p_up.vol = p.vol + h;
    let mut p_dn = p;
    p_dn.vol = (p.vol - h).max(1e-8);
    let v_sigma_up = price_unchecked(p_up, option_type).0;
    let v_sigma_dn = price_unchecked(p_dn, option_type).0;
    let vega = (v_sigma_up - v_sigma_dn) / (p_up.vol - p_dn.vol);

    CrrGreeks { delta, gamma, vega }
}

fn payoff(s: f64, k: f64, option_type: OptionType) -> f64 {
    match option_type {
        OptionType::Call => (s - k).max(0.0),
        OptionType::Put => (k - s).max(0.0),
    }
}

/// Build CRR tree. Captures step-1 / step-2 option values from the **same** tree
/// (critical for N=1 Δ and for Γ). Does **not** reprice independent sub-trees.
fn tree_core_full(p: CrrParams, option_type: OptionType, capture: bool) -> TreeResult {
    let n = p.steps;
    if p.time_years == 0.0 {
        let px = payoff(p.spot, p.strike, option_type);
        return TreeResult {
            price: px,
            step1: None,
            step2: None,
            u: 1.0,
            d: 1.0,
            nodes: Vec::new(),
        };
    }
    if p.vol == 0.0 {
        let f = p.spot * ((p.rate - p.dividend_yield) * p.time_years).exp();
        let disc = (-p.rate * p.time_years).exp();
        let px = disc * payoff(f, p.strike, option_type);
        return TreeResult {
            price: px,
            step1: None,
            step2: None,
            u: 1.0,
            d: 1.0,
            nodes: Vec::new(),
        };
    }

    let dt = p.time_years / n as f64;
    let u = (p.vol * dt.sqrt()).exp();
    let d = 1.0 / u;
    let a = ((p.rate - p.dividend_yield) * dt).exp();
    let denom = u - d;
    // Caller should have run ensure_risk_neutral_prob; keep a defensive clamp only
    // for IV intermediate probes that may briefly leave the interval.
    let p_star = if denom <= 0.0 {
        0.5
    } else {
        ((a - d) / denom).clamp(0.0, 1.0)
    };
    let disc = (-p.rate * dt).exp();
    let q_star = 1.0 - p_star;

    // Terminal (step n): index j = number of up moves
    let mut v: Vec<f64> = (0..=n)
        .map(|j| {
            let s = p.spot * u.powi(j as i32) * d.powi((n - j) as i32);
            payoff(s, p.strike, option_type)
        })
        .collect();

    let mut nodes = Vec::new();
    if capture {
        for j in 0..=n {
            let s = p.spot * u.powi(j as i32) * d.powi((n - j) as i32);
            let ex = payoff(s, p.strike, option_type);
            nodes.push(CrrNode {
                step: n,
                up_moves: j,
                stock: s,
                option: v[j],
                exercise: ex,
                continuation: v[j],
                early_exercise: false,
            });
        }
    }

    // When N=1 terminal *is* step 1; when N=2 terminal *is* step 2.
    let mut step1 = if n == 1 && v.len() >= 2 {
        Some((v[0], v[1]))
    } else {
        None
    };
    let mut step2 = if n == 2 && v.len() >= 3 {
        Some((v[0], v[1], v[2]))
    } else {
        None
    };

    for step in (0..n).rev() {
        let mut next = vec![0.0; step + 1];
        for j in 0..=step {
            let s = p.spot * u.powi(j as i32) * d.powi((step - j) as i32);
            let cont = disc * (p_star * v[j + 1] + q_star * v[j]);
            let ex = payoff(s, p.strike, option_type);
            let val = match p.style {
                ExerciseStyle::European => cont,
                ExerciseStyle::American => cont.max(ex),
            };
            next[j] = val;
            if capture {
                nodes.push(CrrNode {
                    step,
                    up_moves: j,
                    stock: s,
                    option: val,
                    exercise: ex,
                    continuation: cont,
                    early_exercise: p.style.is_american() && ex > cont + 1e-12,
                });
            }
        }
        v = next;
        if step == 1 && v.len() >= 2 {
            step1 = Some((v[0], v[1]));
        }
        if step == 2 && v.len() >= 3 {
            step2 = Some((v[0], v[1], v[2]));
        }
    }

    let price = v[0];
    if capture {
        nodes.sort_by(|a, b| a.step.cmp(&b.step).then(a.up_moves.cmp(&b.up_moves)));
    }
    TreeResult {
        price,
        step1,
        step2,
        u,
        d,
        nodes,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::derivatives::black_scholes::bsm_price;
    use crate::derivatives::types::BsmParams;

    #[test]
    fn european_near_bsm() {
        let p = CrrParams::atm_one_year(100.0, 0.05, 0.20, 500, ExerciseStyle::European);
        let tree = crr_price(p, OptionType::Call).unwrap();
        let bsm = bsm_price(BsmParams::atm_one_year(100.0, 0.05, 0.20), OptionType::Call).unwrap();
        assert!((tree - bsm).abs() < 0.05, "tree={tree} bsm={bsm}");
    }

    #[test]
    fn american_put_ge_european() {
        let base = CrrParams::new(
            100.0,
            100.0,
            1.0,
            0.05,
            0.0,
            0.25,
            100,
            ExerciseStyle::American,
        );
        let am = crr_price(base, OptionType::Put).unwrap();
        let eu = crr_price(base.with_style(ExerciseStyle::European), OptionType::Put).unwrap();
        assert!(am + 1e-9 >= eu, "am={am} eu={eu}");
    }

    #[test]
    fn american_call_q0_near_european() {
        // With q=0, American call ≈ European (no early exercise optimally)
        let base = CrrParams::atm_one_year(100.0, 0.05, 0.2, 80, ExerciseStyle::American);
        let am = crr_price(base, OptionType::Call).unwrap();
        let eu = crr_price(base.with_style(ExerciseStyle::European), OptionType::Call).unwrap();
        assert!((am - eu).abs() < 1e-6);
    }

    #[test]
    fn american_call_with_dividend_ge_european() {
        // q > 0: American call can exceed European
        let base = CrrParams::new(
            100.0,
            100.0,
            1.0,
            0.05,
            0.08,
            0.25,
            120,
            ExerciseStyle::American,
        );
        let am = crr_price(base, OptionType::Call).unwrap();
        let eu = crr_price(base.with_style(ExerciseStyle::European), OptionType::Call).unwrap();
        assert!(am + 1e-9 >= eu, "am={am} eu={eu}");
    }

    #[test]
    fn iv_round_trip_american_put() {
        let p = CrrParams::new(
            100.0,
            100.0,
            1.0,
            0.05,
            0.0,
            0.30,
            80,
            ExerciseStyle::American,
        );
        let mkt = crr_price(p, OptionType::Put).unwrap();
        let iv = american_implied_vol(p, OptionType::Put, mkt).unwrap();
        assert!((iv - 0.30).abs() < 1e-3, "iv={iv}");
    }

    #[test]
    fn solution_nodes_small_n() {
        let p = CrrParams::atm_one_year(100.0, 0.05, 0.2, 3, ExerciseStyle::American);
        let sol = crr_solution(p, OptionType::Put).unwrap();
        assert!(!sol.nodes.is_empty());
        assert!(sol.price > 0.0);
        // Node count = sum_{k=0}^{N} (k+1) = (N+1)(N+2)/2
        assert_eq!(sol.nodes.len(), (3 + 1) * (3 + 2) / 2);
    }

    #[test]
    fn rejects_zero_steps() {
        let mut p = CrrParams::atm_one_year(100.0, 0.05, 0.2, 1, ExerciseStyle::European);
        p.steps = 0;
        assert!(crr_price(p, OptionType::Call).is_err());
    }

    #[test]
    fn n1_delta_not_zero_for_otm_put() {
        // Regression: N=1 used to leave step-1 values at 0 → delta = 0 wrongly.
        let p = CrrParams::new(
            100.0,
            100.0,
            1.0,
            0.05,
            0.0,
            0.25,
            1,
            ExerciseStyle::European,
        );
        let g = crr_greeks(p, OptionType::Put).unwrap();
        assert!(
            g.delta < -0.05 && g.delta > -1.0,
            "N=1 put delta should be meaningfully negative, got {}",
            g.delta
        );
        assert_eq!(g.gamma, 0.0); // need N>=2 for tree gamma
    }

    #[test]
    fn delta_matches_finite_difference() {
        let p = CrrParams::atm_one_year(100.0, 0.05, 0.2, 80, ExerciseStyle::European);
        let g = crr_greeks(p, OptionType::Call).unwrap();
        let h = 0.05;
        let mut up = p;
        up.spot += h;
        let mut dn = p;
        dn.spot -= h;
        let fd = (crr_price(up, OptionType::Call).unwrap()
            - crr_price(dn, OptionType::Call).unwrap())
            / (2.0 * h);
        assert!(
            (g.delta - fd).abs() < 0.02,
            "tree delta {} vs fd {}",
            g.delta,
            fd
        );
    }

    #[test]
    fn gamma_nonnegative_call() {
        let p = CrrParams::atm_one_year(100.0, 0.05, 0.25, 60, ExerciseStyle::European);
        let g = crr_greeks(p, OptionType::Call).unwrap();
        assert!(g.gamma >= -1e-8, "gamma={}", g.gamma);
    }

    #[test]
    fn expiry_is_intrinsic() {
        let p = CrrParams::new(
            110.0,
            100.0,
            0.0,
            0.05,
            0.0,
            0.2,
            10,
            ExerciseStyle::American,
        );
        assert!((crr_price(p, OptionType::Call).unwrap() - 10.0).abs() < 1e-12);
        assert!(crr_price(p, OptionType::Put).unwrap().abs() < 1e-12);
    }

    #[test]
    fn american_iv_rejects_below_intrinsic() {
        let p = CrrParams::new(
            90.0,
            100.0,
            0.5,
            0.05,
            0.0,
            0.2,
            40,
            ExerciseStyle::American,
        );
        // Intrinsic put = 10; mid 5 is impossible
        assert!(american_implied_vol(p, OptionType::Put, 5.0).is_err());
    }

    #[test]
    fn large_n_solution_omits_nodes() {
        let p = CrrParams::atm_one_year(100.0, 0.05, 0.2, 50, ExerciseStyle::European);
        let sol = crr_solution(p, OptionType::Call).unwrap();
        assert!(sol.nodes.is_empty());
        assert!(sol.price > 0.0);
    }

    #[test]
    fn rejects_impossible_risk_neutral_prob() {
        // Huge rate vs tiny vol and few steps can push p* outside [0,1]
        let p = CrrParams::new(
            100.0,
            100.0,
            1.0,
            5.0, // absurd continuous rate
            0.0,
            0.01,
            2,
            ExerciseStyle::European,
        );
        assert!(crr_price(p, OptionType::Call).is_err());
    }
}
