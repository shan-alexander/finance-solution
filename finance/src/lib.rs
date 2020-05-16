use num_format::{Locale, ToFormattedString};
use itertools::Itertools;

extern crate float_cmp;
pub extern crate num_format;

pub mod future_value;
#[doc(inline)]
pub use future_value::*;

pub mod payment;
#[doc(inline)]
pub use payment::*;

pub mod present_value;
#[doc(inline)]
pub use present_value::*;

pub mod present_value_annuity;
#[doc(inline)]
pub use present_value_annuity::*;

pub mod future_value_annuity;
#[doc(inline)]
pub use future_value_annuity::*;

pub mod net_present_value;
#[doc(inline)]
pub use net_present_value::*;

pub mod rate;
#[doc(inline)]
pub use rate::*;

pub mod periods;
#[doc(inline)]
pub use periods::*;

pub mod convert_rate;
#[doc(inline)]
pub use convert_rate::*;

pub mod round;
#[doc(inline)]
pub use round::*;

pub mod tvm_cashflow;
#[doc(inline)]
pub use tvm_cashflow::*;

pub mod tvm_simple;
#[doc(inline)]
pub use tvm_simple::*;

pub mod tvm_convert_rate;
#[doc(inline)]
pub use tvm_convert_rate::*;
use std::cmp::max;

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
        assert!(float_cmp::approx_eq!(f64, $x1, $x2, epsilon = 0.000001, ulps = 20));
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
        if (($x1 > 0.000001 && $x1 < 1_000_000.0) || ($x1 < -0.000001 && $x1 > -1_000_000.0)) && (($x2 > 0.000001 && $x2 < 1_000_000.0) || ($x2 < -0.000001 && $x2 > -1_000_000.0)) {
            float_cmp::approx_eq!(f64, $x1, $x2, epsilon = 0.00000001, ulps = 2)
        } else {
            true
        }
    };
}

#[macro_export]
macro_rules! assert_approx_equal_symmetry_test {
    ( $x1:expr, $x2:expr ) => {
        if (($x1 > 0.000001 && $x1 < 1_000_000.0) || ($x1 < -0.000001 && $x1 > -1_000_000.0)) && (($x2 > 0.000001 && $x2 < 1_000_000.0) || ($x2 < -0.000001 && $x2 > -1_000_000.0)) {
            assert!(float_cmp::approx_eq!(f64, $x1, $x2, epsilon = 0.00000001, ulps = 2));
        }
    };
}

#[macro_export]
macro_rules! assert_rounded_2 {
    ( $x1:expr, $x2:expr ) => {
        assert_eq!(($x1 * 100.0f64).round() / 100.0, ($x2 * 100.0f64).round() / 100.0);
    };
}

#[macro_export]
macro_rules! assert_rounded_4 {
    ( $x1:expr, $x2:expr ) => {
        assert_eq!(($x1 * 10_000.0f64).round() / 10_000.0, ($x2 * 10_000.0f64).round() / 10_000.0);
    };
}

#[macro_export]
macro_rules! assert_rounded_6 {
    ( $x1:expr, $x2:expr ) => {
        assert_eq!(($x1 * 1_000_000.0f64).round() / 1_000_000.0, ($x2 * 1_000_000f64).round() / 1_000_000.0);
    };
}

#[macro_export]
macro_rules! assert_rounded_8 {
    ( $x1:expr, $x2:expr ) => {
        assert_eq!(($x1 * 100_000_000.0f64).round() / 100_000_000.0, ($x2 * 100_000_000.0f64).round() / 100_000_000.0);
    };
}

/// Convert APR to all variations of the rate. Returns a custom type with additional functionality and extra information available in the dbg!().
// #[macro_export]
// macro_rules! apr {
//     ( $x1:expr ) => {
//         finance::convert_rate::apr_continuous($x1)
//     };
//     ( $x1:expr, $x2:expr ) => {
//         finance::convert_rate::apr($x1, $x2)
//     };
//     ( $x1:expr, $x2:expr, $x3:expr ) => {
//         if $x3 == "Continuous" {
//             finance::convert_rate::apr_continuous($x1)
//         } else if $x3 == "Default" {
//             finance::convert_rate::apr($x1, $x2)
//         } else {
//             panic!("Something went wrong in the apr! macro")
//         }
//     };
// }

