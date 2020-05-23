//! The internal module which supports the solution struct for the family of Time-value-of-money equations
//! which do not involve payments. For example, future value, present value, rate, and periods.

use crate::*;
use std::ops::Deref;
use std::fmt::{Display, Formatter, Error};

pub mod future_value;
#[doc(inline)]
pub use future_value::*;

pub mod present_value;
#[doc(inline)]
pub use present_value::*;

pub mod periods;
#[doc(inline)]
pub use periods::*;

pub mod rate;
#[doc(inline)]
pub use rate::*;

/// Enumeration used for the `calculated_field` field in [`TvmSolution`] and [`TvmSchedule`] to keep
/// track of what was calculated, either the periodic rate, the number of periods, the present
/// value, or the future value.
#[derive(Clone, Debug, Hash, PartialEq)]
pub enum TvmVariable {
    Rate,
    Periods,
    PresentValue,
    FutureValue,
}

#[derive(Clone, Debug)]
pub struct TvmSolution {
    calculated_field: TvmVariable,
    continuous_compounding: bool,
    rate: f64,
    periods: u32,
    fractional_periods: f64,
    present_value: f64,
    future_value: f64,
    formula: String,
    symbolic_formula: String,
}

/// A record of a Time Value of Money calculation where the rate may vary by period.
///
/// It's the result of calling [FutureValueScheduleSolution.tvm_solution](./struct.FutureValueScheduleSolution.html#method.tvm_solution)
/// or [PresentValueScheduleSolution.tvm_solution](./struct.PresentValueScheduleSolution.html#method.tvm_solution)
#[derive(Clone, Debug)]
pub struct TvmScheduleSolution {
    calculated_field: TvmVariable,
    rates: Vec<f64>,
    periods: u32,
    present_value: f64,
    future_value: f64,
}

#[derive(Clone, Debug)]
pub struct TvmSeries(Vec<TvmPeriod>);

/// The value of an investment at the end of a given period, part of a Time Value of Money
/// calculation.
///
/// This is either:
/// * Part of [`TvmSolution`] produced by calling [`rate_solution`], [`periods_solution`],
/// [`present_value_solution`], or [`future_value_solution`].
/// * Part of [`TvmSchedule`] produced by calling [`present_value_schedule`] or
/// [`future_value_schedule`].
#[derive(Clone, Debug)]
pub struct TvmPeriod {
    period: u32,
    rate: f64,
    value: f64,
    formula: String,
    symbolic_formula: String,
}

impl TvmVariable {
    /// Returns true if the variant is TvmVariable::Rate indicating that the periodic rate was
    /// calculated from the number of periods, the present value, and the future value.
    pub fn is_rate(&self) -> bool {
        match self {
            TvmVariable::Rate => true,
            _ => false,
        }
    }

    /// Returns true if the variant is TvmVariable::Periods indicating that the number of periods
    /// was calculated from the periocic rate, the present value, and the future value.
    pub fn is_periods(&self) -> bool {
        match self {
            TvmVariable::Periods => true,
            _ => false,
        }
    }

    /// Returns true if the variant is TvmVariable::PresentValue indicating that the present value
    /// was calculated from one or more periocic rates, the number of periods, and the future value.
    pub fn is_present_value(&self) -> bool {
        match self {
            TvmVariable::PresentValue => true,
            _ => false,
        }
    }

    /// Returns true if the variant is TvmVariable::FutureValue indicating that the future value
    /// was calculated from one or more periocic rates, the number of periods, and the present value.
    pub fn is_future_value(&self) -> bool {
        match self {
            TvmVariable::FutureValue => true,
            _ => false,
        }
    }

    pub(crate) fn table_column_spec(&self, visible: bool) -> (String, String, bool) {
        // Return something like ("period", "i") or ("rate", "r") with the column label and data
        // type needed by a print_table() or similar function.
        let data_type = match self {
            TvmVariable::Periods => "i",
            TvmVariable::Rate => "r",
            _ => "f",
        };
        // We don't do anything with the visible argument except include it in the tuple. This
        // makes the calling code simpler.
        (self.to_string(), data_type.to_string(), visible)
    }


    }

impl Display for TvmVariable {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            TvmVariable::Rate => write!(f, "Rate"),
            TvmVariable::Periods => write!(f, "Periods"),
            TvmVariable::PresentValue => write!(f, "Present Value"),
            TvmVariable::FutureValue => write!(f, "Future Value"),
        }
    }
}

impl Eq for TvmVariable {}

impl TvmSolution {
    pub(crate) fn new(calculated_field: TvmVariable, continuous_compounding: bool, rate: f64, periods: u32, present_value: f64, future_value: f64, formula: &str, symbolic_formula: &str) -> Self {
        assert!(rate.is_finite());
        assert!(present_value.is_finite());
        assert!(future_value.is_finite());
        assert!(!formula.is_empty());
        assert!(!symbolic_formula.is_empty());
        Self::new_fractional_periods(calculated_field, continuous_compounding, rate, periods as f64, present_value, future_value, formula, symbolic_formula)
    }

