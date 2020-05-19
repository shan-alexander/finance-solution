//! **Number of periods calculations.** Given a periodic rate, present value, and future value, find the
//! number of periods needed to satisfy the equation.
//! 
//! For most common usages, we recommend the [periods_solution](fn.periods_solution.html) function.
//!
//! # Concepts
//!
//! Suppose we invest $100 at 10% annual interest. After one year the investment is worth $110.
//! After two years it's worth $110 plus 10% or $121, and so on:
//!
//! <img src="http://i.upmath.me/svg/%24%24%5Cbegin%7Btikzpicture%7D%5Bscale%3D1.0544%5D%5Csmall%0A%5Cbegin%7Baxis%7D%5Baxis%20line%20style%3Dgray%2C%0A%09samples%3D12%2C%0A%09width%3D9.0cm%2Cheight%3D6.4cm%2C%0A%09xmin%3D0%2C%20xmax%3D12%2C%0A%09ymin%3D70%2C%20ymax%3D370%2C%0A%09restrict%20y%20to%20domain%3D0%3A1000%2C%0A%09ytick%3D%7B100%2C%20150%2C%20200%2C%20250%2C%20300%2C%20350%7D%2C%0A%09xtick%3D%7B1%2C2%2C3%2C4%2C5%2C6%2C7%2C8%2C9%2C10%2C11%2C12%7D%2C%0A%09axis%20x%20line%3Dcenter%2C%0A%09axis%20y%20line%3Dcenter%2C%0A%09xlabel%3D%24n%24%2Cylabel%3D%24fv%24%5D%0A%5Caddplot%5Bblue%2Cdomain%3D1%3A12%2Csemithick%2Conly%20marks%5D%7B100*((1.1)%5Ex)%7D%3B%0A%5Caddplot%5Bblue%5D%20coordinates%20%7B(5.4%2C110)%7D%20node%7B%24fv%3D100(1.1%5En)%24%7D%3B%0A%5Cpath%20(axis%20cs%3A0%2C122)%20node%20%5Banchor%3Dnorth%20west%2Cyshift%3D-0.07cm%5D%3B%0A%5Cend%7Baxis%7D%0A%5Cend%7Btikzpicture%7D%24%24" />
//!
//! Here `n` is the number of periods, in this case years, and `fv` is the future value, or the
//! value of the investment after some number of years. After 12 years the investment would grow to
//! a little over $300.
//!
//! But suppose our goal is to reach $250 and we need to know exactly how many years that will take.
//! This is where the periods calculations come in. They find the point where an investment reaches
//! some fixed value:
//!
//! <img src="http://i.upmath.me/svg/%24%24%5Cbegin%7Btikzpicture%7D%5Bscale%3D1.0544%5D%5Csmall%0A%5Cbegin%7Baxis%7D%5Baxis%20line%20style%3Dgray%2C%0A%09samples%3D100%2C%0A%09width%3D9.0cm%2Cheight%3D6.4cm%2C%0A%09xmin%3D0%2C%20xmax%3D12%2C%0A%09ymin%3D70%2C%20ymax%3D370%2C%0A%09restrict%20y%20to%20domain%3D0%3A1000%2C%0A%09ytick%3D%7B100%2C%20150%2C%20200%2C%20250%2C%20300%2C%20350%7D%2C%0A%09xtick%3D%7B1%2C2%2C3%2C4%2C5%2C6%2C7%2C8%2C9%2C10%2C11%2C12%7D%2C%0A%09axis%20x%20line%3Dcenter%2C%0A%09axis%20y%20line%3Dcenter%2C%0A%09xlabel%3D%24n%24%2Cylabel%3D%24fv%24%5D%0A%5Caddplot%5Bblue%2Cdomain%3D1%3A9.614%2Cthick%5D%7B100*((1.1)%5Ex)%7D%3B%0A%5Caddplot%5Bblue%2Cdomain%3D9.614%3A12%2Cthick%2Cdashed%5D%7B100*((1.1)%5Ex)%7D%3B%0A%5Caddplot%5Bblack%2Cdomain%3D1%3A12%5D%7B250%7D%3B%0A%5Caddplot%5B%5D%20coordinates%20%7B(2.1%2C%20270)%7D%20node%7B%24fv%3D250%24%7D%3B%0A%5Caddplot%5Bblue%5D%20coordinates%20%7B(5.5%2C120.3)%7D%20node%7B%24fv%3D100(1.1%5En)%24%7D%3B%0A%5Caddplot%5Bred%5D%20coordinates%20%7B(10.6%2C235)%7D%20node%7B%24n%3D9.61%24%7D%3B%0A%5Cpath%20(axis%20cs%3A0%2C122)%20node%20%5Banchor%3Dnorth%20west%2Cyshift%3D-0.07cm%5D%3B%0A%5Cend%7Baxis%7D%0A%5Cend%7Btikzpicture%7D%24%24" />
//!
//! Here the investment reaches $250 after 9.61 years.
//!
//! The same ideas apply with a negative rate. Suppose we have a value that starts at $100 and
//! declines by 10% per year. At what point does the value fall to $70?
//!
//! <img src="http://i.upmath.me/svg/%24%24%5Cbegin%7Btikzpicture%7D%5Bscale%3D1.0544%5D%5Csmall%0A%5Cbegin%7Baxis%7D%5Baxis%20line%20style%3Dgray%2C%0A%09samples%3D100%2C%0A%09width%3D9.0cm%2Cheight%3D6.4cm%2C%0A%09xmin%3D0%2C%20xmax%3D12%2C%0A%09ymin%3D0%2C%20ymax%3D120%2C%0A%09restrict%20y%20to%20domain%3D0%3A1000%2C%0A%09ytick%3D%7B10%2C%2020%2C%2030%2C%2040%2C%2050%2C%2060%2C%2070%2C80%2C%2090%2C%20100%7D%2C%0A%09xtick%3D%7B1%2C2%2C3%2C4%2C5%2C6%2C7%2C8%2C9%2C10%2C11%2C12%7D%2C%0A%09axis%20x%20line%3Dcenter%2C%0A%09axis%20y%20line%3Dcenter%2C%0A%09xlabel%3D%24n%24%2Cylabel%3D%24fv%24%5D%0A%5Caddplot%5Bblue%2Cdomain%3D1%3A3.385%2Cthick%5D%7B100*((0.9)%5Ex)%7D%3B%0A%5Caddplot%5Bblue%2Cdomain%3D3.385%3A12%2Cthick%2Cdashed%5D%7B100*((0.9)%5Ex)%7D%3B%0A%5Caddplot%5Bblack%2Cdomain%3D1%3A12%5D%7B70%7D%3B%0A%5Caddplot%5B%5D%20coordinates%20%7B(11%2C%2064)%7D%20node%7B%24fv%3D70%24%7D%3B%0A%5Caddplot%5Bblue%5D%20coordinates%20%7B(6.8%2C30)%7D%20node%7B%24fv%3D100(0.9%5En)%24%7D%3B%0A%5Caddplot%5Bred%5D%20coordinates%20%7B(4.5%2C75.1)%7D%20node%7B%24n%3D3.39%24%7D%3B%0A%5Cpath%20(axis%20cs%3A0%2C122)%20node%20%5Banchor%3Dnorth%20west%2Cyshift%3D-0.07cm%5D%3B%0A%5Cend%7Baxis%7D%0A%5Cend%7Btikzpicture%7D%24%24" />
//!
//! After 3.39 periods the value is $70.
//!
//! # Formulas
//!
//! ## Simple Compounding
//!
//! With simple compound interest, the number of periods is calculated with:
//!
//! <img src="http://i.upmath.me/svg/periods%20%3D%20%7Blog_%7B1%2Brate%7D%5Cleft(%7Bfuture%5C_value%20%5Cover%20present%5C_value%7D%5Cright)%20%5Cover%20rate%7D" />
//!
//! Or using some more common variable names:
//!
//! <img src="http://i.upmath.me/svg/n%20%3D%20%7Blog_%7B1%2Br%7D%5Cleft(%7Bfv%20%5Cover%20pv%7D%5Cright)%20%5Cover%20r%7D" />
//!
//! `n` is often used for the number of periods, though it may be `t` for time if each period is
//! assumed to be one year as in continuous compounding. `r` is the periodic rate, though this may
//! appear as `i` for interest.
//!
//! Throughout this crate we use `pv` for present value and `fv` for future value. You may see these
//! values called `P` for principal in some references.
//!
//! Within the [PeriodsSolution](./struct.PeriodsSolution.html) struct we record the formula used for the particular calculation
//! using both concrete values and symbols. For the example above with $100 growing at 10%, where we
//! want to end up with $250 the struct contains:
//! ```text
//! formula: "9.61 = log(250.0000 / 100.0000, base 1.100000)",
//! symbolic_formula: "n = log(fv / pv, base (1 + r))",
//! ```
//!
//! ## Continuous Compounding
//!
//! With continuous compounding it's:
//!
//! <img src="http://i.upmath.me/svg/periods%20%3D%20%7Blog_e%5Cleft(%7Bfuture%5C_value%20%5Cover%20present%5C_value%7D%5Cright)%20%5Cover%20rate%7D" />
//!
//! or:
//!
//! <img src="http://i.upmath.me/svg/n%20%3D%20%7Blog_e%5Cleft(%7Bfv%20%5Cover%20pv%7D%5Cright)%20%5Cover%20r%7D" />
//!
//! With continuous compounding the period is assumed to be years and `t` (time) is often used as
//! the variable name. Within this crate we stick with `n` for the number of periods so that it's
//! easier to compare formulas when they're printed as simple text as part of the [PeriodsSolution](./struct.PeriodsSolution.html)
//! struct, as in:
//! ```text
//! formula: "9.16 = log(250.0000 / 100.0000, base 2.718282) / 0.100000",
//! symbolic_formula: "n = log(fv / pv, base e) / r",
//! ```

