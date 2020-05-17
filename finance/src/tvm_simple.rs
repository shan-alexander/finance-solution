//! The internal module which supports the solution struct for the family of Time-value-of-money equations
//! which do not involve payments. For example, future value, present value, rate, and periods.
#[allow(unused_imports)]

// use std::fmt::Debug;
// use std::fmt;

// Import needed for the function references in the Rustdoc comments.

use crate::*;
use std::ops::Deref;
use std::fmt::{Display, Formatter, Error};

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
/// It's the result of calling [`present_value_schedule`] or [`future_value_schedule`].
#[derive(Clone, Debug)]
pub struct TvmSchedule {
    calculated_field: TvmVariable,
    rates: Vec<f64>,
    periods: u32,
    present_value: f64,
    future_value: f64,
}

#[derive(Clone, Debug)]
pub struct TvmSeries(Vec<TvmPeriod>);

pub trait TimeValueOfMoneySolution {

    /// Returns the shared inner structure that holds fields common to any `TimeValueOfMoneySolution`
    /// struct such as [RateSolution](././rate/struct.RateSolution.html) or [PresentValueSolution](././present_value/struct.PresentValueSolution.html).
    /// # Examples
    /// Compare equivalent solutions calculated in different ways. The calculations are done using
    /// [rate_solution](././fn.rate_solution.html), [periods_solution](././fn.periods_solution.html),
    /// [present_value_solution](././fn.present_value_solution.html), and
    /// [future_value_solution](././fn.future_value_solution.html).
    /// ```
    /// use finance::TimeValueOfMoneySolution;
    ///
    /// // Set up inputs for a variety of calculations that should all be equivalent.
    /// let rate = 0.015;
    /// let periods = 24;
    /// let present_value = 10_000.0;
    /// let future_value = finance::future_value(rate, periods, present_value);
    ///
    /// // Create a list of solution structs. For simplicity, instead of holding references to a
    /// // `RateSolution`, a `PeriodsSolution`, and so on turn each result into a more general
    /// // `TvmSolution`.
    /// let list = vec![
    ///     finance::rate_solution(periods, present_value, future_value).tvm_solution(),
    ///     finance::periods_solution(rate, present_value, future_value).tvm_solution(),
    ///     finance::present_value_solution(rate, periods, future_value).tvm_solution(),
    ///     finance::future_value_solution(rate, periods, present_value).tvm_solution(),
    /// ];
    /// dbg!(&list);
    ///
    /// // Print the formulas used by the four types of calculations.
    /// for solution in list.iter() {
    ///     println!("{}:\n\t{}\n\t{}", solution.calculated_field(), solution.symbolic_formula(), solution.formula());
    /// }
    ///
    /// // An alternative would be to create a vector of references to the solutions.
    /// let _list: Vec<& dyn TimeValueOfMoneySolution> = vec![
    ///     &finance::rate_solution(periods, present_value, future_value),
    ///     &finance::periods_solution(rate, present_value, future_value),
    ///     &finance::present_value_solution(rate, periods, future_value),
    ///     &finance::future_value_solution(rate, periods, present_value),
    /// ];
    /// ```
    fn tvm_solution(&self) -> TvmSolution;

    /// Returns the period-by-period details of this calculation in a form that's shared between
    /// `TimeValueOfMoneySolution` structs such as [PeriodsSolution](././periods/struct.PeriodsSolution.html) and [FutureValueSolution](././future_value/struct.FutureValueSolution.html).
    fn tvm_series(&self) -> TvmSeries;

    fn tvm_solution_and_series(&self) -> (TvmSolution, TvmSeries) {
        (self.tvm_solution(), self.tvm_series())
    }

    fn rate_solution(&self, continuous_compounding: bool, compounding_periods: Option<u32>) -> RateSolution {
        self.tvm_solution().rate_solution(continuous_compounding, compounding_periods)
    }

    fn periods_solution(&self, continuous_compounding: bool) -> PeriodsSolution {
        self.tvm_solution().periods_solution(continuous_compounding)
    }

