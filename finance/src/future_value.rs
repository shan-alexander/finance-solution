//! **Future value calculations.** Given an initial investment amount, a number of periods such as
//! periods, and fixed or varying interest rates, what is the value of the investment at the end?
//!
//! For most common usages, we recommend the [future_value_solution](./fn.future_value_solution.html) function, which provides a better debugging experience and additional features.
//! 
//! For more complex scenarios, which involve varying rates in each period, we recommend the [future_value_schedule_solution](./fn.future_value_schedule_solution.html) function.
//! 
//! To simply return an f64 value of the future value answer, use the [future_value](./fn.future_value.html) function.
//! 
// ! If you need to calculate the present value given a future value, a number of periods, and one
// ! or more rates use [`present_value`] or related functions.
// !
// ! If you need to calculate a fixed rate given a present value, future value, and number of periods
// ! use [`rate`] or related functions.
// !
// ! If you need to calculate the number of periods given a fixed rate and a present and future value
// ! use [`periods`] or related functions.
//!
//! ## Example
//! 
//! ```
//! let (rate, periods, present_value) = (0.034, 10, 1_000);
//! let fv = finance::future_value_solution(rate, periods, present_value);
//! dbg!(fv);
//! ```
//! Outputs to terminal:
//! ```text
//! {
//!     calculated_field: FutureValue,
//!     continuous_compounding: false,
//!     rate: 0.034,
//!     periods: 10,
//!     fractional_periods: 10.0,
//!     present_value: 1000.0,
//!     future_value: 1397.0288910795477,
//!     formula: "1397.0289 = 1000.0000 * (1.034000 ^ 10)",
//!     symbolic_formula: "fv = pv * (1 + r)^n",
//! }
//! ```

use log::warn;

use crate::tvm_simple::*;

#[allow(unused_imports)]
use crate::{rate::*, periods::*, present_value::*};
use std::ops::Deref;

/// A record of a call to [future_value_solution](./fn.future_value_solution.html), a Future Value calculation where the rate is
/// fixed. The structure shows details such as the formula and can calculate the period-by-period
/// details.
#[derive(Clone, Debug)]
pub struct FutureValueSolution {
    tvm_solution: TvmSolution,
}

/// A record of a call to [future_value_schedule_solution](./fn.future_value_schedule_solution.html), a Future Value calculation where the rate may
/// vary for each period. The structure can calculate the period-by-period details.
#[derive(Clone, Debug)]
pub struct FutureValueScheduleSolution {
    tvm_solution: TvmScheduleSolution,
}

/// The period-by-period details of a Future Value calculation. This is the result of a call to
/// ['FutureValueSolution::series`] if the rate is fixed or a call to
/// [`FutureValueScheduleSolution::series`] if the rate may vary for each period.
#[derive(Clone, Debug)]
pub struct FutureValueSeries(TvmSeries);

impl FutureValueSolution {
    fn new (tvm_solution: TvmSolution) -> Self {
        assert!(tvm_solution.calculated_field().is_future_value());
        Self {
            tvm_solution,
        }
    }

    /// Returns true if the value was compounded continuously rather than period-by-period.
    pub fn continuous_compounding(&self) -> bool {
        self.tvm_solution.continuous_compounding()
    }

    /// Returns the periodic rate that was given as an input to the future value calculation.
    pub fn rate(&self) -> f64 {
        self.tvm_solution.rate()
    }

    /// Returns the number of periods that were given as an input to the future value calculation.
    /// In the rare case where the number of periods might not be a whole number use
    /// [fractional_periods](./struct.FutureValueSolution.html#method.fractional_periods).
    pub fn periods(&self) -> u32 {
        self.tvm_solution.periods()
    }

    /// Returns the number of periods as a floating point number. Most of the time this is unneeded
    /// and it's better to use [periods](./struct.PresentValueSolution.html#method.periods) which is an integer. The floating point number is relevant
    /// only in the unusual case where the current `FutureValueSolution` was created by starting
    /// with a period calculation, then transforming it into a future value calculation with a call
    /// to [TvmSolution::future_value_solution](./struct.TvmSolution.html#method.future_value_solution).
    pub fn fractional_periods(&self) -> f64 {
        self.tvm_solution.fractional_periods()
    }