// use log::warn;

use crate::tvm_simple::*;

// Needed for the Rustdoc comments.
#[allow(unused_imports)]
use crate::{future_value::future_value, present_value::present_value, rate::rate};
use std::ops::Deref;

/// A record of a periods calculation. The struct shows details such as the formula and can be used
/// to calculate the period-by-period details.
///
/// This is the result of a call to [periods_solution](fn.periods_solution.html) or [periods_continuous_solution](fn.periods_continuous_solution.html).
#[derive(Clone, Debug)]
pub struct PeriodsSolution {
    tvm_solution: TvmSolution,
}

/// The period-by-period details of a periods calculation showing the value at each step.
///
/// This is the result of a call to [PeriodsSolution::series](./struct.PeriodsSolution.html#method.series).
#[derive(Clone, Debug)]
pub struct PeriodsSeries(TvmSeries);

impl PeriodsSolution {
    fn new (tvm_solution: TvmSolution) -> Self {
        assert!(tvm_solution.calculated_field().is_periods());
        Self {
            tvm_solution,
        }
    }

    /// Returns the period-by-period details of a periods calculation showing the value at each
    /// step.
    pub fn series(&self) -> PeriodsSeries {
        // For a rate, periods, or future value calculation the the period-by-period values are
        // calculated the same way.
        PeriodsSeries::new(self.tvm_solution.series())
    }

