//! Shared option types and BSM parameter packs.
//!
//! ## Trading vs engineering
//!
//! - **Trading:** [`OptionType`] is the side of the contract; [`BsmParams`] is the
//!   *market + contract* snapshot you reprice (spot from the tape, strike/expiry fixed,
//!   vol from your surface or from mid via IV).
//! - **Engineering:** keep [`BsmParams`] as a plain `Copy` struct so configs and message
//!   handlers stay allocation-free; validate once with [`ValidatedBsm::new`] or store a
//!   [`crate::derivatives::BsmState`] per contract key.

use crate::util::error::{require_finite, FinanceError, FinanceResult};
use std::fmt;

/// Call or put (European exercise in this module).
///
/// **Trading:** long call = bullish / long convexity; long put = bearish / hedge inventory.  
/// **Engineering:** store next to strike/expiry in your contract id; pass into every price/greeks call.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OptionType {
    Call,
    Put,
}

impl OptionType {
    pub fn is_call(self) -> bool {
        matches!(self, OptionType::Call)
    }

    pub fn is_put(self) -> bool {
        matches!(self, OptionType::Put)
    }
}

impl fmt::Display for OptionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OptionType::Call => write!(f, "Call"),
            OptionType::Put => write!(f, "Put"),
        }
    }
}

/// Black–Scholes–Merton inputs (European, continuous dividend yield `q`).
///
/// # Field meanings (trading)
///
/// | Field | Trading meaning |
/// |-------|-----------------|
/// | `spot` | Underlier mid / last (or your mark) |
/// | `strike` | Option strike |
/// | `time_years` | Fraction of year to expiry (day-count is **your** policy) |
/// | `rate` | Continuous funding / risk-free input to the model |
/// | `dividend_yield` | Continuous yield `q` (dividends, or borrow approximation) |
/// | `vol` | Annualized σ — model input or **implied** from market |
///
/// # Examples
/// ```
/// use finance_solution::derivatives::BsmParams;
/// let p = BsmParams::atm_one_year(100.0, 0.05, 0.20);
/// assert_eq!(p.spot, p.strike);
/// assert_eq!(p.time_years, 1.0);
/// ```
///
/// Convert calendar days with an explicit day-count (engine responsibility):
/// ```
/// use finance_solution::derivatives::BsmParams;
/// let days = 21.0;
/// let p = BsmParams {
///     spot: 50.0,
///     strike: 55.0,
///     time_years: days / 365.25,
///     rate: 0.04,
///     dividend_yield: 0.01,
///     vol: 0.30,
/// };
/// assert!(p.time_years < 0.1);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BsmParams {
    pub spot: f64,
    pub strike: f64,
    /// Time to expiry in **years**.
    pub time_years: f64,
    /// Continuous risk-free rate.
    pub rate: f64,
    /// Continuous dividend yield (equity) or foreign-rate analog in FX-style setups.
    pub dividend_yield: f64,
    /// Annualized volatility (absolute).
    pub vol: f64,
}

impl BsmParams {
    /// ATM, one year, zero dividend yield — convenient textbook / smoke fixture.
    pub const fn atm_one_year(spot: f64, rate: f64, vol: f64) -> Self {
        Self {
            spot,
            strike: spot,
            time_years: 1.0,
            rate,
            dividend_yield: 0.0,
            vol,
        }
    }

    /// Build from calendar days using **Actual/365.25** style (`days / 365.25`).
    ///
    /// Desks differ (business days, 365, 252). Prefer this helper only when that
    /// convention is intentional; otherwise set `time_years` yourself.
    pub fn with_days_365_25(
        spot: f64,
        strike: f64,
        days: f64,
        rate: f64,
        dividend_yield: f64,
        vol: f64,
    ) -> Self {
        Self {
            spot,
            strike,
            time_years: days / 365.25,
            rate,
            dividend_yield,
            vol,
        }
    }
}

/// Validated BSM pack (strictly positive S,K; non-negative T,σ; finite rates).
///
/// **Engineering:** construct once per “clean” snapshot; use on hot paths so validation
/// is not mixed into every formula line. For mutable live fields prefer
/// [`crate::derivatives::BsmState`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ValidatedBsm {
    params: BsmParams,
}

