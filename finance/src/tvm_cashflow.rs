// use std::fmt::Debug;
use std::fmt;
// use colored::*;

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

#[derive(Debug)]
pub struct TvmCashflowSolution {
    calculated_field: TvmCashflowVariable,
    rate: f64,
    periods: u32,
    present_value: f64,
    future_value: f64,
    due_at_beginning: bool,
    payment: f64,
    sum_of_payments: f64,
    sum_of_interest: f64,
    formula: String,
    formula_symbolic: String,
    // pub input_in_percent: String,
}

impl TvmCashflowSolution {
    pub(crate) fn new(
        calculated_field: TvmCashflowVariable,
        rate: f64,
        periods: u32,
        present_value: f64,
        future_value: f64,
        due_at_beginning: bool,
        payment: f64,
        formula: &str,
        formula_symbolic: &str,
    ) -> Self {
        assert!(formula.len() > 0);
        let sum_of_payments = payment * periods as f64;
        let sum_of_interest = sum_of_payments + present_value + future_value;
        Self {
            calculated_field,
            rate,
            periods,
            present_value,
            future_value,
            due_at_beginning,
            payment,
            sum_of_payments,
            sum_of_interest,
            formula: formula.to_string(),
            formula_symbolic: formula_symbolic.to_string(),
        }
    }

    pub fn series(&self) -> Vec<TvmCashflowPeriod> {
        if self.calculated_field.is_payment() || self.calculated_field.is_payment_due() {
            payment_series(self)
        } else {
            unimplemented!()
        }
    }

    pub fn calculated_field(&self) -> &TvmCashflowVariable {
        &self.calculated_field
    }

    pub fn rate(&self) -> f64 {
        self.rate
    }

    pub fn periods(&self) -> u32 {
        self.periods
    }

    pub fn present_value(&self) -> f64 {
        self.present_value
    }

    pub fn future_value(&self) -> f64 {
        self.future_value
    }

    pub fn due_at_beginning(&self) -> bool {
        self.due_at_beginning
    }

    pub fn payment(&self) -> f64 {
        self.payment
    }

    pub fn sum_of_payments(&self) -> f64 {
        self.sum_of_payments
    }

    pub fn sum_of_interest(&self) -> f64 {
        self.sum_of_interest
    }

    pub fn formula(&self) -> &str {
        &self.formula
    }

    pub fn formula_symbolic(&self) -> &str {
        &self.formula_symbolic
    }
}

/*
impl Debug for TvmCashflowSolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{}{}{}{}{}{}{}{}{}{}{}\n}}",
               &format!("\n\tcalculated_field: {}", self.calculated_field.to_string().magenta()),
               &format!("\n\trate (r): {}", format!("{:?}", self.rate).yellow()),
               &format!("\n\tperiods (n): {}", self.periods.to_string().yellow()),
               &format!("\n\tpresent_value (pv): {}", self.present_value),
               &format!("\n\tfuture_value (fv): {}", self.future_value),
               &format!("\n\tdue_at_beginning: {}", self.due_at_beginning),
               //    if self.calculated_field.is_net_present_value() { format!("\n\tcashflow: {}", self.cashflow.to_string().red()) } else { "".to_string() },
               //    if self.calculated_field.is_net_present_value() { format!("\n\tcashflow_0: {}", self.cashflow_0.to_string().red()) } else { "".to_string() },
               &format!("\n\tpayment (pmt): {}", if self.calculated_field.is_payment() || self.calculated_field.is_payment_due() { self.payment.to_string().green() } else { self.payment.to_string().normal() }),
               &format!("\n\tsum_of_payments: {}", self.sum_of_payments),
               &format!("\n\tsum_of_interest: {}", self.sum_of_interest),
               &format!("\n\tformula: {:?}", self.formula),
               &format!("\n\tformula_symbolic: {:?}", self.formula_symbolic),
               // &format!("input_in_percent: {:.6}%", self.input_in_percent),
               // &format!("output: {}", self.output.to_string().green()),
        )
    }
}
*/

