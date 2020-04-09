//! Future value calculations. Given an initial investment amount, a number of periods such as
//! years, and fixed or varying interest rates, what is the value of the investment at the end?
//!
//! If you need to calculate the present value given a future value, a number of periods, and one
//! or more rates use [`present_value`] or related functions.
//!
//! If you need to calculate a fixed rate given a present value, future value, and number of periods
//! use [`rate`] or related functions.
//!
//! If you need to calculate the number of periods given a fixed rate and a present and future value
//! use [`periods`] or related functions.

use log::warn;

use crate::tvm_simple::*;

// Needed for the Rustdoc comments.
#[allow(unused_imports)]
use crate::{present_value::present_value, rate::rate, periods::periods};

/// Returns the value of an investment after it has grown or shrunk over time, using a fixed rate.
///
/// Related functions:
/// * To calculate a future value with a fixed rate and return a struct that shows the formula and
/// optionally produces the the period-by-period values use [`future_value_solution`].
/// * To calculate the future value if the rates vary by period use [`future_value_schedule`].
/// * To calculate the future value with varying rates and return a struct that can produce the
/// period-by-period values use [`future_value_schedule_solution`].
///
/// Functions to solve for other values:
/// * To calculate the present value given a future value, a number of periods, and one or more
/// rates use [`present_value`] or related functions.
/// * To calculate a fixed rate given a present value, future value, and number of periods use
/// [`rate`] or related functions.
/// * To calculate the number of periods given a fixed rate and a present and future value use
/// [`periods`] or related functions.
///
/// The formula is:
///
/// future_value = present_value * (1 + rate)<sup>periods</sup>
///
/// # Arguments
/// * `rate` - The rate at which the investment grows or shrinks per period, expressed as a
/// floating point number. For instance 0.05 would mean 5% growth. Often appears as `r` or `i` in
/// formulas.
/// * `periods` - The number of periods such as quarters or years. Often appears as `n` or `t`.
/// * `present_value` - The starting value of the investment. May appear as `pv` in formulas, or `C`
/// for cash flow or `P` for principal.
///
/// # Panics
/// The call will fail if `rate` is less than -1.0 as this would mean the investment is
/// losing more than its full value every period.
///
/// # Examples
/// Investment that grows quarter by quarter.
/// ```
/// // The investment grows by 3.4% per quarter.
/// let rate = 0.034;
///
/// // The investment will grow for 5 quarters.
/// let periods = 5;
///
/// // The initial investment is $250,000.
/// let present_value = 250_000;
///
/// let future_value = finance::future_value(rate, periods, present_value);
/// // Confirm that the future value is correct to four decimal places (one
/// // hundredth of a cent).
/// finance::assert_rounded_4(295_489.9418, future_value);
/// ```
/// Investment that loses money each year.
/// ```
/// // The investment loses 5% per year.
/// let rate = -0.05;
///
/// // The investment will shrink for 6 years.
/// let periods = 6;
///
/// // The initial investment is $10,000.75.
/// let present_value = 10_000.75;
///
/// let future_value = finance::future_value(rate, periods, present_value);
/// // Confirm that the future value is correct to the penny.
/// finance::assert_rounded_2(7351.47, future_value);
/// ```
/// Error case: The investment loses 105% per year. There's no way to work out
/// what this means so the call will panic.
/// ```should_panic
/// let rate = -1.05;
/// let periods = 6;
/// let present_value = 10_000.75;
/// let future_value = finance::future_value(rate, periods, present_value);
/// ```
pub fn future_value<T>(rate: f64, periods: u32, present_value: T) -> f64
    where T: Into<f64> + Copy
{
    let present_value = present_value.into();
    check_future_value_parameters(rate, periods, present_value);

    let future_value = present_value * (1.0 + rate).powi(periods as i32);
    assert!(future_value.is_finite());
    future_value
}

