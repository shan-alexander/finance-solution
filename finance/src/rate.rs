//! **Periodic rate calculations.** Given an initial investment amount, a final amount, and a number of
//! periods such as periods, what does the rate per period need to be?
//! 
//! For most common usages, we recommend the [`rate_solution`](./fn.rate_solution.html) function to provide the best experience with debugging and additional features.
//!
// ! If you need to calculate the future value given a starting value, a number of periods, and one
// ! or more rates use [`future_value`] or related functions.
// !
// ! If you need to calculate the present value given a future value, a number of periods, and one
// ! or more rates use [`present_value`] or related functions.
// !
// ! If you need to calculate the number of periods given a fixed rate and a present and future value
// ! use [`periods`] or related functions.

// use log::warn;

use crate::tvm_simple::*;

// Needed for the Rustdoc comments.
#[allow(unused_imports)]
use crate::*;
use std::ops::Deref;

/// A record of a call to [rate_solution](./fn.rate_solution.html). The structure shows details such as the formula and
/// can calculate the period-by-period details.
#[derive(Clone, Debug)]
pub struct RateSolution {
    tvm_solution: TvmSolution,
}

/// The period-by-period details of a rate calculation. This is the result of a call to
/// [RateSolution::series](./struct.RateSolution.html#method.series).
#[derive(Clone, Debug)]
pub struct RateSeries(TvmSeries);

impl RateSolution {
    fn new(tvm_solution: TvmSolution) -> Self {
        assert!(tvm_solution.calculated_field().is_rate());
        Self {
            tvm_solution,
        }
    }

    /// Calculates the period-by-period details of a rate calculation.
    ///
    /// # Examples
    /// A rate calculation with simple compounding using [rate_solution](./fn.rate_solution.html).
    /// ```
    /// // The interest will compound monthly for two years.
    /// let periods = 24;
    ///
    /// // The starting value is $100,000.
    /// let present_value = 100_000.00;
    ///
    /// // The ending value is $15,000.
    /// let future_value = 125_000.00;
    ///
    /// // Calculate the periodic rate and create a struct that supports further operations.
    /// let solution = finance::rate_solution(periods, present_value, future_value);
    /// dbg!(&solution);
    ///
    /// // Calculate the period-by-period values.
    /// let series = solution.series();
    /// dbg!(&series);
    ///
    /// // Print the period-by-period details in a formatted table using 2 decimal places.
    /// let locale = finance::num_format::Locale::en;
    /// series.print_table_locale(&locale, 2);
    ///
    /// // Print only the periods where the value has grown to at least $120,000, and use default
    /// // formatting for the numbers.
    /// series
    ///     .filter(|entry| entry.value() >= 120_000.0)
    ///     .print_table();
    /// ```
    pub fn series(&self) -> RateSeries {
        // For a rate, periods, or future value calculation the the period-by-period values are
        // calculated the same way.
        RateSeries::new(self.tvm_solution.series())
    }

    pub fn print_series_table(&self) {
        self.series().print_table();
    }

    pub fn print_series_table_locale(&self, locale: &num_format::Locale, precision: usize) {
        self.series().print_table_locale(locale, precision);
    }

    /// Returns true if the value is compounded continuously rather than period-by-period.
    pub fn continuous_compounding(&self) -> bool {
        self.tvm_solution.continuous_compounding()
    }

    /// Returns the periodic rate that was calculated based on the periods, present value, and
    /// future value.
    pub fn rate(&self) -> f64 {
        self.tvm_solution.rate()
    }

    /// Returns the number of periods that were given as an input to the rate calculation. In the
    /// rare case where the number of periods might not be a whole number use [fractional_periods](./struct.RateSolution.html#method.fractional_periods).
    pub fn periods(&self) -> u32 {
        self.tvm_solution.periods()
    }

    /// Returns the number of periods as a floating point number. Most of the time this is unneeded
    /// and it's better to use [periods](./struct.RateSolution.html#method.periods) which is an integer. The floating point number is relevant
    /// only in the unusual case where the current `RateSolution` was created by starting with a
    /// period calculation, then transforming it into a rate calculation with a call to
    //  [TvmSolution::rate_solution](./struct.TvmSolution.html#method.rate_solution).
    pub fn fractional_periods(&self) -> f64 {
        self.tvm_solution.fractional_periods()
    }

    /// Returns the present value that was given as an input to the rate calculation.
    pub fn present_value(&self) -> f64 {
        self.tvm_solution.present_value()
    }

    /// Returns the future value that was given as an input to the rate calculation.
    pub fn future_value(&self) -> f64 {
        self.tvm_solution.future_value()
    }

    /// Returns a text version of the formula used to calculate the rate. The formula includes the
    /// actual values rather than variable names. For the formula with variables such as "n" for
    /// periods call [symbolic_formula](./struct.RateSolution.html#method.symbolic_formula).
    pub fn formula(&self) -> &str {
        &self.tvm_solution.formula()
    }

    /// Returns a text version of the formula used to calculate the rate. The formula uses variables
    /// such as "n" for the number of periods. For the formula with the actual values rather than
    /// variables call [formula](./struct.RateSolution.html#method.formula).
    pub fn symbolic_formula(&self) -> &str {
        &self.tvm_solution.symbolic_formula()
    }

}

impl TimeValueOfMoneySolution for RateSolution {
    fn tvm_solution(&self) -> TvmSolution {
        self.tvm_solution.clone()
    }

    fn tvm_series(&self) -> TvmSeries {
        self.series().into()
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
/// * To calculate a periodic rate returning an f64 value instead of solution object, use [`rate`](./fn.rate.html).
///
// / Functions to solve for other values:
// / * To calculate the future value given a present value, a number of periods, and one or more
// / rates use [`future_value`] or related functions.
// / * To calculate the present value given a future value, a number of periods, and one or more
// / rates use [`present_value`] or related functions.
// / * To calculate the number of periods given a fixed rate and a present and future value use
// / [`periods`] or related functions.
///
/// **The formula:**
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
/// // The starting value is $10,000.
/// // The ending value is $15,000.
/// let periods = 10;
/// let present_value = 10_000.00;
/// let future_value = 15_000.00;
///
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

/// Returns the annual percentage rate (APR) of an investment with continuous compounding.
///
pub fn rate_continuous<P, F>(periods: u32, present_value: P, future_value: F) -> f64
    where
        P: Into<f64> + Copy,
        F: Into<f64> + Copy
{
    rate_internal(periods, present_value.into(), future_value.into(), true)
}

/// Returns a solution object including the annual percentage rate (APR) of an investment with
/// continuous compounding.
///
pub fn rate_continuous_solution<P, F>(periods: u32, present_value: P, future_value: F) -> RateSolution
    where
        P: Into<f64> + Copy,
        F: Into<f64> + Copy
{
    rate_solution_internal(periods, present_value.into(), future_value.into(), true)
}

fn rate_internal(periods: u32, present_value: f64, future_value: f64, continuous_compounding: bool) -> f64 {
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
        // http://www.edmichaelreggie.com/TMVContent/APR.htm
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

pub (crate) fn rate_solution_internal(periods: u32, present_value: f64, future_value: f64, continuous_compounding: bool) -> RateSolution {
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
    //use crate::*;

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

