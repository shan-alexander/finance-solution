//! Future value calculations. Given an initial investment amount, a number of
//! periods such as years, and fixed or varying interest rates, what is the
//! value of the investment at the end?

use log::warn;
use std::fmt::{self, Debug};

/// Returns the value of an investment after it has grown or shrunk over time,
/// using a fixed rate.
///
/// To calculate a future value with a fixed rate while
/// retaining the input values use [`future_value_solution`]. To calculate
///  the value for each period instead of only the final value use
/// [`future_value_series`]. If the rates vary by period use
/// [`future_value_schedule`].
///
/// The formula is:
///
/// future_value = present_value * (1 + periodic_rate)<sup>periods</sup>
///
/// # Arguments
/// * `periodic_rate` - The rate at which the investment grows or shrinks per
/// period, expressed as a floating point number. For instance 0.05 would mean 5% growth.
/// * `present_value` - The starting value of the investment.
/// * `periods` - The number of periods such as quarters or years.
///
/// # Panics
/// The call will fail if `periodic_rate` is less than -1.0 as this would mean
/// the investment is losing more than its full value every period.
///
/// # Examples
/// Investment that grows quarter by quarter.
/// ```
/// // The investment grows by 3.4% per quarter.
/// let periodic_rate = 0.034;
///
/// // The initial investment is $250,000.
/// let present_value = 250_000;
///
/// // The investment will grow for 5 quarters.
/// let periods = 5;
///
/// let future_value = finance::future_value(periodic_rate, present_value, periods);
/// // Confirm that the future value is correct to four decimal places (one
/// // hundredth of a cent).
/// assert_eq!(295_489.9418, finance::round_to_fraction_of_cent(future_value));
/// ```
/// Investment that loses money each year.
/// ```
/// // The investment loses 5% per year.
/// let periodic_rate = -0.05;
///
/// // The initial investment is $10,000.75.
/// let present_value = 10_000.75;
///
/// // The investment will shrink for 6 years.
/// let periods = 6;
///
/// let future_value = finance::future_value(periodic_rate, present_value, periods);
/// // Confirm that the future value is correct to the penny.
/// assert_eq!(7351.47, finance::round_to_cent(future_value));
/// ```
/// Error case: The investment loses 105% per year. There's no way to work out what this
/// means so the call to future_value() will panic.
/// ```should_panic
/// let periodic_rate = -1.05;
/// let present_value = 10_000.75;
/// let periods = 6;
/// let future_value = finance::future_value(periodic_rate, present_value, periods);
/// ```
pub fn future_value<T>(periodic_rate: f64, present_value: T, periods: u32) -> f64
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
        warn!("You provided a periodic rate ({}) greater than 1. Are you sure you expect a {}% return?", r, r*100.0);
    }
    // final computation for future value
    pv * (1. + r).powf(n as f64)
}

/// A record of a future value calculation produced by calling [`future_value_solution`].
pub struct FutureValueSolution {
    pub periodic_rate: f64,
    pub periods: u32,
    pub present_value: f64,
    pub future_value: f64,
    pub formula: String,
}

impl FutureValueSolution {
    fn new(periodic_rate: f64, periods: u32, present_value: f64, future_value: f64) -> Self {
        let formula = format!("{} * (1 + {})^{}", present_value, periodic_rate, periods);
        Self {
            periodic_rate,
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
               &format!("periodic_rate: {:.6}", self.periodic_rate),
               &format!("periods: {}", self.periods),
               &format!("present_value: {:.4}", self.present_value),
               &format!("future_value: {:.4}", self.future_value),
               &format!("formula: {:?}", self.formula),
        )
    }
}

// type FV = FutureValueSolution; // Creates a type alias

