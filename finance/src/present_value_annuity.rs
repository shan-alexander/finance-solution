//! Present value annuity calculations. Given a series of cashflows, a number of periods such as years, and fixed
//! or varying interest rates, what is the current value of the series of cashflows?
//!

use log::warn;

use crate::tvm_cashflow::*;
// Needed for the Rustdoc comments.
#[allow(unused_imports)]
use crate::present_value::present_value;
use crate::*;


/// Returns the present value of a future series of constant cashflows and constant rate. For simple cases. Returns f64.
///
/// Related functions:
/// * To calculate a present value with a varying rate or varying cashflow or both, use [`present_value_annuity_schedule`].
///
/// The present value annuity formula is:
///
/// present_value = sum( cashflow / (1 + rate)<sup>period</sup> )
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
/// // Confirm that the present value is correct to four decimal places (one hundredth of a cent).
/// // finance::assert_approx_equal!( , present_value_ann);
/// ```
pub fn present_value_annuity<T>(rate: f64, periods: u32, cashflow: T) -> f64
    where T: Into<f64> + Copy
{
    let pmt = cashflow.into();
    // check_present_value__annuity_parameters(rate, periods, cashflow);
    let mut pv_accumulator = 0_f64;
    for i in 0..periods { 
        let present_value = present_value(rate, i as u32, pmt);
        pv_accumulator = pv_accumulator + present_value;
    }
    pv_accumulator
}


/// Returns the present value of a series of cashflows and rates, which can be varying. Receives vectors and returns f64.
///
/// Related functions:
/// * To calculate a present value with a constant cashflow and rate, use [`present_value_annuity`].
///
/// The present value annuity formula is:
///
/// present_value = sum( cashflow / (1 + rate)<sup>period</sup> )
///
/// # Arguments
/// * `rate` - The rate at which the investment grows or shrinks per period,
/// expressed as a floating point number. For instance 0.05 would mean 5%. Often appears as
/// `r` or `i` in formulas.
/// * `periods` - The number of periods such as quarters or years. Often appears as `n` or `t`.
/// * `cashflow` - The value of the cashflow at the time of that period (ie, future value).
///
/// # Panics
/// The call will fail if `rate` is less than -1.0 as this would mean the investment is
/// losing more than its full value every period. 
///
/// # Examples
/// Present value of a series of $2000 cashflows.
/// ```
/// // The rate is varying each month. 
/// let rates = vec![0.021, 0.028, 0.019];
/// 
/// // The cashflow will be $2,000.
/// // The number of periods is inferred by the length of the vector.
/// // The rep! macro is used to create a vector of repeating values.
/// // let cashflows = finance::repeat!(2_000, rate.len());
/// let  cashflows = vec![2000,2000,2000];
/// 
/// // Find the current value.
/// let present_value_ann = finance::present_value_annuity_schedule(rates, cashflows);
/// dbg!(&present_value_ann);
///
/// // Confirm that the present value is correct to four decimal places (one hundredth of a cent).
/// // finance::assert_approx_equal!( , present_value_ann);
/// ```
pub fn present_value_annuity_schedule<T>(rates: Vec<f64>, cashflows: Vec<T>) -> f64
    where T: Into<f64> + Copy
{
    // check_present_value__annuity_varying_parameters(rate, periods, cashflow);
    let periods = rates.len();
    let mut pv_accumulator = 0_f64;
    for i in 0..periods { 
        let pmt = cashflows[i].into();
        let rate = rates[i];
        let present_value = present_value(rate, i as u32, pmt);
        pv_accumulator = pv_accumulator + present_value;
    }
    pv_accumulator
}


/// Returns the present value of a future series of constant cashflows and constant rate. For simple cases. Returns custom solution type with additional information and functionality.
///
/// Related functions:
/// * To calculate a present value returning an f64, use [`present_value_annuity`].
/// * To calculate a present value with a varying rate or varying cashflow or both, use [`present_value_annuity_schedule`].
///
/// The present value annuity formula is:
///
/// present_value = sum( cashflow / (1 + rate)<sup>period</sup> )
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
/// let present_value_ann = finance::present_value_annuity_solution(rate, periods, cashflow);
/// dbg!(&present_value_ann);
///
/// 
/// ```
pub fn present_value_annuity_solution<T>(rate: f64, periods: u32, cashflow: T) -> tvm_cashflow::TvmCashflowSolution
    where T: Into<f64> + Copy
{
    let annuity = cashflow.into();
    // check_present_value__annuity_varying_parameters(rate, periods, cashflow);
    let pv = present_value_annuity(rate, periods, annuity);
    let formula = format!("{} * ((1. - (1. / (1. + {})).powf({})) / {});", annuity, rate, periods, rate);
    let formula_symbolic = format!("annuity * ((1. - (1. / (1. + rate)).powf(periods)) / rate);");
    // let fv = future_value_annuity(rate, periods, cashflow);
    let fv = 22./3.;
    let sum_of_interest = (annuity*periods as f64) - pv;
    let rates = repeat!(rate, periods);
    TvmCashflowSolution::new(TvmCashflowVariable::PresentValueAnnuity, rates, periods, pv, fv, false, annuity, 0.0,  annuity, sum_of_interest, &formula, &formula_symbolic)

}