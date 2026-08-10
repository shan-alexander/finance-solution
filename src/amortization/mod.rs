//! Amortization schedules with the same **solution / series / table** pattern as payment TVM.
//!
//! # What you get
//!
//! | Layer | API | Purpose |
//! |-------|-----|---------|
//! | Scalar (Excel) | [`ipmt`], [`ppmt`], [`cumipmt`], [`cumprinc`] | Single numbers, spreadsheet parity |
//! | Solution | [`amortization_solution`] | Inputs + payment + formulas |
//! | Series | [`AmortizationSolution::series`] | Period-by-period principal / interest |
//! | Table | [`AmortizationSeries::print_table`] | Pretty terminal / copy-paste output |
//!
//! # Error handling (v0.1+)
//!
//! Public construction and Excel-style scalars return [`FinanceResult`](crate::FinanceResult).
//! Invalid rates, zero periods, and out-of-range period indices are **errors**, not panics.
//! [`AmortizationSolution::series`] is infallible once a solution was constructed successfully.
//!
//! Period numbers are **1-based**, matching Excel / Google Sheets.
//! Sign conventions match [`payment`](crate::payment).
//! Timing uses [`PaymentTiming`](crate::PaymentTiming) (or `bool` via [`From`]).
//!
//! # Example: solution → series → table
//!
//! ```
//! use finance_solution::*;
//!
//! // $10,000 at 8% APR, monthly, 12 months
//! let rate = 0.08 / 12.0;
//! let periods = 12;
//! let principal = 10_000.0;
//!
//! let solution = amortization_solution(rate, periods, principal, 0.0, false).unwrap();
//! assert!(solution.payment() < 0.0); // positive principal → negative payment
//!
//! let series = solution.series();
//! assert_eq!(series.len(), periods as usize);
//!
//! // Pretty table (running totals + remaining amounts).
//! series.print_table(true, true);
//!
//! assert_approx_equal!(solution.ipmt(1).unwrap(), series[0].interest());
//! assert_approx_equal!(solution.ppmt(1).unwrap(), series[0].principal());
//! ```
//!
//! `print_table(true, true)` prints a schedule like this (truncated for width in some
//! terminals; values match a 12-month $10k loan at 8% APR monthly):
//!
//! ```text
//! period    payment  principal  interest     balance  principal_to_date  interest_to_date  payments_to_date  principal_remaining  interest_remaining  payments_remaining
//! ------  ---------  ---------  --------  ----------  -----------------  ----------------  ----------------  -------------------  ------------------  ------------------
//!      1  -869.8843  -803.2176  -66.6667  9_196.7824          -803.2176          -66.6667         -869.8843          -9_196.7824           -371.9448         -9_568.7272
//!      2  -869.8843  -808.5724  -61.3119  8_388.2100        -1_611.7900         -127.9785       -1_739.7686          -8_388.2100           -310.6329         -8_698.8429
//!      3  -869.8843  -813.9629  -55.9214  7_574.2471        -2_425.7529         -183.8999       -2_609.6529          -7_574.2471           -254.7115         -7_828.9586
//!    ...
//!     12  -869.8843  -864.1235   -5.7608     -0.0000       -10_000.0000         -438.6115      -10_438.6115               0.0000             -0.0000              0.0000
//! ```
//!
//! Period 1: payment ≈ −869.88 splits into interest ≈ −66.67 and principal ≈ −803.22;
//! balance after payment ≈ 9,196.78. Period 12 pays the loan down to ≈ 0.
use std::ops::Deref;

use crate::util::error::{require_finite, require_rate_gt_minus_one, FinanceError, FinanceResult};
use crate::{columns_with_strings, print_table_locale_opt};

// ---------------------------------------------------------------------------
// Solution
// ---------------------------------------------------------------------------

