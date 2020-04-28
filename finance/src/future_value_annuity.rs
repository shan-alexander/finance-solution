#![allow(unused_imports)]

//! Future value annuity calculations. Given a series of cashflows, a number of periods such as years, and fixed
//! or varying interest rates, what is the value of the series of cashflows at the final payment?
//!

// to do: add "use log::warn;" and helper logs

// use crate::tvm_cashflow::*;
// Needed for the Rustdoc comments.
use crate::present_value::present_value;
use crate::future_value::future_value;
use crate::tvm_cashflow::*;
// use crate::*;



/// Returns the future value of a future series of constant cashflows and constant rate. For simple cases. Returns f64.
///
/// Related functions:
// / * To calculate a future value with a varying rate or varying cashflow or both, use [`future_value_annuity_schedule`].
///
/// The future value annuity formula is:
///
/// future value = sum( cashflow * (1 + rate)<sup>period</sup> )
/// or
/// future value = annuity * ?
///
/// # Arguments
/// * `rate` - The rate at which the investment grows or shrinks per period,
/// expressed as a floating point number. For instance 0.05 would mean 5%. Often appears as
/// `r` or `i` in formulas.
/// * `periods` - The number of periods such as quarters or years. Often appears as `n` or `t`.
/// * `cashflow` - The value of the constant cashflow (aka payment).
///
/// # Panics
/// The call will fail if `rate` is less than -1.0 as this would mean the investment is
/// losing more than its full value every period.
///
/// # Examples
/// Future value of a series of $2000 cashflows.
/// ```
/// // The rate is 2.1% per month.
/// let rate = 0.021;
///
/// // The investment will grow for 12 months.
/// let periods = 12;
///
/// // The cashflow will be $2,000.
/// let cashflow = 2_000;
///
/// // Find the future value.
/// let future_value_ann = finance::future_value_annuity(rate, periods, cashflow);
/// dbg!(&future_value_ann);
///
/// // Confirm that the future value is correct to four decimal places (one hundredth of a cent).
/// // finance::assert_approx_equal!( , future_value_ann);
/// ```
pub fn future_value_annuity<T>(rate: f64, periods: u32, annuity: T) -> f64
    where T: Into<f64> + Copy
{
    let pmt = annuity.into();
    // check_future_value__annuity_parameters(rate, periods, cashflow);
    // let mut fv_accumulator = 0_f64;
    // for i in 0..periods { 
    //     let future_value = future_value(rate, i as u32, pmt);
    //     fv_accumulator = fv_accumulator + future_value;
    // }
    // fv_accumulator

    // FV_ann = Constant_Cashflow * [ ( (1+periodic_rate)^n -1 )/ periodic_rate ]
    let fv_ann = pmt * ((1. + rate).powf(periods as f64) - 1.) / rate;
    fv_ann
}

