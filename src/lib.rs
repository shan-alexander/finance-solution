//! `finance_solution` is a collection of financial functions for time-value-of-money, cashflows,
//! amortization, returns, equity path metrics, **technical analysis** (`stocks::ta`), and
//! **European option math** (`derivatives`: BSM price, Greeks, implied vol).
//!
//! In addition to being rigorously tested with symmetry tests, Excel-matching tests, and
//! property tests on TA batch↔stream parity, the library provides `solution` structs with
//! formulas and period-by-period series — useful for financial software audit trails and for
//! students of finance. Live systems can use incremental `*State` types (`push` / `push_bars`)
//! without this crate owning market data or multi-symbol orchestration.
//!
//! # Error handling (v0.1+)
//!
//! Public financial calculations return [`FinanceResult`] (`Result<T, `[`FinanceError`]`)`).
//! Invalid rates, non-finite amounts, and unsolvable inputs are **errors**, not panics.
//! Compose with `?` or match on [`FinanceError`] variants.
//!
//! There is **no dual panicking / `try_*` API** for public math: the ordinary name
//! (e.g. [`future_value`], [`payment`], [`amortization_solution`], [`SmaState::new`]) is the
//! fallible function when construction or domain validation can fail.
//!
//! ## Example
//! ```
//! use finance_solution::*;
//! let (rate, periods, present_value, is_continuous) = (0.034, 10, -1000.0, false);
//! let fv = future_value_solution(rate, periods, present_value, is_continuous).unwrap();
//! dbg!(&fv);
//! ```
//! which prints to the terminal:
//! ```text
//! fv = TvmSolution {
//!    calculated_field: FutureValue,
//!    continuous_compounding: false,
//!    rate: 0.034,
//!    periods: 10,
//!    fractional_periods: 10.0,
//!    present_value: -1000.0,
//!    future_value: 1397.0288910795477,
//!    formula: "1397.0289 = 1000.0000 * (1.034000 ^ 10)",
//!    symbolic_formula: "fv = -pv * (1 + r)^n",
//! }
//! ```
//! and if you run this line:
//! ```
//! # use finance_solution::*;
//! # let (rate, periods, present_value, is_continuous) = (0.034, 10, -1000.0, false);
//! # let fv = future_value_solution(rate, periods, present_value, is_continuous).unwrap();
//! fv.series().print_table();
//! ```
//! a pretty-printed table will be displayed in the terminal:
//! ```text
//! period      rate        value
//! ------  --------  -----------
//!      0  0.000000  -1_000.0000
//!      1  0.034000  -1_034.0000
//!      2  0.034000  -1_069.1560
//!      3  0.034000  -1_105.5073
//!      4  0.034000  -1_143.0946
//!      5  0.034000  -1_181.9598
//!      6  0.034000  -1_222.1464
//!      7  0.034000  -1_263.6994
//!      8  0.034000  -1_306.6652
//!      9  0.034000  -1_351.0918
//!     10  0.034000  -1_397.0289
//! ```
//! This can be very useful for functions in the `cashflow` family, such as a payment.
//! ```
//! # use finance_solution::*;
//! let (rate, periods, present_value, future_value, due) = (0.034, 10, 1000, 0, false);
//! let pmt = payment_solution(rate, periods, present_value, future_value, due).unwrap();
//! pmt.print_table();
//! ```
//! Which prints to the terminal:
//! ```
//! // period  payments_to_date  payments_remaining  principal  principal_to_date  principal_remaining  interest  interest_to_date  interest_remaining
//! // ------  ----------------  ------------------  ---------  -----------------  -------------------  --------  ----------------  ------------------
//! //      1         -119.6361         -1_076.7248   -85.6361           -85.6361            -914.3639  -34.0000          -34.0000           -162.3609
//! //      2         -239.2722           -957.0887   -88.5477          -174.1838            -825.8162  -31.0884          -65.0884           -131.2725
//! //      3         -358.9083           -837.4526   -91.5583          -265.7421            -734.2579  -28.0778          -93.1661           -103.1947
//! //      4         -478.5443           -717.8165   -94.6713          -360.4134            -639.5866  -24.9648         -118.1309            -78.2300
//! //      5         -598.1804           -598.1804   -97.8901          -458.3036            -541.6964  -21.7459         -139.8768            -56.4840
//! //      6         -717.8165           -478.5443  -101.2184          -559.5220            -440.4780  -18.4177         -158.2945            -38.0663
//! //      7         -837.4526           -358.9083  -104.6598          -664.1818            -335.8182  -14.9763         -173.2708            -23.0901
//! //      8         -957.0887           -239.2722  -108.2183          -772.4001            -227.5999  -11.4178         -184.6886            -11.6723
//! //      9       -1_076.7248           -119.6361  -111.8977          -884.2978            -115.7022   -7.7384         -192.4270             -3.9339
//! //     10       -1_196.3609             -0.0000  -115.7022          -999.0000              -0.0000   -3.9339         -196.3609              0.0000
//! ```
#![allow(dead_code)]

