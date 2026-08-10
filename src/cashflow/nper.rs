//! **Number of periods with payments (NPER).** How many periods for an annuity cashflow to grow from
//! a present value to a future value at a periodic rate?
//!
//! Excel / Google Sheets equivalent: `NPER`.
//!
//! # Error handling (v0.1+)
//!
//! All entry points return [`FinanceResult`]. Invalid rates, non-negative payments (Excel sign
//! convention), or unsolvable combinations yield [`FinanceError`] — they do not panic.
use crate::util::error::{require_finite, require_rate, FinanceError, FinanceResult};

/// Returns the number of periods for an annuity (payments) to reach a future value.
///
/// Related functions:
/// * [`nper_solution`] – same calculation with a solution struct
/// * [`nper_due`] – payments due at the beginning of each period
///
/// Formula (end-of-period payments):
///
/// ```text
/// n = ln( (pmt - fv * r) / (pmt + pv * r) ) / ln(1 + r)
/// ```
///
/// # Arguments
/// * `periodic_rate` – growth rate per period (e.g. `0.05` for 5%)
/// * `payment` – payment per period; **must be negative** (Excel convention) when PV/FV are ≥ 0
/// * `present_value` – present value (≥ 0 in the Excel-style sign convention used here)
/// * `future_value` – future value (≥ 0); at least one of PV/FV must be nonzero
///
/// # Errors
/// Returns [`FinanceError`] when rate/payment/PV/FV cannot produce a finite period count.
///
/// # Examples
/// ```
/// use finance_solution::{nper, FinanceError};
///
/// let n = nper(0.034, -500.0, 1000.0, 20_000.0).unwrap();
/// assert!((n - 27.7879559).abs() < 1e-4);
///
/// assert!(matches!(
///     nper(0.05, 100.0, 0.0, 1000.0),
///     Err(FinanceError::InvalidCashflow { .. })
/// ));
/// ```
pub fn nper<C, P, F>(
    periodic_rate: f64,
    payment: C,
    present_value: P,
    future_value: F,
) -> FinanceResult<f64>
where
    C: Into<f64> + Copy,
    P: Into<f64> + Copy,
    F: Into<f64> + Copy,
{
    Ok(nper_solution(periodic_rate, payment, present_value, future_value)?.periods)
}

/// [`nper`] with a solution struct (formula string + inputs).
///
/// # Errors
/// Same domain rules as [`nper`].
pub fn nper_solution<C, P, F>(
    periodic_rate: f64,
    payment: C,
    present_value: P,
    future_value: F,
) -> FinanceResult<NperSolution>
where
    C: Into<f64> + Copy,
    P: Into<f64> + Copy,
    F: Into<f64> + Copy,
{
    nper_solution_internal(
        periodic_rate,
        payment.into(),
        present_value.into(),
        future_value.into(),
        false,
    )
}

/// Number of periods when payments are due at the **beginning** of each period (Excel `type=1`).
///
/// # Errors
/// Same domain rules as [`nper`].
pub fn nper_due<C, P, F>(
    periodic_rate: f64,
    payment: C,
    present_value: P,
    future_value: F,
) -> FinanceResult<f64>
where
    C: Into<f64> + Copy,
    P: Into<f64> + Copy,
    F: Into<f64> + Copy,
{
    Ok(nper_due_solution(periodic_rate, payment, present_value, future_value)?.periods)
}

/// [`nper_due`] with a solution struct.
///
/// # Errors
/// Same domain rules as [`nper`].
pub fn nper_due_solution<C, P, F>(
    periodic_rate: f64,
    payment: C,
    present_value: P,
    future_value: F,
) -> FinanceResult<NperSolution>
where
    C: Into<f64> + Copy,
    P: Into<f64> + Copy,
    F: Into<f64> + Copy,
{
    nper_solution_internal(
        periodic_rate,
        payment.into(),
        present_value.into(),
        future_value.into(),
        true,
    )
}