/// Calculates the value of an investment after it has grown or shrunk over time and returns a
/// struct with the inputs and the calculated value. This is used for keeping track of a collection
/// of financial scenarios so that they can be examined later.
///
/// For simply calculating a single future value use [`future_value`]. To
/// calculate the value for each period instead of only the final value use
/// [`future_value_series`]. To calculate a future value with a fixed rate
/// while retaining the input values use [`future_value_solution`]. If the
/// rates vary by period use [`future_value_schedule`].
///
/// The future value formula is:
///
/// future_value = present_value * (1 + periodic_rate)<sup>periods</sup>
///
/// # Arguments
/// * `periodic_rate` - The rate at which the investment grows or shrinks per
/// period, expressed as a floating point number. For instance 0.05 would mean 5% growth.
/// * `present_value` - The starting value of the investment.
/// * `periods` - The number of periods such as quarters or years.
///
/// # Panics
/// The call will fail if `periodic_rate` is less than -1.0 as this would mean
/// the investment is losing more than its full value every period.
///
/// # Examples
/// Create a collection of future value calculations ranging over several interest rates.
/// ```
/// // The initial investment is $100,000.
/// let present_value = 100_000;
///
/// // The investment will grow for 12 years.
/// let periods = 12;
///
/// // We'll keep a collection of the calculated future values along with their inputs.
/// let mut scenarios = vec![];
/// for i in 2..=15 {
///     // The rate is between 2% and 15% per year.
///     let periodic_rate = i as f64 / 100.0;
///     // Calculate the future value for this periodic rate and add the details to the collection.
///     scenarios.push(finance::future_value_solution(periodic_rate, present_value, periods));
/// }
/// dbg!(&scenarios);
/// assert_eq!(14, scenarios.len());
///
/// // Keep only the scenarios where the future value was between $200,000 and $400,000.
/// scenarios.retain(|x| x.future_value >= 200_000.00 && x.future_value <= 400_000.00);
/// dbg!(&scenarios);
/// assert_eq!(7, scenarios.len());
///
/// // Check the formula for the first scenario.
/// dbg!(&scenarios[0].formula);
/// assert_eq!("100000 * (1 + 0.06)^12", scenarios[0].formula);
/// ```
pub fn future_value_solution<T>(periodic_rate: f64, present_value: T, periods: u32) -> FutureValueSolution
    where T: Into<f64> + Copy
{
    FutureValueSolution::new(periodic_rate, periods, present_value.into(), future_value(periodic_rate, present_value, periods))
}

/// The value of an investment after a given period produced by calling
/// [`future_value_series`].
pub struct FutureValuePeriod {
    pub period: u32,
    pub periodic_rate: f64,
    pub present_value: f64,
    pub period_value: f64,
    pub future_value: f64,
}

impl FutureValuePeriod {
    fn new(period: u32, periodic_rate: f64, present_value: f64, period_value: f64, future_value: f64) -> Self {
        Self {
            period,
            periodic_rate,
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
               &format!("periodic_rate: {:.6}", self.periodic_rate),
               &format!("present_value: {:.4}", self.present_value),
               &format!("period_value: {:.4}", self.period_value),
               &format!("future_value: {:.4}", self.future_value),
        )
    }
}

