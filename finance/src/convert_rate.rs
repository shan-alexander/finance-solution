//! **Rate conversions**. Given a rate and number of compound periods per year, what is this rate
//! when converted to APR, Effective annual, and Periodic rates? Also consider the `apr` `ear` and `epr` helper functions.
//!
//! **APR**: **Annual Percentage Rate**, also written as Nominal Rate, or annual discount rate. An annualized represenation of the interest rate.
//!
//! ><small>For general use, try the [`apr`] function by providing rate and compounds_per_year, for example `apr(0.034, 12)`.</small>
//!
//! ><small>To _calculate_ the Annual Percentage Rate (APR) of a given rate, use the [`convert_ear_to_apr`] or [`convert_epr_to_apr`] functions.</small>
//! 
//! ><small>To _convert_ an Annual Percentage Rate (APR) into a different rate, use the [`convert_apr_to_ear`] or [`convert_apr_to_epr`] functions.</small>
//! 
//! **EPR**: **Effective Periodic Rate**, also written as **Periodic Rate**. The rate of the compounding period.
//! 
//! ><small>For general use, try the [`epr`] function by providing rate and compounds_per_year, for example `epr(0.034, 12)`.</small>
//!
//! ><small>To <i>calculate</i> the Effective Periodic Rate (EPR) of a given rate use the [`convert_apr_to_epr`] or [`convert_ear_to_epr`] functions.</small>
//! 
//! ><small>To convert an Effective Period Rate (EPR) into a different rate, use the [`convert_epr_to_ear`] or [`convert_epr_to_apr`] functions.</small>
//! 
//! **EAR**: **Effective Annual Rate**. The effective rate of a year which (typically) has multiple compound periods.
//! 
//! ><small>For general use, try the [`ear`] function by providing rate and compounds_per_year, for example `ear(0.034, 12)`.</small>
//!
//! ><small>To calculate the Effective Annual Rate (EAR) of a given rate use the [`convert_apr_to_ear`] or [`convert_epr_to_ear`] functions.</small>
//!
//! ><small>To convert an Effective Annual Rate (EAR) into a different rate, use the [`convert_ear_to_apr`] or [`convert_ear_to_epr`] functions.</small>
//! 
//! # Examples
//! All functions in this module can be written with the suffix **_solution**, except for the ['apr'], ['ear'], ['epr'] functions which already provide a solution struct. 
//! The solution functions provide helpful information in the dbg!() output, for example:
//! 
//! ```
//! use finance::*;
//! // Example 1: Give the apr function an apr and compounding-periods-per-year. 
//! let rate = apr(0.034, 12);
//! dbg!(rate);
//! 
//! // prints to terminal: 
//! ```
//! >{<br>
//! >input_name: <span style="color:Magenta">Apr</span><br>
//! >input_rate: <span style="color:Orange">0.034</span><br>
//! >compounds_per_year: <span style="color:Orange">12</span><br>
//! >apr_in_percent: <span style="color:Orange">3.4000%</span><br>
//! >epr_in_percent: 0.2833%<br>
//! >ear_in_percent: 3.4535%<br>
//! >apr: 0.034<br>
//! >epr: 0.0028333333333333335<br>
//! >ear: 0.03453486936028982<br>
//! >apr_formula:<br>
//! >epr_formula: 0.034 / 12<br>
//! >ear_formula: (1 + (0.034/12))^12 - 1<br>
//! >}
//! 
//! ```
//! // example 2: explicit call to f64 function
//! use finance::*;
//! 
//! let apr = convert_apr_to_ear(0.034, 12);
//! dbg!(apr);
//! // prints to terminal: 
//! ```
//! >0.03453486936028982
//!
//! ```
//! // example 3: explicit call to _solution function
//! use finance::*;
//! 
//! let apr = convert_rate::convert_apr_to_ear_solution(0.034, 12);  // provides same output as apr! macro                                                       
//! dbg!(apr.ear());
//! // prints to terminal: 
//! ```
//!  >0.03453486936028982
//! 
//! Here are a few variations of how someone can use the `convert_rate` module functions:
//! ```
//! use finance::*;
//! 
//! // What is the future value of $500 in 1 year 
//! // if the APR is 3.4% and it's compounded monthly?
//! // Solve twice, first using EPR and then using EAR.
//! 
//! // to solve, first convert the annual rate into a periodic rate (monthly):
//! let epr = convert_rate::convert_apr_to_epr(0.034, 12);
//! assert_approx_equal!(epr, 0.002833333333333333); // true
//!
//! // then solve for future value:
//! let fv = future_value::future_value_solution;
//! let answer_1 = fv(epr, 12, 500);
//! dbg!(&answer_1);
//! // which prints:
//! ```
//! >&answer_1 = TvmSolution {<br>
//! >    calculated_field: <span style="color:Magenta">FutureValue</span>,<br>
//! >    rate: <span style="color:Orange">0.0028333333333333335</span>,<br>
//! >    periods: <span style="color:Orange">12</span>,<br>
//! >    present_value: <span style="color:Orange">500.0</span>,<br>
//! >    future_value: <span style="color:Green">517.2674346801452</span>,<br>
//! >    formula: "500.0000 * (1.002833 ^ 12)",<br>
//! >}
//! 
//! ```
//! use finance::*;
//! // then let's double-check the previous answer_1 by solving the future_value
//! // using 1 year as the period and the effective annual rate, 
//! // instead of using 12 monthly periods of the periodic rate.
//! let rate = apr(0.034, 12);
//! let answer_2 = future_value::future_value_solution(rate.ear(), 1, 500);
//! dbg!(&answer_2.future_value());
//! // assert_approx_equal!(answer_1.future_value, answer_2.future_value); // true
//! ```
//! >&answer_2.future_value = 517.2674346801449
//! 
//! Now you've learned Time-Value-of-Money problems can be
//! solved using different rates and periods, while providing the same
//! correct answer. And you've learned how to use this crate for rate conversions!

