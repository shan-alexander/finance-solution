//! Domain **newtypes** — zero-cost wrappers that make invalid values harder to pass by accident.
//!
//! # Design (see `rust_design_patterns` Newtype)
//!
//! - Private fields; construction is fallible ([`TryFrom`] / associated `try_` constructors).
//! - **Not** type aliases: `Rate` is not interchangeable with bare `f64` at the type level.
//! - **Additive for 0.1.x / 0.2:** existing free functions still take `f64` / `u32`. Newtypes
//!   are for call sites and TA config that want compile-time clarity + one-time validation.
//! - Extract with [`.get()`](Rate::get) / [`Into<f64>`] when calling f64-based APIs.
//!
//! # When to use
//!
//! | Type | Prefer when |
//! |------|-------------|
//! | [`Rate`] | You validated a rate once and pass it through several calls |
//! | [`Periods`] | Period counts should not mix with money amounts |
//! | [`PositivePrice`] | Equity prices / volumes that must be `> 0` |
//! | [`Money`] | Finite signed amounts (loans, payments) without unit claims |
//! | [`PeriodLength`] | TA lookbacks (`SMA(20)`, stoch `k_period`) — `usize ≥ 1` |
//!
//! # Examples
//! ```
//! use finance_solution::{future_value, PositivePrice, Rate, Periods, FinanceResult};
//!
//! fn grow(rate: Rate, n: Periods, pv: f64) -> FinanceResult<f64> {
//!     future_value(rate.get(), n.get(), pv, false)
//! }
//!
//! let r = Rate::tvm(0.05)?;
//! let n = Periods::new(10)?;
//! assert!(grow(r, n, -1_000.0).is_ok());
//! assert!(Rate::tvm(-1.5).is_err());
//! assert!(PositivePrice::new(0.0).is_err());
//! # Ok::<(), finance_solution::FinanceError>(())
//! ```

use crate::util::error::{
    require_finite, require_rate, require_rate_gt_minus_one, FinanceError, FinanceResult,
};
use std::fmt;

// ---------------------------------------------------------------------------
// Rate
// ---------------------------------------------------------------------------

/// Periodic or continuous **interest / return rate** as a decimal (e.g. `0.05` = 5%).
///
/// Domain depends on constructor: [`Rate::tvm`] (`≥ -1`), [`Rate::payment`] (`> -1`),
/// [`Rate::positive`] (`> 0`), [`Rate::finite`] (any finite).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Rate(f64);

impl Rate {
    /// Any finite rate (no lower bound). Prefer domain-specific constructors for TVM.
    pub fn finite(value: f64) -> FinanceResult<Self> {
        require_finite("rate", value)?;
        Ok(Rate(value))
    }

    /// TVM-compatible rate: finite and `≥ -1.0`.
    pub fn tvm(value: f64) -> FinanceResult<Self> {
        require_rate(value)?;
        Ok(Rate(value))
    }

    /// Payment / annuity rate: finite and `> -1.0`.
    pub fn payment(value: f64) -> FinanceResult<Self> {
        require_rate_gt_minus_one(value)?;
        Ok(Rate(value))
    }

    /// Strictly positive finite rate (doubling rules, growth rates).
    pub fn positive(value: f64) -> FinanceResult<Self> {
        require_finite("rate", value)?;
        if value == 0.0 {
            return Err(FinanceError::ZeroValue { field: "rate" });
        }
        if value < 0.0 {
            return Err(FinanceError::InvalidRate { rate: value });
        }
        Ok(Rate(value))
    }

    /// Inner `f64` (by value; type is `Copy`).
    #[inline]
    pub fn get(self) -> f64 {
        self.0
    }
}

impl From<Rate> for f64 {
    #[inline]
    fn from(r: Rate) -> f64 {
        r.0
    }
}

impl TryFrom<f64> for Rate {
    type Error = FinanceError;
    /// Defaults to [`Rate::tvm`] (most common finance domain).
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Rate::tvm(value)
    }
}

impl fmt::Display for Rate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ---------------------------------------------------------------------------
// Periods (TVM u32)
// ---------------------------------------------------------------------------

/// Count of compounding / payment **periods** (`u32`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Periods(u32);

impl Periods {
    /// Any period count (including zero where formulas allow).
    pub fn new(value: u32) -> FinanceResult<Self> {
        Ok(Periods(value))
    }

    /// At least one period (annuities, many TA windows expressed as `u32`).
    pub fn at_least_one(value: u32) -> FinanceResult<Self> {
        if value == 0 {
            return Err(FinanceError::InvalidPeriod {
                period: 0,
                periods: 0,
                message: "periods must be at least 1",
            });
        }
        Ok(Periods(value))
    }

