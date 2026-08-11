//! # Black ’76 — European options on a forward / futures
//!
//! Prices an option when the natural underlier is a **forward** \(F\) (futures mark,
//! index future, etc.), discounted at continuous rate \(r\).
//!
//! ---
//!
//! ## Trading perspective
//!
//! | Use | Why Black ’76 not spot BSM |
//! |-----|----------------------------|
//! | Options on **futures** | Underlier is \(F\), not cash equity \(S\) |
//! | Many **index** listed products | Quoted vs futures strip |
//! | Rates / commodity vanillas | Forward-first modeling |
//!
//! **Parity (discounted):** \(C - P = e^{-rT}(F - K)\).
//!
//! ---
//!
//! ## Engineering perspective
//!
//! Same layering as BSM: [`Black76Params`] → [`ValidatedBlack76`] / free functions →
//! [`Black76State`] → [`black76_solution`].
//!
//! **Equivalence:** Black ’76 with \((F,K,T,r,\sigma)\) matches BSM with
//! \(S=F\), \(q=r\) (so the equity forward is \(F\)). Prefer this module when your
//! inputs are already forwards — clearer field names and forward Δ.
//!
//! ## Units
//!
//! | Field | Unit |
//! |-------|------|
//! | `forward`, `strike` | same money units |
//! | `time_years` | years |
//! | `rate` | continuous discount rate |
//! | `vol` | annualized absolute |
//! | `delta` | ∂V/∂F (forward delta) |
//! | `vega` | per +1.0 absolute vol |
//! | `theta` | per year |
//!
//! ## Word problem
//!
//! > Futures 100, strike 100, T=1y, r=5%, σ=20%. Black ’76 call?
//!
//! Expect: about **7.58** (less than equity BSM ATM with \(q=0\) ≈ 10.45).
//!
//! ```
//! use finance_solution::derivatives::{black76_price, Black76Params, OptionType};
//! let p = Black76Params::atm_one_year(100.0, 0.05, 0.20);
//! let c = black76_price(p, OptionType::Call).unwrap();
//! assert!((c - 7.577_082).abs() < 1e-3);
//! ```

use crate::derivatives::norm::{norm_cdf, norm_pdf};
use crate::derivatives::types::OptionType;
use crate::util::error::{require_finite, FinanceError, FinanceResult};
use crate::{columns_with_strings, print_table_locale_opt};

/// Black ’76 inputs (European option on a forward).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Black76Params {
    /// Forward / futures price \(F\).
    pub forward: f64,
    pub strike: f64,
    pub time_years: f64,
    /// Continuous discount rate (funding of the option premium).
    pub rate: f64,
    pub vol: f64,
}

impl Black76Params {
    pub const fn atm_one_year(forward: f64, rate: f64, vol: f64) -> Self {
        Self {
            forward,
            strike: forward,
            time_years: 1.0,
            rate,
            vol,
        }
    }

    pub fn with_days_365_25(forward: f64, strike: f64, days: f64, rate: f64, vol: f64) -> Self {
        Self {
            forward,
            strike,
            time_years: days / 365.25,
            rate,
            vol,
        }
    }

    /// Map to BSM with \(S=F\), \(q=r\) (same prices and most Greeks).
    pub fn to_bsm_equiv(self) -> crate::derivatives::types::BsmParams {
        crate::derivatives::types::BsmParams {
            spot: self.forward,
            strike: self.strike,
            time_years: self.time_years,
            rate: self.rate,
            dividend_yield: self.rate,
            vol: self.vol,
        }
    }
}

/// Validated Black ’76 snapshot.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ValidatedBlack76 {
    params: Black76Params,
}

impl ValidatedBlack76 {
    pub fn new(params: Black76Params) -> FinanceResult<Self> {
        validate_black76_params(params)?;
        Ok(Self { params })
    }

    pub fn params(self) -> Black76Params {
        self.params
    }

    pub fn price(self, option_type: OptionType) -> FinanceResult<f64> {
        black76_price(self.params, option_type)
    }

    pub fn greeks(self, option_type: OptionType) -> FinanceResult<Black76Greeks> {
        black76_greeks(self.params, option_type)
    }
}

/// First-order Black ’76 Greeks (Δ is **forward** delta).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Black76Greeks {
    /// ∂V/∂F
    pub delta: f64,
    pub gamma: f64,
    pub vega: f64,
    pub theta: f64,
    /// ∂V/∂r with **F held fixed** (pure discount rho) ≈ −T × price.
    pub rho: f64,
}

