//! Present value calculations. Given a final amount, a number of periods such as years, and fixed
//! or varying interest periodic_rates, what is the current value?

use log::warn;
use std::fmt::{self, Debug};

use crate::shared;

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
/// // The investment will grow by 1.1% per month.
/// let periodic_rate = 0.011;
///
/// // The investment will grow for 12 months.
/// let periods = 12;
///
/// // The final value will be $50,000.
/// let future_value = 50_000;
///
/// // Find the current value.
/// let present_value = finance::present_value(periodic_rate, periods, future_value);
/// dbg!(&present_value);
///
/// // Confirm that the present value is correct to four decimal places (one hundredth of a cent).
/// assert_eq!(43848.6409, finance::round_to_fraction_of_cent(present_value));
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
    check_present_value_parameters(periodic_rate, periods, future_value);

    future_value / (1. + periodic_rate).powi(periods as i32)
}

/// A record of a present value calculation produced by calling [`present_value_solution`].
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
/// Create a collection of present value calculations where the future value and periodic rate are
/// fixed but the number of periods varies, then filter the results.
/// ```
/// // The rate is 0.9% per month.
/// let periodic_rate = 0.009;
///
/// // The final value is $100,000.
/// let future_value = 100_000;
///
/// // We'll keep a collection of the calculated present values along with their inputs.
/// let mut scenarios = vec![];
/// // Calculate the present value for terms ranging from 1 to 36 months.
/// for periods in 1..=36 {
///     // Calculate the future value for this number of months and add the details to the
///     // collection.
///     scenarios.push(finance::present_value_solution(periodic_rate, periods, future_value));
/// }
/// dbg!(&scenarios);
/// assert_eq!(36, scenarios.len());
///
/// // Keep only the scenarios where the present value is less than or equal to $80,000.
/// scenarios.retain(|x| x.present_value <= 80_000.00);
/// dbg!(&scenarios);
/// assert_eq!(12, scenarios.len());
///
/// // Find the range of months for the remaining scenarios.
/// let min_months = scenarios.iter().map(|x| x.periods).min().unwrap();
/// let max_months = scenarios.iter().map(|x| x.periods).max().unwrap();
/// dbg!(min_months, max_months);
/// assert_eq!(25, min_months);
/// assert_eq!(36, max_months);
///
/// // Check the formula for the first scenario.
/// dbg!(&scenarios[0].formula);
/// assert_eq!("100000 / (1 + 0.009)^25", scenarios[0].formula);
/// ```
/// Error case: The investment loses 111% per year. There's no way to work out what this means so
/// the call to present_value() will panic.
/// ```should_panic
/// let periodic_rate = -1.11;
/// let periods = 12;
/// let present_value = 100_000.85;
/// let present_value = finance::present_value_solution(periodic_rate, periods, present_value);
/// ```
pub fn present_value_solution<T>(periodic_rate: f64, periods: u32, future_value: T) -> PresentValueSolution
    where T: Into<f64> + Copy
{
    let present_value = present_value(periodic_rate, periods, future_value);
    PresentValueSolution::new(periodic_rate, periods, present_value, future_value.into())
}

/// The value of an investment after a given period produced by calling
/// [`present_value_series`].
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

/// Calculates a present value including the value at the end of each period. This is used to show
/// the growth or decline of an investment step by step.
///
/// Related functions:
/// For simply calculating a single present value use [`present_value`].
/// To calculate a present value with a fixed rate while retaining the input values use
/// * [`present_value_solution`].
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
/// Calculate a present value including the value at the end of each period, then filter the results.
/// ```
/// // The interest rate is 7.8% per year.
/// let interest_rate = 0.078;
///
/// // The investment will grow for 10 years.
/// let periods = 10;
///
/// // The final value is $8112.75.
/// let future_value = 8_112.75;
///
/// // Calculate the present value as well as the value at the end of each year.
/// let series = finance::present_value_series(interest_rate, periods, future_value);
/// dbg!(&series);
///
/// // Confirm that we have one entry for the present value, that is the initial value before any
/// // interest is applied, and one entry for each period.
/// assert_eq!(11, series.len());
///
/// // Create a reduced vector with every other period not including period 0,
/// // the initial state.
/// let filtered_series = series
///     .iter()
///     .filter(|x| x.period % 2 == 0 && x.period != 0)
///     .collect::<Vec<_>>();
/// dbg!(&filtered_series);
/// assert_eq!(5, filtered_series.len());
/// ```
pub fn present_value_series<T>(periodic_rate: f64, periods: u32, future_value: T) -> Vec<PresentValuePeriod>
    where T: Into<f64> + Copy
{
    let future_value = future_value.into();
    check_present_value_parameters(periodic_rate, periods, future_value);

    let present_value = present_value(periodic_rate, periods, future_value);
    // Start with an entry for period 0 representing the starting value of the investment.
    let mut present_value_periods = vec![PresentValuePeriod::new(0, periodic_rate, future_value, present_value, present_value)];
    for period in 1..=periods {
        // In order to build the vector in a straightforward manner from beginning to end starting
        // at period 1, call future_value() at each step instead of present_value().
        let period_value = crate::future_value(periodic_rate, period, present_value);
        present_value_periods.push(PresentValuePeriod::new(period, periodic_rate, future_value, period_value, present_value));
    }
    present_value_periods
}

