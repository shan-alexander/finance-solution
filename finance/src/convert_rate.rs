//! **Rate conversions**. Given a rate and number of compound periods per year, what is this rate
//! when converted to/from:
//!
//! **APR**: **Annual Percentage Rate**, also written as Nominal Rate, or annual discount rate. An annualized represenation of the interest rate.
//!
//! ><small>For general use, try the `apr!` macro by providing rate and compounds_per_year, for example `apr!(0.034, 12)`.</small>
//!
//! ><small>To _calculate_ the Annual Percentage Rate (APR) of a given rate, use the [`convert_ear_to_apr`] or [`convert_epr_to_apr`] functions.</small>
//! 
//! ><small>To _convert_ an Annual Percentage Rate (APR) into a different rate, use the [`convert_apr_to_ear`] or [`convert_apr_to_epr`] functions.</small>
//! 
//! **EPR**: **Effective Periodic Rate**, also written as **Periodic Rate**. The rate of the compounding period.
//! 
//! <small>For general use, try the `epr!` macro by providing rate and compounds_per_year, for example `epr!(0.034, 12)`.</small>
//!
//! ><small>To <i>calculate</i> the Effective Periodic Rate (EPR) of a given rate use the [`convert_apr_to_epr`] or [`convert_ear_to_epr`] functions.</small>
//! 
//! ><small>To convert an Effective Period Rate (EPR) into a different rate, use the [`convert_epr_to_ear`] or [`convert_epr_to_apr`] functions.</small>
//! 
//! **EAR**: **Effective Annual Rate**. The effective rate of a year which (typically) has multiple compound periods.
//! 
//! <small>For general use, try the `ear!` macro by providing rate and compounds_per_year, for example `ear!(0.034, 12)`.</small>
//!
//! ><small>To calculate the Effective Annual Rate (EAR) of a given rate use the [`convert_apr_to_ear`] or [`convert_epr_to_ear`] functions.</small>
//!
//! ><small>To convert an Effective Annual Rate (EAR) into a different rate, use the [`convert_ear_to_apr`] or [`convert_ear_to_epr`] functions.</small>
//! 
//! # Examples
//! All functions in this module can be written with the suffix **_solution** which is recommended for most users.
//! The solution functions provide helpful information in the dbg!() output, for example:
//! 
//! ```rust
//! // example 1: easy method
//! let rate = apr(0.034, 12)
//! dbg!(rate);
//! 
//! // prints to terminal: 
//! ```
//! >apr here
//! 
//! ```rust
//! // example 2: explicit call to f64 function
//! let apr = convert_apr_to_ear(0.034, 12);
//! dbg!(apr);
//! // prints to terminal: 
//! ```
//! >0.03453486936028982
//!
//! ```rust
//! // example 3: explicit call to _solution function
//! dbg!(convert_apr_to_ear_solution(0.034, 12));
//! // prints to terminal: 
//! ```
//!  >{<br>
//!  >calculated_field: <span style="color:Magenta">AprToEar</span>,<br>
//!  >input_rate: <span style="color:Orange">0.034,</span><br>
//!  >num_periods_in_year: <span style="color:Orange">12,</span><br>
//!  >output_rate: <span style="color:Green">0.03453486936028982</span><br>
//!  >input_in_percent: 3.4000%<br>
//!  >output_in_percent: <span style="color:Green">3.4535%</span><br>
//!  >formula: "(1 + (0.034/12))^12 - 1"<br>
//!  >apr: 0.034<br>
//!  >epr: 0.0028333333333333335<br>
//!  ><span style="color:Green">ear</span>: 0.03453486936028982<br>
//!  >}<br>
//! 
//! Here are a few variations of how someone can use the `convert_rate` module functions:
//! ```rust
//! use finance::convert_rate::*;
//! 
//! // What is the future value of $500 in 1 year 
//! // if the APR is 3.4% and it's compounded monthly?
//! // Solve twice, first using EPR and then using EAR.
//! 
//! // to solve, first convert the annual rate into a periodic rate (monthly):
//! let epr = convert_rate::convert_apr_to_epr(0.034, 12);
//! dbg!(&epr);
//! // which prints: 
//! ```
//! >&epr = 0.0028333333333333335
//! ```rust
//! // then solve for future value:
//! let fv = finance::future_value::future_value_solution;
//! let answer_1 = fv(epr, 12, 500);
//! dbg!(answer_1);
//! // which prints:
//! ```
//! >answer = TvmSolution {<br>
//! >    calculated_field: <span style="color:Magenta">FutureValue</span>,<br>
//! >    rate: <span style="color:Orange">0.0028333333333333335</span>,<br>
//! >    periods: <span style="color:Orange">12</span>,<br>
//! >    present_value: <span style="color:Orange">500.0</span>,<br>
//! >    future_value: <span style="color:Green">517.2674346801452</span>,<br>
//! >    formula: "500.0000 * (1.002833 ^ 12)",<br>
//! >}
//! 
//! ```rust
//! // then let's double-check the previous answer_1 by solving the future_value
//! // using 1 year as the period and the effective annual rate, 
//! // instead of using 12 monthly periods of the periodic rate.
//! let rate = finance::convert_rate::apr(0.034, 12);
//! let answer_2 = fv(rate.ear, 1, 500);
//! dbg!(&answer_2.future_value);
//! assert_approx_equal!(answer_1.future_value, answer_2.future_value); // true
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

