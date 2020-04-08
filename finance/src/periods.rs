//! Number of periods calculations. Given a periodic rate and an initial and final amount, find the
//! number of periods needed.
//!
//! If you need to calculate a fixed rate given a present value, future value, and number of periods
//! use [`rate`] or related functions.
//!
//! If you need to calculate the future value given a starting value, a number of periods, and one
//! or more rates use [`future_value`] or related functions.
//!
//! If you need to calculate the present value given a future value, a number of periods, and one
//! or more rates use [`present_value`] or related functions.

// use log::warn;

use crate::tvm_simple::*;

// Needed for the Rustdoc comments.
#[allow(unused_imports)]
use crate::{future_value::future_value, present_value::present_value, rate::rate};

/// Returns the periodic rate of an investment given the number of periods along with the present
/// and future values.
///
/// Note that the returned number of periods will be a floating point number representing fractional
/// periods.
///
/// Related functions:
/// * To calculate a periodic rate and return a struct that shows the formula and optionally
/// produces the the period-by-period values use [`periods_solution`].
///
/// Functions to solve for other values:
/// * To calculate the future value given a present value, a number of periods, and one or more
/// rates use [`future_value`] or related functions.
/// * To calculate the present value given a future value, a number of periods, and one or more
/// rates use [`present_value`] or related functions.
/// * To calculate a fixed rate given a present value, future value, and number of periods use
/// [`rate`] or related functions.
///
/// The formula is:
///
/// periods = log<sub>(1 + rate)</sub> (future_value / present_value)
///
/// # Arguments
/// * `periodic_rate` - The rate at which the investment grows or shrinks per period, expressed as a
/// floating point number. For instance 0.05 would mean 5% growth. Often appears as `r` or `i` in
/// formulas.
/// * `present_value` - The starting value of the investment. May appear as `pv` in formulas, or `C`
/// for cash flow or `P` for principal.
/// * `future_value` - The final value of the investment.
///
/// # Panics
/// The call will fail if either of the periodic_rate, the present value, or the future value is
/// infinite or not a number (NaN).
///
/// The call will also fail in any of the follwing cases because there is no number of periods that
/// would make the calculation work:
/// * The future value is greater than the present value and the periodic rate is zero or negative.
/// * The future value is less than the present value and the periodic rate is zero or positive.
/// * The present value is zero and the future value is nonzero.
/// * The present value is nonzero and the future value is zero, unless the rate is exactly -1.0%.
/// * The present value is negative and the future value is positive or vice versa.
///
/// # Examples
/// ```
/// // The interest rate is 8% per year.
/// let periodic_rate = 0.08;
///
/// // The starting value is $5,000.00.
/// let present_value = 5_000.00;
///
/// // The ending value is $7,000.00.
/// let future_value = 7_000.00;
///
/// // Calculate the number of years required.
/// let fractional_periods = finance::periods(periodic_rate, present_value, future_value);
/// dbg!(&fractional_periods);
/// finance::assert_rounded_2(4.37, fractional_periods);
///
/// // Round up to get a whole number of years.
/// let periods = fractional_periods.ceil() as u32;
/// dbg!(&periods);
/// assert_eq!(5, periods);
/// ```
pub fn periods<P, F>(periodic_rate: f64, present_value: P, future_value: F) -> f64
    where
        P: Into<f64> + Copy,
        F: Into<f64> + Copy
{
    let present_value = present_value.into();
    let future_value = future_value.into();
    if present_value == future_value {
        // This is a special case that doesn't require us to check the parameters and which covers
        // the case where both are zero.
        return 0.0;
    }
    if future_value == 0.0 && periodic_rate == -1.0 {
        // This is a special case that we can't run through the log function. Since the rate is
        // -100%, given any present value the future value will be zero and it will take only one
        // period to get there.
        // We already know that the present value is nonzero because that case would have been
        // caught above.
        assert!(present_value != 0.0);
        return 1.0;
    }

    check_period_parameters(periodic_rate, present_value, future_value);

    let fractional_periods = (future_value / present_value).log(1.0 + periodic_rate);
    assert!(fractional_periods >= 0.0);
    fractional_periods
}

