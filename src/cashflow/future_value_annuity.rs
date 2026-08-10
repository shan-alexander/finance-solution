#![allow(unused_imports)]

//! **Future value _annuity_ calculations**. Given a series of constant cashflows, a number of periods
//! such as years, and a fixed interest rate, what is the value of the series at the final payment?
//!
//! Timing uses [`crate::PaymentTiming`] (or Excel-style `bool` via [`From`]):
//! - [`PaymentTiming::EndOfPeriod`] / `false` — ordinary annuity (Excel `type=0`)
//! - [`PaymentTiming::BeginningOfPeriod`] / `true` — annuity due (Excel `type=1`)
//!
//! Prefer the enum in new code; `bool` remains for spreadsheet parity.
//!
//! For teaching / debugging, use [`future_value_annuity_solution`], which carries formulas and
//! related PV/FV fields.
//!
//! ## Examples
//!
//! Ordinary annuity (end of period) with `bool`:
//! ```
//! use finance_solution::future_value_annuity_solution;
//! let (rate, periods, annuity, due) = (0.034, 10, 500, false);
//! let fv_ann = future_value_annuity_solution(rate, periods, annuity, due).unwrap();
//! assert!(fv_ann.future_value().abs() > 5_000.0);
//! ```
//!
//! Same scenario with the enum (preferred):
//! ```
//! use finance_solution::{future_value_annuity, PaymentTiming};
//! let ordinary = future_value_annuity(0.034, 10, 500, PaymentTiming::EndOfPeriod).unwrap();
//! let due = future_value_annuity(0.034, 10, 500, PaymentTiming::BeginningOfPeriod).unwrap();
//! // Annuity due compounds one extra period → larger magnitude than ordinary
//! assert!(due.abs() > ordinary.abs());
//! ```
//!
//! Zero-rate special case (both timings reduce to `-payment * periods`):
//! ```
//! use finance_solution::{future_value_annuity, PaymentTiming};
//! let z = future_value_annuity(0.0, 12, 100, PaymentTiming::EndOfPeriod).unwrap();
//! assert!((z - (-1_200.0)).abs() < 1e-12);
//! ```
//!

// to-do: add "use log::warn;" and helper logs

// Needed for the Rustdoc comments and module.
use crate::assert_approx_equal;
use crate::cashflow::*;
use crate::future_value::future_value;
use crate::present_value::present_value;

fn check_future_value_annuity_parameters(
    rate: f64,
    periods: u32,
    cashflow: f64,
) -> crate::FinanceResult<()> {
    crate::util::error::require_rate_gt_minus_one(rate)?;
    crate::util::error::require_money("annuity", cashflow)?;
    if periods == 0 {
        return Err(crate::FinanceError::InvalidPeriod {
            period: 0,
            periods: 0,
            message: "annuity requires at least one period",
        });
    }
    Ok(())
}