pub fn assert_inputs(rate:f64, periods:u32, fn_type: ConvertRateVariable) {
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


/// Convert a nominal interest rate (Annual rate, APR) to EAR (effective annual rate). Returns f64.
/// 
/// Related Functions:
/// * [`convert_apr_to_ear_solution`] to convert APR to EAR and return a custom type with additional functionality and extra information available in the dbg!().
/// 
/// The formula is:
/// 
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
/// /// Convert annual rate to effective annual rate.
/// ```
/// // The annual percentage rate is 3.4%.
/// let nominal_rate = 0.034;
///
/// // There are 12 compounding periods per year.
/// let periods = 12;
///
/// let effective_annual_rate = finance::convert_apr_to_ear(rate, periods);
/// 
/// // Confirm that the future value is correct to six decimal places.
/// finance::assert_rounded_6(0.034535, effective_annual_rate);
/// ```
pub fn convert_apr_to_ear(apr: f64, compounding_periods_in_year: u32) -> f64 {
    assert_inputs(apr, compounding_periods_in_year, ConvertRateVariable::Apr);
    (1_f64 + (apr/compounding_periods_in_year as f64)).powf(compounding_periods_in_year as f64) - 1_f64
}

// /// Convert a nominal interest rate (Annual rate, APR) to EAR (effective annual rate). Returns a custom type with additional functionality and extra information available in the dbg!().
// pub fn convert_apr_to_ear_solution(apr: f64, compounding_periods_in_year: u32) -> ConvertRateSolution {
//     assert_inputs(apr, compounding_periods_in_year, ConvertRateVariable::AprToEar);
//     let ear = (1_f64 + (apr/compounding_periods_in_year as f64)).powf(compounding_periods_in_year as f64) - 1_f64;
//     let input_in_percent = format!("{:.4}%", apr * 100.);
//     let output_in_percent = format!("{:.4}%", ear * 100.);
//     let formula = format!("(1 + ({}/{}))^{} - 1", apr, compounding_periods_in_year, compounding_periods_in_year);
//     let epr= convert_apr_to_epr(apr, compounding_periods_in_year);
//     ConvertRateSolution::new(ConvertRateVariable::AprToEar, apr, compounding_periods_in_year, ear, input_in_percent, output_in_percent, &formula, apr, epr, ear)
// }







/// Convert APR (annual rate) to periodic rate. Returns f64.
/// /// 
/// Related Functions:
/// * [`convert_apr_to_epr_solution`] to convert APR to EPR and return a custom type with additional functionality and extra information available in the dbg!().
/// 
/// The formula is:
/// 
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
/// /// Convert annual rate to periodic rate.
/// ```
/// // The annual percentage rate is 3.4%.
/// let nominal_rate = 0.034;
///
/// // There are 12 compounding periods per year.
/// let periods = 12;
///
/// let periodic_rate = finance::convert_apr_to_epr(rate, periods);
/// 
/// // Confirm that the future value is correct to six decimal places.
/// finance::assert_rounded_6(0.002833, periodic_rate);
/// ```
pub fn convert_apr_to_epr(apr: f64, compounding_periods_in_year: u32) -> f64 {
    assert_inputs(apr, compounding_periods_in_year, ConvertRateVariable::Apr);
    apr / compounding_periods_in_year as f64
}
// /// Convert APR (annual rate) to periodic rate. Returns a custom solution type.
// pub fn convert_apr_to_epr_solution(apr: f64, compounding_periods_in_year: u32) -> ConvertRateSolution {
//     assert_inputs(apr, compounding_periods_in_year, ConvertRateVariable::AprToEpr);
//     let epr = apr / compounding_periods_in_year as f64;
//     let input_in_percent = format!("{}%", (apr * 100_000.0).round() / 100_000.0 * 100.);
//     let output_in_percent = format!("{:.4}%", (epr * 100.));
//     let formula = format!("{} / {}", apr, compounding_periods_in_year);
//     let ear = convert_apr_to_epr(apr, compounding_periods_in_year);
//     ConvertRateSolution::new(ConvertRateVariable::AprToEpr, apr, compounding_periods_in_year, epr, input_in_percent, output_in_percent, &formula, apr, epr, ear)
// }






/// Convert an EAR (effective annual rate) to APR (a nominal interest rate, annual rate). This involves converting the EAR to periodic rate first, and then APR = periodic rate * number of periods. Returns f64.
///  
/// Related Functions:
/// * [`convert_ear_to_apr_solution`] to convert EAR to APR and return a custom type with additional functionality and extra information available in the dbg!().
/// 
/// The formula is:
/// 
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
/// /// Convert annual rate to periodic rate.
/// ```
/// // The annual percentage rate is 3.4%.
/// let effective_annual_rate = 0.034;
///
/// // There are 12 compounding periods per year.
/// let periods = 12;
///
/// let nominal_rate = finance::convert_ear_to_apr(effective_annual_rate, periods);
/// 
/// // Confirm that the future value is correct to six decimal places.
/// finance::assert_rounded_6(0.002833, nominal_rate);
/// ```
pub fn convert_ear_to_apr(ear: f64, compounding_periods_in_year: u32) -> f64 {
    assert_inputs(ear, compounding_periods_in_year, ConvertRateVariable::Ear);
    ((1_f64 + ear).powf(1_f64/compounding_periods_in_year as f64) - 1_f64) * compounding_periods_in_year as f64
}
// // APR is the periodic rate * number of periods, so convert EAR to periodic, then * n
// // convert_ear_to_periodic_f64(ear, n) * n
// /// Convert an EAR (effective annual rate) to a nominal interest rate (Annual rate, APR). This involves converting the EAR to periodic rate first, and then APR = periodic rate * number of periods. Returns f64.
// pub fn convert_ear_to_apr_solution(ear: f64, compounding_periods_in_year: u32) -> ConvertRateSolution {
//     assert_inputs(ear, compounding_periods_in_year, ConvertRateVariable::EarToApr);
//     let epr= (1_f64 + ear).powf(1_f64/compounding_periods_in_year as f64) - 1_f64;
//     let apr = epr* compounding_periods_in_year as f64;
//     let input_in_percent = format!("{}%", (ear * 100_000.0).round() / 100_000.0 * 100.);
//     let output_in_percent = format!("{:.4}%", (apr * 100.));
//     let formula = format!("((1 + {})^(1/{}) - 1) * {}", ear, compounding_periods_in_year, compounding_periods_in_year);
//     ConvertRateSolution::new(ConvertRateVariable::EarToApr, ear, compounding_periods_in_year, apr, input_in_percent, output_in_percent, &formula, apr, epr, ear)
// }






/// Convert an EAR (Effective Annual Rate) to periodic rate.
/// /// Convert an EAR (effective annual rate) to a periodic rate (effective periodic rate, EPR). This involves converting the EAR to periodic rate first, and then APR = periodic rate * number of periods. Returns f64.
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
/// /// Convert effective annual rate to periodic rate.
/// ```
/// // The annual percentage rate is 3.4%.
/// let effective_annual_rate = 0.034;
///
/// // There are 12 compounding periods per year.
/// let periods = 12;
///
/// let periodic_rate = finance::convert_ear_to_epr(effective_annual_rate, periods);
/// 
/// // Confirm that the future value is correct to six decimal places.
/// finance::assert_rounded_6(0.002833, periodic_rate);
/// ```
pub fn convert_ear_to_epr(ear: f64, compounding_periods_in_year: u32) -> f64 {
    assert_inputs(ear, compounding_periods_in_year, ConvertRateVariable::Ear);
    (1_f64 + ear).powf(1_f64/compounding_periods_in_year as f64) - 1_f64
}
// /// Convert an EAR (Effective Annual Rate) to periodic rate (also known as EPR).
// pub fn convert_ear_to_epr_solution(ear: f64, compounding_periods_in_year: u32) -> ConvertRateSolution {
//     assert_inputs(ear, compounding_periods_in_year, ConvertRateVariable::EarToEpr);
//     // EPR = (1 + annual rate)^(1/#ofPeriodsPerYear) 
//     let epr = (1_f64 + ear).powf(1_f64/compounding_periods_in_year as f64) - 1_f64;
//     let input_string = (ear*100.).to_string();
//     let output_string = (epr*100.).to_string();
//     let formula = format!("(1 + {})^(1 / {}) - 1", ear, compounding_periods_in_year);
//     let apr = epr * compounding_periods_in_year as f64;
//     ConvertRateSolution::new(ConvertRateVariable::EarToEpr, ear, compounding_periods_in_year, epr, input_string, output_string, &formula, apr, epr, ear)
// }


/// Convert a periodic interest rate (APR / num of compounding periods) to EAR (effective annual rate).
pub fn convert_epr_to_ear(epr: f64, compounding_periods_in_year: u32) -> f64 {
    assert_inputs(epr, compounding_periods_in_year, ConvertRateVariable::Epr);
    (1_f64 + epr).powf(compounding_periods_in_year as f64) - 1_f64
}
// /// Convert a periodic interest rate (APR / num of compounding periods) to EAR (effective annual rate).
// pub fn convert_epr_to_ear_solution(epr: f64, compounding_periods_in_year: u32) -> ConvertRateSolution {
//     assert_inputs(epr, compounding_periods_in_year, ConvertRateVariable::EprToEar);
//     let ear = (1_f64 + epr).powf(compounding_periods_in_year as f64) - 1_f64;
//     let input_string = format!("{:.4}%", epr*100.);
//     let output_string = format!("{:.4}%", ear*100.);
//     let formula = format!("(1 + {})^{} - 1", epr, compounding_periods_in_year);
//     let apr = epr * compounding_periods_in_year as f64;
//     ConvertRateSolution::new(ConvertRateVariable::EprToEar, epr, compounding_periods_in_year, ear, input_string, output_string, &formula, apr, epr, ear)
// }



/// Convert periodic rate to APR (aka Annual rate, nominal interest rate, Annual Percentage Rate). Returns f64.
pub fn convert_epr_to_apr(epr: f64, compounding_periods_in_year: u32) -> f64 {
    assert_inputs(epr, compounding_periods_in_year, ConvertRateVariable::Epr);
    epr * compounding_periods_in_year as f64
}
// /// Convert periodic rate to APR (aka Annual rate, nominal interest rate, Annual Percentage Rate). Returns a custom solution type.
// pub fn convert_epr_to_apr_solution(epr: f64, compounding_periods_in_year: u32) -> ConvertRateSolution {
//     assert_inputs(epr, compounding_periods_in_year, ConvertRateVariable::EprToApr);
//     let apr = epr * compounding_periods_in_year as f64;
//     let input_in_percent = format!("{:.4}%", epr * 100.);
//     let output_in_percent = format!("{:.4}%", apr * 100.);
//     let formula = format!("{} * {}", epr, compounding_periods_in_year);
//     let ear = convert_apr_to_ear(apr, compounding_periods_in_year);
//     ConvertRateSolution::new(ConvertRateVariable::EprToApr, epr, compounding_periods_in_year, apr, input_in_percent, output_in_percent, &formula, apr, epr, ear)
// }


// fn round_6(val:f64) -> f64 { (val * 1_000_000.).round() / 1_000_000.}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    
    #[test]
    fn test_convert_rate_solution_symmetry() {
        let apr_epr_ear_rates = vec!(0.034, -0.034, 0.00283333333, -0.00283333333, 0.0345348693603, 0.0, -0.0, 1.0, 2.1, 0.00001);
        let periods = vec![12, 1, 2, 3, 4, 6, 24, 52, 365, 780];
        
        for rates_i in apr_epr_ear_rates {
            for &periods_i in periods.iter() {
                check_rate_conversion_symmetry(rates_i, periods_i);

                fn check_rate_conversion_symmetry(rate:f64, periods:u32) {
                    // apr scenarios
                    let apr_epr = convert_apr_to_epr(rate, periods);
                    let _epr_apr = convert_epr_to_apr(apr_epr, periods);

                    // ear scenarios


                    // let apr_epr = convert_apr_to_epr_solution(rate, periods);
                    // let ae = convert_apr_to_ear_solution(rate, periods);

                    // assert_approx_equal!(apr_epr.output_rate, convert_ear_to_epr(ae.output_rate, periods));
                    // assert_approx_equal!(ae.output_rate, convert_epr_to_ear(apr_epr.output_rate, periods));
                    

                    // let pa = convert_epr_to_apr_solution(rate, periods);
                    // let pe = convert_epr_to_ear_solution(rate, periods);
                    
                    // let ea = convert_ear_to_apr_solution(rate, periods);
                    // let ep = convert_ear_to_epr_solution(rate, periods);

                    
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
                                    //  (1.0, 0.08333333333, 1.6130352902247),
                                    //  (-1.0, -0.083333333333, -0.64800437199),
                                    //  (2.1, 0.175, 5.9255520766347), 
                                    //  (-2.1, -0.175,-0.90058603794)
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