    /// Returns the present value that was given as an input to the future value calculation.
    pub fn present_value(&self) -> f64 {
        self.tvm_solution.present_value()
    }

    /// Returns the future value that was calculated based on the provided rate, periods, and
    /// present value.
    pub fn future_value(&self) -> f64 {
        self.tvm_solution.future_value()
    }

    /// Returns a text version of the formula used to calculate the future value. The formula
    /// includes the actual values rather than variable names. For the formula with variables such
    /// as "n" for periods call [symbolic_formula](./struct.FutureValueSolution.html#method.symbolic_formula).
    pub fn formula(&self) -> &str {
        &self.tvm_solution.formula()
    }

    /// Returns a text version of the formula used to calculate the future value. The formula uses
    /// variables such as "r" for the rate. For the formula with the actual values rather than
    /// variables call [formula](./struct.FutureValueSolution.html#method.formula).
    pub fn symbolic_formula(&self) -> &str {
        &self.tvm_solution.symbolic_formula()
    }

    /// Calculates the period-by-period details of a future value calculation.
    ///
    /// # Examples
    /// Future value calculation with a fixed periodic rate. Uses [`future_value_solution`].
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
    /// // Print the period-by-period numbers in a formatted table.
    /// series.print_table();
    ///
    /// // Create a vector with every fourth period.
    /// let filtered_series = series
    ///     .iter()
    ///     .filter(|x| x.period() % 4 == 0)
    ///     .collect::<Vec<_>>();
    /// dbg!(&filtered_series);
    /// assert_eq!(7, filtered_series.len());
    /// ```
    pub fn series(&self) -> FutureValueSeries {
        // For a rate, periods, or future value calculation the period-by-period values are
        // calculated the same way.
        FutureValueSeries::new(self.tvm_solution.series())
    }

    /// Prints a formatted table with the period-by-period details of a future value calculation.
    ///
    /// Money amounts are rounded to four decimal places, rates to six places, and numbers are
    /// formatted similar to Rust constants such as "10_000.0322". For more control over formatting
    /// use [`FutureValueSolution::print_series_table_locale'].
    ///
    /// # Examples
    /// ```
    /// finance::future_value_solution(0.045, 5, 10_000)
    ///     .print_series_table();
    /// ```
    /// Output:
    /// ```text
    /// period      rate        value
    /// ------  --------  -----------
    ///      0  0.000000  10_000.0000
    ///      1  0.045000  10_450.0000
    ///      2  0.045000  10_920.2500
    ///      3  0.045000  11_411.6612
    ///      4  0.045000  11_925.1860
    ///      5  0.045000  12_461.8194
    /// ```
    pub fn print_series_table(&self) {
        self.series().print_table();
    }

    /// Prints a formatted table with the period-by-period details of a future value calculation.
    ///
    /// For a simpler function that doesn't require a locale use
    /// [`FutureValueSolution::print_series_table'].
    ///
    /// # Arguments
    /// * `locale` - A locale constant from the `num-format` crate such as `Locale::en` for English
    /// or `Locale::vi` for Vietnamese. The locale determines the thousands separator and decimal
    /// separator.
    /// * `precision` - The number of decimal places for money amounts. Rates will appear with at
    /// least six places regardless of this argument.
    ///
    /// # Examples
    /// ```
    /// // English formatting with "," for the thousands separator and "." for the decimal
    /// // separator.
    /// let locale = finance::num_format::Locale::en;
    ///
    /// // Show money amounts to two decimal places.
    /// let precision = 2;
    ///
    /// finance::future_value_solution(0.11, 4, 5_000)
    ///     .print_series_table_locale(&locale, precision);
    /// ```
    /// Output:
    /// ```text
    /// period      rate     value
    /// ------  --------  --------
    ///      0  0.000000  5,000.00
    ///      1  0.110000  5,550.00
    ///      2  0.110000  6,160.50
    ///      3  0.110000  6,838.16
    ///      4  0.110000  7,590.35
    /// ```
    pub fn print_series_table_locale(&self, locale: &num_format::Locale, precision: usize) {
        self.series().print_table_locale(locale, precision);
    }

}

