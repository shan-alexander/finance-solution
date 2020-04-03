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
/// period, expressed as a floating point number. For instance 0.05 would mean
/// 5% growth. Often appears as `r` or `i` in formulas.
/// * `periods` - The number of periods such as quarters or years. Often appears
/// as `n` or `t`.
/// * `present_value` - The starting value of the investment. May appear as `pv`
/// in formulas, or `C` for cash flow or `P` for principal.
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
/// // The investment will grow for 5 quarters.
/// let periods = 5;
///
/// // The initial investment is $250,000.
/// let present_value = 250_000;
///
/// let future_value = finance::future_value(periodic_rate, periods, present_value);
/// // Confirm that the future value is correct to four decimal places (one
/// // hundredth of a cent).
/// assert_eq!(295_489.9418, finance::round_to_fraction_of_cent(future_value));
/// ```
/// Investment that loses money each year.
/// ```
/// // The investment loses 5% per year.
/// let periodic_rate = -0.05;
///
/// // The investment will shrink for 6 years.
/// let periods = 6;
///
/// // The initial investment is $10,000.75.
/// let present_value = 10_000.75;
///
/// let future_value = finance::future_value(periodic_rate, periods, present_value);
/// // Confirm that the future value is correct to the penny.
/// assert_eq!(7351.47, finance::round_to_cent(future_value));
/// ```
/// Error case: The investment loses 105% per year. There's no way to work out what this
/// means so the call to future_value() will panic.
/// ```should_panic
/// let periodic_rate = -1.05;
/// let periods = 6;
/// let present_value = 10_000.75;
/// let future_value = finance::future_value(periodic_rate, periods, present_value);
/// ```
pub fn future_value<T>(periodic_rate: f64, periods: u32, present_value: T) -> f64
    where T: Into<f64> + Copy
{
    let present_value = present_value.into();
    check_present_value_parameters(periodic_rate, periods, present_value);

    present_value * (1.0 + periodic_rate).powi(periods as i32)
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
        // The formula field is here to explain the calculation. It's not used by any code.
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
/// period, expressed as a floating point number. For instance 0.05 would mean
/// 5% growth. Often appears as `r` or `i` in formulas.
/// * `periods` - The number of periods such as quarters or years. Often appears
/// as `n` or `t`.
/// * `present_value` - The starting value of the investment. May appear as `pv`
/// in formulas, or `C` for cash flow or `P` for principal.
///
/// # Panics
/// The call will fail if `periodic_rate` is less than -1.0 as this would mean
/// the investment is losing more than its full value every period.
///
/// # Examples
/// Create a collection of future value calculations ranging over several interest rates.
/// ```
/// // The investment will grow for 12 years.
/// let periods = 12;
///
/// // The initial investment is $100,000.
/// let present_value = 100_000;
///
/// // We'll keep a collection of the calculated future values along with their inputs.
/// let mut scenarios = vec![];
/// for i in 2..=15 {
///     // The rate is between 2% and 15% per year.
///     let periodic_rate = i as f64 / 100.0;
///     // Calculate the future value for this periodic rate and add the details to the collection.
///     scenarios.push(finance::future_value_solution(periodic_rate, periods, present_value));
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
pub fn future_value_solution<T>(periodic_rate: f64, periods: u32, present_value: T) -> FutureValueSolution
    where T: Into<f64> + Copy
{
    let future_value = future_value(periodic_rate, periods, present_value);
    FutureValueSolution::new(periodic_rate, periods, present_value.into(), future_value)
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
/// period, expressed as a floating point number. For instance 0.05 would mean
/// 5% growth. Often appears as `r` or `i` in formulas.
/// * `periods` - The number of periods such as quarters or years. Often appears
/// as `n` or `t`.
/// * `present_value` - The starting value of the investment. May appear as `pv`
/// in formulas, or `C` for cash flow or `P` for principal.
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
pub fn future_value_series<T>(periodic_rate: f64, periods: u32, present_value: T) -> Vec<FutureValuePeriod>
    where T: Into<f64> + Copy
{
    let present_value = present_value.into();
    check_present_value_parameters(periodic_rate, periods, present_value);

    let interest_multiplier = 1.0 + periodic_rate;

    // This is the future value at the end of all periods.
    let future_value = present_value * interest_multiplier.powi(periods as i32);

    // Start with period 0 which represents the starting values. Since no time has passed,
    // period_value is the same as present_value.
    let mut future_value_series = vec![FutureValuePeriod::new(0, periodic_rate, present_value, present_value, future_value)];
    // Add the values at each period.
    for period in 1..=periods {
        let period_value = present_value * interest_multiplier.powi(period as i32);
        future_value_series.push(FutureValuePeriod::new(period, periodic_rate, present_value, period_value, future_value));
    }
    future_value_series
}

/// The value of an investment over several periods with varying rates,
/// produced by calling [`future_value_schedule`].
#[derive(Debug)]
pub struct FutureValueSchedule {
    pub periods: u32,
    pub present_value: f64,
    pub future_value: f64,
    pub schedule_periods: Vec<FutureValueSchedulePeriod>,
}

/// The value of an investment at the end of a given period. This is part of
/// the [`FutureValueSchedule`] produced by calling [`future_value_schedule`].
pub struct FutureValueSchedulePeriod {
    pub period: u32,
    pub periodic_rate: f64,
    pub value: f64,
}

impl FutureValueSchedule {
    fn new(periods: u32, present_value: f64, future_value: f64) -> Self {
        let schedule = Self {
            periods,
            present_value,
            future_value,
            schedule_periods: vec![],
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
    let present_value= present_value.into();
    let periods = periodic_rates.len();

    // Check the parameters including all of the provided rates.
    for rate in periodic_rates {
        check_present_value_parameters(*rate, periods as u32, present_value);
    }

    // Calculated the value after each period. The first entry for period 0 is the present value.
    let mut period_values = vec![present_value];
    for period in 1..=periods {
        // The value for this period is the value of the previous period adjusted by the rate for
        // this period.
        let previous_value = period_values[period-1];
        let rate = periodic_rates[period-1];
        let period_value = previous_value * (1.0 + rate);
        period_values.push(period_value);
    }

    // The overall future value is the same as the value at the end of the last period.
    let future_value = period_values[periods];

    let mut schedule = FutureValueSchedule::new(periods as u32, present_value, future_value);
    for period in 1..=periods {
        let rate = periodic_rates[period - 1];
        let value = period_values[period];
        schedule.schedule_periods.push(FutureValueSchedulePeriod::new(period as u32, rate, value));
    }
    schedule
}

fn check_present_value_parameters(periodic_rate: f64, periods: u32, present_value: f64) {
    assert!(periodic_rate.is_finite());
    assert!(periodic_rate >= -1.);
    assert!(periods >= 1);
    assert!(present_value.is_finite());
    if periodic_rate.abs() > 1. {
        warn!("You provided a periodic rate ({}) greater than 1. Are you sure you expect a {}% return?", periodic_rate, periodic_rate * 100.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_future_value_nominal() {
        assert_eq!(295_489.9418, round_to_fraction_of_cent(future_value(0.034, 5, 250_000.00)));
        assert_eq!(20_629.3662, round_to_fraction_of_cent(future_value(0.08, 6, 13_000.0)));
        assert_eq!(5_000.0000, round_to_fraction_of_cent(future_value(-0.09, 6, 8_804.84368898)));
    }

    #[should_panic]
    #[test]
    fn test_future_value_error_rate_low() {
        future_value(-101.0, 5, 250_000.00);
    }

    #[test]
    fn test_future_value_solution_1() {
        let rate_of_return = 0.034;
        let periods = 5;
        let present_value_1 = 250_000.00;
        let expected_value = 295489.941778856;
        let actual_value = future_value_solution(rate_of_return, periods, present_value_1).future_value;
        assert_eq!(round_to_cent(expected_value), round_to_cent(actual_value));
        assert!( float_cmp::approx_eq!(f64, expected_value, actual_value, ulps = 4) );
    }

    #[test]
    fn test_future_value_solution_2() {
        let rate_of_return = 0.08;
        let periods = 6;
        let present_value_1 = 13_000.0;
        let expected_value = 20_629.37;
        let actual_value = future_value_solution(rate_of_return, periods, present_value_1).future_value;
        assert_eq!(round_to_cent(expected_value), round_to_cent(actual_value));
    }

    #[test]
    fn test_future_value_solution_3() {
        // test negative rate
        let rate_of_return = -0.09;
        let periods = 6;
        let present_value = 8_804.84368898;
        let expected_value = 5_000.00;
        let actual_value = future_value_solution(rate_of_return, periods, present_value).future_value;
        assert_eq!(round_to_cent(expected_value), round_to_cent(actual_value));
    }

    #[should_panic]
    #[test]
    fn test_future_value_solution_5() {
        // test zero for periods
        let rate_of_return = 0.09;
        let periods = 0;
        let present_value = 5_000.00;
        let _should_panic = future_value_solution(rate_of_return, periods, present_value);
    }


    #[should_panic]
    #[test]
    fn test_future_value_solution_6() {
        // test infinity on rate
        let rate_of_return = 1.0f64 / 0.0f64;
        let periods = 6;
        let present_value = 5_000.00;
        let _should_panic = future_value_solution(rate_of_return, periods, present_value);
    }

    #[should_panic]
    #[test]
    fn test_future_value_solution_7() {
        // test infinity on fv
        let rate_of_return = 0.03;
        let periods = 6;
        let present_value = 1.0f64 / 0.0f64;
        let _should_panic = future_value_solution(rate_of_return, periods, present_value);
    }

    #[test]
    fn test_future_value_solution_8() {
        // test various negative rates, pv should be > fv
        let rate_of_return = -0.03;
        let periods = 12;
        let present_value = 5000.00;
        let try_1 = future_value_solution(rate_of_return, periods, present_value).future_value;
        assert!(try_1 < present_value.into());

        let rate_of_return = -0.9;
        let try_2 = future_value_solution(rate_of_return, periods, present_value).future_value;
        assert!(try_2 < present_value.into());

        let rate_of_return = -3.2;
        let result = std::panic::catch_unwind(|| future_value_solution(rate_of_return, periods, present_value));
        assert!(result.is_err());  //probe further for specific error type here, if desired
    }

}

