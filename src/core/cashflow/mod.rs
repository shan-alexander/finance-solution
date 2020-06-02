#![allow(unused_imports)]

//! The internal module which supports the solution struct for the Cashflow family of functions (e.g., `payment`).

// use std::fmt::Debug;
use std::fmt;
// use colored::*;

// Import needed for the function references in the Rustdoc comments.
use crate::*;
use std::cmp::max;
use std::ops::Deref;

pub mod future_value_annuity;
#[doc(inline)]
pub use future_value_annuity::*;

pub mod payment;
#[doc(inline)]
pub use payment::*;

pub mod present_value_annuity;
#[doc(inline)]
pub use present_value_annuity::*;

pub mod net_present_value;
#[doc(inline)]
pub use net_present_value::*;

pub mod nper;
#[doc(inline)]
pub use nper::*;

/// Enumeration used in [CashflowSolution](././struct.CashflowSolution.html) to keep track of what
/// value was calculated such as the payment, future value or net present value.
///
/// It can be checked with [CashflowSolution::calculated_field](././struct.CashflowSolution.html#method.calculated_field).
#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub enum CashflowVariable {
    PresentValueAnnuity,
    PresentValueAnnuityDue,
    FutureValueAnnuity,
    Payment,
    FutureValueAnnuityDue,
    NetPresentValue,
}

impl CashflowVariable {
    /// Returns true if the variant is CashflowVariable::PresentValueAnnuity indicating that the
    /// solution was created by calculating the present value of an annuity with the payment due at
    /// the end of the month.
    pub fn is_present_value_annuity(&self) -> bool {
        match self {
            CashflowVariable::PresentValueAnnuity => true,
            _ => false,
        }
    }

    /// Returns true if the variant is CashflowVariable::FutureValueAnnuity indicating that the
    /// solution was created by calculating the future value of an annuity with the payment due at
    /// the end of the month.
    pub fn is_future_value_annuity(&self) -> bool {
        match self {
            CashflowVariable::FutureValueAnnuity => true,
            _ => false,
        }
    }

    /// Returns true if the variant is CashflowVariable::Payment indicating that the solution
    /// was created in a call to [`payment_solution`].
    pub fn is_payment(&self) -> bool {
        match self {
            CashflowVariable::Payment => true,
            _ => false,
        }
    }

    /// Returns true if the variant is CashflowVariable::PresentValueAnnuityDue indicating that
    /// the solution was created by calculating the present value of an annuity with the payment due
    /// at the beginning of the month.
    pub fn is_present_value_annuity_due(&self) -> bool {
        match self {
            CashflowVariable::PresentValueAnnuityDue => true,
            _ => false,
        }
    }

    /// Returns true if the variant is CashflowVariable::FutureValueAnnuityDue indicating that
    /// the solution was created by calculating the future value of an annuity with the payment due
    /// at the beginning of the month.
    pub fn is_future_value_annuity_due(&self) -> bool {
        match self {
            CashflowVariable::FutureValueAnnuityDue => true,
            _ => false,
        }
    }

    /// Returns true if the variant is CashflowVariable::NetPresentValue indicating that the
    /// solution was created by calculating a net present value.
    pub fn is_net_present_value(&self) -> bool {
        match self {
            CashflowVariable::NetPresentValue => true,
            _ => false,
        }
    }
}

impl fmt::Display for CashflowVariable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CashflowVariable::PresentValueAnnuity => write!(f, "Present Value Annuity"),
            CashflowVariable::FutureValueAnnuity => write!(f, "Future Value Annuity"),
            CashflowVariable::Payment => write!(f, "Payment"),
            CashflowVariable::PresentValueAnnuityDue => write!(f, "Present Value Annuity Due"),
            CashflowVariable::FutureValueAnnuityDue => write!(f, "Future Value Annuity Due"),
            CashflowVariable::NetPresentValue => write!(f, "Net Present Value"),
        }
    }
}

/// **A record of a cash flow calculation** such as payment, net present value, or the present value or
/// future value of an annuity.
#[derive(Clone, Debug)]
pub struct CashflowSolution {
    calculated_field: CashflowVariable,
    rate: f64,
    periods: u32,
    present_value: f64,
    future_value: f64,
    due_at_beginning: bool,
    payment: f64,
    sum_of_payments: f64,
    sum_of_interest: f64,
    formula: String,
    symbolic_formula: String,
    // pub input_in_percent: String,
}

