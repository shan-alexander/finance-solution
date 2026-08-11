//! # Garman–Kohlhagen — European FX options
//!
//! Spot FX \(S\) (domestic per unit foreign), continuous **domestic** rate \(r_d\)
//! and **foreign** rate \(r_f\). Mathematically BSM with dividend yield \(q = r_f\).
//!
//! ---
//!
//! ## Trading perspective
//!
//! | Greek | FX desk habit |
//! |-------|----------------|
//! | **Δ** | Hedge in foreign currency units per option |
//! | **ρ_domestic** | Discount / domestic rate risk |
//! | **ρ_foreign** | Foreign-rate risk (like −q rho in equity BSM) |
//! | **IV** | FX vol surfaces (10Δ risk reversals, etc. — build in *your* engine) |
//!
//! **Call** here = right to **buy foreign** (pay domestic strike \(K\)).
//!
//! ---
//!
//! ## Engineering perspective
//!
//! Prefer this module when rates are labeled domestic/foreign. Internally maps to
//! [`crate::derivatives::BsmParams`] with `rate = r_d`, `dividend_yield = r_f` so
//! prices match BSM exactly.
//!
//! ## Word problem
//!
//! > Spot 1.20, K=1.20, T=1y, r_d=5%, r_f=3%, σ=10%. GK call?
//!
//! ```
//! use finance_solution::derivatives::{gk_price, GkParams, OptionType};
//! let p = GkParams {
//!     spot: 1.20,
//!     strike: 1.20,
//!     time_years: 1.0,
//!     domestic_rate: 0.05,
//!     foreign_rate: 0.03,
//!     vol: 0.10,
//! };
//! let c = gk_price(p, OptionType::Call).unwrap();
//! assert!(c > 0.0 && c < 0.10);
//! ```

use crate::derivatives::black_scholes::{
    bsm_cross_greeks, bsm_greeks, bsm_price, bsm_terms, BsmCrossGreeks, BsmGreeks, BsmTerms,
};
use crate::derivatives::types::{validate_bsm_params, BsmParams, OptionType};
use crate::util::error::{require_finite, FinanceError, FinanceResult};
use crate::{columns_with_strings, print_table_locale_opt};

/// Garman–Kohlhagen inputs.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GkParams {
    /// Spot FX: domestic currency per one unit foreign.
    pub spot: f64,
    pub strike: f64,
    pub time_years: f64,
    /// Continuous domestic risk-free rate \(r_d\).
    pub domestic_rate: f64,
    /// Continuous foreign risk-free rate \(r_f\).
    pub foreign_rate: f64,
    pub vol: f64,
}

impl GkParams {
    pub const fn atm_one_year(spot: f64, domestic_rate: f64, foreign_rate: f64, vol: f64) -> Self {
        Self {
            spot,
            strike: spot,
            time_years: 1.0,
            domestic_rate,
            foreign_rate,
            vol,
        }
    }

    pub fn with_days_365_25(
        spot: f64,
        strike: f64,
        days: f64,
        domestic_rate: f64,
        foreign_rate: f64,
        vol: f64,
    ) -> Self {
        Self {
            spot,
            strike,
            time_years: days / 365.25,
            domestic_rate,
            foreign_rate,
            vol,
        }
    }

    /// Map to BSM: \(r = r_d\), \(q = r_f\).
    pub fn to_bsm(self) -> BsmParams {
        BsmParams {
            spot: self.spot,
            strike: self.strike,
            time_years: self.time_years,
            rate: self.domestic_rate,
            dividend_yield: self.foreign_rate,
            vol: self.vol,
        }
    }
}

/// Validated GK snapshot.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ValidatedGk {
    params: GkParams,
}

impl ValidatedGk {
    pub fn new(params: GkParams) -> FinanceResult<Self> {
        validate_gk_params(params)?;
        Ok(Self { params })
    }

    pub fn params(self) -> GkParams {
        self.params
    }

    pub fn price(self, option_type: OptionType) -> FinanceResult<f64> {
        gk_price(self.params, option_type)
    }

    pub fn greeks(self, option_type: OptionType) -> FinanceResult<GkGreeks> {
        gk_greeks(self.params, option_type)
    }
}

/// GK Greeks: BSM-style plus dual rate rhos.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GkGreeks {
    pub delta: f64,
    pub gamma: f64,
    pub vega: f64,
    pub theta: f64,
    /// ∂V/∂r_d (domestic).
    pub rho_domestic: f64,
    /// ∂V/∂r_f (foreign).
    pub rho_foreign: f64,
}

impl GkGreeks {
    #[inline]
    pub fn vega_per_vol_point(self) -> f64 {
        self.vega / 100.0
    }

