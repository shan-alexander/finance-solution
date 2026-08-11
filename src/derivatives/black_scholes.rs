//! # Black–Scholes–Merton European options
//!
//! Continuous dividend yield `q`. Call/put **prices** and first-order **Greeks**.
//!
//! ---
//!
//! ## Trading perspective — reading the Greeks
//!
//! | Greek | Sign (long call, typical) | How desks use it |
//! |-------|---------------------------|------------------|
//! | **Delta** | 0…1 | Hedge ratio; “30Δ call” screening; book Δ sum |
//! | **Gamma** | ≥ 0 | Convexity; how often to re-hedge; pin risk near expiry |
//! | **Vega** | ≥ 0 | IV long/short; event premium; surface relative value |
//! | **Theta** | often ≤ 0 for long options | Overnight bleed; why short premium earns carry |
//! | **Rho** | call ≥ 0, put ≤ 0 | Rate sensitivity; larger for long-dated |
//!
//! **Hedge sketch (per long call):** short ≈ `delta` shares of underlier to flatten Δ.
//! Gamma makes that hedge wrong as spot moves — traders who are long gamma rebalance
//! productively; short gamma pay for stability.
//!
//! **IV vs realized:** model vol is an *input*. Market IV is solved from mid
//! ([`crate::derivatives::bsm_implied_vol`]). Comparing IV to realized vol (from
//! [`crate::stocks`]) is a classic relative-value story — join those in *your* engine.
//!
//! ---
//!
//! ## Engineering perspective
//!
//! - Hot path: [`bsm_price`] / [`bsm_greeks`] or [`ValidatedBsm`] after one validate.  
//! - Live tape: [`crate::derivatives::BsmState`] per contract.  
//! - Teaching / audit: [`bsm_solution`] + [`BsmSolution::print_table`].  
//! - Vega is per **+1.0** in σ; for “per vol point (1%)” use [`BsmGreeks::vega_per_vol_point`].  
//! - Theta is per **year**; for daily use [`BsmGreeks::theta_per_calendar_day`].
//!
//! Limits handled explicitly: `T = 0` → intrinsic; `σ = 0` → discounted forward intrinsic.
//!
//! ---
//!
//! ## Word problem
//!
//! > Spot 100, strike 100, T=1y, r=5%, q=0, σ=20%. What is the BSM call roughly?
//!
//! Expect: about **10.45** (classic textbook ATM).
//!
//! ```
//! use finance_solution::derivatives::{bsm_price, BsmParams, OptionType};
//! let p = BsmParams::atm_one_year(100.0, 0.05, 0.20);
//! let c = bsm_price(p, OptionType::Call).unwrap();
//! assert!((c - 10.4506).abs() < 1e-3);
//! ```
//!
//! ## Sample `bsm_solution` table
//!
//! ```text
//! type    price   delta   gamma     vega    theta      rho
//! ----  -------  ------  ------  -------  -------  -------
//! Call  10.4506  0.6368  0.0188  37.5240  -6.4140  53.2325
//! ```

use crate::derivatives::norm::{norm_cdf, norm_pdf};
use crate::derivatives::types::{
    forward_moneyness, intrinsic, time_value, validate_bsm_params, BsmParams, OptionType,
    ValidatedBsm,
};
use crate::util::error::FinanceResult;
use crate::{columns_with_strings, print_table_locale_opt};

/// First-order BSM Greeks.
///
/// # Units (critical)
///
/// | Field | Unit |
/// |-------|------|
/// | `delta` | ∂V/∂S (share equivalent per option) |
/// | `gamma` | ∂²V/∂S² |
/// | `vega` | ∂V/∂σ per **+1.0** absolute vol (not per 1%) |
/// | `theta` | ∂V/∂T per **year** |
/// | `rho` | ∂V/∂r per +1.0 absolute rate |
///
/// **Trading:** risk systems often show vega “per 1%” and theta “per day” — use the helpers
/// below so you do not silently mis-scale P&amp;L.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BsmGreeks {
    pub delta: f64,
    pub gamma: f64,
    /// Per +1.0 absolute vol (divide by 100 for “per vol point”).
    pub vega: f64,
    /// Calendar year basis (same time unit as `time_years`).
    pub theta: f64,
    pub rho: f64,
}

impl BsmGreeks {
    /// Vega per **one percentage point** of vol (desk convention): `vega / 100`.
    ///
    /// Example: if IV rises from 20% to 21%, P&amp;L ≈ `vega_per_vol_point()` per option.
    #[inline]
    pub fn vega_per_vol_point(self) -> f64 {
        self.vega / 100.0
    }

