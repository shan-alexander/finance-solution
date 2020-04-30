// use std::fmt::Debug;
use std::fmt;
// use colored::*;

// Import needed for the function references in the Rustdoc comments.
#[allow(unused_imports)]
use crate::*;
use std::cmp::max;
use std::ops::Deref;

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

    pub fn series(&self) -> TvmCashflowSeries {
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

#[derive(Clone, Debug)]
pub struct TvmCashflowSeries(Vec<TvmCashflowPeriod>);

impl TvmCashflowSeries {
    pub fn new(series: Vec<TvmCashflowPeriod>) -> Self {
        Self {
            0: series,
        }
    }

    pub fn filter<P>(&self, predicate: P) -> Self
        where P: Fn(&&TvmCashflowPeriod) -> bool
    {
        Self {
            0: self.iter().filter(|x| predicate(x)).map(|x| x.clone()).collect()
        }
    }

    pub fn print_table(&self, locale: &num_format::Locale, precision: usize) {
        let columns = vec![("period", "i"), ("payments_to_date", "f"), ("payments_remaining", "f"),
                           ("principal", "f"), ("principal_to_date", "f"), ("principal_remaining", "f"),
                           ("interest", "f"), ("interest_to_date", "f"), ("interest_remaining", "f")];
        let mut data = self.iter()
            .map(|entry| vec![entry.period.to_string(), entry.payments_to_date.to_string(), entry.payments_remaining.to_string(),
                              entry.principal.to_string(), entry.principal_to_date.to_string(), entry.principal_remaining.to_string(),
                              entry.interest.to_string(), entry.interest_to_date.to_string(), entry.interest_remaining.to_string()])
            .collect::<Vec<_>>();
        print_table_locale(&columns, &mut data, locale, precision);
    }
}

impl Deref for TvmCashflowSeries {
    type Target = Vec<TvmCashflowPeriod>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Debug)]
pub struct TvmCashflowPeriod {
    period: u32,
    rate: f64,
    due_at_beginning: bool,
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
    formula: String,
    formula_symbolic: String,
    // pub input_in_percent: String,
}

