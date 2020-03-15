#![allow(dead_code)]
#![allow(unused_imports)]

use float_cmp::ApproxEq;
use log::Level;
use log::{info, warn, log_enabled};

pub fn main() {
    try_present_value();
    // try_present_value_series()
    // try_pv();
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
    dbg!(&present_value_3);
    // println!("{:?}", present_value_3); 
    dbg!(present_value_3.present_value_series());

}

fn try_pv() {
    // expect 211_513.1216
    let rate_of_return = 0.034f64;
    let future_value_1 = 250_000f64;
    let periods = 5;
    let pv_1 = pv(rate_of_return, periods, future_value_1,);
    dbg!(pv_1);
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
pub struct PresentValueSolution {
    pub rate: f64,
    pub periods: f64,
    pub present_value: f64,
    pub future_value: f64,
}

impl PresentValueSolution {
    pub fn new(rate: f64, periods: f64, present_value: f64, future_value: f64, ) -> Self {
        Self {
            rate,
            periods,
            present_value,
            future_value,
        }
    }
    pub fn present_value_series(&self) -> Vec<PresentValuePeriod> {
        // assertions to ensure valid financial computation
        assert!(self.rate.is_finite());
        assert!(self.future_value.is_finite());
        assert!(self.future_value >= 0.);
        // final computation for returning a series of present values
        let interest_mult = 1. + self.rate;
        let _present_value = self.future_value / (interest_mult).powi(self.periods as i32);
        let mut present_value_periods = vec![PresentValuePeriod::new(self.periods as u16, self.rate, self.future_value, self.future_value, self.present_value)];
        for period in 1..=self.periods as u16 {
            let period_value = self.future_value / (interest_mult).powi(period as i32);
            present_value_periods.insert(0, PresentValuePeriod::new(self.periods as u16 - period, self.rate, self.future_value, period_value, self.present_value));
        }
        present_value_periods
    }
}
type PV = PresentValueSolution; // Creates a type alias

/// Return the Present Value of a future amount.
pub fn present_value<T: Into<f64> + Copy, F: Into<f64> + Copy>(rate_of_return: f64, future_value: F, periods: T) -> PV {
    // Bench: 1.4776 us  when including PeriodValues
    // Bench: 26.650 ns  when removing the PeriodValues calculation
    // assertions to ensure valid financial computation
    let fv = future_value.into();
    assert!(rate_of_return.is_finite());
    assert!(fv.is_finite());
    assert!(fv >= 0.);
    if rate_of_return > 1. || rate_of_return < -1. { 
        warn!("You used a rate of return ({}) greater than 1, therefore implying a return of {}%. Are you sure?", rate_of_return, rate_of_return*100.);
    }
    let n = periods.into();
    // final computation for returning a present value
    let present_value = fv / (1. + rate_of_return).powf(n);

    if rate_of_return < 0.0 {
        assert!(present_value > fv);
    }

    PresentValueSolution::new(rate_of_return, n, present_value, fv)
}


pub fn pv<T: Into<f64> + Copy, C: Into<f64> + Copy>(r: f64, n: T, fv: C) -> f64 {
    // PV = 𝐅𝐕 / (𝟏 + 𝐢)^n
    // Bench:  17.781 ns
    let c = fv.into();
    let t = n.into();
    c / (1. + r).powf(t)
}

#[derive(Debug)]
pub struct PresentValuePeriod {
    pub period: u16,
    pub rate: f64,
    pub future_value: f64,
    pub period_value: f64,
    pub present_value: f64,
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
    fn test_present_value_1() {
        let rate_of_return = 0.08;
        let future_value = 20_629.37;
        let periods = 6;
        let expected_value = 13_000.0;
        let actual_value = present_value(rate_of_return, future_value, periods).present_value;
        assert_eq!(round_to_cent(expected_value), round_to_cent(actual_value));
    }

    #[test]
    fn test_present_value_2() {
        // test different types
        let rate_of_return = 0.08;
        let future_value = 20_629.37_f32;
        let periods = 6_u8;
        let expected_value = 13_000.0;
        let actual_value = present_value(rate_of_return, future_value, periods).present_value;
        assert_eq!(round_to_cent(expected_value), round_to_cent(actual_value));
    }
    #[test]
    fn test_present_value_3() {
        // test negative rate
        let rate_of_return = -0.09;
        let future_value = 5_000_u32;
        let periods = 6.0;
        let expected_value = 8_804.84368898;
        let actual_value = present_value(rate_of_return, future_value, periods).present_value;
        assert_eq!(round_to_cent(expected_value), round_to_cent(actual_value));
    }
    
    #[should_panic]
    #[test]
    fn test_present_value_4() {
        // test negative future value 
        let rate_of_return = 0.09;
        let future_value = -5_000_i32;
        let periods = 6;
        // let expected_value = 8_804.84368898;
        let actual_value = present_value(rate_of_return, future_value, periods).present_value;
        // assert_eq!(round_to_cent(expected_value), round_to_cent(actual_value));

    }
 

}