use itertools::Itertools;
use num_format::{Locale, ToFormattedString};

pub use float_cmp;
pub use num_format;

// ---------------------------------------------------------------------------
// Core utilities
// ---------------------------------------------------------------------------

pub mod util;
#[doc(inline)]
pub use util::{FinanceError, FinanceResult, Money, PeriodLength, Periods, PositivePrice, Rate};

pub mod round;
#[doc(inline)]
pub use round::*;

// ---------------------------------------------------------------------------
// Time value of money & cashflows
// ---------------------------------------------------------------------------

pub mod tvm;
#[doc(inline)]
pub use tvm::*;

pub mod cashflow;
#[doc(inline)]
pub use cashflow::*;

/// Rate conversions (APR / EAR / EPR).
/// Historical module path kept for API stability (cannot be named `rate` at the crate
/// root because [`rate`] is a TVM function).
pub mod convert_rate;
#[doc(inline)]
pub use convert_rate::*;

pub mod tvm_convert_rate;
#[doc(inline)]
pub use tvm_convert_rate::*;

// ---------------------------------------------------------------------------
// Domain modules (only modules with real, tested APIs)
// ---------------------------------------------------------------------------

pub mod amortization;
#[doc(inline)]
pub use amortization::*;

pub mod returns;
#[doc(inline)]
pub use returns::*;

pub mod stocks;
#[doc(inline)]
pub use stocks::*;

pub mod derivatives;
#[doc(inline)]
pub use derivatives::{
    bsm_greeks, bsm_implied_vol, bsm_price, bsm_solution, bsm_terms, forward_moneyness, intrinsic,
    put_call_parity_residual, spot_moneyness, time_value, BsmGreeks, BsmParams, BsmSolution,
    BsmState, BsmTerms, OptionType, ValidatedBsm,
};

use std::cmp::max;
use std::fmt::{Debug, Error, Formatter};

// use tvm_convert_rate::*;
// use convert_rate::*;

/*
#[macro_export]
macro_rules! assert_approx_equal {
    ( $x1:expr, $x2:expr ) => {
        if ($x1 * 10_000.0f64).round() / 10_000.0 != ($x2 * 10_000.0f64).round() / 10_000.0 {
            let max_length = 6;
            let mut str_1 = format!("{}", $x1);
            let mut str_2 = format!("{}", $x2);
            if str_1 == "-0.".to_string() {
                str_1 = "0.0".to_string();
            }
            if str_2 == "-0.".to_string() {
                str_2 = "0.0".to_string();
            }
            let mut length = std::cmp::min(str_1.len(), str_2.len());
            length = std::cmp::min(length, max_length);
            assert_eq!(str_1[..length], str_2[..length]);
        }
    };
}
*/

#[macro_export]
macro_rules! is_approx_equal {
    ( $x1:expr, $x2:expr ) => {
        float_cmp::approx_eq!(f64, $x1, $x2, epsilon = 0.000001, ulps = 20)
    };
}

#[macro_export]
macro_rules! assert_approx_equal {
    ( $x1:expr, $x2:expr ) => {
        assert!(float_cmp::approx_eq!(
            f64,
            $x1,
            $x2,
            epsilon = 0.000001,
            ulps = 20
        ));
    };
}