/// Full amortization setup: inputs, level payment, formulas, and access to a period series.
///
/// Create with [`amortization_solution`] (returns [`FinanceResult`]).
///
/// # Examples
/// ```
/// use finance_solution::*;
///
/// let s = amortization_solution(0.08 / 12.0, 12, 10_000.0, 0.0, false).unwrap();
/// println!("{}", s.formula());
/// println!("{}", s.symbolic_formula());
/// assert_eq!(s.periods(), 12);
/// s.print_table();
/// ```
///
/// `print_table()` is sugar for `series().print_table(true, true)` and looks like:
///
/// ```text
/// period    payment  principal  interest     balance  ...  payments_remaining
/// ------  ---------  ---------  --------  ----------  ...  ------------------
///      1  -869.8843  -803.2176  -66.6667  9_196.7824  ...         -9_568.7272
///      2  -869.8843  -808.5724  -61.3119  8_388.2100  ...         -8_698.8429
///    ...
///     12  -869.8843  -864.1235   -5.7608     -0.0000  ...              0.0000
/// ```
#[derive(Clone, Debug)]
pub struct AmortizationSolution {
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
}

impl AmortizationSolution {
    pub(crate) fn new(
        rate: f64,
        periods: u32,
        present_value: f64,
        future_value: f64,
        due_at_beginning: bool,
        payment: f64,
        formula: String,
        symbolic_formula: String,
    ) -> Self {
        let sum_of_payments = payment * periods as f64;
        let sum_of_interest = sum_of_payments + present_value + future_value;
        Self {
            rate,
            periods,
            present_value,
            future_value,
            due_at_beginning,
            payment,
            sum_of_payments,
            sum_of_interest,
            formula,
            symbolic_formula,
        }
    }

    /// Periodic rate.
    pub fn rate(&self) -> f64 {
        self.rate
    }

    /// Total number of periods.
    pub fn periods(&self) -> u32 {
        self.periods
    }

    /// Present value (principal).
    pub fn present_value(&self) -> f64 {
        self.present_value
    }

    /// Future value (balloon / residual).
    pub fn future_value(&self) -> f64 {
        self.future_value
    }

    /// Whether payment is due at the beginning of each period.
    pub fn due_at_beginning(&self) -> bool {
        self.due_at_beginning
    }

    /// Level payment each period (Excel `PMT`).
    pub fn payment(&self) -> f64 {
        self.payment
    }

    /// `payment * periods`.
    pub fn sum_of_payments(&self) -> f64 {
        self.sum_of_payments
    }

    /// Total interest over the full term (`sum_of_payments + present_value + future_value`).
    pub fn sum_of_interest(&self) -> f64 {
        self.sum_of_interest
    }

    /// Concrete formula string with substituted values.
    pub fn formula(&self) -> &str {
        &self.formula
    }

    /// Symbolic formula (e.g. `pmt = ...`).
    pub fn symbolic_formula(&self) -> &str {
        &self.symbolic_formula
    }

    /// Interest portion for a 1-based period (Excel `IPMT`).
    ///
    /// # Errors
    /// [`FinanceError::InvalidPeriod`] if `period` is not in `1..=periods`.
    pub fn ipmt(&self, period: u32) -> FinanceResult<f64> {
        self.period_at(period).map(|p| p.interest())
    }

    /// Principal portion for a 1-based period (Excel `PPMT`).
    ///
    /// # Errors
    /// [`FinanceError::InvalidPeriod`] if `period` is not in `1..=periods`.
    pub fn ppmt(&self, period: u32) -> FinanceResult<f64> {
        self.period_at(period).map(|p| p.principal())
    }

    /// Cumulative interest from `start_period`..=`end_period` (1-based, inclusive).
    ///
    /// # Errors
    /// [`FinanceError::InvalidPeriod`] if the range is empty, inverted, or outside `1..=periods`.
    pub fn cumipmt(&self, start_period: u32, end_period: u32) -> FinanceResult<f64> {
        validate_range(start_period, end_period, self.periods)?;
        let series = self.series();
        let mut total = 0.0;
        for p in start_period..=end_period {
            total += series[(p - 1) as usize].interest();
        }
        Ok(total)
    }

    /// Cumulative principal from `start_period`..=`end_period` (1-based, inclusive).
    ///
    /// # Errors
    /// [`FinanceError::InvalidPeriod`] if the range is empty, inverted, or outside `1..=periods`.
    pub fn cumprinc(&self, start_period: u32, end_period: u32) -> FinanceResult<f64> {
        validate_range(start_period, end_period, self.periods)?;
        let series = self.series();
        let mut total = 0.0;
        for p in start_period..=end_period {
            total += series[(p - 1) as usize].principal();
        }
        Ok(total)
    }