    pub(crate) fn new_fractional_periods(calculated_field: TvmVariable, continuous_compounding: bool, rate: f64, fractional_periods: f64, present_value: f64, future_value: f64, formula: &str, symbolic_formula: &str) -> Self {
        assert!(rate >= -1.0);
        assert!(fractional_periods >= 0.0);
        assert!(present_value.is_finite());
        assert!(future_value.is_finite());
        assert!(formula.len() > 0);
        assert!(symbolic_formula.len() > 0);
        Self {
            calculated_field,
            continuous_compounding,
            rate,
            periods: round_fractional_periods(fractional_periods),
            fractional_periods,
            present_value,
            future_value,
            formula: formula.to_string(),
            symbolic_formula: symbolic_formula.to_string(),
        }
    }

    /// Calculates the value of an investment after each period.
    ///
    /// # Examples
    /// Calculates the period-by-period details of a future value calculation. Uses
    /// [`future_value_solution`].
    /// ```
    /// // The initial investment is $10,000.12, the interest rate is 1.5% per month, and the
    /// // investment will grow for 24 months using simple compounding.
    /// let solution = finance_solution::future_value_solution(0.015, 24, 10_000.12, false);
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
    /// Calculate a present value with a fixed rate then examine the period-by-period values. Uses
    /// [`present_value_solution`].
    /// ```
    /// // The interest rate is 7.8% per year, the investment will grow for 10 years using simple
    /// // compounding, and the final value will be 8_112.75.
    /// let solution = finance_solution::present_value_solution(0.078, 10, 8_112.75, false);
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
    /// let solution = finance_solution::present_value_schedule_solution(&rates, future_value);
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
    pub fn series(&self) -> TvmSeries {
        let rates = initialized_vector(self.periods as usize, self.rate);
        series_internal(self.calculated_field.clone(), self.continuous_compounding, &rates, self.fractional_periods, self.present_value, self.future_value)
    }

    /// Prints a formatted table with the period-by-period details of a time-value-of-money
    /// calculation.
    ///
    /// Money amounts are rounded to four decimal places, rates to six places, and numbers are
    /// formatted similar to Rust constants such as "10_000.0322". For more control over formatting
    /// use [`TvmSolution::print_series_table_locale'].
    ///
    /// # Examples
    /// ```
    /// finance_solution::future_value_solution(0.045, 5, 10_000, false)
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

    /// Prints a formatted table with the period-by-period details of a time-value-of-money
    /// calculation.
    ///
    /// For a simpler function that doesn't require a locale use
    /// [`TvmSolution::print_series_table'].
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
    /// let locale = finance_solution::num_format::Locale::en;
    ///
    /// // Show money amounts to two decimal places.
    /// let precision = 2;
    ///
    /// finance_solution::future_value_solution(0.11, 4, 5_000, false)
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

    /// Returns a variant of [`TvmVariable`] showing which value was calculated, either the periodic
    /// rate, number of periods, present value, or future value. To test for the enum variant use
    /// functions like `TvmVariable::is_rate`.
    ///
    /// # Examples
    /// ```
    /// // Calculate the future value of $25,000 that grows at 5% for 12 yeors.
    /// let solution = finance_solution::future_value_solution(0.05, 12, 25_000, false);
    /// assert!(solution.calculated_field().is_future_value());
    /// ```
    pub fn calculated_field(&self) -> &TvmVariable {
        &self.calculated_field
    }

    /// Returns true if the value is compounded continuously rather than period-by-period.
    pub fn continuous_compounding(&self) -> bool {
        self.continuous_compounding
    }

    /// Returns the periodic rate which is a calculated value if this `TvmSolution` struct is the
    /// result of a call to [`rate_solution`] and otherwise is one of the input values.
    pub fn rate(&self) -> f64 {
        self.rate
    }

    /// Returns the number of periods as a whole number. This is a calculated value if this
    /// `TvmSolution` struct is the result of a call to [`periods_solution`] and otherwise it's
    /// one of the input values. If the value was calculated the true result may not have been a
    /// whole number so this is that number rounded away from zero.
    pub fn periods(&self) -> u32 {
        self.periods
    }

    /// Returns the number of periods as a floating point number. This is a calculated value if this
    /// `TvmSolution` struct is the result of a call to [`periods_solution`] and otherwise it's
    /// one of the input values.
    pub fn fractional_periods(&self) -> f64 {
        self.fractional_periods
    }

    /// Returns the present value which is a calculated value if this `TvmSolution` struct is the
    /// result of a call to [`present_value_solution`] and otherwise is one of the input values.
    pub fn present_value(&self) -> f64 {
        self.present_value
    }