#[derive(Debug)]
pub struct TvmCashflowPeriod {
    rate: f64,
    period: u32,
    // pub cashflow: f64,
    // pub cashflow_0: f64,
    payment: f64,
    payments_to_date: f64,
    payments_remaining: f64,
    principal: f64,
    principal_to_date: f64,
    principal_remaining: f64,
    interest: f64,
    interest_to_date: f64,
    interest_remaining: f64,
    due_at_beginning: bool,
    formula: String,
    formula_symbolic: String,
    // pub input_in_percent: String,
}

impl TvmCashflowPeriod {
    pub(crate) fn new(
        rate: f64,
        period: u32,
        payment: f64,
        payments_to_date: f64,
        payments_remaining: f64,
        principal: f64,
        principal_to_date: f64,
        principal_remaining: f64,
        interest: f64,
        interest_to_date: f64,
        interest_remaining: f64,
        due_at_beginning: bool,
        formula: String,
        formula_symbolic: String,
    ) -> Self {
        Self {
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
            due_at_beginning,
            formula,
            formula_symbolic,
        }
    }

    pub fn rate(&self) -> f64 {
        self.rate
    }

    pub fn period(&self) -> u32 {
        self.period
    }

    pub fn payment(&self) -> f64 {
        self.payment
    }

    pub fn payments_to_date(&self) -> f64 {
        self.payments_to_date
    }

    pub fn payments_remaining(&self) -> f64 {
        self.payments_remaining
    }

    pub fn principal(&self) -> f64 {
        self.principal
    }

    pub fn principal_to_date(&self) -> f64 {
        self.principal_to_date
    }

    pub fn principal_remaining(&self) -> f64 {
        self.principal_remaining
    }

    pub fn interest(&self) -> f64 {
        self.interest
    }

    pub fn interest_to_date(&self) -> f64 {
        self.interest_to_date
    }

    pub fn interest_remaining(&self) -> f64 {
        self.interest_remaining
    }

    pub fn due_at_beginning(&self) -> bool {
        self.due_at_beginning
    }

    pub fn formula(&self) -> &str {
        &self.formula
    }

    pub fn formula_symbolic(&self) -> &str {
        &self.formula_symbolic
    }

    fn print_flat(&self, scale: usize) {
        println!("TvmCashflowPeriod = {{ {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {} }}",
                 &format!("\n\trate: {:.}", format!("{:?}", self.rate).yellow()),



                 rate: f64,
                 period: u32,
                 // pub cashflow: f64,
                 // pub cashflow_0: f64,
                 payment: f64,
                 payments_to_date: f64,
                 payments_remaining: f64,
                 principal: f64,
                 principal_to_date: f64,
                 principal_remaining: f64,
                 interest: f64,
                 interest_to_date: f64,
                 interest_remaining: f64,
                 due_at_beginning: bool,
                 formula: String,
                 formula_symbolic: String,
                 &format!("calculated_field: {}", self.calculated_field.to_string()),
    &format!("\n\tperiods (n): {}", self.periods.to_string().yellow()),
    &format!("\n\tpresent_value (pv): {}", self.present_value),
    &format!("\n\tfuture_value (fv): {}", self.future_value),
    &format!("\n\tdue_at_beginning: {}", self.due_at_beginning),
    //    if self.calculated_field.is_net_present_value() { format!("\n\tcashflow: {}", self.cashflow.to_string().red()) } else { "".to_string() },
    //    if self.calculated_field.is_net_present_value() { format!("\n\tcashflow_0: {}", self.cashflow_0.to_string().red()) } else { "".to_string() },
    &format!("\n\tpayment (pmt): {}", if self.calculated_field.is_payment() || self.calculated_field.is_payment_due() { self.payment.to_string().green() } else { self.payment.to_string().normal() }),
    &format!("\n\tsum_of_payments: {}", self.sum_of_payments),
    &format!("\n\tsum_of_interest: {}", self.sum_of_interest),
    &format!("\n\tformula: {:?}", self.formula),
    &format!("\n\tformula_symbolic: {:?}", self.formula_symbolic),
    // &format!("input_in_percent: {:.6}%", self.input_in_percent),
    // &format!("output: {}", self.output.to_string().green()),
    )
    }
    }
    */


}