    fn period_at(&self, period: u32) -> FinanceResult<AmortizationPeriod> {
        if period == 0 || period > self.periods {
            return Err(FinanceError::InvalidPeriod {
                period,
                periods: self.periods,
                message: "period must be in 1..=periods",
            });
        }
        Ok(self.series()[(period - 1) as usize].clone())
    }

    /// Period-by-period amortization schedule.
    ///
    /// # Examples
    /// ```
    /// use finance_solution::*;
    ///
    /// let series = amortization_solution(0.08 / 12.0, 12, 10_000.0, 0.0, false)
    ///     .unwrap()
    ///     .series();
    /// for row in series.iter() {
    ///     assert_approx_equal!(row.principal() + row.interest(), row.payment());
    /// }
    /// series.print_table(true, true);
    /// ```
    ///
    /// Example terminal output (first/last rows; full 12-period table in the module docs):
    ///
    /// ```text
    /// period    payment  principal  interest     balance  principal_to_date  ...
    /// ------  ---------  ---------  --------  ----------  -----------------  ...
    ///      1  -869.8843  -803.2176  -66.6667  9_196.7824          -803.2176  ...
    ///    ...
    ///     12  -869.8843  -864.1235   -5.7608     -0.0000       -10_000.0000  ...
    /// ```
    pub fn series(&self) -> AmortizationSeries {
        build_series(
            self.rate,
            self.periods,
            self.present_value,
            self.future_value,
            self.due_at_beginning,
            self.payment,
            self.sum_of_payments,
            self.sum_of_interest,
        )
    }

    /// Print the full schedule with running totals and remaining amounts.
    pub fn print_table(&self) {
        self.series().print_table(true, true);
    }

    /// Locale-aware table (thousands separators, decimal places).
    pub fn print_table_locale(&self, locale: &num_format::Locale, precision: usize) {
        self.series()
            .print_table_locale(true, true, locale, precision);
    }
}

// ---------------------------------------------------------------------------
// Series & period
// ---------------------------------------------------------------------------

/// Period-by-period amortization rows. Derefs to `[AmortizationPeriod]`.
#[derive(Clone, Debug)]
pub struct AmortizationSeries(Vec<AmortizationPeriod>);

impl AmortizationSeries {
    pub(crate) fn new(rows: Vec<AmortizationPeriod>) -> Self {
        Self(rows)
    }

    /// Filter periods while preserving the series type (e.g. year-end months only).
    ///
    /// # Examples
    /// ```
    /// use finance_solution::*;
    ///
    /// let filtered = amortization_solution(0.01, 24, 8_000.0, 0.0, false)
    ///     .unwrap()
    ///     .series()
    ///     .filter(|row| row.period() % 12 == 0);
    /// assert_eq!(filtered.len(), 2);
    /// filtered.print_table(true, false);
    /// ```
    pub fn filter<P>(&self, predicate: P) -> Self
    where
        P: Fn(&&AmortizationPeriod) -> bool,
    {
        Self(self.iter().filter(|x| predicate(x)).cloned().collect())
    }