impl Black76Greeks {
    #[inline]
    pub fn vega_per_vol_point(self) -> f64 {
        self.vega / 100.0
    }

    #[inline]
    pub fn theta_per_calendar_day(self) -> f64 {
        self.theta / 365.25
    }
}

/// d1/d2 and discount for Black ’76.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Black76Terms {
    pub d1: f64,
    pub d2: f64,
    pub discount: f64,
    pub sqrt_t: f64,
}

/// Teaching solution for Black ’76.
#[derive(Clone, Debug)]
pub struct Black76Solution {
    pub option_type: OptionType,
    pub params: Black76Params,
    pub price: f64,
    pub greeks: Black76Greeks,
    pub terms: Black76Terms,
    /// `C − P − e^{-rT}(F − K)`; model → ≈ 0.
    pub parity_residual: f64,
    formula: String,
    symbolic_formula: String,
}

impl Black76Solution {
    pub fn formula(&self) -> &str {
        &self.formula
    }
    pub fn symbolic_formula(&self) -> &str {
        &self.symbolic_formula
    }

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

/// Live Black ’76 contract state (forward / vol / time updates).
#[derive(Clone, Debug, PartialEq)]
pub struct Black76State {
    params: Black76Params,
    option_type: OptionType,
}

impl Black76State {
    pub fn new(params: Black76Params, option_type: OptionType) -> FinanceResult<Self> {
        validate_black76_params(params)?;
        Ok(Self {
            params,
            option_type,
        })
    }

    pub fn params(&self) -> Black76Params {
        self.params
    }

    pub fn option_type(&self) -> OptionType {
        self.option_type
    }

    pub fn set_forward(&mut self, forward: f64) -> FinanceResult<()> {
        require_finite("forward", forward)?;
        let mut p = self.params;
        p.forward = forward;
        validate_black76_params(p)?;
        self.params = p;
        Ok(())
    }

    pub fn set_vol(&mut self, vol: f64) -> FinanceResult<()> {
        require_finite("vol", vol)?;
        let mut p = self.params;
        p.vol = vol;
        validate_black76_params(p)?;
        self.params = p;
        Ok(())
    }

    pub fn set_time_years(&mut self, time_years: f64) -> FinanceResult<()> {
        require_finite("time_years", time_years)?;
        let mut p = self.params;
        p.time_years = time_years;
        validate_black76_params(p)?;
        self.params = p;
        Ok(())
    }

    pub fn set_vol_from_price(&mut self, market_price: f64) -> FinanceResult<f64> {
        let iv = black76_implied_vol(self.params, self.option_type, market_price)?;
        self.set_vol(iv)?;
        Ok(iv)
    }

    pub fn price(&self) -> FinanceResult<f64> {
        black76_price(self.params, self.option_type)
    }

