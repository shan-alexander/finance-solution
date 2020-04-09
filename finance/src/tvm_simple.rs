use std::fmt::Debug;
use std::fmt;

// Import needed for the function references in the Rustdoc comments.
#[allow(unused_imports)]
use crate::*;

#[derive(Debug)]
pub enum TvmVariable {
    Rate,
    Periods,
    PresentValue,
    FutureValue,
}

impl TvmVariable {
    fn is_rate(&self) -> bool {
        match self {
            TvmVariable::Rate => true,
            _ => false,
        }
    }

    fn is_periods(&self) -> bool {
        match self {
            TvmVariable::Periods => true,
            _ => false,
        }
    }

    fn is_present_value(&self) -> bool {
        match self {
            TvmVariable::PresentValue => true,
            _ => false,
        }
    }

    fn is_future_value(&self) -> bool {
        match self {
            TvmVariable::FutureValue => true,
            _ => false,
        }
    }
}

/// A record of a Time Value of Money calculation where the rate is the same for every period.
/// 
/// It's the result of calling [`rate_solution`], [`periods_solution`], [`present_value_solution`],
/// or [`future_value_solution`].
pub struct TvmSolution {
    pub calculated_field: TvmVariable,
    pub periodic_rate: f64,
    pub periods: u32,
    pub fractional_periods: f64,
    pub present_value: f64,
    pub future_value: f64,
    pub formula: String,
}

impl TvmSolution {
    pub(crate) fn new(calculated_field: TvmVariable, periodic_rate: f64, periods: u32, present_value: f64, future_value: f64, formula: &str) -> Self {
        Self::new_fractional_periods(calculated_field, periodic_rate, periods, periods as f64, present_value, future_value, formula)
    }

    pub(crate) fn new_fractional_periods(calculated_field: TvmVariable, periodic_rate: f64, periods: u32, fractional_periods: f64, present_value: f64, future_value: f64, formula: &str) -> Self {
        assert!(periodic_rate >= -1.0);
        assert!(fractional_periods >= 0.0);
        Self {
            calculated_field,
            periodic_rate,
            periods,
            fractional_periods,
            present_value,
            future_value,
            formula: formula.to_string(),
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
    ///     .filter(|x| x.period % 4 == 0)
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
    /// finance::assert_rounded_4(62534.3257, solution.future_value);
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
    /// finance::assert_rounded_4(solution.future_value, series.last().unwrap().value);
    ///
    /// // Find the first period where the value of the investment was at least
    /// // $60,000.
    /// let period = series.iter().find(|x| x.value >= 60_000.00);
    /// dbg!(&period);
    /// assert_eq!(2, period.unwrap().period);
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
    ///     .filter(|x| x.period % 2 == 0 && x.period != 0)
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
    ///     .filter(|x| x.rate < 0.0)
    ///     .collect::<Vec<_>>();
    /// dbg!(&filtered_series);
    /// assert_eq!(2, filtered_series.len());
    /// ```
    pub fn series(&self) -> Vec<TvmPeriod>
    {
        let rate_multiplier = 1.0 + self.periodic_rate;
        assert!(rate_multiplier >= 0.0);
        let mut series = vec![];
        // Add the values at each period.
        if self.calculated_field.is_present_value() {
            let mut prev_value = None;
            for period in (0..=self.periods).rev() {
                let (value, formula) = if period == self.periods {
                    (self.future_value, format!("{:.4}", self.future_value))
                } else {
                    (prev_value.unwrap() / rate_multiplier, format!("{:.4} / {:.6}", prev_value.unwrap(), rate_multiplier))
                };
                assert!(value.is_finite());
                prev_value = Some(value);
                series.insert(0, TvmPeriod::new(period, self.periodic_rate, value, &formula))
            };
        } else if self.calculated_field.is_rate() || self.calculated_field.is_periods() || self.calculated_field.is_future_value() {
            let mut prev_value = None;
            for period in 0..=self.periods {
                let (value, formula) = if period == 0 {
                    (self.present_value, format!("{:.4}", self.present_value))
                } else {
                    (prev_value.unwrap() * rate_multiplier, format!("{:.4} * {:.6}", prev_value.unwrap(), rate_multiplier))
                };
                assert!(value.is_finite());
                prev_value = Some(value);
                series.push(TvmPeriod::new(period, self.periodic_rate, value, &formula))
            };
        }
        series
    }
}

impl Debug for TvmSolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ {}, {}, {}, {}, {}, {}, {} }}",
               &format!("calculated_field: {:?}", self.calculated_field),
               &format!("periodic_rate: {:.6}", self.periodic_rate),
               &format!("periods: {}", self.periods),
               &format!("fractional_periods: {:.2}", self.fractional_periods),
               &format!("present_value: {:.4}", self.present_value),
               &format!("future_value: {:.4}", self.future_value),
               &format!("formula: {:?}", self.formula),
        )
    }
}

