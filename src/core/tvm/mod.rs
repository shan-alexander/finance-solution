//! The internal module which supports the solution struct for the family of Time-value-of-money equations
//! which do not involve payments. For example, future value, present value, rate, and periods.
//!
//! # Continuous Compounding
//!
//! Most of the functions in this module have an option for continuous compounding. In each case the
//! formula is quite different but the result is similar to using period-by-period compounding with
//! a lot of compounding periods.
//!
//! For example, say we have an investment that starts at $100 and grows at 20% per year for a
//! single year. There's one compounding period so the future value is simply $100 x 1.2 = $120.
//! We could instead compound the interest twice during the year, each time using a rate of
//! 20% / 2 = 10%. That would give a slightly larger future amount, $100 x 1.1 x 1.1 = $121. We
//! could compound the interest quarterly using a rate of 20% / 4 = 5%. Each time we increase the
//! number of compounding periods the future value gets a little higher:
//!
//! > <img src="http://i.upmath.me/svg/%24%24%5Cbegin%7Btikzpicture%7D%5Bscale%3D1.0544%5D%5Csmall%0A%5Cbegin%7Baxis%7D%5Baxis%20line%20style%3Dgray%2C%0A%09samples%3D12%2C%0A%09width%3D9.0cm%2Cheight%3D6.4cm%2C%0A%09xmin%3D0%2C%20xmax%3D12%2C%0A%09ymin%3D119%2C%20ymax%3D123%2C%0A%09restrict%20y%20to%20domain%3D0%3A1000%2C%0A%09ytick%3D%7B120%2C%20121%2C%20122%7D%2C%0A%09xtick%3D%7B1%2C2%2C3%2C4%2C5%2C6%2C7%2C8%2C9%2C10%2C11%2C12%7D%2C%0A%09axis%20x%20line%3Dcenter%2C%0A%09axis%20y%20line%3Dcenter%2C%0A%09xlabel%3D%24n%24%2Cylabel%3D%24fv%24%5D%0A%5Caddplot%5Bblue%2Cdomain%3D1%3A12%2Cthick%2C%20only%20marks%5D%7B100*((1%2B(0.2%2Fx))%5Ex)%7D%3B%0A%5Caddplot%5Bblue%5D%20coordinates%20%7B(4.8%2C120.7)%7D%20node%7B%24fv%3D100(1%2B%7B0.2%20%5Cover%20n%7D)%5En%24%7D%3B%0A%5Cpath%20(axis%20cs%3A0%2C122)%20node%20%5Banchor%3Dnorth%20west%2Cyshift%3D-0.07cm%5D%3B%0A%5Cend%7Baxis%7D%0A%5Cend%7Btikzpicture%7D%24%24" />
//!
//! `n` is the number of compounding periods and `fv` is the future value based on that number of
//! periods. The formula in the chart is the general solution for calculating the future value with
//! any finite number of periods. For instance with one million periods the future value is about
//! $122.140273.
//!
//! The increase in the future value keeps getting smaller so it seems to be approaching some value.
//! No matter how many compounding periods we use there's some limit to the future value. And that's
//! what we can find with continuous compounding. Using continuous compounding rather than a lot of
//! compounding periods this limit turns out to be about $122.140276, very slightly higher than the
//! future value using one million periods. This is the limit that we were approaching by using 1
//! through 12 periods. Here the black line and formula show the result with continuous compounding:
//!
//! > <img src="http://i.upmath.me/svg/%24%24%5Cbegin%7Btikzpicture%7D%5Bscale%3D1.0544%5D%5Csmall%0A%5Cbegin%7Baxis%7D%5Baxis%20line%20style%3Dgray%2C%0A%09samples%3D12%2C%0A%09width%3D9.0cm%2Cheight%3D6.4cm%2C%0A%09xmin%3D0%2C%20xmax%3D12%2C%0A%09ymin%3D119%2C%20ymax%3D123%2C%0A%09restrict%20y%20to%20domain%3D0%3A1000%2C%0A%09ytick%3D%7B120%2C%20121%2C%20122%7D%2C%0A%09xtick%3D%7B1%2C2%2C3%2C4%2C5%2C6%2C7%2C8%2C9%2C10%2C11%2C12%7D%2C%0A%09axis%20x%20line%3Dcenter%2C%0A%09axis%20y%20line%3Dcenter%2C%0A%09xlabel%3D%24n%24%2Cylabel%3D%24fv%24%5D%0A%5Caddplot%5Bblue%2Cdomain%3D1%3A12%2Cthick%2C%20only%20marks%5D%7B100*((1%2B(0.2%2Fx))%5Ex)%7D%3B%0A%5Caddplot%5Bblack%2Cdomain%3D1%3A12%2Cthick%5D%7B100*(e%5E(0.2))%7D%3B%0A%5Caddplot%5B%5D%20coordinates%20%7B(3.7%2C122.4)%7D%20node%7B%24fv%3D100e%5E%7B0.2%7D%24%5Capprox122.14%24%7D%3B%0A%5Caddplot%5Bblue%5D%20coordinates%20%7B(4.8%2C120.7)%7D%20node%7B%24fv%3D100(1%2B%7B0.2%20%5Cover%20n%7D)%5En%24%7D%3B%0A%5Cpath%20(axis%20cs%3A0%2C122)%20node%20%5Banchor%3Dnorth%20west%2Cyshift%3D-0.07cm%5D%3B%0A%5Cend%7Baxis%7D%0A%5Cend%7Btikzpicture%7D%24%24" />
//!
//! When we started with a fixed rate and present value and increased the number of compounding
//! periods, the future value rose each time. The converse would be starting with a fixed future
//! value and seeing how the present value changes by increasing the compounding periods.
//!
//! Suppose we have the same 20% rate but this time it's the _future value_ that's $100.
//! Compounding only once the calculation for the present value is $100 / 1.2 ≈ $83.33. As we
//! compound the interest more frequently we need a slightly smaller present value to reach the same
//! future value of $100 in one year. With more frequent compounding the required present value
//! approaches $81.87, the value needed with continuous compounding that's shown here with a black
//! line and formula:
//!
//! > <img src="http://i.upmath.me/svg/%24%24%5Cbegin%7Btikzpicture%7D%5Bscale%3D1.0544%5D%0A%5Cbegin%7Baxis%7D%5Baxis%20line%20style%3Dgray%2C%0A%09samples%3D12%2C%0A%09width%3D9.0cm%2Cheight%3D6.4cm%2C%0A%09xmin%3D0%2C%20xmax%3D12%2C%0A%09ymin%3D80.5%2C%20ymax%3D84.5%2C%0A%09restrict%20y%20to%20domain%3D0%3A1000%2C%0A%09ytick%3D%7B81%2C%2082%2C%2083%2C%2084%7D%2C%0A%09xtick%3D%7B1%2C2%2C3%2C4%2C5%2C6%2C7%2C8%2C9%2C10%2C11%2C12%7D%2C%0A%09axis%20x%20line%3Dcenter%2C%0A%09axis%20y%20line%3Dcenter%2C%0A%09xlabel%3D%24n%24%2Cylabel%3D%24pv%24%5D%0A%5Caddplot%5Bblue%2Cdomain%3D1%3A12%2Csemithick%2C%20only%20marks%5D%7B100%2F((1%2B(0.2%2Fx))%5Ex)%7D%3B%0A%5Caddplot%5Bblack%2Cdomain%3D1%3A12%2C%20thick%5D%7B100%2F(e%5E(0.2))%7D%3B%0A%5Caddplot%5B%5D%20coordinates%20%7B(3.3%2C81.53)%7D%20node%7B%24pv%3D%7B100%20%5Cover%20e%5E%7B0.2%7D%7D%5Capprox81.87%24%7D%3B%0A%5Caddplot%5Bblue%5D%20coordinates%20%7B(4.5%2C82.8)%7D%20node%7B%24pv%3D%7B100%20%5Cover%20(1%2B%7B0.2%20%5Cover%20n%7D)%5En%7D%24%7D%3B%0A%5Cpath%20(axis%20cs%3A0%2C83)%20node%20%5Banchor%3Dnorth%20west%2Cyshift%3D-0.07cm%5D%3B%0A%5Cend%7Baxis%7D%0A%5Cend%7Btikzpicture%7D%24%24" />
//!
//! It's fine for a calculation to specify more than one period and continuous compounding at the
//! same time. All this means is that the interest for each period is done with continuous
//! compounding. It works out to the same answer as if there were only one period with the rate
//! adjusted accordingly. For instance these three calculations produce the same future value:
//! ```
//! use finance_solution::core::*;
//!
//! let present_value = 100;
//! let continuous = true;
//!
//! // 1% per month compounded over twelve months.
//! let fv_12_months = future_value(0.01, 12, present_value, continuous);
//!
//! // 12% applied only once at the end of the year.
//! let fv_1_year = future_value(0.12, 1, present_value, continuous);
//! dbg!(fv_12_months, fv_1_year);
//! assert_eq!(fv_12_months, fv_1_year);
//!
//! // 3% per quarter compounded over four quarters.
//! let fv_4_quarters = future_value(0.03, 4, present_value, continuous);
//! dbg!(fv_4_quarters);
//! assert_eq!(fv_12_months, fv_4_quarters);
//! ```
//! This works only with continuous compounding. With period-by-period compounding the future value
//! for four quarters would be higher than that for one year and the value for twelve months would
//! be higher than that.
//!
//! The library has a convenient way to check multiple what-if scenarios with different numbers of
//! compounding periods and with continuous compounding. See
//! [future_value_vary_periods](struct.TvmSolution.html#method.future_value_vary_periods)
//! and [present_value_vary_periods](struct.TvmSolution.html#method.present_value_vary_periods).

//#![warn(missing_docs)]

use std::ops::Deref;
use std::fmt::{Display, Formatter, Error};

use crate::*;

pub mod future_value;
// #[doc(inline)]
pub use future_value::*;

pub mod present_value;
// #[doc(inline)]
pub use present_value::*;

pub mod periods;
// #[doc(inline)]
pub use periods::*;

pub mod rate;
// #[doc(inline)]
pub use rate::*;

const CALL_INVARIANT: bool = true;

/// Enumeration used in [TvmSolution](struct.TvmSolution.html) and
/// [TvmScheduleSolution](struct.TvmScheduleSolution.html) to keep track of which value was
/// calculated, either the periodic rate, the number of periods, the present value, or the future
/// value.
///
/// It can be checked with [TvmSolution::calculated_field](struct.TvmSolution.html#method.calculated_field)
/// or [TvmScheduleSolution::calculated_field](struct.TvmScheduleSolution.html#method.calculated_field).
#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub enum TvmVariable {
    Rate,
    Periods,
    PresentValue,
    FutureValue,
}

