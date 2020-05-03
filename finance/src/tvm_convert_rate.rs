use std::fmt::Debug;
use std::fmt;
use colored::*;


// Import needed for the function references in the Rustdoc comments.
#[allow(unused_imports)]
use crate::*;

#[derive(Debug, Clone)]
pub enum ConvertRateVariable {
    Apr,
    Ear,
    Epr,
    AprContinuous,
    EarContinuous
}
impl ConvertRateVariable {
    pub fn is_apr(&self) -> bool {
        match self {
            ConvertRateVariable::Apr => true,
            _ => false,
        }
    }
    pub fn is_epr(&self) -> bool {
        match self {
            ConvertRateVariable::Epr => true,
            _ => false,
        }
    }
    pub fn is_ear(&self) -> bool {
        match self {
            ConvertRateVariable::Ear => true,
            _ => false,
        }
    }
    pub fn is_apr_continuous(&self) -> bool {
        match self {
            ConvertRateVariable::AprContinuous => true,
            _ => false,
        }
    }
    pub fn is_ear_continuous(&self) -> bool {
        match self {
            ConvertRateVariable::EarContinuous => true,
            _ => false,
        }
    }
}
impl fmt::Display for ConvertRateVariable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           ConvertRateVariable::Apr => write!(f, "Apr"),
           ConvertRateVariable::Epr => write!(f, "Epr"),
           ConvertRateVariable::Ear => write!(f, "Ear"),
           ConvertRateVariable::AprContinuous => write!(f, "AprContinuous"),
           ConvertRateVariable::EarContinuous => write!(f, "EarContinuous"),
       }
    }
}

// #[derive(Debug)]
pub struct ConvertRateSolution {
    input_name: ConvertRateVariable,
    input_rate: f64,
    compounds_per_year: u32,
    apr_in_percent: String,
    epr_in_percent: String,
    ear_in_percent: String,
    apr: f64,
    epr: f64,
    ear: f64,
    apr_formula: String,
    epr_formula: String,
    ear_formula: String,
}
impl ConvertRateSolution {
    pub fn new(input_name: ConvertRateVariable, input_rate: f64, compounds_per_year: u32, apr_in_percent: String, epr_in_percent: String, ear_in_percent: String, apr:f64, epr:f64, ear:f64, apr_formula: &str, epr_formula: &str, ear_formula: &str) -> Self {
        Self {
            input_name,
            input_rate,
            compounds_per_year,
            apr_in_percent,
            epr_in_percent,
            ear_in_percent,
            apr,
            epr,
            ear,
            apr_formula: apr_formula.to_string(),
            epr_formula: epr_formula.to_string(),
            ear_formula: ear_formula.to_string(),
        }
    }

    /// Returns the input rate.
    pub fn input_rate(&self) -> f64 {
        self.input_rate
    }
    /// Returns the annual rate (apr).
    pub fn apr(&self) -> f64 {
        self.apr
    }
    /// Returns the periodic rate (epr).
    pub fn epr(&self) -> f64 {
        self.epr
    }
    /// Returns the effective annual rate (ear).
    pub fn ear(&self) -> f64 {
        self.ear
    }
    /// Returns the input name (Ear, Apr, Epr, AprContinuous...etc).
    pub fn input_name(&self) -> &ConvertRateVariable {
        &self.input_name
    }
    /// Returns the compounds per year as u32.
    pub fn compounds_per_year(&self) -> u32 {
        self.compounds_per_year
    }
    /// Returns the annual rate (APR) in percentage format (for example, 3.58%).
    pub fn apr_in_percent(&self) -> &String {
        &self.apr_in_percent
    }
    /// Returns the effective annual rate (EAR) in percentage format (for example, 3.69%).
    pub fn ear_in_percent(&self) -> &String {
        &self.ear_in_percent
    }
    /// Returns the periodic rate (EPR) in percentage format (for example, 1.12%).
    pub fn epr_in_percent(&self) -> &String {
        &self.epr_in_percent
    }
}


impl Debug for ConvertRateSolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{{\n {},\n {},\n {},\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n}}",
               &format!("input_name: {}", self.input_name.to_string().magenta()),
               &format!("input_rate: {}", self.input_rate.to_string().yellow()),
               &format!("compounds_per_year: {:.4}", self.compounds_per_year.to_string().yellow()),
               &format!("apr_in_percent: {:.6}%", self.apr_in_percent),
               &format!("epr_in_percent: {:.6}%", self.epr_in_percent),
               &format!("ear_in_percent: {:.6}%", self.ear_in_percent),
               &format!("apr: {}", self.apr),
               &format!("epr: {}", self.epr),
               &format!("ear: {}", self.ear),
               &format!("apr_formula: {}", self.apr_formula),
               &format!("epr_formula: {}", self.epr_formula),
               &format!("ear_formula: {}", self.ear_formula),
        )
    }
}