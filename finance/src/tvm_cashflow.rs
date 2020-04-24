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
    pub present_value: f64,
    pub future_value: f64,
    pub due_at_beginning: bool,
    pub cashflow: f64,
    pub cashflow_0: f64,
    pub payment: f64,
    pub sum_of_payments: f64,
    pub sum_of_interest: f64,
    pub formula: String,
    pub formula_symbolic: String,
    // pub input_in_percent: String,
}

impl TvmCashflowSolution {
    pub(crate) fn new(
        calculated_field: TvmCashflowVariable,
        rates: Schedule,
        periods: u32,
        present_value: f64,
        future_value: f64,
        due_at_beginning: bool,
        cashflow: f64,
        cashflow_0: f64,
        payment: f64,
        formula: &str,
        formula_symbolic: &str,
    ) -> Self {
        assert!(rates.is_rate());
        assert!(formula.len() > 0);
        let sum_of_payments = payment * periods as f64;
        let sum_of_interest = sum_of_payments + present_value + future_value;
        Self {
            calculated_field,
            rates,
            periods,
            present_value,
            future_value,
            due_at_beginning,
            cashflow,
            cashflow_0,
            payment,
            sum_of_payments,
            sum_of_interest,
            formula: formula.to_string(),
            formula_symbolic: formula_symbolic.to_string(),
        }
    }

    pub fn series(&self) -> Vec<TvmCashflowPeriod> {
        let mut series = vec![];
        let payment = self.payment;
        let mut payments_to_date = 0.0;
        let mut principal_to_date = 0.0;
        let mut interest_to_date = 0.0;
        for period in 1..=self.periods {
            let principal_remaining_at_start_of_period = self.present_value + self.future_value + principal_to_date;
            let rate_index = period as usize - 1;
            let rate = self.rates.get(rate_index);
            // let rate_for_calculation = rate_for_period / self.periods as f64;
            let interest = -principal_remaining_at_start_of_period * rate;
            let principal = payment - interest;
            payments_to_date += payment;
            principal_to_date += principal;
            interest_to_date += interest;
            let payments_remaining = self.sum_of_payments - payments_to_date;
            let principal_remaining = -(self.present_value + self.future_value + principal_to_date);
            let interest_remaining = self.sum_of_interest - interest_to_date;
            let formula = format!("{:.4} = -({:.4} * {:.6})", interest, principal_remaining_at_start_of_period, rate);
            let formula_symbolic = "interest = -(principal * rate)".to_string();
            let entry = TvmCashflowPeriod {
                rate,
                period,
                payment,
                payments_to_date,
                payments_remaining,
                principal,
                principal_to_date,
                principal_remaining,
                interest,
                interest_to_date,
                interest_remaining,
                due_at_beginning: self.due_at_beginning,
                formula,
                formula_symbolic,
            };
            series.push(entry);
        }
        series
    }

}

impl Debug for TvmCashflowSolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{}{}{}{}{}{}{}{}{}{}{}{}{}\n}}",
               &format!("\n\tcalculated_field: {}", self.calculated_field.to_string().magenta()),
               &format!("\n\trates (r): {}", format!("{:?}", self.rates).yellow()),
               &format!("\n\tperiods (n): {}", self.periods.to_string().yellow()),
               &format!("\n\tpresent_value (pv): {}", self.present_value),
               &format!("\n\tfuture_value (fv): {}", self.future_value),
               &format!("\n\tdue_at_beginning: {}", self.due_at_beginning),
               if self.calculated_field.is_net_present_value() { format!("\n\tcashflow: {}", self.cashflow.to_string().red()) } else { "".to_string() },
               if self.calculated_field.is_net_present_value() { format!("\n\tcashflow_0: {}", self.cashflow_0.to_string().red()) } else { "".to_string() },
               &format!("\n\tpayment (pmt): {}", if self.calculated_field.is_payment() || self.calculated_field.is_payment_due() { self.cashflow.to_string().green() } else { self.cashflow.to_string().normal() }),
               &format!("\n\tsum_of_payments: {}", self.sum_of_payments),
               &format!("\n\tsum_of_interest: {}", self.sum_of_interest),
               &format!("\n\tformula: {:?}", self.formula),
               &format!("\n\tformula_symbolic: {:?}", self.formula_symbolic),
               // &format!("input_in_percent: {:.6}%", self.input_in_percent),
               // &format!("output: {}", self.output.to_string().green()),
        )
    }
}

#[derive(Debug)]
pub struct TvmCashflowPeriod {
    pub rate: f64,
    pub period: u32,
    // pub cashflow: f64,
    // pub cashflow_0: f64,
    pub payment: f64,
    pub payments_to_date: f64,
    pub payments_remaining: f64,
    pub principal: f64,
    pub principal_to_date: f64,
    pub principal_remaining: f64,
    pub interest: f64,
    pub interest_to_date: f64,
    pub interest_remaining: f64,
    pub due_at_beginning: bool,
    pub formula: String,
    pub formula_symbolic: String,
    // pub input_in_percent: String,
}
