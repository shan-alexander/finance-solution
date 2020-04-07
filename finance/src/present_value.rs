//! Present value calculations. Given a final amount, a number of periods such as years, and fixed
//! or varying interest periodic_rates, what is the current value?
//!
//! If you need to calculate the future value given a present value, a number of periods, and one
//! or more rates use [`future_value`] or related functions.
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
use crate::{future_value::future_value, rate::rate, periods::periods};

/// Returns the current value of a future amount using a fixed periodic_rate.
///
/// Related functions:
/// * To calculate a present value with a fixed rate and return a struct that shows the formula and
/// optionally produces the the period-by-period values use [`present_value_solution`].
/// * To calculate the present value if the rates vary by period use [`present_value_schedule`].
/// * To calculate the present value with varying rates and return a struct that can produce the
/// period-by-period values use [`present_value_schedule_solution`].
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
/// assert_eq!(43_848.6409, finance::round_to_fraction_of_cent(present_value));
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

/// Calculates the current value of a future amount using a fixed periodic_rate and returns a struct
/// with the inputs and the calculated value. This is used for keeping track of a collection of
/// financial scenarios so that they can be examined later.
///
/// Related functions:
/// * For simply calculating a single present value using a fixed rate use [`present_value`].
/// * To calculate the present value if the rates vary by period use [`present_value_schedule`].
/// * To calculate the present value with varying rates and return a struct that can produce the
/// period-by-period values use [`present_value_schedule_solution`].
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
/// Calculate a present value and examine the period-by-period values.
/// ```
/// // The rate is 8.45% per year.
/// let periodic_rate = 0.0845;
///
/// // The investment will grow for six years.
/// let periods = 6;
///
/// // The final value is $50,000.
/// let future_value = 50_000;
///
/// // Calculate the present value and create a struct with the input values and
/// // the formula used.
/// let solution= finance::present_value_solution(periodic_rate, periods, future_value);
/// dbg!(&solution);
///
/// let present_value = solution.present_value;
/// finance::assert_rounded_4(present_value, 30_732.1303);
///
/// // Examine the formula.
/// let formula = solution.formula.clone();
/// dbg!(&formula);
/// assert_eq!(formula, "50000.0000 / (1.084500 ^ 6)");
///
/// // Calculate the amount at the end of each period.
/// let series = solution.series();
/// dbg!(&series);
/// ```
/// Build a collection of present value calculations where the future value and periodic rate are
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
/// // Calculate the future value for this number of months and add the details to the
/// // collection.
/// scenarios.push(finance::present_value_solution(periodic_rate, periods, future_value));
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
/// assert_eq!("100000.0000 / (1.009000 ^ 25)", scenarios[0].formula);
/// ```
/// Error case: The investment loses 111% per year. There's no way to work out what this means so
/// the call to present_value() will panic.
/// ```should_panic
/// let periodic_rate = -1.11;
/// let periods = 12;
/// let present_value = 100_000.85;
/// let present_value = finance::present_value_solution(periodic_rate, periods, present_value);
/// ```
pub fn present_value_solution<T>(periodic_rate: f64, periods: u32, future_value: T) -> TvmSolution
    where T: Into<f64> + Copy
{
    let present_value = present_value(periodic_rate, periods, future_value);
    let rate_multiplier = 1.0 + periodic_rate;
    assert!(rate_multiplier >= 0.0);
    let formula = format!("{:.4} / ({:.6} ^ {})", future_value.into(), rate_multiplier, periods);
    TvmSolution::new(TvmVariable::PresentValue, periodic_rate, periods, present_value, future_value.into(), &formula)
}