fn nper_solution_internal(
    periodic_rate: f64,
    payment: f64,
    present_value: f64,
    future_value: f64,
    due_at_beginning: bool,
) -> FinanceResult<NperSolution> {
    require_rate(periodic_rate)?;
    require_finite("payment", payment)?;
    require_finite("present_value", present_value)?;
    require_finite("future_value", future_value)?;

    if present_value < 0.0 || future_value < 0.0 {
        return Err(FinanceError::InvalidCashflow {
            message: "nper expects present_value and future_value >= 0 (Excel-style)",
        });
    }
    if present_value + future_value <= 0.0 {
        return Err(FinanceError::InvalidCashflow {
            message: "either present_value and/or future_value must be greater than 0",
        });
    }
    if payment >= 0.0 {
        return Err(FinanceError::InvalidCashflow {
            message: "payment must be negative (Excel / Google Sheets convention)",
        });
    }
    if periodic_rate == -1.0 {
        return Err(FinanceError::InvalidRate {
            rate: periodic_rate,
        });
    }

    // For annuity-due, Excel adjusts by treating the rate factor on the payment side.
    // Standard approach: nper_due uses the same formula with an adjusted present value
    // relationship. We use:
    //   n = ln((pmt - fv*r) / (pmt + pv*r)) / ln(1+r)   for type=0
    // For type=1 (due), the closed form is:
    //   n = ln((pmt - fv*r) / (pmt + pv*r + pmt*r)) / ln(1+r)  ... when r != 0
    // which is equivalent to dividing the payment-side by shifting interest on first period.

    let (num_periods, formula) = if periodic_rate == 0.0 {
        // No interest: periods = -(pv + fv) / pmt
        let n = -(present_value + future_value) / payment;
        if !n.is_finite() || n < 0.0 {
            return Err(FinanceError::Unsolvable {
                message: "nper with zero rate produced a non-finite or negative result",
            });
        }
        let formula = format!("-({} + {}) / {}", present_value, future_value, payment);
        (n, formula)
    } else {
        let (numer, denom_inner) = if due_at_beginning {
            let pmt_adj = payment * (1.0 + periodic_rate);
            (
                pmt_adj - future_value * periodic_rate,
                pmt_adj + present_value * periodic_rate,
            )
        } else {
            (
                payment - future_value * periodic_rate,
                payment + present_value * periodic_rate,
            )
        };

        if denom_inner == 0.0 || numer / denom_inner <= 0.0 {
            return Err(FinanceError::Unsolvable {
                message: "nper arguments do not admit a real solution (check signs and magnitudes)",
            });
        }

        let ratio = numer / denom_inner;
        let n = ratio.ln() / (1.0 + periodic_rate).ln();
        if !n.is_finite() || n < 0.0 {
            return Err(FinanceError::Unsolvable {
                message: "nper produced a non-finite or negative period count",
            });
        }

        let formula = if due_at_beginning {
            format!(
                "ln(({}*(1+{}) - {}*{}) / ({}*(1+{}) + {}*{})) / ln(1 + {})",
                payment,
                periodic_rate,
                future_value,
                periodic_rate,
                payment,
                periodic_rate,
                present_value,
                periodic_rate,
                periodic_rate
            )
        } else {
            format!(
                "ln(({} - {}*{}) / ({} + {}*{})) / ln(1 + {})",
                payment,
                future_value,
                periodic_rate,
                payment,
                present_value,
                periodic_rate,
                periodic_rate
            )
        };
        (n, formula)
    };

    Ok(NperSolution::new(
        periodic_rate,
        num_periods,
        payment,
        present_value,
        future_value,
        due_at_beginning,
        formula,
    ))
}

/// Solution struct for an NPER calculation.
#[derive(Debug, Clone)]
pub struct NperSolution {
    pub periodic_rate: f64,
    pub periods: f64,
    pub payment: f64,
    pub present_value_total: f64,
    pub future_value_total: f64,
    pub due_at_beginning: bool,
    pub formula: String,
}

impl NperSolution {
    pub fn new(
        periodic_rate: f64,
        periods: f64,
        payment: f64,
        present_value_total: f64,
        future_value_total: f64,
        due_at_beginning: bool,
        formula: String,
    ) -> Self {
        Self {
            periodic_rate,
            periods,
            payment,
            present_value_total,
            future_value_total,
            due_at_beginning,
            formula,
        }
    }

    pub fn periods(&self) -> f64 {
        self.periods
    }

    pub fn formula(&self) -> &str {
        &self.formula
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_approx_equal, round_6};

    #[test]
    fn test_nper_excel_values() {
        assert_eq!(
            round_6(27.7879559),
            round_6(nper(0.034, -500, 1000, 20_000).unwrap())
        );
        assert_eq!(
            round_6(59.76100743),
            round_6(nper(0.034, -50, 1000, 2_000).unwrap())
        );
        assert_eq!(
            round_6(25.68169193),
            round_6(nper(0.034, -50, 0, 2_000).unwrap())
        );
        assert_eq!(
            round_6(80.18661533),
            round_6(nper(0.034, -5, 0, 2_000).unwrap())
        );
        assert_eq!(
            round_6(106.3368288),
            round_6(nper(0.034, -200, 0, 200_000).unwrap())
        );
    }

    #[test]
    fn test_nper_zero_rate() {
        assert_approx_equal!(nper(0.0, -100.0, 0.0, 1000.0).unwrap(), 10.0);
    }

    #[test]
    fn test_nper_due_less_or_equal_end() {
        let end = nper(0.05, -100.0, 0.0, 1000.0).unwrap();
        let due = nper_due(0.05, -100.0, 0.0, 1000.0).unwrap();
        assert!(due <= end);
    }

    #[test]
    fn test_nper_rejects_positive_payment() {
        assert!(nper(0.05, 100.0, 0.0, 1000.0).is_err());
    }

    #[test]
    fn test_nper_err_rate_inf() {
        assert!(nper(1_f64 / 0_f64, -500, 1000, 20_000).is_err());
    }

    #[test]
    fn test_nper_err_positive_payment() {
        assert!(nper(0.034, 500, 1000, 20_000).is_err());
    }

    #[test]
    fn test_nper_err_zero_payment() {
        assert!(nper(0.034, 0, 1000, 20_000).is_err());
    }
}