/// **A record of a Time Value of Money calculation** using a fixed rate.
///
/// It's produced by calling [rate_solution](rate/fn.rate_solution.html),
/// [periods_solution](periods/fn.periods_solution.html),
/// [present_value_solution](present_value/fn.present_value_solution.html), or
/// [future_value_solution](future_value/fn.future_value_solution.html).
#[derive(Clone, Debug)]
pub struct TvmSolution {
    calculated_field: TvmVariable,
    calculation_type: CalculationType,
    continuous_compounding: bool,
    rate: f64,
    periods: u32,
    fractional_periods: f64,
    present_value: f64,
    future_value: f64,
    formula: String,
    symbolic_formula: String,
}

/// **A record of a Time Value of Money calculation** where the rate may vary by period.
///
/// It's the result of calling [future_value_schedule_solution](future_value/fn.future_value_schedule_solution.html)
/// or [present_value_schedule_solution](present_value/fn.present_value_schedule_solution.html)
#[derive(Clone, Debug)]
pub struct TvmScheduleSolution {
    calculated_field: TvmVariable,
    rates: Vec<f64>,
    periods: u32,
    present_value: f64,
    future_value: f64,
}

/// **The period-by-period details** of a Time Value of Money calculation.
///
/// It's produced by calling [TvmSolution::series](struct.TvmSolution.html#method.series) or
/// [TvmScheduleSolution::series](struct.TvmScheduleSolution.html#method.series).
#[derive(Clone, Debug)]
pub struct TvmSeries(Vec<TvmPeriod>);

/// **The value of an investment at the end of a given period**, part of a Time Value of Money
/// calculation.
#[derive(Clone, Debug)]
pub struct TvmPeriod {
    period: u32,
    rate: f64,
    value: f64,
    formula: String,
    symbolic_formula: String,
}

impl TvmVariable {
    /// Returns true if the variant is `TvmVariable::Rate` indicating that the periodic rate was
    /// calculated from the number of periods, the present value, and the future value.
    pub fn is_rate(&self) -> bool {
        match self {
            TvmVariable::Rate => true,
            _ => false,
        }
    }

    /// Returns true if the variant is `TvmVariable::Periods` indicating that the number of periods
    /// was calculated from the periocic rate, the present value, and the future value.
    pub fn is_periods(&self) -> bool {
        match self {
            TvmVariable::Periods => true,
            _ => false,
        }
    }

    /// Returns true if the variant is `TvmVariable::PresentValue` indicating that the present value
    /// was calculated from one or more periocic rates, the number of periods, and the future value.
    pub fn is_present_value(&self) -> bool {
        match self {
            TvmVariable::PresentValue => true,
            _ => false,
        }
    }

    /// Returns true if the variant is `TvmVariable::FutureValue` indicating that the future value
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

    /*
    pub(crate) fn precision(&self) -> usize {
        match self {
            TvmVariable::Periods => 0,
            TvmVariable::Rate => 6,
            _ => 4,
        }
    }
    */

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
    pub(crate) fn new(calculated_field: TvmVariable, calculation_type: CalculationType, continuous_compounding: bool, rate: f64, periods: u32, present_value: f64, future_value: f64, formula: &str, symbolic_formula: &str) -> Self {
        assert!(rate.is_finite());
        assert!(present_value.is_finite());
        assert!(future_value.is_finite());
        assert!(!formula.is_empty());
        assert!(!symbolic_formula.is_empty());
        let solution = Self::new_fractional_periods(calculated_field, calculation_type, continuous_compounding, rate, periods as f64, present_value, future_value, formula, symbolic_formula);
        if CALL_INVARIANT {
            solution.invariant();
        }
        solution
    }