    /// Theta per **calendar day** using 365.25 days/year: `theta / 365.25`.
    ///
    /// Business-day or trading-day conventions differ by desk — override externally if needed.
    #[inline]
    pub fn theta_per_calendar_day(self) -> f64 {
        self.theta / 365.25
    }
}

/// Intermediate terms shared by price and Greeks (`d1`, `d2`, discounts).
///
/// **Teaching:** show `d1`/`d2` next to N(d1) stories.  
/// **Engineering:** rarely needed on the hot path if you only consume price/greeks.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BsmTerms {
    pub d1: f64,
    pub d2: f64,
    pub discount: f64,
    pub dividend_discount: f64,
    pub sqrt_t: f64,
}

/// Teaching solution: price, greeks, parity check, formulas.
///
/// Prefer this for notebooks and audit logs. Prefer [`bsm_price`] / [`BsmState`] in production.
#[derive(Clone, Debug)]
pub struct BsmSolution {
    pub option_type: OptionType,
    pub params: BsmParams,
    pub price: f64,
    pub greeks: BsmGreeks,
    pub terms: BsmTerms,
    pub intrinsic: f64,
    pub time_value: f64,
    pub forward_moneyness: f64,
    /// `C − P − (S e^{-qT} − K e^{-rT})`; model-consistent prices → ≈ 0.
    pub parity_residual: f64,
    formula: String,
    symbolic_formula: String,
}

impl BsmSolution {
    pub fn formula(&self) -> &str {
        &self.formula
    }
    pub fn symbolic_formula(&self) -> &str {
        &self.symbolic_formula
    }

    /// Print a one-row summary table of price + Greeks.
    ///
    /// ```text
    /// type    price   delta   gamma     vega    theta      rho
    /// ----  -------  ------  ------  -------  -------  -------
    /// Call  10.4506  0.6368  0.0188  37.5240  -6.4140  53.2325
    /// ```
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
        let columns = columns_with_strings(&[
            ("type", "s", true),
            ("price", "f", true),
            ("delta", "f", true),
            ("gamma", "f", true),
            ("vega", "f", true),
            ("theta", "f", true),
            ("rho", "f", true),
        ]);
        let data = vec![vec![
            self.option_type.to_string(),
            self.price.to_string(),
            self.greeks.delta.to_string(),
            self.greeks.gamma.to_string(),
            self.greeks.vega.to_string(),
            self.greeks.theta.to_string(),
            self.greeks.rho.to_string(),
        ]];
        print_table_locale_opt(&columns, data, locale, precision);
    }
}

/// European BSM price.
///
/// **Trading:** model mark for edge vs mid (after fees).  
/// **Engineering:** pure function of [`BsmParams`]; validate domain first.
///
/// # Errors
/// Non-finite or non-positive spot/strike; negative time or vol.
pub fn bsm_price(params: BsmParams, option_type: OptionType) -> FinanceResult<f64> {
    validate_bsm_params(params)?;
    Ok(price_unchecked(params, option_type))
}

/// European BSM Greeks (see [`BsmGreeks`] for units).
///
/// **Trading:** feed risk aggregators and hedge ratios.  
/// **Engineering:** one call returns all first-order Greeks for a snapshot.
pub fn bsm_greeks(params: BsmParams, option_type: OptionType) -> FinanceResult<BsmGreeks> {
    validate_bsm_params(params)?;
    Ok(greeks_unchecked(params, option_type))
}

/// d1/d2 and discount factors (for teaching / advanced use).
pub fn bsm_terms(params: BsmParams) -> FinanceResult<BsmTerms> {
    validate_bsm_params(params)?;
    Ok(terms_unchecked(params))
}

/// Put–call parity residual: `C − P − (S e^{−qT} − K e^{−rT})` (≈ 0 for BSM).
///
/// **Trading:** large residual on **market** mids may mean bad quotes, early exercise
/// premium (American), or dividends — not free arb without costs.  
/// **Engineering:** use on **model** prices as a self-consistency test (should be ~0).
pub fn put_call_parity_residual(params: BsmParams) -> FinanceResult<f64> {
    let c = bsm_price(params, OptionType::Call)?;
    let p = bsm_price(params, OptionType::Put)?;
    let disc = (-params.rate * params.time_years).exp();
    let div = (-params.dividend_yield * params.time_years).exp();
    Ok(c - p - (params.spot * div - params.strike * disc))
}