    /// Returns the future value which is a calculated value if this `TvmSolution` struct is the
    /// result of a call to [`future_value_solution`] and otherwise is one of the input values.
    pub fn future_value(&self) -> f64 {
        self.future_value
    }

    /// Returns a text version of the formula used to calculate the result which may have been the
    /// periodic rate, number of periods, present value, or future value depending on which function
    /// was called. The formula includes the actual values rather than variable names. For the
    /// formula with variables such as r for rate call [symbolic_formula](./struct.TvmSolution.html#method.symbolic_formula).
    pub fn formula(&self) -> &str {
        &self.formula
    }

    /// Returns a text version of the formula used to calculate the result which may have been the
    /// periodic rate, number of periods, present value, or future value depending on which function
    /// was called. The formula uses variables such as n for the number of periods. For the formula
    /// with the actual values rather than variables call [formula](./struct.TvmSolution.html#method.formula).
    pub fn symbolic_formula(&self) -> &str {
        &self.symbolic_formula
    }
    
    pub fn rate_solution(&self, continuous_compounding: bool, compounding_periods: Option<u32>) -> TvmSolution {
        let periods= compounding_periods.unwrap_or(self.periods);
        rate_solution_internal(periods, self.present_value, self.future_value, continuous_compounding)
    }

    pub fn periods_solution(&self, continuous_compounding: bool) -> TvmSolution {
        periods_solution_internal(self.rate, self.present_value, self.future_value, continuous_compounding)
    }

    pub fn present_value_solution(&self, continuous_compounding: bool, compounding_periods: Option<u32>) -> TvmSolution {
        let (rate, periods) = match compounding_periods {
            Some(periods) => ((self.rate * self.fractional_periods) / periods as f64, periods as f64),
            None => (self.rate, self.fractional_periods),
        };
        present_value_solution_internal(rate, periods, self.future_value, continuous_compounding)
    }

    pub fn future_value_solution(&self, continuous_compounding: bool, compounding_periods: Option<u32>) -> TvmSolution {
        let (rate, periods) = match compounding_periods {
            Some(periods) => ((self.rate * self.fractional_periods) / periods as f64, periods as f64),
            None => (self.rate, self.fractional_periods),
        };
        future_value_solution_internal(rate, periods, self.present_value, continuous_compounding)
    }