use log::{warn};

// Import needed for the function references in the Rustdoc comments.
#[allow(unused_imports)]
use crate::tvm_convert_rate::*;
use crate::*;

fn assert_inputs(rate:f64, periods:u32, fn_type: ConvertRateVariable) {
    assert!(periods >= 1);
    assert!(rate.is_finite());
    if fn_type.is_ear() { 
        assert!(rate > -1.0, "The Effective Annual Rate (EAR) must be greater than -100% or the exponential formula will create an imaginary number.");
    }
    if rate > 1. || rate < -1. {
        warn!("You provided an rate of {}%. Are you sure?", rate*100.);
    }
    if periods > 366 {
        warn!("You provided more than 366 compounding periods in a year (You provided {}). Are you sure?", periods);
    }
}


/// **Helper function** to convert an quoted annual rate (apr) into all possible conversions (ear, epr). Returns a solution struct.
pub fn apr(apr:f64, compounding_periods_in_year:u32) -> ConvertRateSolution {
    assert_inputs(apr, compounding_periods_in_year, tvm_convert_rate::ConvertRateVariable::Apr);
    let ear = (1_f64 + (apr/compounding_periods_in_year as f64)).powf(compounding_periods_in_year as f64) - 1_f64;
    let epr = convert_rate::convert_apr_to_epr(apr, compounding_periods_in_year);  
    let apr_in_percent = format!("{:.4}%", apr * 100.);
    let epr_in_percent = format!("{:.4}%", epr * 100.);
    let ear_in_percent = format!("{:.4}%", ear * 100.);
    let apr_formula = format!("");
    let epr_formula = format!("{} / {}", apr, compounding_periods_in_year);
    let ear_formula = format!("(1 + ({}/{}))^{} - 1", apr, compounding_periods_in_year, compounding_periods_in_year);
    tvm_convert_rate::ConvertRateSolution::new(tvm_convert_rate::ConvertRateVariable::Apr, apr, compounding_periods_in_year, apr_in_percent, epr_in_percent, ear_in_percent, apr, epr, ear, &apr_formula, &epr_formula, &ear_formula,)
}

