//! Net Present Value calculations. Given a series of cashflows, an initial investment, a number of periods such as years, and fixed
//! or varying interest rates, what is the net value of the series of cashflows (initial investment is the cashflow at time0)?
//!

// use crate::tvm_cashflow::*;
// Needed for the Rustdoc comments.
#[allow(unused_imports)]
use crate::present_value_annuity::present_value_annuity;
use crate::*;

/// Returns the net present value of a future series of constant cashflows and constant rate, subtracting the initial investment cost. Returns f64.
///
/// Related functions:
/// * To calculate a present value with a varying rate or varying cashflow or both, use [`present_value_annuity_schedule`].
///
/// The net present value annuity formula is:
///
/// npv = initial_investment + sum( cashflow / (1 + rate)<sup>period</sup> )
/// or
/// npv = initial_investment +  cashflow * ((1. - (1. / (1. + rate)).powf(periods)) / rate)
///
/// # Arguments
/// * `rate` - The rate at which the investment grows or shrinks per period,
/// expressed as a floating point number. For instance 0.05 would mean 5%. Often appears as
/// `r` or `i` in formulas.
/// * `periods` - The number of periods such as quarters or years. Often appears as `n` or `t`.
/// * `cashflow` - The value of the constant cashflow (aka payment).
/// * `initial investment` - The value of the initial investment (should be negative, or 0).
///
/// # Panics
/// The call will fail if `rate` is less than -1.0 as this would mean the investment is
/// losing more than its full value every period.
///
/// # Examples
/// Net Present Value of a series of -$1000 investment which will payback $500 yearly for 10 years.
/// ```
/// // The rate is 3.4% per month.
/// let rate = 0.034;
///
/// // The investment will grow for 10 months.
/// let periods = 10;
///
/// // The initial investment is -$1000
/// let initial_investment = -1000;
/// 
/// // The cashflow is $500.
/// let cashflow = 500;
///
/// // Find the current value of this scenario.
/// let net_present_value = finance::net_present_value(rate, periods, initial_investment, cashflow);
/// dbg!(&net_present_value);
///
/// // Confirm that the present value is correct to four decimal places (one hundredth of a cent).
/// finance::assert_approx_equal!( 3321.438623, net_present_value);
/// ```
pub fn net_present_value<C, I>(rate: f64, periods: u32, initial_investment: I, cashflow: C) -> f64 
where I: Into<f64> + Copy, C: Into<f64> + Copy
{
    let annuity = cashflow.into();
    let ii = initial_investment.into();
    let pv_cashflow = present_value_annuity(rate, periods, annuity);
    let npv = ii + pv_cashflow;
    npv
}

/// Returns the net present value of a schedule of rates and cashflows (can be varying), subtracting the initial investment cost. Returns f64.
///
pub fn net_present_value_schedule<C>(rates: &[f64], cashflows: &[C]) -> f64 
where C: Into<f64> + Copy
{
    let mut cflows = vec![];
    for i in 0..cashflows.len() {
        cflows.push(cashflows[i].into());
    }
    let cashflows = &cflows;
    assert!(cashflows[0] <= 0.0, "The initial investment (cashflows[0]) should be negative or zero");
    assert!(cashflows.len() >= 2, "Must provide at least 2 values in cashflows, the initial investment at the 0 position and the following cashflows, or a single cashflow representing a repeating constant cashflow.");
    assert!(rates.len() >= 1, "Must provide at least 1 rate.");
    let rate_length = rates.len();
    let cashflow_length = cashflows.len();
    let initial_investment = cashflows[0];
    let mut cashflow_vec = vec![initial_investment];
    let mut rate_vec = vec![];
    let periods: u32;
    let r: &[f64];
    let c: &[f64];

    if rate_length == 1 && cashflow_length == 2 {
        r = &rates;
        c = &cashflows;
        periods = 1_u32;
    } else if rate_length > 1 && cashflow_length > 2 {
        r = &rates;
        c = &cashflows;
        periods = rate_length as u32;
    } else if rate_length > 1 && cashflow_length == 2 {
        r = &rates;
        periods = rate_length as u32;
        for _i in 0..periods {
            cashflow_vec.push(cashflows[1])
        }
        c = &cashflow_vec;
    } else if rate_length == 1 && cashflow_length > 2 {
        c = &cashflows;
        periods = cashflow_length as u32 - 1;
        for _i in 0..periods {
            rate_vec.push(rates[0])
        }
        r = &rate_vec;
    } else {
        // revise this panic message
        panic!("At least rates or cashflows for net_present_value_schedule must provide the full series of inputs. Only one input can be a shorthand expression of a repeating input. If both are repeating constant inputs, use the net_present_value function.");
    }

    let mut pv_accumulator = 0_f64;
    for i in 0..periods { 
        let present_value = present_value(r[i as usize], i as u32, c[i as usize + 1]);
        pv_accumulator = pv_accumulator + present_value;
    }
    let npv = initial_investment + pv_accumulator;
    npv
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_net_present_value() {
        let rate = 0.034;
        let periods = 10;
        let ii = -1000;
        let cf = 500;
        let npv = net_present_value(rate, periods, ii, cf);
        assert_approx_equal!(3321.4386237990, npv);
    }
}