impl CashflowSolution {
    pub(crate) fn new(
        calculated_field: CashflowVariable,
        rate: f64,
        periods: u32,
        present_value: f64,
        future_value: f64,
        due_at_beginning: bool,
        payment: f64,
        formula: &str,
        symbolic_formula: &str,
    ) -> Self {
        assert!(!formula.is_empty());
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
            symbolic_formula: symbolic_formula.to_string(),
        }
    }

    pub fn calculated_field(&self) -> &CashflowVariable {
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

    pub fn symbolic_formula(&self) -> &str {
        &self.symbolic_formula
    }

}

/*
impl Debug for CashflowSolution {
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
               &format!("\n\tsymbolic_formula: {:?}", self.symbolic_formula),
               // &format!("input_in_percent: {:.6}%", self.input_in_percent),
               // &format!("output: {}", self.output.to_string().green()),
        )
    }
}
*/

#[derive(Clone, Debug)]
/// **The period-by-period details** of a cash flow calculation.
pub struct CashflowSeries(Vec<CashflowPeriod>);

impl CashflowSeries {
    pub(crate) fn new(series: Vec<CashflowPeriod>) -> Self {
        Self {
            0: series,
        }
    }

    pub fn filter<P>(&self, predicate: P) -> Self
        where P: Fn(&&CashflowPeriod) -> bool
    {
        Self {
            0: self.iter().filter(|x| predicate(x)).cloned().collect()
        }
    }

    pub(crate) fn summarize(&self, periods: u32) -> Self {
        let mut series = vec![];
        for entry in self.iter().filter(|entry| entry.period() % periods == 0 || entry.period() == self.len() as u32 + 1) {
            let mut payment_sum = 0.0;
            let mut principal_sum = 0.0;
            let mut interest_sum = 0.0;
            let period_min = (entry.period as i128 - periods as i128) + 1;
            let period_max = entry.period;
            for sum_entry in self.iter().filter(|entry| entry.period as i128 >= period_min && entry.period <= period_max) {
                payment_sum += sum_entry.payment;
                principal_sum += sum_entry.principal;
                interest_sum += sum_entry.interest;
            }
            series.push(CashflowPeriod::new(
                entry.period, entry.rate, entry.due_at_beginning,
                payment_sum, entry.payments_to_date, entry.payments_remaining,
                principal_sum, entry.principal_to_date, entry.principal_remaining,
                interest_sum, entry.interest_to_date, entry.interest_remaining,
                &entry.formula, &entry.symbolic_formula));
        }
        CashflowSeries::new(series)
    }

    pub fn print_table(
        &self,
        include_running_totals: bool,
        include_remaining_amounts: bool)
    {
        self.print_table_locale_opt(include_running_totals, include_remaining_amounts, None, None);
    }

    pub fn print_table_locale(
        &self,
        include_running_totals: bool,
        include_remaining_amounts: bool,
        locale: &num_format::Locale,
        precision: usize)
    {
        self.print_table_locale_opt(include_running_totals, include_remaining_amounts, Some(locale), Some(precision));
    }

    pub(crate) fn print_table_locale_opt(
        &self,
        include_running_totals: bool,
        include_remaining_amounts: bool,
        locale: Option<&num_format::Locale>,
        precision: Option<usize>)
    {
        let columns = columns_with_strings(&[
            ("period", "i", true),
            ("payment", "f", true), ("payments_to_date", "f", include_running_totals), ("payments_remaining", "f", include_remaining_amounts),
            ("principal", "f", true), ("principal_to_date", "f", include_running_totals), ("principal_remaining", "f", include_remaining_amounts),
            ("interest", "f", true), ("interest_to_date", "f", include_running_totals), ("interest_remaining", "f", include_remaining_amounts)]);
        let data = self.iter()
            .map(|entry| vec![entry.period.to_string(), entry.payment.to_string(), entry.payments_to_date.to_string(), entry.payments_remaining.to_string(),
                              entry.principal.to_string(), entry.principal_to_date.to_string(), entry.principal_remaining.to_string(),
                              entry.interest.to_string(), entry.interest_to_date.to_string(), entry.interest_remaining.to_string()])
            .collect::<Vec<_>>();
        print_table_locale_opt(&columns, data, locale, precision);
    }

    pub fn print_ab_comparison(
        &self,
        other: &CashflowSeries,
        include_running_totals: bool,
        include_remaining_amounts: bool)
    {
        self.print_ab_comparison_locale_opt(other, include_running_totals, include_remaining_amounts, None, None);
    }

    pub fn print_ab_comparison_locale(
            &self,
            other: &CashflowSeries,
            include_running_totals: bool,
            include_remaining_amounts: bool,
            locale: &num_format::Locale,
            precision: usize) {
        self.print_ab_comparison_locale_opt(other, include_running_totals, include_remaining_amounts, Some(locale), Some(precision));
    }

