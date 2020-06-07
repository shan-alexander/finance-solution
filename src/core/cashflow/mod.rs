#![warn(missing_docs)]
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
// #[doc(inline)]
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
#[allow(missing_docs)]
pub enum CashflowVariable {
    PresentValueAnnuity,
    PresentValueAnnuityDue,
    FutureValueAnnuity,
    Payment,
    FutureValueAnnuityDue,
    NetPresentValue,
}

impl CashflowVariable {
    /// Returns true if the variant is `CashflowVariable::PresentValueAnnuity` indicating that the
    /// solution was created by calculating the present value of an annuity with the payment due at
    /// the end of the month.
    pub fn is_present_value_annuity(&self) -> bool {
        match self {
            CashflowVariable::PresentValueAnnuity => true,
            _ => false,
        }
    }

    /// Returns true if the variant is `CashflowVariable::FutureValueAnnuity` indicating that the
    /// solution was created by calculating the future value of an annuity with the payment due at
    /// the end of the month.
    pub fn is_future_value_annuity(&self) -> bool {
        match self {
            CashflowVariable::FutureValueAnnuity => true,
            _ => false,
        }
    }

    /// Returns true if the variant is `CashflowVariable::Payment` indicating that the solution
    /// was created in a call to [payment_solution](payment/fn.payment_solution.html).
    pub fn is_payment(&self) -> bool {
        match self {
            CashflowVariable::Payment => true,
            _ => false,
        }
    }

    /// Returns true if the variant is `CashflowVariable::PresentValueAnnuityDue` indicating that
    /// the solution was created by calculating the present value of an annuity with the payment due
    /// at the beginning of the month.
    pub fn is_present_value_annuity_due(&self) -> bool {
        match self {
            CashflowVariable::PresentValueAnnuityDue => true,
            _ => false,
        }
    }

    /// Returns true if the variant is `CashflowVariable::FutureValueAnnuityDue` indicating that
    /// the solution was created by calculating the future value of an annuity with the payment due
    /// at the beginning of the month.
    pub fn is_future_value_annuity_due(&self) -> bool {
        match self {
            CashflowVariable::FutureValueAnnuityDue => true,
            _ => false,
        }
    }

    /// Returns true if the variant is `CashflowVariable::NetPresentValue` indicating that the
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

/// **A record of a cash flow calculation** such as payment, net present value, or the present value
/// or future value of an annuity.
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

    /// Returns a variant of [CashflowVariable](enum.CashflowVariable.html) showing which value was
    /// calculated such as the payment or future value. To test for the enum variant use functions
    /// like [CashflowVariable::is_payment](enum.CashflowVariable.html#method.is_payment).
    ///
    /// # Examples
    /// ```
    /// // Calculate a payment.
    /// let solution = finance_solution::core::payment_solution(0.015,  24, 25_000, 0, false);
    /// assert!(solution.calculated_field().is_payment());
    /// ```
    pub fn calculated_field(&self) -> &CashflowVariable {
        &self.calculated_field
    }

    /// Returns the periodic rate which may be a calculated value or may have been one of the inputs
    /// depending on which function was used to create this struct.
    pub fn rate(&self) -> f64 {
        self.rate
    }

    /// Returns the number of periods which may be a calculated value or may have been one of the
    /// inputs depending on which function was used to create this struct.
    pub fn periods(&self) -> u32 {
        self.periods
    }

    /// Returns the present value which may be a calculated value or may have been one of the inputs
    /// depending on which function was used to create this struct.
    pub fn present_value(&self) -> f64 {
        self.present_value
    }

    /// Returns the future value which may be a calculated value or may have been one of the inputs
    /// depending on which function was used to create this struct.
    pub fn future_value(&self) -> f64 {
        self.future_value
    }

    /// Returns true if the payment is due at the beginning of the period. The typical case is that
    /// the payment is due at the end.
    pub fn due_at_beginning(&self) -> bool {
        self.due_at_beginning
    }

    /// Returns the periodic payment which may be a calculated value or may have been one of the
    /// inputs depending on which function was used to create this struct.
    pub fn payment(&self) -> f64 {
        self.payment
    }

    /// Returns the sum of the periodic payments. This is simply [payment](#method.payment) times
    /// [periods](#method.periods).
    pub fn sum_of_payments(&self) -> f64 {
        self.sum_of_payments
    }

    /// Returns the sum of the portion of the periodic payments that went to interest.
    pub fn sum_of_interest(&self) -> f64 {
        self.sum_of_interest
    }