/// Full teaching solution (price, greeks, intrinsic, parity, formulas).
///
/// # Examples
/// ```
/// use finance_solution::derivatives::{bsm_solution, BsmParams, OptionType};
/// let sol = bsm_solution(BsmParams::atm_one_year(100.0, 0.05, 0.20), OptionType::Call).unwrap();
/// assert!(sol.price > 0.0);
/// assert!(sol.parity_residual.abs() < 1e-8);
/// assert!(sol.greeks.vega_per_vol_point() > 0.0);
/// // sol.print_table();
/// ```
pub fn bsm_solution(params: BsmParams, option_type: OptionType) -> FinanceResult<BsmSolution> {
    let _ = ValidatedBsm::new(params)?;
    let price = price_unchecked(params, option_type);
    let greeks = greeks_unchecked(params, option_type);
    let terms = terms_unchecked(params);
    let intrinsic_v = intrinsic(params.spot, params.strike, option_type)?;
    let tv = time_value(price, params.spot, params.strike, option_type)?;
    let fm = forward_moneyness(params)?;
    let parity = put_call_parity_residual(params)?;
    let formula = format!(
        "{option_type} BSM S={} K={} T={} r={} q={} σ={} → price={:.6}",
        params.spot,
        params.strike,
        params.time_years,
        params.rate,
        params.dividend_yield,
        params.vol,
        price
    );
    let symbolic = match option_type {
        OptionType::Call => {
            "C = S e^{-qT} N(d1) - K e^{-rT} N(d2); d1 = [ln(S/K)+(r-q+σ²/2)T]/(σ√T); d2 = d1-σ√T"
                .to_string()
        }
        OptionType::Put => "P = K e^{-rT} N(-d2) - S e^{-qT} N(-d1); d1,d2 as in call".to_string(),
    };
    Ok(BsmSolution {
        option_type,
        params,
        price,
        greeks,
        terms,
        intrinsic: intrinsic_v,
        time_value: tv,
        forward_moneyness: fm,
        parity_residual: parity,
        formula,
        symbolic_formula: symbolic,
    })
}

fn terms_unchecked(p: BsmParams) -> BsmTerms {
    let sqrt_t = p.time_years.sqrt();
    let discount = (-p.rate * p.time_years).exp();
    let dividend_discount = (-p.dividend_yield * p.time_years).exp();

    if p.time_years == 0.0 || p.vol == 0.0 {
        let forward = p.spot * ((p.rate - p.dividend_yield) * p.time_years).exp();
        let d1 = if forward > p.strike {
            f64::INFINITY
        } else if forward < p.strike {
            f64::NEG_INFINITY
        } else {
            0.0
        };
        return BsmTerms {
            d1,
            d2: d1,
            discount,
            dividend_discount,
            sqrt_t,
        };
    }

    let sig_s = p.vol * sqrt_t;
    let d1 = ((p.spot / p.strike).ln()
        + (p.rate - p.dividend_yield + 0.5 * p.vol * p.vol) * p.time_years)
        / sig_s;
    let d2 = d1 - sig_s;
    BsmTerms {
        d1,
        d2,
        discount,
        dividend_discount,
        sqrt_t,
    }
}

fn price_unchecked(p: BsmParams, option_type: OptionType) -> f64 {
    if p.time_years == 0.0 {
        return match option_type {
            OptionType::Call => (p.spot - p.strike).max(0.0),
            OptionType::Put => (p.strike - p.spot).max(0.0),
        };
    }
    if p.vol == 0.0 {
        let f = p.spot * ((p.rate - p.dividend_yield) * p.time_years).exp();
        let disc = (-p.rate * p.time_years).exp();
        return match option_type {
            OptionType::Call => disc * (f - p.strike).max(0.0),
            OptionType::Put => disc * (p.strike - f).max(0.0),
        };
    }

    let t = terms_unchecked(p);
    let df_q = t.dividend_discount;
    let df_r = t.discount;
    match option_type {
        OptionType::Call => p.spot * df_q * norm_cdf(t.d1) - p.strike * df_r * norm_cdf(t.d2),
        OptionType::Put => p.strike * df_r * norm_cdf(-t.d2) - p.spot * df_q * norm_cdf(-t.d1),
    }
}

