// extern crate num_traits;
// extern crate ordered_float;
extern crate float_cmp;
extern crate finance;

pub mod alternative_1;
//pub mod format;
pub mod verbose;


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

/// Convert APR to all variations of the rate. Returns a custom type with additional functionality and extra information available in the dbg!().
#[macro_export]
macro_rules! apr {
    ( $x1:expr, $x2:expr ) => {{
        let apr: f64 = $x1;
        let compounding_periods_in_year: u32 = $x2;
        finance::convert_rate::assert_inputs(apr, compounding_periods_in_year, finance::tvm_convert_rate::ConvertRateVariable::Apr);
        let ear = (1_f64 + (apr/compounding_periods_in_year as f64)).powf(compounding_periods_in_year as f64) - 1_f64;
        let epr = finance::convert_rate::convert_apr_to_epr(apr, compounding_periods_in_year);  
        let apr_in_percent = format!("{:.4}%", apr * 100.);
        let epr_in_percent = format!("{:.4}%", epr * 100.);
        let ear_in_percent = format!("{:.4}%", ear * 100.);
        let apr_formula = format!("");
        let epr_formula = format!("{} / {}", apr, compounding_periods_in_year);
        let ear_formula = format!("(1 + ({}/{}))^{} - 1", apr, compounding_periods_in_year, compounding_periods_in_year);
        finance::tvm_convert_rate::ConvertRateSolution::new(finance::tvm_convert_rate::ConvertRateVariable::Apr, apr, compounding_periods_in_year, apr_in_percent, epr_in_percent, ear_in_percent, apr, epr, ear, &apr_formula, &epr_formula, &ear_formula,)
    }}
}

/// Convert EPR to all variations of the rate. Returns a custom type with additional functionality and extra information available in the dbg!().
#[macro_export]
    macro_rules! epr {
        ( $x1:expr, $x2:expr ) => {{
            let epr: f64 = $x1;
            let compounding_periods_in_year: u32 = $x2;
            finance::convert_rate::assert_inputs(epr, compounding_periods_in_year, finance::tvm_convert_rate::ConvertRateVariable::Epr);
            let apr = epr * compounding_periods_in_year as f64;
            let ear = finance::convert_rate::convert_apr_to_ear(apr, compounding_periods_in_year);
            let apr_in_percent = format!("{:.4}%", apr * 100.);
            let epr_in_percent = format!("{:.4}%", epr * 100.);
            let ear_in_percent = format!("{:.4}%", ear * 100.);
            let apr_formula = format!("{} * {}", epr, compounding_periods_in_year);
            let epr_formula = format!("");
            let ear_formula = format!("(1 + {})^{} - 1", epr, compounding_periods_in_year);
            finance::tvm_convert_rate::ConvertRateSolution::new(finance::tvm_convert_rate::ConvertRateVariable::Epr, epr, compounding_periods_in_year, apr_in_percent, epr_in_percent, ear_in_percent, apr, epr, ear, &apr_formula, &epr_formula, &ear_formula,)
        }}
    }

/// Convert EAR to all variations of the rate. Returns a custom type with additional functionality and extra information available in the dbg!().
#[macro_export]
    macro_rules! ear {
        ( $x1:expr, $x2:expr ) => {{
            let ear: f64 = $x1;
            let compounding_periods_in_year: u32 = $x2;
            finance::convert_rate::assert_inputs(ear, compounding_periods_in_year, finance::tvm_convert_rate::ConvertRateVariable::Ear);
            let apr = finance::convert_rate::convert_ear_to_apr(ear, compounding_periods_in_year);
            let epr = finance::convert_rate::convert_ear_to_epr(ear, compounding_periods_in_year);
            let apr_in_percent = format!("{:.4}%", apr * 100.);
            let epr_in_percent = format!("{:.4}%", epr * 100.);
            let ear_in_percent = format!("{:.4}%", ear * 100.);
            let apr_formula = format!("{} * {}", epr, compounding_periods_in_year);
            let epr_formula = format!("(1 + {})^(1 / {}) - 1", ear, compounding_periods_in_year);
            let ear_formula = format!("{}", ear);
            finance::tvm_convert_rate::ConvertRateSolution::new(finance::tvm_convert_rate::ConvertRateVariable::Ear, ear, compounding_periods_in_year, apr_in_percent, epr_in_percent, ear_in_percent, apr, epr, ear, &apr_formula, &epr_formula, &ear_formula,)
        }}
    }