    #[inline]
    pub fn get(self) -> u32 {
        self.0
    }
}

impl From<Periods> for u32 {
    #[inline]
    fn from(p: Periods) -> u32 {
        p.0
    }
}

impl TryFrom<u32> for Periods {
    type Error = FinanceError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Periods::new(value)
    }
}

impl fmt::Display for Periods {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ---------------------------------------------------------------------------
// PeriodLength (TA usize lookback)
// ---------------------------------------------------------------------------

/// Positive lookback / window length for technical indicators (`usize ≥ 1`).
///
/// Distinct from [`Periods`] (`u32` TVM counts) so TA windows do not silently mix with NPER.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PeriodLength(usize);

impl PeriodLength {
    /// Fallible constructor: `n >= 1`.
    pub fn new(n: usize) -> FinanceResult<Self> {
        if n == 0 {
            return Err(FinanceError::InvalidPeriod {
                period: 0,
                periods: 0,
                message: "period length must be at least 1",
            });
        }
        Ok(PeriodLength(n))
    }

    /// `const` constructor for **known-valid** compile-time windows (e.g. `20` for SMA-20).
    ///
    /// Panics in debug if `n == 0`; release builds still store `0` — prefer [`PeriodLength::new`]
    /// for runtime input. For `const` presets, only pass literals `≥ 1`.
    pub const fn new_const(n: usize) -> Self {
        assert!(n >= 1, "PeriodLength::new_const requires n >= 1");
        PeriodLength(n)
    }

    #[inline]
    pub const fn get(self) -> usize {
        self.0
    }
}

impl From<PeriodLength> for usize {
    #[inline]
    fn from(p: PeriodLength) -> usize {
        p.0
    }
}

impl TryFrom<usize> for PeriodLength {
    type Error = FinanceError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        PeriodLength::new(value)
    }
}

impl fmt::Display for PeriodLength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ---------------------------------------------------------------------------
// PositivePrice
// ---------------------------------------------------------------------------

/// Strictly **positive finite** price (or similar quantity used as a price level).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct PositivePrice(f64);

impl PositivePrice {
    pub fn new(value: f64) -> FinanceResult<Self> {
        require_finite("price", value)?;
        if value <= 0.0 {
            return Err(FinanceError::InvalidCashflow {
                message: "price must be strictly positive",
            });
        }
        Ok(PositivePrice(value))
    }

    #[inline]
    pub fn get(self) -> f64 {
        self.0
    }
}

impl From<PositivePrice> for f64 {
    #[inline]
    fn from(p: PositivePrice) -> f64 {
        p.0
    }
}

impl TryFrom<f64> for PositivePrice {
    type Error = FinanceError;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        PositivePrice::new(value)
    }
}

impl fmt::Display for PositivePrice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ---------------------------------------------------------------------------
// Money
// ---------------------------------------------------------------------------

/// Finite **signed** monetary amount (no currency unit — pure magnitude + sign).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Money(f64);

impl Money {
    pub fn new(value: f64) -> FinanceResult<Self> {
        require_finite("money", value)?;
        Ok(Money(value))
    }

    /// Finite and nonzero.
    pub fn nonzero(value: f64) -> FinanceResult<Self> {
        require_finite("money", value)?;
        if value == 0.0 {
            return Err(FinanceError::ZeroValue { field: "money" });
        }
        Ok(Money(value))
    }

    #[inline]
    pub fn get(self) -> f64 {
        self.0
    }
}

impl From<Money> for f64 {
    #[inline]
    fn from(m: Money) -> f64 {
        m.0
    }
}

impl TryFrom<f64> for Money {
    type Error = FinanceError;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Money::new(value)
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rate_domains() {
        assert!(Rate::tvm(-1.0).is_ok());
        assert!(Rate::tvm(-1.1).is_err());
        assert!(Rate::payment(-1.0).is_err());
        assert!(Rate::positive(0.08).is_ok());
        assert!(Rate::positive(0.0).is_err());
    }

    #[test]
    fn periods_and_length() {
        assert_eq!(Periods::new(0).unwrap().get(), 0);
        assert!(Periods::at_least_one(0).is_err());
        assert_eq!(PeriodLength::new(20).unwrap().get(), 20);
        assert!(PeriodLength::new(0).is_err());
        assert_eq!(PeriodLength::new_const(14).get(), 14);
    }

    #[test]
    fn price_and_money() {
        assert!(PositivePrice::new(100.0).is_ok());
        assert!(PositivePrice::new(0.0).is_err());
        assert!(Money::new(-50.0).is_ok());
        assert!(Money::nonzero(0.0).is_err());
    }
}