/// Returns the future value of annuity (a series of constant cashflows) at a constant rate. Returns f64.
///
/// The future value annuity formula is:
///
/// future value ann = sum( cashflow * (1 + rate)<sup>period</sup> )
///
/// or
///
/// future value ann = Constant_Cashflow * ((1+periodic_rate)^n -1) / periodic_rate
///
/// # Arguments
/// * `rate` - The rate at which the investment grows or shrinks per period,
/// expressed as a floating point number. For instance 0.05 would mean 5%. Often appears as
/// `r` or `i` in formulas.
/// * `periods` - The number of periods such as quarters or years. Often appears as `n` or `t`.
/// * `cashflow` - The value of the constant cashflow (aka payment).
/// * `timing` - [`PaymentTiming`] or `bool` (`false` = end of period / Excel `type=0`).
///
/// # Errors
/// Returns [`crate::FinanceError`] if `rate` is less than or equal to -1.0, money is non-finite,
/// or `periods` is zero.
///
/// # Examples
/// Ordinary annuity (`bool` Excel style):
/// ```
/// use finance_solution::*;
/// let my_annuity = future_value_annuity(0.034, 5, 500, false).unwrap();
/// assert_approx_equal!(my_annuity, -2_675.8789282);
/// ```
///
/// Annuity due with the enum (preferred):
/// ```
/// use finance_solution::{future_value_annuity, PaymentTiming};
/// let due = future_value_annuity(0.034, 5, 500, PaymentTiming::BeginningOfPeriod).unwrap();
/// let ordinary = future_value_annuity(0.034, 5, 500, PaymentTiming::EndOfPeriod).unwrap();
/// assert!(due.abs() > ordinary.abs());
/// ```
///
/// Solution struct (formulas + related fields):
/// ```
/// use finance_solution::*;
/// let sol = future_value_annuity_solution(0.034, 5, 500, PaymentTiming::EndOfPeriod).unwrap();
/// let final_answer = sol.future_value();
/// assert_approx_equal!(final_answer, -2_675.8789282);
/// ```
///
/// Monthly contributions:
/// ```
/// # use finance_solution::*;
/// let rate = 0.021;       // 2.1% per month
/// let periods = 12;
/// let cashflow = 2_000;
/// let future_value_ann = future_value_annuity(rate, periods, cashflow, false).unwrap();
/// assert!(future_value_ann.is_finite());
/// ```
pub fn future_value_annuity<T, D>(
    rate: f64,
    periods: u32,
    annuity: T,
    timing: D,
) -> crate::FinanceResult<f64>
where
    T: Into<f64> + Copy,
    D: Into<crate::PaymentTiming>,
{
    let pmt = annuity.into();
    let timing = timing.into();
    check_future_value_annuity_parameters(rate, periods, pmt)?;

    // Ordinary annuity (end): FV = -pmt * ((1+r)^n - 1) / r
    // Annuity due (beginning): multiply by (1 + r) — one extra period of interest on each payment.
    // Zero rate: FV = -pmt * n for both timings (limit of the geometric series).
    let fv_ann = match (rate == 0.0, timing) {
        (true, _) => -pmt * periods as f64,
        (false, crate::PaymentTiming::EndOfPeriod) => {
            -pmt * ((1.0 + rate).powf(periods as f64) - 1.0) / rate
        }
        (false, crate::PaymentTiming::BeginningOfPeriod) => {
            -pmt * (1.0 + rate) * ((1.0 + rate).powf(periods as f64) - 1.0) / rate
        }
    };
    if fv_ann.is_finite() {
        Ok(fv_ann)
    } else {
        Err(crate::FinanceError::NonFinite {
            field: "future_value_annuity",
            value: fv_ann,
        })
    }
}