    /// Returns a text version of the formula used to calculate the result. The formula includes the
    /// actual values rather than variable names. For the formula with variables such as "r" for
    /// rate call [symbolic_formula](#method.symbolic_formula).
    pub fn formula(&self) -> &str {
        &self.formula
    }

    /// Returns a text version of the formula used to calculate the result using variables such as
    /// "n" for the number of periods. For the formula with the actual numbers rather than variables
    /// call [formula](#method.formula).
    pub fn symbolic_formula(&self) -> &str {
        &self.symbolic_formula
    }
}

#[derive(Clone, Debug)]
/// **The period-by-period details** of a cash flow calculation.
pub struct CashflowSeries(Vec<CashflowPeriod>);

impl CashflowSeries {
    pub(crate) fn new(series: Vec<CashflowPeriod>) -> Self {
        Self {
            0: series,
        }
    }

    /// Produces a series with a subset of the entries from the current series. This is intended to
    /// be used to help with examining and troubleshooting calculations since the resulting filtered
    /// series wouldn't make much sense on its own.
    ///
    /// # Arguments
    /// * `predicate` - A function that takes a reference to a
    /// [CashflowPeriod](struct.CashflowPeriod.html) and returns a boolean such as the closure
    /// `|entry| entry.principal() < 800.0`. Entries for which the predicate returns
    /// true are included in the new series.
    ///
    /// # Examples
    /// ```
    /// use finance_solution::core::*;
    ///
    /// let (rate, periods, present_value, future_value, due_at_beginning) = (0.01, 60, -10_000, 0.0, false);
    /// let solution = payment_solution(rate, periods, present_value, future_value, due_at_beginning);
    ///
    /// // Print the period-by-period details starting at the point where 99% of the interest has been
    /// // paid.
    /// let threshold = solution.sum_of_interest() * 0.99;
    /// solution
    ///     .series()
    ///     .filter(|entry| entry.interest_to_date() >= threshold)
    ///     .print_table();
    /// ```
    /// Output:
    /// ```text
    /// period   payment  payments_to_date  payments_remaining  principal  principal_to_date  principal_remaining  interest  interest_to_date  interest_remaining
    /// ------  --------  ----------------  ------------------  ---------  -----------------  -------------------  --------  ----------------  ------------------
    ///     55  222.4445       12_234.4462          1_112.2224   209.5528         8_920.3810           1_079.6190   12.8917        3_314.0652             32.6034
    ///     56  222.4445       12_456.8907            889.7779   211.6483         9_132.0293             867.9707   10.7962        3_324.8614             21.8072
    ///     57  222.4445       12_679.3352            667.3334   213.7648         9_345.7941             654.2059    8.6797        3_333.5411             13.1275
    ///     58  222.4445       12_901.7797            444.8890   215.9024         9_561.6965             438.3035    6.5421        3_340.0832              6.5855
    ///     59  222.4445       13_124.2241            222.4445   218.0614         9_779.7579             220.2421    4.3830        3_344.4662              2.2024
    ///     60  222.4445       13_346.6686              0.0000   220.2421         9_999.0000               0.0000    2.2024        3_346.6686              0.0000
    /// ```
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

    /// Prints a formatted table with the period-by-period details of a cashflow calculation.
    ///
    /// For more control over which columns appear use
    /// [print_table_custom](#method.print_table_custom).
    ///
    /// Money amounts are rounded to four decimal places, rates to six places, and numbers are
    /// formatted similar to Rust constants such as "10_000.0322". For more control over formatting
    /// use [print_table_locale](#method.print_table_locale) which also includes options for which
    /// columns appear.
    ///
    /// # Examples
    /// See [PaymentSolution::print_table](payment/struct.PaymentSolution.html#method.print_table).
    ///
    pub fn print_table(&self) {
        self.print_table_locale_opt(true, true, None, None);
    }

    /// Prints a formatted table with the period-by-period details of a cashflow calculation with
    /// options for which columns appear.
    ///
    /// For a simpler method that includes all columns use [print_table](#method.print_table). To
    /// control number formatting use [print_table_locale](#method.print_table_locale).
    ///
    /// # Arguments
    /// * `include_running_totals` - If true include the columns "payments_to_date",
    /// "principal_to_date", and "interest_to_date".
    /// * `include_remaining_amounts` - If true include the columns "payments_remaining",
    /// "principal_remaining", and "interest_remaining".
    ///
    /// # Examples
    /// See [PaymentSolution::print_table_custom](payment/struct.PaymentSolution.html#method.print_table_custom).
    ///
    pub fn print_table_custom(
        &self,
        include_running_totals: bool,
        include_remaining_amounts: bool)
    {
        self.print_table_locale_opt(include_running_totals, include_remaining_amounts, None, None);
    }