    pub(crate) fn new_fractional_periods(calculated_field: TvmVariable, calculation_type: CalculationType, continuous_compounding: bool, rate: f64, fractional_periods: f64, present_value: f64, future_value: f64, formula: &str, symbolic_formula: &str) -> Self {
        assert!(rate >= -1.0);
        assert!(fractional_periods >= 0.0);
        assert!(present_value.is_finite());
        assert!(future_value.is_finite());
        assert!(formula.len() > 0);
        assert!(symbolic_formula.len() > 0);
        Self {
            calculated_field,
            calculation_type,
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
    /// Calculate a future value using [future_value_solution](future_value/fn.future_value_solution.html) then
    /// view the period-by-period details.
    /// ```
    /// // The initial investment is $10,000.12, the interest rate is 1.5% per month, and the
    /// // investment will grow for 24 months using simple compounding.
    /// let solution = finance_solution::core::future_value_solution(0.015, 24, 10_000.12, false);
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
    /// let filtered_series = series.filter(|x| x.period() % 4 == 0);
    /// dbg!(&filtered_series);
    /// assert_eq!(7, filtered_series.len());
    /// ```
    pub fn series(&self) -> TvmSeries {
        let rates = initialized_vector(self.periods as usize, self.rate);
        series_internal(self.calculated_field.clone(), self.continuous_compounding, &rates, self.fractional_periods, self.present_value, self.future_value)
    }

    /// Prints a formatted table with the period-by-period details of a Time Value of Money
    /// calculation.
    ///
    /// Money amounts are rounded to four decimal places, rates to six places, and numbers are
    /// formatted similar to Rust constants such as "10_000.0322". For more control over formatting
    /// use [print_table_locale](#method.print_table_locale).
    ///
    /// # Examples
    /// ```
    /// finance_solution::core::future_value_solution(0.045, 5, 10_000, false)
    ///     .print_table();
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
    pub fn print_table(&self) {
        self.series().print_table();
    }

    /// Prints a formatted table with the period-by-period details of a time-value-of-money
    /// calculation, with options for formatting numbers.
    ///
    /// For a simpler method that doesn't require a locale use
    /// [print_table](#method.print_table).
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
    /// let locale = finance_solution::core::num_format::Locale::en;
    ///
    /// // Show money amounts to two decimal places.
    /// let precision = 2;
    ///
    /// finance_solution::core::future_value_solution(0.11, 4, 5_000, false)
    ///     .print_table_locale(&locale, precision);
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
    pub fn print_table_locale(&self, locale: &num_format::Locale, precision: usize) {
        self.series().print_table_locale(locale, precision);
    }

    /// Returns a variant of [TvmVariable](enum.TvmVariable.html) showing which value was calculated, either the periodic
    /// rate, number of periods, present value, or future value. To test for the enum variant use
    /// functions like [TvmVariable::is_rate](enum.TvmVariable.html#method.is_rate).
    ///
    /// # Examples
    /// ```
    /// // Calculate the future value of $25,000 that grows at 5% for 12 yeors.
    /// let solution = finance_solution::core::future_value_solution(0.05, 12, 25_000, false);
    /// assert!(solution.calculated_field().is_future_value());
    /// ```
    pub fn calculated_field(&self) -> &TvmVariable {
        &self.calculated_field
    }

    /// Returns the API that was used to make the calculation, either [core](../../core/index.html),
    /// [academic](../../academic/index.html), or [excel](../../excel/index.html). To test for the
    /// enum variant use functions like
    /// [CalculationType::is_academic](../../enum.CalculationType.html#method.is_academic).
    pub fn calculation_type(&self) -> &CalculationType {
        &self.calculation_type
    }

    /// Returns true if the interest was compounded continuously rather than period-by-period.
    pub fn continuous_compounding(&self) -> bool {
        self.continuous_compounding
    }

    /// Returns the periodic rate which is a calculated value if this `TvmSolution` struct is the
    /// result of a call to [rate_solution](rate/fn.rate_solution.html) and otherwise is one of the
    /// input values.
    pub fn rate(&self) -> f64 {
        self.rate
    }

    /// Returns the number of periods as a whole number. This is a calculated value if this
    /// `TvmSolution` struct is the result of a call to
    /// [periods_solution](periods/fn.periods_solution.html) and otherwise it's one of the input values.
    /// If the value was calculated the true result may not have been a whole number so this is that
    /// number rounded away from zero.
    pub fn periods(&self) -> u32 {
        self.periods
    }

    /// Returns the number of periods as a floating point number. This is a calculated value if this
    /// `TvmSolution` struct is the result of a call to
    /// [periods_solution](periods/fn.periods_solution.html) and otherwise it's one of the input values.
    pub fn fractional_periods(&self) -> f64 {
        self.fractional_periods
    }

    /// Returns the present value which is a calculated value if this `TvmSolution` struct is the
    /// result of a call to [present_value_solution](present_value/fn.present_value_solution.html) and otherwise
    /// is one of the input values.
    pub fn present_value(&self) -> f64 {
        self.present_value
    }

    /// Returns the future value which is a calculated value if this `TvmSolution` struct is the
    /// result of a call to [future_value_solution](future_value/fn.future_value_solution.html) and otherwise
    /// is one of the input values.
    pub fn future_value(&self) -> f64 {
        self.future_value
    }

    /// Returns a text version of the formula used to calculate the result which may have been the
    /// periodic rate, number of periods, present value, or future value depending on which function
    /// was called. The formula includes the actual values rather than variable names. For the
    /// formula with variables such as r for rate call [symbolic_formula](#method.symbolic_formula).
    pub fn formula(&self) -> &str {
        &self.formula
    }

    /// Returns a text version of the formula used to calculate the result which may have been the
    /// periodic rate, number of periods, present value, or future value depending on which function
    /// was called. The formula uses variables such as "n" for the number of periods. For the
    /// formula with the actual numbers rather than variables call
    /// [formula](#method.formula).
    pub fn symbolic_formula(&self) -> &str {
        &self.symbolic_formula
    }

    /// Returns a new calculation where we start with the current calculation and solve for the
    /// periodic rate leaving the periods, present value, and future value constant; optionally
    /// switching between normal and continuous compounding or changing the number of compounding periods.
    ///
    /// This works for any Time Value of Money result, not only those that calculated a periodic
    /// rate. For instance, we can call [present_value_solution](present_value/fn.present_value_solution.html) to
    /// calculate a present value then call this method to calculate a new rate while changing to
    /// continuous compounding.
    ///
    /// # Arguments
    /// * `continuous_compounding` - If true, use continuous compounding. Otherwise use
    /// period-by-period compounding.
    /// * `compounding_periods` - If None, use the periods from the original calculation. If given a
    /// value like `Some(12)` change the number of compounding periods while keeping everything else
    /// the same. The periodic rate from the original calculation will be adjusted.
    ///
    /// # Examples
    /// For an example of changing the compounding periods see
    /// [future_value_solution](#method.future_value_solution).
    ///
    /// Calculate a future value then use that as a basis for calculating a new rate by switching
    /// to continuous compounding.
    /// ```
    /// use finance_solution::core::*;
    ///
    /// // First calculate a future value given the other three inputs. $10,000 grows at 10% per year
    /// // for 12 years using period-by-period compounding.
    /// let solution_fv = future_value_solution(0.1, 12, -10_000, false);
    /// dbg!(&solution_fv);
    /// assert_rounded_2!(31_384.28, solution_fv.future_value());
    ///
    /// // Find out what the rate would have to be to get the same results with continuous compounding
    /// // instead of compounding once per year.
    /// let continuous = true;
    /// let compounding_periods = None; // Don't change the periods.
    /// let solution_continuous = solution_fv.rate_solution(continuous, compounding_periods);
    /// dbg!(&solution_continuous);
    ///
    /// // Compare the two calculations.
    /// solution_fv.print_ab_comparison(&solution_continuous, false);
    /// ```
    /// The output of that last statement is below. Note that:
    /// * The calculated field changed from future value to rate.
    /// * Only the second calculation used continuous compounding.
    /// * The rate is slightly smaller in the second case which we'd expect because we switched
    /// to continuous compounding while keeping the same end result.
    /// * The periods, present value, and future value haven't changed.
    /// * The formulas are quite different because we're calculating different values.
    /// ```text
    /// calculated_field a: Future Value
    /// calculated_field b: Rate
    /// calculation_type: Core
    /// continuous_compounding a: false
    /// continuous_compounding b: true
    /// rate a: 0.100000
    /// rate b: 0.095310
    /// periods: 12
    /// present_value: -10_000.0000
    /// future_value: 31_384.2838
    /// formula a: 31384.2838 = 10000.0000 * (1.100000 ^ 12)
    /// formula b: 0.095310 = ln(-31384.2838 / -10000.0000) / 12
    /// symbolic_formula a: fv = -pv * (1 + r)^n
    /// symbolic_formula b: r = ln(-fv / pv) / t
    /// ```
    pub fn rate_solution(&self, continuous_compounding: bool, compounding_periods: Option<u32>) -> TvmSolution {
        let periods= compounding_periods.unwrap_or(self.periods);
        rate_solution_internal(periods, self.present_value, self.future_value, continuous_compounding)
    }

    /// Returns a new calculation where we start with the current calculation and solve for the
    /// number of periods leaving the rate, present value, and future value constant; optionally switching
    /// between normal and continuous compounding.
    ///
    /// This works for any Time Value of Money result, not only those that calculated the number of
    /// periods.
    ///
    /// # Arguments
    /// * `continuous_compounding` - If true, use continuous compounding. Otherwise use
    /// period-by-period compounding.
    ///
    /// # Examples
    /// For an example of switching to continuous compounding see
    /// [rate_solution](#method.rate_solution). The only difference is that
    /// the current method doesn't have a `compounding_periods` argument.
    pub fn periods_solution(&self, continuous_compounding: bool) -> TvmSolution {
        periods_solution_internal(self.rate, self.present_value, self.future_value, continuous_compounding)
    }

    /// Returns a new calculation where we start with the current calculation and solve for the
    /// present value leaving the rate, periods, and future value constant; optionally switching
    /// between normal and continuous compounding or changing the number of compounding periods.
    ///
    /// This works for any Time Value of Money result, not only those that calculated a present
    /// value.
    ///
    /// If the goal is to compare present values given several different compounding periods, that
    /// can be done in one step with
    /// [present_value_vary_periods](#method.present_value_vary_periods).
    ///
    /// # Arguments
    /// * `continuous_compounding` - If true, use continuous compounding. Otherwise use
    /// period-by-period compounding.
    /// * `compounding_periods` - If None, use the periods from the original calculation. If given a
    /// value like `Some(12)` change the number of compounding periods while keeping everything else
    /// the same. The periodic rate from the original calculation will be adjusted.
    ///
    /// # Examples
    /// For an example of switching to continuous compounding see
    /// [rate_solution](#method.rate_solution).
    ///
    /// For an example of changing the compounding periods see
    /// [future_value_solution](#method.future_value_solution).
    pub fn present_value_solution(&self, continuous_compounding: bool, compounding_periods: Option<u32>) -> TvmSolution {
        let (rate, periods) = match compounding_periods {
            Some(periods) => ((self.rate * self.fractional_periods) / periods as f64, periods as f64),
            None => (self.rate, self.fractional_periods),
        };
        present_value_solution_internal(rate, periods, self.future_value, continuous_compounding)
    }

    /// Returns a new calculation where we start with the current calculation and solve for the
    /// future value leaving the rate, periods, and present value constant; optionally switching
    /// between normal and continuous compounding or changing the number of compounding periods.
    ///
    /// This works for any Time Value of Money result, not only those that calculated a future
    /// value.
    ///
    /// If the goal is to compare future values given several different compounding periods, that
    /// can be done in one step with
    /// [future_value_vary_periods](#method.future_value_vary_periods).
    ///
    /// # Arguments
    /// * `continuous_compounding` - If true, use continuous compounding. Otherwise use
    /// period-by-period compounding.
    /// * `compounding_periods` - If None, use the periods from the original calculation. If given a
    /// value like `Some(12)` change the number of compounding periods while keeping everything else
    /// the same. The periodic rate from the original calculation will be adjusted.
    ///
    /// # Examples
    /// For an example of switching to continuous compounding see
    /// [rate_solution](#method.rate_solution).
    ///
    /// Calculate a future value then see how that would change with more frequent compounding.
    /// ```
    /// use finance_solution::core::*;
    ///
    /// // First calculate a future value given the other three inputs. $10,000 grows at 10% per year
    /// // for 12 years using year-by-year compounding.
    /// let years = 12;
    /// let solution_annual = future_value_solution(0.1, years, -10_000, false);
    /// dbg!(&solution_annual);
    /// assert_rounded_2!(31_384.28, solution_annual.future_value());
    ///
    /// // Calculate what the future value would be if we compounded quarterly instead of annualy. We
    /// // could have started with something other than a future value calculation such as present value
    /// // or periods, but for this example we'll calculate the future value both times.
    /// let continuous = false;
    /// let compounding_periods = Some(years * 4);
    /// let solution_quarterly = solution_annual.future_value_solution(continuous, compounding_periods);
    /// dbg!(&solution_quarterly);
    ///
    /// // The rate was automatically changed to 2.5% (10% / 4) because each period is now a quarter
    /// // instead of a year. Had we used the original rate the future value would have been
    /// // completely different and the calculation would not have made sense.
    /// assert_rounded_4!(0.0250, solution_quarterly.rate());
    ///
    /// // The future value is slightly higher as we'd expect from increasing the frequency of
    /// // compounding while holding everything else constant.
    /// assert_rounded_2!(32_714.90, solution_quarterly.future_value());
    ///
    /// // Compare the two calculations.
    /// solution_annual.print_ab_comparison(&solution_quarterly, false);
    /// ```
    /// The output of that last statement is below. Note that the rate in the second case is one
    /// fourth of that in the first case while the number of periods has quadrupled.
    /// ```text
    /// calculated_field: Future Value
    /// calculation_type: Core
    /// continuous_compounding: false
    /// rate a: 0.100000
    /// rate b: 0.025000
    /// periods a: 12
    /// periods b: 48
    /// present_value: -10_000.0000
    /// future_value a: 31_384.2838
    /// future_value b: 32_714.8956
    /// formula a: 31384.2838 = 10000.0000 * (1.100000 ^ 12)
    /// formula b: 32714.8956 = 10000.0000 * (1.025000 ^ 48)
    /// symbolic_formula: fv = -pv * (1 + r)^n
    /// ```
    pub fn future_value_solution(&self, continuous_compounding: bool, compounding_periods: Option<u32>) -> TvmSolution {
        let (rate, periods) = match compounding_periods {
            Some(periods) => ((self.rate * self.fractional_periods) / periods as f64, periods as f64),
            None => (self.rate, self.fractional_periods),
        };
        future_value_solution_internal(rate, periods, self.present_value, continuous_compounding)
    }

    /// Returns a struct with a set of what-if scenarios for the present value needed with a variety
    /// of compounding periods, with an option for continuous compounding.
    ///
    /// For an overview of the effects of increasing the compounding periods or using continuous
    /// compounding see [Continuous Compounding](index.html#continuous-compounding). The last
    /// graph in that section use the same setup as the example below.
    ///
    /// # Arguments
    /// * `compounding_periods` - The compounding periods to include in the scenarios. The result
    /// will have a computed present value for each compounding period in this list.
    /// * `include_continuous_compounding` - If true, adds one scenario at the end of the results
    /// with continuous compounding instead of a given number of compounding periods.
    ///
    /// # Examples
    /// For a more detailed example with a related function see
    /// [future_value_vary_periods](#method.future_value_vary_periods)
    /// ```
    /// // Calculate the future value of an investment that starts at $83.33 and grows 20% in one
    /// // year using simple compounding. Note that we're going to examine how the present value
    /// // varies by the number of compounding periods but we're starting with a future value
    /// // calculation. It would have been fine to start with a rate, periods, or present value
    /// // calculation as well. It just depends on what information we have to work with.
    /// let solution = finance_solution::core::future_value_solution(0.20, 1, -83.333, false);
    /// dbg!(&solution);
    ///
    /// // The present value of $83.33 gives us a future value of about $100.00.
    /// finance_solution::core::assert_rounded_2!(100.00, solution.future_value());
    ///
    /// // We'll experiment with compounding annually, quarterly, monthly, weekly, and daily.
    /// let compounding_periods = [1, 4, 12, 52, 365];
    ///
    /// // Add a final scenario with continuous compounding.
    /// let include_continuous_compounding = true;
    ///
    /// // Compile a list of the present values needed to arrive at the calculated future value of $100
    /// // each of the above compounding periods as well a continous compounding.
    /// let scenarios = solution.present_value_vary_periods(&compounding_periods, include_continuous_compounding);
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
    pub fn present_value_vary_periods(&self, compounding_periods: &[u32], include_continuous_compounding: bool) -> ScenarioList {
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
    /// a variety of compounding periods, with an option for continuous compounding.
    ///
    /// For an overview of the effects of increasing the compounding periods or using continuous
    /// compounding see [Continuous Compounding](index.html#continuous-compounding). The first two
    /// graphs in that section use the same setup as the example below.
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
    /// let solution = finance_solution::core::future_value_solution(rate, periods, present_value, continuous_compounding);
    ///
    /// // We'll experiment with compounding annually, quarterly, monthly, weekly, and daily.
    /// let compounding_periods = [1, 4, 12, 52, 365];
    ///
    /// // Add a final scenario with continuous compounding.
    /// let include_continuous_compounding = true;
    ///
    /// // Compile a list of the future values with each of the above compounding periods as well as
    /// // continous compounding.
    /// let scenarios = solution.future_value_vary_periods(&compounding_periods, include_continuous_compounding);
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
    pub fn future_value_vary_periods(&self, compounding_periods: &[u32], include_continuous_compounding: bool) -> ScenarioList {
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

    /// Compares the results of two Time Value of Money calculations, such as from two calls to
    /// [future_value_solution](future_value/fn.future_value_solution.html) with different periodic rates.
    ///
    /// It's fine to compare calculations that solved for different variables such as a rate
    /// calculation vs. a present value calculation.
    ///
    /// The first solution is labeled "a" and the second is "b". Money amounts are rounded to four
    /// decimal places, rates to six places, and numbers are formatted similar to Rust constants
    /// such as "10_000.0322". For more control over formatting use
    /// [print_ab_comparison_locale](#method.print_ab_comparison_locale).
    ///
    /// # Arguments
    /// * `other` - The second `TvmSolution` in the comparison which will be labeled "b".
    /// * `include_period_detail` - If true, print the month-by-month details of both calculations.
    ///
    /// # Examples
    /// See [future_value_solution](#method.future_value_solution).
    /// The last line of the example calls this method and the text output is shown right after
    /// that.
    pub fn print_ab_comparison(&self, other: &TvmSolution, include_period_detail: bool)
    {
        self.print_ab_comparison_locale_opt(other, include_period_detail, None, None);
    }

    /// Compares the results of two Time Value of Money calculations, such as from two calls to
    /// [rate_solution](rate/fn.rate_solution.html) with different numbers of periods. The method has
    /// options for formatting numbers.
    ///
    /// It's fine to compare calculations that solved for different variables such as a rate
    /// calculation vs. a present value calculation.
    ///
    /// The first solution is labeled "a" and the second is "b".
    ///
    /// For a simpler method that doesn't require locale information use
    /// [print_ab_comparison](#method.print_ab_comparison).
    ///
    /// # Arguments
    /// * `other` - The second `TvmSolution` in the comparison which will be labeled "b".
    /// * `include_period_detail` - If true, print the month-by-month details of both calculations.
    /// * `locale` - A variant of the num_format::Locale enum which determines the characters used
    /// for thousands separators and the decimal separator
    /// * `precision` - The number of decimal places. Rates always appear with at least six places
    /// regardless of this value.
    ///
    /// # Examples
    /// ```
    /// use finance_solution::core::*;
    ///
    /// // Two future value calculations that are the same except for the interest rate.
    /// let rate_a = 0.021;
    /// let rate_b = 0.023;
    /// let periods = 60;
    /// let present_value = -10_000;
    /// let continuous = false;
    /// let solution_a = future_value_solution(rate_a, periods, present_value, continuous);
    /// let solution_b = future_value_solution(rate_b, periods, present_value, continuous);
    ///
    /// // Compare the two calculations using English number formatting and two decimal places.
    /// let include_period_detail = true;
    /// let locale = num_format::Locale::en;
    /// let precision = 2;
    /// solution_a.print_ab_comparison_locale(&solution_b, include_period_detail, &locale, precision);
    /// ```
    /// Output (with only the first few periods shown):
    /// ```text
    /// calculated_field: Future Value
    /// calculation_type: Core
    /// continuous_compounding: false
    /// rate a: 0.021000
    /// rate b: 0.023000
    /// periods: 60
    /// present_value: -10,000.00
    /// future_value a: 34,797.22
    /// future_value b: 39,132.54
    /// formula a: 34797.2181 = 10000.0000 * (1.021000 ^ 60)
    /// formula b: 39132.5386 = 10000.0000 * (1.023000 ^ 60)
    /// symbolic_formula: fv = -pv * (1 + r)^n
    ///
    /// period    rate_a    rate_b    value_a    value_b
    /// ------  --------  --------  ---------  ---------
    ///      0  0.000000  0.000000  10,000.00  10,000.00
    ///      1  0.021000  0.023000  10,209.00  10,230.00
    ///      2  0.021000  0.023000  10,424.41  10,465.29
    ///      3  0.021000  0.023000  10,643.32  10,705.99
    /// ```
    pub fn print_ab_comparison_locale(
        &self,
        other: &TvmSolution,
        include_period_detail: bool,
        locale: &num_format::Locale,
        precision: usize)
    {
        self.print_ab_comparison_locale_opt(other, include_period_detail,Some(locale), Some(precision));
    }

    fn print_ab_comparison_locale_opt(
        &self,
        other: &TvmSolution,
        include_period_detail: bool,
        locale: Option<&num_format::Locale>,
        precision: Option<usize>)
    {
        println!();
        print_ab_comparison_values_string("calculated_field", &self.calculated_field.to_string(), &other.calculated_field.to_string());
        print_ab_comparison_values_string("calculation_type", &self.calculation_type.to_string(), &other.calculation_type.to_string());
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

        if include_period_detail {
            self.series().print_ab_comparison_locale_opt(&other.series(), locale, precision);
        }
    }

    pub(crate) fn invariant(&self) {
        assert!(self.rate.is_finite());
        assert!(self.fractional_periods.is_finite());
        assert_eq!(self.periods, round_fractional_periods(self.fractional_periods));
        assert!(self.present_value.is_finite());
        assert!(self.future_value.is_finite());
        assert!(!self.formula.is_empty());
        assert!(!self.symbolic_formula.is_empty());
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
        let solution = Self {
            calculated_field,
            rates: rates.to_vec(),
            periods: rates.len() as u32,
            present_value,
            future_value,
        };
        if CALL_INVARIANT {
            solution.invariant();
        }
        solution
    }

    /// Returns a variant of [TvmVariable](enum.TvmVariable.html) showing which value was
    /// calculated, either the present value or future value. To test for the enum variant use
    /// functions like [TvmVariable::is_rate](enum.TvmVariable.html#method.is_rate).
    ///
    /// # Examples
    /// ```
    /// let solution = finance_solution::core::present_value_schedule_solution(&[0.011, 0.012, 0.009], 75_000);
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
    /// let solution = finance_solution::core::future_value_schedule_solution(&[0.05, 0.07, 0.05], 100_000);
    /// assert_eq!(3, solution.periods());
    /// ```
    pub fn periods(&self) -> u32 {
        self.periods
    }

    /// Returns the present value which is a calculated value if this `TvmScheduleSolution` struct
    /// is the result of a call to
    /// [present_value_schedule_solution](present_value/fn.present_value_schedule_solution.html)
    /// and otherwise is one of the input values.
    pub fn present_value(&self) -> f64 {
        self.present_value
    }

    /// Returns the future value which is a calculated value if this `TvmScheduleSolution` struct
    /// is the result of a call to
    /// [future_value_schedule_solution](future_value/fn.future_value_schedule_solution.html)
    /// and otherwise is one of the input values.
    pub fn future_value(&self) -> f64 {
        self.future_value
    }

    /// Calculates the period-by-period details of a Time Value of Money calculation with rates that
    /// can vary by period.
    ///
    /// # Examples
    /// Print the period-by-period details of a future value calculation. Uses
    /// [future_value_schedule_solution](future_value/fn.future_value_schedule_solution.html).
    /// ```
    /// use finance_solution::core::*;
    ///
    /// // Each period has it's own rate which is why we're calling one of the "schedule" functions.
    /// let rates = [0.05, 0.07, 0.065, 0.07];
    ///
    /// // The initial investment is $10,000.
    /// let present_value = -10_000;
    ///
    /// let solution = future_value_schedule_solution(&rates, present_value);
    /// dbg!(&solution);
    ///
    /// // Calculate the period-by-period details.
    /// let series = solution.series();
    /// dbg!(&series);
    ///
    /// // Confirm that we have one entry for the initial value and one entry for each period.
    /// assert_eq!(5, series.len());
    ///
    /// // Print the period-by-period numbers in a formatted table.
    /// series.print_table_locale(&num_format::Locale::en, 2);
    /// ```
    /// Output from the last line:
    /// ```text
    /// period      rate      value
    /// ------  --------  ---------
    ///      0  0.000000  10,000.00
    ///      1  0.050000  10,500.00
    ///      2  0.070000  11,235.00
    ///      3  0.065000  11,965.27
    ///      4  0.070000  12,802.84
    /// ```
    pub fn series(&self) -> TvmSeries {
        series_internal(self.calculated_field.clone(), false, &self.rates,0.0, self.present_value, self.future_value)
    }

    pub(crate) fn invariant(&self) {
        for rate in self.rates.iter() {
            assert!(rate.is_finite());
        }
        assert!(self.present_value.is_finite());
        assert!(self.future_value.is_finite());
    }
}

impl TvmSeries {
    pub(crate) fn new(series: Vec<TvmPeriod>) -> Self {
        Self {
            0: series,
        }
    }

    /// Produces a series with a subset of the entries from the current series. This is intended to
    /// be used to help with examining and troubleshooting calculations since the resulting filtered
    /// series wouldn't make much sense on its own.
    ///
    /// # Arguments
    /// * `predicate` - A function that takes a reference to a [TvmPeriod](struct.TvmPeriod.html)
    /// and returns a boolean. For instance the closure `|entry| entry.rate() < 0.0` would return
    /// `true` for all entries with a negative rate, and those entries would be included in the new
    /// series.
    ///
    /// # Examples
    /// The example for [series](struct.TvmSolution.html#method.series) ends with a call to this
    /// method.
    ///
    pub fn filter<P>(&self, predicate: P) -> Self
        where P: Fn(&&TvmPeriod) -> bool
    {
        Self {
            0: self.iter().filter(|x| predicate(x)).cloned().collect()
        }
    }

    /// Prints a formatted table with the period-by-period details.
    ///
    /// Money amounts are rounded to four decimal places, rates to six places, and numbers are
    /// formatted similar to Rust constants such as "10_000.0322". For more control over formatting
    /// use [print_table_locale](#method.print_table_locale).
    ///
    /// # Examples
    /// ```
    /// finance_solution::core::future_value_solution(0.045, 5, 10_000, false)
    ///     .series()
    ///     .print_table();
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
    /// Note that since [TvmSolution](struct.TvmSolution.html) has its own
    /// [print_table](struct.TvmSolution.html#method.print_table) method this would produce the same
    /// results without doing the interim step of creating the series:
    /// ```
    /// finance_solution::core::future_value_solution(0.045, 5, 10_000, false)
    ///     .print_table();
    /// ```
    /// So the only reason to use this method is if the series has been changed in some way such as
    /// being filtered or summarized.
    pub fn print_table(&self) {
        self.print_table_locale_opt(None, None);
    }

    /// Prints a formatted table with the period-by-period details and options for formatting
    /// numbers.
    ///
    /// For a simpler method that doesn't require a locale use [print_table](#method.print_table).
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
    /// use finance_solution::core::*;
    ///
    /// // English formatting with "," for the thousands separator and "." for the decimal
    /// // separator.
    /// let locale = num_format::Locale::en;
    ///
    /// // Show money amounts to two decimal places.
    /// let precision = 2;
    ///
    /// future_value_solution(0.11, 4, 5_000, false)
    ///     .series()
    ///     .print_table_locale(&locale, precision);
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
    /// [TvmSolution](struct.TvmSolution.html) has its own
    /// [print_table_locale](struct.TvmSolution.html#method.print_table_locale) method so this would
    /// produce the same results without doing the interim step of creating the series:
    /// ```
    /// # use finance_solution::core::*;
    /// # let locale = num_format::Locale::en;
    /// # let precision = 2;
    /// future_value_solution(0.11, 4, 5_000, false)
    ///     .print_table_locale(&locale, precision);
    /// ```
    /// Therefore the only reason to use this method is if the series has been changed in some way
    /// such as being filtered or summarized.
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

    /// Compares the results of two series, such as the results from two calls to
    /// [future_value_solution](future_value/fn.future_value_solution.html) with different periodic rates.
    ///
    /// It's fine to compare calculations that solved for different variables such as a rate
    /// calculation vs. a present value calculation.
    ///
    /// The column headers for values from the first series end in "_a" and those for the second
    /// series end in "_b". Money amounts are rounded to four decimal places, rates to six places,
    /// and numbers are formatted similar to Rust constants such as "10_000.0322". For more control
    /// over formatting use
    /// [print_ab_comparison_locale](#method.print_ab_comparison_locale).
    ///
    /// # Arguments
    /// * `other` - The second `TvmSeries` in the comparison which will be labeled "b".
    ///
    /// # Examples
    /// See [print_ab_comparison_locale](#method.print_ab_comparison_locale).
    /// The only difference is that there are no `locale` and `precision` arguments so the last line
    /// would simply be:
    /// ```
    /// # use finance_solution::core::*;
    /// # let series_a = future_value_solution(0.01, 1, 1, false).series();
    /// # let series_b = series_a.clone();
    /// series_a.print_ab_comparison(&series_b);
    /// ```
    pub fn print_ab_comparison(
        &self,
        other: &TvmSeries)
    {
        self.print_ab_comparison_locale_opt(other, None, None);
    }

    /// Compares the results of two series such as the results from two calls to
    /// [rate_solution](rate/fn.rate_solution.html) with different future values. The method has
    /// options for formatting numbers.
    ///
    /// It's fine to compare calculations that solved for different variables such as a rate
    /// calculation vs. a present value calculation.
    ///
    /// The column headers for values from the first series end in "_a" and those for the second
    /// series end in "_b". For a simpler method that doesn't require locale information use
    /// [print_ab_comparison](#method.print_ab_comparison).
    ///
    /// # Arguments
    /// * `other` - The second `TvmSeries` in the comparison which will be labeled "b".
    /// * `locale` - A variant of the num_format::Locale enum which determines the characters used
    /// for thousands separators and the decimal separator
    /// * `precision` - The number of decimal places. Rates always appear with at least six places
    /// regardless of this value.
    ///
    /// # Examples
    /// ```
    /// use finance_solution::core::*;
    ///
    /// // Two future value calculations that are the same except for the interest rate.
    /// let rate_a = 0.021;
    /// let rate_b = 0.023;
    /// let periods = 60;
    /// let present_value = -10_000;
    /// let continuous = false;
    /// let series_a = future_value_solution(rate_a, periods, present_value, continuous)
    ///     .series();
    /// let series_b = future_value_solution(rate_b, periods, present_value, continuous)
    ///     .series();
    ///
    /// // Compare the two series using English number formatting and two decimal places.
    /// let locale = num_format::Locale::en;
    /// let precision = 2;
    /// series_a.print_ab_comparison_locale(&series_b, &locale, precision);
    /// ```
    /// Output (with only the first few periods shown):
    /// ```text
    /// period    rate_a    rate_b    value_a    value_b
    /// ------  --------  --------  ---------  ---------
    ///      0  0.000000  0.000000  10,000.00  10,000.00
    ///      1  0.021000  0.023000  10,209.00  10,230.00
    ///      2  0.021000  0.023000  10,424.41  10,465.29
    ///      3  0.021000  0.023000  10,643.32  10,705.99
    /// ```
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
    /// [TvmSolution](struct.TvmSolution.html) every period will have the same rate. If it's a
    /// [TvmScheduleSolution](struct.TvmScheduleSolution.html) each period
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
    /// variables such as pv for present value call [symbolic_formula](#method.symbolic_formula).
    pub fn formula(&self) -> &str {
        &self.formula
    }

    /// Returns a text version of the formula used to calculate the value for the current period.
    /// The formula includes variables such as r for the rate. For the formula with actual values
    /// rather than variables call [formula](#method.formula).
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
                let value = -present_value;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::cashflow::payment;

    #[test]
    fn test_tvm_symmetry_one() {
        let rate = 0.10;
        let periods = 4;
        let present_value = -5_000.00;
        // Check the symmetry with simple compounding then continuous compounding.
        check_symmetry(rate, periods, present_value, false);
        check_symmetry(rate, periods, present_value, true);
    }

    #[test]
    fn test_tvm_symmetry_multiple() {
        let rates = vec![-1.0, -0.05, -0.005, 0.0, 0.005, 0.05];
        // let rates = vec![-0.5, -0.05, -0.005, 0.0, 0.005, 0.05, 0.5, 1.0, 10.0, 100.0];
        // let periods: Vec<u32> = vec![0, 1, 2, 5, 10, 36, 100, 1_000];
        let periods: Vec<u32> = vec![0, 1, 2, 5, 10, 36];
        let present_values: Vec<f64> = vec![-1_000_000.0, -1_234.98, -1.0, 0.0, 5.55555, 99_999.99];
        for rate_one in rates.iter() {
            for periods_one in periods.iter() {
                for present_value_one in present_values.iter() {
                    if !(*periods_one > 50 && *rate_one > 0.01) {
                        if !(*periods_one == 0 && *present_value_one != 0.0) {
                            for continuous_one in [false, true].iter() {

                                check_symmetry(*rate_one, *periods_one, *present_value_one, *continuous_one);
                            }
                        }
                    }
                }
            }
        }
    }

    fn check_symmetry(rate_in: f64, periods_in: u32, present_value_in: f64, continuous_in: bool) {
        let display = false;

        if display { dbg!("check_symmetry", rate_in, periods_in, present_value_in); }

        // Calculate the future value given the other three inputs so that we have all four values
        // which we can use in various combinations to confirm that all four basic TVM functions
        // return consistent values.
        let future_value_calc = future_value(rate_in, periods_in, present_value_in, continuous_in);
        if display { dbg!(future_value_calc); }
        if display { dbg!(future_value_calc.is_normal()); }
        if is_approx_equal!(0.0, future_value_calc) && rate_in > -1.0 {
            // In this case the rate is negative enough when compounded by the number of periods
            // that the future value is effectively zero even though the rate is not -100%. That's
            // fine as far as the future value calculation and it's returning a number very close to
            // zero as it should. But such a case will run into problems in a symmetry test because
            // for instance we'll try to determine the present value with a future value of zero and
            // there's no way to do that.
            return;
        }

        let rate_calc = rate(periods_in, present_value_in, future_value_calc, continuous_in);

        if display {
            println!("\ncheck_symmetry(): rate_in = {}, periods_in = {}, present_value_in = {}, continuous_in = {}\n\tfuture_value_calc = {}, rate_calc = {}",
                     rate_in, periods_in, present_value_in, continuous_in, future_value_calc, rate_calc);
        }

        if display { dbg!(rate_calc); }
        if periods_in == 0 || is_approx_equal!(0.0, present_value_in) {
            // With zero periods or zero for the present value, presumably the future value is the
            // same as the present value and any periodic rate would be fine so we arbitrarily
            // return zero.
            assert_approx_equal_symmetry_test!(present_value_in, future_value_calc);
            assert_approx_equal_symmetry_test!(0.0, rate_calc);
        } else {
            if display { dbg!(rate_calc, rate_in); }
            assert_approx_equal_symmetry_test!(rate_calc, rate_in);
        }

        let fractional_periods_calc = periods(rate_in, present_value_in, future_value_calc, continuous_in);
        if display { dbg!(fractional_periods_calc); }
        let periods_calc = round_4(fractional_periods_calc).ceil() as u32;
        if display { dbg!(periods_calc); }
        if is_approx_equal!(0.0, rate_in) || is_approx_equal!(0.0, present_value_in) || periods_in == 0 {
            // If the rate is zero or the present value is zero then the present value and future
            // value will be the same (but with opposite signs) and periods() will return zero since
            // no periods are required.
            assert_approx_equal_symmetry_test!(present_value_in, -future_value_calc);
            assert_eq!(0, periods_calc);
        } else if is_approx_equal!(-1.0, rate_in) {
            // The investment will drop to zero by the end of the first period so periods() will
            // return 1.
            assert_approx_equal_symmetry_test!(0.0, future_value_calc);
            assert_eq!(1, periods_calc);
        } else {
            // This is the normal case and we expect periods() to return the same number of periods
            // we started with.
            assert_eq!(periods_calc, periods_in);
        }

        if future_value_calc.is_normal() {
            let present_value_calc = present_value(rate_in, periods_in, future_value_calc, continuous_in);
            if display { dbg!(present_value_calc); }
            assert_approx_equal_symmetry_test!(present_value_calc, present_value_in);
        };

        // Create TvmSolution structs by solving for each of the four possible variables.
        let mut solutions = vec![
            rate_solution(periods_in, present_value_in, future_value_calc, continuous_in),
            periods_solution(rate_in, present_value_in, future_value_calc, continuous_in),
            future_value_solution(rate_in, periods_in, present_value_in, continuous_in),
        ];

        if future_value_calc.is_normal() {
            solutions.push(present_value_solution(rate_in, periods_in, future_value_calc, continuous_in));
        }
        for solution in solutions.iter() {
            if display { dbg!(solution); }
            if solution.calculated_field().is_rate() {
                // There are a few special cases in which the calculated rate is arbitrarily set to
                // zero since any value would work. We've already checked rate_calc against those
                // special cases, so use that here for the comparison.
                if !is_approx_equal_symmetry_test!(rate_calc, solution.rate()) {
                    if display { dbg!(rate_calc, solution.rate(), &solution); }
                }
                assert_approx_equal_symmetry_test!(rate_calc, solution.rate());
            } else {
                assert_approx_equal_symmetry_test!(rate_in, solution.rate());
            }
            if solution.calculated_field().is_periods() {
                // There are a few special cases in which the number of periods might be zero or one
                // instead of matching periods_in. So check against the number returned from
                // periods().
                assert_eq!(periods_calc, solution.periods());
            } else {
                assert_eq!(periods_in, solution.periods());
            }
            assert_approx_equal_symmetry_test!(present_value_in, solution.present_value());
            assert_approx_equal_symmetry_test!(future_value_calc, solution.future_value());
        }

        // Check each series in isolation.
        for solution in solutions.iter() {
            let label = format!("Solution for {:?}", solution.calculated_field());
            if display { dbg!(&label); }
            check_series_internal(label, solution.calculated_field(), &solution.series(), rate_in, periods_in, present_value_in, continuous_in, future_value_calc, rate_calc, periods_calc);
        }

        // Confirm that all of the series have the same values for all periods regardless of how we
        // did the calculation. For the reference solution take the result of
        // future_value_solution(). It would also work to use the result of rate_solution() and
        // present_value_solution() but not periods_solution() since there are some special cases in
        // which this will create fewer periods than the other functions.
        let reference_solution = solutions.iter().find(|solution| solution.calculated_field().is_future_value()).unwrap();
        let reference_series = reference_solution.series();
        for solution in solutions.iter().filter(|solution| !solution.calculated_field().is_future_value()) {
            let label = format!("Solution for {:?}", solution.calculated_field());
            check_series_same_values(reference_solution, &reference_series,label, solution.calculated_field(), &solution.series());
        }

        if !continuous_in {
            // Create a list of rates that are all the same so that we can try the _schedule functions
            // For present value and future value
            let mut rates_in = vec![];
            for _ in 0..periods_in {
                rates_in.push(rate_in);
            }

            if future_value_calc.is_normal() {
                let present_value_schedule_calc = present_value_schedule(&rates_in, future_value_calc);
                if display { dbg!(present_value_schedule_calc); }
                assert_approx_equal_symmetry_test!(present_value_schedule_calc, present_value_in);
            }

            let future_value_schedule_calc = future_value_schedule(&rates_in, present_value_in);
            if display { dbg!(future_value_schedule_calc); }
            assert_approx_equal_symmetry_test!(future_value_schedule_calc, future_value_calc);
            let mut schedules = vec![future_value_schedule_solution(&rates_in, present_value_in)];
            if future_value_calc.is_normal() {
                schedules.push(present_value_schedule_solution(&rates_in, future_value_calc));
            }

            for schedule in schedules.iter() {
                if display { dbg!(schedule); }
                assert_eq!(periods_in, schedule.rates().len() as u32);
                assert_eq!(periods_in, schedule.periods());
                assert_approx_equal_symmetry_test!(present_value_in, schedule.present_value());
                assert_approx_equal_symmetry_test!(future_value_calc, schedule.future_value());
            }

            for solution in schedules.iter() {
                let label = format!("Schedule for {:?}", solution.calculated_field());
                if display { dbg!(&label); }
                check_series_internal(label, solution.calculated_field(),  &solution.series(), rate_in, periods_in, present_value_in, continuous_in, future_value_calc, rate_calc, periods_calc);
            }

            let reference_solution = solutions.iter().find(|solution| solution.calculated_field().is_future_value()).unwrap();
            let reference_series = reference_solution.series();
            for schedule in schedules.iter() {
                let label = format!("Schedule for {:?}", schedule.calculated_field());
                check_series_same_values(reference_solution, &reference_series, label, schedule.calculated_field(), &schedule.series());
            }

        }

        if !continuous_in && rate_in > -1.0 {
            // Check that we can use the given values in a payment calculation and get zero for the
            // payment.
            let payment_calc = payment::payment(rate_in, periods_in, present_value_in, future_value_calc, false);
            assert_approx_equal!(0.0, payment_calc);
        }
    }

    fn check_series_internal(
        label: String,
        calculated_field: &TvmVariable,
        series: &TvmSeries,
        rate_in: f64,
        periods_in: u32,
        present_value_in: f64,
        continuous_in: bool,
        future_value_calc: f64,
        rate_calc: f64,
        periods_calc: u32)
    {
        let display = false;

        if display { dbg!(label); }
        if display { dbg!(&series); }
        if calculated_field.is_periods() {
            // There are a few special cases in which the number of periods might be zero or one
            // instead of matching periods_in. So check against the number returned from
            // periods().
            assert_eq!(periods_calc + 1, series.len() as u32);
        } else {
            assert_eq!(periods_in + 1, series.len() as u32);
        }

        check_series_from_to(series, rate_in, periods_in, present_value_in, future_value_calc, continuous_in);

        let mut prev_value: Option<f64> = None;
        for (period, entry) in series.iter().enumerate() {
            assert_eq!(period as u32, entry.period());
            if period == 0 {
                assert_approx_equal_symmetry_test!(0.0, entry.rate());
                // The first entry should always contain the starting value.
                assert_approx_equal_symmetry_test!(-present_value_in, entry.value());
            } else {
                // We're past period 0.
                let effective_rate = if calculated_field.is_rate() {
                    // There are a few special cases in which the calculated rate is arbitrarily set
                    // to zero since any value would work. We've already checked rate_calc against
                    // those special cases, so use that here for the comparison.
                    assert_approx_equal_symmetry_test!(rate_calc, entry.rate());
                    rate_calc
                } else {
                    assert_approx_equal_symmetry_test!(rate_in, entry.rate());
                    rate_in
                };
                // Compare this period's value to the one before.
                if is_approx_equal!(0.0, effective_rate) || is_approx_equal!(0.0, prev_value.unwrap()) {
                    // The rate is zero or the previous value was zero so each period's value should
                    // be the same as the one before.
                    assert_approx_equal_symmetry_test!(entry.value(), prev_value.unwrap());
                } else if effective_rate < 0.0 {
                    // The rate is negative so the value should be shrinking from period to period,
                    // but since the value could be negative shrinking in this case means getting
                    // closer to zero.
                    assert!(entry.value.abs() < prev_value.unwrap().abs());
                } else {
                    // The rate is negative so the value should be growing from period to period,
                    // but since the value could be negative growing in this case means moving away
                    // from zero.
                    assert!(entry.value.abs() > prev_value.unwrap().abs());
                }
                /*
                } else if present_value_in.signum() == effective_rate.signum() {
                    // Either the starting value and the rate are both positive or they're both
                    // negative. In either case each period's value should be greater than the one
                    // before.
                    assert!(entry.value() > prev_value.unwrap());
                } else {
                    // Either the starting value is positive and the rate is negative or vice versa.
                    // In either case each period's value should be smaller than the one before.
                    assert!(entry.value() < prev_value.unwrap());
                }*/
            }
            if period == series.len() - 1 {
                // This is the last period's entry. It should contain the future value.
                //bg!(future_value_calc, entry.value());
                assert_approx_equal_symmetry_test!(future_value_calc, entry.value());
            }
            prev_value = Some(entry.value());
        }
    }

    fn check_series_from_to(series: &TvmSeries, r: f64, n: u32, pv: f64, fv: f64, continuous: bool) {
        // For each period in the series, we should be able to do all of the TVM calculations as if
        // we'd started at that point. Likewise we should be able to do the calculations as if the
        // current period is the endpoint.

        // This should work for all periods including period 0 and the last period except for a few
        // special cases such as trying to calculate a rate when there are zero periods.

        let display = false;

        if display { println!("\ncheck_series_from_to(): r = {}, n = {}, pv = {}, fv = {}, continuous = {}", r, n, pv, fv, continuous); }

        for (period, entry) in series.iter().enumerate() {
            if display {
                if display { println!("\ncheck_series_from_to(): r = {}, n = {}, pv = {}, fv = {}, continuous = {}, period = {}", r, n, pv, fv, continuous, period); }
                //bg!(entry);
            }

            assert_eq!(period as u32, entry.period());

            let n_so_far = period as u32;
            let n_remaining = n - period as u32;

            if n_remaining > 0 {
                // Calculate the rate as if all we knew was the current period and the future value.
                // This should be the same as the rate from the real solution.
                let r_from = rate(n_remaining, -entry.value(), fv, continuous);
                if display { dbg!(r, r_from); }
                assert_approx_equal_symmetry_test!(r, r_from);
            }

            if n_so_far > 0 {
                // Calculate the rate as if all we knew was the present value and the current period.
                // This should be the same as the rate from the real solution.
                let r_to = rate(n_so_far, pv, entry.value(), continuous);
                if display { dbg!(r, r_to); }
                assert_approx_equal_symmetry_test!(r, r_to);
            }

            if r > -1.0 && !is_approx_equal!(0.0, pv + fv) {
                // Calculate the periods as if all we knew was the current period and the future value.
                // This should be the same as the periods from this point forward.
                let n_from = periods(r, -entry.value(), fv, continuous);
                let n_from = n_from.round() as u32;
                if display { dbg!(n_remaining, n_from); }
                assert_eq!(n_remaining, n_from);

                // Calculate the periods as if all we knew was the current period and the present value.
                // This should be the same as the current period number.
                let n_to = periods(r, pv, entry.value(), continuous);
                let n_to = n_to.round() as u32;
                if display { dbg!(n_so_far, n_to); }
                assert_eq!(n_so_far, n_to);
            }

            if !is_approx_equal!(0.0, fv) {
                // Calculate the present value as if we'd started at this period rather than period 0.
                // This should be the same as the value of this period (with the signs reversed).
                let pv_from = present_value(r, n_remaining, fv, continuous);
                if display { dbg!(-entry.value(), pv_from); }
                assert_approx_equal_symmetry_test!(-entry.value(), pv_from);

                // Calculate the present value as if we'd ended after this period. This should be the
                // same as the original present value.
                let pv_to = present_value(r, n_so_far, entry.value(), continuous);
                if display { dbg!(pv, pv_to); }
                assert_approx_equal_symmetry_test!(pv, pv_to);
            }

            // Calculate the future value as if we'd started at this period rather than period 0.
            // This should be the same as the future value from the real solution.
            let fv_from = future_value(r, n_remaining, -entry.value(), continuous);
            if display { dbg!(fv, fv_from); }
            assert_approx_equal_symmetry_test!(fv, fv_from);

            // Calculate the future value as if we'd ended after this period. This should be the
            // same as the value of this period.
            let fv_to = future_value(r, n_so_far, pv, continuous);
            if display { dbg!(entry.value(), fv_to); }
            assert_approx_equal_symmetry_test!(entry.value(), fv_to);

            if !continuous && r > -1.0 {
                // Calculate the payment starting from this point. It should be zero because we're
                // starting with the four variables of the simple TVM calculations (rate, periods,
                // present value, and future value) and the calculation works out without any payments.
                let pmt_from = payment(r, n_remaining, -entry.value(), fv, false);
                if display { dbg!(pmt_from); }
                assert_approx_equal!(0.0, pmt_from);

                // Calculate the payment as if we'd ended after this period. It should be zero.
                let pmt_to = payment(r, n_so_far, pv, entry.value(), false);
                if display { dbg!(pmt_to); }
                assert_approx_equal!(0.0, pmt_to);
            }

        }
    }

    fn check_series_same_values(_reference_solution: &TvmSolution, reference_series: &TvmSeries, _label: String, calculated_field: &TvmVariable, series: &[TvmPeriod]) {
        //bg!(reference_solution);
        //bg!(&reference_series);

        //bg!(label);
        //bg!(&series);

        if calculated_field.is_periods() && reference_series.len() != series.len() {

            // There are a few special cases in which the number of periods might be zero or one
            // instead of matching periods_in.

            // There will always be at least a period 0.
            let reference_entry = &reference_series[0];
            let entry = &series[0];
            //bg!(&reference_entry, &entry);
            assert_eq!(reference_entry.period(), entry.period());
            assert_approx_equal_symmetry_test!(reference_entry.rate(), entry.rate());
            assert_approx_equal_symmetry_test!(reference_entry.value(), entry.value());

            // Check the last period.
            let reference_entry = &reference_series.last().unwrap();
            let entry = &series.last().unwrap();
            //bg!(&reference_entry, &entry);
            if reference_series.len() > 1 && series.len() > 1 {
                assert_approx_equal_symmetry_test!(reference_entry.rate(), entry.rate());
            }
            assert_approx_equal_symmetry_test!(reference_entry.value(), entry.value());
        } else {

            // This is the usual case where we expect the two series to be identical except for
            // the formulas.

            assert_eq!(reference_series.len(), series.len());

            for (period, reference_entry) in reference_series.iter().enumerate() {
                let entry = &series[period];
                //bg!(&reference_entry, &entry);
                assert_eq!(reference_entry.period(), entry.period());
                if calculated_field.is_rate() {
                    // There are a few special cases where the calculated rate will be zero since
                    // any answer would work.
                    if entry.rate() != 0.0 {
                        assert_approx_equal_symmetry_test!(reference_entry.rate(), entry.rate());
                    }
                } else {
                    assert_approx_equal_symmetry_test!(reference_entry.rate(), entry.rate());
                }
                //bg!(reference_entry.value(), round_4(reference_entry.value()), entry.value(), round_4(entry.value()));
                assert_approx_equal_symmetry_test!(reference_entry.value(), entry.value());
                // assert_eq!(reference_entry.value.round(), entry.value.round());
            }
        }

    }



    #[test]
    fn test_continuous_symmetry_one() {
        let rate = 0.10;
        let periods = 4;
        let present_value = 5_000.00;
        check_continuous_symmetry(rate, periods, present_value);
    }

    /*
    #[test]
    fn test_symmetry_multiple() {
        let rates = vec![-1.0, -0.5, -0.05, -0.005, 0.0, 0.005, 0.05, 0.5, 1.0, 10.0, 100.0];
        // let rates = vec![-0.5, -0.05, -0.005, 0.0, 0.005, 0.05, 0.5, 1.0, 10.0, 100.0];
        // let periods: Vec<u32> = vec![0, 1, 2, 5, 10, 36, 100, 1_000];
        let periods: Vec<u32> = vec![0, 1, 2, 5, 10, 36];
        let present_values: Vec<f64> = vec![-1_000_000.0, -1_234.98, -1.0, 0.0, 5.55555, 99_999.99];
        for rate_one in rates.iter() {
            for periods_one in periods.iter() {
                for present_value_one in present_values.iter() {
                    if !(*periods_one > 50 && *rate_one > 0.01) {
                        check_symmetry(*rate_one, *periods_one, *present_value_one);
                    }
                }
            }
        }
    }
    */

    fn check_continuous_symmetry(rate_in: f64, periods_in: u32, present_value_in: f64) {
        let display = false;

        if display {
            println!();
            dbg!("check_continuous_symmetry", rate_in, periods_in, present_value_in);
        }

        /*
        let fv_calc = present_value_in * std::f64::consts::E.powf(rate_in * periods_in as f64);
        dbg!(fv_calc);
        let pv_calc = fv_calc / std::f64::consts::E.powf(rate_in * periods_in as f64);
        dbg!(pv_calc);
        */

        // Calculate the future value given the other three inputs so that we have all four values
        // which we can use in various combinations to confirm that all four continuous TVM
        // functions return consistent values.
        let future_value_calc = future_value(rate_in, periods_in, present_value_in, true);
        if display { dbg!(future_value_calc); }

        let rate_calc = rate::rate(periods_in, present_value_in, future_value_calc, true);
        if display { dbg!(rate_calc); }
        if periods_in == 0 || present_value_in == 0.0 {
            // With zero periods or zero for the present value, presumably the future value is the
            // same as the present value and any rate would be fine so we arbitrarily
            // return zero.
            assert_approx_equal_symmetry_test!(present_value_in, future_value_calc);
            assert_approx_equal_symmetry_test!(0.0, rate_calc);
        } else {
            if display { dbg!(rate_calc, rate_in); }
            assert_approx_equal_symmetry_test!(rate_calc, rate_in);
        }

        let fractional_periods_calc = periods(rate_in, present_value_in, future_value_calc, true);
        if display { dbg!(fractional_periods_calc); }
        let periods_calc = round_4(fractional_periods_calc).ceil() as u32;
        if display { dbg!(periods_calc); }
        if rate_in == 0.0 || present_value_in == 0.0 || periods_in == 0 {
            // If the rate is zero or the present value is zero then the present value and future
            // value will be the same and periods() will return zero since no periods are required.
            assert_approx_equal_symmetry_test!(present_value_in, future_value_calc);
            assert_eq!(0, periods_calc);
        } else if rate_in == -1.0 {
            // The investment will drop to zero by the end of the first period so periods() will
            // return 1.
            assert_approx_equal_symmetry_test!(0.0, future_value_calc);
            assert_eq!(1, periods_calc);
        } else {
            // This is the normal case and we expect periods() to return the same number of periods
            // we started with.
            assert_eq!(periods_calc, periods_in);
        }

        if future_value_calc.is_normal() {
            let present_value_calc = present_value(rate_in, periods_in, future_value_calc, true);
            if display { dbg!(present_value_calc); }
            assert_approx_equal_symmetry_test!(present_value_calc, present_value_in);
        };

        // Create TvmSolution structs by solving for each of the four possible variables.
        let mut solutions = vec![
            rate_solution(periods_in, present_value_in, future_value_calc, true),
            periods_solution(rate_in, present_value_in, future_value_calc, true),
            future_value_solution(rate_in, periods_in, present_value_in, true),
        ];

        if future_value_calc.is_normal() {
            solutions.push(present_value_solution(rate_in, periods_in, future_value_calc, true));
        }
        for solution in solutions.iter() {
            if display { dbg!(solution); }
            // let series = solution.series();
            // dbg!(&series);
            if solution.calculated_field().is_rate() {
                // There are a few special cases in which the calculated rate is arbitrarily set to
                // zero since any value would work. We've already checked rate_calc against those
                // special cases, so use that here for the comparison.
                assert_approx_equal_symmetry_test!(rate_calc, solution.rate());
            } else {
                assert_approx_equal_symmetry_test!(rate_in, solution.rate());
            }
            if solution.calculated_field().is_periods() {
                // There are a few special cases in which the number of periods might be zero or one
                // instead of matching periods_in. So check against the number returned from
                // periods().
                assert_eq!(periods_calc, solution.periods());
            } else {
                assert_eq!(periods_in, solution.periods());
            }
            assert_approx_equal_symmetry_test!(present_value_in, solution.present_value());
            assert_approx_equal_symmetry_test!(future_value_calc, solution.future_value());
        }

        // Check each series in isolation.
        /*
        for solution in solutions.iter() {
            let label = format!("Solution for {:?}", solution.calculated_field());
            //bg!(&label);
            check_series_internal(label, solution.calculated_field().clone(), &solution.series(), rate_in, periods_in, present_value_in, future_value_calc, rate_calc, periods_calc);
        }
        */

        // Confirm that all of the series have the same values for all periods regardless of how we
        // did the calculation. For the reference solution take the result of
        // future_value_solution(). It would also work to use the result of rate_solution() and
        // present_value_solution() but not periods_solution() since there are some special cases in
        // which this will create fewer periods than the other functions.
        let reference_solution = solutions.iter().find(|solution| solution.calculated_field().is_future_value()).unwrap();
        let reference_series = reference_solution.series();
        for solution in solutions.iter().filter(|solution| !solution.calculated_field().is_future_value()) {
            let label = format!("Solution for {:?}", solution.calculated_field());
            check_series_same_values(reference_solution, &reference_series, label, solution.calculated_field(), &solution.series());
        }
    }

    #[test]
    fn test_simple_to_continuous_symmetry_one() {
        let rate = 0.10;
        let periods = 4;
        let present_value = 5_000.00;
        check_simple_to_continuous_symmetry(rate, periods, present_value);
    }

    /*
    #[test]
    fn test_symmetry_multiple() {
        let rates = vec![-1.0, -0.5, -0.05, -0.005, 0.0, 0.005, 0.05, 0.5, 1.0, 10.0, 100.0];
        // let rates = vec![-0.5, -0.05, -0.005, 0.0, 0.005, 0.05, 0.5, 1.0, 10.0, 100.0];
        // let periods: Vec<u32> = vec![0, 1, 2, 5, 10, 36, 100, 1_000];
        let periods: Vec<u32> = vec![0, 1, 2, 5, 10, 36];
        let present_values: Vec<f64> = vec![-1_000_000.0, -1_234.98, -1.0, 0.0, 5.55555, 99_999.99];
        for rate_one in rates.iter() {
            for periods_one in periods.iter() {
                for present_value_one in present_values.iter() {
                    if !(*periods_one > 50 && *rate_one > 0.01) {
                        check_symmetry(*rate_one, *periods_one, *present_value_one);
                    }
                }
            }
        }
    }
    */

    fn check_simple_to_continuous_symmetry(rate_in: f64, periods_in: u32, present_value_in: f64) {
        let display = false;

        if display {
            println!();
            dbg!("check_simple_to_continuous_symmetry", rate_in, periods_in, present_value_in);
        }

        // Calculate the future value given the other three inputs so that we have all four values
        // which we can use in various combinations to confirm that all four continuous TVM
        // functions return consistent values.
        let future_value_calc = future_value(rate_in, periods_in, present_value_in, true);
        if display { dbg!(future_value_calc); }

        // Create TvmSolution structs with continuous compounding by solving for each of the four possible variables.
        let continuous_solutions = vec![
            rate_solution(periods_in, present_value_in, future_value_calc, true),
            periods_solution(rate_in, present_value_in, future_value_calc, true),
            present_value_solution(rate_in, periods_in, future_value_calc, true),
            future_value_solution(rate_in, periods_in, present_value_in, true),
        ];

        // For each solution with continuous compounding create a corresponding solution with
        // simple compounding.
        /*
        let simple_solutions = continuous_solutions.iter()
            .map(|continuous_solution| continuous_solution.with_simple_compounding())
            .collect::<Vec<_>>();
        */
        let simple_solutions = [
            continuous_solutions[0].rate_solution(false, None),
            continuous_solutions[1].periods_solution(false),
            continuous_solutions[2].present_value_solution(false, None),
            continuous_solutions[3].future_value_solution(false, None),
        ];

        // Compare the continuous solutions to the corresponding simple solutions.
        for (index, continuous_solution) in continuous_solutions.iter().enumerate() {
            let simple_solution = &simple_solutions[index];
            if display {
                println!("\nContinuous compounding vs. simple compounding adjusting {} while keeping the other three values constant.\n", continuous_solution.calculated_field().to_string().to_lowercase());
                dbg!(&continuous_solution, &simple_solution);
            }
            assert_eq!(continuous_solution.calculated_field(), simple_solution.calculated_field());
            assert!(continuous_solution.continuous_compounding());
            assert!(!simple_solution.continuous_compounding());
            if continuous_solution.calculated_field().is_rate() {
                // We expect the rate to be lower with continuous compounding when the other three
                // inputs are held constant.
                assert!(continuous_solution.rate().abs() < simple_solution.rate().abs());
            } else {
                // The rate was an input rather than being calculated, so it should be the same.
                assert_eq!(continuous_solution.rate(), simple_solution.rate());
            }
            if continuous_solution.calculated_field().is_periods() {
                // We expect the fractional periods to be the same or lower with continuous
                // compounding when the other three inputs are held constant.
                assert!(continuous_solution.fractional_periods() <= simple_solution.fractional_periods());
                // Depending on rounding the number of periods may be the same or less for
                // continuous compounding.
                assert!(continuous_solution.periods() <= simple_solution.periods());
            } else {
                // The number of periods was an input rather than being calculated, so it should be
                // the same.
                assert_eq!(continuous_solution.periods(), simple_solution.periods());
            }
            if continuous_solution.calculated_field().is_present_value() {
                // We expect the present value to be lower with continuous compounding when the
                // other three inputs are held constant. This is because it takes less of an initial
                // investment to reach the same final value.
                assert!(continuous_solution.present_value().abs() < simple_solution.present_value().abs());
            } else {
                // The present value was an input rather than being calculated, so it should be the
                // same.
                assert_eq!(continuous_solution.present_value(), simple_solution.present_value());
            }
            if continuous_solution.calculated_field().is_future_value() {
                // We expect the future value to be higher with continuous compounding when the
                // other three inputs are held constant.
                assert!(continuous_solution.future_value().abs() > simple_solution.future_value().abs());
            } else {
                // The future value was an input rather than being calculated, so it should be the
                // same.
                assert_eq!(continuous_solution.future_value(), simple_solution.future_value());
            }
            assert_ne!(continuous_solution.formula(), simple_solution.formula());
            assert_ne!(continuous_solution.symbolic_formula(), simple_solution.symbolic_formula());
        }

        // For each solution with simple compounding create a corresponding solution with
        // continuous compounding. This should get us back to the equivalents of our original list
        // of solutions with continuous compounding.
        /*
        let continuous_solutions_round_trip = simple_solutions.iter()
            .map(|simple_solution| simple_solution.with_continuous_compounding())
            .collect::<Vec<_>>();
        */
        let continuous_solutions_round_trip = [
            continuous_solutions[0].rate_solution(true, None),
            continuous_solutions[1].periods_solution(true),
            continuous_solutions[2].present_value_solution(true, None),
            continuous_solutions[3].future_value_solution(true, None),
        ];

        // Compare the recently created continuous solutions to the original continuous solutions.
        for (index, solution) in continuous_solutions.iter().enumerate() {
            let solution_round_trip = &continuous_solutions_round_trip[index];
            println!("\nOriginal continuous compounding vs. derived continuous compounding where the calculated field is {}.\n", solution.calculated_field().to_string().to_lowercase());
            if display { dbg!(&solution, &solution_round_trip); }
            assert_eq!(solution, solution_round_trip);
        }
        /*
        for (calculated_field, continuous_solution) in continuous_solutions.iter() {
            dbg!(&continuous_solution);
            dbg!(&continuous_solution.series());

        }
        */

        // Check each series in isolation.
        /*
        for solution in solutions.iter() {
            let label = format!("Solution for {:?}", solution.calculated_field());
            //bg!(&label);
            check_series_internal(label, solution.calculated_field().clone(), &solution.series(), rate_in, periods_in, present_value_in, future_value_calc, rate_calc, periods_calc);
        }
        */

        /*
        // Confirm that all of the series have the same values for all periods regardless of how we
        // did the calculation. For the reference solution take the result of
        // future_value_solution(). It would also work to use the result of rate_solution() and
        // present_value_solution() but not periods_solution() since there are some special cases in
        // which this will create fewer periods than the other functions.
        let reference_solution = solutions.iter().find(|x| x.calculated_field().is_future_value()).unwrap();
        for solution in solutions.iter().filter(|x| !x.calculated_field().is_future_value()) {
            let label = format!("Solution for {:?}", solution.calculated_field());
            check_series_same_values(reference_solution, label, solution.calculated_field().clone(), &solution.series());
        }
        */
    }

    fn setup_for_compounding_periods() -> (TvmSolution, Vec<u32>) {
        let rate = 0.10;
        let periods = 4;
        let present_value = 5_000.00;
        let compounding_periods = vec![1, 2, 4, 6, 12, 24, 52, 365];
        (future_value_solution(rate, periods, present_value, false), compounding_periods)
    }

    #[test]
    fn test_with_compounding_periods_vary_future_value() {
        let display = false;
        if display { println!("\ntest_with_compounding_periods_vary_future_value()\n"); }

        let (solution, compounding_periods) = setup_for_compounding_periods();
        if display { dbg!(&compounding_periods); }

        for one_compounding_period in compounding_periods.iter() {
            if display {
                println!("\nSimple compounding original vs. compounding periods = {} while varying future value.\n", one_compounding_period);
                dbg!(&solution, solution.future_value_solution(false, Some(*one_compounding_period)));
            }
        }
    }

    #[test]
    fn test_with_compounding_periods_vary_present_value() {
        let display = false;

        if display { println!("\ntest_with_compounding_periods_vary_present_value()\n"); }

        let (solution, compounding_periods) = setup_for_compounding_periods();
        if display { dbg!(&compounding_periods); }

        for one_compounding_period in compounding_periods.iter() {
            if display {
                println!("\nSimple compounding original vs. compounding periods = {} while varying present value.\n", one_compounding_period);
                dbg!(&solution, solution.present_value_solution(false, Some(*one_compounding_period)));
            }
        }
    }
}

#[derive(Debug)]
pub struct ScenarioList {
    pub setup: String,
    pub input_variable: TvmVariable,
    pub output_variable: TvmVariable,
    pub entries: Vec<ScenarioEntry>,
}

pub struct ScenarioEntry {
    pub input: f64,
    pub output: f64,
    input_precision: usize,
    output_precision: usize,
}

impl ScenarioList {

    pub(crate) fn new(setup: String, input_variable: TvmVariable, output_variable: TvmVariable, entries: Vec<(f64, f64)>) -> Self {
        let input_precision = match input_variable {
            TvmVariable::Periods => 0,
            TvmVariable::Rate => 6,
            _ => 4,
        };
        let output_precision = match output_variable {
            TvmVariable::Periods => 0,
            TvmVariable::Rate => 6,
            _ => 4,
        };
        let entries= entries.iter().map(|entry| ScenarioEntry::new(entry.0, entry.1, input_precision, output_precision)).collect();
        Self {
            setup,
            input_variable,
            output_variable,
            entries,
        }
    }

    pub fn print_table(&self) {
        self.print_table_locale_opt(None, None);
    }

    pub fn print_table_locale(&self, locale: &num_format::Locale, precision: usize) {
        self.print_table_locale_opt(Some(locale), Some(precision));
    }

    fn print_table_locale_opt(&self, locale: Option<&num_format::Locale>, precision: Option<usize>) {
        let columns = vec![self.input_variable.table_column_spec(true), self.output_variable.table_column_spec(true)];
        // let columns = columns_with_strings.iter().map(|x| &x.0[..], &x.1[..], x.2);
        let data = self.entries.iter()
            .map(|entry| vec![entry.input.to_string(), entry.output.to_string()])
            .collect::<Vec<_>>();
        print_table_locale_opt(&columns, data, locale, precision);
    }

}

impl ScenarioEntry {
    pub(crate) fn new(input: f64, output: f64, input_precision: usize, output_precision: usize) -> Self {
        Self { input, output, input_precision, output_precision }
    }
}

impl Debug for ScenarioEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let input = format_float_locale_opt(self.input, None, Some(self.input_precision));
        let output = format_float_locale_opt(self.output, None, Some(self.output_precision));
        write!(f, "{{ input: {}, output: {} }}", input, output)
    }
}