    /// Returns a struct with a set of what-if scenarios for the present value needed with a variety
    /// of compounding periods.
    ///
    /// # Arguments
    /// * `compounding_periods` - The compounding periods to include in the scenarios. The result
    /// will have a computed present value for each compounding period in this list.
    /// * `include_continuous_compounding` - If true, adds one scenario at the end of the results
    /// with continuous compounding instead of a given number of compounding periods.
    ///
    /// # Examples
    /// For a more detailed example with a related function see
    /// [future_value_vary_compounding_periods](./struct.TVMoneySolution.html#method.future_value_vary_compounding_periods)
    /// ```
    /// // Calculate the future value of an investment that starts at $83.33 and grows 20% in one
    /// // year using simple compounding. Note that we're going to examine how the present value
    /// // varies by the number of compounding periods but we're starting with a future value
    /// // calculation. It would have been fine to start with a rate, periods, or present value
    /// // calculation as well. It just depends on what information we have to work with.
    /// let solution = finance_solution::future_value_solution(0.20, 1, 83.333, false);
    /// dbg!(&solution);
    ///
    /// // The present value of $83.33 gives us a future value of about $100.00.
    /// finance_solution::assert_rounded_2!(100.00, solution.future_value());
    ///
    /// // We'll experiment with compounding annually, quarterly, monthly, weekly, and daily.
    /// let compounding_periods = [1, 4, 12, 52, 365];
    ///
    /// // Add a final scenario with continuous compounding.
    /// let include_continuous_compounding = true;
    ///
    /// // Compile a list of the present values needed to arrive at the calculated future value of $100
    /// // each of the above compounding periods as well a continous compounding.
    /// let scenarios = solution.present_value_vary_compounding_periods(&compounding_periods, include_continuous_compounding);
    /// dbg!(&scenarios);
    ///
    /// // Print the results in a formatted table.
    /// scenarios.print_table();
    ///
    /// ```
    /// Output from the last line:
    /// ```text
    /// Periods  Present Value
    /// -------  -------------
    ///       1        83.3330
    ///       4        82.2699
    ///      12        82.0078
    ///      52        81.9042
    ///     365        81.8772
    ///     inf        81.8727
    /// ```
    /// As we compound the interest more frequently we need a slightly smaller initial value to
    /// reach the same final value of $100 in one year. With more frequent compounding the required
    /// initial value approaches $81.87, the present value needed with continuous compounding.
    ///
    /// If we plot this using between 1 and 12 compounding periods it's clear that the required
    /// present value drops sharply if we go from compounding annually to compounding semiannually
    /// or quarterly but then is affected less and less as we compound more frequently:
    ///
    /// <img src="http://i.upmath.me/svg/%24%24%5Cbegin%7Btikzpicture%7D%5Bscale%3D1.0544%5D%0A%5Cbegin%7Baxis%7D%5Baxis%20line%20style%3Dgray%2C%0A%09samples%3D12%2C%0A%09width%3D9.0cm%2Cheight%3D6.4cm%2C%0A%09xmin%3D0%2C%20xmax%3D12%2C%0A%09ymin%3D80.5%2C%20ymax%3D84.5%2C%0A%09restrict%20y%20to%20domain%3D0%3A1000%2C%0A%09ytick%3D%7B81%2C%2082%2C%2083%2C%2084%7D%2C%0A%09xtick%3D%7B1%2C2%2C3%2C4%2C5%2C6%2C7%2C8%2C9%2C10%2C11%2C12%7D%2C%0A%09axis%20x%20line%3Dcenter%2C%0A%09axis%20y%20line%3Dcenter%2C%0A%09xlabel%3D%24n%24%2Cylabel%3D%24pv%24%5D%0A%5Caddplot%5Bblue%2Cdomain%3D1%3A12%2Csemithick%2C%20only%20marks%5D%7B100%2F((1%2B(0.2%2Fx))%5Ex)%7D%3B%0A%5Caddplot%5Bblack%2Cdomain%3D1%3A12%2C%20thick%5D%7B100%2F(e%5E(0.2))%7D%3B%0A%5Caddplot%5B%5D%20coordinates%20%7B(2.3%2C81.53)%7D%20node%7B%24pv%3D%7B100%20%5Cover%20e%5E%7B0.2%7D%7D%24%7D%3B%0A%5Caddplot%5Bblue%5D%20coordinates%20%7B(4.5%2C82.8)%7D%20node%7B%24pv%3D%7B100%20%5Cover%20(1%2B%7B0.2%20%5Cover%20n%7D)%5En%7D%24%7D%3B%0A%5Cpath%20(axis%20cs%3A0%2C83)%20node%20%5Banchor%3Dnorth%20west%2Cyshift%3D-0.07cm%5D%3B%0A%5Cend%7Baxis%7D%0A%5Cend%7Btikzpicture%7D%24%24" />
    pub fn present_value_vary_compounding_periods(&self, compounding_periods: &[u32], include_continuous_compounding: bool) -> ScenarioList {
        let rate_for_single_period = self.rate * self.fractional_periods;
        let mut entries = vec![];
        for periods in compounding_periods {
            let rate = rate_for_single_period / *periods as f64;
            let present_value = present_value_internal(rate, *periods as f64, self.future_value, self.continuous_compounding);
            entries.push((*periods as f64, present_value));
        }
        if include_continuous_compounding {
            let rate = rate_for_single_period;
            let periods = 1;
            let continuous_compounding = true;
            let present_value = present_value_internal(rate, periods as f64, self.future_value, continuous_compounding);
            entries.push((std::f64::INFINITY, present_value));
        }

        let setup = format!("Compare present values with different compounding periods where the rate is {} and the future value is {}.", format_rate(rate_for_single_period), format_float(self.future_value));
        ScenarioList::new(setup, TvmVariable::Periods, TvmVariable::PresentValue, entries)
    }