// /// Convert EPR to all variations of the rate. Returns a custom type with additional functionality and extra information available in the dbg!().
// #[macro_export]
//     macro_rules! epr {
//         ( $x1:expr, $x2:expr ) => {{
//             let epr: f64 = $x1;
//             let compounding_periods_in_year: u32 = $x2;
//             convert_rate::assert_inputs(epr, compounding_periods_in_year, tvm_convert_rate::ConvertRateVariable::Epr);
//             let apr = epr * compounding_periods_in_year as f64;
//             let ear = convert_rate::convert_apr_to_ear(apr, compounding_periods_in_year);
//             let apr_in_percent = format!("{:.4}%", apr * 100.);
//             let epr_in_percent = format!("{:.4}%", epr * 100.);
//             let ear_in_percent = format!("{:.4}%", ear * 100.);
//             let apr_formula = format!("{} * {}", epr, compounding_periods_in_year);
//             let epr_formula = format!("");
//             let ear_formula = format!("(1 + {})^{} - 1", epr, compounding_periods_in_year);
//             tvm_convert_rate::ConvertRateSolution::new(tvm_convert_rate::ConvertRateVariable::Epr, epr, compounding_periods_in_year, apr_in_percent, epr_in_percent, ear_in_percent, apr, epr, ear, &apr_formula, &epr_formula, &ear_formula,)
//         }}
//     }

// /// Convert EAR to all variations of the rate. Returns a custom type with additional functionality and extra information available in the dbg!().
// #[macro_export]
//     macro_rules! ear {
//         ( $x1:expr, $x2:expr ) => {{
//             let ear: f64 = $x1;
//             let compounding_periods_in_year: u32 = $x2;
//             convert_rate::assert_inputs(ear, compounding_periods_in_year, tvm_convert_rate::ConvertRateVariable::Ear);
//             let apr = convert_rate::convert_ear_to_apr(ear, compounding_periods_in_year);
//             let epr = convert_rate::convert_ear_to_epr(ear, compounding_periods_in_year);
//             let apr_in_percent = format!("{:.4}%", apr * 100.);
//             let epr_in_percent = format!("{:.4}%", epr * 100.);
//             let ear_in_percent = format!("{:.4}%", ear * 100.);
//             let apr_formula = format!("{} * {}", epr, compounding_periods_in_year);
//             let epr_formula = format!("(1 + {})^(1 / {}) - 1", ear, compounding_periods_in_year);
//             let ear_formula = format!("{}", ear);
//             tvm_convert_rate::ConvertRateSolution::new(tvm_convert_rate::ConvertRateVariable::Ear, ear, compounding_periods_in_year, apr_in_percent, epr_in_percent, ear_in_percent, apr, epr, ear, &apr_formula, &epr_formula, &ear_formula,)
//         }}
//     }

//     #[macro_export]
//     macro_rules! repeat {
//     ( $x1:expr, $x2:expr ) => {{
//         let mut repeats = vec![];
//         for i in 0..$x2 {
//             repeats.push($x1);
//         }
//         repeats
//     }};
// }

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

pub(crate) fn format_int_locale_opt<T>(val: T, locale: Option<&Locale>) -> String
    where T: ToFormattedString
{
    match locale {
        Some(locale) => val.to_formatted_string(locale),
        None => val.to_formatted_string(&Locale::en).replace(",", "_"),
    }
}

pub(crate) fn format_float_locale_opt<T>(val: T, locale: Option<&Locale>, precision: Option<usize>) -> String
    where T: Into<f64>
{
    let precision = precision.unwrap_or(4);
    let val = val.into();
    if val.is_finite() {
        // let locale = SystemLocale::default().unwrap();
        if precision == 0 {
            format_int_locale_opt(val.round() as i128, locale)
        } else {
            let left = format_int_locale_opt(val.trunc().abs() as i128, locale);
            let right = &format!("{:.*}", precision, val.fract().abs())[2..];
            let minus_sign = minus_sign_locale_opt(val as f64, locale);
            format!("{}{}{}{}", minus_sign, left, decimal_separator_locale_opt(locale), right)
        }
    } else {
        format!("{:?}", val)
    }
}