    /// Prints a formatted table with the period-by-period details of a cashflow calculation with
    /// options for which columns appear and how numbers are formatted.
    ///
    /// For a simpler method that doesn't require a locale but still has optional columns use
    /// [print_table_custom](#method.print_table_custom). The simplest table method is
    /// [print_table](#method.print_table) which prints all columns with default formatting.
    ///
    /// # Arguments
    /// * `include_running_totals` - If true include the columns "payments_to_date",
    /// "principal_to_date", and "interest_to_date".
    /// * `include_remaining_amounts` - If true include the columns "payments_remaining",
    /// "principal_remaining", and "interest_remaining".
    /// * `locale` - A locale constant from the `num-format` crate such as `Locale::en` for English
    /// or `Locale::vi` for Vietnamese. The locale determines the thousands separator and decimal
    /// separator.
    /// * `precision` - The number of decimal places for money amounts. Rates will appear with at
    /// least six places regardless of this argument.
    ///
    /// # Examples
    /// See [PaymentSolution::print_table_locale](payment/struct.PaymentSolution.html#method.print_table_locale).
    ///
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

    /// Compares the period-by-period details from two cashflow calculations.
    ///
    /// The values from the first calculation are labeled "a" and those from the second calculation
    /// are labeled "b". To control which columns appear in the table use
    /// [print_ab_comparison_custom](#method.print_ab_comparison_custom).
    ///
    /// Money amounts are rounded to four decimal places, rates to six places, and numbers are
    /// formatted similar to Rust constants such as "10_000.0322". For more control over formatting
    /// use [print_ab_comparison_locale](#method.print_ab_comparison_locale).
    ///
    /// # Arguments
    /// * `other` - The second `CashflowSeries` in the comparison which will be labeled "b".
    ///
    /// # Examples
    /// See [PaymentSolution::print_ab_comparison](payment/struct.PaymentSolution.html#method.print_ab_comparison).
    /// The difference is that the current method will show only the table with period-by-period
    /// details.
    ///
    pub fn print_ab_comparison(&self, other: &CashflowSeries) {
        self.print_ab_comparison_locale_opt(other, true, true, None, None);
    }

    /// Compares the period-by-period details from two cashflow calculations with options for which
    /// columns appear in the table. For a simpler method that includes all columns use
    /// [print_ab_comparison](#method.print_ab_comparison).
    ///
    /// The values from the first calculation are labeled "a" and those from the second calculation
    /// are labeled "b".
    ///
    /// Money amounts are rounded to four decimal places, rates to six places, and numbers are
    /// formatted similar to Rust constants such as "10_000.0322". For more control over formatting
    /// use [print_ab_comparison_locale](#method.print_ab_comparison_locale).
    ///
    /// # Arguments
    /// * `other` - The second `CashflowSeries` in the comparison which will be labeled "b".
    /// * `include_running_totals` - If true include "payments_to_date_a" (from the first
    /// calculation), "payments_to_date_b" (from the second calculation), and similar columns
    /// in the table.
    /// * `include_remaining_amounts` - If true include "principal_remaining_a" (from the first
    /// calculation), "principal_remaining_b" (from the second calculation), and similar columns
    /// in the table.
    ///
    /// # Examples
    /// See [PaymentSolution::print_ab_comparison_custom](payment/struct.PaymentSolution.html#method.print_ab_comparison_custom).
    /// The difference is that the current method will show only the table with period-by-period
    /// details.
    ///
    pub fn print_ab_comparison_custom(&self, other: &CashflowSeries, include_running_totals: bool, include_remaining_amounts: bool) {
        self.print_ab_comparison_locale_opt(other, include_running_totals, include_remaining_amounts, None, None);
    }