/// **Helper function** to convert an APR into an EAR using continuous compounding. Returns solution struct.
pub fn apr_continuous(apr:f64) -> ConvertRateSolution {
    let compounding_periods_in_year = 1; // not used
    assert_inputs(apr, compounding_periods_in_year, tvm_convert_rate::ConvertRateVariable::AprContinuous);
    // formula: e^apr - 1
    let e: f64 = 2.71828182845904; 
    let ear: f64;
    if apr < 0.0 {
        // when apr is negative...
        ear = (e.powf(apr.abs()) - 1_f64) * -1_f64;
    } else {
        ear = e.powf(apr) - 1_f64;
    }     
    let epr = 0.0;  // epr cannot exist for infinite periods
    let apr_in_percent = format!("{:.4}%", apr * 100.);
    let epr_in_percent = format!("NaN"); // epr cannot exist for infinite periods
    let ear_in_percent = format!("{:.4}%", ear * 100.);
    let apr_formula = format!("");
    let epr_formula = format!("");
    let ear_formula = format!("({}^{} - 1", e, apr);
    tvm_convert_rate::ConvertRateSolution::new(tvm_convert_rate::ConvertRateVariable::AprContinuous, apr, compounding_periods_in_year, apr_in_percent, epr_in_percent, ear_in_percent, apr, epr, ear, &apr_formula, &epr_formula, &ear_formula,)          
}

/// **Helper function** to convert an EAR into an APR using continuous compounding. Returns solution struct.
pub fn ear_continuous(ear:f64) -> ConvertRateSolution {
    let compounding_periods_in_year = 1; // not used
    assert_inputs(ear, compounding_periods_in_year, tvm_convert_rate::ConvertRateVariable::EarContinuous);
    // formula: e^apr - 1
    let apr: f64;
    if ear < 0.0 {
        // when ear is negative...
        apr = (ear.abs() + 1_f64).ln() * -1_f64;
    } else {
        // APR = ln (EAR + 1)
        apr = (ear + 1_f64).ln();
    }     
    let epr = 0.0;  // epr cannot exist for infinite periods
    let apr_in_percent = format!("{:.4}%", apr * 100.);
    let epr_in_percent = format!("NaN"); // epr cannot exist for infinite periods
    let ear_in_percent = format!("{:.4}%", ear * 100.);
    let apr_formula = format!("(ln({} + 1)", apr);
    let epr_formula = format!("");
    let ear_formula = format!("");
    tvm_convert_rate::ConvertRateSolution::new(tvm_convert_rate::ConvertRateVariable::EarContinuous, ear, compounding_periods_in_year, apr_in_percent, epr_in_percent, ear_in_percent, apr, epr, ear, &apr_formula, &epr_formula, &ear_formula,)          
}
/// **Helper function** to convert an effective annual rate (ear) into all possible conversions (apr, epr). Returns a solution struct.
pub fn ear(ear:f64, compounding_periods_in_year:u32) -> ConvertRateSolution {
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
}

/// Helper function to convert a periodic interest rate (EPR) to all rate conversions. Returns solution struct.
/// 
/// Note: an EPR of 0.99 with a large number of periods can create decimal inaccuracies due to floating point representation. 
/// The epr conversion method is tested and guaranteed accurate up to 780 periods between rates -0.034 and 0.989 however any rates outside this
/// range may result in floating point representation / rounding errors (extremely small differences like 0.00001, but if the period count is an extreme case like 2000, this can result in a difference of $1-$4 on a TVM problem).
/// 
pub fn epr(epr:f64, compounding_periods_in_year:u32) -> ConvertRateSolution {
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
}


