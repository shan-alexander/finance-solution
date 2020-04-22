extern crate float_cmp;

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

use tvm_convert_rate::*;
use convert_rate::*;

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
macro_rules! assert_approx_equal_symmetry_test {
    ( $x1:expr, $x2:expr ) => {
        if ($x1 > 0.000001 && $x1 < 1_000_000.0 && $x2 > 0.000001 && $x2 < 1_000_000.0) {
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
#[macro_export]
macro_rules! apr {
    ( $x1:expr, $x2:expr ) => {{
        let apr: f64 = $x1;
        let compounding_periods_in_year: u32 = $x2;
        convert_rate::assert_inputs(apr, compounding_periods_in_year, tvm_convert_rate::ConvertRateVariable::Apr);
        let ear = (1_f64 + (apr/compounding_periods_in_year as f64)).powf(compounding_periods_in_year as f64) - 1_f64;
        let epr = convert_rate::convert_apr_to_epr(apr, compounding_periods_in_year);  
        let apr_in_percent = format!("{:.4}%", apr * 100.);
        let epr_in_percent = format!("{:.4}%", epr * 100.);
        let ear_in_percent = format!("{:.4}%", ear * 100.);
        let apr_formula = format!("");
        let epr_formula = format!("{} / {}", apr, compounding_periods_in_year);
        let ear_formula = format!("(1 + ({}/{}))^{} - 1", apr, compounding_periods_in_year, compounding_periods_in_year);
        tvm_convert_rate::ConvertRateSolution::new(tvm_convert_rate::ConvertRateVariable::Apr, apr, compounding_periods_in_year, apr_in_percent, epr_in_percent, ear_in_percent, apr, epr, ear, &apr_formula, &epr_formula, &ear_formula,)
    }}
}

/// Convert EPR to all variations of the rate. Returns a custom type with additional functionality and extra information available in the dbg!().
#[macro_export]
    macro_rules! epr {
        ( $x1:expr, $x2:expr ) => {{
            let epr: f64 = $x1;
            let compounding_periods_in_year: u32 = $x2;
            convert_rate::assert_inputs(epr, compounding_periods_in_year, tvm_convert_rate::ConvertRateVariable::Epr);
            let apr = epr * compounding_periods_in_year as f64;
            let ear = convert_rate::convert_apr_to_ear(apr, compounding_periods_in_year);
            let apr_in_percent = format!("{:.4}%", apr * 100.);
            let epr_in_percent = format!("{:.4}%", epr * 100.);
            let ear_in_percent = format!("{:.4}%", ear * 100.);
            let apr_formula = format!("{} * {}", epr, compounding_periods_in_year);
            let epr_formula = format!("");
            let ear_formula = format!("(1 + {})^{} - 1", epr, compounding_periods_in_year);
            tvm_convert_rate::ConvertRateSolution::new(tvm_convert_rate::ConvertRateVariable::Epr, epr, compounding_periods_in_year, apr_in_percent, epr_in_percent, ear_in_percent, apr, epr, ear, &apr_formula, &epr_formula, &ear_formula,)
        }}
    }

/// Convert EAR to all variations of the rate. Returns a custom type with additional functionality and extra information available in the dbg!().
#[macro_export]
    macro_rules! ear {
        ( $x1:expr, $x2:expr ) => {{
            let ear: f64 = $x1;
            let compounding_periods_in_year: u32 = $x2;
            convert_rate::assert_inputs(ear, compounding_periods_in_year, tvm_convert_rate::ConvertRateVariable::Ear);
            let apr = convert_rate::convert_ear_to_apr(ear, compounding_periods_in_year);
            let epr = convert_rate::convert_ear_to_epr(ear, compounding_periods_in_year);
            let apr_in_percent = format!("{:.4}%", apr * 100.);
            let epr_in_percent = format!("{:.4}%", epr * 100.);
            let ear_in_percent = format!("{:.4}%", ear * 100.);
            let apr_formula = format!("{} * {}", epr, compounding_periods_in_year);
            let epr_formula = format!("(1 + {})^(1 / {}) - 1", ear, compounding_periods_in_year);
            let ear_formula = format!("{}", ear);
            tvm_convert_rate::ConvertRateSolution::new(tvm_convert_rate::ConvertRateVariable::Ear, ear, compounding_periods_in_year, apr_in_percent, epr_in_percent, ear_in_percent, apr, epr, ear, &apr_formula, &epr_formula, &ear_formula,)
        }}
    }

    #[macro_export]
    macro_rules! repeat {
    ( $x1:expr, $x2:expr ) => {
        let mut repeats = vec![];
        for i in 0..$x2 {
            repeats.push($x1);
        }
        repeats
    };
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



