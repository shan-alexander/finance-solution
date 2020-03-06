#![allow(dead_code)]
#![allow(unused_imports)]

// use std::ops::{Add, Mul};
// use crate::num_traits::pow;
// use crate::ordered_float::*;
use float_cmp::ApproxEq;

// present value, future value, net present value.

pub fn main() {
    try_future_and_present_value()
    // try_net_present_value();
}

fn try_future_and_present_value() {
    let rate_of_return = 0.034f64;
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

pub fn net_present_value_constant_cash_flow_sum(initial_investment: f64, discount_rate: f64, n: u16, cash_flow: f64) -> f64 {
    assert!(initial_investment.is_finite());
    assert!(discount_rate.is_finite());
    assert!(n > 0);
    assert!(cash_flow.is_finite());
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

pub fn future_value(interest_rate: f64, present_value: f64, periods: u16) -> f64 {
    assert!(interest_rate.is_finite());
    assert!(interest_rate >= 0.0);
    assert!(present_value.is_finite());
    assert!(present_value >= 0.0);
    present_value * (1.0 + interest_rate).powi(periods as i32)
}

pub fn future_value_series(interest_rate: f64, present_value: f64, periods: u16) -> Vec<f64> {
    assert!(interest_rate.is_finite());
    assert!(interest_rate >= 0.0);
    assert!(present_value.is_finite());
    assert!(present_value >= 0.0);
    let mut v = vec![];
    let interest_mult = 1.0 + interest_rate;
    for period in 1..=periods {
        v.push(present_value * interest_mult.powi(period as i32));
    }
    v
}

pub fn present_value(rate_of_return: f64, future_value: f64, periods: u16) -> f64 {
    assert!(rate_of_return.is_finite());
    assert!(future_value.is_finite());
    assert!(future_value >= 0.0);
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

