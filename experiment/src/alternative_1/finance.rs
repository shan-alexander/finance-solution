#![allow(dead_code)]
#![allow(unused_imports)]
// use std::ops::{Add, Mul};
// use crate::num_traits::pow;
// use crate::ordered_float::*;
use float_cmp::ApproxEq;
use log::Level;
use log::{info, warn, log_enabled};
use env_logger::Env;





// present value, future value, net present value.

pub fn main() {
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    // try_future_and_present_value();
    // try_net_present_value();
    try_net_present_value_constant_cash_flow_sum();
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
    dbg!(future_value_series(0.0f64, 250_000f64, 5));
    dbg!(future_value_series(0.034f64, 0f64, 5));
    dbg!(future_value_series(5f64, 250_000_000_000_000_000_000_000f64, 5));
    dbg!(future_value_series(1.0f64/3f64, 250_000f64, 5));
    // dbg!(future_value_vec(f64::NAN, 250_000f64, 5));
    // dbg!(future_value_vec(0.034f32, 250_000f32, 5));
    // dbg!(future_value_vec(NotNaN::unchecked_new(0.034f64), NotNaN::unchecked_new(250_000.0f64), 5));
}

fn try_net_present_value() {
    // Expect $22,891.34.
    let initial_investment = 100_000.0;
    let discount_rate = 0.10;
    let n = 10;
    let cash_flow = 20_000.0;
    dbg!(net_present_value_constant_cash_flow_sum(initial_investment, discount_rate, n, cash_flow));

    // dbg!(net_present_value_constant_cash_flow_single(initial_investment, discount_rate, n, cash_flow));
}

// try with negative initial investment, to test warning
fn try_net_present_value_constant_cash_flow_sum() {
    // Expect $22,891.34.
    let initial_investment = -100_000.0;
    let discount_rate = 0.10;
    let n = 10;
    let cash_flow = 20_000.0;
    let npv = net_present_value_constant_cash_flow_sum(initial_investment, discount_rate, n, cash_flow);
    dbg!(npv);

}

/// Return the Net Present Value (NPV) of a constant cash flow (cf_1..cf_n are all equal). 
pub fn net_present_value_constant_cash_flow_sum(initial_investment: f64, discount_rate: f64, n: u16, cash_flow: f64) -> f64 {
    // assertions to ensure valid financial computation
    assert!(initial_investment.is_finite());
    assert!(discount_rate.is_finite());
    assert!(n > 0);
    assert!(cash_flow.is_finite());
    // warning to ensure developer did not mistake rate with percentage
    if initial_investment < 0. { 
        warn!("You used a negative initial investment amount (your cf0 = {}). Are you sure? The fn net_present_value_constant_cash_flow_sum() will sum the NPV of cash flows and then subtract the initial investment amount.", initial_investment);
        info!(target: "net_present_value_constant_cash_flow_sum", "This function expects your initial investment (cf0) to be a positive number when it is a cash outflow. The formula will subtract cf0 from the sum(npv of cashflows). cf0 should not be negative unless you actually receive cash inflow in cf0, which is a rare financial situation.");
    }
    // if INFO logs are enabled, show the computation of each cashflow
    if log_enabled!(Level::Info) {
        let mut cf_vec = vec![initial_investment];
        for t in 1..=n {
            let cf_n = cash_flow / (1.0 + discount_rate).powi(t as i32);
            assert!(cf_n.is_finite());
            cf_vec.push(cf_n);
        }
        info!(target: "NPV of each cash flow, in order from 0 --> period n", "{:?}", cf_vec);
    }
    
    // final computation for NPV with constant cashflow (cf1..cf_n is the same)
    let mut sum = 0.0f64;
    for t in 1..=n {
        sum += cash_flow / (1.0 + discount_rate).powi(t as i32);
        assert!(sum.is_finite());
    }
    sum - initial_investment
}

/*
pub fn net_present_value_constant_cash_flow_single(initial_investment: f64, discount_rate: f64, n: u16, cash_flow: f64) -> f64 {
    let numerator = 1.0 - ((1.0 + discount_rate).recip()).powi(n as i32 + 1);
    let denominator = 1.0 - (1.0 + discount_rate).recip();
    (cash_flow * (numerator / denominator)) + initial_investment
}
*/

/// Returns a Future Value of a present amount.
pub fn future_value(interest_rate: f64, present_value: f64, periods: u16) -> f64 {
    // assertions to ensure valid financial computation
    assert!(interest_rate.is_finite());
    assert!(interest_rate >= 0.0);
    assert!(present_value.is_finite());
    assert!(present_value >= 0.0);
    // warning to ensure developer did not mistake rate with percentage
    if interest_rate > 1. { 
        warn!("You provided a rate ({}) greater than 1. Are you sure you expect a {}% return?", interest_rate, interest_rate*100.0); 
    }
    // final computation for future value
    present_value * (1.0 + interest_rate).powi(periods as i32)
}

/// Return a vector of future values for each period, starting with Period0 (present value) to Period_n (future value).
pub fn future_value_series(interest_rate: f64, present_value: f64, periods: u16) -> Vec<f64> {
    // assertions to ensure valid financial computation
    assert!(interest_rate.is_finite());
    assert!(interest_rate >= 0.0);
    assert!(present_value.is_finite());
    assert!(present_value >= 0.0);
    // final computation for returning a series of future values
    let mut v = vec![];
    let interest_mult = 1.0 + interest_rate;
    for period in 1..=periods {
        v.push(present_value * interest_mult.powi(period as i32));
    }
    v
}

/// Return the Present Value of a future amount.
pub fn present_value(rate_of_return: f64, future_value: f64, periods: u16) -> f64 {
    // assertions to ensure valid financial computation
    assert!(rate_of_return.is_finite());
    assert!(future_value.is_finite());
    assert!(future_value >= 0.0);
    // final computation for returning a present value
    future_value / (1.0 + rate_of_return).powi(periods as i32)
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
    fn test_present_value_1() {
        let rate_of_return = 0.08;
        let future_value = 20_629.37;
        let periods = 6;
        let expected_value = 13_000.0;
        let actual_value = present_value(rate_of_return, future_value, periods);
        assert_eq!(round_to_cent(expected_value), round_to_cent(actual_value));
    }

}

