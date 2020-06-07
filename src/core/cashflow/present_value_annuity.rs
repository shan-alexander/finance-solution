#![allow(missing_docs)]
#![allow(unused_imports)]

//! **Present value _annuity_ calculations**. Given a series of cashflows, a number of periods such as years, and fixed
//! or varying interest rates, what is the current value of the series of cashflows (annuity) right now?
//! 
//! For most common usages, we recommend the [`present_value_annuity_solution`](./fn.present_value_annuity_solution.html) function, which provides a better debugging experience and additional features.
//! 
//! ## Example
//! ```
//! # use finance_solution::core::present_value_annuity_solution; // this function will be hidden
//! let (rate, periods, annuity, due) = (0.034, 10, 500, false);
//! let pv_ann = present_value_annuity_solution(rate, periods, annuity, due);
//! dbg!(pv_ann);
//! ```
//! Outputs to terminal:
//! ```text
//! {
//!     calculated_field: PresentValueAnnuity,
//!     rate: 0.034,
//!     periods: 10,
//!     present_value: -4179.341028819192,
//!     future_value: -5838.660162934531,
//!     due_at_beginning: false,
//!     payment: 500.0,
//!     sum_of_payments: 5000.0,
//!     sum_of_interest: -5018.001191753723,
//!     formula: "-500 * ((1. - (1. / (1. + 0.034)).powf(10)) / 0.034) * (1 + (0.034 * 0));",
//!     symbolic_formula: "-annuity * ((1. - (1. / (1. + rate)).powf(periods)) / rate) * (1. + (rate * due));",
//! }
//! ```
//!  

// to do: add "use log::warn;" and helper logs

// Needed for the Rustdoc comments.
// use crate::present_value::present_value;
// use crate::future_value::future_value;
// use crate::cashflow::*;

use super::*;
use crate::core::tvm::present_value::present_value;
use crate::core::tvm::future_value::future_value;

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
/// * `due_at_beginning` - True if the payment is due at the beginning of the period. Typically the
/// payment will be due at the end of the period so this will be false.
///
/// # Panics
/// The call will fail if `rate` is less than -1.0 as this would mean the investment is
/// losing more than its full value every period.
///
/// # Examples
/// 
/// Quick glance, how to use:
/// ```
/// # use finance_solution::core::*;
/// let (rate, periods, annuity, due_at_beginning)  = (0.034, 10, 21_000, false);
/// let my_annuity = present_value_annuity_solution(rate, periods, annuity, due_at_beginning);
/// dbg!(my_annuity);
/// ```
/// 
/// Present value of a series of $2000 cashflows.
/// ```
/// # use finance_solution::core::*;
/// // The rate is 2.1% per month.
/// let rate = 0.021;
///
/// // The investment will grow for 12 months.
/// let periods = 12;
///
/// // The cashflow will be $2,000.
/// let cashflow = 2_000;
///
/// let due_at_beginning = false;
///
/// // Find the current value.
/// let present_value_ann = present_value_annuity(rate, periods, cashflow, due_at_beginning);
/// dbg!(&present_value_ann);
///
/// // Confirm that the present value is correct to four decimal places (one hundredth of a cent).
/// assert_approx_equal!(-21021.368565, present_value_ann);
/// ```
pub fn present_value_annuity<T>(rate: f64, periods: u32, annuity: T, due_at_beginning: bool) -> f64
    where T: Into<f64> + Copy
{
    let pmt = annuity.into();
    // check_present_value__annuity_parameters(rate, periods, cashflow);
    let pv_ann = -(1. + (rate * due_at_beginning as u32 as f64)) * pmt * ((1. - (1. / (1. + rate)).powf(periods as f64)) / rate);
    pv_ann

}

