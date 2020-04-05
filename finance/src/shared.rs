use std::fmt::Debug;
use std::fmt;




/// The value of an investment at the end of a given period.
///
/// This is either:
/// * Part of [`FutureValueSchedule`] produced by calling [`future_value_schedule`]
/// * Part of [`PresentValueSchedule`] produced by calling [`present_value_schedule`]
pub struct SchedulePeriod {
    pub period: u32,
    pub periodic_rate: f64,
    pub value: f64,
}

impl SchedulePeriod {
    pub(crate) fn new(period: u32, periodic_rate: f64, value: f64) -> Self {
        Self { period, periodic_rate, value }
    }
}

impl Debug for SchedulePeriod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ {}, {}, {} }}",
               &format!("period: {}", self.period),
               &format!("periodic_rate: {:.6}", self.periodic_rate),
               &format!("value: {:.4}", self.value),
        )
    }
}


