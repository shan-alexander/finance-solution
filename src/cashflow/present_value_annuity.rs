#![allow(unused_imports)]

//! **Present value _annuity_ calculations**. Given a series of constant cashflows, a number of periods
//! such as years, and a fixed interest rate, what is the current value of the series right now?
//!
//! Timing uses [`crate::PaymentTiming`] (or Excel-style `bool` via [`From`]):
//! - [`PaymentTiming::EndOfPeriod`] / `false` — ordinary annuity (Excel `type=0`)
//! - [`PaymentTiming::BeginningOfPeriod`] / `true` — annuity due (Excel `type=1`)
//!
//! Prefer the enum in new code; `bool` remains for spreadsheet parity.
//!
//! For teaching / debugging, use [`present_value_annuity_solution`].
//!
//! ## Examples
//!
//! Ordinary annuity with `bool`:
//! ```
//! use finance_solution::present_value_annuity_solution;
//! let (rate, periods, annuity, due) = (0.034, 10, 500, false);
//! let pv_ann = present_value_annuity_solution(rate, periods, annuity, due).unwrap();
//! assert!(pv_ann.present_value().abs() > 4_000.0);
//! ```
//!
//! Enum timing (preferred) — due has larger magnitude:
//! ```
//! use finance_solution::{present_value_annuity, PaymentTiming};
//! let ordinary = present_value_annuity(0.034, 10, 500, PaymentTiming::EndOfPeriod).unwrap();
//! let due = present_value_annuity(0.034, 10, 500, PaymentTiming::BeginningOfPeriod).unwrap();
//! assert!(due.abs() > ordinary.abs());
//! ```
//!
//! Integer money via `Into<f64>` still works:
//! ```
//! use finance_solution::{present_value_annuity, PaymentTiming};
//! let pv = present_value_annuity(0.021, 12, 2_000, PaymentTiming::EndOfPeriod).unwrap();
//! assert!(pv.is_finite());
//! ```
//!  

// to do: add "use log::warn;" and helper logs

// Needed for the Rustdoc comments.
use crate::cashflow::*;
use crate::future_value::future_value;
use crate::present_value::present_value;

/// Returns the **present value of an annuity** (series of constant cashflows) at a constant rate. Returns f64.
///
/// The present value annuity formula is (both yield the same result):
///
/// present_value = sum( cashflow / (1 + rate)<sup>period</sup> )
///
/// or
///
/// present value = annuity * ((1. - (1. / (1. + rate)).powf(periods)) / rate)
///
/// # Arguments
/// * `rate` - The rate at which the investment grows or shrinks per period,
/// expressed as a floating point number. For instance 0.05 would mean 5%. Often appears as
/// `r` or `i` in formulas.
/// * `periods` - The number of periods such as quarters or years. Often appears as `n` or `t`.
/// * `cashflow` - The value of the constant cashflow (aka payment, or annuity).
/// * `timing` - [`PaymentTiming`] or `bool` (`false` = end of period / Excel `type=0`).
///
/// # Errors
/// Returns [`crate::FinanceError`] if `rate` is less than or equal to -1.0, money is non-finite,
/// or `periods` is zero.
///
/// # Examples
/// Solution API with spreadsheet-style `bool`:
/// ```
/// # use finance_solution::*;
/// let my_annuity = present_value_annuity_solution(0.034, 10, 21_000, false).unwrap();
/// assert!(my_annuity.present_value().is_finite());
/// ```
///
/// Scalar PV of twelve $2,000 cashflows at 2.1% monthly:
/// ```
/// # use finance_solution::*;
/// let present_value_ann = present_value_annuity(0.021, 12, 2_000, false).unwrap();
/// assert_approx_equal!(-21021.368565, present_value_ann);
/// ```
///
/// Annuity due with the enum:
/// ```
/// # use finance_solution::*;
/// let due = present_value_annuity(0.034, 10, 500, PaymentTiming::BeginningOfPeriod).unwrap();
/// let ordinary = present_value_annuity(0.034, 10, 500, PaymentTiming::EndOfPeriod).unwrap();
/// assert!(due.abs() > ordinary.abs());
/// ```
pub fn present_value_annuity<T, D>(
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
    crate::util::error::require_rate_gt_minus_one(rate)?;
    crate::util::error::require_money("annuity", pmt)?;
    if periods == 0 {
        return Err(crate::FinanceError::InvalidPeriod {
            period: 0,
            periods: 0,
            message: "annuity requires at least one period",
        });
    }
    // Ordinary (end): PV = -pmt * (1 - (1+r)^-n) / r
    // Due (beginning): multiply by (1 + r)
    // Zero rate: PV = -pmt * n for both timings
    let pv_ann = match (rate == 0.0, timing) {
        (true, _) => -pmt * periods as f64,
        (false, crate::PaymentTiming::EndOfPeriod) => {
            -pmt * ((1.0 - (1.0 / (1.0 + rate)).powf(periods as f64)) / rate)
        }
        (false, crate::PaymentTiming::BeginningOfPeriod) => {
            -pmt * (1.0 + rate) * ((1.0 - (1.0 / (1.0 + rate)).powf(periods as f64)) / rate)
        }
    };
    if pv_ann.is_finite() {
        Ok(pv_ann)
    } else {
        Err(crate::FinanceError::NonFinite {
            field: "present_value_annuity",
            value: pv_ann,
        })
    }
}