    /// Pretty-print the schedule to stdout.
    ///
    /// * `include_running_totals` – payments/principal/interest to date  
    /// * `include_remaining_amounts` – remaining principal / interest / payments
    ///
    /// # Example output
    ///
    /// For `amortization_solution(0.08/12, 12, 10_000, 0, false).series().print_table(true, true)`:
    ///
    /// ```text
    /// period    payment  principal  interest     balance  principal_to_date  interest_to_date  payments_to_date  principal_remaining  interest_remaining  payments_remaining
    /// ------  ---------  ---------  --------  ----------  -----------------  ----------------  ----------------  -------------------  ------------------  ------------------
    ///      1  -869.8843  -803.2176  -66.6667  9_196.7824          -803.2176          -66.6667         -869.8843          -9_196.7824           -371.9448         -9_568.7272
    ///      2  -869.8843  -808.5724  -61.3119  8_388.2100        -1_611.7900         -127.9785       -1_739.7686          -8_388.2100           -310.6329         -8_698.8429
    ///      3  -869.8843  -813.9629  -55.9214  7_574.2471        -2_425.7529         -183.8999       -2_609.6529          -7_574.2471           -254.7115         -7_828.9586
    ///      4  -869.8843  -819.3893  -50.4950  6_754.8578        -3_245.1422         -234.3949       -3_479.5372          -6_754.8578           -204.2166         -6_959.0743
    ///      5  -869.8843  -824.8519  -45.0324  5_930.0059        -4_069.9941         -279.4273       -4_349.4215          -5_930.0059           -159.1842         -6_089.1900
    ///      6  -869.8843  -830.3509  -39.5334  5_099.6549        -4_900.3451         -318.9607       -5_219.3057          -5_099.6549           -119.6508         -5_219.3057
    ///      7  -869.8843  -835.8866  -33.9977  4_263.7684        -5_736.2316         -352.9584       -6_089.1900          -4_263.7684            -85.6531         -4_349.4215
    ///      8  -869.8843  -841.4592  -28.4251  3_422.3092        -6_577.6908         -381.3835       -6_959.0743          -3_422.3092            -57.2280         -3_479.5372
    ///      9  -869.8843  -847.0689  -22.8154  2_575.2403        -7_424.7597         -404.1989       -7_828.9586          -2_575.2403            -34.4126         -2_609.6529
    ///     10  -869.8843  -852.7160  -17.1683  1_722.5243        -8_277.4757         -421.3672       -8_698.8429          -1_722.5243            -17.2443         -1_739.7686
    ///     11  -869.8843  -858.4008  -11.4835    864.1235        -9_135.8765         -432.8507       -9_568.7272            -864.1235             -5.7608           -869.8843
    ///     12  -869.8843  -864.1235   -5.7608     -0.0000       -10_000.0000         -438.6115      -10_438.6115               0.0000             -0.0000              0.0000
    /// ```
    pub fn print_table(&self, include_running_totals: bool, include_remaining_amounts: bool) {
        self.print_table_locale_opt(
            include_running_totals,
            include_remaining_amounts,
            None,
            None,
        );
    }

    /// Locale-aware pretty-print.
    pub fn print_table_locale(
        &self,
        include_running_totals: bool,
        include_remaining_amounts: bool,
        locale: &num_format::Locale,
        precision: usize,
    ) {
        self.print_table_locale_opt(
            include_running_totals,
            include_remaining_amounts,
            Some(locale),
            Some(precision),
        );
    }

    fn print_table_locale_opt(
        &self,
        include_running_totals: bool,
        include_remaining_amounts: bool,
        locale: Option<&num_format::Locale>,
        precision: Option<usize>,
    ) {
        let columns = columns_with_strings(&[
            ("period", "i", true),
            ("payment", "f", true),
            ("principal", "f", true),
            ("interest", "f", true),
            ("balance", "f", true),
            ("principal_to_date", "f", include_running_totals),
            ("interest_to_date", "f", include_running_totals),
            ("payments_to_date", "f", include_running_totals),
            ("principal_remaining", "f", include_remaining_amounts),
            ("interest_remaining", "f", include_remaining_amounts),
            ("payments_remaining", "f", include_remaining_amounts),
        ]);
        let data = self
            .iter()
            .map(|e| {
                vec![
                    e.period.to_string(),
                    e.payment.to_string(),
                    e.principal.to_string(),
                    e.interest.to_string(),
                    e.balance.to_string(),
                    e.principal_to_date.to_string(),
                    e.interest_to_date.to_string(),
                    e.payments_to_date.to_string(),
                    e.principal_remaining.to_string(),
                    e.interest_remaining.to_string(),
                    e.payments_remaining.to_string(),
                ]
            })
            .collect::<Vec<_>>();
        print_table_locale_opt(&columns, data, locale, precision);
    }
}

