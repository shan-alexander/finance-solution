// use log::Level;
// use log::{info, warn, log_enabled};
use log::warn;
// use std::fmt::{Debug, Formatter, Error};
use std::fmt::{self, Debug};
// use std::ops::{Deref, DerefMut};

pub struct FutureValueSolution {
    pub rate: f64,
    pub periods: u32,
    pub present_value: f64,
    pub future_value: f64,
    pub formula: String,
}

impl FutureValueSolution {
    fn new(rate: f64, periods: u32, present_value: f64, future_value: f64) -> Self {
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

// type FV = FutureValueSolution; // Creates a type alias

/// Returns the value of an investment after it has grown or shrunk over time.
/// Constructor for the letter trie. The root of each trie is the same regardless of what words will be added to
/// the trie so there are no parameters.
///
/// # Examples
/// ```rust
/// // The investment grows by 3.4% per quarter.
/// let periodic_rate = 0.034;
/// // The initial investment is $250,000.
/// let present_value = 250_000;
/// // The investment will grow for 5 quarters.
/// let periods = 5;
/// let future_value = finance::future_value_f64(periodic_rate, present_value, periods);
/// // Confirm that the future value is correct to four decimal places.
/// assert_eq!(295_489.9418, finance::round_to_fraction_of_cent(future_value));
/// ```

pub fn future_value_f64<T>(periodic_rate: f64, present_value: T, periods: u32) -> f64
    where T: Into<f64> + Copy
{
    future_value(periodic_rate, present_value, periods).future_value
}

pub fn future_value<T>(periodic_rate: f64, present_value: T, periods: u32) -> FutureValueSolution
    where T: Into<f64> + Copy
{
    // pv, n, and r are the standard shorthands for the three values used in the calculation.
    let pv = present_value.into();
    let n = periods;
    let r = periodic_rate;
    // assertions to ensure valid financial computation
    assert!(r.is_finite());

    assert!(pv.is_finite());
    assert!(pv >= 0.);
    assert!(r >= -1.);
    assert!(n >= 1);
    if r.abs() > 1. {
        warn!("You provided a rate ({}) greater than 1. Are you sure you expect a {}% return?", r, r*100.0);
    }
    // final computation for future value
    let future_value = pv * (1. + r).powf(n as f64);
    FutureValueSolution::new(r, n, pv, future_value)
}

pub struct FutureValuePeriod {
    pub period: u32,
    pub rate: f64,
    pub present_value: f64,
    pub period_value: f64,
    pub future_value: f64,
}

impl FutureValuePeriod {
    fn new(period: u32, rate: f64, present_value: f64, period_value: f64, future_value: f64) -> Self {
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
               &format!("period: {}", self.period),
               &format!("rate: {:.6}", self.rate),
               &format!("present_value: {:.4}", self.present_value),
               &format!("period_value: {:.4}", self.period_value),
               &format!("future_value: {:.4}", self.future_value),
        )
    }
}

/// Return a vector of future values for each period, starting with Period0 (present value) to Period_n (future value).
pub fn future_value_series<T>(interest_rate: f64, present_value: T, periods: u32) -> Vec<FutureValuePeriod>
    where T: Into<f64> + Copy
{
    // pv, n, and r are the standard shorthands for the three values used in the calculation.
    let pv = present_value.into();
    let r = interest_rate;
    let n = periods;
    // assertions to ensure valid financial computation
    assert!(r.is_finite());
    assert!(r >= 0.);
    assert!(pv.is_finite());
    assert!(pv >= 0.);
    assert!(n >= 1);
    // final computation for returning a series of future values
    let interest_mult = 1. + r;
    let future_value = pv * interest_mult.powf(n as f64);
    let mut v = vec![FutureValuePeriod::new(0, r, pv, pv, future_value)];
    // to do: how do we handle fractional periods? should we allow fractions in this function?
    for period in 1..=n {
        let period_value = pv * interest_mult.powi(period as i32);
        v.push(FutureValuePeriod::new(period, r, pv, period_value, future_value));
    }
    v
}

#[derive(Debug)]
pub struct FutureValueSchedule {
    pub num_periods: u32,
    pub present_value: f64,
    pub future_value: f64,
    pub periods: Vec<FutureValueSchedulePeriod>,
}

pub struct FutureValueSchedulePeriod {
    pub period: u32,
    pub rate: f64,
    pub value: f64,
}

impl FutureValueSchedule {
    fn new(num_periods: u32, present_value: f64, future_value: f64) -> Self {
        let schedule = Self {
            num_periods,
            present_value,
            future_value,
            periods: vec![],
        };
        schedule
    }
}

impl FutureValueSchedulePeriod {
    fn new(period: u32, rate: f64, value: f64) -> Self {
        Self { period, rate, value }
    }
}

impl Debug for FutureValueSchedulePeriod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ {}, {}, {} }}",
               &format!("period: {}", self.period),
               &format!("rate: {:.6}", self.rate),
               &format!("value: {:.4}", self.value),
        )
    }
}

