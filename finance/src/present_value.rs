//! Present value calculations. Given a final amount, a number of periods such as years, and fixed
//! or varying interest periodic_rates, what is the current value?

use log::warn;
use std::fmt::{self, Debug};

/// Returns the current value of a future amount using a fixed periodic_rate.
///
/// Related functions:
/// * To calculate a present value with a fixed periodic_rate while retaining the input values use
/// [`present_value_solution`].
/// * To calculate the value for each period instead of only the final value use
/// [`present_value_series`].
/// * If the periodic_rates vary by period use [`present_value_schedule`].
///
/// The present value formula is:
///
/// present_value = future_value / (1 + periodic_rate)<sup>periods</sup>
///
/// # Arguments
/// * `periodic_rate` - The periodic_rate at which the investment grows or shrinks per period,
/// expressed as a floating point number. For instance 0.05 would mean 5% growth. Often appears as
/// `r` or `i` in formulas.
/// * `periods` - The number of periods such as quarters or years. Often appears as `n` or `t`.
/// * `future_value` - The final value of the investment.
///
/// # Panics
/// The call will fail if `periodic_rate` is less than -1.0 as this would mean the investment is
/// losing more than its full value every period.
///
/// # Examples
/// Investment that grows month by month.
/// ```
/// ```
/// Error case: The investment loses 105% per year. There's no way to work out what this means so
/// the call to present_value() will panic.
/// ```should_panic
/// let periodic_rate = -1.05;
/// let periods = 6;
/// let present_value = 10_000.75;
/// let present_value = finance::present_value(periodic_rate, periods, present_value);
/// ```
pub fn present_value<T>(periodic_rate: f64, periods: u32, future_value: T) -> f64
    where T: Into<f64> + Copy
{
    let future_value = future_value.into();
    check_future_value_parameters(periodic_rate, periods, future_value);

    future_value / (1. + periodic_rate).powi(periods as i32)
}

pub struct PresentValueSolution {
    pub periodic_rate: f64,
    pub periods: u32,
    pub present_value: f64,
    pub future_value: f64,
    pub formula: String,
}

impl PresentValueSolution {
    pub fn new(periodic_rate: f64, periods: u32, present_value: f64, future_value: f64) -> Self {
        let formula = format!("{} / (1 + {})^{}", future_value, periodic_rate, periods);
        Self {
            periodic_rate,
            periods,
            present_value,
            future_value,
            formula,
        }
    }
    
    pub fn present_value_series(&self) -> Vec<PresentValuePeriod> {
        // assertions to ensure valid financial computation
        assert!(self.periodic_rate.is_finite());
        assert!(self.future_value.is_finite());

        // final computation for returning a series of present values
        let interest_mult = 1. + self.periodic_rate;
        let _present_value = self.future_value / (interest_mult).powi(self.periods as i32);
        let mut present_value_periods = vec![PresentValuePeriod::new(self.periods, self.periodic_rate, self.future_value, self.future_value, self.present_value)];
        for period in 1..=self.periods {
            let period_value = self.future_value / (interest_mult).powi(period as i32);
            present_value_periods.insert(0, PresentValuePeriod::new(self.periods - period, self.periodic_rate, self.future_value, period_value, self.present_value));
        }
        present_value_periods
    }
}

impl Debug for PresentValueSolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ {}, {}, {}, {}, {} }}",
               &format!("periodic_rate: {:.6}", self.periodic_rate),
               &format!("periods: {}", self.periods),
               &format!("present_value: {:.4}", self.present_value),
               &format!("future_value: {:.4}", self.future_value),
               &format!("formula: {:?}", self.formula),
        )
    }
}

