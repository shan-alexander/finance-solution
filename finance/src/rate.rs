//! Periodic rate calculations. Given an initial investment amount, a final amount, and a a number
//! of periods such as years, what does the rate per period need to be?
//!
//! If you need to calculate the future value given a starting value, a number of periods, and one
//! or more rates use [`future_value`] or related functions.
//!
//! If you need to calculate the present value given a future value, a number of periods, and one
//! or more rates use [`present_value`] or related functions.
//!
//! If you need to calculate the number of periods given a fixed rate and a present and future value
//! use [`periods`] or related functions.

// use log::warn;

use crate::tvm_simple::*;

// Needed for the Rustdoc comments.
#[allow(unused_imports)]
use crate::{future_value::future_value, present_value::present_value, periods::periods};

/// Returns the periodic rate of an investment given the number of periods along with the present
/// and future values.
///
/// Related functions:
/// * To calculate a periodic rate and return a struct that shows the formula and optionally
/// produces the the period-by-period values use [`rate_solution`].
///
/// Functions to solve for other values:
/// * To calculate the future value given a present value, a number of periods, and one or more
/// rates use [`future_value`] or related functions.
/// * To calculate the present value given a future value, a number of periods, and one or more
/// rates use [`present_value`] or related functions.
/// * To calculate the number of periods given a fixed rate and a present and future value use
/// [`periods`] or related functions.
///
/// The formula is:
///
/// rate = ((future_value / present_value) ^ (1 / periods)) - 1
///
/// # Arguments
/// * `periods` - The number of periods such as quarters or years. Often appears as `n` or `t`.
/// * `present_value` - The starting value of the investment. May appear as `pv` in formulas, or `C`
/// for cash flow or `P` for principal.
/// * `future_value` - The final value of the investment.
///
/// If present_value and future_value are both zero then any rate will work so the function returns
/// zero.
///
/// # Panics
/// The call will fail if the present value is zero and the future value is nonzero or vice versa.
/// It will also fail if the number of periods is zero and the present value is not equal to the
/// future value. In both cases this is because there's no periodic rate that could make that work.
///
/// # Examples
/// ```
/// // The interest will compound for 365 days.
/// let periods = 365;
///
/// // The starting value is $10,000.
/// let present_value = 10_000.00;
///
/// // The ending value is $11,000.
/// let future_value = 11_000.00;
///
/// // Calculate the periodic rate needed.
/// let rate = finance::rate(periods, present_value, future_value);
/// dbg!(&rate);
/// // The rate is 0.0261% per day.
/// finance::assert_rounded_6(0.000261, rate);
/// ```
pub fn rate<P, F>(periods: u32, present_value: P, future_value: F) -> f64
    where
        P: Into<f64> + Copy,
        F: Into<f64> + Copy
{
    let present_value = present_value.into();
    let future_value = future_value.into();
    if present_value == 0.0 && future_value == 0.0 {
        // This is a special case where any rate will work.
        return 0.0;
    }
    check_rate_parameters(periods, present_value, future_value);

    let rate = (future_value / present_value).powf(1.0 / periods as f64) - 1.0;
    assert!(rate.is_finite());
    rate
}