/// Returns the future value of annuity (a series of constant cashflows) at a constant rate. Returns custom solution struct with additional information and functionality.
///
/// Related functions:
/// * To calculate a future value returning an f64, use [`present_value_annuity`].
/// * To calculate a future value with a varying rate or varying cashflow or both, use [`present_value_annuity_schedule`].
///
/// The future value annuity formula is:
///
/// future value ann = sum( cashflow * (1 + rate)<sup>period</sup> )
/// or
/// future value ann = Constant_Cashflow * ((1+periodic_rate)^n -1) / periodic_rate
///
/// # Arguments
/// * `rate` - The rate at which the investment grows or shrinks per period,
/// expressed as a floating point number. For instance 0.05 would mean 5%. Often appears as
/// `r` or `i` in formulas.
/// * `periods` - The number of periods such as quarters or years. Often appears as `n` or `t`.
/// * `cashflow` - The value of the constant cashflow (aka payment).
/// * `timing` - [`PaymentTiming`] or `bool` (`false` = end of period).
///
/// # Errors
/// Same domain failures as [`future_value_annuity`].
///
/// # Examples
/// Future value of a $500 annuity at 3.4% for 10 years (ordinary):
/// ```
/// use finance_solution::*;
/// let my_annuity = future_value_annuity_solution(
///     0.034, 10, 500, PaymentTiming::EndOfPeriod
/// ).unwrap();
/// assert!(my_annuity.future_value().abs() > 5_000.0);
/// assert!(!my_annuity.due_at_beginning());
/// ```
///
/// Annuity due variant:
/// ```
/// use finance_solution::*;
/// let due = future_value_annuity_solution(
///     0.034, 10, 500, PaymentTiming::BeginningOfPeriod
/// ).unwrap();
/// assert!(due.due_at_beginning());
/// ```
pub fn future_value_annuity_solution<T, D>(
    rate: f64,
    periods: u32,
    cashflow: T,
    timing: D,
) -> crate::FinanceResult<CashflowSolution>
where
    T: Into<f64> + Copy,
    D: Into<crate::PaymentTiming>,
{
    let annuity = cashflow.into();
    let timing = timing.into();
    let due_at_beginning = timing.is_beginning();
    let fv = future_value_annuity(rate, periods, annuity, timing)?;
    let fvann_type = match timing {
        crate::PaymentTiming::BeginningOfPeriod => CashflowVariable::FutureValueAnnuityDue,
        crate::PaymentTiming::EndOfPeriod => CashflowVariable::FutureValueAnnuity,
    };

    let (formula, formula_symbolic) = match timing {
        crate::PaymentTiming::EndOfPeriod => (
            format!(
                "-{} * (((1. + {}).powf({}) - 1.) / {});",
                annuity, rate, periods, rate
            ),
            "-annuity * (((1. + rate).powf(periods) - 1.) / rate);".to_string(),
        ),
        crate::PaymentTiming::BeginningOfPeriod => (
            format!(
                "-{} * (1. + {}) * (((1. + {}).powf({}) - 1.) / {});",
                annuity, rate, rate, periods, rate
            ),
            "-annuity * (1. + rate) * (((1. + rate).powf(periods) - 1.) / rate);".to_string(),
        ),
    };
    let pv = present_value(rate, periods, fv, false)?;
    Ok(CashflowSolution::new(
        fvann_type,
        rate,
        periods,
        pv,
        fv,
        due_at_beginning,
        annuity,
        &formula,
        &formula_symbolic,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_future_value_annuity() {
        let rate = 0.034;
        let periods = 10;
        let annuity = 500;
        let fv = future_value_annuity(rate, periods, annuity, false).unwrap();
        // assert_approx_equal!(5838.66016, fv);
        assert_eq!(-5838.66016, (fv * 100000.).round() / 100000.);
    }

    #[test]
    fn test_future_value_annuity_1() {
        let rate = 0.034;
        let periods = 1;
        let annuity = 500;
        let fv = future_value_annuity(rate, periods, annuity, false).unwrap();
        // assert_approx_equal!(5838.66016, fv);
        assert_eq!(-500.0000, (fv * 100000.).round() / 100000.);
    }
    #[test]
    fn test_future_value_annuity_2() {
        let rate = 0.034;
        let periods = 400;
        let annuity = 500;
        let fv = future_value_annuity(rate, periods, annuity, false).unwrap();
        // assert_approx_equal!(9455966284.4844600, fv);
        assert_eq!(-9455966284.4844600, (fv * 100000.).round() / 100000.);
    }

    #[test]
    fn test_future_value_annuity_3() {
        // big rate
        let rate = 0.989;
        let periods = 8;
        let annuity = 120_000;
        let fv = future_value_annuity(rate, periods, annuity, false).unwrap();
        assert_eq!(-29_599_651.75013, (fv * 100000.).round() / 100000.);
    }

    #[test]
    fn test_future_value_annuity_4() {
        let rate = 0.00009;
        let periods = 780;
        let annuity = 120_000;
        let fv = future_value_annuity(rate, periods, annuity, false).unwrap();
        assert_eq!(-96_959_087.75951, (fv * 100000.).round() / 100000.);
    }

    #[test]
    fn test_future_value_annuity_5() {
        // negative rate
        let rate = -0.0314;
        let periods = 10;
        let annuity = 13_000;
        let fv = future_value_annuity(rate, periods, annuity, false).unwrap();
        assert_eq!(-113_087.68194, (fv * 100000.).round() / 100000.);
    }

    #[test]
    fn test_future_value_annuity_6() {
        // big negative rate
        let rate = -0.999;
        let periods = 10;
        let annuity = 13_000;
        let fv = future_value_annuity(rate, periods, annuity, false).unwrap();
        assert_eq!(-13_013.01301, (fv * 100000.).round() / 100000.);
    }

    #[test]
    fn test_future_value_annuity_7() {
        // big negative rate, big periods
        // note: the convergence with the previous test
        let rate = -0.999;
        let periods = 780;
        let annuity = 13_000;
        let fv = future_value_annuity(rate, periods, annuity, false).unwrap();
        assert_eq!(-13_013.01301, (fv * 100000.).round() / 100000.);
    }

    #[test]
    fn test_future_value_annuity_payment_timing_parity() {
        use crate::PaymentTiming;
        let ordinary_bool = future_value_annuity(0.034, 10, 500, false).unwrap();
        let ordinary_enum =
            future_value_annuity(0.034, 10, 500, PaymentTiming::EndOfPeriod).unwrap();
        assert_eq!(ordinary_bool, ordinary_enum);

        let due_bool = future_value_annuity(0.034, 10, 500, true).unwrap();
        let due_enum =
            future_value_annuity(0.034, 10, 500, PaymentTiming::BeginningOfPeriod).unwrap();
        assert_eq!(due_bool, due_enum);
        assert!(due_enum.abs() > ordinary_enum.abs());
    }
}
