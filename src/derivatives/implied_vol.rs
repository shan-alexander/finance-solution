//! # Implied volatility
//!
//! Invert a European **market premium** → annualized σ for BSM, Black ’76, or
//! Garman–Kohlhagen (same Newton + bisection core).
//!
//! ---
//!
//! ## Trading perspective
//!
//! The market does not quote “model vol”; it quotes **prices**. IV is the σ that makes
//! the model match that price — the language of surfaces, skew, and “IV crush”.
//!
//! | Use | How |
//! |-----|-----|
//! | Mark a book | Mid → IV → store on the contract |
//! | Relative value | Compare IV across strikes/expiries (skew, term structure) |
//! | Events | IV vs realized (realized from underlier returns / TA path) |
//!
//! **Gotchas desks already know:** American early exercise, discrete dividends, and
//! wide markets break naive European IV — treat results as model-dependent.
//!
//! ---
//!
//! ## Engineering perspective
//!
//! - Solver: Newton–Raphson on vega, **Brent** bracketed fallback (not plain bisection).  
//! - Model `vol` field is **ignored** as a seed (only other market/contract inputs matter).  
//! - Prefer `*State::set_vol_from_price` in live loops so IV is stored once.  
//! - Hot path cost: a handful of model evaluations — fine per quote; batch chains may parallelize **across strikes** in your engine.
//!
//! ---
//!
//! ## Word problem
//!
//! > Market call mid is ~10.45 with S=K=100, T=1, r=5%, q=0. What is IV?
//!
//! Expect: about **20%**.
//!
//! ```
//! use finance_solution::derivatives::{
//!     bsm_implied_vol, BsmParams, OptionType,
//! };
//! let p = BsmParams::atm_one_year(100.0, 0.05, 0.20);
//! let iv = bsm_implied_vol(p, OptionType::Call, 10.4506).unwrap();
//! assert!((iv - 0.20).abs() < 1e-3);
//! ```

use crate::derivatives::black_scholes::{price_raw, vega_raw};
use crate::derivatives::types::{validate_bsm_params, BsmParams, OptionType};
use crate::util::error::{require_finite, FinanceError, FinanceResult};

/// Solve for annualized vol given a target BSM premium.
///
/// Uses Newton–Raphson on vega with **Brent** fallback. `params.vol` is not used as a seed.
///
/// # Errors
/// Invalid BSM domain, non-finite premium, T=0, premium outside achievable range, or solver failure.
pub fn bsm_implied_vol(
    params: BsmParams,
    option_type: OptionType,
    market_price: f64,
) -> FinanceResult<f64> {
    validate_bsm_params(params)?;
    require_finite("market_price", market_price)?;
    if market_price < 0.0 {
        return Err(FinanceError::Unsolvable {
            message: "market_price must be non-negative",
        });
    }

    let intrinsic = match option_type {
        OptionType::Call => (params.spot - params.strike).max(0.0),
        OptionType::Put => (params.strike - params.spot).max(0.0),
    };
    if market_price + 1e-12 < intrinsic && params.time_years == 0.0 {
        return Err(FinanceError::Unsolvable {
            message: "market_price below intrinsic",
        });
    }

    if params.time_years == 0.0 {
        return Err(FinanceError::Unsolvable {
            message: "implied vol undefined at expiry (T=0)",
        });
    }

    solve_implied_vol(
        market_price,
        |sigma| {
            let mut q = params;
            q.vol = sigma;
            price_raw(q, option_type).unwrap_or(f64::NAN)
        },
        |sigma| {
            let mut q = params;
            q.vol = sigma;
            vega_raw(q, option_type).unwrap_or(0.0)
        },
    )
}

/// Generic IV root-finder shared by BSM / Black ’76 / GK.
///
/// `price_at(σ)` and `vega_at(σ)` must be pure in σ for fixed contract inputs.
pub(crate) fn solve_implied_vol(
    market_price: f64,
    mut price_at: impl FnMut(f64) -> f64,
    mut vega_at: impl FnMut(f64) -> f64,
) -> FinanceResult<f64> {
    let lo = 1e-8;
    let mut hi = 5.0;
    let mut price_hi = price_at(hi);
    if !price_hi.is_finite() {
        return Err(FinanceError::Unsolvable {
            message: "model price non-finite during IV solve",
        });
    }
    let mut expand = 0;
    while price_hi < market_price && hi < 50.0 && expand < 10 {
        hi *= 2.0;
        price_hi = price_at(hi);
        expand += 1;
    }
    let price_lo = price_at(lo);
    if !price_lo.is_finite() || !price_hi.is_finite() {
        return Err(FinanceError::Unsolvable {
            message: "model price non-finite during IV solve",
        });
    }
    if market_price < price_lo - 1e-10 {
        return Err(FinanceError::Unsolvable {
            message: "market_price below zero-vol price",
        });
    }
    if market_price > price_hi + 1e-8 {
        return Err(FinanceError::Unsolvable {
            message: "market_price above max-vol price in search range",
        });
    }

    let mut sigma = 0.25_f64.max(lo).min(hi);
    for _ in 0..30 {
        let price = price_at(sigma);
        let diff = price - market_price;
        if diff.abs() < 1e-10 {
            return Ok(sigma);
        }
        let vega = vega_at(sigma);
        if vega < 1e-14 {
            break;
        }
        let next = sigma - diff / vega;
        sigma = next.clamp(lo, hi);
    }

    // Brent on residual price(σ) - market (sign change: price_lo - mkt <= 0 <= price_hi - mkt
    // for calls/puts with monotone vol under usual conditions).
    let target = market_price;
    crate::util::root_find::brent_root(lo, hi, |s| price_at(s) - target, 1e-12, 100)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::derivatives::black_scholes::bsm_price;

    #[test]
    fn round_trip_atm() {
        let p = BsmParams::atm_one_year(100.0, 0.05, 0.22);
        let mkt = bsm_price(p, OptionType::Call).unwrap();
        let iv = bsm_implied_vol(p, OptionType::Call, mkt).unwrap();
        assert!((iv - 0.22).abs() < 1e-6);
    }

    #[test]
    fn put_round_trip() {
        let p = BsmParams {
            spot: 90.0,
            strike: 100.0,
            time_years: 0.75,
            rate: 0.01,
            dividend_yield: 0.02,
            vol: 0.35,
        };
        let mkt = bsm_price(p, OptionType::Put).unwrap();
        let iv = bsm_implied_vol(p, OptionType::Put, mkt).unwrap();
        assert!((iv - 0.35).abs() < 1e-6);
    }
}