    /// Returns a struct with a set of what-if scenarios for the future value of an investment given
    /// a variety of compounding periods.
    ///
    /// # Arguments
    /// * `compounding_periods` - The compounding periods to include in the scenarios. The result
    /// will have a computed future value for each compounding period in this list.
    /// * `include_continuous_compounding` - If true, adds one scenario at the end of the results
    /// with continuous compounding instead of a given number of compounding periods.
    ///
    /// # Examples
    /// ```
    /// // The interest rate is 5% per quarter.
    /// let rate = 0.05;
    ///
    /// // The interest will be applied once per quarter for one year.
    /// let periods = 4;
    ///
    /// // The starting value is $100.00.
    /// let present_value = 100;
    ///
    /// let continuous_compounding = false;
    ///
    /// let solution = finance_solution::future_value_solution(rate, periods, present_value, continuous_compounding);
    /// dbg!(&solution);
    ///
    /// // We'll experiment with compounding annually, quarterly, monthly, weekly, and daily.
    /// let compounding_periods = [1, 4, 12, 52, 365];
    ///
    /// // Add a final scenario with continuous compounding.
    /// let include_continuous_compounding = true;
    ///
    /// // Compile a list of the future values with each of the above compounding periods as well as
    /// // continous compounding.
    /// let scenarios = solution.future_value_vary_compounding_periods(&compounding_periods, include_continuous_compounding);
    /// // The description in the `setup` field states that the rate is 20% since that's 5% times the
    /// // number of periods in the original calculation. The final entry has `input: inf` indicating
    /// // that we used continuous compounding.
    /// dbg!(&scenarios);
    ///
    /// // Print the results in a formatted table.
    /// scenarios.print_table();
    /// ```
    /// Output:
    /// ```text
    /// &solution = FutureValueSolution {
    ///     tvm_solution: TvmSolution {
    ///     calculated_field: FutureValue,
    ///     continuous_compounding: false,
    ///     rate: 0.05,
    ///     periods: 4,
    ///     fractional_periods: 4.0,
    ///     present_value: 100.0,
    ///     future_value: 121.55062500000003,
    ///     formula: "121.5506 = 100.0000 * (1.050000 ^ 4)",
    ///     symbolic_formula: "fv = pv * (1 + r)^n",
    /// },
    ///
    /// &scenarios = ScenarioList {
    ///     setup: "Compare future values with different compounding periods where the rate is 0.200000 and the present value is 100.0000.",
    ///     input_variable: Periods,
    ///     output_variable: FutureValue,
    ///     entries: [
    ///         { input: 1, output: 120.0000 },
    ///         { input: 4, output: 121.5506 },
    ///         { input: 12, output: 121.9391 },
    ///         { input: 52, output: 122.0934 },
    ///         { input: 365, output: 122.1336 },
    ///         { input: inf, output: 122.1403 },
    ///     ],
    /// }
    ///
    /// Periods  Future Value
    /// -------  ------------
    ///       1      120.0000
    ///       4      121.5506
    ///      12      121.9391
    ///      52      122.0934
    ///     365      122.1336
    ///     inf      122.1403
    /// ```
    /// With the same interest rate and overall time period, an amount grows faster if we compound
    /// the interest more frequently. As the number of compounding periods grows the future value
    /// approaches the limit of $122.14 that we get with continuous compounding.
    ///
    /// As a chart it looks like this, here using only 1 through 12
    /// compounding periods for clarity:
    ///
    /// <img src="http://i.upmath.me/svg/%24%24%5Cbegin%7Btikzpicture%7D%5Bscale%3D1.0544%5D%5Csmall%0A%5Cbegin%7Baxis%7D%5Baxis%20line%20style%3Dgray%2C%0A%09samples%3D12%2C%0A%09width%3D9.0cm%2Cheight%3D6.4cm%2C%0A%09xmin%3D0%2C%20xmax%3D12%2C%0A%09ymin%3D119%2C%20ymax%3D123%2C%0A%09restrict%20y%20to%20domain%3D0%3A1000%2C%0A%09ytick%3D%7B120%2C%20121%2C%20122%7D%2C%0A%09xtick%3D%7B1%2C2%2C3%2C4%2C5%2C6%2C7%2C8%2C9%2C10%2C11%2C12%7D%2C%0A%09axis%20x%20line%3Dcenter%2C%0A%09axis%20y%20line%3Dcenter%2C%0A%09xlabel%3D%24n%24%2Cylabel%3D%24fv%24%5D%0A%5Caddplot%5Bblue%2Cdomain%3D1%3A12%2Cthick%2C%20only%20marks%5D%7B100*((1%2B(0.2%2Fx))%5Ex)%7D%3B%0A%5Caddplot%5Bblack%2Cdomain%3D1%3A12%2Cthick%5D%7B100*(e%5E(0.2))%7D%3B%0A%5Caddplot%5B%5D%20coordinates%20%7B(2.5%2C122.4)%7D%20node%7B%24fv%3D100e%5E%7B0.2%7D%24%7D%3B%0A%5Caddplot%5Bblue%5D%20coordinates%20%7B(4.8%2C120.7)%7D%20node%7B%24fv%3D100(1%2B%7B0.2%20%5Cover%20n%7D)%5En%24%7D%3B%0A%5Cpath%20(axis%20cs%3A0%2C122)%20node%20%5Banchor%3Dnorth%20west%2Cyshift%3D-0.07cm%5D%3B%0A%5Cend%7Baxis%7D%0A%5Cend%7Btikzpicture%7D%24%24" />
    pub fn future_value_vary_compounding_periods(&self, compounding_periods: &[u32], include_continuous_compounding: bool) -> ScenarioList {
        let rate_for_single_period = self.rate * self.fractional_periods;
        let mut entries = vec![];
        for periods in compounding_periods {
            let rate = rate_for_single_period / *periods as f64;
            let future_value = future_value_internal(rate, *periods as f64, self.present_value, self.continuous_compounding);
            entries.push((*periods as f64, future_value));
        }
        if include_continuous_compounding {
            let rate = rate_for_single_period;
            let periods = 1;
            let continuous_compounding = true;
            let future_value = future_value_internal(rate, periods as f64, self.present_value, continuous_compounding);
            entries.push((std::f64::INFINITY, future_value));
        }

        let setup = format!("Compare future values with different compounding periods where the rate is {} and the present value is {}.", format_rate(rate_for_single_period), format_float(self.present_value));
        ScenarioList::new(setup, TvmVariable::Periods, TvmVariable::FutureValue, entries)
    }

