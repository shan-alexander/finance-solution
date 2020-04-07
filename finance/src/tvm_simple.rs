use std::fmt::Debug;
use std::fmt;

#[derive(Debug)]
pub enum TvmVariable {
    Rate,
    Periods,
    PresentValue,
    FutureValue,
}

impl TvmVariable {
    /*
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
    */

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
    pub present_value: f64,
    pub future_value: f64,
    pub formula: String,
}

impl TvmSolution {
    pub(crate) fn new(calculated_field: TvmVariable, periodic_rate: f64, periods: u32, present_value: f64, future_value: f64, formula: &str) -> Self {
        Self {
            calculated_field,
            periodic_rate,
            periods,
            present_value,
            future_value,
            formula: formula.to_string(),
        }
    }

    pub fn series(&self) -> Vec<TvmPeriod>
    {
        let mut series = vec![];
        // Add the values at each period.
        if self.calculated_field.is_present_value() {
            let mut prev_value = None;
            for period in (0..=self.periods).rev() {
                let (value, formula) = if period == self.periods {
                    (self.future_value, "future_value".to_string())
                } else {
                    (prev_value.unwrap() / (1.0 + self.periodic_rate), format!("{:.4} / (1 + {:.6})", prev_value.unwrap(), self.periodic_rate))
                };
                assert!(value.is_finite());
                prev_value = Some(value);
                series.insert(0, TvmPeriod::new(period, self.periodic_rate, value, &formula))
            };
        } else if self.calculated_field.is_future_value() {
            let mut prev_value = None;
            for period in 0..=self.periods {
                let (value, formula) = if period == 0 {
                    (self.present_value, "present_value".to_string())
                } else {
                    (prev_value.unwrap() * (1.0 + self.periodic_rate), format!("{:.4} * (1 + {:.6})", prev_value.unwrap(), self.periodic_rate))
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
        write!(f, "{{ {}, {}, {}, {}, {}, {} }}",
               &format!("calculated_field: {:?}", self.calculated_field),
               &format!("periodic_rate: {:.6}", self.periodic_rate),
               &format!("periods: {}", self.periods),
               &format!("present_value: {:.4}", self.present_value),
               &format!("future_value: {:.4}", self.future_value),
               &format!("formula: {:?}", self.formula),
        )
    }
}

/// A record of a Time Value of Money calculation where the rate may vary by period.
/// 
/// It's the result of calling [`rate_schedule`], [`periods_schedule`], [`present_value_schedule`],
/// or [`future_value_schedule`].
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
            let mut prev_value = None;
            for period in (0..=self.periods).rev() {
                let (value, formula, rate) = if period == self.periods {
                    (self.future_value, "future_value".to_string(), 0.0)
                } else {
                    // We want the rate for the period after this one. Since periods are 1-based and
                    // the vector of rates is 0-based, this means using the current period number as
                    // the index into the vector.
                    let rate = self.periodic_rates[period as usize];
                    (prev_value.unwrap() / (1.0 + rate), format!("{:.4} / (1 + {:.6})", prev_value.unwrap(), rate), rate)
                };
                assert!(value.is_finite());
                prev_value = Some(value);
                series.insert(0, TvmPeriod::new(period, rate, value, &formula))
            };
        } else if self.calculated_field.is_future_value() {
            let mut prev_value = None;
            for period in 0..=self.periods {
                let (value, formula, rate) = if period == 0 {
                    (self.present_value, "present_value".to_string(), 0.0)
                } else {
                    // We want the rate for the current period. However, periods are 1-based and
                    // the vector of rates is 0-based, so the corresponding rate is at period - 1.
                    let rate = self.periodic_rates[period as usize - 1];
                    (prev_value.unwrap() * (1.0 + rate), format!("{:.4} * (1 + {:.6})", prev_value.unwrap(), rate), rate)
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
/// * Part of [`PvmSolution`] produced by calling [`rate_solution`], [`periods_solution`],
/// [`present_value_solution`], or [`future_value_solution`].
/// * Part of [`PvmSchedule`] produced by calling [`rate_schedule`], [`periods_schedule`],
/// [`present_value_schedule`], or [`future_value_schedule`]
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