    #[inline]
    pub fn theta_per_calendar_day(self) -> f64 {
        self.theta / 365.25
    }
}

/// Teaching solution for GK.
#[derive(Clone, Debug)]
pub struct GkSolution {
    pub option_type: OptionType,
    pub params: GkParams,
    pub price: f64,
    pub greeks: GkGreeks,
    pub cross_greeks: BsmCrossGreeks,
    pub terms: BsmTerms,
    pub parity_residual: f64,
    formula: String,
    symbolic_formula: String,
}

impl GkSolution {
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
            ("rho_d", "f", true),
            ("rho_f", "f", true),
        ]);
        let data = vec![vec![
            self.option_type.to_string(),
            self.price.to_string(),
            self.greeks.delta.to_string(),
            self.greeks.gamma.to_string(),
            self.greeks.vega.to_string(),
            self.greeks.theta.to_string(),
            self.greeks.rho_domestic.to_string(),
            self.greeks.rho_foreign.to_string(),
        ]];
        print_table_locale_opt(&columns, data, locale, precision);
    }
}

/// Live FX option state.
#[derive(Clone, Debug, PartialEq)]
pub struct GkState {
    params: GkParams,
    option_type: OptionType,
}

impl GkState {
    pub fn new(params: GkParams, option_type: OptionType) -> FinanceResult<Self> {
        validate_gk_params(params)?;
        Ok(Self {
            params,
            option_type,
        })
    }

    pub fn params(&self) -> GkParams {
        self.params
    }

    pub fn option_type(&self) -> OptionType {
        self.option_type
    }

    pub fn set_spot(&mut self, spot: f64) -> FinanceResult<()> {
        require_finite("spot", spot)?;
        let mut p = self.params;
        p.spot = spot;
        validate_gk_params(p)?;
        self.params = p;
        Ok(())
    }

    pub fn set_vol(&mut self, vol: f64) -> FinanceResult<()> {
        require_finite("vol", vol)?;
        let mut p = self.params;
        p.vol = vol;
        validate_gk_params(p)?;
        self.params = p;
        Ok(())
    }

    pub fn set_time_years(&mut self, time_years: f64) -> FinanceResult<()> {
        require_finite("time_years", time_years)?;
        let mut p = self.params;
        p.time_years = time_years;
        validate_gk_params(p)?;
        self.params = p;
        Ok(())
    }

    pub fn set_vol_from_price(&mut self, market_price: f64) -> FinanceResult<f64> {
        let iv = gk_implied_vol(self.params, self.option_type, market_price)?;
        self.set_vol(iv)?;
        Ok(iv)
    }

    pub fn price(&self) -> FinanceResult<f64> {
        gk_price(self.params, self.option_type)
    }

    pub fn greeks(&self) -> FinanceResult<GkGreeks> {
        gk_greeks(self.params, self.option_type)
    }
}

pub fn gk_price(params: GkParams, option_type: OptionType) -> FinanceResult<f64> {
    validate_gk_params(params)?;
    bsm_price(params.to_bsm(), option_type)
}

pub fn gk_greeks(params: GkParams, option_type: OptionType) -> FinanceResult<GkGreeks> {
    validate_gk_params(params)?;
    let bsm = params.to_bsm();
    let g: BsmGreeks = bsm_greeks(bsm, option_type)?;
    // Foreign rho: ∂V/∂r_f. Equity BSM rho is ∂V/∂r; foreign is −∂V/∂q style.
    // For call: rho_f = −S T e^{-r_f T} N(d1)
    // For put:  rho_f = +S T e^{-r_f T} N(-d1)
    let terms = bsm_terms(bsm)?;
    let df_f = terms.dividend_discount;
    let tt = params.time_years;
    let rho_foreign = match option_type {
        OptionType::Call => -params.spot * tt * df_f * crate::derivatives::norm::norm_cdf(terms.d1),
        OptionType::Put => params.spot * tt * df_f * crate::derivatives::norm::norm_cdf(-terms.d1),
    };
    Ok(GkGreeks {
        delta: g.delta,
        gamma: g.gamma,
        vega: g.vega,
        theta: g.theta,
        rho_domestic: g.rho,
        rho_foreign,
    })
}

pub fn gk_cross_greeks(params: GkParams, option_type: OptionType) -> FinanceResult<BsmCrossGreeks> {
    validate_gk_params(params)?;
    bsm_cross_greeks(params.to_bsm(), option_type)
}

