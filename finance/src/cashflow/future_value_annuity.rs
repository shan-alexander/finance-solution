#![allow(unused_imports)]

//! **Future value _annuity_ calculations**. Given a series of cashflows, a number of periods such as years, and a fixed
//! interest rate, what is the value of the series of cashflows (annuity) at the final payment?
//! 
//! For most common usages, we recommend the [`future_value_annuity_solution`](./fn.future_value_annuity_solution.html) function, which provides a better debugging experience and additional features.
//! 
//! ## Example
//! ```
//! let (rate, periods, annuity, due) = (0.034, 10, 500, false);
//! let fv_ann = finance_solution::future_value_annuity_solution(rate, periods, annuity, due);
//! dbg!(fv_ann);
//! ```
//! Outputs to terminal:
//! ```text
//! {
//! calculated_field: FutureValueAnnuity,
//! rate: 0.034,
//! periods: 10,
//! present_value: -4179.341028819186,
//! future_value: -5838.660162934523,
//! due_at_beginning: false,
//! payment: 500.0,
//! sum_of_payments: 5000.0,
//! sum_of_interest: -5018.0011917537095,
//! formula: "-500 * ((1. - (1. / (1. + 0.034)).powf(10)) / 0.034);",
//! symbolic_formula: "-annuity * ((1. - (1. / (1. + rate)).powf(periods)) / rate);",
//! }
//! ```
//! 

// to-do: add "use log::warn;" and helper logs

// Needed for the Rustdoc comments and module.
use crate::future_value::future_value;
use crate::present_value::present_value;
use crate::cashflow::*;
use crate::assert_approx_equal;