#[macro_export]
macro_rules! assert_same_sign_or_zero {
    ( $x1:expr, $x2:expr ) => {
        assert!(
            is_approx_equal!($x1, 0.0)
                || is_approx_equal!($x2, 0.0)
                || ($x1 > 0.0 && $x2 > 0.0)
                || ($x1 < -0.0 && $x2 < -0.0)
        );
    };
}

#[macro_export]
macro_rules! is_approx_equal_symmetry_test {
    ( $x1:expr, $x2:expr ) => {
        if (($x1 > 0.000001 && $x1 < 1_000_000.0) || ($x1 < -0.000001 && $x1 > -1_000_000.0))
            && (($x2 > 0.000001 && $x2 < 1_000_000.0) || ($x2 < -0.000001 && $x2 > -1_000_000.0))
        {
            float_cmp::approx_eq!(f64, $x1, $x2, epsilon = 0.00000001, ulps = 2)
        } else {
            true
        }
    };
}

#[macro_export]
macro_rules! assert_approx_equal_symmetry_test {
    ( $x1:expr, $x2:expr ) => {
        if (($x1 > 0.000001 && $x1 < 1_000_000.0) || ($x1 < -0.000001 && $x1 > -1_000_000.0))
            && (($x2 > 0.000001 && $x2 < 1_000_000.0) || ($x2 < -0.000001 && $x2 > -1_000_000.0))
        {
            assert!(float_cmp::approx_eq!(
                f64,
                $x1,
                $x2,
                epsilon = 0.00000001,
                ulps = 2
            ));
        }
    };
}

#[macro_export]
macro_rules! assert_rounded_2 {
    ( $x1:expr, $x2:expr ) => {
        assert_eq!(
            ($x1 * 100.0f64).round() / 100.0,
            ($x2 * 100.0f64).round() / 100.0
        );
    };
}

#[macro_export]
macro_rules! assert_rounded_4 {
    ( $x1:expr, $x2:expr ) => {
        assert_eq!(
            ($x1 * 10_000.0f64).round() / 10_000.0,
            ($x2 * 10_000.0f64).round() / 10_000.0
        );
    };
}

#[macro_export]
macro_rules! assert_rounded_6 {
    ( $x1:expr, $x2:expr ) => {
        assert_eq!(
            ($x1 * 1_000_000.0f64).round() / 1_000_000.0,
            ($x2 * 1_000_000f64).round() / 1_000_000.0
        );
    };
}

#[macro_export]
macro_rules! assert_rounded_8 {
    ( $x1:expr, $x2:expr ) => {
        assert_eq!(
            ($x1 * 100_000_000.0f64).round() / 100_000_000.0,
            ($x2 * 100_000_000.0f64).round() / 100_000_000.0
        );
    };
}

#[macro_export]
macro_rules! repeating_vec {
    ( $x1:expr, $x2:expr ) => {{
        let mut repeats = vec![];
        for _i in 0..$x2 {
            repeats.push($x1);
        }
        repeats
    }};
}

fn decimal_separator_locale_opt(locale: Option<&Locale>) -> String {
    match locale {
        Some(locale) => locale.decimal().to_string(),
        None => ".".to_string(),
    }
}

fn minus_sign_locale_opt(val: f64, locale: Option<&Locale>) -> String {
    if val.is_sign_negative() {
        match locale {
            Some(locale) => locale.minus_sign().to_string(),
            None => "-".to_string(),
        }
    } else {
        "".to_string()
    }
}

pub(crate) fn parse_and_format_int(val: &str) -> String {
    parse_and_format_int_locale_opt(val, None)
}

pub(crate) fn parse_and_format_int_locale_opt(val: &str, locale: Option<&Locale>) -> String {
    let float_val: f64 = val.parse().unwrap();
    if float_val.is_finite() {
        let int_val: i128 = val.parse().unwrap();
        format_int_locale_opt(int_val, locale)
    } else {
        // This is a special case where the value was originally a floating point number that we
        // normally wish to display as an integer, but it might be something like f64::INFINITY in
        // which case we'd show something like "Inf" rather than try to convert it into an integer.
        val.to_string()
    }
}

pub(crate) fn format_int<T>(val: T) -> String
where
    T: ToFormattedString,
{
    format_int_locale_opt(val, None)
}