pub fn present_value_annuity_accumulator<T>(rate: f64, periods: u32, annuity: T, due_at_beginning: bool) -> f64
    where T: Into<f64> + Copy
{
    let pmt = annuity.into();
    // check_present_value__annuity_parameters(rate, periods, cashflow);

    let mut pv_accumulator = if due_at_beginning {
        (1. + rate) * pmt
    } else {
        0.0
    };
    for i in 1..=periods {
        let present_value = present_value(rate, i as u32, pmt, false);
        pv_accumulator += present_value;
    }
    pv_accumulator
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
// / # Panics
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
// / // let cashflows = finance_solution::core::repeat!(2_000, rate.len());
// / let  cashflows = vec![2000,2000,2000];
// / 
// / // Find the current value.
// / let present_value_ann = finance_solution::core::present_value_annuity_schedule(rates, cashflows);
// / dbg!(&present_value_ann);
// /
// / // Confirm that the present value is correct to four decimal places (one hundredth of a cent).
// / // finance_solution::core::assert_approx_equal!( , present_value_ann);
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
/// * `due_at_beginning` - True if the payment is due at the beginning of the period. Typically the
/// payment will be due at the end of the period so this will be false.
///
// / # Panics
// / The call will fail if `rate` is less than -1.0 as this would mean the investment is
// / losing more than its full value every period.
// /
/// # Examples
/// Present value of a $500 annuity (a series of $500 cashflows).
/// ```
/// # use finance_solution::core::*;
/// // The rate is 3.4% per month.
/// let rate = 0.034;
///
/// // The annuity will provide yearly payments for 10 years.
/// let periods = 10;
///
/// // The cashflow will be $500.
/// let cashflow = 500;
///
/// let due_at_beginning = false;
///
/// // Find the current value.
/// let present_value_ann = present_value_annuity_solution(rate, periods, cashflow, due_at_beginning);
/// dbg!(&present_value_ann);
/// ```
/// Outputs to terminal:
/// ```text
///  {
///      calculated_field: PresentValueAnnuity,
///      rate: 0.034,
///      periods: 10,
///      present_value: -4179.341028819192,
///      future_value: -5838.660162934531,
///      due_at_beginning: false,
///      payment: 500.0,
///      sum_of_payments: 5000.0,
///      sum_of_interest: -5018.001191753723,
///      formula: "-500 * ((1. - (1. / (1. + 0.034)).powf(10)) / 0.034) * (1 + (0.034 * 0));",
///      symbolic_formula: "-annuity * ((1. - (1. / (1. + rate)).powf(periods)) / rate) * (1. + (rate * due));",
///  }
/// ```
pub fn present_value_annuity_solution<T>(rate: f64, periods: u32, cashflow: T, due_at_beginning: bool) -> CashflowSolution
    where T: Into<f64> + Copy
{
    let annuity = cashflow.into();
    let pv = present_value_annuity(rate, periods, annuity, due_at_beginning);
    let pvann_type = if due_at_beginning {
        CashflowVariable::PresentValueAnnuityDue
    } else {
        CashflowVariable::PresentValueAnnuity
    };
    // check_present_value__annuity_varying_parameters(rate, periods, cashflow);
    let formula = format!("-{} * ((1. - (1. / (1. + {})).powf({})) / {}) * (1 + ({} * {}));", annuity, rate, periods, rate, rate, due_at_beginning as u32 as f64);
    let formula_symbolic = format!("-annuity * ((1. - (1. / (1. + rate)).powf(periods)) / rate) * (1. + (rate * due));");
    // let fv = future_value_annuity(rate, periods, cashflow, false);
    let fv = 0.0;
    CashflowSolution::new(pvann_type, rate, periods, pv, fv, due_at_beginning, annuity, &formula, &formula_symbolic)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

#[test]
    fn test_present_value_annuity_1() {
        // one period
        let (rate, periods, annuity) = (0.034, 1, 500);
        let pv = present_value_annuity(rate, periods, annuity, false);
        assert_eq!(-483.55899, (pv * 100000.).round() / 100000.);
    }
    #[test]
    fn test_present_value_annuity_2() {
        // big periods
        let (rate, periods, annuity) = (0.034, 400, 500);
        let pv = present_value_annuity(rate, periods, annuity, false);
        assert_eq!(-14705.85948, (pv * 100000.).round() / 100000.);
    }
    #[test]
    fn test_present_value_annuity_due_2() {
        // big periods, due
        let (rate, periods, annuity) = (0.034, 400, 500);
        let pv = present_value_annuity(rate, periods, annuity, true);
        assert_eq!(-15205.8587, (pv * 100000.).round() / 100000.);
    }

    #[test]
    fn test_present_value_annuity_3() {
        // negative rate
        let (rate, periods, annuity) = (-0.034, 52, 500);
        let pv = present_value_annuity(rate, periods, annuity, false);
        assert_eq!(-74_148.8399, (pv * 100000.).round() / 100000.);
    }

    #[test]
    fn test_present_value_annuity_4() {
        // big negative rate
        let (rate, periods, annuity) = (-0.999, 3, 500);
        let pv = present_value_annuity(rate, periods, annuity, false);
        assert_eq!(-500_500_499_999.999, (pv * 1000.).round() / 1000.);
    }

    #[test]
    fn test_present_value_annuity_due_4() {
        // big negative rate, due
        let (rate, periods, annuity) = (-0.999, 3, 500);
        let pv = present_value_annuity(rate, periods, annuity, true);
        assert_eq!(-500_500_499.999999, (pv * 1000000.).round() / 1000000.);
    }

    #[test]
    fn test_present_value_annuity_5() {
        // big precision
        let (rate, periods, annuity) = (0.00034, 2_800, 5_000_000);
        let pv = present_value_annuity(rate, periods, annuity, false);
        assert_eq!(-9028959259.06, (pv * 100.).round() / 100.);
    }

}