/// Calculates the current value of a future amount using a fixed periodic_rate and returns a struct
/// with the inputs and the calculated value. This is used for keeping track of a collection of
/// financial scenarios so that they can be examined later.
///
/// Related functions:
/// * For simply calculating a single present value use [`present_value`].
/// * To calculate the value for each period instead of only the final value use
/// [`present_value_series`].
/// * If the rates vary by period use [`present_value_schedule`].
///
/// The present value formula is:
///
/// present_value = future_value / (1 + periodic_rate)<sup>periods</sup>
///
/// # Arguments
/// * `periodic_rate` - The periodic_rate at which the investment grows or shrinks per period,
/// expressed as a floating point number. For instance 0.05 would mean 5% growth. Often appears as
/// `r` or `i` in formulas.
/// * `periods` - The number of periods such as quarters or years. Often appears as `n` or `t`.
/// * `future_value` - The final value of the investment.
///
/// # Panics
/// The call will fail if `periodic_rate` is less than -1.0 as this would mean the investment is
/// losing more than its full value every period.
///
/// # Examples
/// Investment that grows month by month.
/// ```
/// ```
/// Error case: The investment loses 105% per year. There's no way to work out what this means so
/// the call to present_value() will panic.
/// ```should_panic
/// let periodic_rate = -1.05;
/// let periods = 6;
/// let present_value = 10_000.75;
/// let present_value = finance::present_value(periodic_rate, periods, present_value);
/// ```
pub fn present_value_solution<T>(periodic_rate: f64, periods: u32, future_value: T) -> PresentValueSolution
    where T: Into<f64> + Copy
{
    let present_value = present_value(periodic_rate, periods, future_value);
    PresentValueSolution::new(periodic_rate, periods, present_value, future_value.into())
}

pub struct PresentValuePeriod {
    pub period: u32,
    pub periodic_rate: f64,
    pub future_value: f64,
    pub period_value: f64,
    pub present_value: f64,
}

impl PresentValuePeriod {
    pub fn new(period: u32, periodic_rate: f64, future_value: f64, period_value: f64, present_value: f64) -> Self {
        Self {
            period,
            periodic_rate,
            future_value,
            period_value,
            present_value,

        }
    }
}

impl Debug for PresentValuePeriod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ {}, {}, {}, {}, {} }}",
               &format!("period: {}", self.period),
               &format!("periodic_rate: {:.6}", self.periodic_rate),
               &format!("future_value: {:.4}", self.future_value),
               &format!("period_value: {:.4}", self.period_value),
               &format!("present_value: {:.4}", self.present_value),
        )
    }
}

/// Return the Present Value of a future amount, as a Vec of periods showing details about each period calculation.
pub fn present_value_series<T>(periodic_rate: f64, periods: u32, future_value: T) -> Vec<PresentValuePeriod>
    where T: Into<f64> + Copy
{
    // assertions to ensure valid financial computation
    let future_value = future_value.into();
    assert!(periodic_rate > -1.0, "The periodic_rate provided must be greater than -1.0 because the exponential characteristics of the formula.");
    assert!(periodic_rate.is_finite(), "The periodic_rate must be finite (not NaN or infinity)");
    assert!(future_value.is_finite(), "The future value must be finite (not NaN or infinity)");
    if periodic_rate > 1.0 {
        warn!("You used a periodic_rate ({}) greater than 1, therefore implying a return of {}%. Are you sure?", periodic_rate, periodic_rate * 100.0);
    }

    // final computation for returning a series of present values
    let interest_mult = 1. + periodic_rate;
    let present_value = future_value / (interest_mult).powi(periods as i32);
    let mut present_value_periods = vec![PresentValuePeriod::new(0, periodic_rate, future_value, future_value, present_value)];
    for period in 1..=periods {
        let period_value = future_value / (interest_mult).powi(period as i32);
        present_value_periods.insert(0, PresentValuePeriod::new(periods-period, periodic_rate, future_value, period_value, present_value));
    }
    present_value_periods
}