    /// Compares the period-by-period details from two cashflow calculations with options for which
    /// columns appear in the table and for how numbers should be formatted. For a simpler method
    /// that doesn't require a locale use
    /// [print_ab_comparison_custom](#method.print_ab_comparison_custom). The simplest A/B
    /// comparison method that displays all columns with default formatting is
    /// [print_ab_comparison](#method.print_ab_comparison).
    ///
    /// The values from the first calculation are labeled "a" and those from the second calculation
    /// are labeled "b".
    ///
    /// # Arguments
    /// * `other` - The second `CashflowSeries` in the comparison which will be labeled "b".
    /// * `include_running_totals` - If true include "payments_to_date_a" (from the first
    /// calculation), "payments_to_date_b" (from the second calculation), and similar columns
    /// in the table.
    /// * `include_remaining_amounts` - If true include "principal_remaining_a" (from the first
    /// calculation), "principal_remaining_b" (from the second calculation), and similar columns
    /// in the table.
    /// * `locale` - A locale constant from the `num-format` crate such as `Locale::en` for English
    /// or `Locale::vi` for Vietnamese. The locale determines the thousands separator and decimal
    /// separator.
    /// * `precision` - The number of decimal places for money amounts. Rates will appear with at
    /// least six places regardless of this argument.
    ///
    /// # Examples
    /// See [PaymentSolution::print_ab_comparison_locale](payment/struct.PaymentSolution.html#method.print_ab_comparison_locale).
    /// The difference is that the current method will show only the table with period-by-period
    /// details.
    ///
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

    /// Returns the periodic rate which will typically be the same for every period.
    pub fn rate(&self) -> f64 {
        self.rate
    }

    /// Returns the period number. The first real period is numbered one, though for some
    /// calculations there may be a period zero showing starting values.
    pub fn period(&self) -> u32 {
        self.period
    }

    /// Returns the payment for this period.
    pub fn payment(&self) -> f64 {
        self.payment
    }

    /// Returns the sum of the payments up to and including this period.
    pub fn payments_to_date(&self) -> f64 {
        self.payments_to_date
    }

    /// Returns the payments that will still be due after this period.
    pub fn payments_remaining(&self) -> f64 {
        self.payments_remaining
    }

    /// Returns the portion of this period's payment that goes to paying down the principal. In an
    /// amortized loan this portion will typically rise from period to period.
    pub fn principal(&self) -> f64 {
        self.principal
    }

    /// Returns the sum of the principal paid down in all periods up to and including this one.
    pub fn principal_to_date(&self) -> f64 {
        self.principal_to_date
    }

    /// Returns the principal still outstanding at the end of this period.
    pub fn principal_remaining(&self) -> f64 {
        self.principal_remaining
    }

    /// Returns the portion of this period's payment that is interest as opposed to paying down the
    /// principal. In an amortized loan this portion will typically fall from period to period.
    pub fn interest(&self) -> f64 {
        self.interest
    }

    /// Returns the sum of interest paid through the end of this period.
    pub fn interest_to_date(&self) -> f64 {
        self.interest_to_date
    }

    /// Returns the interest that will be paid in all of the remaining periods after this one.
    pub fn interest_remaining(&self) -> f64 {
        self.interest_remaining
    }

    /// Returns true if payments are due at the beginning of the period. In most cases the payment is
    /// due at the end.
    pub fn due_at_beginning(&self) -> bool {
        self.due_at_beginning
    }

    /// Returns a text version of the formula used to calculate the most relevant value for this
    /// period which will depend on which calculation was originally run. The formula includes the
    /// actual values rather than variable names. For the formula with variables such as "r" for
    /// rate call [symbolic_formula](#method.symbolic_formula).
    pub fn formula(&self) -> &str {
        &self.formula
    }

    /// Returns a text version of the formula used to calculate the most relevant value for this
    /// period which will depend on which calculation was originally run. The formula uses variables
    /// such as "r" for rate. For the formula with the actual numbers rather than variables call
    /// call [formula](#method.formula).
    pub fn symbolic_formula(&self) -> &str {
        &self.symbolic_formula
    }

    /*
    pub(crate) fn print_flat(&self, precision: usize) {
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
    */
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

#[test]
    fn test_symmetry_payment_annuities_simple() {
        let rate = 0.034;
        let periods = 36;
        let present_value = 100_000_000;
        let pmt_solution = payment_solution(rate, periods, present_value, 0, false);
        let pmt_amount = pmt_solution.payment();
        let pv_solution = present_value_annuity_solution(rate, periods, pmt_amount, false);
        assert_approx_equal!(pv_solution.present_value(), present_value as f64);
        assert_approx_equal!(pv_solution.future_value(), pmt_solution.future_value());
        let future_value = 400_000_000;
        let pmt_solution = payment_solution(rate, periods, 0, future_value, false);
        let pmt_amount = pmt_solution.payment();
        let fv_solution = future_value_annuity_solution(rate, periods, pmt_amount, false);
        assert_approx_equal!(fv_solution.future_value(), future_value as f64);
        assert_approx_equal!(fv_solution.present_value(), pmt_solution.present_value());
    }

}