/// The value of an investment over several periods with a known future value and varying rates,
/// produced by calling [`present_value_schedule`].
#[derive(Debug)]
pub struct PresentValueSchedule {
    pub periods: u32,
    pub future_value: f64,
    pub present_value: f64,
    pub schedule_periods: Vec<shared::SchedulePeriod>,
}

impl PresentValueSchedule {
    fn new(periods: u32, future_value: f64, present_value: f64) -> Self {
        let schedule = Self {
            periods,
            future_value,
            present_value,
            schedule_periods: vec![],
        };
        schedule
    }
}

/// Calculates a present value based on rates that change for each period.
///
/// Related functions:
/// * For simply calculating a single present value with a fixed rate use [`present_value`].
/// * To calculate a present value with a fixed rate while retaining the input values use
/// [`present_value_solution`].
/// * To calculate the value for each period instead of only the final value use
/// [`present_value_series`].
///
/// The present value formula is:
///
/// present_value = future_value / (1 + periodic_rate)<sup>periods</sup>
///
/// # Arguments
/// * `periodic_rates` - A collection of rates, one for each period.
/// * `future_value` - The ending value of the investment.
///
/// # Panics
/// The call will fail if any of the rates is less than -1.0 as this would mean the investment is
/// losing more than its full value.
///
/// # Examples
/// Calculate the value of an investment whose rates vary by year, then view only those periods
/// where the rate is negative.
/// ```
/// // The annual rate varies from -12% to 11%.
/// let rates = [0.04, 0.07, -0.12, -0.03, 0.11];
///
/// // The value of the investment after applying all of these periodic rates will be $100_000.25.
/// let future_value = 100_000.25;
///
/// // Calculate the present value as well as the value at the end of each period.
/// let schedule = finance::present_value_schedule(&rates, future_value);
/// dbg!(&schedule);
/// assert_eq!(5, schedule.schedule_periods.len());
///
/// // Create a filtered list of periods, only those with a negative rate.
/// let filtered_periods = schedule.schedule_periods
///     .iter()
///     .filter(|x| x.periodic_rate < 0.0)
///     .collect::<Vec<_>>();
/// dbg!(&filtered_periods);
/// assert_eq!(2, filtered_periods.len());
/// ```
pub fn present_value_schedule<T>(periodic_rates: &[f64], future_value: T) -> PresentValueSchedule
    where T: Into<f64> + Copy
{
    let periods = periodic_rates.len();
    let future_value = future_value.into();

    // Check the parameters including all of the provided rates.
    for rate in periodic_rates {
        check_present_value_parameters(*rate, periods as u32, future_value);
    }

    // Calculate the value after each period, starting from the end. We'll end up with a period 0
    // at the beginning representing the starting value of the investment before any interest has
    // been applied.
    let mut period_values = vec![future_value];
    for period in (0..periods).rev() {
        // The value for this period is the value of the following period adjusted by the rate for
        // that period. At each step the following period is the first entry in the period_value
        // vector.
        let following_value = period_values[0];
        let following_rate = periodic_rates[period];
        let period_value = following_value / (1.0 + following_rate);
        period_values.insert(0, period_value);
    }

    // The overall present value is the same as the value at period 0.
    let present_value = period_values[0];

    let mut schedule = PresentValueSchedule::new(periods as u32, future_value, present_value);
    for period in 1..=periods {
        let rate = periodic_rates[period - 1];
        let value = period_values[period];
        schedule.schedule_periods.push(shared::SchedulePeriod::new(period as u32, rate, value));
    }
    schedule
}

fn check_present_value_parameters(periodic_rate: f64, periods: u32, future_value: f64) {
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

    #[test]
    fn test_present_value_schedule() {
        let rates = [0.04, 0.07, -0.12, -0.03, 0.11];
        let future_value = 100_000.25;
        let schedule = present_value_schedule(&rates, future_value);
        assert_eq!(100000.25, round_to_fraction_of_cent(schedule.future_value));
        assert_eq!(94843.2841, round_to_fraction_of_cent(schedule.present_value));
        assert_eq!(5, schedule.schedule_periods.len());

        let period_index = 0;
        assert_eq!(1, schedule.schedule_periods[period_index].period);
        assert_eq!(0.04, schedule.schedule_periods[period_index].periodic_rate);
        assert_eq!(98_637.0154, round_to_fraction_of_cent(schedule.schedule_periods[period_index].value));

        let period_index = 1;
        assert_eq!(2, schedule.schedule_periods[period_index].period);
        assert_eq!(0.07, schedule.schedule_periods[period_index].periodic_rate);
        assert_eq!(105_541.6065, round_to_fraction_of_cent(schedule.schedule_periods[period_index].value));

        let period_index = 2;
        assert_eq!(3, schedule.schedule_periods[period_index].period);
        assert_eq!(-0.12, schedule.schedule_periods[period_index].periodic_rate);
        assert_eq!(92_876.6137, round_to_fraction_of_cent(schedule.schedule_periods[period_index].value));

        let period_index = 3;
        assert_eq!(4, schedule.schedule_periods[period_index].period);
        assert_eq!(-0.03, schedule.schedule_periods[period_index].periodic_rate);
        assert_eq!(90_090.3153, round_to_fraction_of_cent(schedule.schedule_periods[period_index].value));

        let period_index = 4;
        assert_eq!(5, schedule.schedule_periods[period_index].period);
        assert_eq!(0.11, schedule.schedule_periods[period_index].periodic_rate);
        assert_eq!(100_000.2500, round_to_fraction_of_cent(schedule.schedule_periods[period_index].value));
    }

}