pub fn present_value_annuity_accumulator<T, D>(
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
    crate::util::error::require_rate_gt_minus_one(rate)?;
    crate::util::error::require_money("annuity", pmt)?;

    let mut pv_accumulator = match timing {
        crate::PaymentTiming::BeginningOfPeriod => (1.0 + rate) * pmt,
        crate::PaymentTiming::EndOfPeriod => 0.0,
    };
    for i in 1..=periods {
        let present_value = present_value(rate, i as u32, pmt, false)?;
        pv_accumulator += present_value;
    }
    if pv_accumulator.is_finite() {
        Ok(pv_accumulator)
    } else {
        Err(crate::FinanceError::NonFinite {
            field: "present_value_annuity",
            value: pv_accumulator,
        })
    }
}

// / Returns the present value of a series of cashflows and rates, which can be varying. Receives vectors and returns f64.
// /
// / Related functions:
// / * To calculate a present value with a constant cashflow and rate, use [`present_value_annuity`].
// /
// / The present value annuity formula is:
// /
// / present_value = sum( cashflow / (1 + rate)<sup>period</sup> )
// /
// / # Arguments
// / * `rate` - The rate at which the investment grows or shrinks per period,
// / expressed as a floating point number. For instance 0.05 would mean 5%. Often appears as
// / `r` or `i` in formulas.
// / * `periods` - The number of periods such as quarters or years. Often appears as `n` or `t`.
// / * `cashflow` - The value of the cashflow at the time of that period (ie, future value).
// /
// / # Errors
// / The call will fail if `rate` is less than -1.0 as this would mean the investment is
// / losing more than its full value every period.
// /
// / # Examples
// / Present value of a series of $2000 cashflows.
// / ```
// / // The rate is varying each month.
// / let rates = vec![0.021, 0.028, 0.019];
// /
// / // The cashflow will be $2,000.
// / // The number of periods is inferred by the length of the vector.
// / // The rep! macro is used to create a vector of repeating values.
// / // let cashflows = finance_solution::repeat!(2_000, rate.len());
// / let  cashflows = vec![2000,2000,2000];
// /
// / // Find the current value.
// / let present_value_ann = finance_solution::present_value_annuity_schedule(rates, cashflows);
// / dbg!(&present_value_ann);
// /
// / // Confirm that the present value is correct to four decimal places (one hundredth of a cent).
// / // finance_solution::assert_approx_equal!( , present_value_ann);
// / ```
// /

// pub fn present_value_annuity_schedule<T>(rates: &[f64], cashflows: &[T]) -> f64
//     where T: Into<f64> + Copy
// {
//     // check_present_value__annuity_varying_parameters(rate, periods, cashflow);

//     // update
//     let periods = rates.len();

//     let mut pv_accumulator = 0_f64;
//     for i in 0..periods {
//         let pmt = cashflows[i].into();
//         let rate = rates[i];
//         let present_value = present_value(rate, i as u32, pmt);
//         pv_accumulator = pv_accumulator + present_value;
//     }
//     pv_accumulator
// }