    /// Prints the period-by-period details of a periods calculation using default formatting in
    /// which numbers appear similar to Rust constants.
    pub fn print_series_table(&self) {
        self.series().print_table();
    }

    /// Prints the period-by-period details of a periods calculation with formatting specified by
    /// a locale and the number of decimal places.
    pub fn print_series_table_locale(&self, locale: &num_format::Locale, precision: usize) {
        self.series().print_table_locale(locale, precision);
    }

    /// Returns true if the value is compounded continuously rather than once per period. This will
    /// be true if the solution was created with a call to [periods_continuous_solution](fn.periods_continuous_solution.html)
    /// and false if it was created by [periods_solution](fn.periods_solution.html).
    pub fn continuous_compounding(&self) -> bool {
        self.tvm_solution.continuous_compounding()
    }

    /// Returns the periodic rate that was given as an input to the periods calculation.
    pub fn rate(&self) -> f64 {
        self.tvm_solution.rate()
    }

    /// Returns the calculated number of periods as a whole number. It's possible that the
    /// calculated periods were not a whole number in which case [fractional_periods](./struct.PeriodsSolution.html#method.fractional_periods)
    /// will have the more precise value. To get the whole number of periods we round up.
    pub fn periods(&self) -> u32 {
        self.tvm_solution.periods()
    }