pub(crate) fn format_int_locale_opt<T>(val: T, locale: Option<&Locale>) -> String
where
    T: ToFormattedString,
{
    match locale {
        Some(locale) => val.to_formatted_string(locale),
        None => val.to_formatted_string(&Locale::en).replace(",", "_"),
    }
}

pub(crate) fn format_float<T>(val: T) -> String
where
    T: Into<f64>,
{
    format_float_locale_opt(val, None, None)
}

pub(crate) fn format_rate<T>(val: T) -> String
where
    T: Into<f64>,
{
    format_float_locale_opt(val, None, Some(6))
}

pub(crate) fn format_float_locale_opt<T>(
    val: T,
    locale: Option<&Locale>,
    precision: Option<usize>,
) -> String
where
    T: Into<f64>,
{
    let precision = precision.unwrap_or(4);
    let val = val.into();
    if val.is_finite() {
        // Round at the requested precision *before* splitting integer / fractional
        // parts. Otherwise values like 9.999999999999998 at precision 4 become
        // "9.0000" (trunc left=9, fract formats as "1.0000"[2..]="0000").
        if precision == 0 {
            format_int_locale_opt(val.round() as i128, locale)
        } else {
            // Round to `precision` decimal places first so fractional rounding can
            // carry into the integer part (e.g. 9.99995 → 10.0000 at 4 places).
            let scale = 10_f64.powi(precision as i32);
            let rounded_abs = (val.abs() * scale).round() / scale;
            let left = format_int_locale_opt(rounded_abs.trunc() as i128, locale);
            let frac_digits = (rounded_abs.fract() * scale).round() as u64;
            let right = format!("{:0>width$}", frac_digits, width = precision);
            let minus_sign = minus_sign_locale_opt(val, locale);
            format!(
                "{}{}{}{}",
                minus_sign,
                left,
                decimal_separator_locale_opt(locale),
                right
            )
        }
    } else {
        format!("{:?}", val)
    }
}

pub(crate) fn print_table_locale_opt(
    columns: &[(String, String, bool)],
    mut data: Vec<Vec<String>>,
    locale: Option<&num_format::Locale>,
    precision: Option<usize>,
) {
    if columns.is_empty() || data.is_empty() {
        return;
    }

    let column_separator = "  ";

    let column_count = data[0].len();

    for row_index in 0..data.len() {
        for col_index in 0..column_count {
            let visible = columns[col_index].2;
            if visible {
                // If the data in this cell is an empty string we're going to leave it with that
                // value regardless of the type.
                if !data[row_index][col_index].is_empty() {
                    let col_type = columns[col_index].1.to_lowercase();
                    //bg!(&col_type, &data[row_index][col_index]);
                    if col_type != "s" {
                        if col_type == "f" || col_type == "r" {
                            let precision = if col_type == "f" {
                                precision
                            } else {
                                precision_opt_set_min(precision, 6)
                            };
                            // Non-numeric placeholders (e.g. "n/a") stay as plain strings.
                            if let Ok(n) = data[row_index][col_index].parse::<f64>() {
                                data[row_index][col_index] =
                                    format_float_locale_opt(n, locale, precision);
                            }
                        } else if col_type == "i" {
                            data[row_index][col_index] = parse_and_format_int_locale_opt(
                                &data[row_index][col_index],
                                locale,
                            );
                        }
                        // Unknown types: leave the cell as a string.
                    }
                }
            }
        }
    }

    let mut column_widths = vec![];
    for col_index in 0..column_count {
        let visible = columns[col_index].2;
        let width = if visible {
            let mut width = columns[col_index].0.len();
            for row in &data {
                width = max(width, row[col_index].len());
            }
            width
        } else {
            0
        };
        column_widths.push(width);
    }

    let header_line = columns
        .iter()
        .enumerate()
        .map(|(col_index, (header, _type, visible))| {
            if *visible {
                format!(
                    "{:>width$}{}",
                    header,
                    column_separator,
                    width = column_widths[col_index]
                )
            } else {
                "".to_string()
            }
        })
        .join("");
    println!("\n{}", header_line.trim_end());

    let dash_line = columns
        .iter()
        .enumerate()
        .map(|(col_index, (_header, _type, visible))| {
            if *visible {
                format!(
                    "{}{}",
                    "-".repeat(column_widths[col_index]),
                    column_separator
                )
            } else {
                "".to_string()
            }
        })
        .join("");
    println!("{}", dash_line.trim_end());

    for row in data.iter() {
        let value_line = row
            .iter()
            .enumerate()
            .map(|(col_index, value)| {
                let visible = columns[col_index].2;
                if visible {
                    format!(
                        "{:>width$}{}",
                        value,
                        column_separator,
                        width = column_widths[col_index]
                    )
                } else {
                    "".to_string()
                }
            })
            .join("");
        println!("{}", value_line.trim_end());
    }
}