/// Returns a Future Value of a present amount with variable rates.
pub fn future_value_schedule<T>(rates: &[f64], present_value: T) -> FutureValueSchedule
    where T: Into<f64> + Copy
{
    // assertions to ensure valid financial computation
    for rate in rates {
        assert!(rate.is_finite());
        assert!(*rate > -1.0);
        // warning to ensure developer did not mistake rate with percentage
        if rate.abs() > 1. {
            warn!("You provided a rate ({}) greater than 1. Are you sure you expect a {}% return?", rate, rate * 100.);
        }
    }
    let pv = present_value.into();
    assert!(pv.is_finite());
    assert!(pv >= 0.);
    let num_periods = rates.len();
    let mut period_values = vec![pv];
    for period in 1..=num_periods {
        let period_value = period_values[period-1] * (1. + rates[period-1]);
        period_values.push(period_value);
    }
    let future_value = period_values[num_periods];
    // final computation for future value
    let mut schedule = FutureValueSchedule::new(num_periods as u32, pv, future_value);
    for period in 1..=num_periods {
        schedule.periods.push(FutureValueSchedulePeriod::new(period as u32, rates[period - 1], period_values[period]));
    }
    schedule
}

#[cfg(test)]
mod tests {
    // use float_cmp::ApproxEq;
    use super::*;
    use crate::*;

    #[test]
    fn test_future_value_f64() {
        assert_eq!(295_489.9418, round_to_fraction_of_cent(future_value_f64(0.034, 250_000.00, 5)));
        assert_eq!(20_629.3662, round_to_fraction_of_cent(future_value_f64(0.08, 13_000.0, 6)));
        assert_eq!(5_000.0000, round_to_fraction_of_cent(future_value_f64(-0.09, 8_804.84368898, 6)));
    }

    #[test]
    fn test_future_value_1() {
        let rate_of_return = 0.034;
        let present_value_1 = 250_000.00;
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
        let periods = 6;
        let expected_value = 5_000.00;
        let actual_value = future_value(rate_of_return, present_value, periods).future_value;
        assert_eq!(round_to_cent(expected_value), round_to_cent(actual_value));
    }

    #[should_panic]
    #[test]
    fn test_future_value_5() {
        // test zero for periods
        let rate_of_return = 0.09;
        let present_value = 5_000.00;
        let periods = 0;
        let _should_panic = future_value(rate_of_return, present_value, periods);
    }


    #[should_panic]
    #[test]
    fn test_future_value_6() {
        // test infinity on rate
        let rate_of_return = 1.0f64 / 0.0f64;
        let present_value = 5_000.00;
        let periods = 6;
        let _should_panic = future_value(rate_of_return, present_value, periods);
    }

    #[should_panic]
    #[test]
    fn test_future_value_7() {
        // test infinity on fv
        let rate_of_return = 0.03;
        let present_value = 1.0f64 / 0.0f64;
        let periods = 6;
        let _should_panic = future_value(rate_of_return, present_value, periods);
    }

    /*
    // This is no longer possible since periods is now an integer. Is there some equivalent test?
    #[should_panic]
    #[test]
    fn test_future_value_8() {
        // test infinity on periods
        let rate_of_return = 0.03;
        let present_value = 500;
        let periods = 1.0f64 / 0.0f64;
        let _should_panic = future_value(rate_of_return, present_value, periods);
    }
    */

    #[test]
    fn test_future_value_9() {
        // test various negative rates, pv should be > fv
        let rate_of_return = -0.03;
        let present_value = 5000.00;
        let periods = 12;
        let try_1 = future_value(rate_of_return, present_value, periods).future_value;
        assert!(try_1 < present_value.into());

        let rate_of_return = -0.9;
        let try_2 = future_value(rate_of_return, present_value, periods).future_value;
        assert!(try_2 < present_value.into());

        let rate_of_return = -3.2;
        let result = std::panic::catch_unwind(|| future_value(rate_of_return, present_value, periods));
        assert!(result.is_err());  //probe further for specific error type here, if desired

    }

}