    /// Returns the calculated number of periods as a floating point number. To get a whole number
    /// of periods (rounded up) use [periods](./struct.PeriodsSolution.html#method.periods).
    pub fn fractional_periods(&self) -> f64 {
        self.tvm_solution.fractional_periods()
    }

    /// Returns the present value that was given as an input to the periods calculation.
    pub fn present_value(&self) -> f64 {
        self.tvm_solution.present_value()
    }

    /// Returns the future value that was given as an input to the periods calculation.
    pub fn future_value(&self) -> f64 {
        self.tvm_solution.future_value()
    }

    /// Returns a text version of the formula used to calculate the periods. The formula includes
    /// the actual values rather than variable names. For the formula with variables such as "r" for
    /// rate call [symbolic_formula](./struct.PeriodsSolution.html#method.symbolic_formula).
    pub fn formula(&self) -> &str {
        &self.tvm_solution.formula()
    }

    /// Returns a text version of the formula used to calculate the periods. The formula uses
    /// variables such as "r" for the rate. For the formula with the actual values rather than
    /// variables call [formula](./struct.PeriodsSolution.html#method.formula).
    pub fn symbolic_formula(&self) -> &str {
        &self.tvm_solution.symbolic_formula()
    }
}

impl TimeValueOfMoneySolution for PeriodsSolution {
    fn tvm_solution(&self) -> TvmSolution {
        self.tvm_solution.clone()
    }

    fn tvm_series(&self) -> TvmSeries {
        self.series().into()
    }
}

impl PeriodsSeries {
    fn new (series: TvmSeries) -> Self {
        Self {
            0: series,
        }
    }
}

