#![allow(dead_code)]
#![allow(unused_imports)]

use float_cmp::ApproxEq;
use log::Level;
use log::{info, warn, log_enabled};
use std::fmt::{Debug, Formatter, Error};
use crate::format;
use std::fmt;
use std::ops::{Deref, DerefMut};

pub fn main() { 
    try_future_value();
    // try_future_value_series();
    // try_future_value_schedule();
}

fn try_future_value() {
    // expect 1100
    let rate_of_return = 0.05f64;
    let present_value_1 = 1_047.6190f64;
    let periods = 1;
    let future_value_1 = future_value(rate_of_return, present_value_1, periods);
    dbg!(&future_value_1);

    // expect 250_000
    let rate_of_return = 0.034f64;
    let present_value_2 = 211_513.1216f64;
    let periods = 5;
    let future_value_2 = future_value(rate_of_return, present_value_2, periods);
    dbg!(&future_value_2);

    let rate_of_return = 1.034f64;
    let present_value_3 = 7_181.0056f64;
    let periods = 5;
    let future_value_3 = future_value(rate_of_return, present_value_3, periods);
    dbg!(&future_value_3);

    let rate_of_return = 0.03_f64;
    let present_value_4 = 7_181_i32;
    let periods = 5.;
    let future_value_4 = future_value(rate_of_return, present_value_4, periods);
    dbg!(&future_value_4);
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

pub struct FutureValueSolution {
    pub rate: f64,
    pub periods: usize,
    pub present_value: f64,
    pub future_value: f64,
    pub formula: String,
}

impl FutureValueSolution {
    pub fn new(rate: f64, periods: usize, present_value: f64, future_value: f64) -> Self {
        let formula = format!("{} * (1 + {})^{}", present_value, rate, periods);
        Self {
            rate,
            periods,
            present_value,
            future_value,
            formula,
        }
    }
}

impl Debug for FutureValueSolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ {}, {}, {}, {}, {} }}",
               &format!("rate: {:.6}", self.rate),
               &format!("periods: {}", self.periods),
               &format!("present_value: {:.4}", self.present_value),
               &format!("future_value: {:.4}", self.future_value),
               &format!("formula: {:?}", self.formula),
        )
    }
}

type FV = FutureValueSolution; // Creates a type alias

/// Returns a Future Value of a present amount.
// pub fn future_value<T: Into<f64> + Copy, P: Into<f64> + Copy>(periodic_rate: f64, present_value: P, periods: T) -> FutureValueSolution {
pub fn future_value<T, P>(periodic_rate: f64, present_value: P, periods: T) -> FutureValueSolution
    where
        P: Into<f64> + Copy,
        T: Into<f64> + Copy,
{
    let pv = present_value.into();
    let n = periods.into();
    let r = periodic_rate;
    // assertions to ensure valid financial computation
    assert!(r.is_finite());
    assert!(n.is_finite());
    assert!(pv.is_finite());
    assert!(pv >= 0.);
    assert!(r >= -1.); 
    assert!(n >= 1.); 
    if r > 1. || r < -1.{ 
        warn!("You provided a rate ({}) greater than 1. Are you sure you expect a {}% return?", r, r*100.0); 
    }
    // final computation for future value
    let future_value = pv * (1. + r).powf(n);
    FutureValueSolution::new(r, n, pv, future_value)
}

pub struct FutureValuePeriod {
    pub period: f64,
    pub rate: f64,
    pub present_value: f64,
    pub period_value: f64,
    pub future_value: f64,
}

impl FutureValuePeriod {
    pub fn new(period: f64, rate: f64, present_value: f64, period_value: f64, future_value: f64) -> Self {
        Self {
            period,
            rate,
            present_value,
            period_value,
            future_value,
        }
    }
}

impl Debug for FutureValuePeriod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ {}, {}, {}, {}, {} }}",
               &format!("period: {}", format::format_period(self.period)),
               &format!("rate: {}", format::format_rate(self.rate)),
               &format!("present_value: {}", format::format_money(self.present_value)),
               &format!("period_value: {}", format::format_money(self.period_value)),
               &format!("future_value: {}", format::format_money(self.future_value)),
        )
    }
}

/// Return a vector of future values for each period, starting with Period0 (present value) to Period_n (future value).
pub fn future_value_series<T: Into<f64> + Copy>(interest_rate: f64, present_value: f64, periods: T) -> Vec<FutureValuePeriod> {
    let n = periods.into();
    // assertions to ensure valid financial computation
    assert!(interest_rate.is_finite());
    assert!(interest_rate >= 0.);
    assert!(present_value.is_finite());
    assert!(present_value >= 0.);
    // final computation for returning a series of future values
    let interest_mult = 1. + interest_rate;
    let future_value = present_value * interest_mult.powf(n);
    let mut v = vec![FutureValuePeriod::new(0.0, interest_rate, present_value, present_value, future_value)];
    // to do: how do we handle fractional periods? should we allow fractions in this function?
    for period in 1..=n as i32 {
        let period_value = present_value * interest_mult.powi(period as i32);
        v.push(FutureValuePeriod::new(period as f64, interest_rate, present_value, period_value, future_value));
    }
    v
}

#[derive(Debug)]
pub struct FutureValueSchedule {
    pub rates: Vec<f64>,
    pub num_periods: f64,
    pub present_value: f64,
    pub future_value: f64,
    pub period_values: Vec<f64>,
}

