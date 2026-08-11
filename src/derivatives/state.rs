//! # Live BSM state — engineering for streaming underliers
//!
//! Holds one European contract’s BSM inputs and exposes `set_*` mutators so your
//! market-data loop does not rebuild parameter graphs on every tick.
//!
//! ---
//!
//! ## Trading perspective
//!
//! On each **underlier** print, risk wants fresh **Δ/Γ** for every open option.
//! On each **option quote**, vol traders update **IV** from mid and recompute vega/theta.
//! This type is the per-contract scratchpad for that loop — not the book itself.
//!
//! ---
//!
//! ## Engineering perspective
//!
//! ```text
//! HashMap<OptionKey, BsmState>   // in YOUR engine
//!
//! on_underlier_tick(s):
//!   for state in map.values_mut() {
//!       state.set_spot(s)?;
//!       let g = state.greeks()?;   // or throttle / rayon
//!       aggregate_risk(g);
//!   }
//!
//! on_option_quote(key, mid):
//!   map[key].set_vol_from_price(mid)?;
//! ```
//!
//! Combine with TA on the same symbol:
//!
//! ```text
//! on_1m_bar → equity.ta.push(...)
//! on_spot   → options[*].set_spot(s)
//! ```
//!
//! Both pipelines are **sync** math. Concurrency is optional and **outside** this crate
//! (`rayon` over keys, async tasks that only deliver messages).
//!
//! ---
//!
//! ## Example
//!
//! ```
//! use finance_solution::derivatives::{BsmParams, BsmState, OptionType};
//!
//! let p = BsmParams::atm_one_year(100.0, 0.05, 0.20);
//! let mut opt = BsmState::new(p, OptionType::Call).unwrap();
//! // underlier tick:
//! opt.set_spot(101.5).unwrap();
//! let g = opt.greeks().unwrap();
//! assert!(g.delta > 0.0);
//! // mark IV from mid:
//! opt.set_vol_from_price(11.0).unwrap();
//! assert!(opt.params().vol > 0.0);
//! ```

use crate::derivatives::black_scholes::{bsm_greeks, bsm_price, BsmGreeks};
use crate::derivatives::implied_vol::bsm_implied_vol;
use crate::derivatives::types::{validate_bsm_params, BsmParams, OptionType};
use crate::util::error::{require_finite, FinanceResult};

/// Mutable European option under BSM (spot / vol / time / strike updates).
///
/// **Fallible constructor** is [`BsmState::new`] → [`FinanceResult`] (Result-only naming).
#[derive(Clone, Debug, PartialEq)]
pub struct BsmState {
    params: BsmParams,
    option_type: OptionType,
}

impl BsmState {
    /// Validate and store contract + market snapshot.
    pub fn new(params: BsmParams, option_type: OptionType) -> FinanceResult<Self> {
        validate_bsm_params(params)?;
        Ok(Self {
            params,
            option_type,
        })
    }

    pub fn params(&self) -> BsmParams {
        self.params
    }

    pub fn option_type(&self) -> OptionType {
        self.option_type
    }

    /// Underlier mark moved (most common live update).
    pub fn set_spot(&mut self, spot: f64) -> FinanceResult<()> {
        require_finite("spot", spot)?;
        let mut p = self.params;
        p.spot = spot;
        validate_bsm_params(p)?;
        self.params = p;
        Ok(())
    }

    /// Set model / implied vol directly (absolute annualized).
    pub fn set_vol(&mut self, vol: f64) -> FinanceResult<()> {
        require_finite("vol", vol)?;
        let mut p = self.params;
        p.vol = vol;
        validate_bsm_params(p)?;
        self.params = p;
        Ok(())
    }

    /// Clock / expiry decay — pass remaining time in **years**.
    pub fn set_time_years(&mut self, time_years: f64) -> FinanceResult<()> {
        require_finite("time_years", time_years)?;
        let mut p = self.params;
        p.time_years = time_years;
        validate_bsm_params(p)?;
        self.params = p;
        Ok(())
    }

    /// Rarely changes live; included for completeness (e.g. corporate action resstrike).
    pub fn set_strike(&mut self, strike: f64) -> FinanceResult<()> {
        require_finite("strike", strike)?;
        let mut p = self.params;
        p.strike = strike;
        validate_bsm_params(p)?;
        self.params = p;
        Ok(())
    }

    pub fn set_rate(&mut self, rate: f64) -> FinanceResult<()> {
        require_finite("rate", rate)?;
        let mut p = self.params;
        p.rate = rate;
        validate_bsm_params(p)?;
        self.params = p;
        Ok(())
    }

    pub fn set_dividend_yield(&mut self, dividend_yield: f64) -> FinanceResult<()> {
        require_finite("dividend_yield", dividend_yield)?;
        let mut p = self.params;
        p.dividend_yield = dividend_yield;
        validate_bsm_params(p)?;
        self.params = p;
        Ok(())
    }

    /// Invert market premium into IV and store it (**trading:** mark to mid).
    pub fn set_vol_from_price(&mut self, market_price: f64) -> FinanceResult<f64> {
        let iv = bsm_implied_vol(self.params, self.option_type, market_price)?;
        self.set_vol(iv)?;
        Ok(iv)
    }

    /// Model price at current params.
    pub fn price(&self) -> FinanceResult<f64> {
        bsm_price(self.params, self.option_type)
    }

    /// Model Greeks at current params.
    pub fn greeks(&self) -> FinanceResult<BsmGreeks> {
        bsm_greeks(self.params, self.option_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_spot_moves_delta() {
        let p = BsmParams::atm_one_year(100.0, 0.05, 0.2);
        let mut s = BsmState::new(p, OptionType::Call).unwrap();
        let d0 = s.greeks().unwrap().delta;
        s.set_spot(110.0).unwrap();
        let d1 = s.greeks().unwrap().delta;
        assert!(d1 > d0);
    }

    #[test]
    fn set_vol_from_price_round_trip() {
        let p = BsmParams::atm_one_year(100.0, 0.05, 0.22);
        let mut s = BsmState::new(p, OptionType::Call).unwrap();
        let px = s.price().unwrap();
        s.set_vol(0.10).unwrap();
        let iv = s.set_vol_from_price(px).unwrap();
        assert!((iv - 0.22).abs() < 1e-5);
    }
}