pub(crate) fn print_ab_comparison_values_string(field_name: &str, value_a: &str, value_b: &str) {
    print_ab_comparison_values_internal(field_name, value_a, value_b, false);
}

pub(crate) fn print_ab_comparison_values_int(
    field_name: &str,
    value_a: i128,
    value_b: i128,
    locale: Option<&num_format::Locale>,
) {
    print_ab_comparison_values_internal(
        field_name,
        &format_int_locale_opt(value_a, locale),
        &format_int_locale_opt(value_b, locale),
        true,
    );
}

pub(crate) fn print_ab_comparison_values_float(
    field_name: &str,
    value_a: f64,
    value_b: f64,
    locale: Option<&num_format::Locale>,
    precision: Option<usize>,
) {
    print_ab_comparison_values_internal(
        field_name,
        &format_float_locale_opt(value_a, locale, precision),
        &format_float_locale_opt(value_b, locale, precision),
        true,
    );
}

pub(crate) fn print_ab_comparison_values_rate(
    field_name: &str,
    value_a: f64,
    value_b: f64,
    locale: Option<&num_format::Locale>,
    precision: Option<usize>,
) {
    let precision = precision_opt_set_min(precision, 6);
    print_ab_comparison_values_float(field_name, value_a, value_b, locale, precision);
}

pub(crate) fn print_ab_comparison_values_bool(field_name: &str, value_a: bool, value_b: bool) {
    print_ab_comparison_values_internal(
        field_name,
        &format!("{:?}", value_a),
        &format!("{:?}", value_b),
        false,
    );
}

fn print_ab_comparison_values_internal(
    field_name: &str,
    value_a: &str,
    value_b: &str,
    right_align: bool,
) {
    if value_a == value_b {
        println!("{}: {}", field_name, value_a);
    } else if right_align {
        let width = max(value_a.len(), value_b.len());
        println!("{} a: {:>width$}", field_name, value_a, width = width);
        println!("{} b: {:>width$}", field_name, value_b, width = width);
    } else {
        println!("{} a: {}", field_name, value_a);
        println!("{} b: {}", field_name, value_b);
    }
}

fn precision_opt_set_min(precision: Option<usize>, min: usize) -> Option<usize> {
    Some(match precision {
        Some(precision) => precision.max(min),
        None => 6,
    })
}

/// Discriminator for values stored in a [`Schedule`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ValueType {
    Payment,
    Rate,
}

impl ValueType {
    pub fn is_payment(&self) -> bool {
        matches!(self, ValueType::Payment)
    }

    pub fn is_rate(&self) -> bool {
        matches!(self, ValueType::Rate)
    }
}

impl std::fmt::Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueType::Payment => write!(f, "Payment"),
            ValueType::Rate => write!(f, "Rate"),
        }
    }
}

/// Sparse or repeating schedule of rates or payments.
///
/// Construct with [`Schedule::new_repeating`] / [`Schedule::new_custom`]
/// (fallible `FinanceResult`) so non-finite values are rejected without panicking.
///
/// # Examples
/// ```
/// use finance_solution::{Schedule, ValueType};
///
/// // Repeating 5% for three periods
/// let rates = Schedule::new_repeating(ValueType::Rate, 0.05, 3)?;
/// assert_eq!(rates.len(), 3);
/// assert_eq!(rates.get(0), Some(0.05));
/// assert_eq!(rates.get(3), None); // OOB → Option, not panic
///
/// // Custom payment ladder
/// let pmts = Schedule::new_custom(ValueType::Payment, &[100.0, 110.0, 120.0])?;
/// assert_eq!(pmts.get(1), Some(110.0));
///
/// // Invalid input is Err, not abort
/// assert!(Schedule::new_repeating(ValueType::Rate, f64::NAN, 1).is_err());
/// assert!(Schedule::new_custom(ValueType::Payment, &[]).is_err());
/// # Ok::<(), finance_solution::FinanceError>(())
/// ```
#[derive(Clone, Debug)]
pub enum Schedule {
    Repeating {
        value_type: ValueType,
        value: f64,
        periods: u32,
    },
    Custom {
        value_type: ValueType,
        values: Vec<f64>,
    },
}

