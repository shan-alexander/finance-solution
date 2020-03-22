#![allow(dead_code)]
#![allow(unused_imports)]

use float_cmp::ApproxEq;
use log::Level;
use log::{error, warn, log_enabled};

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
    // dbg!(present_value_3.present_value_series());
    
    // expect 7181.0056
    let rate_of_return = 3.034f64;
    let future_value_1 = 250_000f64;
    let periods = 12;
    let present_value_3 = present_value(rate_of_return, future_value_1, periods);
    dbg!(&present_value_3);
    
    // expect 7181.0056
    let rate_of_return = -3.034f64;
    let future_value_1 = 250_000f64;
    let periods = 12;
    let present_value_4 = present_value(rate_of_return, future_value_1, periods);
    dbg!(&present_value_4);

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
    pub formula: String,
}

impl PresentValueSolution {
    pub fn new(rate: f64, periods: f64, present_value: f64, future_value: f64, ) -> Self {
        let formula = format!("{} / (1 + {})^{}", future_value, rate, periods);
        Self {
            rate,
            periods,
            present_value,
            future_value,
            formula,
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
    let n = periods.into();
    assert!(fv >= 0., "The future value must be positive");
    assert!(n > 0.0, "The number of periods must be greater than 0.0");
    assert!(rate_of_return > -1.,"The rate provided must be greater than -1.0 because the exponential characteristics of the formula.");
    assert!(rate_of_return.is_finite(), "The rate must be finite (not 1/0)");
    assert!(n.is_finite(), "The number of periods must be finite (not 1/0)");
    assert!(fv.is_finite(), "The future value must be finite (not 1/0)");
    if rate_of_return > 1. { 
        warn!("You used a rate ({}) greater than 1, therefore implying a return of {}%. Are you sure?", rate_of_return, rate_of_return*100.);
    }

    // final computation for returning a present value
    let present_value = fv / (1. + rate_of_return).powf(n);

    PresentValueSolution::new(rate_of_return, n, present_value, fv)
}


pub fn pv<T: Into<f64> + Copy, C: Into<f64> + Copy>(r: f64, n: T, fv: C) -> f64 {
    // PV = 𝐅𝐕 / (𝟏 + 𝐢)^n
    // Bench:  17.781 ns
    let c = fv.into();
    let t = n.into();
    assert!(c >= 0., "The future value must be positive");
    assert!(t > 0.0, "The number of periods must be greater than 0.0");
    assert!(r > -1.,"The rate provided must be greater than -1.0 because the exponential characteristics of the formula.");
    assert!(r.is_finite(), "The rate must be finite (not 1/0)");
    assert!(t.is_finite(), "The number of periods must be finite (not 1/0)");
    assert!(c.is_finite(), "The future value must be finite (not 1/0)");
    if r > 1. { 
        warn!("You used a rate ({}) greater than 1, therefore implying a return of {}%. Are you sure?", r, r*100.);
    }
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
        let expected_value = 13_000.0; // google sheet
        let actual_value = present_value(rate_of_return, future_value, periods).present_value;
        assert_eq!(round_to_cent(expected_value), round_to_cent(actual_value));
    }

    #[test]
    fn test_present_value_2() {
        // test different types
        let rate_of_return = 0.08;
        let future_value = 20_629.37_f32;
        let periods = 6_u8;
        let expected_value = 13_000.0; // google sheet
        let actual_value = present_value(rate_of_return, future_value, periods).present_value;
        assert_eq!(round_to_cent(expected_value), round_to_cent(actual_value));
    }
    #[test]
    fn test_present_value_3() {
        // test negative rate
        let rate_of_return = -0.09;
        let future_value = 5_000_u32;
        let periods = 6.0;
        let expected_value = 8_804.84368898; // google sheet
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
        let _should_panic = present_value(rate_of_return, future_value, periods).present_value;
    }

    #[should_panic]
    #[test]
    fn test_present_value_5() {
        // test negative periods
        let rate_of_return = 0.09;
        let future_value = 5_000_i32;
        let periods = -6;
        let _should_panic = present_value(rate_of_return, future_value, periods).present_value;
    }

    #[should_panic]
    #[test]
    fn test_present_value_6() {
        // test infinity on rate
        let rate_of_return = 1.0f64 / 0.0f64;
        let future_value = 5_000_i32;
        let periods = -6;
        let _should_panic = present_value(rate_of_return, future_value, periods).present_value;
    }
    
    #[should_panic]
    #[test]
    fn test_present_value_7() {
        // test infinity on fv
        let rate_of_return = 0.03;
        let future_value = 1.0f64 / 0.0f64;
        let periods = -6;
        let _should_panic = present_value(rate_of_return, future_value, periods).present_value;
    }

    #[should_panic]
    #[test]
    fn test_present_value_8() {
        // test infinity on periods
        let rate_of_return = 0.03;
        let future_value = 500;
        let periods = 1.0f64 / 0.0f64;
        let _should_panic = present_value(rate_of_return, future_value, periods).present_value;
    }
 

    #[test]
    fn test_present_value_9() {
        // test various negative rates, pv should be > fv
        let rate_of_return = -0.03;
        let future_value = 5000.;
        let periods = 12;
        let try_1 = present_value(rate_of_return, future_value, periods).present_value;
        assert!(try_1 > future_value);
        let rate_of_return = -0.9;
        let try_2 = present_value(rate_of_return, future_value, periods).present_value;
        assert!(try_2 > future_value);
        
        let rate_of_return = -3.2;
        let result = std::panic::catch_unwind(|| present_value(rate_of_return, future_value, periods));
        assert!(result.is_err());  //probe further for specific error type here, if desired

        let rate_of_return = -1.00;
        let result = std::panic::catch_unwind(|| present_value(rate_of_return, future_value, periods));
        assert!(result.is_err());  //probe further for specific error type here, if desired

    }

    #[test]
    fn test_present_value_10() {
        // test rate of 100%
        let rate_of_return = 1.00;
        let future_value = 5_000_u32;
        let periods = 12;
        let expected_value = 1.22070313; // google sheet
        let actual_value = present_value(rate_of_return, future_value, periods).present_value;
        assert_eq!(round_to_cent(expected_value), round_to_cent(actual_value));
    }

    #[test]
    fn test_present_value_11() {
        // test rate over 100%
        let rate_of_return = 3.00;
        let future_value = 5_000_000_u32;
        let periods = 12;
        let expected_value = 0.298023223876953; // google sheet
        let actual_value = present_value(rate_of_return, future_value, periods).present_value;
        assert_eq!(round_to_cent(expected_value), round_to_cent(actual_value));
    }

    #[test]
    fn test_present_value_12() {
        // test fractional period count
        let rate_of_return = 0.50;
        let future_value = 5_000_u32;
        let periods = 0.5;
        let expected_value = 4_082.48290463863; // google sheet
        let actual_value = present_value(rate_of_return, future_value, periods).present_value;
        assert_eq!(round_to_cent(expected_value), round_to_cent(actual_value));
        
    }

    #[test]
    fn test_present_value_13() {
        // test fractional period count
        let rate_of_return = 0.50;
        let future_value = 5_000_u32;
        let periods = 0.1;
        let expected_value = 4_801.3225039610; // google sheet
        let actual_value = present_value(rate_of_return, future_value, periods).present_value;
        assert_eq!(round_to_cent(expected_value), round_to_cent(actual_value));  
    }

    #[test]
    fn test_present_value_14() {
        // test fractional future value
        let rate_of_return = 0.13;
        let future_value = 0.75_f64;
        let periods = 9;
        let expected_value = 0.249663625036891; // google sheet
        let actual_value = present_value(rate_of_return, future_value, periods).present_value;
        assert_eq!(round_to_cent(expected_value), round_to_cent(actual_value));  
    }


}