/// Calculates the periodic rate of an investment given the number of periods along with the present
/// and future values and builds a struct with the input values, an explanation of the formula, and
/// the option to calculate the period-by-period values.
///
/// Note that the calculated number of periods in the [`TvmSolution.fractional_periods`] field will
/// be a floating point number. To get the periods as a whole number (rounded up) use
/// [`TvmSolution.periods`]
///
/// Related functions:
/// * To calculate the periods as a simple number use [`period`].
///
/// Functions to solve for other values:
/// * To calculate the future value given a present value, a number of periods, and one or more
/// rates use [`future_value`] or related functions.
/// * To calculate the present value given a future value, a number of periods, and one or more
/// rates use [`present_value`] or related functions.
/// * To calculate a fixed rate given a present value, future value, and number of periods use
/// [`rate`] or related functions.
///
/// The formula is:
///
/// periods = log<sub>(1 + rate)</sub> (future_value / present_value)
///
/// # Arguments
/// * `periodic_rate` - The rate at which the investment grows or shrinks per period, expressed as a
/// floating point number. For instance 0.05 would mean 5% growth. Often appears as `r` or `i` in
/// formulas.
/// * `present_value` - The starting value of the investment. May appear as `pv` in formulas, or `C`
/// for cash flow or `P` for principal.
/// * `future_value` - The final value of the investment.
///
/// # Panics
/// The call will fail if either of the periodic_rate, the present value, or the future value is
/// infinite or not a number (NaN).
///
/// The call will also fail in any of the follwing cases because there is no number of periods that
/// would make the calculation work:
/// * The future value is greater than the present value and the periodic rate is zero or negative.
/// * The future value is less than the present value and the periodic rate is zero or positive.
/// * The present value is zero and the future value is nonzero.
/// * The present value is nonzero and the future value is zero, unless the rate is exactly -1.0%.
/// * The present value is negative and the future value is positive or vice versa.
///
/// # Examples
/// ```
/// // The interest rate is 3.5% per quarter.
/// let periodic_rate = 0.035;
///
/// // The starting value is $100,000.00.
/// let present_value = 100_000.00;
///
/// // The ending value is $200,000.00.
/// let future_value = 200_000.00;
///
/// // Calculate the number of quarters required and build a struct with the
/// // input values, an explanation of the formula, and an option to calculate
/// // the period-by-period values.
/// let solution = finance::periods_solution(periodic_rate, present_value, future_value);
///
/// let fractional_periods = solution.fractional_periods;
/// dbg!(&fractional_periods);
/// finance::assert_rounded_2(20.15, fractional_periods);
///
/// // Get the whole number of periods.
/// let periods = solution.periods;
/// dbg!(&periods);
/// assert_eq!(21, periods);
///
/// // Examine the formula.
/// let formula = solution.formula.clone();
/// dbg!(&formula);
/// assert_eq!("log(200000.0000 / 100000.0000) base (1 + 0.035000))", formula);
///
/// let series = solution.series();
/// dbg!(&series);
///
/// // The last period is calculated as if it were a full period so the value is higher than the
/// // original future value.
/// let last_entry = series.last().unwrap();
/// dbg!(&last_entry);
/// finance::assert_rounded_4(205_943.1474, last_entry.value);
///
/// // Create a reduced series with the value at the end of each year.
/// let filtered_series = series
///     .iter()
///     .filter(|x| x.period % 4 == 0 && x.period != 0)
///     .collect::<Vec<_>>();
/// dbg!(&filtered_series);
/// assert_eq!(5, filtered_series.len());
/// ```
/// Negative interest rate.
/// ```
/// // The interest rate is -6% per year and the value falls from $15,000.00 to
/// // $12,000.00.
/// let solution = finance::periods_solution(-0.06, 15_000.00, 12_000.00);
/// dbg!(&solution);
/// finance::assert_rounded_2(3.61, solution.fractional_periods);
/// assert_eq!(4, solution.periods);
///
/// // View the period-by-period values.
/// dbg!(solution.series());
/// ```
pub fn periods_solution<P, F>(periodic_rate: f64, present_value: P, future_value: F) -> TvmSolution
    where
        P: Into<f64> + Copy,
        F: Into<f64> + Copy
{
    let fractional_periods = periods(periodic_rate, present_value, future_value);
    assert!(fractional_periods > 0.0);

    let periods = fractional_periods.ceil() as u32;

    let present_value = present_value.into();
    let future_value = future_value.into();
    let formula = format!("log({:.4} / {:.4}) base (1 + {:.6}))", future_value, present_value, periodic_rate);
    TvmSolution::new_fractional_periods(TvmVariable::Periods,periodic_rate, periods, fractional_periods, present_value, future_value, &formula)
}