/// Calculates a future value including the value at the end of each period. This is used to show
/// the growth or decline of an investment step by step.
///
/// For simply calculating a single future value use [`future_value`].
///
///
/// If the rates vary by period use
///// [`future_value_schedule`].
///
/// The future value formula is:
///
/// future_value = present_value * (1 + periodic_rate)<sup>periods</sup>
///
/// # Arguments
/// * `periodic_rate` - The rate at which the investment grows or shrinks per
/// period, expressed as a floating point number. For instance 0.05 would mean 5% growth.
/// * `present_value` - The starting value of the investment.
/// * `periods` - The number of periods such as quarters or years.
///
/// # Panics
/// The call will fail if `periodic_rate` is less than -1.0 as this would mean
/// the investment is losing more than its full value every period.
///
/// # Examples
/// Calculate a future value including the value at the end of each period, then filter the
/// results.
/// ```
/// // The initial investment is $10,000.12.
/// let present_value = 10_000.12;
///
/// // The interest rate is 1.5% per month.
/// let periodic_rate = 0.015;
///
/// // The investment will grow for 24 months.
/// let periods = 24;
///
/// let results = finance::future_value_series(periodic_rate, present_value, periods);
/// dbg!(&results);
///
/// // Confirm that we have one entry for the initial value and one entry for
/// // each period.
/// assert_eq!(25, results.len());
///
/// // Create a reduced vector with every fourth period. This will include
/// // period 0 representing the initial value of the investment.
/// let filtered_results = results
///     .iter()
///     .filter(|x| x.period % 4 == 0)
///     .collect::<Vec<_>>();
/// dbg!(&filtered_results);
/// assert_eq!(7, filtered_results.len());
/// ```
pub fn future_value_series<T>(periodic_rate: f64, present_value: T, periods: u32) -> Vec<FutureValuePeriod>
    where T: Into<f64> + Copy
{
    // pv, n, and r are the standard shorthands for the three values used in the calculation.
    let pv = present_value.into();
    let r = periodic_rate;
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

/// The value of an investment over several periods with varying rates,
/// produced by calling [`future_value_schedule`].
#[derive(Debug)]
pub struct FutureValueSchedule {
    pub num_periods: u32,
    pub present_value: f64,
    pub future_value: f64,
    pub periods: Vec<FutureValueSchedulePeriod>,
}

/// The value of an investment at the end of a given period. This is part of
/// the [`FutureValueSchedule`] produced by calling [`future_value_schedule`].
pub struct FutureValueSchedulePeriod {
    pub period: u32,
    pub periodic_rate: f64,
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
    fn new(period: u32, periodic_rate: f64, value: f64) -> Self {
        Self { period, periodic_rate, value }
    }
}

impl Debug for FutureValueSchedulePeriod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ {}, {}, {} }}",
               &format!("period: {}", self.period),
               &format!("periodic_rate: {:.6}", self.periodic_rate),
               &format!("value: {:.4}", self.value),
        )
    }
}