impl Deref for PeriodsSeries {
    type Target = TvmSeries;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<TvmSeries> for PeriodsSeries {
    fn into(self) -> TvmSeries {
        self.0
    }
}

/// Returns the number of periods given a periodic rate along with the present and future values,
/// using simple compounding.
///
/// Note that the returned number of periods will be a floating point number representing fractional
/// periods.
///
/// See the [periods](./index.html) module page for the formulas.
///
/// Related functions:
/// * To calculate the periods using simple compounding and return a struct that shows the formula
/// and can be used to produce the the period-by-period values use [periods_solution](fn.periods_solution.html).
/// * To calculate the periods using continuous compounding use [periods_continuous](fn.periods_continuous.html)
/// or [periods_continuous_solution](fn.periods_continuous_solution.html).
///
/// # Arguments
/// * `rate` - The rate at which the investment grows or shrinks per period, expressed as a
/// floating point number. For instance 0.05 would mean 5% growth. Often appears as `r` or `i` in
/// formulas.
/// * `present_value` - The starting value of the investment. May appear as `pv` in formulas, or `C`
/// for cash flow or `P` for principal.
/// * `future_value` - The final value of the investment.
///
/// # Panics
/// The call will fail if the rate, the present value, or the future value is infinite or not a
/// number (NaN).
///
/// The call will also fail in any of the follwing cases because there is no number of periods that
/// would make the calculation work:
/// * The periodic rate is less than -1.0.
/// * The present value is zero and the future value is nonzero.
/// * The present value is nonzero and the future value is zero, unless the rate is exactly -1.0%.
/// * The present value is negative and the future value is positive or vice versa.
/// * The present value and future value are both negative, the future value is less than the
/// present value, and the periodic rate is zero or negative.
/// * The present value and future value are both negative, the future value is greater than the
/// present value, and the periodic rate is zero or positive.
/// * The present value and future value are both positive, the future value is greater than the
/// present value, and the periodic rate is zero or negative.
/// * The present value and future value are both positive, the future value is less than the
/// present value, and the periodic rate is zero or positive.
///
/// # Examples
/// ```
/// // The interest rate is 8% per year.
/// let rate = 0.08;
///
/// // The starting value is $5,000.00.
/// let present_value = 5_000.00;
///
/// // The ending value is $7,000.00.
/// let future_value = 7_000.00;
///
/// // Calculate the number of years required.
/// let fractional_periods = finance::periods(rate, present_value, future_value);
/// dbg!(&fractional_periods);
/// finance::assert_rounded_2(4.37, fractional_periods);
///
/// // Round up to get a whole number of years.
/// let periods = fractional_periods.ceil() as u32;
/// dbg!(&periods);
/// assert_eq!(5, periods);
/// ```
pub fn periods<P, F>(rate: f64, present_value: P, future_value: F) -> f64
    where
        P: Into<f64> + Copy,
        F: Into<f64> + Copy
{
    periods_internal(rate, present_value.into(), future_value.into(), false)
}

/// Calculates the number of periods given a periodic rate along with the present and future values
/// using simple compounding; and builds a struct with the input values, an explanation of the
/// formula, and the option to calculate the period-by-period values.
///
/// Note that the calculated number of periods from [PeriodsSolution::fractional_periods](./struct.PeriodsSolution.html#method.fractional_periods) field will
/// be a floating point number. To get the periods as a whole number (rounded up) use
/// [PeriodsSolution::periods](./struct.PeriodsSolution.html#method.periods).
///
/// See the [periods](./index.html) module page for the formulas.
///
/// Related functions:
/// * To calculate the periods as a single number with simple compounding use [periods](fn.periods.html).
/// * To calculate the periods using continuous compounding use [periods_continuous](fn.periods_continuous.html)
/// or [periods_continuous_solution](fn.periods_continuous_solution.html).
///
/// # Arguments
/// * `rate` - The rate at which the investment grows or shrinks per period, expressed as a
/// floating point number. For instance 0.05 would mean 5% growth. Often appears as `r` or `i` in
/// formulas.
/// * `present_value` - The starting value of the investment. May appear as `pv` in formulas, or `P`
/// for principal.
/// * `future_value` - The final value of the investment.
///
/// # Panics
/// The call will fail if the rate, the present value, or the future value is infinite or not a
/// number (NaN).
///
/// The call will also fail in any of the follwing cases because there is no number of periods that
/// would make the calculation work:
/// * The periodic rate is less than -1.0.
/// * The present value is zero and the future value is nonzero.
/// * The present value is nonzero and the future value is zero, unless the rate is exactly -1.0%.
/// * The present value is negative and the future value is positive or vice versa.
/// * The present value and future value are both negative, the future value is less than the
/// present value, and the periodic rate is zero or negative.
/// * The present value and future value are both negative, the future value is greater than the
/// present value, and the periodic rate is zero or positive.
/// * The present value and future value are both positive, the future value is greater than the
/// present value, and the periodic rate is zero or negative.
/// * The present value and future value are both positive, the future value is less than the
/// present value, and the periodic rate is zero or positive.
///
/// # Examples
/// ```
/// // The interest rate is 3.5% per quarter.
/// let rate = 0.035;
///
/// // The starting value is $100,000.00.
/// let present_value = 100_000.00;
///
/// // The ending value is $200,000.00.
/// let future_value = 200_000.00;
///
/// // Calculate the number of quarters required and build a struct with the
/// // input values, an explanation of the formula, and an option to calculate
/// // the quarter-by-quarter values.
/// let solution = finance::periods_solution(rate, present_value, future_value);
///
/// let fractional_quarters = solution.fractional_periods();
/// dbg!(&fractional_quarters);
/// finance::assert_rounded_2(20.15, fractional_quarters);
///
/// // Get the whole number of quarters.
/// let quarters = solution.periods();
/// dbg!(&quarters);
/// assert_eq!(21, quarters);
///
/// // Examine the formulas.
/// let formula = solution.formula();
/// dbg!(&formula);
/// assert_eq!("20.15 = log(200000.0000 / 100000.0000, base 1.035000)", formula);
/// let symbolic_formula = solution.symbolic_formula();
/// dbg!(&symbolic_formula);
/// assert_eq!("n = log(fv / pv, base (1 + r))", symbolic_formula);
///
/// let series = solution.series();
/// dbg!(&series);
///
/// let last_entry = series.last().unwrap();
/// dbg!(&last_entry);
/// finance::assert_rounded_4(200_000.0, last_entry.value());
///
/// // Create a reduced series with the value at the end of each year.
/// let filtered_series = series
///     .iter()
///     .filter(|x| x.period() % 4 == 0 && x.period() != 0)
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
/// finance::assert_rounded_2(3.61, solution.fractional_periods());
/// assert_eq!(4, solution.periods());
///
/// // Print the period-by-period values as a formatted table.
/// solution.print_series_table();
/// ```
pub fn periods_solution<P, F>(rate: f64, present_value: P, future_value: F) -> PeriodsSolution
    where
        P: Into<f64> + Copy,
        F: Into<f64> + Copy
{
    periods_solution_internal(rate, present_value.into(), future_value.into(), false)
}

/// Returns the number of periods given a periodic rate along with the present and future values,
/// using continuous compounding.
///
/// Note that the returned number of periods will be a floating point number representing fractional
/// periods.
///
/// See the [periods](./index.html) module page for the formulas.
///
/// Related functions:
/// * To calculate the periods using continuous compounding and return a struct that shows the
/// formula and can be used to produce the the period-by-period values use [periods_continuous_solution](fn.periods_continuous_solution.html).
/// * To calculate the periods using single compounding use [periods](fn.periods.html)
/// or [periods_solution](fn.periods_solution.html).
///
/// # Arguments
/// * `rate` - The rate at which the investment grows or shrinks per period, expressed as a
/// floating point number. For instance 0.05 would mean 5% growth. Often appears as `r` or `i` in
/// formulas.
/// * `present_value` - The starting value of the investment. May appear as `pv` in formulas, or `C`
/// for cash flow or `P` for principal.
/// * `future_value` - The final value of the investment.
///
/// # Panics
/// The call will fail if either of the rate, the present value, or the future value is
/// infinite or not a number (NaN).
///
/// The call will also fail in any of the follwing cases because there is no number of periods that
/// would make the calculation work:
/// * The periodic rate is less than -1.0.
/// * The present value is zero and the future value is nonzero.
/// * The present value is nonzero and the future value is zero, unless the rate is exactly -1.0%.
/// * The present value is negative and the future value is positive or vice versa.
/// * The present value and future value are both negative, the future value is less than the
/// present value, and the periodic rate is zero or negative.
/// * The present value and future value are both negative, the future value is greater than the
/// present value, and the periodic rate is zero or positive.
/// * The present value and future value are both positive, the future value is greater than the
/// present value, and the periodic rate is zero or negative.
/// * The present value and future value are both positive, the future value is less than the
/// present value, and the periodic rate is zero or positive.
///
/// # Examples
/// ```
/// // The interest rate is 11% per year.
/// let rate = 0.11;
///
/// // The starting value is $25,000.00.
/// let present_value = 25_000;
///
/// // The ending value is $45,000.00.
/// let future_value = 45_000;
///
/// // Calculate the number of years required if interest is compounded continuously.
/// let fractional_periods = finance::periods_continuous(rate, present_value, future_value);
/// dbg!(&fractional_periods);
/// finance::assert_rounded_2(5.34, fractional_periods);
/// ```
pub fn periods_continuous<P, F>(rate: f64, present_value: P, future_value: F) -> f64
    where
        P: Into<f64> + Copy,
        F: Into<f64> + Copy
{
    periods_internal(rate, present_value.into(), future_value.into(), true)
}

/// Calculates the number of periods given a periodic rate along with the present and future values
/// using continuous compounding. Returns a struct with details and with a method for calculating
/// the period-by-period details.
///
/// Note that the returned number of periods will be a floating point number representing fractional
/// periods.
///
/// See the [periods](./index.html) module page for the formulas.
///
/// Related functions:
/// * To calculate the periods as a single number using continuous compounding use [periods_continuous](fn.periods_continuous.html).
/// * To calculate the periods using single compounding use [periods](fn.periods.html)
/// or [periods_solution](fn.periods_solution.html).
///
/// # Arguments
/// * `rate` - The rate at which the investment grows or shrinks per period, expressed as a
/// floating point number. For instance 0.05 would mean 5% growth. Often appears as `r` or `i` in
/// formulas.
/// * `present_value` - The starting value of the investment. May appear as `pv` in formulas, or `C`
/// for cash flow or `P` for principal.
/// * `future_value` - The final value of the investment.
///
/// # Panics
/// The call will fail if either of the rate, the present value, or the future value is
/// infinite or not a number (NaN).
///
/// The call will also fail in any of the follwing cases because there is no number of periods that
/// would make the calculation work:
/// * The periodic rate is less than -1.0.
/// * The present value is zero and the future value is nonzero.
/// * The present value is nonzero and the future value is zero, unless the rate is exactly -1.0%.
/// * The present value is negative and the future value is positive or vice versa.
/// * The present value and future value are both negative, the future value is less than the
/// present value, and the periodic rate is zero or negative.
/// * The present value and future value are both negative, the future value is greater than the
/// present value, and the periodic rate is zero or positive.
/// * The present value and future value are both positive, the future value is greater than the
/// present value, and the periodic rate is zero or negative.
/// * The present value and future value are both positive, the future value is less than the
/// present value, and the periodic rate is zero or positive.
///
/// # Examples
/// ```
/// // The interest rate is 8.5% per year.
/// let rate = 0.085;
///
/// // The starting value is $10,000.00.
/// let present_value = 10_000;
///
/// // The ending value is $30,000.00.
/// let future_value = 30_000;
///
/// // Calculate the number of years required if interest is compounded continuously returning a
/// // struct that cat be used for further operations.
/// let solution = finance::periods_continuous_solution(rate, present_value, future_value);
/// dbg!(&solution);
/// finance::assert_rounded_2(12.92, solution.fractional_periods());
///
/// // Print the period-by-period details in a formatted table.
/// solution.print_series_table();
///
/// // Create a list of period-by-period details.
/// let series = solution.series();
/// ```
pub fn periods_continuous_solution<P, F>(rate: f64, present_value: P, future_value: F) -> PeriodsSolution
    where
        P: Into<f64> + Copy,
        F: Into<f64> + Copy
{
    periods_solution_internal(rate, present_value.into(), future_value.into(), true)
}

pub(crate) fn periods_internal(rate: f64, present_value: f64, future_value: f64, continuous_compounding: bool) -> f64 {
    if present_value == future_value {
        // This is a special case that doesn't require us to check the parameters and which covers
        // the case where both are zero.
        return 0.0;
    }
    if future_value == 0.0 && rate == -1.0 {
        // This is a special case that we can't run through the log function. Since the rate is
        // -100%, given any present value the future value will be zero and it will take only one
        // period to get there.
        // We already know that the present value is nonzero because that case would have been
        // caught above.
        assert!(present_value != 0.0);
        return 1.0;
    }

    check_periods_parameters(rate, present_value, future_value);

    let fractional_periods = if continuous_compounding {
        // http://www.edmichaelreggie.com/TMVContent/rate.htm
        (future_value / present_value).log(std::f64::consts::E) / rate
    } else {
        (future_value / present_value).log(1.0 + rate)
    };
    assert!(fractional_periods >= 0.0);
    fractional_periods
}

pub(crate) fn periods_solution_internal(rate: f64, present_value: f64, future_value: f64, continuous_compounding: bool) -> PeriodsSolution {
    let fractional_periods = periods_internal(rate, present_value, future_value, continuous_compounding);
    assert!(fractional_periods >= 0.0);
    let (formula, symbolic_formula) = if continuous_compounding {
        let formula = format!("{:.2} = log({:.4} / {:.4}, base {:.6}) / {:.6}", fractional_periods, future_value, present_value, std::f64::consts::E, rate);
        let symbolic_formula = "n = log(fv / pv, base e) / r";
        (formula, symbolic_formula)
    } else {
        let rate_multiplier = 1.0 + rate;
        let formula = format!("{:.2} = log({:.4} / {:.4}, base {:.6})", fractional_periods, future_value, present_value, rate_multiplier);
        let symbolic_formula = "n = log(fv / pv, base (1 + r))";
        (formula, symbolic_formula)
    };
    PeriodsSolution::new(TvmSolution::new_fractional_periods(TvmVariable::Periods,continuous_compounding, rate, fractional_periods, present_value, future_value, &formula, symbolic_formula))
}

fn check_periods_parameters(rate: f64, present_value: f64, future_value: f64) {
    assert!(rate.is_finite(), "The rate must be finite (not NaN or infinity)");
    assert!(present_value.is_finite(), "The present value must be finite (not NaN or infinity)");
    assert!(future_value.is_finite(), "The future value must be finite (not NaN or infinity)");
    assert!(rate >= -1.0, "The rate must be greater than or equal to -1.0 because a rate lower than -100% would mean the investment loses more than its full value in a period.");
    assert!(!(present_value == 0.0 && future_value != 0.0), "The present value is zero and the future value is nonzero so there's no way to solve for the number of periods.");
    assert!(!(present_value != 0.0 && future_value == 0.0 && rate != -1.0), "The present value is nonzero, the future value is zero, and the rate is not -100% so there's no way to solve for the number of periods.");
    assert!(!(present_value < 0.0 && future_value > 0.0), "The present value is negative and the future value is positive so there's no way to solve for the number of periods.");
    assert!(!(present_value > 0.0 && future_value < 0.0), "The present value is positive and the future value is negative so there's no way to solve for the number of periods.");
    assert!(!(present_value < 0.0 && future_value < present_value && rate <= 0.0), "The present value and future value are both negative, the future value is less than the present value, and the periodic rate is zero or negative. There's no way to solve for the number of periods.");
    assert!(!(present_value < 0.0 && future_value > present_value && rate >= 0.0), "The present value and future value are both negative, the future value is greater than the present value, and the periodic rate is zero or positive. There's no way to solve for the number of periods.");
    assert!(!(present_value > 0.0 && future_value > present_value && rate <= 0.0), "The present value and future value are both positive, the future value is greater than the present value, and the periodic rate is zero or negative. There's no way to solve for the number of periods.");
    assert!(!(present_value > 0.0 && future_value < present_value && rate >= 0.0), "The present value and future value are both positive, the future value is less than the present value, and the periodic rate is zero or positive. There's no way to solve for the number of periods.");
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



