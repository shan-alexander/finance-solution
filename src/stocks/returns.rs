//! Simple and logarithmic returns from prices.
//!
//! # Error handling (v0.1+)
//!
//! All public functions return [`FinanceResult`]. Empty series, zero start prices, and
//! non-positive prices for log/CAGR paths are structured errors.
use crate::util::error::{require_finite, FinanceError, FinanceResult};

/// Simple return between two prices: `(p1 - p0) / p0`.
///
/// # Examples
/// ```
/// use finance_solution::{simple_return, FinanceError};
///
/// assert!(simple_return(100.0, 110.0).is_ok());
/// match simple_return(0.0, 110.0) {
///     Err(FinanceError::ZeroValue { field }) => assert_eq!(field, "price_start"),
///     other => panic!("expected ZeroValue, got {other:?}"),
/// }
/// ```
pub fn simple_return(price_start: f64, price_end: f64) -> FinanceResult<f64> {
    require_finite("price_start", price_start)?;
    require_finite("price_end", price_end)?;
    if price_start == 0.0 {
        return Err(FinanceError::ZeroValue {
            field: "price_start",
        });
    }
    Ok((price_end - price_start) / price_start)
}

/// Logarithmic return: `ln(p1 / p0)`. Requires strictly positive prices.
///
/// # Examples
/// ```
/// use finance_solution::{log_return, FinanceError};
///
/// assert!(log_return(100.0, 110.0).is_ok());
/// match log_return(-1.0, 110.0) {
///     Err(FinanceError::InvalidCashflow { message }) => {
///         assert!(message.contains("positive"));
///     }
///     other => panic!("expected InvalidCashflow, got {other:?}"),
/// }
/// ```
pub fn log_return(price_start: f64, price_end: f64) -> FinanceResult<f64> {
    require_finite("price_start", price_start)?;
    require_finite("price_end", price_end)?;
    if price_start <= 0.0 || price_end <= 0.0 {
        return Err(FinanceError::InvalidCashflow {
            message: "log_return requires strictly positive prices",
        });
    }
    Ok((price_end / price_start).ln())
}

/// Simple returns for consecutive prices: length `prices.len() - 1`.
///
/// # Examples
/// ```
/// use finance_solution::{simple_returns, FinanceError};
///
/// assert_eq!(simple_returns(&[100.0, 110.0]).unwrap().len(), 1);
/// match simple_returns(&[100.0]) {
///     Err(FinanceError::Unsolvable { message }) => assert!(message.contains("two")),
///     other => panic!("expected Unsolvable, got {other:?}"),
/// }
/// ```
pub fn simple_returns(prices: &[f64]) -> FinanceResult<Vec<f64>> {
    if prices.len() < 2 {
        return Err(FinanceError::Unsolvable {
            message: "simple_returns requires at least two prices",
        });
    }
    let mut out = Vec::with_capacity(prices.len() - 1);
    for window in prices.windows(2) {
        out.push(simple_return(window[0], window[1])?);
    }
    Ok(out)
}

/// Log returns for consecutive prices: length `prices.len() - 1`.
///
/// # Examples
/// ```
/// use finance_solution::{log_returns, FinanceError};
///
/// assert!(log_returns(&[100.0, 110.0]).is_ok());
/// assert!(matches!(
///     log_returns(&[100.0, 0.0]),
///     Err(FinanceError::InvalidCashflow { .. })
/// ));
/// ```
pub fn log_returns(prices: &[f64]) -> FinanceResult<Vec<f64>> {
    if prices.len() < 2 {
        return Err(FinanceError::Unsolvable {
            message: "log_returns requires at least two prices",
        });
    }
    let mut out = Vec::with_capacity(prices.len() - 1);
    for window in prices.windows(2) {
        out.push(log_return(window[0], window[1])?);
    }
    Ok(out)
}

/// Compound annual growth rate: `(end / start)^(1/years) - 1`.
///
/// # Examples
/// ```
/// use finance_solution::{cagr, FinanceError};
///
/// assert!(cagr(100.0, 121.0, 2.0).is_ok());
/// match cagr(100.0, 121.0, 0.0) {
///     Err(FinanceError::Unsolvable { message }) => assert!(message.contains("years")),
///     other => panic!("expected Unsolvable, got {other:?}"),
/// }
/// ```
pub fn cagr(price_start: f64, price_end: f64, years: f64) -> FinanceResult<f64> {
    require_finite("price_start", price_start)?;
    require_finite("price_end", price_end)?;
    require_finite("years", years)?;
    if price_start <= 0.0 || price_end <= 0.0 {
        return Err(FinanceError::InvalidCashflow {
            message: "cagr requires strictly positive prices",
        });
    }
    if years <= 0.0 {
        return Err(FinanceError::Unsolvable {
            message: "cagr requires years > 0",
        });
    }
    Ok((price_end / price_start).powf(1.0 / years) - 1.0)
}

/// Arithmetic mean of a return series.
///
/// # Examples
/// ```
/// use finance_solution::{mean_return, FinanceError};
///
/// assert!(mean_return(&[0.01, 0.02]).is_ok());
/// match mean_return(&[]) {
///     Err(FinanceError::Unsolvable { message }) => assert!(message.contains("non-empty")),
///     other => panic!("expected Unsolvable, got {other:?}"),
/// }
/// ```
pub fn mean_return(returns: &[f64]) -> FinanceResult<f64> {
    if returns.is_empty() {
        return Err(FinanceError::Unsolvable {
            message: "mean_return requires a non-empty series",
        });
    }
    for r in returns {
        require_finite("returns", *r)?;
    }
    Ok(returns.iter().sum::<f64>() / returns.len() as f64)
}

/// Total simple return from first to last price: `(end - start) / start`.
///
/// # Examples
/// ```
/// use finance_solution::{total_return, FinanceError};
///
/// let r = total_return(100.0, 125.0).unwrap();
/// assert!((r - 0.25).abs() < 1e-12);
/// assert!(matches!(
///     total_return(0.0, 125.0),
///     Err(FinanceError::ZeroValue { .. })
/// ));
/// ```
pub fn total_return(price_start: f64, price_end: f64) -> FinanceResult<f64> {
    simple_return(price_start, price_end)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_simple_and_log_return() {
        assert_approx_equal!(simple_return(100.0, 110.0).unwrap(), 0.1);
        assert!(log_return(100.0, 110.0).unwrap() < 0.1);
        assert!(log_return(100.0, 110.0).unwrap() > 0.09);
    }

    #[test]
    fn test_returns_series() {
        let prices = [100.0, 110.0, 105.0];
        let r = simple_returns(&prices).unwrap();
        assert_eq!(r.len(), 2);
        assert_approx_equal!(r[0], 0.1);
    }

    #[test]
    fn test_cagr() {
        assert_approx_equal!(cagr(100.0, 121.0, 2.0).unwrap(), 0.1);
    }

    #[test]
    fn test_round_trip_product() {
        let prices = [100.0, 110.0, 99.0, 108.9];
        let rets = simple_returns(&prices).unwrap();
        let mut wealth = 1.0;
        for r in rets {
            wealth *= 1.0 + r;
        }
        assert_approx_equal!(wealth, prices[prices.len() - 1] / prices[0]);
    }
}
