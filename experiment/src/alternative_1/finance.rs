#![allow(dead_code)]
#![allow(unused_imports)]
// #[macro_use]
// use std::ops::{Add, Mul};
// use crate::num_traits::pow;
// use crate::ordered_float::*;
use float_cmp::ApproxEq;
use log::Level;
use log::{info, warn, log_enabled};
use env_logger::Env;



// macro_rules! pv_debug {(
//     $( #[$meta:meta] )* // attributes, such as docstrings
//     $pub:vis // optional `pub` modifier
//     fn // raw `fn` token
//     $fname:ident // identifier for the name of the function
//     $($rest:tt)* // and whatever follows, a.k.a., `...`
// ) => ( // becomes:
//     // first, define the macro:
//     macro_rules! debug {(
//         // input of debug! macro
//     ) => (
//         // you can use `stringify!($fname)` here
//         eprintln!("In function `{}`: ...", stringify!($fname));
//     )}

//     // then expand the input as is:
//     $( #[$meta] )*
//     $pub
//     fn $fname $($rest)*
// )}



// present value, future value, net present value.

pub fn main() {
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    // try_future_and_present_value();
    // try_npv();
    // try_npv_neg();
    try_npv_series();
    // try_npv_var_cf();
}

fn try_future_and_present_value() {
    let rate_of_return = 1.034f64;
    let present_value_1 = 250_000f64;
    let periods = 5;
    dbg!(future_value_series(rate_of_return, present_value_1, periods));
    let future_value_1 = future_value(rate_of_return, present_value_1, periods);
    dbg!(&future_value_1);
    
    let present_value_2 = present_value(rate_of_return, future_value_1, periods);
    dbg!(&present_value_2);
    // pv_debug!(&present_value_2);

    dbg!(future_value_series(0.0f64, 250_000f64, 5));
    dbg!(future_value_series(0.034f64, 0f64, 5));
    dbg!(future_value_series(5f64, 250_000_000_000_000_000_000_000f64, 5));
    dbg!(future_value_series(1.0f64/3f64, 250_000f64, 5));
    // dbg!(future_value_vec(f64::NAN, 250_000f64, 5));
    // dbg!(future_value_vec(0.034f32, 250_000f32, 5));
    // dbg!(future_value_vec(NotNaN::unchecked_new(0.034f64), NotNaN::unchecked_new(250_000.0f64), 5));
}

fn try_npv() {
    // Expect $22,891.34.
    let initial_investment = 100_000.0;
    let discount_rate = 0.10;
    let n = 10;
    let cash_flow = 20_000.0;
    dbg!(npv(initial_investment, discount_rate, n, cash_flow));

    // dbg!(net_present_value_constant_cash_flow_single(initial_investment, discount_rate, n, cash_flow));
}

// try with negative initial investment, to test warning
fn try_npv_neg() {
    // Expect $22,891.34.
    let initial_investment = -100_000.0;
    let discount_rate = 0.10;
    let n = 10;
    let cash_flow = 20_000.0;
    let npv = npv(initial_investment, discount_rate, n, cash_flow);
    dbg!(npv);

}

fn try_npv_series() {
    // Expect $22,891.34.
    let initial_investment = 100_000.0;
    let discount_rate = 0.10;
    let n = 10;
    let cash_flow = 20_000.0;
    dbg!(npv_series(initial_investment, discount_rate, n, cash_flow));

    // dbg!(net_present_value_constant_cash_flow_single(initial_investment, discount_rate, n, cash_flow));
}


fn try_npv_var_cf() {
    // Expect $80,015.02  https://financeformulas.net/Net_Present_Value.html
    let cashflows = vec![-500_000.,200_000.,300_000.,200_000.];
    let discount_rate = 0.10;
    let npv = npv_var_cf(discount_rate, &cashflows);
    dbg!(npv);
}

#[derive(Debug)]
pub struct NpvPeriod {
    period: u16,
    initial_investment: f64, 
    rate: f64, 
    cash_flow: f64,
    present_value: f64,
}