/// Convert a nominal interest rate (Annual rate, APR) to EAR (effective annual rate). Returns f64.
/// 
/// Related Functions:
/// * [`apr!`] macro to convert APR to all forms of rate conversion, and return a custom type with additional functionality and extra information available in the dbg!().
/// * [`convert_apr_to_ear_solution`] to convert APR to EAR and return a custom type with additional functionality and extra information available in the dbg!().
/// 
/// The formula for Apr -> Ear is:
/// (1 + (apr/periods))<sup>periods</sup> - 1
/// 
/// # Arguments
/// * `rate` - The input rate, expressed as a floating point number. 
/// For instance 0.05 indicates 5%. Often appears as `r` or `i` in formulas.
/// * `periods` - The number of compounding periods in a year. Often appears as `n` or `t`. Must be u32.
/// 
/// # Panics
/// * `periods` - must be a u32 value greater than 0.
/// 
/// # Examples
/// Convert annual rate to effective annual rate.
/// ```
/// use finance::*;
/// // The annual percentage rate is 3.4% and 12 compounding periods per year.
/// let nominal_rate = 0.034;
/// let periods = 12;
///
/// let effective_annual_rate = convert_rate::convert_apr_to_ear(nominal_rate, periods);
/// 
/// // Confirm that the future value is correct to six decimal places.
/// assert_approx_equal!(0.034535, effective_annual_rate);
/// ```
pub fn convert_apr_to_ear(apr: f64, compounding_periods_in_year: u32) -> f64 {
    assert_inputs(apr, compounding_periods_in_year, ConvertRateVariable::Apr);
    (1_f64 + (apr/compounding_periods_in_year as f64)).powf(compounding_periods_in_year as f64) - 1_f64
}

/// Convert an APR to EAR (effective annual rate). Returns a custom type with additional functionality and extra information available in the dbg!().
/// 
/// Related Functions:
/// * [`apr!`] macro to convert APR to all forms of rate conversion, and return a custom type with additional functionality and extra information available in the dbg!().
/// * [`convert_apr_to_ear`] to convert APR to EAR and return the f64 value.
/// 
/// The formula for Apr -> Ear is:
/// (1 + (apr/periods))<sup>periods</sup> - 1
/// 
/// # Arguments
/// * `rate` - The input rate, expressed as a floating point number. 
/// For instance 0.05 indicates 5%. Often appears as `r` or `i` in formulas.
/// * `compounding_periods_in_year` - The number of compounding periods in a year. Often appears as `n` or `t`. Must be u32.
/// 
/// # Panics
/// * `periods` - must be a u32 value greater than 0.
/// 
/// # Examples
/// /// Convert annual rate to effective annual rate.
/// ```
/// use finance::*;
/// // The annual percentage rate is 3.4%.
/// let nominal_rate = 0.034;
///
/// // There are 12 compounding periods per year (monthly compounding).
/// let periods = 12;
///
/// let effective_annual_rate = convert_apr_to_ear_solution(nominal_rate, periods).ear();
/// 
/// // Confirm that the future value is correct to six decimal places.
/// assert_approx_equal!(0.034535, effective_annual_rate);
/// ```

pub fn convert_apr_to_ear_solution(apr: f64, compounding_periods_in_year: u32) -> ConvertRateSolution {
    assert_inputs(apr, compounding_periods_in_year, ConvertRateVariable::Apr);
    self::apr(apr, compounding_periods_in_year)
}

/// Convert APR (annual rate) to periodic rate. Returns f64.
/// 
/// Related Functions:
/// * [`convert_apr_to_epr_solution`] to convert APR to EPR and return a custom type with additional functionality and extra information available in the dbg!().
/// 
/// The formula is:
/// apr / compounding_periods_in_year
/// 
/// # Arguments
/// * `rate` - The input rate, expressed as a floating point number. 
/// For instance 0.05 would mean 5% growth. Often appears as `r` or `i` in formulas.
/// * `compounding_periods_in_year` - The number of compounding periods in a year. Often appears as `n` or `t`.
/// 
/// # Panics
/// * `compounding_periods_in_year` - must be a u32 value greater than 0.
/// 
/// # Examples
/// Convert annual rate to periodic rate.
/// ```
/// use finance::*;
/// // The annual percentage rate is 3.4%.
/// let nominal_rate = 0.034;
///
/// // There are 12 compounding periods per year.
/// let periods = 12;
///
/// let periodic_rate = convert_apr_to_epr(nominal_rate, periods);
/// 
/// // Confirm that the future value is correct to six decimal places.
/// assert_approx_equal!(0.00283333, periodic_rate);
/// ```
pub fn convert_apr_to_epr(apr: f64, compounding_periods_in_year: u32) -> f64 {
    assert_inputs(apr, compounding_periods_in_year, ConvertRateVariable::Apr);
    apr / compounding_periods_in_year as f64
}
/// Convert APR (annual rate) to periodic rate. Returns a custom solution type.
pub fn convert_apr_to_epr_solution(apr: f64, compounding_periods_in_year: u32) -> ConvertRateSolution {
    assert_inputs(apr, compounding_periods_in_year, ConvertRateVariable::Apr);
    self::apr(apr, compounding_periods_in_year)
}