/// Returns the future value of an annuity due (a future series of constant cashflows) with constant rate, where the payment is at the beginning of the period. Returns f64.
///
/// Related functions:
// / * To calculate a future value with a varying rate or varying cashflow or both, use [`present_value_annuity_schedule`].
///
/// The future value annuity due formula is:
///
/// present_value = sum( (cashflow / (1 + rate)<sup>period</sup>) + ((1 + rate) * cashflow) )
/// or
/// future value = annuity * (1 + rate) + annuity * ((1. - (1. / (1. + rate)).powf(periods)) / rate)
///
/// # Arguments
/// * `rate` - The rate at which the investment grows or shrinks per period,
/// expressed as a floating point number. For instance 0.05 would mean 5%. Often appears as
/// `r` or `i` in formulas.
/// * `periods` - The number of periods such as quarters or years. Often appears as `n` or `t`.
/// * `cashflow` - The value of the constant cashflow (aka payment).
///
/// # Panics
/// The call will fail if `rate` is less than -1.0 as this would mean the investment is
/// losing more than its full value every period.
///
/// # Examples
/// Present value of a series of $2000 cashflows.
/// ```
/// // The rate is 2.1% per month.
/// let rate = 0.021;
///
/// // The investment will grow for 12 months.
/// let periods = 12;
///
/// // The cashflow will be $2,000.
/// let cashflow = 2_000;
///
/// // Find the current value.
/// let present_value_ann = finance::present_value_annuity(rate, periods, cashflow);
/// dbg!(&present_value_ann);
///
/// // Confirm that the future value is correct to four decimal places (one hundredth of a cent).
/// // finance::assert_approx_equal!( , present_value_ann);
/// ```
pub fn future_value_annuity_due<T>(rate: f64, periods: u32, annuity: T) -> f64
    where T: Into<f64> + Copy
{
    let pmt = annuity.into();
    // check_present_value__annuity_parameters(rate, periods, cashflow);
    let mut fv_accumulator = (1. +  rate) * pmt;
    for i in 0..periods { 
        let future_value = future_value(rate, i as u32, pmt);
        fv_accumulator = fv_accumulator + future_value;
    }
    fv_accumulator
}


/// Returns the future value of a future series of constant cashflows and constant rate. Returns custom solution type with additional information and functionality.
///
/// Related functions:
/// * To calculate a future value returning an f64, use [`present_value_annuity`].
/// * To calculate a future value with a varying rate or varying cashflow or both, use [`present_value_annuity_schedule`].
///
/// The future value annuity formula is:
///
/// present_value = sum( cashflow / (1 + rate)<sup>period</sup> )
/// or
/// future value = annuity * ((1. - (1. / (1. + rate)).powf(periods)) / rate)
/// 
/// # Arguments
/// * `rate` - The rate at which the investment grows or shrinks per period,
/// expressed as a floating point number. For instance 0.05 would mean 5%. Often appears as
/// `r` or `i` in formulas.
/// * `periods` - The number of periods such as quarters or years. Often appears as `n` or `t`.
/// * `cashflow` - The value of the constant cashflow (aka payment).
///
// / # Panics
// / The call will fail if `rate` is less than -1.0 as this would mean the investment is
// / losing more than its full value every period.
// /
/// # Examples
/// Future value of a $500 annuity (a series of $500 cashflows).
/// ```
/// // The rate is 3.4% per month.
/// let rate = 0.034;
///
/// // The annuity will provide yearly payments for 10 years.
/// let periods = 10;
///
/// // The cashflow will be $500.
/// let cashflow = 500;
///
/// // Find the future value.
/// let future_value_ann = finance::future_value_annuity_solution(rate, periods, cashflow);
/// dbg!(&future_value_ann);
/// ```
/// The `dbg!` above will display:  (need to fix)<br>
/// >{<br>
/// >calculated_field: Future Value Annuity<br>
/// >rate (r): 0.034<br>
/// >periods (n): 10<br>
/// >present_value (pv): 4321.438623799037<br>
/// >future_value (fv): ????<br>
/// >due_at_beginning: false<br>
/// >payment (pmt): 500<br>
/// >sum_of_payments: 5000<br>
/// >sum_of_interest: 9328.77195713237 (wrong)<br>
/// >formula: "500 * ((1. - (1. / (1. + 0.034)).powf(10)) / 0.034);"<br>
/// >formula_symbolic: "annuity * ((1. - (1. / (1. + rate)).powf(periods)) / rate);"<br>
/// >}<br>
/// 
pub fn future_value_annuity_solution<T>(rate: f64, periods: u32, cashflow: T) -> TvmCashflowSolution
    where T: Into<f64> + Copy
{
    let annuity = cashflow.into();
    future_value_annuity_internal(rate, periods, annuity, false)
}