impl NpvPeriod {
   pub fn new(period: u16, initial_investment: f64, rate: f64, cash_flow: f64, present_value: f64) -> Self {
    Self {
        period,
        initial_investment, 
        rate, 
        cash_flow,
        present_value,
    }
   }
}
/// Return the Net Present Value (NPV) of a constant cash flow (cf_1..cf_n are all equal). 
pub fn npv_series(initial_investment: f64, discount_rate: f64, n: u16, cash_flow: f64) -> Vec<NpvPeriod> {
    // assertions to ensure valid financial computation
    assert!(initial_investment.is_finite());
    assert!(discount_rate.is_finite());
    assert!(n > 0);
    assert!(cash_flow.is_finite());
    // warning to ensure developer did not mistake rate with percentage
    if initial_investment > 0. { 
        warn!("You used a positive initial investment amount (your cf0 = {}). Are you sure? The fn npv() will sum the NPV of cash flows.", initial_investment);
        info!(target: "npv", "This function expects your initial investment (cf0) to be a positive number when it is a cash outflow. The formula will subtract cf0 from the sum(npv of cashflows). cf0 should not be negative unless you actually receive cash inflow in cf0, which is a rare financial situation.");
    }
    
    // final computation for NPV with constant cashflow (cf1..cf_n is the same)
    let npv_period = NpvPeriod::new(0, initial_investment, discount_rate, initial_investment, initial_investment);
    
    let mut cashflows = vec![npv_period];
    for t in 1..=n {
        let pv = cash_flow / (1.0 + discount_rate).powi(t as i32);
        assert!(pv.is_finite());
        cashflows.push(NpvPeriod::new(t, initial_investment, discount_rate, cash_flow, pv));
    }
    cashflows
}

/// Return the Net Present Value (NPV) of a constant cash flow (cf_1..cf_n are all equal). 
pub fn npv(initial_investment: f64, discount_rate: f64, n: u16, cash_flow: f64) -> f64 {
    // assertions to ensure valid financial computation
    assert!(initial_investment.is_finite());
    assert!(discount_rate.is_finite());
    assert!(n > 0);
    assert!(cash_flow.is_finite());
    // warning to ensure developer did not mistake rate with percentage
    if initial_investment > 0. { 
        warn!("You used a positive initial investment amount (your cf0 = {}). Are you sure? The fn npv() will sum the NPV of cash flows.", initial_investment);
        info!(target: "npv", "This function expects your initial investment (cf0) to be a positive number when it is a cash outflow. The formula will subtract cf0 from the sum(npv of cashflows). cf0 should not be negative unless you actually receive cash inflow in cf0, which is a rare financial situation.");
    }
    // if INFO logs are enabled, show the computation of each cashflow
    if log_enabled!(Level::Info) {
        let mut cf_vec = vec![initial_investment];
        for t in 1..=n {
            let cf_n = cash_flow / (1. + discount_rate).powi(t as i32);
            assert!(cf_n.is_finite());
            cf_vec.push(cf_n);
        }
        info!(target: "NPV of each cash flow, in order from 0 --> period n", "{:?}", cf_vec);
    }
    
    // final computation for NPV with constant cashflow (cf1..cf_n is the same)
    let mut sum = initial_investment;
    for t in 1..=n {
        sum += cash_flow / (1.0 + discount_rate).powi(t as i32);
        assert!(sum.is_finite());
    }
    sum
}

/*
pub fn net_present_value_constant_cash_flow_single(initial_investment: f64, discount_rate: f64, n: u16, cash_flow: f64) -> f64 {
    let numerator = 1.0 - ((1.0 + discount_rate).recip()).powi(n as i32 + 1);
    let denominator = 1.0 - (1.0 + discount_rate).recip();
    (cash_flow * (numerator / denominator)) + initial_investment
}
*/