impl TimeValueOfMoneySolution for FutureValueSolution {
    fn tvm_solution(&self) -> TvmSolution {
        self.tvm_solution.clone()
    }

    fn tvm_series(&self) -> TvmSeries {
        self.series().into()
    }
}

impl FutureValueScheduleSolution {
    fn new (tvm_solution: TvmScheduleSolution) -> Self {
        Self {
            tvm_solution,
        }
    }

    /// Returns the periodic rates that were given as an input to the future value calculation.
    pub fn rates(&self) -> &[f64] {
        self.tvm_solution.rates()
    }

    /// Returns the number of periods that were based on the number of rates passed to the future
    /// value schedule calculation.
    pub fn periods(&self) -> u32 {
        self.tvm_solution.periods()
    }

    /// Returns the present value that was given as an input to the future value calculation.
    pub fn present_value(&self) -> f64 {
        self.tvm_solution.present_value()
    }

    /// Returns the future value that was calculated based on the provided rates and present value.
    pub fn future_value(&self) -> f64 {
        self.tvm_solution.future_value()
    }

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
    pub fn series(&self) -> FutureValueSeries {
        // After period 0 this will hold the value of the previous period.
        let mut prev_value = None;

        // Add the values at each period.
        let mut series = vec![];
        for period in 0..=self.periods() {
            let (value, formula, symbolic_formula, rate) = if period == 0 {
                // This is period 0, the starting point, so the value at the end of this period is
                // simply the present value.
                let value = self.present_value();
                let formula = format!("{:.4}", value);
                let symbolic_formula = "value = pv";
                let rate = 0.0;
                (value, formula, symbolic_formula, rate)
            } else {
                // We want the rate for the current period. However, periods are 1-based and
                // the vector of rates is 0-based, so the corresponding rate is at period - 1.
                let rate = self.rates()[period as usize - 1];
                assert!(rate >= -1.0);
                let rate_multiplier = 1.0 + rate;
                assert!(rate_multiplier >= 0.0);
                let value = prev_value.unwrap() * rate_multiplier;
                let formula = format!("{:.4} = {:.4} * {:.6}", value, prev_value.unwrap(), rate_multiplier);
                let symbolic_formula = "value = {previous period value} * (1 + r)";
                (value, formula, symbolic_formula, rate)
            };
            assert!(value.is_finite());
            prev_value = Some(value);
            series.push(TVMPeriod::new(period, rate, value, &formula, &symbolic_formula))
        }
        FutureValueSeries::new(TvmSeries::new(series))
    }

    /// Prints a formatted table with the period-by-period details of a future value calculation.
    ///
    /// Money amounts are rounded to four decimal places, rates to six places, and numbers are
    /// formatted similar to Rust constants such as "10_000.0322". For more control over formatting
    /// use [`FutureValueScheduleSolution::print_series_table_locale'].
    ///
    /// # Examples
    /// ```
    /// finance::future_value_schedule_solution(&[0.11, 0.13, 0.14], 50_000)
    ///     .print_series_table();
    /// ```
    pub fn print_series_table(&self) {
        self.series().print_table();
    }

    /// Prints a formatted table with the period-by-period details of a future value schedule
    /// calculation.
    ///
    /// For a simpler function that doesn't require a locale use
    /// [`FutureValueScheduleSolution::print_series_table'].
    ///
    /// # Arguments
    /// * `locale` - A locale constant from the `num-format` crate such as `Locale::en` for English
    /// or `Locale::vi` for Vietnamese. The locale determines the thousands separator and decimal
    /// separator.
    /// * `precision` - The number of decimal places for money amounts. Rates will appear with at
    /// least six places regardless of this argument.
    ///
    /// # Examples
    /// ```
    /// // Vietnamese formatting with "." for the thousands separator and "," for the decimal
    /// // separator.
    /// let locale = finance::num_format::Locale::vi;
    ///
    /// // Round money amounts to whole numbers. Rates will still have six decimal places.
    /// let precision = 0;
    ///
    /// finance::future_value_schedule_solution(&[0.05, 0.043, 0.071], 1_000)
    ///     .print_series_table_locale(&locale, precision);
    /// ```
    pub fn print_series_table_locale(&self, locale: &num_format::Locale, precision: usize) {
        self.series().print_table_locale(locale, precision);
    }

}

