//! Error types and validation for finance calculations.
//!
//! # API contract (v0.1+)
//!
//! All **public** financial computations return [`FinanceResult`] — that is,
//! `Result<T, FinanceError>`. Invalid rates, non-finite amounts, empty series, and
//! similar domain failures are **values**, not panics.
//!
//! Compose with `?` or `match` on variants for field-level recovery (e.g. highlight
//! only the rate widget in a UI).
//!
//! ```
//! use finance_solution::{future_value, FinanceError, FinanceResult};
//!
//! fn project(pv: f64, years: u32) -> FinanceResult<f64> {
//!     future_value(0.07, years, pv, false)
//! }
//!
//! assert!(project(-5_000.0, 5).is_ok());
//!
//! match future_value(-1.5, 10, 1_000.0, false) {
//!     Err(FinanceError::InvalidRate { rate }) => assert!(rate < -1.0),
//!     other => panic!("expected InvalidRate, got {other:?}"),
//! }
//! ```
//!
//! # Why structured errors
//!
//! 1. **Safe composition** — handlers and batch jobs can skip one bad input without aborting.
//! 2. **Matchable variants** — `InvalidRate` vs `NonFinite` vs `EmptyInput` for metrics and UX.
//! 3. **`Display` + `Error` + [`code`](FinanceError::code)** — logs, `?` into app error types, telemetry keys.
//! 4. **Same formulas** — success paths match the historical math; only failure mode changed.
//!
//! # Stable codes
//!
//! [`FinanceError::code`] returns a snake_case token suitable for metrics (e.g. `"invalid_rate"`).
use std::fmt;

/// Result alias for fallible finance functions.
///
/// Equivalent to `Result<T, FinanceError>`. Prefer this in public signatures.
///
/// # Examples
/// ```
/// use finance_solution::{future_value, FinanceResult};
///
/// fn grow(pv: f64) -> FinanceResult<f64> {
///     future_value(0.05, 10, pv, false)
/// }
///
/// assert!(grow(-1_000.0).is_ok());
/// assert!(grow(f64::NAN).is_err());
/// ```
pub type FinanceResult<T> = Result<T, FinanceError>;

/// Domain and input errors from finance calculations.
///
/// Marked `non_exhaustive` so new variants can appear in minor releases without
/// breaking downstream `match` expressions that include a wildcard arm.
///
/// # Examples
///
/// Pattern-match for recovery or user-facing messages:
///
/// ```
/// use finance_solution::{payment, FinanceError};
///
/// match payment(-1.5, 36, 10_000.0, 0.0, false) {
///     Ok(fv) => println!("fv = {fv}"),
///     Err(FinanceError::InvalidRate { rate }) => {
///         assert!(rate < -1.0);
///         println!("bad rate: {rate} (code={})", FinanceError::InvalidRate { rate }.code());
///     }
///     Err(FinanceError::NonFinite { field, value }) => {
///         println!("{field} was non-finite ({value})");
///     }
///     Err(e) => println!("other finance error: {e}"),
/// }
/// ```
///
/// Propagate with `?`:
///
/// ```
/// use finance_solution::{present_value, future_value, FinanceResult};
///
/// fn round_trip(rate: f64, n: u32, fv: f64) -> FinanceResult<f64> {
///     let pv = present_value(rate, n, fv, false)?;
///     future_value(rate, n, pv, false)
/// }
///
/// let back = round_trip(0.04, 5, 10_000.0).unwrap();
/// assert!((back.abs() - 10_000.0).abs() < 1e-6);
/// assert!(round_trip(-2.0, 5, 10_000.0).is_err());
/// ```
///
/// Zero-value failure (present value of a zero future value is undefined):
///
/// ```
/// use finance_solution::{present_value, FinanceError};
///
/// match present_value(0.05, 10, 0.0, false) {
///     Err(FinanceError::ZeroValue { field }) => assert_eq!(field, "future_value"),
///     other => panic!("expected ZeroValue, got {other:?}"),
/// }
/// ```
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq)]
pub enum FinanceError {
    /// A numeric field was NaN or infinite.
    NonFinite { field: &'static str, value: f64 },
    /// Periodic rate outside the allowed domain for the formula (typically `< -1.0` or `<= -1.0`).
    InvalidRate { rate: f64 },
    /// Period index or count is out of range for the calculation.
    InvalidPeriod {
        period: u32,
        periods: u32,
        message: &'static str,
    },
    /// A required money amount was zero (or subnormal) when a nonzero value is required.
    ZeroValue { field: &'static str },
    /// Present and future value have the same sign when opposite signs are required.
    SameSignValues {
        present_value: f64,
        future_value: f64,
    },
    /// Inputs make the equation unsolvable (e.g. zero periods with nonzero cash difference).
    Unsolvable { message: &'static str },
    /// Cashflow / payment constraint violated (sign, missing values, etc.).
    InvalidCashflow { message: &'static str },
    /// A required collection or series was empty.
    EmptyInput { what: &'static str },
    /// Two series or slices that must align have different lengths.
    LengthMismatch {
        left: usize,
        right: usize,
        context: &'static str,
    },
}

impl FinanceError {
    /// Stable snake_case code for logs and metrics (not localized).
    pub fn code(&self) -> &'static str {
        match self {
            FinanceError::NonFinite { .. } => "non_finite",
            FinanceError::InvalidRate { .. } => "invalid_rate",
            FinanceError::InvalidPeriod { .. } => "invalid_period",
            FinanceError::ZeroValue { .. } => "zero_value",
            FinanceError::SameSignValues { .. } => "same_sign_values",
            FinanceError::Unsolvable { .. } => "unsolvable",
            FinanceError::InvalidCashflow { .. } => "invalid_cashflow",
            FinanceError::EmptyInput { .. } => "empty_input",
            FinanceError::LengthMismatch { .. } => "length_mismatch",
        }
    }
}

impl fmt::Display for FinanceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FinanceError::NonFinite { field, value } => {
                write!(
                    f,
                    "{field} must be finite (not NaN or infinity); got {value}"
                )
            }
            FinanceError::InvalidRate { rate } => {
                write!(
                    f,
                    "rate is outside the allowed domain for this formula; got {rate}"
                )
            }
            FinanceError::InvalidPeriod {
                period,
                periods,
                message,
            } => {
                write!(f, "{message} (period={period}, periods={periods})")
            }
            FinanceError::ZeroValue { field } => {
                write!(f, "{field} must be nonzero for this calculation")
            }
            FinanceError::SameSignValues {
                present_value,
                future_value,
            } => {
                write!(
                    f,
                    "present_value ({present_value}) and future_value ({future_value}) must have opposite signs"
                )
            }
            FinanceError::Unsolvable { message } => write!(f, "{message}"),
            FinanceError::InvalidCashflow { message } => write!(f, "{message}"),
            FinanceError::EmptyInput { what } => write!(f, "{what} must not be empty"),
            FinanceError::LengthMismatch {
                left,
                right,
                context,
            } => {
                write!(f, "{context}: length mismatch ({left} vs {right})")
            }
        }
    }
}

impl std::error::Error for FinanceError {}

// ---------------------------------------------------------------------------
// Validators (compose with ?)
// ---------------------------------------------------------------------------

/// Ensure a value is finite.
pub(crate) fn require_finite(field: &'static str, value: f64) -> FinanceResult<()> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(FinanceError::NonFinite { field, value })
    }
}