/// Put–call parity residual: `C − P − (S e^{-r_f T} − K e^{-r_d T})`.
pub fn gk_parity_residual(params: GkParams) -> FinanceResult<f64> {
    let c = gk_price(params, OptionType::Call)?;
    let p = gk_price(params, OptionType::Put)?;
    let df_d = (-params.domestic_rate * params.time_years).exp();
    let df_f = (-params.foreign_rate * params.time_years).exp();
    Ok(c - p - (params.spot * df_f - params.strike * df_d))
}

pub fn gk_solution(params: GkParams, option_type: OptionType) -> FinanceResult<GkSolution> {
    let _ = ValidatedGk::new(params)?;
    let price = gk_price(params, option_type)?;
    let greeks = gk_greeks(params, option_type)?;
    let cross_greeks = gk_cross_greeks(params, option_type)?;
    let terms = bsm_terms(params.to_bsm())?;
    let parity = gk_parity_residual(params)?;
    let formula = format!(
        "{option_type} GK S={} K={} T={} r_d={} r_f={} σ={} → price={:.6}",
        params.spot,
        params.strike,
        params.time_years,
        params.domestic_rate,
        params.foreign_rate,
        params.vol,
        price
    );
    let symbolic = match option_type {
        OptionType::Call => {
            "C = S e^{-r_f T} N(d1) - K e^{-r_d T} N(d2); d1=[ln(S/K)+(r_d-r_f+σ²/2)T]/(σ√T)"
                .to_string()
        }
        OptionType::Put => "P = K e^{-r_d T} N(-d2) - S e^{-r_f T} N(-d1)".to_string(),
    };
    Ok(GkSolution {
        option_type,
        params,
        price,
        greeks,
        cross_greeks,
        terms,
        parity_residual: parity,
        formula,
        symbolic_formula: symbolic,
    })
}

pub fn gk_implied_vol(
    params: GkParams,
    option_type: OptionType,
    market_price: f64,
) -> FinanceResult<f64> {
    validate_gk_params(params)?;
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
    // Reuse BSM IV on the mapped pack.
    crate::derivatives::implied_vol::bsm_implied_vol(params.to_bsm(), option_type, market_price)
}

pub(crate) fn validate_gk_params(p: GkParams) -> FinanceResult<()> {
    // Reuse BSM domain checks via map.
    validate_bsm_params(p.to_bsm())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::derivatives::black_scholes::bsm_price;

    #[test]
    fn matches_bsm_mapping() {
        let g = GkParams::atm_one_year(1.25, 0.04, 0.02, 0.12);
        let c_gk = gk_price(g, OptionType::Call).unwrap();
        let c_bsm = bsm_price(g.to_bsm(), OptionType::Call).unwrap();
        assert!((c_gk - c_bsm).abs() < 1e-12);
    }

    #[test]
    fn parity() {
        let p = GkParams {
            spot: 1.10,
            strike: 1.05,
            time_years: 0.5,
            domestic_rate: 0.03,
            foreign_rate: 0.01,
            vol: 0.15,
        };
        assert!(gk_parity_residual(p).unwrap().abs() < 1e-10);
    }

    #[test]
    fn iv_round_trip() {
        let p = GkParams::atm_one_year(1.0, 0.05, 0.03, 0.18);
        let mkt = gk_price(p, OptionType::Put).unwrap();
        let iv = gk_implied_vol(p, OptionType::Put, mkt).unwrap();
        assert!((iv - 0.18).abs() < 1e-6);
    }

    #[test]
    fn foreign_rho_sign_call() {
        let p = GkParams::atm_one_year(1.0, 0.05, 0.02, 0.1);
        let g = gk_greeks(p, OptionType::Call).unwrap();
        assert!(g.rho_foreign < 0.0);
        assert!(g.rho_domestic > 0.0);
    }

    #[test]
    fn foreign_rho_sign_put() {
        let p = GkParams::atm_one_year(1.0, 0.05, 0.02, 0.1);
        let g = gk_greeks(p, OptionType::Put).unwrap();
        assert!(g.rho_foreign > 0.0);
        assert!(g.rho_domestic < 0.0);
    }

    #[test]
    fn state_spot_moves_price() {
        let p = GkParams::atm_one_year(1.10, 0.04, 0.02, 0.12);
        let mut s = GkState::new(p, OptionType::Call).unwrap();
        let p0 = s.price().unwrap();
        s.set_spot(1.15).unwrap();
        assert!(s.price().unwrap() > p0);
    }

    #[test]
    fn cross_greeks_finite() {
        let p = GkParams::atm_one_year(1.2, 0.03, 0.01, 0.15);
        let x = gk_cross_greeks(p, OptionType::Call).unwrap();
        assert!(x.vanna.is_finite() && x.volga.is_finite() && x.charm.is_finite());
    }
}