fn greeks_unchecked(p: BsmParams, option_type: OptionType) -> BsmGreeks {
    if p.time_years == 0.0 {
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
        return BsmGreeks {
            delta,
            gamma: 0.0,
            vega: 0.0,
            theta: 0.0,
            rho: 0.0,
        };
    }

    if p.vol == 0.0 {
        let price_up = {
            let mut q = p;
            q.spot *= 1.0 + 1e-6;
            price_unchecked(q, option_type)
        };
        let price_0 = price_unchecked(p, option_type);
        let delta = (price_up - price_0) / (p.spot * 1e-6);
        return BsmGreeks {
            delta,
            gamma: 0.0,
            vega: 0.0,
            theta: 0.0,
            rho: 0.0,
        };
    }

    let t = terms_unchecked(p);
    let df_q = t.dividend_discount;
    let df_r = t.discount;
    let n_d1 = norm_pdf(t.d1);
    let sqrt_t = t.sqrt_t;
    let gamma = df_q * n_d1 / (p.spot * p.vol * sqrt_t);
    let vega = p.spot * df_q * n_d1 * sqrt_t;

    let (delta, theta, rho) = match option_type {
        OptionType::Call => {
            let delta = df_q * norm_cdf(t.d1);
            let theta = -p.spot * df_q * n_d1 * p.vol / (2.0 * sqrt_t)
                - p.rate * p.strike * df_r * norm_cdf(t.d2)
                + p.dividend_yield * p.spot * df_q * norm_cdf(t.d1);
            let rho = p.strike * p.time_years * df_r * norm_cdf(t.d2);
            (delta, theta, rho)
        }
        OptionType::Put => {
            let delta = df_q * (norm_cdf(t.d1) - 1.0);
            let theta = -p.spot * df_q * n_d1 * p.vol / (2.0 * sqrt_t)
                + p.rate * p.strike * df_r * norm_cdf(-t.d2)
                - p.dividend_yield * p.spot * df_q * norm_cdf(-t.d1);
            let rho = -p.strike * p.time_years * df_r * norm_cdf(-t.d2);
            (delta, theta, rho)
        }
    };

    BsmGreeks {
        delta,
        gamma,
        vega,
        theta,
        rho,
    }
}

pub(crate) fn price_raw(params: BsmParams, option_type: OptionType) -> FinanceResult<f64> {
    bsm_price(params, option_type)
}

pub(crate) fn vega_raw(params: BsmParams, option_type: OptionType) -> FinanceResult<f64> {
    Ok(bsm_greeks(params, option_type)?.vega)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atm_call_textbook() {
        let p = BsmParams::atm_one_year(100.0, 0.05, 0.20);
        let c = bsm_price(p, OptionType::Call).unwrap();
        assert!((c - 10.450_583_57).abs() < 1e-4);
    }

    #[test]
    fn put_call_parity() {
        let p = BsmParams {
            spot: 100.0,
            strike: 95.0,
            time_years: 0.5,
            rate: 0.03,
            dividend_yield: 0.01,
            vol: 0.25,
        };
        assert!(put_call_parity_residual(p).unwrap().abs() < 1e-10);
    }

    #[test]
    fn expiry_intrinsic() {
        let p = BsmParams {
            spot: 110.0,
            strike: 100.0,
            time_years: 0.0,
            rate: 0.05,
            dividend_yield: 0.0,
            vol: 0.2,
        };
        assert!((bsm_price(p, OptionType::Call).unwrap() - 10.0).abs() < 1e-12);
        assert!((bsm_price(p, OptionType::Put).unwrap()).abs() < 1e-12);
    }

    #[test]
    fn delta_bounds_call() {
        let p = BsmParams::atm_one_year(100.0, 0.05, 0.2);
        let d = bsm_greeks(p, OptionType::Call).unwrap().delta;
        assert!(d > 0.0 && d < 1.0);
    }

    #[test]
    fn vega_scale_helpers() {
        let g = bsm_greeks(BsmParams::atm_one_year(100.0, 0.05, 0.2), OptionType::Call).unwrap();
        assert!((g.vega_per_vol_point() * 100.0 - g.vega).abs() < 1e-12);
        assert!((g.theta_per_calendar_day() * 365.25 - g.theta).abs() < 1e-12);
    }

    #[test]
    fn rejects_bad_spot() {
        let mut p = BsmParams::atm_one_year(100.0, 0.05, 0.2);
        p.spot = 0.0;
        assert!(bsm_price(p, OptionType::Call).is_err());
    }
}
