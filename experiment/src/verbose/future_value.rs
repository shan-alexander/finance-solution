#![allow(dead_code)]
#![allow(unused_imports)]

use float_cmp::ApproxEq;
use log::Level;
use log::{info, warn, log_enabled};

pub fn main() { 
    // try_future_value();
    // try_future_value_series();
    try_future_value_schedule();
}


fn try_future_value() {
    // expect 1100
    let rate_of_return = 0.05f64;
    let present_value_1 = 1_047.6190f64;
    let periods = 1;
    let future_value_1 = future_value(rate_of_return, present_value_1, periods);
    dbg!(future_value_1);
    
    // expect 250_000
    let rate_of_return = 0.034f64;
    let present_value_2 = 211_513.1216f64;
    let periods = 5;
    let future_value_2 = future_value(rate_of_return, present_value_2, periods);
    dbg!(future_value_2);

    // expect 250_000
    let rate_of_return = 1.034f64;
    let present_value_3 = 7_181.0056f64;
    let periods = 5;
    let present_value_3 = future_value(rate_of_return, present_value_3, periods);
    dbg!(present_value_3);
}

fn try_future_value_series() {
    // expect 1100
    let rate_of_return = 0.05f64;
    let present_value_1 = 1_047.6190f64;
    let periods = 1;
    let future_value_1 = future_value_series(rate_of_return, present_value_1, periods);
    dbg!(future_value_1);
    
    // expect 250_000
    let rate_of_return = 0.034f64;
    let present_value_2 = 211_513.1216f64;
    let periods = 5;
    let future_value_2 = future_value_series(rate_of_return, present_value_2, periods);
    dbg!(future_value_2);

    // expect 250_000
    let rate_of_return = 1.034f64;
    let present_value_3 = 7_181.0056f64;
    let periods = 5;
    let present_value_3 = future_value_series(rate_of_return, present_value_3, periods);
    dbg!(present_value_3);
}

fn try_future_value_schedule() {
    // expect 1147.26
    let rates: Vec<f64> = vec![0.02,0.03,0.04,0.05];
    let present_value_1 = 1_000.;
    let future_value_1 = future_value_schedule(&rates, present_value_1);
    dbg!(future_value_1);
}

#[derive(Debug)]
pub struct FutureValue {
    pub rate: f64,
    pub periods: u16,
    pub present_value: f64,
    pub future_value: f64,
}
impl FutureValue {
    pub fn new(rate: f64, periods: u16, present_value: f64, future_value: f64) -> Self {
        Self {
            rate,
            periods,
            present_value,
            future_value,
        }
    }
}
/// Returns a Future Value of a present amount.
pub fn future_value(interest_rate: f64, present_value: f64, periods: u16) -> FutureValue {
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
    let future_value = present_value * (1. + interest_rate).powi(periods as i32);
    FutureValue::new(interest_rate, periods, present_value, future_value)
}

#[derive(Debug)]
pub struct FutureValuePeriod {
    pub period: u16,
    pub rate: f64,
    pub present_value: f64,
    pub period_value: f64,
    pub future_value: f64,
}
impl FutureValuePeriod {
    pub fn new(period: u16, rate: f64, present_value: f64, period_value: f64, future_value: f64) -> Self {
        Self {
            period,
            rate,
            present_value,
            period_value,
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
    let interest_mult = 1. + interest_rate;
    let future_value = present_value * interest_mult.powi(periods as i32);
    let mut v = vec![FutureValuePeriod::new(0, interest_rate, present_value, present_value, future_value)];
    for period in 1..=periods {
        let period_value = present_value * interest_mult.powi(period as i32);
        v.push(FutureValuePeriod::new(period, interest_rate, present_value, period_value, future_value));
    }
    v
}

#[derive(Debug)]
pub struct FutureValueSchedule {
    pub rates: Vec<f64>,
    pub num_periods: u16,
    pub present_value: f64,
    pub future_value: f64,
    pub period_values: Vec<f64>,
}
impl FutureValueSchedule {
    pub fn new(rates: Vec<f64>, num_periods: u16, present_value: f64, future_value: f64, period_values: Vec<f64>) -> Self {
        Self {
            rates,
            num_periods,
            present_value,
            future_value,
            period_values,
        }
    }
}
/// Returns a Future Value of a present amount with variable rates.
pub fn future_value_schedule(rates: &[f64], present_value: f64) -> FutureValueSchedule {
    // assertions to ensure valid financial computation
    for r in rates {
        assert!(r.is_finite());
        assert!(r >= &0.);
        // warning to ensure developer did not mistake rate with percentage
        if r > &1. { 
            warn!("You provided a rate ({}) greater than 1. Are you sure you expect a {}% return?", r, r*100.); 
        }
    }
    assert!(present_value.is_finite());
    assert!(present_value >= 0.);
    let num_periods = rates.len();
    let all_rates = rates.to_vec();
    let mut period_values = vec![present_value * (1. + rates[0])];
    for i in 1..num_periods {
        let period_value = period_values[i-1] * (1. + rates[i]);
        period_values.push(period_value);
    }
    let future_value = period_values[num_periods-1];
    // final computation for future value
    FutureValueSchedule::new(all_rates, num_periods as u16, present_value, future_value, period_values)
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
}