/*
#[derive(Debug)]
pub struct FutureValueSchedule {
    pub periods: Vec<FutureValuePeriod>,
    pub num_periods: f64,
    pub present_value: f64,
    pub future_value: f64,
}

pub struct FutureValuePeriod {
    pub period: usize,
    pub rate: f64,
    pub value: f64,
}
*/

impl FutureValueSchedule {
    pub fn new(rates: Vec<f64>, num_periods: f64, present_value: f64, future_value: f64, period_values: Vec<f64>) -> Self {
        Self {
            rates,
            num_periods,
            present_value,
            future_value,
            period_values,
        }
    }

    /*
    fn print(&self, locale: &num_format::Locale) {
        println!("{{ {}, {}, {}, {}, {} }}",
                 &format!("period: {}", format::format_period_locale(self.period, locale)),
                 &format!("rate: {}", format::format_rate_locale(self.rate, locale)),
                 &format!("present_value: {}", format::format_money_locale(self.present_value, locale)),
                 &format!("period_value: {}", format::format_money_locale(self.period_value, locale)),
                 &format!("future_value: {}", format::format_money_locale(self.future_value, locale)),
        )
    }
    */
}

/*
impl Debug for FutureValueSchedule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ {}, {}, {}, {}, {} }}",
               &format!("period: {}", format::format_period(self.period)),
               &format!("rate: {}", format::format_rate(self.rate)),
               &format!("present_value: {}", format::format_money(self.present_value)),
               &format!("period_value: {}", format::format_money(self.period_value)),
               &format!("future_value: {}", format::format_money(self.future_value)),
        )
    }
}
*/

/// Returns a Future Value of a present amount with variable rates.
pub fn future_value_schedule<P: Into<f64> + Copy>(rates: &[f64], present_value: P) -> FutureValueSchedule {
    // assertions to ensure valid financial computation
    for r in rates {
        assert!(r.is_finite());
        assert!(r > &-1.);
        // warning to ensure developer did not mistake rate with percentage
        if r > &1. { 
            warn!("You provided a rate ({}) greater than 1. Are you sure you expect a {}% return?", r, r*100.); 
        }
    }
    let pv = present_value.into();
    assert!(pv.is_finite());
    assert!(pv >= 0.);
    let num_periods = rates.len();
    let all_rates = rates.to_vec();
    let mut period_values = vec![pv * (1. + rates[0])];
    for i in 1..num_periods {
        let period_value = period_values[i-1] * (1. + rates[i]);
        period_values.push(period_value);
    }
    let future_value = period_values[num_periods-1];
    // final computation for future value
    FutureValueSchedule::new(all_rates, num_periods as f64, pv, future_value, period_values)
}


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
        let actual_value = future_value(rate_of_return, present_value_1, periods).future_value;
        assert_eq!(round_to_cent(expected_value), round_to_cent(actual_value));
        assert!( float_cmp::approx_eq!(f64, expected_value, actual_value, ulps = 4) );
    }

    #[test]
    fn test_future_value_2() {
        let rate_of_return = 0.08;
        let present_value_1 = 13_000.0;
        let periods = 6;
        let expected_value = 20_629.37;
        let actual_value = future_value(rate_of_return, present_value_1, periods).future_value;
        assert_eq!(round_to_cent(expected_value), round_to_cent(actual_value));
        // assert!(exp_value.approx_eq(act_value, (0.0, 2)));

    }

    #[test]
    fn test_future_value_3() {
        // test negative rate
        let rate_of_return = -0.09;
        let present_value = 8_804.84368898; 
        let periods = 6.0;
        let expected_value = 5_000_f64;
        let actual_value = future_value(rate_of_return, present_value, periods).future_value;
        assert_eq!(round_to_cent(expected_value), round_to_cent(actual_value));
    }

    #[should_panic]
    #[test]
    fn test_future_value_5() {
        // test negative periods
        let rate_of_return = 0.09;
        let present_value = 5_000_i32;
        let periods = -6;
        let _should_panic = future_value(rate_of_return, present_value, periods);
    }


    #[should_panic]
    #[test]
    fn test_future_value_6() {
        // test infinity on rate
        let rate_of_return = 1.0f64 / 0.0f64;
        let present_value = 5_000_i32;
        let periods = -6;
        let _should_panic = future_value(rate_of_return, present_value, periods);
    }

    #[should_panic]
    #[test]
    fn test_future_value_7() {
        // test infinity on fv
        let rate_of_return = 0.03;
        let present_value = 1.0f64 / 0.0f64;
        let periods = -6;
        let _should_panic = future_value(rate_of_return, present_value, periods);
    }

    #[should_panic]
    #[test]
    fn test_future_value_8() {
        // test infinity on periods
        let rate_of_return = 0.03;
        let present_value = 500;
        let periods = 1.0f64 / 0.0f64;
        let _should_panic = future_value(rate_of_return, present_value, periods);
    }

    #[test]
    fn test_future_value_9() {
        // test various negative rates, pv should be > fv
        let rate_of_return = -0.03;
        let present_value = 5000.;
        let periods = 12;
        let try_1 = future_value(rate_of_return, present_value, periods).future_value;
        assert!(try_1 < present_value);
        
        let rate_of_return = -0.9;
        let try_2 = future_value(rate_of_return, present_value, periods).future_value;
        assert!(try_2 < present_value);
        
        let rate_of_return = -3.2;
        let result = std::panic::catch_unwind(|| future_value(rate_of_return, present_value, periods));
        assert!(result.is_err());  //probe further for specific error type here, if desired

    }

}