    pub fn print_ab_comparison(
        &self,
        other: &TvmSolution)
    {
        self.print_ab_comparison_locale_opt(other, None, None);
    }

    pub fn print_ab_comparison_locale(
        &self,
        other: &TvmSolution,
        locale: &num_format::Locale,
        precision: usize)
    {
        self.print_ab_comparison_locale_opt(other, Some(locale), Some(precision));
    }

    fn print_ab_comparison_locale_opt(
        &self,
        other: &TvmSolution,
        locale: Option<&num_format::Locale>,
        precision: Option<usize>)
    {
        println!();
        print_ab_comparison_values_string("calculated_field", &self.calculated_field.to_string(), &other.calculated_field.to_string());
        print_ab_comparison_values_bool("continuous_compounding", self.continuous_compounding, other.continuous_compounding);
        print_ab_comparison_values_rate("rate", self.rate, other.rate, locale, precision);
        print_ab_comparison_values_int("periods", self.periods as i128, other.periods as i128, locale);
        if self.calculated_field.is_periods() {
            print_ab_comparison_values_float("fractional_periods", self.fractional_periods, other.fractional_periods, locale, precision);
        }
        print_ab_comparison_values_float("present_value", self.present_value, other.present_value, locale, precision);
        print_ab_comparison_values_float("future_value", self.future_value, other.future_value, locale, precision);
        print_ab_comparison_values_string("formula", &self.formula, &other.formula);
        print_ab_comparison_values_string("symbolic_formula", &self.symbolic_formula, &other.symbolic_formula);

        self.series().print_ab_comparison_locale_opt(&other.series(), locale, precision);
    }
}

impl PartialEq for TvmSolution {
    fn eq(&self, other: &Self) -> bool {
        self.calculated_field == other.calculated_field
            && self.continuous_compounding == other.continuous_compounding
            && is_approx_equal!(self.rate, other.rate)
            && self.periods == other.periods
            && is_approx_equal!(self.fractional_periods, other.fractional_periods)
            && is_approx_equal!(self.present_value, other.present_value)
            && is_approx_equal!(self.future_value, other.future_value)
            && self.formula == other.formula
            && self.symbolic_formula == other.symbolic_formula
    }
}

impl TvmScheduleSolution {
    pub(crate) fn new(calculated_field: TvmVariable, rates: &[f64], present_value: f64, future_value: f64) -> Self {
        for rate in rates.iter() {
            assert!(rate.is_finite());
        }
        assert!(present_value.is_finite());
        assert!(future_value.is_finite());
        Self {
            calculated_field,
            rates: rates.to_vec(),
            periods: rates.len() as u32,
            present_value,
            future_value,
        }
    }

    /// Returns a variant of [`TvmVariable`] showing which value was calculated, either the present
    /// value or the future value. To test for the enum variant use functions like
    /// `TvmVariable::is_future_value`.
    ///
    /// # Examples
    /// ```
    /// let solution = finance_solution::present_value_schedule_solution(&[0.011, 0.012, 0.009], 75_000);
    /// assert!(solution.calculated_field().is_present_value());
    /// ```
    pub fn calculated_field(&self) -> &TvmVariable {
        &self.calculated_field
    }

    /// Returns the periodic rates that were passed to the function.
    pub fn rates(&self) -> &[f64] {
        &self.rates
    }

    /// Returns the number of periods which was derived from the number of rates passed to the
    /// function.
    ///
    /// # Examples
    /// ```
    /// let solution = finance_solution::future_value_schedule_solution(&[0.05, 0.07, 0.05], 100_000);
    /// assert_eq!(3, solution.periods());
    /// ```
    pub fn periods(&self) -> u32 {
        self.periods
    }

    /// Returns the present value which is a calculated value if this `TvmSchedule` struct is the
    /// result of a call to [`present_value_schedule_solution`] and otherwise is one of the input
    /// values.
    pub fn present_value(&self) -> f64 {
        self.present_value
    }

    /// Returns the future value which is a calculated value if this `TvmSchedule` struct is the
    /// result of a call to [`future_value_schedule_solution`] and otherwise is one of the input
    /// values.
    pub fn future_value(&self) -> f64 {
        self.future_value
    }