fn check_period_parameters(periodic_rate: f64, present_value: f64, future_value: f64) {
    assert!(periodic_rate.is_finite(), "The periodic_rate must be finite (not NaN or infinity)");
    assert!(present_value.is_finite(), "The present value must be finite (not NaN or infinity)");
    assert!(future_value.is_finite(), "The future value must be finite (not NaN or infinity)");
    assert!(!(present_value < future_value && periodic_rate <= 0.0), "The future value is greater than the present value and the periodic rate is zero or negative so there's no way to solve for the number of periods.");
    assert!(!(present_value > future_value && periodic_rate >= 0.0), "The future value is less than the present value and the periodic rate is zero or positive so there's no way to solve for the number of periods.");
    assert!(!(present_value == 0.0 && future_value != 0.0), "The present value is zero and the future value is nonzero so there's no way to solve for the number of periods.");
    assert!(!(present_value != 0.0 && future_value == 0.0 && periodic_rate != -1.0), "The present value is nonzero, the future value is zero, and the rate is not -100% so there's no way to solve for the number of periods.");
    assert!(!(present_value < 0.0 && future_value > 0.0), "The present value is negative and the future value is positive so there's no way to solve for the number of periods.");
    assert!(!(present_value > 0.0 && future_value < 0.0), "The present value is positive and the future value is negative so there's no way to solve for the number of periods.");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_periods_nominal() {
        // The test values come from the Excel nper function.
        assert_rounded_2(4.37, periods(0.08, 5_000, 7_000));
        assert_rounded_2(4.04, periods(-0.08, 7000, 5_000));
        assert_rounded_2(346.92, periods(0.002, 10_000, 20_000));
        assert_rounded_2(346.23, periods(-0.002, 20000, 10_000));
    }

    #[test]
    fn test_periods_edge() {
        // Present and future values are the same so no periods are needed.
        assert_rounded_2(0.0, periods(0.04, 10_000.0, 10_000.0));

        // The present value is negative and the future value is zero, which works only if the rate
        // is exactly -1.0%.
        assert_rounded_6(1.0, periods(-1.0, -10_000.0, 0.0));

        // The present value is positive and the future value is zero, which works only if the rate
        // is exactly -1.0%.
        assert_rounded_6(1.0, periods(-1.0, 10_000.0, 0.0));
    }

    #[should_panic]
    #[test]
    fn test_periods_err_rate_nan() {
        periods(std::f64::NAN, 1_000.0, 2_000.0);
    }

    #[should_panic]
    #[test]
    fn test_periods_err_rate_inf() {
        periods(std::f64::NEG_INFINITY, 1_000.0, 2_000.0);
    }

    #[should_panic]
    #[test]
    fn test_periods_err_present_value_nan() {
        periods(0.04, std::f64::NAN, 1_000.0);
    }

    #[should_panic]
    #[test]
    fn test_periods_err_present_value_inf() {
        periods(0.04, std::f64::INFINITY, 1_000.0);
    }

    #[should_panic]
    #[test]
    fn test_periods_err_future_value_nan() {
        periods(0.04, 1_000.0, std::f64::NAN);
    }

    #[should_panic]
    #[test]
    fn test_periods_err_future_value_inf() {
        periods(0.04, 1_000.0, std::f64::NEG_INFINITY);
    }

    #[should_panic]
    #[test]
    fn test_periods_err_future_greater_bad_rate_1() {
        // The future value is greater than the present value and the periodic rate is zero.
        periods(0.0, 1_000.0, 2_000.0);
    }

    #[should_panic]
    #[test]
    fn test_periods_err_future_greater_bad_rate_2() {
        // The future value is greater than the present value and the periodic rate is negative.
        periods(-0.04, 1_000.0, 2_000.0);
    }

    #[should_panic]
    #[test]
    fn test_periods_err_future_less_bad_rate_1() {
        // The future value is less than the present value and the periodic rate is zero.
        periods(0.0, 2_000.0, 1_000.0);
    }

    #[should_panic]
    #[test]
    fn test_periods_err_future_less_bad_rate_2() {
        // The future value is less than the present value and the periodic rate is positive.
        periods(0.04, 2_000.0, 1_000.0);
    }

    #[should_panic]
    #[test]
    fn test_periods_err_present_zero_future_negative() {
        // The present value is zero and the future value is negative.
        periods(0.04, 0.0, -1_000.0);
    }

    #[should_panic]
    #[test]
    fn test_periods_err_present_zero_future_positive() {
        // The present value is zero and the future value is positive.
        periods(0.04, 0.0, 1_000.0);
    }

    #[should_panic]
    #[test]
    fn test_periods_err_present_negative_future_zero() {
        // The present value is negative and the future value is zero.
        periods(0.04, -1_000.0, 0.0);
    }

    #[should_panic]
    #[test]
    fn test_periods_err_present_positive_future_zero() {
        // The present value is positive and the future value is zero. This will fail unless the
        // rate is exactly -1.0%.
        periods(-0.04, 1_000.0, 0.0);
    }

    #[should_panic]
    #[test]
    fn test_periods_err_present_negative_future_positive() {
        // The present value is negative and the future value is positive.
        periods(0.04, -1_000.0, 1_000.0);
    }

    #[should_panic]
    #[test]
    fn test_periods_err_present_positive_future_negative() {
        // The present value is positive and the future value is negative.
        periods(0.04, 1_000.0, -1_000.0);
    }

}