impl Schedule {
    /// Repeating constant value for `periods` steps.
    ///
    /// # Errors
    /// [`FinanceError::NonFinite`] if `value` is not finite.
    pub fn new_repeating(value_type: ValueType, value: f64, periods: u32) -> FinanceResult<Self> {
        crate::util::error::require_finite("value", value)?;
        Ok(Schedule::Repeating {
            value_type,
            value,
            periods,
        })
    }

    /// Custom value series (one entry per period).
    ///
    /// # Errors
    /// [`FinanceError::NonFinite`] if any value is not finite;
    /// [`FinanceError::EmptyInput`] if `values` is empty.
    pub fn new_custom(value_type: ValueType, values: &[f64]) -> FinanceResult<Self> {
        if values.is_empty() {
            return Err(FinanceError::EmptyInput { what: "values" });
        }
        for (i, &value) in values.iter().enumerate() {
            if !value.is_finite() {
                return Err(FinanceError::NonFinite {
                    field: "values",
                    value,
                });
            }
            let _ = i;
        }
        Ok(Schedule::Custom {
            value_type,
            values: values.to_vec(),
        })
    }

    pub fn is_payment(&self) -> bool {
        self.value_type().is_payment()
    }

    pub fn is_rate(&self) -> bool {
        self.value_type().is_rate()
    }

    pub fn value_type(&self) -> &ValueType {
        match self {
            Schedule::Repeating { value_type, .. } => value_type,
            Schedule::Custom { value_type, .. } => value_type,
        }
    }

    /// Constant value for a repeating schedule; `None` for custom series.
    pub fn value(&self) -> Option<f64> {
        match self {
            Schedule::Repeating { value, .. } => Some(*value),
            Schedule::Custom { .. } => None,
        }
    }

    /// Value at a 0-based index, or `None` if out of range.
    pub fn get(&self, index: usize) -> Option<f64> {
        match self {
            Schedule::Repeating { value, periods, .. } => {
                if index < *periods as usize {
                    Some(*value)
                } else {
                    None
                }
            }
            Schedule::Custom { values, .. } => values.get(index).copied(),
        }
    }

    /// Maximum value in the schedule, if any.
    pub fn max(&self) -> Option<f64> {
        match self {
            Schedule::Repeating { value, .. } => Some(*value),
            Schedule::Custom { values, .. } => {
                if values.is_empty() {
                    None
                } else {
                    Some(values.iter().cloned().fold(f64::NAN, f64::max))
                }
            }
        }
    }

    /// Number of periods / entries.
    pub fn len(&self) -> usize {
        match self {
            Schedule::Repeating { periods, .. } => *periods as usize,
            Schedule::Custom { values, .. } => values.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Debug)]
pub struct ScenarioList {
    pub setup: String,
    pub input_variable: TvmVariable,
    pub output_variable: TvmVariable,
    pub entries: Vec<ScenarioEntry>,
}

pub struct ScenarioEntry {
    pub input: f64,
    pub output: f64,
    input_precision: usize,
    output_precision: usize,
}

impl ScenarioList {
    pub(crate) fn new(
        setup: String,
        input_variable: TvmVariable,
        output_variable: TvmVariable,
        entries: Vec<(f64, f64)>,
    ) -> Self {
        let input_precision = match input_variable {
            TvmVariable::Periods => 0,
            TvmVariable::Rate => 6,
            _ => 4,
        };
        let output_precision = match output_variable {
            TvmVariable::Periods => 0,
            TvmVariable::Rate => 6,
            _ => 4,
        };
        let entries = entries
            .iter()
            .map(|entry| ScenarioEntry::new(entry.0, entry.1, input_precision, output_precision))
            .collect();
        Self {
            setup,
            input_variable,
            output_variable,
            entries,
        }
    }