fn check_future_value_parameters(periodic_rate: f64, periods: u32, future_value: f64) {
    assert!(periodic_rate.is_finite(), "The periodic_rate must be finite (not NaN or infinity)");
    assert!(periodic_rate > -1.0, "The periodic_rate must be greater than -1.0 because a rate lower than -100% would mean the investment loses more than its full value in a period.");
    if periodic_rate.abs() > 1. {
        warn!("You provided a periodic rate ({}) greater than 1. Are you sure you expect a {}% return?", periodic_rate, periodic_rate * 100.0);
    }
    assert!(periods >= 1);
    assert!(future_value.is_finite(), "The future value must be finite (not NaN or infinity)");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    
    #[test]
    fn test_present_value_solution_1() {
        let periodic_rate = 0.08;
        let future_value = 20_629.37;
        let periods = 6;
        let expected_value_solution = 13_000.0; // google sheet
        let actual_value_solution = present_value_solution(periodic_rate, periods, future_value).present_value;
        assert_eq!(round_to_cent(expected_value_solution), round_to_cent(actual_value_solution));
    }

    #[test]
    fn test_present_value_solution_2() {
        // test different types
        let periodic_rate = 0.08;
        let future_value = 20_629.37;
        let periods = 6;
        let expected_value_solution = 13_000.0; // google sheet
        let actual_value_solution = present_value_solution(periodic_rate, periods, future_value).present_value;
        assert_eq!(round_to_cent(expected_value_solution), round_to_cent(actual_value_solution));
    }
    #[test]
    fn test_present_value_solution_3() {
        // test negative periodic_rate
        let periodic_rate = -0.09;
        let future_value = 5_000;
        let periods = 6;
        let expected_value_solution = 8_804.84368898; // google sheet
        let actual_value_solution = present_value_solution(periodic_rate, periods, future_value).present_value;
        assert_eq!(round_to_cent(expected_value_solution), round_to_cent(actual_value_solution));
    }

    #[test]
    fn test_present_value_solution_4() {
        // test negative future value_solution 
        let periodic_rate = 0.09;
        let future_value = -5_000;
        let periods = 6;
        let _should_panic = present_value_solution(periodic_rate, periods, future_value).present_value;
    }

    #[should_panic]
    #[test]
    fn test_present_value_solution_5() {
        // test infinity on periodic_rate
        let periodic_rate = 1.0f64 / 0.0;
        let future_value = 5_000;
        let periods = 6;
        let _should_panic = present_value_solution(periodic_rate, periods, future_value).present_value;
    }

    #[should_panic]
    #[test]
    fn test_present_value_solution_6() {
        // test infinity on fv
        let periodic_rate = 0.03;
        let future_value = 1.0 / 0.0;
        let periods = 6;
        let _should_panic = present_value_solution(periodic_rate, periods, future_value).present_value;
    }

    #[test]
    fn test_present_value_solution_7() {
        // test various negative periodic_rates, pv should be > fv
        let periodic_rate = -0.03;
        let future_value = 5000.0;
        let periods = 12;
        let try_1 = present_value_solution(periodic_rate, periods, future_value).present_value;
        assert!(try_1 > future_value);
        let periodic_rate = -0.9;
        let try_2 = present_value_solution(periodic_rate, periods, future_value).present_value;
        assert!(try_2 > future_value);

        let periodic_rate = -3.2;
        let result = std::panic::catch_unwind(|| present_value_solution(periodic_rate, periods, future_value));
        assert!(result.is_err());  //probe further for specific error type here, if desired

        let periodic_rate = -1.00;
        let result = std::panic::catch_unwind(|| present_value_solution(periodic_rate, periods, future_value));
        assert!(result.is_err());  //probe further for specific error type here, if desired
    }

    #[test]
    fn test_present_value_solution_8() {
        // test periodic_rate of 100%
        let periodic_rate = 1.00;
        let future_value = 5_000;
        let periods = 12;
        let expected_value_solution = 1.22070313; // google sheet
        let actual_value_solution = present_value_solution(periodic_rate, periods, future_value).present_value;
        assert_eq!(round_to_cent(expected_value_solution), round_to_cent(actual_value_solution));
    }

    #[test]
    fn test_present_value_solution_9() {
        // test periodic_rate over 100%
        let periodic_rate = 3.00;
        let future_value = 5_000_000;
        let periods = 12;
        let expected_value_solution = 0.298023223876953; // google sheet
        let actual_value_solution = present_value_solution(periodic_rate, periods, future_value).present_value;
        assert_eq!(round_to_cent(expected_value_solution), round_to_cent(actual_value_solution));
    }

    #[test]
    fn test_present_value_solution_10() {
        // test fractional future value_solution
        let periodic_rate = 0.13;
        let future_value = 0.75;
        let periods = 9;
        let expected_value_solution = 0.249663625036891; // google sheet
        let actual_value_solution = present_value_solution(periodic_rate, periods, future_value).present_value;
        assert_eq!(round_to_cent(expected_value_solution), round_to_cent(actual_value_solution));
    }
    
}