    pub(crate) fn print_ab_comparison_locale_opt(
            &self,
            other: &CashflowSeries,
            include_running_totals: bool,
            include_remaining_amounts: bool,
            locale: Option<&num_format::Locale>,
            precision: Option<usize>) {
        let columns = columns_with_strings(&[("period", "i", true),
            ("payment_a", "f", true), ("payment_b", "f", true),
            ("pmt_to_date_a", "f", include_running_totals), ("pmt_to_date_b", "f", include_running_totals),
            ("pmt_remaining_a", "f", include_remaining_amounts), ("pmt_remaining_b", "f", include_remaining_amounts),
            ("principal_a", "f", true), ("principal_b", "f", true),
            ("princ_to_date_a", "f", include_running_totals), ("princ_to_date_b", "f", include_running_totals),
            ("princ_remaining_a", "f", include_remaining_amounts), ("princ_remaining_b", "f", include_remaining_amounts),
            ("interest_a", "f", true), ("interest_b", "f", true),
            ("int_to_date_a", "f", include_running_totals), ("int_to_date_b", "f", include_running_totals),
            ("int_remaining_a", "f", include_remaining_amounts), ("int_remaining_b", "f", include_remaining_amounts)
        ]);
        let mut data = vec![];
        let rows = max(self.len(), other.len());
        for row_index in 0..rows {
            data.push(vec![
                (row_index + 1).to_string(),
                self.get(row_index).map_or("".to_string(), |x| x.payment.to_string()),
                other.get(row_index).map_or("".to_string(), |x| x.payment.to_string()),
                self.get(row_index).map_or("".to_string(), |x| x.payments_to_date.to_string()),
                other.get(row_index).map_or("".to_string(), |x| x.payments_to_date.to_string()),
                self.get(row_index).map_or("".to_string(), |x| x.payments_remaining.to_string()),
                other.get(row_index).map_or("".to_string(), |x| x.payments_remaining.to_string()),
                self.get(row_index).map_or("".to_string(), |x| x.principal.to_string()),
                other.get(row_index).map_or("".to_string(), |x| x.principal.to_string()),
                self.get(row_index).map_or("".to_string(), |x| x.principal_to_date.to_string()),
                other.get(row_index).map_or("".to_string(), |x| x.principal_to_date.to_string()),
                self.get(row_index).map_or("".to_string(), |x| x.principal_remaining.to_string()),
                other.get(row_index).map_or("".to_string(), |x| x.principal_remaining.to_string()),
                self.get(row_index).map_or("".to_string(), |x| x.interest.to_string()),
                other.get(row_index).map_or("".to_string(), |x| x.interest.to_string()),
                self.get(row_index).map_or("".to_string(), |x| x.interest_to_date.to_string()),
                other.get(row_index).map_or("".to_string(), |x| x.interest_to_date.to_string()),
                self.get(row_index).map_or("".to_string(), |x| x.interest_remaining.to_string()),
                other.get(row_index).map_or("".to_string(), |x| x.interest_remaining.to_string()),
            ]);
        }
        print_table_locale_opt(&columns, data, locale, precision);
    }

}

impl Deref for CashflowSeries {
    type Target = Vec<CashflowPeriod>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// **The values at the end of one period** for a cash flow calculation.
#[derive(Clone, Debug)]
pub struct CashflowPeriod {
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
    symbolic_formula: String,
    // pub input_in_percent: String,
}

impl CashflowPeriod {
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
        formula: &str,
        symbolic_formula: &str,
    ) -> Self {
        Self {
            period,
            rate,
            due_at_beginning,
            payment,
            payments_to_date,
            payments_remaining: fix_zero(payments_remaining),
            principal,
            principal_to_date,
            principal_remaining: fix_zero(principal_remaining),
            interest,
            interest_to_date,
            interest_remaining: fix_zero(interest_remaining),
            formula: formula.to_string(),
            symbolic_formula: symbolic_formula.to_string(),
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

    pub fn symbolic_formula(&self) -> &str {
        &self.symbolic_formula
    }

    pub fn print_flat(&self, precision: usize) {
        println!("CashflowPeriod = {{ {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {} }}",
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
                 &format!("symbolic_formula: {:?}", self.symbolic_formula));
    }
}

// pub fn print_series_filtered(series: &[TvmPeriod], filter: )

/*
pub fn print_series_table(series: &[CashflowPeriod], precision: usize) {
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
pub fn print_series_table_locale(series: &[CashflowPeriod], locale: &num_format::Locale, precision: usize) {
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
pub fn print_series_table_filtered(series: &[CashflowPeriod], predicate: P, precision: usize)
    where P: FnMut(&CashflowPeriod) -> bool
{
}
*/