    pub fn print_table(&self) {
        self.print_table_locale_opt(None, None);
    }

    pub fn print_table_locale(&self, locale: &num_format::Locale, precision: usize) {
        self.print_table_locale_opt(Some(locale), Some(precision));
    }

    fn print_table_locale_opt(
        &self,
        locale: Option<&num_format::Locale>,
        precision: Option<usize>,
    ) {
        let columns = vec![
            self.input_variable.table_column_spec(true),
            self.output_variable.table_column_spec(true),
        ];
        // let columns = columns_with_strings.iter().map(|x| &x.0[..], &x.1[..], x.2);
        let data = self
            .entries
            .iter()
            .map(|entry| vec![entry.input.to_string(), entry.output.to_string()])
            .collect::<Vec<_>>();
        print_table_locale_opt(&columns, data, locale, precision);
    }
}

impl ScenarioEntry {
    pub(crate) fn new(
        input: f64,
        output: f64,
        input_precision: usize,
        output_precision: usize,
    ) -> Self {
        Self {
            input,
            output,
            input_precision,
            output_precision,
        }
    }
}

impl Debug for ScenarioEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let input = format_float_locale_opt(self.input, None, Some(self.input_precision));
        let output = format_float_locale_opt(self.output, None, Some(self.output_precision));
        write!(f, "{{ input: {}, output: {} }}", input, output)
    }
}

pub(crate) fn columns_with_strings(columns: &[(&str, &str, bool)]) -> Vec<(String, String, bool)> {
    columns
        .iter()
        .map(|(label, data_type, visible)| (label.to_string(), data_type.to_string(), *visible))
        .collect()
}

pub(crate) fn initialized_vector<L, V>(length: L, value: V) -> Vec<V>
where
    L: Into<usize>,
    V: Copy,
{
    let mut v = vec![];
    for _ in 0..length.into() {
        v.push(value);
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schedule_new_and_get() {
        let s = Schedule::new_repeating(ValueType::Rate, 0.05, 3).unwrap();
        assert_eq!(s.len(), 3);
        assert_eq!(s.get(0), Some(0.05));
        assert_eq!(s.get(3), None);
        assert!(Schedule::new_repeating(ValueType::Rate, f64::NAN, 1).is_err());

        let c = Schedule::new_custom(ValueType::Payment, &[1.0, 2.0]).unwrap();
        assert_eq!(c.get(1), Some(2.0));
        assert_eq!(c.get(2), None);
        assert!(Schedule::new_custom(ValueType::Payment, &[]).is_err());
        assert!(Schedule::new_custom(ValueType::Payment, &[1.0, f64::INFINITY]).is_err());
    }

    #[test]
    fn test_assert_same_sign_or_zero_nominal() {
        assert_same_sign_or_zero!(0.0, 0.0);
        assert_same_sign_or_zero!(0.0, -0.0);
        assert_same_sign_or_zero!(-0.0, 0.0);
        assert_same_sign_or_zero!(-0.0, -0.0);
        assert_same_sign_or_zero!(0.023, 0.023);
        assert_same_sign_or_zero!(10.0, 0.023);
        assert_same_sign_or_zero!(-0.000045, -100.0);
        assert_same_sign_or_zero!(0.023, 0.0);
        assert_same_sign_or_zero!(0.0, 0.023);
        assert_same_sign_or_zero!(0.023, -0.0);
        assert_same_sign_or_zero!(-0.0, 0.023);
        assert_same_sign_or_zero!(-0.000045, -100.0);
        assert_same_sign_or_zero!(-0.000045, 0.0);
        assert_same_sign_or_zero!(0.0, -100.0);
        assert_same_sign_or_zero!(-0.000045, -0.0);
        assert_same_sign_or_zero!(-0.0, -100.0);
        assert_same_sign_or_zero!(100.0, -0.00000000001864464138634503);
    }

    #[should_panic]
    #[test]
    fn test_assert_same_sign_or_zero_fail_diff_sign() {
        assert_same_sign_or_zero!(-0.000045, 100.0);
    }
}
