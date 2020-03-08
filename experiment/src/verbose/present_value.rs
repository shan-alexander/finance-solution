#![allow(dead_code)]
#![allow(unused_imports)]

use float_cmp::ApproxEq;
use log::Level;
use log::{info, warn, log_enabled};

pub fn main() {
    try_present_value();
    // try_present_value_series()
}

fn try_present_value() {
    // expect 1047.6190
    let rate_of_return = 0.05f64;
    let future_value_1 = 1_100f64;
    let periods = 1;
    let present_value_1 = present_value(rate_of_return, future_value_1, periods);
    dbg!(present_value_1);
    
    // expect 211_513.1216
    let rate_of_return = 0.034f64;
    let future_value_1 = 250_000f64;
    let periods = 5;
    let present_value_2 = present_value(rate_of_return, future_value_1, periods);
    dbg!(present_value_2);

    // expect 7181.0056
    let rate_of_return = 1.034f64;
    let future_value_1 = 250_000f64;
    let periods = 5;
    let present_value_3 = present_value(rate_of_return, future_value_1, periods);
    dbg!(present_value_3);
    // println!("{:?}", present_value_3); 

}

fn try_present_value_series() {
    // expect 1047.6190
    let rate_of_return = 0.05f64;
    let future_value_1 = 1_100f64;
    let periods = 1;
    let present_value_1 = present_value_series(rate_of_return, future_value_1, periods);
    dbg!(present_value_1);
    
    // expect 211_513.1216
    let rate_of_return = 0.034f64;
    let future_value_1 = 250_000f64;
    let periods = 5;
    let present_value_2 = present_value_series(rate_of_return, future_value_1, periods);
    dbg!(present_value_2);

    // expect 7181.0056
    let rate_of_return = 1.034f64;
    let future_value_1 = 250_000f64;
    let periods = 5;
    let present_value_3 = present_value_series(rate_of_return, future_value_1, periods);
    dbg!(present_value_3);
}


#[derive(Debug)]
pub struct PresentValue {
    rate: f64,
    periods: u16,
    present_value: f64,
    future_value: f64,
    period_values: Vec<PresentValuePeriod>,
}
impl PresentValue {
    pub fn new(rate: f64, periods: u16, present_value: f64, future_value: f64, period_values: Vec<PresentValuePeriod>) -> Self {
        Self {
            rate,
            periods,
            present_value,
            future_value,
            period_values,
        }
    }
}
/// Return the Present Value of a future amount.
pub fn present_value(rate_of_return: f64, future_value: f64, periods: u16) -> PresentValue {
    // assertions to ensure valid financial computation
    assert!(rate_of_return.is_finite());
    assert!(future_value.is_finite());
    assert!(future_value >= 0.);
    if rate_of_return > 1. { 
        warn!("You used a rate of return ({}) greater than 1, therefore implying a return of {}%. Are you sure?", rate_of_return, rate_of_return*100.);
    }
    // final computation for returning a present value
    let present_value = future_value / (1. + rate_of_return).powi(periods as i32);
    
    let mut period_values = vec![PresentValuePeriod::new(periods, rate_of_return, future_value, future_value, present_value)];
    for period in 1..=periods {
        let period_value = future_value / (1. + rate_of_return).powi(period as i32);
        period_values.insert(0, PresentValuePeriod::new(periods-period, rate_of_return, future_value, period_value, present_value));
    }

    PresentValue::new(rate_of_return, periods, present_value, future_value, period_values)
}

#[derive(Debug)]
pub struct PresentValuePeriod {
    period: u16,
    rate: f64,
    future_value: f64,
    period_value: f64,
    present_value: f64,
}
impl PresentValuePeriod {
    pub fn new(period: u16, rate: f64, future_value: f64, period_value: f64, present_value: f64) -> Self {
        Self {
            period,
            rate,
            future_value,
            period_value,
            present_value,

        }
    }
}
/// Return the Present Value of a future amount, as a Vec of periods showing details about each period calculation.
pub fn present_value_series(rate_of_return: f64, future_value: f64, periods: u16) -> Vec<PresentValuePeriod> {
    // assertions to ensure valid financial computation
    assert!(rate_of_return.is_finite());
    assert!(future_value.is_finite());
    assert!(future_value >= 0.);
    // final computation for returning a series of present values
    let interest_mult = 1. + rate_of_return;
    let present_value = future_value / (interest_mult).powi(periods as i32);
    let mut present_value_periods = vec![PresentValuePeriod::new(periods, rate_of_return, future_value, future_value, present_value)];
    for period in 1..=periods {
        let period_value = future_value / (interest_mult).powi(period as i32);
        present_value_periods.insert(0, PresentValuePeriod::new(periods-period, rate_of_return, future_value, period_value, present_value));
    }
    present_value_periods
}


#[cfg(test)]
mod tests {
    use super::*;
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