impl TvmCashflowPeriod {
    pub(crate) fn new(
        period: u32,
        rate: f64,
        due_at_beginning: bool,
        payment: f64,
        payments_to_date: f64,
        payments_remaining: f64,
        principal: f64,
        principal_to_date: f64,
        principal_remaining: f64,
        interest: f64,
        interest_to_date: f64,
        interest_remaining: f64,
        formula: String,
        formula_symbolic: String,
    ) -> Self {
        Self {
            period,
            rate,
            due_at_beginning,
            payment,
            payments_to_date,
            payments_remaining,
            principal,
            principal_to_date,
            principal_remaining,
            interest,
            interest_to_date,
            interest_remaining,
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

    pub fn print_flat(&self, precision: usize) {
        println!("TvmCashflowPeriod = {{ {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {} }}",
                 &format!("period: {}", self.period),
                 &format!("due_at_beginning: {}", self.due_at_beginning),
                 &format!("payment: {:.prec$}", self.payment, prec = precision),
                 &format!("payments_to_date: {:.prec$}", self.payments_to_date, prec = precision),
                 &format!("payments_remaining: {:.prec$}", self.payments_remaining, prec = precision),
                 &format!("principal: {:.prec$}", self.principal, prec = precision),
                 &format!("principal_to_date: {:.prec$}", self.principal_to_date, prec = precision),
                 &format!("principal_remaining: {:.prec$}", self.principal_remaining, prec = precision),
                 &format!("interest: {:.prec$}", self.interest, prec = precision),
                 &format!("interest_to_date: {:.prec$}", self.interest_to_date, prec = precision),
                 &format!("interest_remaining: {:.prec$}", self.interest_remaining, prec = precision),
                 &format!("formula: {:?}", self.formula),
                 &format!("formula_symbolic: {:?}", self.formula_symbolic));
    }
}

// pub fn print_series_filtered(series: &[TvmPeriod], filter: )

/*
pub fn print_series_table(series: &[TvmCashflowPeriod], precision: usize) {
    if series.len() == 0 {
        return;
    }
    let period_width = max("period".len(), series.iter().map(|x| x.period().to_string().len()).max().unwrap());
    let payments_to_date_width = max("payments_to_date".len(), series.iter().map(|x| format!("{:.prec$}", x.payments_to_date(), prec = precision).len()).max().unwrap());
    let payments_remaining_width = max("payments_remaining".len(), series.iter().map(|x| format!("{:.prec$}", x.payments_remaining(), prec = precision).len()).max().unwrap());
    let principal_width = max("principal_width".len(), series.iter().map(|x| format!("{:.prec$}", x.principal(), prec = precision).len()).max().unwrap());
    let principal_to_date_width = max("principal_to_date".len(), series.iter().map(|x| format!("{:.prec$}", x.principal_to_date(), prec = precision).len()).max().unwrap());
    let principal_remaining_width = max("principal_remaining".len(), series.iter().map(|x| format!("{:.prec$}", x.principal_remaining(), prec = precision).len()).max().unwrap());
    let interest_width = max("interest".len(), series.iter().map(|x| format!("{:.prec$}", x.interest(), prec = precision).len()).max().unwrap());
    let interest_to_date_width = max("interest_to_date".len(), series.iter().map(|x| format!("{:.prec$}", x.interest_to_date(), prec = precision).len()).max().unwrap());
    let interest_remaining_width = max("interest_remaining".len(), series.iter().map(|x| format!("{:.prec$}", x.interest_remaining(), prec = precision).len()).max().unwrap());
    println!("\ndue_at_beginning: {}", series[0].due_at_beginning);
    println!("payment: {:.prec$}", series[0].payment, prec = precision);
    println!("{:>pe$}  {:>pmtd$}  {:>pmr$}  {:>pr$}  {:>prtd$}  {:>prr$}  {:>i$}  {:>itd$}  {:>ir$}",
             "period", "payments_to_date", "payments_remaining", "principal", "principal_to_date", "principal_remaining", "interest", "interest_to_date", "interest_remaining",
             pe = period_width, pmtd = payments_to_date_width, pmr = payments_remaining_width,
             pr = principal_width, prtd = principal_to_date_width, prr = principal_remaining_width,
             i = interest_width, itd = interest_to_date_width, ir = interest_remaining_width);
    println!("{}  {}  {}  {}  {}  {}  {}  {}  {}",
             "-".repeat(period_width), "-".repeat(payments_to_date_width), "-".repeat(payments_remaining_width),
             "-".repeat(principal_width), "-".repeat(principal_to_date_width), "-".repeat(principal_remaining_width),
             "-".repeat(interest_width), "-".repeat(interest_to_date_width), "-".repeat(interest_remaining_width));
    for entry in series.iter() {
        println!("{:>pe$}  {:>pmtd$.prec$}  {:>pmr$.prec$}  {:>pr$.prec$}  {:>prtd$.prec$}  {:>prr$.prec$}  {:>i$.prec$}  {:>itd$.prec$}  {:>ir$.prec$}",
                 entry.period(), entry.payments_to_date(), entry.payments_remaining(),
                 entry.principal(), entry.principal_to_date(), entry.principal_remaining(),
                 entry.interest(), entry.interest_to_date(), entry.interest_remaining(),
                 pe = period_width, pmtd = payments_to_date_width, pmr = payments_remaining_width,
                 pr = principal_width, prtd = principal_to_date_width, prr = principal_remaining_width,
                 i = interest_width, itd = interest_to_date_width, ir = interest_remaining_width, prec = precision);
    }
}
*/

/*
pub fn print_series_table_locale(series: &[TvmCashflowPeriod], locale: &num_format::Locale, precision: usize) {
    if series.len() == 0 {
        return;
    }
    let period_width = max("period".len(), series.iter().map(|x| format_int_locale(x.period(), locale).len()).max().unwrap());
    let payments_to_date_width = max("payments_to_date".len(), series.iter().map(|x| format_float_locale(x.payments_to_date(), locale, precision).len()).max().unwrap());
    let payments_remaining_width = max("payments_remaining".len(), series.iter().map(|x| format_float_locale(x.payments_remaining(), locale, precision).len()).max().unwrap());
    let principal_width = max("principal".len(), series.iter().map(|x| format_float_locale(x.principal(), locale, precision).len()).max().unwrap());
    let principal_to_date_width = max("principal_to_date".len(), series.iter().map(|x| format_float_locale(x.principal_to_date(), locale, precision).len()).max().unwrap());
    let principal_remaining_width = max("principal_remaining".len(), series.iter().map(|x| format_float_locale(x.principal_remaining(), locale, precision).len()).max().unwrap());
    let interest_width = max("interest".len(), series.iter().map(|x| format_float_locale(x.interest(), locale, precision).len()).max().unwrap());
    let interest_to_date_width = max("interest_to_date".len(), series.iter().map(|x| format_float_locale(x.interest_to_date(), locale, precision).len()).max().unwrap());
    let interest_remaining_width = max("interest_remaining".len(), series.iter().map(|x| format_float_locale(x.interest_remaining(), locale, precision).len()).max().unwrap());
    println!("\ndue_at_beginning: {}", series[0].due_at_beginning);
    println!("payment: {:.prec$}", series[0].payment, prec = precision);
    println!("{:>pe$}  {:>pmtd$}  {:>pmr$}  {:>pr$}  {:>prtd$}  {:>prr$}  {:>i$}  {:>itd$}  {:>ir$}",
             "period", "payments_to_date", "payments_remaining", "principal", "principal_to_date", "principal_remaining", "interest", "interest_to_date", "interest_remaining",
             pe = period_width, pmtd = payments_to_date_width, pmr = payments_remaining_width,
             pr = principal_width, prtd = principal_to_date_width, prr = principal_remaining_width,
             i = interest_width, itd = interest_to_date_width, ir = interest_remaining_width);
    println!("{}  {}  {}  {}  {}  {}  {}  {}  {}",
             "-".repeat(period_width), "-".repeat(payments_to_date_width), "-".repeat(payments_remaining_width),
             "-".repeat(principal_width), "-".repeat(principal_to_date_width), "-".repeat(principal_remaining_width),
             "-".repeat(interest_width), "-".repeat(interest_to_date_width), "-".repeat(interest_remaining_width));
    for entry in series.iter() {
        println!("{:>pe$}  {:>pmtd$}  {:>pmr$}  {:>pr$}  {:>prtd$}  {:>prr$}  {:>i$}  {:>itd$}  {:>ir$}",
                 format_int_locale(entry.period(), locale), format_float_locale(entry.payments_to_date(), locale, precision), format_float_locale(entry.payments_remaining(), locale, precision),
                 format_float_locale(entry.principal(), locale, precision), format_float_locale(entry.principal_to_date(), locale, precision), format_float_locale(entry.principal_remaining(), locale, precision),
                 format_float_locale(entry.interest(), locale, precision), format_float_locale(entry.interest_to_date(), locale, precision), format_float_locale(entry.interest_remaining(), locale, precision),
                 pe = period_width, pmtd = payments_to_date_width, pmr = payments_remaining_width,
                 pr = principal_width, prtd = principal_to_date_width, prr = principal_remaining_width,
                 i = interest_width, itd = interest_to_date_width, ir = interest_remaining_width);
    }
}
*/

/*
pub fn print_series_table_filtered(series: &[TvmCashflowPeriod], predicate: P, precision: usize)
    where P: FnMut(&TvmCashflowPeriod) -> bool
{

}
*/

