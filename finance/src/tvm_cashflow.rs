use std::fmt::Debug;
use std::fmt;
use colored::*;

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
            TvmCashflowVariable::PresentValueAnnuity => write!(f, "Present Value Annuity"),
            TvmCashflowVariable::FutureValueAnnuity => write!(f, "Future Value Annuity"),
            TvmCashflowVariable::Payment => write!(f, "Payment"),
            TvmCashflowVariable::PresentValueAnnuityDue => write!(f, "Present Value Annuity Due"),
            TvmCashflowVariable::FutureValueAnnuityDue => write!(f, "Future Value Annuity Due"),
            TvmCashflowVariable::PaymentDue => write!(f, "Payment Due"),
            TvmCashflowVariable::NetPresentValue => write!(f, "Net Present Value"),
        }
    }
}

// #[derive(Debug)]
pub struct TvmCashflowSolution {
    pub calculated_field: TvmCashflowVariable,
    pub rates: Schedule,
    pub periods: u32,
    pub cashflow: f64,
    pub cashflow_0: f64,
    pub present_value: f64,
    pub future_value: f64,
    pub payment: f64,
    pub sum_of_payments: f64,
    pub formula: String,
    // pub input_in_percent: String,
}

impl TvmCashflowSolution {
    pub(crate) fn new(
            calculated_field: TvmCashflowVariable,
            rates: Schedule,
            periods: u32,
            cashflow: f64,
            cashflow_0: f64,
            present_value: f64,
            future_value: f64,
            payment: f64,
            formula: &str
    ) -> Self {
        assert!(rates.is_rate());
        assert!(formula.len() > 0);
        let sum_of_payments = payment * periods as f64;
        Self {
            calculated_field,
            rates,
            periods,
            cashflow,
            cashflow_0,
            present_value,
            future_value,
            payment,
            sum_of_payments,
            formula: formula.to_string(),
        }
    }

}

impl Debug for TvmCashflowSolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{}{}{}{}{}{}{}{}{}{}\n}}",
               &format!("\n\tcalculated_field: {}", self.calculated_field.to_string().magenta()),
               &format!("\n\trates: {}", format!("{:?}", self.rates).yellow()),
               &format!("\n\tperiods: {}", self.periods.to_string().yellow()),
               &format!("\n\tcashflow: {}", self.cashflow),
               if self.calculated_field.is_net_present_value() { format!("\n\tcashflow_0: {}", self.cashflow_0.to_string().red()) } else { "".to_string() },
               &format!("\n\tpayment: {}", if self.calculated_field.is_payment() || self.calculated_field.is_payment_due() { self.cashflow.to_string().green() } else { self.cashflow.to_string().normal() }),
               &format!("\n\tsum_of_payments: {}", self.sum_of_payments),
               &format!("\n\tpresent_value: {}", self.present_value),
               &format!("\n\tfuture_value: {}", self.future_value),
               &format!("\n\tformula: {:?}", self.formula),
               // &format!("input_in_percent: {:.6}%", self.input_in_percent),
               // &format!("output: {}", self.output.to_string().green()),
        )
    }
}

