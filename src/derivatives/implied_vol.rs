//! # Implied volatility
//!
//! Invert a European BSM **market premium** → annualized σ.
//!
//! ---
//!
//! ## Trading perspective
//!
//! The market does not quote “model vol”; it quotes **prices**. IV is the σ that makes
//! BSM match that price — the language of surfaces, skew, and “IV crush”.
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
//! - Solver: Newton–Raphson on vega, bisection fallback.  
//! - `params.vol` is **ignored** as an input seed for the root (only S,K,T,r,q, type matter).  
//! - Prefer [`crate::derivatives::BsmState::set_vol_from_price`] in live loops so IV is stored once.  
//! - Hot path cost: a handful of BSM evaluations — fine per quote; batch chains may parallelize **across strikes** in your engine.
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

/// Solve for annualized vol given a target premium.
///
/// Uses Newton–Raphson on vega with bisection fallback. `params.vol` is not used as a seed.
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

    let mut p = params;
    let intrinsic = match option_type {
        OptionType::Call => (p.spot - p.strike).max(0.0),
        OptionType::Put => (p.strike - p.spot).max(0.0),
    };
    if market_price + 1e-12 < intrinsic && p.time_years == 0.0 {
        return Err(FinanceError::Unsolvable {
            message: "market_price below intrinsic",
        });
    }

    if p.time_years == 0.0 {
        return Err(FinanceError::Unsolvable {
            message: "implied vol undefined at expiry (T=0)",
        });
    }

    let mut lo = 1e-8;
    let mut hi = 5.0;
    p.vol = hi;
    let mut price_hi = price_raw(p, option_type)?;
    let mut expand = 0;
    while price_hi < market_price && hi < 50.0 && expand < 10 {
        hi *= 2.0;
        p.vol = hi;
        price_hi = price_raw(p, option_type)?;
        expand += 1;
    }
    p.vol = lo;
    let price_lo = price_raw(p, option_type)?;
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
        p.vol = sigma;
        let price = price_raw(p, option_type)?;
        let diff = price - market_price;
        if diff.abs() < 1e-10 {
            return Ok(sigma);
        }
        let vega = vega_raw(p, option_type)?;
        if vega < 1e-14 {
            break;
        }
        let next = sigma - diff / vega;
        sigma = next.clamp(lo, hi);
    }

    for _ in 0..80 {
        let mid = 0.5 * (lo + hi);
        p.vol = mid;
        let price = price_raw(p, option_type)?;
        if (price - market_price).abs() < 1e-10 {
            return Ok(mid);
        }
        if price > market_price {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    Ok(0.5 * (lo + hi))
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