    /// Calculates the value of an investment after each period.
    ///
    /// # Examples
    /// Calculate the period-by-period details of a future value calculation. Uses
    /// [`future_value_solution`].
    /// ```
    /// // The initial investment is $10,000.12, the interest rate is 1.5% per month, and the
    /// // investment will grow for 24 months using simple compounding.
    /// let solution = finance_solution::future_value_solution(0.015, 24, 10_000.12, false);
    /// dbg!(&solution);
    ///
    /// // Calculate the period-by-period details.
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
    pub fn series(&self) -> TvmSeries {
        series_internal(self.calculated_field.clone(), false, &self.rates,0.0, self.present_value, self.future_value)
    }

}

impl TvmSeries {
    pub(crate) fn new(series: Vec<TvmPeriod>) -> Self {
        Self {
            0: series,
        }
    }

    pub fn filter<P>(&self, predicate: P) -> Self
        where P: Fn(&&TvmPeriod) -> bool
    {
        Self {
            0: self.iter().filter(|x| predicate(x)).cloned().collect()
        }
    }

    pub fn print_table(&self) {
        self.print_table_locale_opt(None, None);
    }

    pub fn print_table_locale(&self, locale: &num_format::Locale, precision: usize) {
        self.print_table_locale_opt(Some(locale), Some(precision));
    }

    fn print_table_locale_opt(&self, locale: Option<&num_format::Locale>, precision: Option<usize>) {
        let columns = columns_with_strings(&[("period", "i", true), ("rate", "r", true), ("value", "f", true)]);
        let data = self.iter()
            .map(|entry| vec![entry.period.to_string(), entry.rate.to_string(), entry.value.to_string()])
            .collect::<Vec<_>>();
        print_table_locale_opt(&columns, data, locale, precision);
    }

    pub fn print_ab_comparison(
        &self,
        other: &TvmSeries)
    {
        self.print_ab_comparison_locale_opt(other, None, None);
    }

    pub fn print_ab_comparison_locale(
        &self,
        other: &TvmSeries,
        locale: &num_format::Locale,
        precision: usize)
    {
        self.print_ab_comparison_locale_opt(other, Some(locale), Some(precision))
    }

    fn print_ab_comparison_locale_opt(
        &self,
        other: &TvmSeries,
        locale: Option<&num_format::Locale>,
        precision: Option<usize>)
    {
        let columns = columns_with_strings(&[("period", "i", true),
                           ("rate_a", "r", true), ("rate_b", "r", true),
                           ("value_a", "f", true), ("value_b", "f", true)]);
        let mut data = vec![];
        let rows = max(self.len(), other.len());
        for row_index in 0..rows {
            data.push(vec![
                row_index.to_string(),
                self.get(row_index).map_or("".to_string(), |x| x.rate.to_string()),
                other.get(row_index).map_or("".to_string(), |x| x.rate.to_string()),
                self.get(row_index).map_or("".to_string(), |x| x.value.to_string()),
                other.get(row_index).map_or("".to_string(), |x| x.value.to_string()),
            ]);
        }
        print_table_locale_opt(&columns, data, locale, precision);
    }

}