/// Calculates a future value based on rates that change for each period.
///
/// For simply calculating a single future value with a fixed rate use
/// [`future_value`]. To calculate a future value with a fixed rate while
/// retaining the input values use [`future_value_solution`]. To calculate
/// the value for each period instead of only the final value use
/// [`future_value_series`].
///
/// The future value formula is:
///
/// future_value = present_value * (1 + periodic_rate)<sup>periods</sup>
///
/// # Arguments
/// * `periodic_rates` - A collection of rates, one for each period.
/// * `present_value` - The starting value of the investment.
///
/// # Panics
/// The call will fail if any of the rates is less than -1.0 as this would mean
/// the investment is losing more than its full value.
///
/// # Examples
/// Calculate the value of an investment whose rates vary by year, then find the
/// point where the value passes a certain threshold.
/// ```
/// // The rates vary by year: 11.6% followed by 13.4%, a 9% drop, and an 8.6% gain.
/// let periodic_rates = [0.116, 0.134, -0.09, 0.086];
///
/// // The initial investment is $50,000.
/// let present_value = 50_000.00;
///
/// let schedule = finance::future_value_schedule(&periodic_rates, present_value);
/// dbg!(&schedule);
///
/// assert_eq!(62534.3257, finance::round_to_fraction_of_cent(schedule.future_value));
///
/// // Confirm that there are four periods, corresponding to the four interest
/// // rates.
/// assert_eq!(4, schedule.periods.len());
///
/// // Confirm that the value of the fourth period is the same as the overall
/// // future value.
/// assert_eq!(schedule.future_value, schedule.periods.last().unwrap().value);
///
/// // Find the first period where the value of the investment was at least
/// // $60,000.
/// let period = schedule.periods.iter().find(|x| x.value >= 60_000.00);
/// dbg!(&period);
/// assert_eq!(2, period.unwrap().period);
/// ```
/// Error case: One of the rates shows a drop of over 100%. There's no way to work out what this
/// means so the call to future_value_schedule() will panic.
/// ```should_panic
/// let periodic_rates = [0.116, -100.134, -0.09, 0.086];
/// let present_value = 4_000.00;
/// let schedule = finance::future_value_schedule(&periodic_rates, present_value);
/// ```
pub fn future_value_schedule<T>(periodic_rates: &[f64], present_value: T) -> FutureValueSchedule
    where T: Into<f64> + Copy
{
    // assertions to ensure valid financial computation
    for rate in periodic_rates {
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
    let num_periods = periodic_rates.len();
    let mut period_values = vec![pv];
    for period in 1..=num_periods {
        let period_value = period_values[period-1] * (1. + periodic_rates[period-1]);
        period_values.push(period_value);
    }
    let future_value = period_values[num_periods];
    // final computation for future value
    let mut schedule = FutureValueSchedule::new(num_periods as u32, pv, future_value);
    for period in 1..=num_periods {
        schedule.periods.push(FutureValueSchedulePeriod::new(period as u32, periodic_rates[period - 1], period_values[period]));
    }
    schedule
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_future_value_nominal() {
        assert_eq!(295_489.9418, round_to_fraction_of_cent(future_value(0.034, 250_000.00, 5)));
        assert_eq!(20_629.3662, round_to_fraction_of_cent(future_value(0.08, 13_000.0, 6)));
        assert_eq!(5_000.0000, round_to_fraction_of_cent(future_value(-0.09, 8_804.84368898, 6)));
    }

    #[should_panic]
    #[test]
    fn test_future_value_error_rate_low() {
        future_value(-101.0, 250_000.00, 5);
    }

    #[test]
    fn test_future_value_solution_1() {
        let rate_of_return = 0.034;
        let present_value_1 = 250_000.00;
        let periods = 5;
        let expected_value = 295489.941778856;
        let actual_value = future_value_solution(rate_of_return, present_value_1, periods).future_value;
        assert_eq!(round_to_cent(expected_value), round_to_cent(actual_value));
        assert!( float_cmp::approx_eq!(f64, expected_value, actual_value, ulps = 4) );
    }

    #[test]
    fn test_future_value_solution_2() {
        let rate_of_return = 0.08;
        let present_value_1 = 13_000.0;
        let periods = 6;
        let expected_value = 20_629.37;
        let actual_value = future_value_solution(rate_of_return, present_value_1, periods).future_value;
        assert_eq!(round_to_cent(expected_value), round_to_cent(actual_value));
        // assert!(exp_value.approx_eq(act_value, (0.0, 2)));

    }

    #[test]
    fn test_future_value_solution_3() {
        // test negative rate
        let rate_of_return = -0.09;
        let present_value = 8_804.84368898;
        let periods = 6;
        let expected_value = 5_000.00;
        let actual_value = future_value_solution(rate_of_return, present_value, periods).future_value;
        assert_eq!(round_to_cent(expected_value), round_to_cent(actual_value));
    }

    #[should_panic]
    #[test]
    fn test_future_value_solution_5() {
        // test zero for periods
        let rate_of_return = 0.09;
        let present_value = 5_000.00;
        let periods = 0;
        let _should_panic = future_value_solution(rate_of_return, present_value, periods);
    }


    #[should_panic]
    #[test]
    fn test_future_value_solution_6() {
        // test infinity on rate
        let rate_of_return = 1.0f64 / 0.0f64;
        let present_value = 5_000.00;
        let periods = 6;
        let _should_panic = future_value_solution(rate_of_return, present_value, periods);
    }

    #[should_panic]
    #[test]
    fn test_future_value_solution_7() {
        // test infinity on fv
        let rate_of_return = 0.03;
        let present_value = 1.0f64 / 0.0f64;
        let periods = 6;
        let _should_panic = future_value_solution(rate_of_return, present_value, periods);
    }

    #[test]
    fn test_future_value_solution_8() {
        // test various negative rates, pv should be > fv
        let rate_of_return = -0.03;
        let present_value = 5000.00;
        let periods = 12;
        let try_1 = future_value_solution(rate_of_return, present_value, periods).future_value;
        assert!(try_1 < present_value.into());

        let rate_of_return = -0.9;
        let try_2 = future_value_solution(rate_of_return, present_value, periods).future_value;
        assert!(try_2 < present_value.into());

        let rate_of_return = -3.2;
        let result = std::panic::catch_unwind(|| future_value_solution(rate_of_return, present_value, periods));
        assert!(result.is_err());  //probe further for specific error type here, if desired

    }

}