/// Convert an EAR to APR. Returns f64.
///  
/// Related Functions:
/// * [`convert_ear_to_apr_solution`] to convert EAR to APR and return a custom type with additional functionality and extra information available in the dbg!().
/// 
/// The formula is:
/// 
/// <small> Note: This formula involves converting the EAR to periodic rate first, and then APR = periodic rate * number of periods.</small>
/// 
/// # Arguments
/// * `rate` - The input rate, expressed as a floating point number. 
/// For instance 0.05 would mean 5% growth. Often appears as `r` or `i` in formulas.
/// * `periods` - The number of compounding periods in a year. Often appears as `n` or `t`.
/// 
/// # Panics
/// * `periods` - must be a u32 value greater than 0.
/// 
/// # Examples
/// Convert effective annual rate (EAR) to annual percentage rate (APR).
/// ```
/// use finance::*;
/// // The effective annual rate is 3.4534%
/// let effective_annual_rate = 0.03453486936;
///
/// // There are 12 compounding periods per year.
/// let periods = 12;
///
/// let nominal_rate = convert_rate::convert_ear_to_apr(effective_annual_rate, periods);
/// 
/// // Confirm that the future value is correct to six decimal places.
/// assert_approx_equal!(0.034, nominal_rate);
/// ```
pub fn convert_ear_to_apr(ear: f64, compounding_periods_in_year: u32) -> f64 {
    assert_inputs(ear, compounding_periods_in_year, ConvertRateVariable::Ear);
    ((1_f64 + ear).powf(1_f64/compounding_periods_in_year as f64) - 1_f64) * compounding_periods_in_year as f64
}
/// Convert an EAR to APR. Returns solution struct with additional information and functionality.
///  
/// Related Functions:
/// * [`ear!`] general-purpose macro to convert EAR into all rate variations.
/// * [`convert_ear_to_apr`] to convert EAR to APR and return an f64 value.
/// 
/// The formula is:
/// 
/// <small> Note: This formula involves converting the EAR to periodic rate first, and then APR = periodic rate * number of periods.</small>
/// 
/// # Arguments
/// * `rate` - The input rate, expressed as a floating point number. 
/// For instance 0.05 would mean 5% growth. Often appears as `r` or `i` in formulas.
/// * `periods` - The number of compounding periods in a year. Often appears as `n` or `t`.
/// 
/// # Panics
/// * `periods` - must be a u32 value greater than 0.
/// 
/// # Examples
/// Convert effective annual rate (EAR) to annual percentage rate (APR).
/// ```
/// use finance::*;
/// // The effective annual rate is 3.453486936028982%
/// let effective_annual_rate = 0.03453486936028982;
///
/// // There are 12 compounding periods per year.
/// let periods = 12;
///
/// let nominal_rate = convert_rate::convert_ear_to_apr_solution(effective_annual_rate, periods).apr();
/// 
/// // Confirm that the future value is correct to six decimal places.
/// assert_approx_equal!(0.034, nominal_rate);
/// ```
pub fn convert_ear_to_apr_solution(ear: f64, compounding_periods_in_year: u32) -> ConvertRateSolution {
    assert_inputs(ear, compounding_periods_in_year, ConvertRateVariable::Ear);
    self::ear(ear, compounding_periods_in_year)
}