/// Return the Net Present Value (NPV) of a variable cash flow (cf_1..cf_n can vary). The 
pub fn npv_var_cf(rate: f64, cashflows: &[f64]) -> f64 {
    let initial_investment = cashflows[0];
    let n = cashflows.len()-1;
    // assertions to ensure valid financial computation
    assert!(initial_investment.is_finite());
    assert!(rate.is_finite());
    assert!(n > 0);
    for cf in cashflows {
        assert!(cf.is_finite());
    }
    
    // warning to ensure developer did not mistake rate with percentage
    if initial_investment > 0. { 
        warn!("You used a positive initial investment amount (your cf0 = {}). Are you sure? The fn npv() will sum the NPV of cash flows and then add the (usually negative) initial investment amount.", initial_investment);
        info!(target: "npv", "This function expects your initial investment (cf0) to be a negative number when it is a cash outflow. The formula will sum the npv of cashflows. cf0 should not be positive unless there is a cash inflow in cf0, which is a rare financial situation.");
    }
    // if INFO logs are enabled, show the computation of each cashflow
    if log_enabled!(Level::Info) {
        let mut cf_vec = vec![initial_investment];
        for t in 1..=n {
            let cf_n = cashflows[t] / (1. + rate).powi(t as i32);
            assert!(cf_n.is_finite());
            cf_vec.push(cf_n);
        }
        info!(target: "NPV of each cash flow, in order from 0 --> period n", "{:?}", cf_vec);
    }
    
    // final computation for NPV with variable cashflow (cf1..cf_n can vary)
    let mut sum = 0.0f64;
    for t in 0..=n {
        sum += cashflows[t] / (1.0 + rate).powi(t as i32);
        assert!(sum.is_finite());
    }
    sum
}



/// Returns a Future Value of a present amount.
pub fn future_value(interest_rate: f64, present_value: f64, periods: u16) -> f64 {
    // assertions to ensure valid financial computation
    assert!(interest_rate.is_finite());
    assert!(interest_rate >= 0.);
    assert!(present_value.is_finite());
    assert!(present_value >= 0.);
    // warning to ensure developer did not mistake rate with percentage
    if interest_rate > 1. { 
        warn!("You provided a rate ({}) greater than 1. Are you sure you expect a {}% return?", interest_rate, interest_rate*100.0); 
    }
    // final computation for future value
    present_value * (1. + interest_rate).powi(periods as i32)
}
#[derive(Debug)]
pub struct FutureValuePeriod {
    period: u16,
    rate: f64,
    present_value: f64,
    future_value: f64,
}
impl FutureValuePeriod {
    pub fn new(period: u16, rate: f64, present_value: f64, future_value: f64) -> Self {
        Self {
            period,
            rate,
            present_value,
            future_value,
        }
    }
}

/// Return a vector of future values for each period, starting with Period0 (present value) to Period_n (future value).
pub fn future_value_series(interest_rate: f64, present_value: f64, periods: u16) -> Vec<FutureValuePeriod> {
    // assertions to ensure valid financial computation
    assert!(interest_rate.is_finite());
    assert!(interest_rate >= 0.);
    assert!(present_value.is_finite());
    assert!(present_value >= 0.);
    // final computation for returning a series of future values
    let mut v = vec![FutureValuePeriod::new(0, interest_rate, present_value, present_value)];
    let interest_mult = 1. + interest_rate;
    for period in 1..=periods {
        let fv = present_value * interest_mult.powi(period as i32);
        v.push(FutureValuePeriod::new(period, interest_rate, present_value, fv));
    }
    v
}

/// Return the Present Value of a future amount.
pub fn present_value(rate_of_return: f64, future_value: f64, periods: u16) -> f64 {
    // assertions to ensure valid financial computation
    assert!(rate_of_return.is_finite());
    assert!(future_value.is_finite());
    assert!(future_value >= 0.);
    // final computation for returning a present value
    future_value / (1. + rate_of_return).powi(periods as i32)
}