/// Ensure rate is finite and `>= -1.0` (TVM simple/continuous formulas).
pub(crate) fn require_rate(rate: f64) -> FinanceResult<()> {
    require_finite("rate", rate)?;
    if rate < -1.0 {
        Err(FinanceError::InvalidRate { rate })
    } else {
        Ok(())
    }
}

/// Ensure rate is finite and strictly greater than -1.0 (payment / annuity formulas).
pub(crate) fn require_rate_gt_minus_one(rate: f64) -> FinanceResult<()> {
    require_finite("rate", rate)?;
    if rate <= -1.0 {
        Err(FinanceError::InvalidRate { rate })
    } else {
        Ok(())
    }
}

/// Ensure a money-like amount is finite (present value, future value, payment, etc.).
pub(crate) fn require_money(field: &'static str, value: f64) -> FinanceResult<()> {
    require_finite(field, value)
}

/// Ensure value is finite and strictly positive (prices for log returns, etc.).
pub(crate) fn require_positive(field: &'static str, value: f64) -> FinanceResult<()> {
    require_finite(field, value)?;
    if value <= 0.0 {
        Err(FinanceError::InvalidCashflow {
            message: "value must be strictly positive",
        })
    } else {
        Ok(())
    }
}

/// Ensure a slice is non-empty.
pub(crate) fn require_nonempty<T>(what: &'static str, items: &[T]) -> FinanceResult<()> {
    if items.is_empty() {
        Err(FinanceError::EmptyInput { what })
    } else {
        Ok(())
    }
}

/// Ensure every rate in a schedule is valid for TVM (`>= -1.0`).
///
/// Empty schedules are allowed (zero periods → no compounding). Call
/// [`require_nonempty`] first when empty input is an error (e.g. return series).
pub(crate) fn require_rates(rates: &[f64]) -> FinanceResult<()> {
    for &rate in rates {
        require_rate(rate)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::future_value;

    #[test]
    fn display_and_code_invalid_rate() {
        let err = FinanceError::InvalidRate { rate: -1.5 };
        assert!(err.to_string().contains("-1.5"));
        assert_eq!(err.code(), "invalid_rate");
    }

    #[test]
    fn future_value_err_invalid_rate() {
        match future_value(-1.5, 12, 1000.0, false) {
            Err(FinanceError::InvalidRate { rate }) => assert_eq!(rate, -1.5),
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn require_rates_empty_ok() {
        assert!(require_rates(&[]).is_ok());
    }

    #[test]
    fn require_rates_invalid() {
        assert!(matches!(
            require_rates(&[-1.5]),
            Err(FinanceError::InvalidRate { .. })
        ));
    }
}