impl ValidatedBsm {
    /// Fallible constructor — Result-only style (`new`, not `try_new`).
    ///
    /// # Errors
    /// Non-finite inputs; non-positive spot/strike; negative time or vol.
    pub fn new(params: BsmParams) -> FinanceResult<Self> {
        validate_bsm_params(params)?;
        Ok(Self { params })
    }

    pub fn params(self) -> BsmParams {
        self.params
    }

    /// Model price for this validated snapshot.
    pub fn price(self, option_type: OptionType) -> FinanceResult<f64> {
        crate::derivatives::black_scholes::bsm_price(self.params, option_type)
    }

    /// Model Greeks for this validated snapshot.
    pub fn greeks(
        self,
        option_type: OptionType,
    ) -> FinanceResult<crate::derivatives::black_scholes::BsmGreeks> {
        crate::derivatives::black_scholes::bsm_greeks(self.params, option_type)
    }

    /// Cross Greeks (vanna, volga, charm).
    pub fn cross_greeks(
        self,
        option_type: OptionType,
    ) -> FinanceResult<crate::derivatives::black_scholes::BsmCrossGreeks> {
        crate::derivatives::black_scholes::bsm_cross_greeks(self.params, option_type)
    }
}

pub(crate) fn validate_bsm_params(p: BsmParams) -> FinanceResult<()> {
    require_finite("spot", p.spot)?;
    require_finite("strike", p.strike)?;
    require_finite("time_years", p.time_years)?;
    require_finite("rate", p.rate)?;
    require_finite("dividend_yield", p.dividend_yield)?;
    require_finite("vol", p.vol)?;
    if p.spot <= 0.0 {
        return Err(FinanceError::InvalidCashflow {
            message: "spot must be strictly positive",
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

/// Intrinsic value (European exercise value at this spot).
///
/// **Trading:** the “dead” part of the premium if exercised now (European still
/// cannot early-exercise, but intrinsic is the mental floor).  
/// **Engineering:** useful for UI columns and for rejecting IV solves below floor at T=0.
pub fn intrinsic(spot: f64, strike: f64, option_type: OptionType) -> FinanceResult<f64> {
    require_finite("spot", spot)?;
    require_finite("strike", strike)?;
    if spot <= 0.0 || strike <= 0.0 {
        return Err(FinanceError::InvalidCashflow {
            message: "spot and strike must be strictly positive",
        });
    }
    Ok(match option_type {
        OptionType::Call => (spot - strike).max(0.0),
        OptionType::Put => (strike - spot).max(0.0),
    })
}

/// Time value = premium − intrinsic (floored at 0 for numerical noise).
///
/// **Trading:** what you pay for optionality / vol.  
/// **Engineering:** `premium` may be model or market mid — you choose.
pub fn time_value(
    premium: f64,
    spot: f64,
    strike: f64,
    option_type: OptionType,
) -> FinanceResult<f64> {
    require_finite("premium", premium)?;
    let i = intrinsic(spot, strike, option_type)?;
    Ok((premium - i).max(0.0))
}

/// Forward moneyness `S e^{(r-q)T} / K`.
///
/// **Trading:** >1 call is ITM on a forward basis; skew is often quoted vs this.  
/// **Engineering:** pure function of [`BsmParams`]; no vol dependence.
pub fn forward_moneyness(p: BsmParams) -> FinanceResult<f64> {
    validate_bsm_params(p)?;
    let f = p.spot * ((p.rate - p.dividend_yield) * p.time_years).exp();
    Ok(f / p.strike)
}

/// Spot moneyness `S / K` (not forward-adjusted).
pub fn spot_moneyness(spot: f64, strike: f64) -> FinanceResult<f64> {
    require_finite("spot", spot)?;
    require_finite("strike", strike)?;
    if spot <= 0.0 || strike <= 0.0 {
        return Err(FinanceError::InvalidCashflow {
            message: "spot and strike must be strictly positive",
        });
    }
    Ok(spot / strike)
}
