use std::fmt::Debug;
use std::fmt;
use colored::*;

// The basic TVM equation, which can be arranged for anything except rate:
//   pv*(1+rate)^nper + pmt(1+rate*type)*((1+rate)^nper-1)/rate +fv = 0


// Import needed for the function references in the Rustdoc comments.
#[allow(unused_imports)]
use crate::*;

#[derive(Debug, Clone)]
pub enum TvmCashflowVariable {
    PresentValueAnnuity,
    FutureValueAnnuity,
    Payment,
    PresentValueAnnuityDue,
    FutureValueAnnuityDue,
    PaymentDue,
    NetPresentValue,
}
impl TvmCashflowVariable {
    pub fn is_present_value_annuity(&self) -> bool {
        match self {
            TvmCashflowVariable::PresentValueAnnuity => true,
            _ => false,
        }
    }
    pub fn is_future_value_annuity(&self) -> bool {
        match self {
            TvmCashflowVariable::FutureValueAnnuity => true,
            _ => false,
        }
    }
    pub fn is_payment(&self) -> bool {
        match self {
            TvmCashflowVariable::Payment => true,
            _ => false,
        }
    }
    pub fn is_present_value_annuity_due(&self) -> bool {
        match self {
            TvmCashflowVariable::PresentValueAnnuityDue => true,
            _ => false,
        }
    }
    pub fn is_future_value_annuity_due(&self) -> bool {
        match self {
            TvmCashflowVariable::FutureValueAnnuityDue => true,
            _ => false,
        }
    }
    pub fn is_payment_due(&self) -> bool {
        match self {
            TvmCashflowVariable::PaymentDue => true,
            _ => false,
        }
    }
    pub fn is_net_present_value(&self) -> bool {
        match self {
            TvmCashflowVariable::NetPresentValue => true,
            _ => false,
        }
    }
}
impl fmt::Display for TvmCashflowVariable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           TvmCashflowVariable::PresentValueAnnuity => write!(f, "PresentValueAnnuity"),
           TvmCashflowVariable::FutureValueAnnuity => write!(f, "FutureValueAnnuity"),
           TvmCashflowVariable::Payment => write!(f, "Payment"),
           TvmCashflowVariable::PresentValueAnnuityDue => write!(f, "PresentValueAnnuityDue"),
           TvmCashflowVariable::FutureValueAnnuityDue => write!(f, "FutureValueAnnuityDue"),
           TvmCashflowVariable::PaymentDue => write!(f, "PaymentDue"),
           TvmCashflowVariable::NetPresentValue => write!(f, "NetPresentValue"),
       }
    }
}

// #[derive(Debug)]
pub struct TvmCashflowSolution {
    pub calculated_field: TvmCashflowVariable,
    pub rate: f64,
    pub periods: u32,
    pub cashflow: f64,
    pub present_value: f64,
    pub future_value: f64,
    pub formula: String,
    pub cashflow_0: f64,
    pub input_in_percent: String,
    pub output: f64,
}
impl TvmCashflowSolution {
    pub(crate) fn new(calculated_field: TvmCashflowVariable, rate: f64, periods: u32, cashflow: f64, present_value: f64, future_value: f64, formula: &str, cashflow_0: f64, input_in_percent: String, output:f64) -> Self {
        assert!(rate.is_finite());
        assert!(formula.len() > 0);
        Self {
            calculated_field,
            rate,
            periods,
            cashflow,
            present_value,
            future_value,
            formula: formula.to_string(),
            cashflow_0,
            input_in_percent,
            output,
        }
    }
}


impl Debug for TvmCashflowSolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{{\n {},\n {},\n {},\n {}\n {}\n {}\n {}\n {}\n {}\n {}\n }}",
               &format!("calculated_field: {}", self.calculated_field.to_string().magenta()),
               &format!("rate: {}", self.rate.to_string().yellow()),
               &format!("periods: {}", self.periods.to_string().yellow()),
               &format!("payment: {}", if self.calculated_field.is_payment() || self.calculated_field.is_payment_due() { self.cashflow.to_string().green() } else { self.cashflow.to_string().normal() }),
               &format!("present_value: {}", self.present_value),
               &format!("future_value: {}", self.future_value),
               &format!("formula: {:?}", self.formula),
               &format!("rate_in_percent: {:.4}%", self.input_in_percent),
               &format!("output: {}", self.output.to_string().green()),
               &format!("{}{}", if self.calculated_field.is_net_present_value() { "cashflow_0: "} else { "" }, if self.calculated_field.is_net_present_value() { self.cashflow_0.to_string().red() } else { "".normal() }),
        )
    }
}