/// Returns the present value of a future series of constant cashflows and constant rate. Returns custom solution type with additional information and functionality.
///
/// Related functions:
/// * To calculate a present value returning an f64, use [`present_value_annuity`].
/// * To calculate a present value with a varying rate or varying cashflow or both, use [`present_value_annuity_schedule`].
///
/// The present value annuity formula is:
///
/// present_value = sum( cashflow / (1 + rate)<sup>period</sup> )
/// or
/// present value = annuity * ((1. - (1. / (1. + rate)).powf(periods)) / rate)
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
/// Same domain failures as [`present_value_annuity`].
///
/// # Examples
/// Present value of a $500 annuity for 10 periods at 3.4%:
/// ```
/// # use finance_solution::*;
/// let present_value_ann = present_value_annuity_solution(
///     0.034, 10, 500, PaymentTiming::EndOfPeriod
/// ).unwrap();
/// assert!(present_value_ann.present_value().abs() > 4_000.0);
/// assert!(!present_value_ann.due_at_beginning());
/// ```
///
/// Annuity due solution:
/// ```
/// # use finance_solution::*;
/// let due = present_value_annuity_solution(
///     0.034, 10, 500, PaymentTiming::BeginningOfPeriod
/// ).unwrap();
/// assert!(due.due_at_beginning());
/// ```
pub fn present_value_annuity_solution<T, D>(
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
    let pv = present_value_annuity(rate, periods, annuity, timing)?;
    let pvann_type = match timing {
        crate::PaymentTiming::BeginningOfPeriod => CashflowVariable::PresentValueAnnuityDue,
        crate::PaymentTiming::EndOfPeriod => CashflowVariable::PresentValueAnnuity,
    };
    let (formula, formula_symbolic) = match timing {
        crate::PaymentTiming::EndOfPeriod => (
            format!(
                "-{} * ((1. - (1. / (1. + {})).powf({})) / {});",
                annuity, rate, periods, rate
            ),
            "-annuity * ((1. - (1. / (1. + rate)).powf(periods)) / rate);".to_string(),
        ),
        crate::PaymentTiming::BeginningOfPeriod => (
            format!(
                "-{} * ((1. - (1. / (1. + {})).powf({})) / {}) * (1. + {});",
                annuity, rate, periods, rate, rate
            ),
            "-annuity * ((1. - (1. / (1. + rate)).powf(periods)) / rate) * (1. + rate);"
                .to_string(),
        ),
    };
    let fv = future_value(rate, periods, pv, false)?;
    Ok(CashflowSolution::new(
        pvann_type,
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
    fn test_present_value_annuity_1() {
        // one period
        let (rate, periods, annuity) = (0.034, 1, 500);
        let pv = present_value_annuity(rate, periods, annuity, false).unwrap();
        assert_eq!(-483.55899, (pv * 100000.).round() / 100000.);
    }
    #[test]
    fn test_present_value_annuity_2() {
        // big periods
        let (rate, periods, annuity) = (0.034, 400, 500);
        let pv = present_value_annuity(rate, periods, annuity, false).unwrap();
        assert_eq!(-14705.85948, (pv * 100000.).round() / 100000.);
    }
    #[test]
    fn test_present_value_annuity_due_2() {
        // big periods, due
        let (rate, periods, annuity) = (0.034, 400, 500);
        let pv = present_value_annuity(rate, periods, annuity, true).unwrap();
        assert_eq!(-15205.8587, (pv * 100000.).round() / 100000.);
    }

    #[test]
    fn test_present_value_annuity_3() {
        // negative rate
        let (rate, periods, annuity) = (-0.034, 52, 500);
        let pv = present_value_annuity(rate, periods, annuity, false).unwrap();
        assert_eq!(-74_148.8399, (pv * 100000.).round() / 100000.);
    }

    #[test]
    fn test_present_value_annuity_4() {
        // big negative rate
        let (rate, periods, annuity) = (-0.999, 3, 500);
        let pv = present_value_annuity(rate, periods, annuity, false).unwrap();
        assert_eq!(-500_500_499_999.999, (pv * 1000.).round() / 1000.);
    }

    #[test]
    fn test_present_value_annuity_due_4() {
        // big negative rate, due
        let (rate, periods, annuity) = (-0.999, 3, 500);
        let pv = present_value_annuity(rate, periods, annuity, true).unwrap();
        assert_eq!(-500_500_499.999999, (pv * 1000000.).round() / 1000000.);
    }

    #[test]
    fn test_present_value_annuity_5() {
        // big precision
        let (rate, periods, annuity) = (0.00034, 2_800, 5_000_000);
        let pv = present_value_annuity(rate, periods, annuity, false).unwrap();
        assert_eq!(-9028959259.06, (pv * 100.).round() / 100.);
    }

    #[test]
    fn test_present_value_annuity_payment_timing_parity() {
        use crate::PaymentTiming;
        let ordinary_bool = present_value_annuity(0.034, 10, 500, false).unwrap();
        let ordinary_enum =
            present_value_annuity(0.034, 10, 500, PaymentTiming::EndOfPeriod).unwrap();
        assert_eq!(ordinary_bool, ordinary_enum);

        let due_bool = present_value_annuity(0.034, 10, 500, true).unwrap();
        let due_enum =
            present_value_annuity(0.034, 10, 500, PaymentTiming::BeginningOfPeriod).unwrap();
        assert_eq!(due_bool, due_enum);
        assert!(due_enum.abs() > ordinary_enum.abs());
    }
}