#[derive(Debug)]
pub struct PresentValuePeriod {
    period: u16,
    rate: f64,
    future_value: f64,
    present_value: f64,
}
impl PresentValuePeriod {
    pub fn new(period: u16, rate: f64, present_value: f64, future_value: f64) -> Self {
        Self {
            period,
            rate,
            future_value,
            present_value,
        }
    }
}
/// Return the Present Value of a future amount.
pub fn present_value_series(rate_of_return: f64, future_value: f64, periods: u16) -> Vec<PresentValuePeriod> {
    // assertions to ensure valid financial computation
    assert!(rate_of_return.is_finite());
    assert!(future_value.is_finite());
    assert!(future_value >= 0.);
    // final computation for returning a series of present values
    let mut v = vec![PresentValuePeriod::new(periods, rate_of_return, future_value, future_value)];
    let interest_mult = 1. + rate_of_return;
    for period in 1..=periods {
        let pv = future_value / (1. + rate_of_return).powi(periods as i32);
        v.insert(0, PresentValuePeriod::new(periods-period, rate_of_return, future_value, pv));
    }
    v
}

/*
pub fn future_value_vec<T>(interest_rate: T, present_value: T, periods: u16) -> Vec<T>
    where T: Mul<T, Output=T> + Add<T, Output=T> + pow::Pow<T, Output=T> + From<u16> + Copy
{
    let mut v: Vec<T> = vec![];
    let interest_mult: T = T::from(1) + interest_rate;
    for period in 1..=periods {
        let period_float: T = T::from(period);
        v.push(present_value * interest_mult.pow(period_float));
    }
    v
}
*/

/*
pub fn future_value_vec<T>(interest_rate: T, present_value: T, periods: u16) -> Vec<T>
    where T: Into<f64> + From<f64> + Copy
{
    let mut v: Vec<T> = vec![];
    let interest_mult: f64 = 1.0 + f64::from(interest_rate);
    let present_value_f64 = present_value as f64;
    for period in 1..=periods {
        let period_float: f64 = period as f64;
        v.push(T::from(present_value_f64 * interest_mult.powf(period_float)));
    }
    v
}
*/

pub fn round_to_fraction_of_cent(val: f64) -> f64 {
    (val * 10_000.0).round() / 10_000.0
}

pub fn round_to_cent(val: f64) -> f64 {
    (val * 100.0).round() / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_future_value_1() {
        let rate_of_return = 0.034;
        let present_value_1 = 250_000.0;
        let periods = 5;
        let expected_value = 295489.941778856;
        let actual_value = future_value(rate_of_return, present_value_1, periods);
        assert_eq!(round_to_cent(expected_value), round_to_cent(actual_value));
        assert!( float_cmp::approx_eq!(f64, expected_value, actual_value, ulps = 4) );
    }

    #[test]
    fn test_future_value_2() {
        let rate_of_return = 0.08;
        let present_value_1 = 13_000.0;
        let periods = 6;
        let expected_value = 20_629.37;
        let actual_value = future_value(rate_of_return, present_value_1, periods);
        assert_eq!(round_to_cent(expected_value), round_to_cent(actual_value));
        // assert!(exp_value.approx_eq(act_value, (0.0, 2)));

    }

    #[test]
    fn test_present_value_1() {
        let rate_of_return = 0.08;
        let future_value = 20_629.37;
        let periods = 6;
        let expected_value = 13_000.0;
        let actual_value = present_value(rate_of_return, future_value, periods);
        assert_eq!(round_to_cent(expected_value), round_to_cent(actual_value));
    }

    #[test]
    fn test_present_value_2() {
        let rate_of_return = 0.08;
        let future_value = 20_629.37;
        let periods = 6;
        let expected_value = 13_000.0;
        let actual_value = present_value(rate_of_return, future_value, periods);
        assert_eq!(round_to_cent(expected_value), round_to_cent(actual_value));
    }

}