/// Convert an EAR (Effective Annual Rate) to periodic rate.
/// Convert an EAR (effective annual rate) to a periodic rate (effective periodic rate, EPR). Returns f64.
///  
/// Related Functions:
/// * [`convert_ear_to_epr_solution`] to convert EAR to EPR and return a custom type with extra information available in the dbg!().
/// 
/// The formula is:
/// 
/// 
/// # Arguments
/// * `ear` - The input rate (effective annual rate), expressed as a floating point number. 
/// For instance 0.05 would mean 5% growth. Often appears as `r` or `i` in formulas.
/// * `periods` - The number of compounding periods in a year. Often appears as `n` or `t`.
/// 
/// # Panics
/// * `periods` - must be a u32 value greater than or equal to 1.
/// 
/// # Examples
/// Convert effective annual rate to periodic rate.
/// ```
/// use finance::*;
/// // The effective annual rate is 3.4534%.
/// let effective_annual_rate = 0.03453486936;
///
/// // There are 12 compounding periods per year.
/// let periods = 12;
///
/// let periodic_rate = convert_rate::convert_ear_to_epr(effective_annual_rate, periods);
/// 
/// // Confirm that the future value is correct to six decimal places.
/// assert_approx_equal!(0.00283333, periodic_rate);
/// ```
pub fn convert_ear_to_epr(ear: f64, compounding_periods_in_year: u32) -> f64 {
    assert_inputs(ear, compounding_periods_in_year, ConvertRateVariable::Ear);
    (1_f64 + ear).powf(1_f64/compounding_periods_in_year as f64) - 1_f64
}
/// Convert an EAR (Effective Annual Rate) to periodic rate (also known as EPR).
pub fn convert_ear_to_epr_solution(ear: f64, compounding_periods_in_year: u32) -> ConvertRateSolution {
    assert_inputs(ear, compounding_periods_in_year, ConvertRateVariable::Ear);
    self::ear(ear, compounding_periods_in_year)
}


/// Convert a periodic interest rate (APR / num of compounding periods) to EAR (effective annual rate).
pub fn convert_epr_to_ear(epr: f64, compounding_periods_in_year: u32) -> f64 {
    assert_inputs(epr, compounding_periods_in_year, ConvertRateVariable::Epr);
    (1_f64 + epr).powf(compounding_periods_in_year as f64) - 1_f64
}
/// Convert a periodic interest rate (APR / num of compounding periods) to EAR (effective annual rate).
pub fn convert_epr_to_ear_solution(epr: f64, compounding_periods_in_year: u32) -> ConvertRateSolution {
    assert_inputs(epr, compounding_periods_in_year, ConvertRateVariable::Epr);
    self::epr(epr, compounding_periods_in_year)
}



/// Convert periodic rate to APR (aka Annual rate, nominal interest rate, Annual Percentage Rate). Returns f64.
pub fn convert_epr_to_apr(epr: f64, compounding_periods_in_year: u32) -> f64 {
    assert_inputs(epr, compounding_periods_in_year, ConvertRateVariable::Epr);
    epr * compounding_periods_in_year as f64
}
/// Convert periodic rate to APR (aka Annual rate, nominal interest rate, Annual Percentage Rate). Returns a custom solution type.
pub fn convert_epr_to_apr_solution(epr: f64, compounding_periods_in_year: u32) -> ConvertRateSolution {
    assert_inputs(epr, compounding_periods_in_year, ConvertRateVariable::Epr);
    self::epr(epr, compounding_periods_in_year)
}



