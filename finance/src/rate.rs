//! Periodic rate calculations. Given an initial investment amount, a final amount, and a a number
//! of periods such as periods, what does the rate per period need to be?
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
use std::ops::Deref;

/// A record of a call to [rate_solution](./fn.rate_solution.html). The structure shows details such as the formula and
/// can calculate the period-by-period details.
#[derive(Clone, Debug)]
pub struct RateSolution(TvmSolution);

/// The period-by-period details of a Rate calculation. This is the result of a call to
/// [RateSolution::series](./struct.RateSolution#method.series.html).
#[derive(Clone, Debug)]
pub struct RateSeries(TvmSeries);

impl RateSolution {
    fn new (solution: TvmSolution) -> Self {
        Self {
            0: solution,
        }
    }

    /// Calculates the period-by-period details of a rate calculation.
    ///
    /// # Examples
    /// A rate calculation with simple compounding using [rate_solution](./fn.rate_solution.html).
    /// ```
    /// // The initial investment is $10,000.12.
    /// let present_value = 10_000.12;
    ///
    /// // The interest rate is 1.5% per month.
    /// let interest_rate = 0.015;
    ///
    /// // The investment will grow for 24 months.
    /// let periods = 24;
    ///
    /// // Calculate the overall solution including the future value.
    /// let solution = finance::future_value_solution(interest_rate, periods, present_value);
    /// dbg!(&solution);
    ///
    /// // Calculate the value at the end of each period.
    /// let series = solution.series();
    /// dbg!(&series);
    ///
    /// // Confirm that we have one entry for the initial value and one entry for each period.
    /// assert_eq!(25, series.len());
    ///
    /// // Create a reduced vector with every fourth period.
    /// let filtered_series = series
    ///     .iter()
    ///     .filter(|x| x.period() % 4 == 0)
    ///     .collect::<Vec<_>>();
    /// dbg!(&filtered_series);
    /// assert_eq!(7, filtered_series.len());
    /// ```
    /// Calculate the future value of an investment whose rates vary by year, then find the point
    /// where the value passes a certain threshold. Uses [`future_value_schedule`].
    /// ```
    /// // The rates vary by year: 11.6% followed by 13.4%, 9%, and 8.6%.
    /// let rates = [0.116, 0.134, -0.09, 0.086];
    ///
    /// // The initial investment is $50,000.
    /// let present_value = 50_000.00;
    ///
    /// // Calculate the future value and create a struct with all of the variables
    /// // and the formula used.
    /// let solution = finance::future_value_schedule_solution(&rates, present_value);
    /// dbg!(&solution);
    /// finance::assert_rounded_4(62534.3257, solution.future_value());
    ///
    /// // Calculate the value at the end of each period.
    /// let series = solution.series();
    /// dbg!(&series);
    ///
    /// // Confirm that there are four periods corresponding to the four interest
    /// // rates as well as one more for period 0 representing the initial value.
    /// assert_eq!(5, series.len());
    ///
    /// // Confirm that the value of the fourth period is the same as the overall
    /// // future value.
    /// finance::assert_rounded_4(solution.future_value(), series.last().unwrap().value());
    ///
    /// // Find the first period where the value of the investment was at least
    /// // $60,000.
    /// let period = series.iter().find(|x| x.value() >= 60_000.00);
    /// dbg!(&period);
    /// assert_eq!(2, period.unwrap().period());
    /// ```
    /// Calculate a present value with a fixed rate then examine the period-by-period values. Uses
    /// [`present_value_solution`].
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
    /// // Calculate the present value.
    /// let solution = finance::present_value_solution(interest_rate, periods, future_value);
    /// dbg!(&solution);
    ///
    /// // Calculate the value at the end of each period.
    /// let series = solution.series();
    /// dbg!(&series);
    ///
    /// // Confirm that we have one entry for the present value, that is the
    /// // initial value before any interest is applied, and one entry for each
    /// // period.
    /// assert_eq!(11, series.len());
    ///
    /// // Create a reduced vector with every other period not including period 0,
    /// // the initial state.
    /// let filtered_series = series
    ///     .iter()
    ///     .filter(|x| x.period() % 2 == 0 && x.period() != 0)
    ///     .collect::<Vec<_>>();
    /// dbg!(&filtered_series);
    /// assert_eq!(5, filtered_series.len());
    /// ```
    /// Calculate a present value with varying rates then examine the period-by-period values. Uses
    /// [`present_value_schedule`].
    /// ```
    /// // The annual rate varies from -12% to 11%.
    /// let rates = [0.04, 0.07, -0.12, -0.03, 0.11];
    ///
    /// // The value of the investment after applying all of these periodic rates
    /// // will be $100_000.25.
    /// let future_value = 100_000.25;
    ///
    /// // Calculate the present value and keep track of the inputs and the formula
    /// // in a struct.
    /// let solution = finance::present_value_schedule_solution(&rates, future_value);
    /// dbg!(&solution);
    ///
    /// // Calculate the value at the end of each period.
    /// let series = solution.series();
    /// dbg!(&series);
    /// // There is one entry for each period and one entry for period 0 containing
    /// // the present value.
    /// assert_eq!(6, series.len());
    ///
    /// // Create a filtered list of periods, only those with a negative rate.
    /// let filtered_series = series
    ///     .iter()
    ///     .filter(|x| x.rate() < 0.0)
    ///     .collect::<Vec<_>>();
    /// dbg!(&filtered_series);
    /// assert_eq!(2, filtered_series.len());
    /// ```
    pub fn series(&self) -> RateSeries {
        // For a rate, periods, or future value calculation the the period-by-period values are
        // calculated the same way
        RateSeries::new(self.0.series())
    }