pub(crate) fn print_table_locale_opt(columns: &Vec<(&str, &str, bool)>, mut data: Vec<Vec<String>>, locale: Option<&num_format::Locale>, precision: Option<usize>) {
    if columns.len() == 0 || data.len() == 0 {
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
                if data[row_index][col_index].len() > 0 {
                    let col_type = columns[col_index].1.to_lowercase();
                    //bg!(&col_type, &data[row_index][col_index]);
                    if col_type != "s" {
                        data[row_index][col_index] = if col_type == "f" {
                            format_float_locale_opt(data[row_index][col_index].parse::<f64>().unwrap(), locale, precision)
                        } else if col_type == "i" {
                            format_int_locale_opt(data[row_index][col_index].parse::<i128>().unwrap(), locale)
                        } else {
                            panic!("Unexpected column type = \"{}\"", col_type)
                        }
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
            for row_index in 0..data.len() {
                width = max(width, data[row_index][col_index].len());
            }
            width
        } else {
            0
        };
        column_widths.push(width);
    }

    let header_line = columns.iter()
        .enumerate()
        .map(|(col_index, (header, _type, visible))|
            if *visible {
                format!("{:>width$}{}", header, column_separator, width = column_widths[col_index])
            } else {
                "".to_string()
            }
        )
        .join("");
    println!("\n{}", header_line.trim_end());

    let dash_line = columns.iter()
        .enumerate()
        .map(|(col_index, (_header, _type, visible))|
            if *visible {
                format!("{}{}", "-".repeat(column_widths[col_index]), column_separator)
            } else {
                "".to_string()
            }
        )
        .join("");
    println!("{}", dash_line.trim_end());

    for row in data.iter() {
        let value_line = row.iter()
            .enumerate()
            .map(|(col_index, value)| {
                let visible = columns[col_index].2;
                if visible {
                    format!("{:>width$}{}", value, column_separator, width = column_widths[col_index])
                } else {
                    "".to_string()
                }
            }).join("");
        println!("{}", value_line.trim_end());
    }
}

pub(crate) fn print_ab_comparison_values_string(field_name: &str, value_a: &str, value_b: &str) {
    print_ab_comparison_values_internal(field_name, value_a, value_b, false);
}

pub(crate) fn print_ab_comparison_values_int(field_name: &str, value_a: i128, value_b: i128, locale: Option<&num_format::Locale>) {
    print_ab_comparison_values_internal(
        field_name,
        &format_int_locale_opt(value_a, locale),
        &format_int_locale_opt(value_b, locale),
        true
    );
}

pub(crate) fn print_ab_comparison_values_float(field_name: &str, value_a: f64, value_b: f64, locale: Option<&num_format::Locale>, precision: Option<usize>) {
    print_ab_comparison_values_internal(
        field_name,
        &format_float_locale_opt(value_a, locale, precision),
        &format_float_locale_opt(value_b, locale, precision),
        true
    );
}

pub(crate) fn print_ab_comparison_values_bool(field_name: &str, value_a: bool, value_b: bool) {
    print_ab_comparison_values_internal(
        field_name,
        &format!("{:?}", value_a),
        &format!("{:?}", value_b),
        false
    );
}

fn print_ab_comparison_values_internal(field_name: &str, value_a: &str, value_b: &str, right_align: bool) {
    if value_a == value_b {
        println!("{}: {}", field_name, value_a);
    } else {
        if right_align {
            let width = max(value_a.len(), value_b.len());
            println!("{} a: {:>width$}", field_name, value_a, width = width);
            println!("{} b: {:>width$}", field_name, value_b, width = width);
        } else {
            println!("{} a: {}", field_name, value_a);
            println!("{} b: {}", field_name, value_b);
        }
    }
}

#[derive(Debug)]
pub enum ValueType {
    Payment,
    Rate,
}

impl ValueType {
    pub fn is_payment(&self) -> bool {
        match self {
            ValueType::Payment => true,
            _ => false,
        }
    }

    pub fn is_rate(&self) -> bool {
        match self {
            ValueType::Rate => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub enum Schedule {
    Repeating {
        value_type: ValueType,
        value: f64,
        periods: u32,
    },
    Custom {
        value_type: ValueType,
        values: Vec<f64>
    },
}

impl Schedule {

    pub fn new_repeating(value_type: ValueType, value: f64, periods: u32) -> Self {
        assert!(value.is_finite());
        Schedule::Repeating {
            value_type,
            value,
            periods,
        }
    }

    pub fn new_custom(value_type: ValueType, values: &[f64]) -> Self {
        for value in values {
            assert!(value.is_finite());
        }
        Schedule::Custom {
            value_type,
            values: values.to_vec(),
        }
    }

    pub fn is_payment(&self) -> bool {
        self.value_type().is_payment()
    }

    pub fn is_rate(&self) -> bool {
        self.value_type().is_rate()
    }

    pub fn value_type(&self) -> &ValueType {
        match self {
            Schedule::Repeating { value_type, value: _, periods: _ } => value_type,
            Schedule::Custom { value_type, values: _ } => value_type,
        }
    }

    pub fn value(&self) -> Option<f64> {
        match self {
            Schedule::Repeating{ value_type: _, value, periods: _ } => Some(*value),
            Schedule::Custom { value_type: _, values: _} => None,
        }
    }

    pub fn get(&self, index: usize) -> f64 {
        match self {
            Schedule::Repeating { value_type: _, value, periods } => {
                assert!(index < *periods as usize);
                *value
            },
            Schedule::Custom { value_type: _, values } => {
                *values.get(index).unwrap()
            },
        }
    }

    pub fn max(&self) -> Option<f64> {
        match self {
            Schedule::Repeating{ value_type: _, value, periods: _ } => Some(*value),
            Schedule::Custom { value_type: _, values} => {
                match values.len() {
                    0 => None,
                    1 => Some(values[0]),
                    // https://www.reddit.com/r/rust/comments/3fg0xr/how_do_i_find_the_max_value_in_a_vecf64/ctoa7mp/
                    _ => Some(values.iter().cloned().fold(0./0., f64::max))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