/// Calculates the value of an investment after it has grown or shrunk over time and returns a
/// struct with the inputs and the calculated value. This is used for keeping track of a collection
/// of financial scenarios so that they can be examined later.
///
/// Related functions:
/// * For simply calculating a single future value using a fixed rate use [`future_value`].
/// * To calculate the future value if the rates vary by period use [`future_value_schedule`].
/// * To calculate the future value with varying rates and return a struct that can produce the
/// period-by-period values use [`future_value_schedule_solution`].
///
/// The future value formula is:
///
/// future_value = present_value * (1 + rate)<sup>periods</sup>
///
/// # Arguments
/// * `rate` - The rate at which the investment grows or shrinks per period, expressed as a
/// floating point number. For instance 0.05 would mean 5% growth. Often appears as `r` or `i` in
/// formulas.
/// * `periods` - The number of periods such as quarters or years. Often appears as `n` or `t`.
/// * `present_value` - The starting value of the investment. May appear as `pv` in formulas, or `C`
/// for cash flow or `P` for principal.
///
/// # Panics
/// The call will fail if `rate` is less than -1.0 as this would mean the investment is
/// losing more than its full value every period.
///
/// # Examples
/// Calculate a future value and examine the period-by-period values.
/// ```
/// // The rate is 1.2% per month.
/// let rate = 0.012;
///
/// // The investment will grow for 8 months.
/// let periods = 8;
///
/// // The initial investment is $200,000.
/// let present_value = 200_000;
///
/// let solution = finance::future_value_solution(rate, periods, present_value);
/// dbg!(&solution);
///
/// let future_value = solution.future_value;
/// finance::assert_rounded_4(future_value, 220_026.0467);
///
/// let formula = solution.formula.clone();
/// assert_eq!(formula, "200000.0000 * (1.012000 ^ 8)");
///
/// // Calculate the value at the end of each period.
/// let series = solution.series();
/// dbg!(&series);
/// ```
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
///
/// for i in 2..=15 {
/// // The rate is between 2% and 15% per year.
///     let rate = i as f64 / 100.0;
///     // Calculate the future value for this periodic rate and add the details to the collection.
///     scenarios.push(finance::future_value_solution(rate, periods, present_value));
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
/// assert_eq!("100000.0000 * (1.060000 ^ 12)", scenarios[0].formula);
/// ```
pub fn future_value_solution<T>(rate: f64, periods: u32, present_value: T) -> TvmSolution
    where T: Into<f64> + Copy
{
    let future_value = future_value(rate, periods, present_value);
    let rate_multiplier = 1.0 + rate;
    assert!(rate_multiplier >= 0.0);
    let formula = format!("{:.4} * ({:.6} ^ {})", present_value.into(), rate_multiplier, periods);
    TvmSolution::new(TvmVariable::FutureValue, rate, periods, present_value.into(), future_value, &formula)
}

/// Calculates a future value based on rates that change for each period.
///
/// Related functions:
/// * For simply calculating a single future value using a fixed rate use [`future_value`].
/// * To calculate a future value with a fixed rate and return a struct that shows the formula and
/// optionally produces the the period-by-period values use [`future_value_solution`].
/// * To calculate the future value with varying rates and return a struct that can produce the
/// period-by-period values use [`future_value_schedule_solution`].
///
/// # Arguments
/// * `rates` - A collection of rates, one for each period.
/// * `present_value` - The starting value of the investment.
///
/// # Panics
/// The call will fail if any of the rates is less than -1.0 as this would mean the investment is
/// losing more than its full value.
///
/// # Examples
/// Calculate the value of an investment whose rates vary by year.
/// ```
/// // The rates vary by year: 4% followed by -3.9%, 10.6%, and -5.7%.
/// let rates = [0.04, -0.039, 0.106, -0.057];
///
/// // The initial investment is $75,000.
/// let present_value = 75_000.00;
///
/// let future_value = finance::future_value_schedule(&rates, present_value);
/// dbg!(&future_value);
/// finance::assert_rounded_4(78_178.0458, future_value);
/// ```
/// Error case: One of the rates shows a drop of over 100%. There's no way to work out what this
/// means so the call will panic.
/// ```should_panic
/// let rates = [0.116, -100.134, -0.09, 0.086];
/// let present_value = 4_000.00;
/// let schedule = finance::future_value_schedule(&rates, present_value);
/// ```
pub fn future_value_schedule<T>(rates: &[f64], present_value: T) -> f64
    where T: Into<f64> + Copy
{
    let present_value= present_value.into();
    let periods = rates.len();

    // Check the parameters including all of the provided rates.
    for rate in rates {
        check_future_value_parameters(*rate, periods as u32, present_value);
    }

    let mut future_value = present_value;
    for i in 0..periods {
        future_value *= 1.0 + rates[i];
    }

    future_value
}