impl Deref for AmortizationSeries {
    type Target = Vec<AmortizationPeriod>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// One period of an amortization schedule.
#[derive(Clone, Debug)]
pub struct AmortizationPeriod {
    period: u32,
    rate: f64,
    payment: f64,
    principal: f64,
    interest: f64,
    principal_to_date: f64,
    interest_to_date: f64,
    principal_remaining: f64,
    interest_remaining: f64,
    payments_to_date: f64,
    payments_remaining: f64,
    /// Outstanding principal balance **after** this period's payment.
    balance: f64,
    formula: String,
    symbolic_formula: String,
}

impl AmortizationPeriod {
    pub fn period(&self) -> u32 {
        self.period
    }
    pub fn rate(&self) -> f64 {
        self.rate
    }
    pub fn payment(&self) -> f64 {
        self.payment
    }
    pub fn principal(&self) -> f64 {
        self.principal
    }
    pub fn interest(&self) -> f64 {
        self.interest
    }
    pub fn principal_to_date(&self) -> f64 {
        self.principal_to_date
    }
    pub fn interest_to_date(&self) -> f64 {
        self.interest_to_date
    }
    pub fn principal_remaining(&self) -> f64 {
        self.principal_remaining
    }
    pub fn interest_remaining(&self) -> f64 {
        self.interest_remaining
    }
    pub fn payments_to_date(&self) -> f64 {
        self.payments_to_date
    }
    pub fn payments_remaining(&self) -> f64 {
        self.payments_remaining
    }
    pub fn balance(&self) -> f64 {
        self.balance
    }
    pub fn formula(&self) -> &str {
        &self.formula
    }
    pub fn symbolic_formula(&self) -> &str {
        &self.symbolic_formula
    }
}

// ---------------------------------------------------------------------------
// Constructors
// ---------------------------------------------------------------------------

/// Build an amortization solution (payment + schedule access).
///
/// Prefer this when you want formulas and tables, not only a single `f64`.
/// Timing accepts [`PaymentTiming`] or `bool` (`false` = end of period).
///
/// # Errors
/// Returns [`FinanceError`] for invalid rates, non-finite amounts, zero periods,
/// or payment calculation failures (same domain as [`crate::payment`]).
///
/// # Examples
/// ```
/// use finance_solution::*;
///
/// let solution = amortization_solution(0.005, 24, 12_000.0, 0.0, false).unwrap();
/// assert!(solution.payment().is_finite());
/// assert_eq!(solution.series().len(), 24);
///
/// // Year-1 interest (months 1–12)
/// let y1 = solution.cumipmt(1, 12).unwrap();
/// assert!(y1.is_finite());
/// ```
///
/// Match structured errors:
/// ```
/// use finance_solution::{amortization_solution, FinanceError};
///
/// match amortization_solution(0.01, 12, 5_000.0, 0.0, false) {
///     Ok(sol) => {
///         assert!(sol.payment().is_finite());
///         sol.series().print_table(true, true);
///     }
///     Err(FinanceError::InvalidRate { rate }) => panic!("bad rate {rate}"),
///     Err(e) => panic!("{e}"),
/// }
///
/// assert!(matches!(
///     amortization_solution(-1.5, 12, 5_000.0, 0.0, false),
///     Err(FinanceError::InvalidRate { .. })
/// ));
/// ```
pub fn amortization_solution<P, F, T>(
    rate: f64,
    periods: u32,
    present_value: P,
    future_value: F,
    timing: T,
) -> FinanceResult<AmortizationSolution>
where
    P: Into<f64> + Copy,
    F: Into<f64> + Copy,
    T: Into<crate::PaymentTiming>,
{
    let present_value = present_value.into();
    let future_value = future_value.into();
    let due_at_beginning = timing.into().is_beginning();

    // Validation chain: compose with ? (design-pattern early return).
    require_rate_gt_minus_one(rate)?;
    require_finite("present_value", present_value)?;
    require_finite("future_value", future_value)?;
    if periods == 0 {
        return Err(FinanceError::InvalidPeriod {
            period: 0,
            periods,
            message: "periods must be greater than zero for an amortization schedule",
        });
    }

    let pmt = crate::payment(rate, periods, present_value, future_value, due_at_beginning)?;

    let rate_mult = 1.0 + rate;
    let (formula, symbolic_formula) = if rate == 0.0 {
        (
            format!(
                "{:.4} = -({:.4} + {:.4}) / {}",
                pmt, present_value, future_value, periods
            ),
            "pmt = -(pv + fv) / n".to_string(),
        )
    } else if due_at_beginning {
        (
            format!(
                "{:.4} = ((({:.4} * {:.6}^{}) + {:.4}) * {:.6}) / (({:.6}^{} - 1) * {:.6})",
                pmt,
                present_value,
                rate_mult,
                periods,
                future_value,
                -rate,
                rate_mult,
                periods,
                rate_mult
            ),
            "pmt = (((pv * (1+r)^n) + fv) * -r) / (((1+r)^n - 1) * (1+r))".to_string(),
        )
    } else {
        (
            format!(
                "{:.4} = ((({:.4} * {:.6}^{}) + {:.4}) * {:.6}) / ({:.6}^{} - 1)",
                pmt, present_value, rate_mult, periods, future_value, -rate, rate_mult, periods
            ),
            "pmt = (((pv * (1+r)^n) + fv) * -r) / ((1+r)^n - 1)".to_string(),
        )
    };

    Ok(AmortizationSolution::new(
        rate,
        periods,
        present_value,
        future_value,
        due_at_beginning,
        pmt,
        formula,
        symbolic_formula,
    ))
}

// ---------------------------------------------------------------------------
// Excel-style scalar helpers (delegate to solution series)
// ---------------------------------------------------------------------------

/// Interest portion of the payment for a single period (Excel `IPMT`).
///
/// # Arguments
/// * `rate` – periodic rate
/// * `period` – 1-based period index (`1..=periods`)
/// * `periods` – total number of periods
/// * `present_value` – principal / PV
/// * `future_value` – residual value at end (often 0)
/// * `timing` – [`PaymentTiming`] or `bool` (`false` = end of period / Excel `type=0`)
///
/// # Errors
/// Propagates construction errors from [`amortization_solution`] and
/// [`FinanceError::InvalidPeriod`] when `period` is out of range.
///
/// # Examples
/// ```
/// use finance_solution::*;
/// let interest = ipmt(0.01, 1, 12, 10_000.0, 0.0, false).unwrap();
/// assert!(interest < 0.0); // opposite sign of positive principal
/// assert_rounded_2!(interest, -100.0);
/// ```
///
/// For schedules and tables, prefer [`amortization_solution`]:
/// ```
/// use finance_solution::*;
/// let s = amortization_solution(0.01, 12, 10_000.0, 0.0, false).unwrap();
/// assert_approx_equal!(
///     s.ipmt(1).unwrap(),
///     ipmt(0.01, 1, 12, 10_000.0, 0.0, false).unwrap()
/// );
/// ```
///
/// Out-of-range period:
/// ```
/// use finance_solution::{ipmt, FinanceError};
///
/// match ipmt(0.01, 1, 12, 10_000.0, 0.0, false) {
///     Ok(interest) => assert!(interest < 0.0),
///     Err(FinanceError::InvalidPeriod { period, periods, .. }) => {
///         panic!("period {period} not in 1..={periods}");
///     }
///     Err(e) => panic!("{e}"),
/// }
///
/// assert!(matches!(
///     ipmt(0.01, 0, 12, 10_000.0, 0.0, false),
///     Err(FinanceError::InvalidPeriod { .. })
/// ));
/// ```
pub fn ipmt<P, F, T>(
    rate: f64,
    period: u32,
    periods: u32,
    present_value: P,
    future_value: F,
    timing: T,
) -> FinanceResult<f64>
where
    P: Into<f64> + Copy,
    F: Into<f64> + Copy,
    T: Into<crate::PaymentTiming>,
{
    amortization_solution(rate, periods, present_value, future_value, timing)?.ipmt(period)
}

/// Principal portion of the payment for a single period (Excel `PPMT`).
///
/// # Errors
/// Same domain as [`ipmt`].
///
/// # Examples
/// ```
/// use finance_solution::*;
/// let principal = ppmt(0.01, 1, 12, 10_000.0, 0.0, false).unwrap();
/// let interest = ipmt(0.01, 1, 12, 10_000.0, 0.0, false).unwrap();
/// let pmt = payment(0.01, 12, 10_000.0, 0.0, false).unwrap();
/// assert_approx_equal!(principal + interest, pmt);
/// ```
pub fn ppmt<P, F, T>(
    rate: f64,
    period: u32,
    periods: u32,
    present_value: P,
    future_value: F,
    timing: T,
) -> FinanceResult<f64>
where
    P: Into<f64> + Copy,
    F: Into<f64> + Copy,
    T: Into<crate::PaymentTiming>,
{
    amortization_solution(rate, periods, present_value, future_value, timing)?.ppmt(period)
}

/// Cumulative principal paid between two periods inclusive (Excel `CUMPRINC`).
///
/// `start_period` and `end_period` are 1-based; require `start_period <= end_period`.
///
/// # Errors
/// Construction failures plus [`FinanceError::InvalidPeriod`] for bad ranges.
pub fn cumprinc<P, F, T>(
    rate: f64,
    periods: u32,
    present_value: P,
    future_value: F,
    start_period: u32,
    end_period: u32,
    timing: T,
) -> FinanceResult<f64>
where
    P: Into<f64> + Copy,
    F: Into<f64> + Copy,
    T: Into<crate::PaymentTiming>,
{
    amortization_solution(rate, periods, present_value, future_value, timing)?
        .cumprinc(start_period, end_period)
}

/// Cumulative interest paid between two periods inclusive (Excel `CUMIPMT`).
///
/// # Errors
/// Same domain as [`cumprinc`].
pub fn cumipmt<P, F, T>(
    rate: f64,
    periods: u32,
    present_value: P,
    future_value: F,
    start_period: u32,
    end_period: u32,
    timing: T,
) -> FinanceResult<f64>
where
    P: Into<f64> + Copy,
    F: Into<f64> + Copy,
    T: Into<crate::PaymentTiming>,
{
    amortization_solution(rate, periods, present_value, future_value, timing)?
        .cumipmt(start_period, end_period)
}

// ---------------------------------------------------------------------------
// Internals
// ---------------------------------------------------------------------------

fn validate_range(start_period: u32, end_period: u32, periods: u32) -> FinanceResult<()> {
    if periods == 0 {
        return Err(FinanceError::InvalidPeriod {
            period: 0,
            periods,
            message: "periods must be greater than zero",
        });
    }
    if start_period == 0 || start_period > periods {
        return Err(FinanceError::InvalidPeriod {
            period: start_period,
            periods,
            message: "start_period must be in 1..=periods",
        });
    }
    if end_period == 0 || end_period > periods {
        return Err(FinanceError::InvalidPeriod {
            period: end_period,
            periods,
            message: "end_period must be in 1..=periods",
        });
    }
    if start_period > end_period {
        return Err(FinanceError::InvalidPeriod {
            period: start_period,
            periods,
            message: "start_period must be <= end_period",
        });
    }
    Ok(())
}

fn build_series(
    rate: f64,
    periods: u32,
    present_value: f64,
    _future_value: f64,
    due_at_beginning: bool,
    pmt: f64,
    sum_of_payments: f64,
    sum_of_interest: f64,
) -> AmortizationSeries {
    let mut rows = Vec::with_capacity(periods as usize);
    let mut balance = present_value;
    let mut payments_to_date = 0.0;
    let mut principal_to_date = 0.0;
    let mut interest_to_date = 0.0;

    for period in 1..=periods {
        let balance_at_start = balance;
        let (interest, principal, formula, symbolic_formula) = if due_at_beginning && period == 1 {
            (0.0, pmt, "0.0000".to_string(), "interest = 0".to_string())
        } else {
            let interest = -balance_at_start * rate;
            let principal = pmt - interest;
            let formula = format!(
                "interest {:.4} = -({:.4} * {:.6}); principal {:.4} = pmt {:.4} - interest",
                interest, balance_at_start, rate, principal, pmt
            );
            let symbolic =
                "interest = -(balance_start * r); principal = pmt - interest".to_string();
            (interest, principal, formula, symbolic)
        };

        balance += principal;
        payments_to_date += pmt;
        principal_to_date += principal;
        interest_to_date += interest;

        rows.push(AmortizationPeriod {
            period,
            rate,
            payment: pmt,
            principal,
            interest,
            principal_to_date,
            interest_to_date,
            principal_remaining: -(present_value + principal_to_date),
            interest_remaining: sum_of_interest - interest_to_date,
            payments_to_date,
            payments_remaining: sum_of_payments - payments_to_date,
            balance,
            formula,
            symbolic_formula,
        });
    }

    AmortizationSeries::new(rows)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_ppmt_ipmt_sum_to_payment() {
        let rate = 0.08 / 12.0;
        let periods = 60;
        let pv = 13_000.0;
        let pmt = payment(rate, periods, pv, 0.0, false).unwrap();
        for period in 1..=periods {
            let princ = ppmt(rate, period, periods, pv, 0.0, false).unwrap();
            let int = ipmt(rate, period, periods, pv, 0.0, false).unwrap();
            assert_approx_equal!(princ + int, pmt);
        }
    }

    #[test]
    fn test_cumprinc_full_term_near_principal() {
        let rate = 0.08 / 12.0;
        let periods = 60;
        let pv = 13_000.0;
        let total_principal = cumprinc(rate, periods, pv, 0.0, 1, periods, false).unwrap();
        assert_approx_equal!(total_principal, -pv);
    }

    #[test]
    fn test_cumipmt_matches_sum_of_interest() {
        let rate = 0.08 / 12.0;
        let periods = 24;
        let pv = 10_000.0;
        let solution = amortization_solution(rate, periods, pv, 0.0, false).unwrap();
        let cum_int = cumipmt(rate, periods, pv, 0.0, 1, periods, false).unwrap();
        assert_approx_equal!(cum_int, solution.sum_of_interest());
    }

    #[test]
    fn test_first_period_interest() {
        assert_rounded_2!(ipmt(0.01, 1, 12, 10_000.0, 0.0, false).unwrap(), -100.0);
    }

    #[test]
    fn test_ipmt_bad_period() {
        assert!(matches!(
            ipmt(0.01, 0, 12, 1000.0, 0.0, false),
            Err(FinanceError::InvalidPeriod { .. })
        ));
        assert!(matches!(
            ipmt(0.01, 13, 12, 1000.0, 0.0, false),
            Err(FinanceError::InvalidPeriod { .. })
        ));
    }

    #[test]
    fn test_solution_series_len_and_formulas() {
        let s = amortization_solution(0.01, 6, 1000.0, 0.0, false).unwrap();
        let series = s.series();
        assert_eq!(series.len(), 6);
        assert!(!s.formula().is_empty());
        assert!(!s.symbolic_formula().is_empty());
        assert!(!series[0].formula().is_empty());
        assert_approx_equal!(s.ipmt(1).unwrap(), series[0].interest());
        assert_approx_equal!(s.ppmt(3).unwrap(), series[2].principal());
    }

    #[test]
    fn test_amortization_invalid_rate() {
        assert!(matches!(
            amortization_solution(-1.0, 12, 1000.0, 0.0, false),
            Err(FinanceError::InvalidRate { .. })
        ));
    }

    #[test]
    fn test_amortization_zero_periods() {
        assert!(matches!(
            amortization_solution(0.01, 0, 1000.0, 0.0, false),
            Err(FinanceError::InvalidPeriod { .. })
        ));
    }

    #[test]
    fn test_cumipmt_invalid_range() {
        let s = amortization_solution(0.01, 12, 1000.0, 0.0, false).unwrap();
        assert!(s.cumipmt(5, 3).is_err());
        assert!(s.cumipmt(0, 5).is_err());
        assert!(s.cumipmt(1, 13).is_err());
    }

    #[test]
    fn test_payment_timing_enum_matches_bool() {
        let a = amortization_solution(0.01, 12, 5_000.0, 0.0, false).unwrap();
        let b = amortization_solution(0.01, 12, 5_000.0, 0.0, PaymentTiming::EndOfPeriod).unwrap();
        assert_approx_equal!(a.payment(), b.payment());
        let due = amortization_solution(0.01, 12, 5_000.0, 0.0, true).unwrap();
        assert!(due.payment().abs() < a.payment().abs());
    }
}