    pub fn greeks(&self) -> FinanceResult<Black76Greeks> {
        black76_greeks(self.params, self.option_type)
    }
}

pub fn black76_price(params: Black76Params, option_type: OptionType) -> FinanceResult<f64> {
    validate_black76_params(params)?;
    Ok(price_unchecked(params, option_type))
}

pub fn black76_greeks(
    params: Black76Params,
    option_type: OptionType,
) -> FinanceResult<Black76Greeks> {
    validate_black76_params(params)?;
    Ok(greeks_unchecked(params, option_type))
}

pub fn black76_terms(params: Black76Params) -> FinanceResult<Black76Terms> {
    validate_black76_params(params)?;
    Ok(terms_unchecked(params))
}

/// `C − P − e^{-rT}(F − K)`.
pub fn black76_parity_residual(params: Black76Params) -> FinanceResult<f64> {
    let c = black76_price(params, OptionType::Call)?;
    let p = black76_price(params, OptionType::Put)?;
    let disc = (-params.rate * params.time_years).exp();
    Ok(c - p - disc * (params.forward - params.strike))
}

pub fn black76_solution(
    params: Black76Params,
    option_type: OptionType,
) -> FinanceResult<Black76Solution> {
    let _ = ValidatedBlack76::new(params)?;
    let price = price_unchecked(params, option_type);
    let greeks = greeks_unchecked(params, option_type);
    let terms = terms_unchecked(params);
    let parity = black76_parity_residual(params)?;
    let formula = format!(
        "{option_type} Black76 F={} K={} T={} r={} σ={} → price={:.6}",
        params.forward, params.strike, params.time_years, params.rate, params.vol, price
    );
    let symbolic = match option_type {
        OptionType::Call => {
            "C = e^{-rT}[F N(d1) - K N(d2)]; d1=[ln(F/K)+σ²T/2]/(σ√T); d2=d1-σ√T".to_string()
        }
        OptionType::Put => "P = e^{-rT}[K N(-d2) - F N(-d1)]; d1,d2 as in call".to_string(),
    };
    Ok(Black76Solution {
        option_type,
        params,
        price,
        greeks,
        terms,
        parity_residual: parity,
        formula,
        symbolic_formula: symbolic,
    })
}

/// Implied vol for Black ’76 given a market premium.
pub fn black76_implied_vol(
    params: Black76Params,
    option_type: OptionType,
    market_price: f64,
) -> FinanceResult<f64> {
    validate_black76_params(params)?;
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

    crate::derivatives::implied_vol::solve_implied_vol(
        market_price,
        |sigma| {
            let mut p = params;
            p.vol = sigma;
            price_unchecked(p, option_type)
        },
        |sigma| {
            let mut p = params;
            p.vol = sigma;
            greeks_unchecked(p, option_type).vega
        },
    )
}

pub(crate) fn validate_black76_params(p: Black76Params) -> FinanceResult<()> {
    require_finite("forward", p.forward)?;
    require_finite("strike", p.strike)?;
    require_finite("time_years", p.time_years)?;
    require_finite("rate", p.rate)?;
    require_finite("vol", p.vol)?;
    if p.forward <= 0.0 {
        return Err(FinanceError::InvalidCashflow {
            message: "forward must be strictly positive",
        });
    }
    if p.strike <= 0.0 {
        return Err(FinanceError::InvalidCashflow {
            message: "strike must be strictly positive",
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
    Ok(())
}

fn terms_unchecked(p: Black76Params) -> Black76Terms {
    let sqrt_t = p.time_years.sqrt();
    let discount = (-p.rate * p.time_years).exp();
    if p.time_years == 0.0 || p.vol == 0.0 {
        let d1 = if p.forward > p.strike {
            f64::INFINITY
        } else if p.forward < p.strike {
            f64::NEG_INFINITY
        } else {
            0.0
        };
        return Black76Terms {
            d1,
            d2: d1,
            discount,
            sqrt_t,
        };
    }
    let sig_s = p.vol * sqrt_t;
    let d1 = ((p.forward / p.strike).ln() + 0.5 * p.vol * p.vol * p.time_years) / sig_s;
    let d2 = d1 - sig_s;
    Black76Terms {
        d1,
        d2,
        discount,
        sqrt_t,
    }
}

fn price_unchecked(p: Black76Params, option_type: OptionType) -> f64 {
    if p.time_years == 0.0 {
        return match option_type {
            OptionType::Call => (p.forward - p.strike).max(0.0),
            OptionType::Put => (p.strike - p.forward).max(0.0),
        };
    }
    if p.vol == 0.0 {
        let disc = (-p.rate * p.time_years).exp();
        return match option_type {
            OptionType::Call => disc * (p.forward - p.strike).max(0.0),
            OptionType::Put => disc * (p.strike - p.forward).max(0.0),
        };
    }
    let t = terms_unchecked(p);
    let df = t.discount;
    match option_type {
        OptionType::Call => df * (p.forward * norm_cdf(t.d1) - p.strike * norm_cdf(t.d2)),
        OptionType::Put => df * (p.strike * norm_cdf(-t.d2) - p.forward * norm_cdf(-t.d1)),
    }
}

fn greeks_unchecked(p: Black76Params, option_type: OptionType) -> Black76Greeks {
    if p.time_years == 0.0 {
        let delta = match option_type {
            OptionType::Call => {
                if p.forward > p.strike {
                    1.0
                } else if p.forward < p.strike {
                    0.0
                } else {
                    0.5
                }
            }
            OptionType::Put => {
                if p.forward < p.strike {
                    -1.0
                } else if p.forward > p.strike {
                    0.0
                } else {
                    -0.5
                }
            }
        };
        return Black76Greeks {
            delta,
            gamma: 0.0,
            vega: 0.0,
            theta: 0.0,
            rho: 0.0,
        };
    }

    if p.vol == 0.0 {
        let price = price_unchecked(p, option_type);
        let disc = (-p.rate * p.time_years).exp();
        let delta = match option_type {
            OptionType::Call => {
                if p.forward > p.strike {
                    disc
                } else {
                    0.0
                }
            }
            OptionType::Put => {
                if p.forward < p.strike {
                    -disc
                } else {
                    0.0
                }
            }
        };
        return Black76Greeks {
            delta,
            gamma: 0.0,
            vega: 0.0,
            theta: 0.0,
            rho: -p.time_years * price,
        };
    }

    let t = terms_unchecked(p);
    let df = t.discount;
    let n_d1 = norm_pdf(t.d1);
    let sqrt_t = t.sqrt_t;
    let gamma = df * n_d1 / (p.forward * p.vol * sqrt_t);
    let vega = df * p.forward * n_d1 * sqrt_t;
    let price = price_unchecked(p, option_type);
    let rho = -p.time_years * price;

    // Diffusion term shared; discount bleed −r·V with F held fixed.
    let theta = -df * p.forward * n_d1 * p.vol / (2.0 * sqrt_t) - p.rate * price;
    let delta = match option_type {
        OptionType::Call => df * norm_cdf(t.d1),
        OptionType::Put => -df * norm_cdf(-t.d1),
    };

    Black76Greeks {
        delta,
        gamma,
        vega,
        theta,
        rho,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::derivatives::black_scholes::bsm_price;
    use crate::derivatives::types::BsmParams;

    #[test]
    fn matches_bsm_with_q_eq_r() {
        let b = Black76Params::atm_one_year(100.0, 0.05, 0.20);
        let bsm = BsmParams {
            spot: 100.0,
            strike: 100.0,
            time_years: 1.0,
            rate: 0.05,
            dividend_yield: 0.05,
            vol: 0.20,
        };
        let c76 = black76_price(b, OptionType::Call).unwrap();
        let c_bsm = bsm_price(bsm, OptionType::Call).unwrap();
        assert!((c76 - c_bsm).abs() < 1e-10);
        assert!((c76 - 7.577_082).abs() < 1e-3);
    }

    #[test]
    fn parity() {
        let p = Black76Params {
            forward: 105.0,
            strike: 100.0,
            time_years: 0.5,
            rate: 0.03,
            vol: 0.22,
        };
        assert!(black76_parity_residual(p).unwrap().abs() < 1e-10);
    }

    #[test]
    fn iv_round_trip() {
        let p = Black76Params::atm_one_year(100.0, 0.04, 0.28);
        let mkt = black76_price(p, OptionType::Call).unwrap();
        let iv = black76_implied_vol(p, OptionType::Call, mkt).unwrap();
        assert!((iv - 0.28).abs() < 1e-6);
    }

    #[test]
    fn state_forward_moves_delta() {
        let p = Black76Params::atm_one_year(100.0, 0.05, 0.2);
        let mut s = Black76State::new(p, OptionType::Call).unwrap();
        let d0 = s.greeks().unwrap().delta;
        s.set_forward(110.0).unwrap();
        assert!(s.greeks().unwrap().delta > d0);
    }

    #[test]
    fn expiry_intrinsic_undiscounted() {
        let p = Black76Params {
            forward: 120.0,
            strike: 100.0,
            time_years: 0.0,
            rate: 0.05,
            vol: 0.2,
        };
        assert!((black76_price(p, OptionType::Call).unwrap() - 20.0).abs() < 1e-12);
        assert!(black76_price(p, OptionType::Put).unwrap().abs() < 1e-12);
    }

    #[test]
    fn put_delta_negative() {
        let g = black76_greeks(
            Black76Params::atm_one_year(100.0, 0.05, 0.2),
            OptionType::Put,
        )
        .unwrap();
        assert!(g.delta < 0.0 && g.delta > -1.0);
    }

    #[test]
    fn rejects_nonpositive_forward() {
        let mut p = Black76Params::atm_one_year(100.0, 0.05, 0.2);
        p.forward = 0.0;
        assert!(black76_price(p, OptionType::Call).is_err());
    }

    #[test]
    fn rho_equals_minus_t_times_price() {
        let p = Black76Params::atm_one_year(100.0, 0.05, 0.2);
        let px = black76_price(p, OptionType::Call).unwrap();
        let g = black76_greeks(p, OptionType::Call).unwrap();
        assert!((g.rho + p.time_years * px).abs() < 1e-10);
    }
}