/// Returns the future value of annuity due (a future series of constant cashflows) with a constant rate and the payments made at the beginning of the period. Returns custom solution type with additional information and functionality.
///
/// Related functions:
/// * To calculate a future value returning an f64, use [`present_value_annuity`].
// / * To calculate a future value with a varying rate or varying cashflow or both, use [`present_value_annuity_schedule`].
/// * To calculate a future value due returning an f64, use [`present_value_annuity_due`].
///
/// The future value annuity due formula is:
///
/// future value = sum( cashflow * (1 + rate)<sup>period</sup> )
/// or
/// future value = annuity * ?
/// 
/// # Arguments
/// * `rate` - The rate at which the investment grows or shrinks per period,
/// expressed as a floating point number. For instance 0.05 would mean 5%. Often appears as
/// `r` or `i` in formulas.
/// * `periods` - The number of periods such as quarters or years. Often appears as `n` or `t`.
/// * `cashflow` - The value of the constant cashflow (aka payment).
///
// / # Panics
// / The call will fail if `rate` is less than -1.0 as this would mean the investment is
// / losing more than its full value every period.
// /
/// # Examples
/// Future value of a $500 annuity (a series of $500 cashflows).
/// ```
/// // The rate is 3.4% per month.
/// let rate = 0.034;
///
/// // The annuity will provide yearly payments for 10 years.
/// let periods = 10;
///
/// // The cashflow will be $500.
/// let cashflow = 500;
///
/// // Find the future value.
/// let future_value_ann = finance::future_value_annuity_solution(rate, periods, cashflow);
/// dbg!(&future_value_ann);
/// ```
/// The `dbg!` above will display: (need to fix)<br>
/// >{<br>
/// >calculated_field: Future Value Annuity<br>
/// >rate (r): 0.034<br>
/// >periods (n): 10<br>
/// >present_value (pv): 4838.438623799037<br>
/// >future_value (fv): ????<br>
/// >due_at_beginning: false<br>
/// >payment (pmt): 500<br>
/// >sum_of_payments: 5000<br>
/// >sum_of_interest: 9328.77195713237 (wrong)<br>
/// >formula: "500 * ((1. - (1. / (1. + 0.034)).powf(10)) / 0.034);"<br>
/// >formula_symbolic: "annuity * ((1. - (1. / (1. + rate)).powf(periods)) / rate);"<br>
/// >}<br>
/// 
pub fn future_value_annuity_due_solution<T>(rate: f64, periods: u32, cashflow: T) -> TvmCashflowSolution
    where T: Into<f64> + Copy {
    let annuity = cashflow.into();
    future_value_annuity_internal(rate, periods, annuity, true)
}

fn future_value_annuity_internal(rate:f64, periods:u32, annuity:f64, due: bool) -> TvmCashflowSolution {
    let fv:f64;
    let fvann_type: TvmCashflowVariable;
    if due == true {
        fv = future_value_annuity_due(rate, periods, annuity);
        fvann_type = TvmCashflowVariable::FutureValueAnnuityDue;
    } else {
        fv = future_value_annuity(rate, periods, annuity);
        fvann_type = TvmCashflowVariable::FutureValueAnnuity;
    }
    // check_present_value__annuity_varying_parameters(rate, periods, cashflow);
    let formula = format!("{} * ((1. - (1. / (1. + {})).powf({})) / {});", annuity, rate, periods, rate);
    let formula_symbolic = format!("annuity * ((1. - (1. / (1. + rate)).powf(periods)) / rate);");
    // let fv = future_value_annuity(rate, periods, cashflow);
    let pv = present_value(rate, periods, fv);
    TvmCashflowSolution::new(fvann_type, rate, periods, pv, fv, due, annuity, &formula, &formula_symbolic)

}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_futue_value_annuity() {
        let rate = 0.034;
        let periods = 10;
        let annuity = 500;
        let fv = future_value_annuity(rate, periods, annuity);
        // assert_approx_equal!(5838.66016, fv);
        assert_eq!(5838.66016, (fv * 100000.).round() / 100000.);
    }
}