#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_convert_rate_apr_symmetry() {
        let apr_rates = vec!(0.034, -0.034, 0.00283333333, -0.00283333333, 0.0345348693603, 0.0, -0.0, 1.0, 2.1, 0.00001);
        let periods = vec![12, 1, 2, 3, 4, 6, 24, 52, 365, 780];
        
        for rates_i in apr_rates {
            for &periods_i in periods.iter() {
                check_rate_conversion_symmetry(rates_i, periods_i);

                fn check_rate_conversion_symmetry(rate:f64, periods:u32) {  
                    // apr scenarios
                    let apr_epr = convert_apr_to_epr(rate, periods);
                    let _epr_apr = convert_epr_to_apr(apr_epr, periods);
                    let apr_ = apr(rate, periods);
                    assert_approx_equal!(apr_epr, apr_.epr());
                    assert_approx_equal!(_epr_apr, rate);

                    let apr_ear = convert_apr_to_ear(rate, periods);
                    let _ear_apr = convert_ear_to_apr(apr_ear, periods);
                    assert_approx_equal!(apr_ear, apr_.ear());
                    assert_approx_equal!(_ear_apr, rate);
                }
                
            }
        }
    }

    #[test]
    fn test_convert_rate_ear_symmetry() {
        let ear_rates = vec!(0.034, -0.034, 0.00283333333, -0.00283333333, 0.0345348693603, 0.0, -0.0, 1.0, 2.1, 0.00001);
        let periods = vec![12, 1, 2, 3, 4, 6, 24, 52, 365, 780];
        
        for rates_i in ear_rates {
            for &periods_i in periods.iter() {
                check_rate_conversion_symmetry(rates_i, periods_i);

                fn check_rate_conversion_symmetry(rate:f64, periods:u32) {
                    // ear scenarios
                    let ear_apr = convert_ear_to_apr(rate, periods);
                    let _apr_ear = convert_apr_to_ear(ear_apr, periods);
                    let ear_ = ear(rate, periods);
                    assert_approx_equal!(ear_apr, ear_.apr());
                    assert_approx_equal!(_apr_ear, rate);

                    let ear_epr = convert_ear_to_epr(rate, periods);
                    let _epr_ear = convert_epr_to_ear(ear_epr, periods);
                    assert_approx_equal!(ear_epr, ear_.epr());
                    assert_approx_equal!(_epr_ear, rate);  
                }
                
            }
        }
    }

    #[test]
    fn test_convert_rate_epr_symmetry() {
        let epr_rates = vec!(0.034, -0.034, 0.00283333333, -0.00283333333, 0.0345348693603, 0.0, -0.039, -0.0, 0.98, 0.00001, 0.98999);
        let periods = vec![12, 1, 2, 3, 4, 6, 24, 52, 365, 780];
        // note: epr_rate of 0.99 causes floating point representation error on big periods. Periods over 780 also cause failed tests.
        
        for rates_i in epr_rates {
            for &periods_i in periods.iter() {
                check_rate_conversion_symmetry(rates_i, periods_i);

                fn check_rate_conversion_symmetry(rate:f64, periods:u32) {
                    // epr scenarios
                    let epr_apr = convert_epr_to_apr(rate, periods);
                    let _apr_epr = convert_apr_to_epr(epr_apr, periods);
                    let epr_ = epr(rate, periods);
                    assert_approx_equal!(epr_apr, epr_.apr());
                    assert_approx_equal!(_apr_epr, rate);

                    let epr_ear = convert_epr_to_ear(rate, periods);
                    let _ear_epr = convert_ear_to_epr(epr_ear, periods);
                    assert_approx_equal!(epr_ear, epr_.ear());
                    assert_approx_equal!(_ear_epr, rate);  
                }
                
            }
        }
    }



    #[test]
    fn test_convert_rates_simple_1() {
        // test on excel values using 12 periods
        const PERIODS: u32 = 12;
        let apr_epr_ear_rates = vec!((0.034, 0.00283333333, 0.0345348693603),
                                     (-0.034, -0.002833333333, -0.03347513889),
                                     (1.0, 0.08333333333, 1.6130352902247),
                                     (-1.0, -0.083333333333, -0.64800437199),
                                     (2.1, 0.175, 5.9255520766347), 
                                     (-2.1, -0.175,-0.90058603794)
                                );
        for rate_tupe in apr_epr_ear_rates {
                let ap = convert_apr_to_epr(rate_tupe.0, PERIODS);
                let ae = convert_apr_to_ear(rate_tupe.0, PERIODS);
                let pa = convert_epr_to_apr(rate_tupe.1, PERIODS);
                let pe = convert_epr_to_ear(rate_tupe.1, PERIODS);
                let ea = convert_ear_to_apr(rate_tupe.2, PERIODS);
                let ep = convert_ear_to_epr(rate_tupe.2, PERIODS);
                check_rate_conversion_symmetry(ap, ae, pa, pe, ea, ep);

                fn check_rate_conversion_symmetry(ap:f64, ae:f64, pa:f64, pe:f64, ea:f64, ep:f64) {
                    assert_eq!( round_6(ap), round_6(ep) );
                    assert_eq!( round_6(ae), round_6(pe) );
                    assert_eq!( round_6(pa), round_6(ea) );
                }
        }
    }
}