    pub fn print_series_table(&self, locale: &num_format::Locale, precision: usize) {
        self.series().print_table(locale, precision);
    }

    pub fn tvm_solution(&self) -> TvmSolution {
        self.clone().into()
    }

    pub fn tvm_solution_and_series(&self) -> (TvmSolution, TvmSeries) {
        let series = self.series();
        (self.clone().into(), series.into())
    }
}

impl Deref for RateSolution {
    type Target = TvmSolution;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<TvmSolution> for RateSolution {
    fn into(self) -> TvmSolution {
        self.0
    }
}

impl RateSeries {
    fn new (series: TvmSeries) -> Self {
        Self {
            0: series,
        }
    }
}

impl Deref for RateSeries {
    type Target = TvmSeries;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<TvmSeries> for RateSeries {
    fn into(self) -> TvmSeries {
        self.0
    }
}

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
/// > rate = (future_value / present_value)<sup>1 / periods</sup> - 1
///
/// or with the more commonly used variables:
/// > r = (fv / pv)<sup>1 / n</sup> - 1
///
/// # Arguments
/// * `periods` - The number of periods such as quarters or periods. Often appears as `n` or `t`.
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
    rate_internal(periods, present_value.into(), future_value.into(), false)
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
/// > rate = (future_value / present_value)<sup>1 / periods</sup> - 1
///
/// or with the more commonly used variables:
/// > r = (fv / pv)<sup>1 / n</sup> - 1
///
/// # Arguments
/// * `periods` - The number of periods such as quarters or periods. Often appears as `n` or `t`.
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
/// // The interest will compound for ten periods.
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
/// let rate = solution.rate();
/// dbg!(&rate);
/// // The rate is 4.138% per period.
/// finance::assert_rounded_6(0.041380, rate);
///
/// // Examine the formulas.
/// let formula = solution.formula();
/// dbg!(&formula);
/// assert_eq!("0.041380 = ((15000.0000 / 10000.0000) ^ (1 / 10)) - 1", formula);
/// let symbolic_formula = solution.symbolic_formula();
/// dbg!(&symbolic_formula);
/// assert_eq!("r = ((fv / pv) ^ (1 / n)) - 1", symbolic_formula);
///
/// // Calculate the period-by-period values.
/// let series = solution.series();
/// dbg!(&series);
/// ```
pub fn rate_solution<P, F>(periods: u32, present_value: P, future_value: F) -> RateSolution
    where
        P: Into<f64> + Copy,
        F: Into<f64> + Copy
{
    rate_solution_internal(periods, present_value.into(), future_value.into(), false)
}

pub fn rate_continuous<P, F>(periods: u32, present_value: P, future_value: F) -> f64
    where
        P: Into<f64> + Copy,
        F: Into<f64> + Copy
{
    rate_internal(periods, present_value.into(), future_value.into(), true)
}

pub fn rate_continuous_solution<P, F>(periods: u32, present_value: P, future_value: F) -> RateSolution
    where
        P: Into<f64> + Copy,
        F: Into<f64> + Copy
{
    rate_solution_internal(periods, present_value.into(), future_value.into(), true)
}

pub fn rate_internal(periods: u32, present_value: f64, future_value: f64, continuous_compounding: bool) -> f64 {
    if present_value + future_value == 0.0 {
        // This is a special case where any rate will work.
        return 0.0;
    }
    if future_value == 0.0 {
        // This is a special case where the rate must be -100% because present value is nonzero.
        return -1.0;
    }
    check_rate_parameters(periods, present_value, future_value);

    let rate = if continuous_compounding {
        // http://www.edmichaelreggie.com/TMVContent/rate.htm
        (future_value / present_value).log(std::f64::consts::E) / periods as f64
    } else {
        (future_value / present_value).powf(1.0 / periods as f64) - 1.0
    };

    if !rate.is_finite() {
        dbg!(periods, present_value, future_value, continuous_compounding, rate);
    }

    assert!(rate.is_finite());
    rate
}

pub fn rate_solution_internal(periods: u32, present_value: f64, future_value: f64, continuous_compounding: bool) -> RateSolution {
    if present_value == 0.0 && future_value == 0.0 {
        // This is a special case where any rate will work.
        let formula = "{special case}";
        let symbolic_formula = "***";
        let rate = 0.0;
        return RateSolution::new(TvmSolution::new(TvmVariable::Rate, continuous_compounding, rate, periods, present_value, future_value, formula, symbolic_formula));
    }

    let rate = rate_internal(periods, present_value, future_value, continuous_compounding);
    let (formula, symbolic_formula) = if continuous_compounding {
        let formula = format!("{:.6} = log({:.4} / {:.4}, base {:.6}) / {}", rate, future_value, present_value, std::f64::consts::E, periods);
        let symbolic_formula = "r = log(fv / pv, base e) / t";
        (formula, symbolic_formula)
    } else {
        let formula = format!("{:.6} = (({:.4} / {:.4}) ^ (1 / {})) - 1", rate, future_value, present_value, periods);
        let symbolic_formula = "r = ((fv / pv) ^ (1 / n)) - 1";
        (formula, symbolic_formula)
    };
    return RateSolution::new(TvmSolution::new(TvmVariable::Rate, continuous_compounding, rate, periods, present_value.into(), future_value, &formula, symbolic_formula))
}

fn check_rate_parameters(periods: u32, present_value: f64, future_value: f64) {
    assert!(present_value.is_finite(), "The present value must be finite (not NaN or infinity)");
    assert!(future_value.is_finite(), "The future value must be finite (not NaN or infinity)");
    assert!(!(present_value == 0.0 && future_value != 0.0), "The present value is zero and the future value is nonzero so there's no way to solve for rate.");
    assert!(!(periods == 0 && present_value + future_value != 0.0), "The number of periods is zero and the present value plus the future value is nonzero so there's no way to solve for rate.");
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
        // Zero periods, values add up to zero.
        assert_rounded_6(0.0, rate(0, 10_000.0, -10_000.0));

        // Nonzero periods, values the same.
        assert_rounded_6(0.0, rate(12, 10_000.0, 10_000.0));
    }

    #[should_panic]
    #[test]
    fn test_rate_err_present_value_nan() {
        // The present value is not a number.
        rate(12, std::f64::NAN, 1_000.0);
    }

    #[should_panic]
    #[test]
    fn test_rate_err_present_value_inf() {
        // The present value is infinite.
        rate(12, std::f64::INFINITY, 1_000.0);
    }

    #[should_panic]
    #[test]
    fn test_rate_err_future_value_nan() {
        // The future value is not a number.
        rate(12, 1_000.0, std::f64::NAN);
    }

    #[should_panic]
    #[test]
    fn test_rate_err_future_value_inf() {
        // The future value is infinite.
        rate(12, 1_000.0, std::f64::NEG_INFINITY);
    }

    #[should_panic]
    #[test]
    fn test_rate_err_zero_periods() {
        // Zero periods, values don't add up to zero.
        rate(0, 10_000.0, 10_000.0);
    }

}