impl Deref for TvmSeries{
    type Target = Vec<TvmPeriod>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TvmPeriod {
    pub(crate) fn new(period: u32, rate: f64, value: f64, formula: &str, symbolic_formula: &str) -> Self {
        assert!(rate.is_finite());
        assert!(value.is_finite());
        assert!(!formula.is_empty());
        assert!(!symbolic_formula.is_empty());
        Self {
            period,
            rate,
            value,
            formula: formula.to_string(),
            symbolic_formula: symbolic_formula.to_string()
        }
    }

    /// Returns the period number. The first real period is 1 but there's also a period 0 which
    /// shows the starting conditions.
    pub fn period(&self) -> u32 {
        self.period
    }

    /// Returns the periodic rate for the current period. If the containing struct is a
    /// [`TvmSolution`] every period will have the same rate. If it's a [`TvmSchedule`] each period
    /// may have a different rate.
    pub fn rate(&self) -> f64 {
        self.rate
    }

    /// Returns the value of the investment at the end of the current period.
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Returns a text version of the formula used to calculate the value for the current period.
    /// The formula includes the actual values rather than variable names. For the formula with
    /// variables such as pv for present value call `symbolic_formula`.
    pub fn formula(&self) -> &str {
        &self.formula
    }

    /// Returns a text version of the formula used to calculate the value for the current period.
    /// The formula includes variables such as r for the rate. For the formula with actual values
    /// rather than variables call `formula`.
    pub fn symbolic_formula(&self) -> &str {
        &self.symbolic_formula
    }
}

/*
impl Debug for TvmPeriod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ {}, {}, {}, {}, {} }}",
               &format!("period: {}", self.period),
               &format!("rate: {:.6}", self.rate),
               &format!("value: {:.4}", self.value),
               &format!("formula: {:?}", self.formula),
               &format!("symbolic_formula: {:?}", self.symbolic_formula),
        )
    }
}
*/

fn series_internal(
    calculated_field: TvmVariable,
    continuous_compounding: bool,
    rates: &[f64],
    _fractional_periods: f64,
    present_value: f64,
    future_value: f64,
) -> TvmSeries {
    let periods = rates.len();
    let mut series = vec![];
    if calculated_field.is_present_value() {
        // next_value refers to the value of the period following the current one in the loop.
        let mut next_value = None;

        // Add the values at each period.
        // Start at the last period since we calculate each period's value from the following period,
        // except for the last period which simply has the future value. We'll have a period 0
        // representing the present value.
        for period in (0..=periods).rev() {
            let one_rate = if period == 0 {
                0.0
            } else {
                rates[period - 1]
            };
            assert!(one_rate.is_finite());
            assert!(one_rate >= -1.0);

            // let rate_multiplier = 1.0 + one_rate;

            let (value, formula, symbolic_formula) = if period == periods {
                // This was a present value calculation so we started with a given future value. The
                // value at the end of the last period is simply the future value.
                let value = future_value;
                let formula = format!("{:.4}", value);
                let symbolic_formula = "value = fv";
                (value, formula, symbolic_formula)
            } else {
                // Since this was a present value calculation we started with the future value, that is
                // the value at the end of the last period. Here we're working with some period other
                // than the last period so we calculate this period's value based on the period after
                // it.
                let rate_next_period = rates[period];
                if continuous_compounding {
                    let value = next_value.unwrap() / std::f64::consts::E.powf(rate_next_period);
                    let formula = format!("{:.4} = {:.4} / ({:.6} ^ {:.6})", value, next_value.unwrap(), std::f64::consts::E, rate_next_period);
                    let symbolic_formula = "pv = fv / e^r";
                    (value, formula, symbolic_formula)
                } else {
                    let rate_multiplier_next_period = 1.0 + rate_next_period;
                    let value = next_value.unwrap() / rate_multiplier_next_period;
                    let formula = format!("{:.4} = {:.4} / {:.6}", value, next_value.unwrap(), rate_multiplier_next_period);
                    let symbolic_formula = "value = {next period value} / (1 + r)";
                    (value, formula, symbolic_formula)
                }
            };
            assert!(value.is_finite());
            next_value = Some(value);
            // We want to end up with the periods in order so for each pass through the loop insert the
            // current TvmPeriod at the beginning of the vector.
            series.insert(0, TvmPeriod::new(period as u32, one_rate, value, &formula, symbolic_formula))
        }
    } else {
        // For a rate, periods, or future value calculation the the period-by-period values are
        // calculated the same way, starting with the present value and multiplying the value by
        // (1 + rate) for each period. The only nuance is that if we got here from a periods
        // calculation the last period may not be a full one, so there is some special handling of
        // the formulas and values.

        // For each period after 0, prev_value will hold the value of the previous period.
        let mut prev_value = None;

        // Add the values at each period.
        for period in 0..=periods {
            let one_rate = if period == 0 {
                0.0
            } else {
                rates[period - 1]
            };
            assert!(one_rate.is_finite());
            assert!(one_rate >= -1.0);

            let rate_multiplier = 1.0 + one_rate;
            assert!(rate_multiplier.is_finite());
            assert!(rate_multiplier >= 0.0);

            let (value, formula, symbolic_formula) = if period == 0 {
                let value = present_value;
                let formula = format!("{:.4}", value);
                let symbolic_formula = "value = pv";
                (value, formula, symbolic_formula)
            } else if calculated_field.is_periods() && period == periods {
                // We calculated periods and this may not be a whole number, so for the last
                // period use the future value. If instead we multiplied the previous
                // period's value by (1 + rate) we could overshoot the future value.
                let value = future_value;
                let formula = format!("{:.4}", value);
                let symbolic_formula = "value = fv";
                (value, formula, symbolic_formula)
            } else {
                // The usual case.
                if continuous_compounding {
                    let value = prev_value.unwrap() * std::f64::consts::E.powf(one_rate);
                    let formula = format!("{:.4} = {:.4} * ({:.6} ^ {:.6})", value, prev_value.unwrap(), std::f64::consts::E, one_rate);
                    let symbolic_formula = "fv = pv * e^r";
                    (value, formula, symbolic_formula)
                } else {
                    let value = prev_value.unwrap() * rate_multiplier;
                    let formula = format!("{:.4} = {:.4} * {:.6}", value, prev_value.unwrap(), rate_multiplier);
                    let symbolic_formula = "value = {previous period value} * (1 + r)";
                    (value, formula, symbolic_formula)
                }
            };
            assert!(value.is_finite());
            prev_value = Some(value);
            series.push(TvmPeriod::new(period as u32, one_rate, value, &formula, symbolic_formula))
        }
    }
    TvmSeries::new(series)
}


fn round_fractional_periods(fractional_periods: f64) -> u32 {
    round_4(fractional_periods).ceil() as u32
}