/// A record of a Time Value of Money calculation where the rate may vary by period.
/// 
/// It's the result of calling [`present_value_schedule`] or [`future_value_schedule`].
#[derive(Debug)]
pub struct TvmSchedule {
    pub calculated_field: TvmVariable,
    pub periodic_rates: Vec<f64>,
    pub periods: u32,
    pub present_value: f64,
    pub future_value: f64,
}

impl TvmSchedule {
    pub(crate) fn new(calculated_field: TvmVariable, periodic_rates: &[f64], present_value: f64, future_value: f64) -> Self {
        Self {
            calculated_field,
            periodic_rates: periodic_rates.to_vec(),
            periods: periodic_rates.len() as u32,
            present_value,
            future_value,
        }
    }

    pub fn series(&self) -> Vec<TvmPeriod>
    {
        let mut series = vec![];
        // Add the values at each period.
        if self.calculated_field.is_present_value() {
            let mut next_value = None;
            for period in (0..=self.periods).rev() {
                let (value, formula, rate) = if period == self.periods {
                    (self.future_value, format!("{:.4}", self.future_value), self.periodic_rates[(period - 1) as usize])
                } else {
                    // We want the rate for the period after this one. Since periods are 1-based and
                    // the vector of rates is 0-based, this means using the current period number as
                    // the index into the vector.
                    let next_rate = self.periodic_rates[period as usize];
                    assert!(next_rate >= -1.0);
                    let next_rate_multiplier = 1.0 + next_rate;
                    assert!(next_rate_multiplier >= 0.0);
                    // While we are going to divide by the rate of the next period, the rate that
                    // will appear in the TvmPeriod struct is the rate for the current period.
                    let rate = if period == 0 {
                        0.0
                    } else {
                        self.periodic_rates[(period - 1) as usize]
                    };
                    (next_value.unwrap() / next_rate_multiplier, format!("{:.4} / {:.6}", next_value.unwrap(), next_rate_multiplier), rate)
                };
                assert!(value.is_finite());
                next_value = Some(value);
                series.insert(0, TvmPeriod::new(period, rate, value, &formula))
            };
        } else if self.calculated_field.is_future_value() {
            let mut prev_value = None;
            for period in 0..=self.periods {
                let (value, formula, rate) = if period == 0 {
                    (self.present_value, format!("{:.4}", self.present_value), 0.0)
                } else {
                    // We want the rate for the current period. However, periods are 1-based and
                    // the vector of rates is 0-based, so the corresponding rate is at period - 1.
                    let rate = self.periodic_rates[period as usize - 1];
                    assert!(rate >= -1.0);
                    let rate_multiplier = 1.0 + rate;
                    assert!(rate_multiplier >= 0.0);
                    (prev_value.unwrap() * rate_multiplier, format!("{:.4} * {:.6}", prev_value.unwrap(), rate_multiplier), rate)
                };
                assert!(value.is_finite());
                prev_value = Some(value);
                series.push(TvmPeriod::new(period, rate, value, &formula))
            };
        }
        series
    }
}

/// The value of an investment at the end of a given period, part of a Time Value of Money
/// calculation.
///
/// This is either:
/// * Part of PvmSolution produced by calling [`rate_solution`], [`periods_solution`],
/// [`present_value_solution`], or [`future_value_solution`].
/// * Part of PvmSchedule produced by calling [`present_value_schedule`] or
/// [`future_value_schedule`].
pub struct TvmPeriod {
    pub period: u32,
    pub rate: f64,
    pub value: f64,
    pub formula: String,
}

impl TvmPeriod {
    pub(crate) fn new(period: u32, rate: f64, value: f64, formula: &str) -> Self {
        Self {
            period,
            rate,
            value,
            formula: formula.to_string() }
    }
}

impl Debug for TvmPeriod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ {}, {}, {}, {} }}",
               &format!("period: {}", self.period),
               &format!("periodic_rate: {:.6}", self.rate),
               &format!("value: {:.4}", self.value),
               &format!("formula: {:?}", self.formula),
        )
    }
}