/// Calculates a present value based on rates that change for each period.
///
/// Related functions:
/// * For simply calculating a single present value using a fixed rate use [`present_value`].
/// * To calculate a present value with a fixed rate and return a struct that shows the formula and
/// optionally produces the the period-by-period values use [`present_value_solution`].
/// * To calculate the present value with varying rates and return a struct that can produce the
/// period-by-period values use [`present_value_schedule_solution`].
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
/// Calculate the present value of an investment whose rates vary by year.
/// ```
/// // The annual rate varies from -3.4% to 12.9%.
/// let rates = [0.04, -0.034, 0.0122, 0.129, 8.5];
///
/// // The value of the investment after applying all of these periodic rates
/// // will be $30_000.
/// let future_value = 30_000.00;
///
/// // Calculate the present value.
/// let present_value = finance::present_value_schedule(&rates, future_value);
/// dbg!(&present_value);
/// ```
pub fn present_value_schedule<T>(periodic_rates: &[f64], future_value: T) -> f64
    where T: Into<f64> + Copy
{
    let periods = periodic_rates.len();
    let future_value = future_value.into();

    // Check the parameters including all of the provided rates.
    for rate in periodic_rates {
        check_present_value_parameters(*rate, periods as u32, future_value);
    }

    let mut present_value = future_value;
    for i in (0..periods).rev() {
        present_value /= 1.0 + periodic_rates[i];
    }

    present_value
}

/// Calculates a present value based on rates that change for each period and returns a struct
/// with the inputs and the calculated value.
///
/// Related functions:
/// * For simply calculating a single present value using a fixed rate use [`present_value`].
/// * To calculate a present value with a fixed rate and return a struct that shows the formula and
/// optionally produces the the period-by-period values use [`present_value_solution`].
/// * To calculate the present value if the rates vary by period use [`present_value_schedule`].
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
/// // The quarterly rate varies from -0.5% to 4%.
/// let rates = [0.04, 0.008, 0.0122, -0.005];
///
/// // The value of the investment after applying all of these periodic rates
/// // will be $25_000.
/// let future_value = 25_000.00;
///
/// // Calculate the present value and keep track of the inputs and the formula
/// // in a struct.
/// let solution = finance::present_value_schedule_solution(&rates, future_value);
/// dbg!(&solution);
///
/// let present_value = solution.present_value;
/// finance::assert_rounded_4(present_value, 23_678.6383);
///
/// // Calculate the value for each period.
/// let series = solution.series();
/// dbg!(&series);
/// ```
pub fn present_value_schedule_solution<T>(periodic_rates: &[f64], future_value: T) -> TvmSchedule
    where T: Into<f64> + Copy
{
    let present_value = present_value_schedule(periodic_rates, future_value);
    TvmSchedule::new(TvmVariable::PresentValue, periodic_rates, present_value, future_value.into())
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

        let present_value = present_value_schedule(&rates, future_value);
        assert_eq!(94843.2841, round_to_fraction_of_cent(present_value));

        let solution = present_value_schedule_solution(&rates, future_value);
        assert_eq!(100000.25, round_to_fraction_of_cent(solution.future_value));
        assert_eq!(94843.2841, round_to_fraction_of_cent(solution.present_value));

        let series = solution.series();
        assert_eq!(6, series.len());

        let period = &series[0];
        assert_eq!(0, period.period);
        assert_eq!(0.0, period.rate);
        assert_rounded_4(present_value,period.value);

        let period = &series[1];
        assert_eq!(1, period.period);
        assert_eq!(0.04, period.rate);
        assert_rounded_4(98_637.0154,period.value);

        let period = &series[2];
        assert_eq!(2, period.period);
        assert_eq!(0.07, period.rate);
        assert_rounded_4(105_541.6065,period.value);

        let period = &series[3];
        assert_eq!(3, period.period);
        assert_eq!(-0.12, period.rate);
        assert_rounded_4(92_876.6137,period.value);

        let period = &series[4];
        assert_eq!(4, period.period);
        assert_eq!(-0.03, period.rate);
        assert_rounded_4(90_090.3153, period.value);

        let period = &series[5];
        assert_eq!(5, period.period);
        assert_eq!(0.11, period.rate);
        assert_rounded_4(100_000.2500, period.value);
    }

}