fn check_future_value_annuity_parameters(rate:f64, periods:u32, cashflow:f64) {
    assert!(rate > -1.0);
    assert!(rate.is_finite());
    assert!(cashflow.is_finite());
    assert!(periods > 0);
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
/// * `due_at_beginning` - True if the payment is due at the beginning of the period. Typically the
/// payment will be due at the end of the period so this will be false.
///
/// # Panics
/// The call will fail if `rate` is less than -1.0 as this would mean the investment is
/// losing more than its full value every period.
///
/// # Examples
/// Quick Glance, how to use:
/// ```
/// use finance_solution::*;
/// let (rate, periods, payment, due_at_beginning) = (0.034, 5, 500, false);
/// let my_annuity = future_value_annuity(rate, periods, payment, due_at_beginning);
/// assert_approx_equal!(my_annuity, -2_675.8789282); 
/// ```
/// 
/// Or use the solution struct (recommended, more helpful to debugging and for student-learning)
/// ```
/// use finance_solution::*;
/// let (rate, periods, pmt, due_at_beginning) = (0.034, 5, 500, false);
/// let my_annuity = future_value_annuity_solution(rate, periods, pmt, due_at_beginning);
/// dbg!(&my_annuity);
/// ```
/// Outputs to terminal: 
/// ```text
/// {
///  calculated_field: FutureValueAnnuity,
///  rate: 0.034,
///  periods: 5,
///  present_value: 2263.9340209510633,
///  future_value: 2675.8789281680038,
///  due_at_beginning: false,
///  payment: 500.0,
///  sum_of_payments: 2500.0,
///  sum_of_interest: 7439.812949119067,
///  formula: "500 * ((1. - (1. / (1. + 0.034)).powf(5)) / 0.034);",
///  formula_symbolic: "annuity * ((1. - (1. / (1. + rate)).powf(periods)) / rate);",
/// }
/// ```
/// ```
/// # use finance_solution::*;
/// # let (rate, periods, pmt, due_at_beginning) = (0.034, 5, 500, false);
/// # let my_annuity = future_value_annuity_solution(rate, periods, pmt, due_at_beginning);
/// // call the value as a method to get the solution value
/// let final_answer = my_annuity.future_value();
/// ```
/// 
/// Another example: Future value of a series of $2000 cashflows.
/// ```
/// # use finance_solution::*;
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
/// // Find the future value.
/// let future_value_ann = future_value_annuity(rate, periods, cashflow, due_at_beginning);
/// dbg!(&future_value_ann);
///
/// // Confirm that the future value is correct to four decimal places (one hundredth of a cent).
/// // assert_approx_equal!( , future_value_ann);
/// ```
pub fn future_value_annuity<T>(rate: f64, periods: u32, annuity: T, due_at_beginning: bool) -> f64
    where T: Into<f64> + Copy
{
    let pmt = annuity.into();
    check_future_value_annuity_parameters(rate, periods, pmt);
    // let mut fv_accumulator = 0_f64;
    // for i in 0..periods { 
    //     let future_value = future_value(rate, i as u32, pmt);
    //     fv_accumulator = fv_accumulator + future_value;
    // }
    // fv_accumulator

    // FV_ann = Constant_Cashflow * [ ( (1+periodic_rate)^n -1 )/ periodic_rate ]
    
    // let fv_ann= if due_at_beginning {
    //     let mut fv_accumulator = (1. + rate) * pmt;
    //     for i in 0..periods {
    //         let future_value = future_value(rate, i as u32, pmt);
    //         fv_accumulator = fv_accumulator + future_value;
    //     }
    //     fv_accumulator
    // } else {
    //     pmt * ((1. + rate).powf(periods as f64) - 1.) / rate
    // };
    // fv_ann

    let fv_ann = -(1. + (rate * due_at_beginning as u32 as f64)) * pmt * ((1. + rate).powf(periods as f64) - 1.) / rate;
    fv_ann
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
/// * `due_at_beginning` - True if the payment is due at the beginning of the period. Typically the
/// payment will be due at the end of the period so this will be false.
///
// / # Panics
// / The call will fail if `rate` is less than -1.0 as this would mean the investment is
// / losing more than its full value every period.
// /
/// # Examples
/// Future value of a $500 annuity (a series of $500 cashflows) at 3.4% for 10 years.
/// ```
/// use finance_solution::*;
/// let (rate, periods, cashflow, due_at_beginning) = (0.034, 10, 500, false);
/// let my_annuity = future_value_annuity_solution(rate, periods, cashflow, due_at_beginning);
/// dbg!(&my_annuity);
/// ```
pub fn future_value_annuity_solution<T>(rate: f64, periods: u32, cashflow: T, due_at_beginning: bool) -> CashflowSolution
    where T: Into<f64> + Copy
{
    let annuity = cashflow.into();
    let fv = future_value_annuity(rate, periods, annuity, due_at_beginning);
    let fvann_type= if due_at_beginning {
        CashflowVariable::FutureValueAnnuityDue
    } else {
        CashflowVariable::FutureValueAnnuity
    };
    // check_future_value__annuity_varying_parameters(rate, periods, cashflow);

    // add due to formulas
    let formula = format!("-{} * ((1. - (1. / (1. + {})).powf({})) / {});", annuity, rate, periods, rate);
    let formula_symbolic = format!("-annuity * ((1. - (1. / (1. + rate)).powf(periods)) / rate);");
    // let fv = future_value_annuity(rate, periods, cashflow);
    let pv = present_value(rate, periods, fv, false);
    CashflowSolution::new(fvann_type, rate, periods, pv, fv, due_at_beginning, annuity, &formula, &formula_symbolic)
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
        let fv = future_value_annuity(rate, periods, annuity, false);
        // assert_approx_equal!(5838.66016, fv);
        assert_eq!(-5838.66016, (fv * 100000.).round() / 100000.);
    }

    #[test]
    fn test_future_value_annuity_1() {
        let rate = 0.034;
        let periods = 1;
        let annuity = 500;
        let fv = future_value_annuity(rate, periods, annuity, false);
        // assert_approx_equal!(5838.66016, fv);
        assert_eq!(-500.0000, (fv * 100000.).round() / 100000.);
    }
    #[test]
    fn test_future_value_annuity_2() {
        let rate = 0.034;
        let periods = 400;
        let annuity = 500;
        let fv = future_value_annuity(rate, periods, annuity, false);
        // assert_approx_equal!(9455966284.4844600, fv);
        assert_eq!(-9455966284.4844600, (fv * 100000.).round() / 100000.);
    }

    #[test]
    fn test_future_value_annuity_3() {
        // big rate
        let rate = 0.989;
        let periods = 8;
        let annuity = 120_000;
        let fv = future_value_annuity(rate, periods, annuity, false);
        assert_eq!(-29_599_651.75013, (fv * 100000.).round() / 100000.);
    }

    #[test]
    fn test_future_value_annuity_4() {
        let rate = 0.00009;
        let periods = 780;
        let annuity = 120_000;
        let fv = future_value_annuity(rate, periods, annuity, false);
        assert_eq!(-96_959_087.75951, (fv * 100000.).round() / 100000.);
    }

    #[test]
    fn test_future_value_annuity_5() {
        // negative rate
        let rate = -0.0314;
        let periods = 10;
        let annuity = 13_000;
        let fv = future_value_annuity(rate, periods, annuity, false);
        assert_eq!(-113_087.68194, (fv * 100000.).round() / 100000.);
    }

    #[test]
    fn test_future_value_annuity_6() {
        // big negative rate
        let rate = -0.999;
        let periods = 10;
        let annuity = 13_000;
        let fv = future_value_annuity(rate, periods, annuity, false);
        assert_eq!(-13_013.01301, (fv * 100000.).round() / 100000.);
    }

    #[test]
    fn test_future_value_annuity_7() {
        // big negative rate, big periods
        // note: the convergence with the previous test
        let rate = -0.999;
        let periods = 780;
        let annuity = 13_000;
        let fv = future_value_annuity(rate, periods, annuity, false);
        assert_eq!(-13_013.01301, (fv * 100000.).round() / 100000.);
    }

}