#![allow(unused_imports)]
//! **Present value _annuity_ calculations**. Given a series of cashflows, a number of periods such as years, and fixed
//! or varying interest rates, what is the current value of the series of cashflows (annuity) right now?
//!

// to do: add "use log::warn;" and helper logs

// Needed for the Rustdoc comments.
use crate::present_value::present_value;
use crate::future_value::future_value;
use crate::tvm_cashflow::*;

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
///
/// # Panics
/// The call will fail if `rate` is less than -1.0 as this would mean the investment is
/// losing more than its full value every period.
///
/// # Examples
/// 
/// Quick glance, how to use:
/// ```
/// use finance::*;
/// let (rate, periods, annuity)  = (0.034, 10, 21_000);
/// let my_annuity = present_value_annuity_solution(rate, periods, annuity);
/// dbg!(my_annuity);
/// ```
/// 
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
pub fn present_value_annuity<T>(rate: f64, periods: u32, annuity: T) -> f64
    where T: Into<f64> + Copy
{
    let pmt = annuity.into();
    // check_present_value__annuity_parameters(rate, periods, cashflow);
    let mut pv_accumulator = 0_f64;
    for i in 0..periods { 
        let present_value = present_value(rate, (i+1) as u32, pmt);
        pv_accumulator = pv_accumulator + present_value;
    }
    pv_accumulator
}

/// Returns the **present value of an annuity due** (a series of constant cashflows with the first payment at the beginning of the period, starting at time-now) with constant rate. Returns f64.
///
/// Related functions:
// / * To calculate a present value with a varying rate or varying cashflow or both, use [`present_value_annuity_schedule`].
///
/// The present value annuity due formula is:
///
/// present_value = sum( (cashflow / (1 + rate)<sup>period</sup>) + ((1 + rate) * cashflow) )
/// or
/// present value = annuity * (1 + rate) + annuity * ((1. - (1. / (1. + rate)).powf(periods)) / rate)
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
pub fn present_value_annuity_due<T>(rate: f64, periods: u32, annuity: T) -> f64
    where T: Into<f64> + Copy
{
    let pmt = annuity.into();
    // check_present_value__annuity_parameters(rate, periods, cashflow);
    let mut pv_accumulator = (1. +  rate) * pmt;
    for i in 0..periods { 
        let present_value = present_value(rate, i as u32, pmt);
        pv_accumulator = pv_accumulator + present_value;
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
// / // let cashflows = finance::repeat!(2_000, rate.len());
// / let  cashflows = vec![2000,2000,2000];
// / 
// / // Find the current value.
// / let present_value_ann = finance::present_value_annuity_schedule(rates, cashflows);
// / dbg!(&present_value_ann);
// /
// / // Confirm that the present value is correct to four decimal places (one hundredth of a cent).
// / // finance::assert_approx_equal!( , present_value_ann);
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
///
// / # Panics
// / The call will fail if `rate` is less than -1.0 as this would mean the investment is
// / losing more than its full value every period.
// /
/// # Examples
/// Present value of a $500 annuity (a series of $500 cashflows).
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
/// // Find the current value.
/// let present_value_ann = finance::present_value_annuity_solution(rate, periods, cashflow);
/// dbg!(&present_value_ann);
/// ```
/// The `dbg!` above will display:<br>
/// >{<br>
/// >calculated_field: Present Value Annuity<br>
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
pub fn present_value_annuity_solution<T>(rate: f64, periods: u32, cashflow: T) -> TvmCashflowSolution
    where T: Into<f64> + Copy
{
    let annuity = cashflow.into();
    present_value_annuity_internal(rate, periods, annuity, false)
}


/// Returns the present value of annuity due (a future series of constant cashflows) with a constant rate and the payments made at the beginning of the period. Returns custom solution type with additional information and functionality.
///
/// Related functions:
/// * To calculate a present value returning an f64, use [`present_value_annuity`].
// / * To calculate a present value with a varying rate or varying cashflow or both, use [`present_value_annuity_schedule`].
/// * To calculate a present value due returning an f64, use [`present_value_annuity_due`].
///
/// The present value annuity due formula is:
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
///
// / # Panics
// / The call will fail if `rate` is less than -1.0 as this would mean the investment is
// / losing more than its full value every period.
// /
/// # Examples
/// Present value of a $500 annuity (a series of $500 cashflows).
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
/// // Find the current value.
/// let present_value_ann = finance::present_value_annuity_solution(rate, periods, cashflow);
/// dbg!(&present_value_ann);
/// ```
/// The `dbg!` above will display:<br>
/// >{<br>
/// >calculated_field: Present Value Annuity<br>
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
pub fn present_value_annuity_due_solution<T>(rate: f64, periods: u32, cashflow: T) -> TvmCashflowSolution
    where T: Into<f64> + Copy {
    let annuity = cashflow.into();
    present_value_annuity_internal(rate, periods, annuity, true)
}

fn present_value_annuity_internal(rate:f64, periods:u32, annuity:f64, due: bool) -> TvmCashflowSolution {
    let pv:f64;
    let pvann_type: TvmCashflowVariable;
    if due == true {
        pv = present_value_annuity_due(rate, periods, annuity);
        pvann_type = TvmCashflowVariable::PresentValueAnnuityDue;
    } else {
        pv = present_value_annuity(rate, periods, annuity);
        pvann_type = TvmCashflowVariable::PresentValueAnnuity;
    }
    // check_present_value__annuity_varying_parameters(rate, periods, cashflow);
    let formula = format!("{} * ((1. - (1. / (1. + {})).powf({})) / {});", annuity, rate, periods, rate);
    let formula_symbolic = format!("annuity * ((1. - (1. / (1. + rate)).powf(periods)) / rate);");
    // let fv = future_value_annuity(rate, periods, cashflow);
    let fv = future_value(rate, periods, pv);
    TvmCashflowSolution::new(pvann_type, rate, periods, pv, fv, due, annuity, &formula, &formula_symbolic)

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

#[test]
    fn test_present_value_annuity_1() {
        let (rate, periods, annuity) = (0.034, 1, 500);
        let pv = present_value_annuity(rate, periods, annuity);
        assert_eq!(483.55899, (pv * 100000.).round() / 100000.);
    }
    #[test]
    fn test_present_value_annuity_2() {
        // big periods
        let (rate, periods, annuity) = (0.034, 400, 500);
        let pv = present_value_annuity(rate, periods, annuity);
        assert_eq!(14705.85948, (pv * 100000.).round() / 100000.);
    }

    #[test]
    fn test_present_value_annuity_3() {
        // negative rate
        let (rate, periods, annuity) = (-0.034, 52, 500);
        let pv = present_value_annuity(rate, periods, annuity);
        assert_eq!(74_148.8399, (pv * 100000.).round() / 100000.);
    }

    #[test]
    fn test_present_value_annuity_4() {
        // big negative rate
        let (rate, periods, annuity) = (-0.999, 3, 500);
        let pv = present_value_annuity(rate, periods, annuity);
        assert_eq!(500_500_499_999.99854, (pv * 100000.).round() / 100000.);
    }

    #[test]
    fn test_present_value_annuity_5() {
        // big precision
        let (rate, periods, annuity) = (0.00034, 2_800, 5_000_000);
        let pv = present_value_annuity(rate, periods, annuity);
        assert_eq!(9028959259.062, (pv * 1000.).round() / 1000.);
    }

}