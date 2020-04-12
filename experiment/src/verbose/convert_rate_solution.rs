use std::fmt::Debug;
use std::fmt;
use colored::*;


// Import needed for the function references in the Rustdoc comments.
#[allow(unused_imports)]
use crate::*;

#[derive(Debug, Clone)]
pub enum ConvertRateVariable {
    AprToEar,
    EarToApr,
    AprToEpr,
    EprToApr,
    EarToEpr,
    EprToEar,
}
impl ConvertRateVariable {
    pub fn is_apr_to_ear(&self) -> bool {
        match self {
            ConvertRateVariable::AprToEar => true,
            _ => false,
        }
    }

    pub fn is_ear_to_apr(&self) -> bool {
        match self {
            ConvertRateVariable::EarToApr => true,
            _ => false,
        }
    }

    pub fn is_apr_to_epr(&self) -> bool {
        match self {
            ConvertRateVariable::AprToEpr => true,
            _ => false,
        }
    }

    pub fn is_epr_to_apr(&self) -> bool {
        match self {
            ConvertRateVariable::EprToApr => true,
            _ => false,
        }
    }
    pub fn is_ear_to_epr(&self) -> bool {
        match self {
            ConvertRateVariable::EarToEpr => true,
            _ => false,
        }
    }
    pub fn is_epr_to_ear(&self) -> bool {
        match self {
            ConvertRateVariable::EprToEar => true,
            _ => false,
        }
    }
}
impl fmt::Display for ConvertRateVariable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           ConvertRateVariable::AprToEar => write!(f, "AprToEar"),
           ConvertRateVariable::EarToApr => write!(f, "EarToApr"),
           ConvertRateVariable::AprToEpr => write!(f, "AprToEpr"),
           ConvertRateVariable::EprToApr => write!(f, "EprToApr"),
           ConvertRateVariable::EarToEpr => write!(f, "EarToEpr"),
           ConvertRateVariable::EprToEar => write!(f, "EprToEar"),
       }
    }
}

// #[derive(Debug)]
pub struct ConvertRateSolution {
    pub calculated_field: ConvertRateVariable,
    pub input_rate: f64,
    pub num_periods_in_year: u32,
    pub output_rate: f64,
    pub input_in_percent: String,
    pub output_in_percent: String,
    pub formula: String,
    pub apr: f64,
    pub epr: f64,
    pub ear: f64,
}
impl ConvertRateSolution {
    pub(crate) fn new(calculated_field: ConvertRateVariable, input_rate: f64, num_periods_in_year: u32, output_rate: f64, input_in_percent: String, output_in_percent: String, formula: &str, apr:f64, epr:f64, ear:f64) -> Self {
        assert!(input_rate.is_finite());
        assert!(input_rate.is_finite());
        assert!(formula.len() > 0);
        Self {
            calculated_field,
            input_rate,
            num_periods_in_year,
            output_rate,
            input_in_percent,
            output_in_percent,
            formula: formula.to_string(),
            apr,
            epr,
            ear,
        }
    }
}


impl Debug for ConvertRateSolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{{\n {},\n {},\n {},\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n }}",
               &format!("calculated_field: {}", self.calculated_field.to_string().magenta()),
               &format!("input_rate: {}", self.input_rate.to_string().yellow()),
               &format!("num_periods_in_year: {:.4}", self.num_periods_in_year.to_string().yellow()),
               &format!("output_rate: {}", self.output_rate.to_string().green()),
               &format!("input_in_percent: {:.6}%", self.input_in_percent),
               &format!("output_in_percent: {:.6}{}", self.output_in_percent.to_string().green(),"%".green()),
               &format!("formula: {:?}", self.formula),
               &format!("{}: {:?}", if self.calculated_field.is_ear_to_apr() || self.calculated_field.is_epr_to_apr() { "apr".green()} else if self.calculated_field.is_apr_to_epr() || self.calculated_field.is_apr_to_ear() { "apr".yellow() } else { "apr".normal() }, self.apr),
               &format!("{}: {:?}", if self.calculated_field.is_ear_to_epr() || self.calculated_field.is_apr_to_epr() { "epr".green()} else if self.calculated_field.is_epr_to_apr() || self.calculated_field.is_epr_to_ear() { "epr".yellow() } else { "epr".normal() }, self.epr),
               &format!("{}: {:?}", if self.calculated_field.is_apr_to_ear() || self.calculated_field.is_epr_to_ear() { "ear".green()} else if self.calculated_field.is_ear_to_apr() || self.calculated_field.is_ear_to_epr() { "ear".yellow() } else { "ear".normal() }, self.ear),
        )
    }
}