    fn present_value_solution(&self, continuous_compounding: bool, compounding_periods: Option<u32>) -> PresentValueSolution {
        self.tvm_solution().present_value_solution(continuous_compounding, compounding_periods)
    }

    fn future_value_solution(&self, continuous_compounding: bool, compounding_periods: Option<u32>) -> FutureValueSolution {
        self.tvm_solution().future_value_solution(continuous_compounding, compounding_periods)
    }

    fn present_value_vary_compounding_periods(&self, compounding_periods: &[u32]) -> Vec<(u32, f64)> {
        self.tvm_solution().present_value_vary_compounding_periods(compounding_periods)
    }

    fn future_value_vary_compounding_periods(&self, compounding_periods: &[u32]) -> Vec<(u32, f64)> {
        self.tvm_solution().future_value_vary_compounding_periods(compounding_periods)
    }

    fn print_ab_comparison(
        &self,
        other: &dyn TimeValueOfMoneySolution)
    {
        self.tvm_solution().print_ab_comparison(&other.tvm_solution());
    }

    fn print_ab_comparison_locale(
        &self,
        other: &dyn TimeValueOfMoneySolution,
        locale: &num_format::Locale,
        precision: usize)
    {
        self.tvm_solution().print_ab_comparison_locale(&other.tvm_solution(), locale, precision);
    }
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
        assert!(formula.len() > 0);
        assert!(symbolic_formula.len() > 0);
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
    pub(crate) fn series(&self) -> TvmSeries {
        // For a rate, periods, or future value calculation the the period-by-period values are
        // calculated the same way, starting with the present value and multiplying the value by
        // (1 + rate) for each period. The only nuance is that if we got here from a periods
        // calculation the last period may not be a full one, so there is some special handling of
        // the formulas and values.
        assert!(self.calculated_field.is_rate() || self.calculated_field.is_periods() || self.calculated_field.is_future_value());

        let rate_multiplier = 1.0 + self.rate;
        assert!(rate_multiplier >= 0.0);

        // For each period after 0, prev_value will hold the value of the previous period.
        let mut prev_value = None;

        let mut series = vec![];

        // Add the values at each period.
        for period in 0..=self.periods {
            let one_rate = if period == 0 {
                0.0
            } else {
                self.rate
            };
            let (value, formula, symbolic_formula) = if period == 0 {
                let value = self.present_value;
                let formula = format!("{:.4}", value);
                let symbolic_formula = "value = pv";
                (value, formula, symbolic_formula)
            } else {
                if self.calculated_field.is_periods() && period == self.periods {
                    // We calculated periods and this may not be a whole number, so for the last
                    // period use the future value. If instead we multiplied the previous
                    // period's value by (1 + rate) we could overshoot the future value.
                    let value = self.future_value;
                    let formula = format!("{:.4}", value);
                    let symbolic_formula = "value = fv";
                    (value, formula, symbolic_formula)
                } else {
                    // The usual case.
                    if self.continuous_compounding() {
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
                }
            };
            assert!(value.is_finite());
            prev_value = Some(value);
            series.push(TvmPeriod::new(period, one_rate, value, &formula, symbolic_formula))
        }
        TvmSeries::new(series)
    }

    pub fn print_series_table(&self) {
        self.series().print_table();
    }

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
    /// use finance::TimeValueOfMoneySolution;
    /// let solution = finance::future_value_solution(0.05, 12, 25_000);
    /// assert!(solution.tvm_solution().calculated_field().is_future_value());
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
    /// formula with variables such as r for rate call `symbolic_formula`.
    pub fn formula(&self) -> &str {
        &self.formula
    }

    /// Returns a text version of the formula used to calculate the result which may have been the
    /// periodic rate, number of periods, present value, or future value depending on which function
    /// was called. The formula uses variables such as n for the number of periods. For the formula
    /// with the actual values rather than variables call `formula`.
    pub fn symbolic_formula(&self) -> &str {
        &self.symbolic_formula
    }
    
    pub fn rate_solution(&self, continuous_compounding: bool, compounding_periods: Option<u32>) -> RateSolution {
        let periods= compounding_periods.unwrap_or(self.periods);
        rate_solution_internal(periods, self.present_value, self.future_value, continuous_compounding)
    }

    pub fn periods_solution(&self, continuous_compounding: bool) -> PeriodsSolution {
        periods_solution_internal(self.rate, self.present_value, self.future_value, continuous_compounding)
    }

    pub fn present_value_solution(&self, continuous_compounding: bool, compounding_periods: Option<u32>) -> PresentValueSolution {
        let (rate, periods) = match compounding_periods {
            Some(periods) => ((self.rate * self.fractional_periods) / periods as f64, periods as f64),
            None => (self.rate, self.fractional_periods),
        };
        present_value_solution_internal(rate, periods, self.future_value, continuous_compounding)
    }

    pub fn future_value_solution(&self, continuous_compounding: bool, compounding_periods: Option<u32>) -> FutureValueSolution {
        let (rate, periods) = match compounding_periods {
            Some(periods) => ((self.rate * self.fractional_periods) / periods as f64, periods as f64),
            None => (self.rate, self.fractional_periods),
        };
        future_value_solution_internal(rate, periods, self.present_value, continuous_compounding)
    }

    pub fn present_value_vary_compounding_periods(&self, compounding_periods: &[u32]) -> Vec<(u32, f64)> {
        compounding_periods.iter()
            .map(|periods| {
                let rate = (self.rate * self.fractional_periods) / *periods as f64;
                (*periods, present_value_internal(rate, *periods as f64, self.future_value, self.continuous_compounding))
            })
            .collect()
    }

    pub fn future_value_vary_compounding_periods(&self, compounding_periods: &[u32]) -> Vec<(u32, f64)> {
        compounding_periods.iter()
            .map(|periods| {
                let rate = (self.rate * self.fractional_periods) / *periods as f64;
                (*periods, future_value_internal(rate, *periods as f64, self.present_value, self.continuous_compounding))
            })
            .collect()
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

impl TvmSchedule {
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
    /// let solution = finance::present_value_schedule_solution(&[0.011, 0.012, 0.009], 75_000);
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
    /// let solution = finance::future_value_schedule_solution(&[0.05, 0.07, 0.05], 100_000);
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
            0: self.iter().filter(|x| predicate(x)).map(|x| x.clone()).collect()
        }
    }

    pub fn print_table(&self) {
        self.print_table_locale_opt(None, None);
    }

    pub fn print_table_locale(&self, locale: &num_format::Locale, precision: usize) {
        self.print_table_locale_opt(Some(locale), Some(precision));
    }

    fn print_table_locale_opt(&self, locale: Option<&num_format::Locale>, precision: Option<usize>) {
        let columns = vec![("period", "i", true), ("rate", "r", true), ("value", "f", true)];
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
        let columns = vec![("period", "i", true),
                           ("rate_a", "r", true), ("rate_b", "r", true),
                           ("value_a", "f", true), ("value_b", "f", true)];
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

impl Deref for TvmSeries {
    type Target = Vec<TvmPeriod>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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

impl TvmPeriod {
    pub(crate) fn new(period: u32, rate: f64, value: f64, formula: &str, symbolic_formula: &str) -> Self {
        assert!(rate.is_finite());
        assert!(value.is_finite());
        assert!(formula.len() > 0);
        assert!(symbolic_formula.len() > 0);
        Self {
            period,
            rate,
            value,
            formula: formula.to_string(),
            symbolic_formula: symbolic_formula.to_string()
        }
    }

    /// Returns the period number. The first real period is 1 but there's also a period 0 which
    /// which shows the starting conditions.
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

fn round_fractional_periods(fractional_periods: f64) -> u32 {
    round_4(fractional_periods).ceil() as u32
}