/// Returns the periodic rate of an investment given the number of periods along with the present
/// and future values.
///
/// Related functions:
/// * To calculate a periodic rate as a simple number use [`rate`].
///
/// Functions to solve for other values:
/// * To calculate the future value given a present value, a number of periods, and one or more
/// rates use [`future_value`] or related functions.
/// * To calculate the present value given a future value, a number of periods, and one or more
/// rates use [`present_value`] or related functions.
/// * To calculate the number of periods given a fixed rate and a present and future value use
/// [`periods`] or related functions.
///
/// The formula is:
///
/// rate = ((future_value / present_value) ^ (1 / periods)) - 1
///
/// # Arguments
/// * `periods` - The number of periods such as quarters or years. Often appears as `n` or `t`.
/// * `present_value` - The starting value of the investment. May appear as `pv` in formulas, or `C`
/// for cash flow or `P` for principal.
/// * `future_value` - The final value of the investment.
///
/// If present_value and future_value are both zero then any rate will work so the function returns
/// zero.
///
/// # Panics
/// The call will fail if the present value is zero and the future value is nonzero or vice versa.
/// It will also fail if the number of periods is zero and the present value is not equal to the
/// future value. In both cases this is because there's no periodic rate that could make that work.
///
/// # Examples
/// Calculate a periodic rate and examine the period-by-period values.
/// ```
/// // The interest will compound for ten years.
/// let periods = 10;
///
/// // The starting value is $10,000.
/// let present_value = 10_000.00;
///
/// // The ending value is $15,000.
/// let future_value = 15_000.00;
///
/// // Calculate the periodic rate and create a struct with a record of the
/// // inputs, a description of the formula, and an option to calculate the
/// // period-by-period values.
/// let solution = finance::rate_solution(periods, present_value, future_value);
/// dbg!(&solution);
///
/// let rate = solution.rate;
/// dbg!(&rate);
/// // The rate is 4.138% per year.
/// finance::assert_rounded_6(0.041380, rate);
///
/// // Examine the formula.
/// let formula = solution.formula.clone();
/// dbg!(&formula);
/// assert_eq!("((15000.0000 / 10000.0000) ^ (1 / 10)) - 1", &formula);
///
/// // Calculate the period-by-period values.
/// let series = solution.series();
/// dbg!(&series);
/// ```
pub fn rate_solution<P, F>(periods: u32, present_value: P, future_value: F) -> TvmSolution
    where
        P: Into<f64> + Copy,
        F: Into<f64> + Copy
{
    let present_value = present_value.into();
    let future_value = future_value.into();
    if present_value == 0.0 && future_value == 0.0 {
        // This is a special case where any rate will work.
        return TvmSolution::new(TvmVariable::Rate, 0.0, periods, present_value, future_value, "{special case}");
    }

    let rate = rate(periods, present_value, future_value);
    let formula = format!("(({:.4} / {:.4}) ^ (1 / {})) - 1", future_value, present_value, periods);
    TvmSolution::new(TvmVariable::Rate,rate, periods, present_value.into(), future_value, &formula)
}

fn check_rate_parameters(periods: u32, present_value: f64, future_value: f64) {
    assert!(present_value.is_finite(), "The present value must be finite (not NaN or infinity)");
    assert!(future_value.is_finite(), "The future value must be finite (not NaN or infinity)");
    assert!(!(present_value == 0.0 && future_value != 0.0), "The present value is zero and the future value is nonzero so there's no way to solve for rate.");
    assert!(!(present_value != 0.0 && future_value == 0.0), "The present value is nonzero and the future value is zero so there's no way to solve for rate.");
    assert!(!(periods == 0 && present_value != future_value), "The number of periods is zero and the future value is different from the present value so there's no way to solve for rate.");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_rate_nominal() {
        // The test values come from the Excel rate function.
        assert_rounded_6(0.028436, rate(12, 5_000, 7_000));
        assert_rounded_6(-0.027650, rate(12, 7_000, 5_000));
        assert_rounded_6(0.100000, rate(1, 10_000, 11_000));
        assert_rounded_6(-0.090909, rate(1, 11_000, 10_000));
        assert_rounded_6(0.001127, rate(360, 8_000, 12_000));
        assert_rounded_6(-0.001126, rate(360, 12_000, 8_000));
    }

    #[test]
    fn test_rate_edge() {
        // Zero periods, values the same.
        assert_rounded_6(0.0, rate(0, 10_000.0, 10_000.0));

        // Nonzero periods, values the same.
        assert_rounded_6(0.0, rate(12, 10_000.0, 10_000.0));
    }

    #[should_panic]
    #[test]
    fn test_rate_err_present_value_nan() {
        rate(12, std::f64::NAN, 1_000.0);
    }

    #[should_panic]
    #[test]
    fn test_rate_err_present_value_inf() {
        rate(12, std::f64::INFINITY, 1_000.0);
    }

    #[should_panic]
    #[test]
    fn test_rate_err_future_value_nan() {
        rate(12, 1_000.0, std::f64::NAN);
    }

    #[should_panic]
    #[test]
    fn test_rate_err_future_value_inf() {
        rate(12, 1_000.0, std::f64::NEG_INFINITY);
    }

    #[should_panic]
    #[test]
    fn test_rate_err_periods_zero_values_diff() {
        rate(0, 1_000.0, 2_000.0);
    }

}