impl TimeValueOfMoneyScheduleSolution for FutureValueScheduleSolution {
    fn tvm_solution(&self) -> TvmScheduleSolution {
        self.tvm_solution.clone()
    }

    fn tvm_series(&self) -> TvmSeries {
        self.series().into()
    }
}

impl FutureValueSeries {
    fn new (series: TvmSeries) -> Self {
        Self {
            0: series,
        }
    }
}

impl Deref for FutureValueSeries {
    type Target = TvmSeries;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<TvmSeries> for FutureValueSeries {
    fn into(self) -> TvmSeries {
        self.0
    }
}

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
/// > future_value = present_value * (1 + rate)<sup>periods</sup>
///
/// or with the more commonly used variables:
/// > fv = pv * (1 + r)<sup>n</sup>
///
/// # Arguments
/// * `rate` - The rate at which the investment grows or shrinks per period, expressed as a
/// floating point number. For instance 0.05 would mean 5% growth. Often appears as `r` or `i` in
/// formulas.
/// * `periods` - The number of periods such as quarters or periods. Often appears as `n` or `t`.
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
/// # use finance::*;
/// // The investment grows by 3.4% per quarter.
/// let rate = 0.034;
///
/// // The investment will grow for 5 quarters.
/// let periods = 5;
///
/// // The initial investment is $250,000.
/// let present_value = 250_000;
///
/// let future_value = future_value(rate, periods, present_value);
/// // Confirm that the future value is correct to four decimal places (one
/// // hundredth of a cent).
/// assert_rounded_4(295_489.9418, future_value);
/// ```
/// Investment that loses money each year.
/// ```
/// # use finance::*;
/// // The investment loses 5% per year.
/// let rate = -0.05;
///
/// // The investment will shrink for 6 periods.
/// let periods = 6;
///
/// // The initial investment is $10,000.75.
/// let present_value = 10_000.75;
///
/// let future_value = future_value(rate, periods, present_value);
/// // Confirm that the future value is correct to the penny.
/// assert_rounded_2(7351.47, future_value);
/// ```
/// Error case: The investment loses 105% per year. There's no way to work out
/// what this means so the call will panic.
/// ```should_panic
/// # use finance::future_value;
/// let (rate, periods, present_value) = (-1.05, 6, 10_000.75);
/// let future_value = future_value(rate, periods, present_value);
/// ```
pub fn future_value<T>(rate: f64, periods: u32, present_value: T) -> f64
    where T: Into<f64> + Copy
{
    future_value_internal(rate, periods as f64, present_value.into(), false)
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
/// The formula is:
/// > future_value = present_value * (1 + rate)<sup>periods</sup>
///
/// or with the more commonly used variables:
/// > fv = pv * (1 + r)<sup>n</sup>
///
/// # Arguments
/// * `rate` - The rate at which the investment grows or shrinks per period, expressed as a
/// floating point number. For instance 0.05 would mean 5% growth. Often appears as `r` or `i` in
/// formulas.
/// * `periods` - The number of periods such as quarters or periods. Often appears as `n` or `t`.
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
/// # use finance::*;
/// // The rate is 1.2% per month.
/// let rate = 0.012;
///
/// // The investment will grow for 8 months.
/// let periods = 8;
///
/// // The initial investment is $200,000.
/// let present_value = 200_000;
///
/// let solution = future_value_solution(rate, periods, present_value);
/// dbg!(&solution);
///
/// let future_value = solution.future_value();
/// assert_rounded_4(future_value, 220_026.0467);
///
/// // Examine the formulas.
/// let formula = solution.formula();
/// dbg!(&formula);
/// assert_eq!(formula, "220026.0467 = 200000.0000 * (1.012000 ^ 8)");
/// let symbolic_formula = solution.symbolic_formula();
/// dbg!(&symbolic_formula);
/// assert_eq!(symbolic_formula, "fv = pv * (1 + r)^n");
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
/// // The investment will grow for 12 periods.
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
/// scenarios.retain(|x| x.future_value() >= 200_000.00 && x.future_value() <= 400_000.00);
/// dbg!(&scenarios);
/// assert_eq!(7, scenarios.len());
///
/// // Check the formulas for the first of the remainingc scenarios.
/// let formula = scenarios[0].formula();
/// dbg!(&formula);
/// assert_eq!("201219.6472 = 100000.0000 * (1.060000 ^ 12)", formula);
/// let symbolic_formula = scenarios[0].symbolic_formula();
/// dbg!(&symbolic_formula);
/// assert_eq!("fv = pv * (1 + r)^n", symbolic_formula);
/// ```
pub fn future_value_solution<T>(rate: f64, periods: u32, present_value: T) -> FutureValueSolution
    where T: Into<f64> + Copy
{
    future_value_solution_internal(rate, periods as f64, present_value.into(), false)
}

pub fn future_value_continuous<T>(rate: f64, periods: u32, present_value: T) -> f64
    where T: Into<f64> + Copy
{
    future_value_internal(rate, periods as f64, present_value.into(), true)
}

pub fn future_value_continuous_solution<T>(rate: f64, periods: u32, present_value: T) -> FutureValueSolution
    where T: Into<f64> + Copy
{
    future_value_solution_internal(rate, periods as f64, present_value.into(), true)
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
/// # use finance::*;
/// // The rates vary by year: 4% followed by -3.9%, 10.6%, and -5.7%.
/// let rates = [0.04, -0.039, 0.106, -0.057];
///
/// // The initial investment is $75,000.
/// let present_value = 75_000.00;
///
/// let future_value = future_value_schedule(&rates, present_value);
/// dbg!(&future_value);
/// assert_rounded_4(78_178.0458, future_value);
/// ```
/// Error case: One of the rates shows a drop of over 100%. There's no way to work out what this
/// means so the call will panic.
/// ```should_panic
/// # use finance::future_value_schedule;
/// let rates = [0.116, -100.134, -0.09, 0.086];
/// let present_value = 4_000.00;
/// let schedule = future_value_schedule(&rates, present_value);
/// ```
pub fn future_value_schedule<T>(rates: &[f64], present_value: T) -> f64
    where T: Into<f64> + Copy
{
    let present_value= present_value.into();
    let periods = rates.len();

    // Check the parameters including all of the provided rates.
    for rate in rates {
        check_future_value_parameters(*rate, periods as f64, present_value);
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
/// use finance::*;
/// // The rates vary by year: 8.1% followed by 11%, 4%, and -2.3%.
/// let rates = [0.081, 0.11, 0.04, -0.023];
///
/// // The initial investment is $10,000.
/// let present_value = 10_000.00;
///
/// let solution = future_value_schedule_solution(&rates, present_value);
/// dbg!(&solution);
///
/// let future_value = solution.future_value();
/// dbg!(&future_value);
/// assert_rounded_4(future_value, 12_192.0455);
///
/// // Calculate the value for each period.
/// let series = solution.series();
/// dbg!(&series);
/// ```
/// Error case: One of the rates shows a drop of over 100%. There's no way to work out what this
/// means so the call will panic.
/// ```should_panic
/// use finance::*;
/// let rates = [0.116, -100.134, -0.09, 0.086];
/// let present_value = 4_000.00;
/// let schedule = future_value_schedule(&rates, present_value);
/// ```
pub fn future_value_schedule_solution<T>(rates: &[f64], present_value: T) -> FutureValueScheduleSolution
    where T: Into<f64> + Copy
{
    let future_value = future_value_schedule(rates, present_value);
    FutureValueScheduleSolution::new(TvmScheduleSolution::new(TvmVariable::FutureValue, rates, present_value.into(), future_value))
}

pub(crate) fn future_value_internal(rate: f64, periods: f64, present_value: f64, continuous_compounding: bool) -> f64 {
    check_future_value_parameters(rate, periods, present_value);
    let future_value = if continuous_compounding {
        // http://www.edmichaelreggie.com/TMVContent/rate.htm
        present_value * std::f64::consts::E.powf(rate * periods)
    } else {
        present_value * (1.0 + rate).powf(periods)
    };
    assert!(future_value.is_finite());
    future_value
}

pub(crate) fn future_value_solution_internal(rate: f64, periods: f64, present_value: f64, continuous_compounding: bool) -> FutureValueSolution {
    let future_value = future_value_internal(rate, periods, present_value, continuous_compounding);
    let (formula, symbolic_formula) = if continuous_compounding {
        let formula = format!("{:.4} = {:.4} * {:.6}^({:.6} * {})", future_value, present_value, std::f64::consts::E, rate, periods);
        let symbolic_formula = "fv = pv * e^(rt)";
        (formula, symbolic_formula)
    } else {
        let rate_multiplier = 1.0 + rate;
        assert!(rate_multiplier >= 0.0);
        let formula = format!("{:.4} = {:.4} * ({:.6} ^ {})", future_value, present_value, rate_multiplier, periods);
        let symbolic_formula = "fv = pv * (1 + r)^n";
        (formula, symbolic_formula)
    };
    FutureValueSolution::new(TvmSolution::new_fractional_periods(TvmVariable::FutureValue, continuous_compounding, rate, periods, present_value.into(), future_value, &formula, symbolic_formula))
}

fn check_future_value_parameters(rate: f64, _periods: f64, present_value: f64) {
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
        let actual_value = future_value_solution(rate_of_return, periods, present_value_1).future_value();
        assert_rounded_4(expected_value, actual_value);
    }

    #[test]
    fn test_future_value_solution_2() {
        let rate_of_return = 0.08;
        let periods = 6;
        let present_value_1 = 13_000.0;
        let expected_value = 20_629.37;
        let actual_value = future_value_solution(rate_of_return, periods, present_value_1).future_value();
        assert_rounded_2(expected_value, actual_value);
    }

    #[test]
    fn test_future_value_solution_3() {
        // test negative rate
        let rate_of_return = -0.09;
        let periods = 6;
        let present_value = 8_804.84368898;
        let expected_value = 5_000.00;
        let actual_value = future_value_solution(rate_of_return, periods, present_value).future_value();
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
        let try_1 = future_value_solution(rate_of_return, periods, present_value).future_value();
        assert!(try_1 < present_value.into());

        let rate_of_return = -0.9;
        let try_2 = future_value_solution(rate_of_return, periods, present_value).future_value();
        assert!(try_2 < present_value.into());

        let rate_of_return = -3.2;
        let result = std::panic::catch_unwind(|| future_value_solution(rate_of_return, periods, present_value));
        assert!(result.is_err());  //probe further for specific error type here, if desired
    }

}

/*
$$\begin{tikzpicture}[scale=1.0544]\small
\begin{axis}[axis line style=gray,
	samples=120,
	width=9.0cm,height=6.4cm,
	xmin=-1.5, xmax=1.5,
	ymin=0, ymax=1.8,
	restrict y to domain=-0.2:2,
	ytick={1},
	xtick={-1,1},
	axis equal,
	axis x line=center,
	axis y line=center,
	xlabel=$x$,ylabel=$y$]
\addplot[red,domain=-2:1,semithick]{exp(x)};
\addplot[black]{x+1};
\addplot[] coordinates {(1,1.5)} node{$y=e^{(rx)}$};
\addplot[red] coordinates {(-1,0.6)} node{$y=(1+{r \over x})^x$};
\path (axis cs:0,0) node [anchor=north west,yshift=-0.07cm] {0};
\end{axis}
\end{tikzpicture}$$



*/