/// Calculates a future value based on rates that change for each period, returning a struct with
/// all of the inputs and results.
///
/// Related functions:
/// * For simply calculating a single future value using a fixed rate use [`future_value`].
/// * To calculate a future value with a fixed rate and return a struct that shows the formula and
/// optionally produces the the period-by-period values use [`future_value_solution`].
/// * To calculate the future value if the rates vary by period use [`future_value_schedule`].
///
/// # Arguments
/// * `rates` - A collection of rates, one for each period.
/// * `present_value` - The starting value of the investment.
///
/// # Panics
/// The call will fail if any of the rates is less than -1.0 as this would mean the investment is
/// losing more than its full value.
///
/// # Examples
/// Calculate the value of an investment whose rates vary by year.
/// ```
/// // The rates vary by year: 8.1% followed by 11%, 4%, and -2.3%.
/// let rates = [0.081, 0.11, 0.04, -0.023];
///
/// // The initial investment is $10,000.
/// let present_value = 10_000.00;
///
/// let solution = finance::future_value_schedule_solution(&rates, present_value);
/// dbg!(&solution);
///
/// let future_value = solution.future_value;
/// dbg!(&future_value);
/// finance::assert_rounded_4(future_value, 12_192.0455);
///
/// // Calculate the value for each period.
/// let series = solution.series();
/// dbg!(&series);
/// ```
/// Error case: One of the rates shows a drop of over 100%. There's no way to work out what this
/// means so the call will panic.
/// ```should_panic
/// let rates = [0.116, -100.134, -0.09, 0.086];
/// let present_value = 4_000.00;
/// let schedule = finance::future_value_schedule(&rates, present_value);
/// ```
pub fn future_value_schedule_solution<T>(rates: &[f64], present_value: T) -> TvmSchedule
    where T: Into<f64> + Copy
{
    let future_value = future_value_schedule(rates, present_value);
    TvmSchedule::new(TvmVariable::FutureValue, rates, present_value.into(), future_value)
}

fn check_future_value_parameters(rate: f64, _periods: u32, present_value: f64) {
    assert!(rate.is_finite(), "The rate must be finite (not NaN or infinity)");
    assert!(rate >= -1.0, "The rate must be greater than or equal to -1.0 because a rate lower than -100% would mean the investment loses more than its full value in a period.");
    if rate.abs() > 1. {
        warn!("You provided a periodic rate ({}) greater than 1. Are you sure you expect a {}% return?", rate, rate * 100.0);
    }
    assert!(present_value.is_finite(), "The present value must be finite (not NaN or infinity)");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_future_value_nominal() {
        assert_rounded_4(295_489.9418, future_value(0.034, 5, 250_000.00));
        assert_rounded_4(20_629.3662, future_value(0.08, 6, 13_000.0));
        assert_rounded_4(5_000.0000, future_value(-0.09, 6, 8_804.84368898));
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
        assert_rounded_4(expected_value, actual_value);
    }

    #[test]
    fn test_future_value_solution_2() {
        let rate_of_return = 0.08;
        let periods = 6;
        let present_value_1 = 13_000.0;
        let expected_value = 20_629.37;
        let actual_value = future_value_solution(rate_of_return, periods, present_value_1).future_value;
        assert_rounded_2(expected_value, actual_value);
    }

    #[test]
    fn test_future_value_solution_3() {
        // test negative rate
        let rate_of_return = -0.09;
        let periods = 6;
        let present_value = 8_804.84368898;
        let expected_value = 5_000.00;
        let actual_value = future_value_solution(rate_of_return, periods, present_value).future_value;
        assert_rounded_4(expected_value, actual_value);
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

