//! Number of periods required to achieve a Future Value with a given set 
//! of annuities (payments) at a specified periodic rate.

use log::{warn};
use libm::log10;

// To do before final: Excel says some calculations cannot be done, like pmt=-5 pv=1000 fv=2000  ...figure out how to replicate this logic 
// should we separate the function into pv and fv? (i think so...)

// To do in next release: add nper_due functions

/// Returns the number of periods required for an annuity (payments) to reach a future value, at a specified periodic rate.
///
/// Related functions:
/// * To calculate nper while retaining the input values use
/// [`nper_solution`].
/// 
/// The NPER formula is:
///
/// Number of Periods = 
/// LN( (payment - future_value * periodic_rate) / (payment + present_value * periodic_rate) ) 
/// / LN(1 + periodic_rate)
/// 
/// ...where LN is the natural log (log10).
/// 
/// # Arguments
/// * `periodic_rate` - The rate at which the investment grows or shrinks per period, expressed as a
/// floating point number. For instance 0.05 would mean 5% growth. Often appears as `r` or `i` in
/// formulas.
/// * `payment` - The payment amount per period, also referred to as an annuity or cashflow. In this formula, it must be negative, and future value must be positive. Often payment appears as `pmt` or `C` (cashflow) in formulas.
/// * `present_value` - The present value, or total value of all payments now. Often appears as `pv` in formulas.
/// * `future_value` - The future value, a cash balance after the last payment is made and interest has accrued in each period. Often appears as `fv` in formulas.
///
/// # Panics
/// The call will fail if `payment` is greater than or equal to 0, because the formula requires `payment` to be negative. 
/// Additionally, `present_value` and `future_value` must be positive, or 0. However, both `present_value` and `future_value` cannot be 0, at least one value must be set.
///

/// Returns f64 for Number of Periods (NPER). 
/// Receive the number of periods required for a present value to equal a future value based on a set of payments at an interest rate. 
/// If there is no initial present value, use 0. 
/// This is equivalent to the NPER function in Excel / Google Sheets.
pub fn nper<C: Into<f64> + Copy, P: Into<f64> + Copy, F: Into<f64> + Copy>(periodic_rate: f64, payment: C, present_value: P, future_value: F) -> f64 {
    nper_solution(periodic_rate, payment, present_value, future_value).periods
}

/// Returns f64 for Number of Periods (NPER). 
/// Receive the number of periods required for a present value to equal a future value based on a set of payments (annuity) at an interest rate. 
/// If there is no initial present value, use 0. 
/// This is equivalent to the NPER function in Excel / Google Sheets.
pub fn nper_solution<C: Into<f64> + Copy, P: Into<f64> + Copy, F: Into<f64> + Copy>(periodic_rate: f64, payment: C, present_value: P, future_value: F) -> NperSolution {
    let pmt = payment.into();
    let pv = present_value.into();
    let fv = future_value.into();
    assert!(pv >= 0.0);
    assert!(fv >= 0.0);
    assert!(pv + fv > 0.0, "Either present_value, and/or future_value, must be greater than 0.");
    assert!(pmt < 0_f64, "The payment amount must be negative, same as Excel / Google Sheets."); // payment must be negative, same as Excel.
    assert!(periodic_rate.is_finite(), "Rate must be finite.");
    assert!(pmt.is_finite(), "Payment amount must be finite.");
    assert!(pv.is_finite(), "Present Value amount must be finite.");
    assert!(fv.is_finite(), "Future Value amount must be finite.");
    
    
    
    // LN((pmt - fv*r_)/(pmt + pv*r_))/LN(1 + r_)
    let numerator = libm::log10( (pmt - fv * periodic_rate) / (pmt +  pv * periodic_rate) );
    let num_periods = numerator / libm::log10(1. + periodic_rate); 
    NperSolution::new(periodic_rate, num_periods, pmt, pv, fv)
}

#[derive(Debug)]
pub struct NperSolution {
    pub periodic_rate: f64,
    pub periods: f64,
    pub payment: f64,
    pub present_value_total: f64,
    pub future_value_total: f64,
    pub formula: String,
    
}
impl NperSolution {
    pub fn new(periodic_rate: f64, periods: f64, payment: f64, present_value_total: f64, future_value_total: f64) -> Self {
        let formula = format!("LOG10(({} - {}*{})/({} + {}*{})) / LOG10(1 + {})", payment, future_value_total, periodic_rate, payment, present_value_total, periodic_rate, periodic_rate);
        Self {
            periodic_rate,
            periods,
            payment,
            present_value_total,
            future_value_total,
            formula,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_nper_1() {
        // normal cases
        assert_eq!(round_to_ulps6(27.7879559), round_to_ulps6(nper(0.034, -500, 1000, 20_000)));
        assert_eq!(round_to_ulps6(59.76100743), round_to_ulps6(nper(0.034, -50, 1000, 2_000)));
        assert_eq!(round_to_ulps6(25.68169193), round_to_ulps6(nper(0.034, -50, 0, 2_000)));
        assert_eq!(round_to_ulps6(80.18661533), round_to_ulps6(nper(0.034, -5, 0, 2_000)));
        assert_eq!(round_to_ulps6(106.3368288), round_to_ulps6(nper(0.034, -200, 0, 200_000)));
    }

    #[should_panic]
    #[test]
    fn test_nper_2() {
        // infinite cases
        nper(1_f64/0_f64, -500, 1000, 20_000);
        nper(0.034, -1_f64/0_f64, 1000, 20_000);
        nper(0.034, -500, 1_f64/0_f64, 20_000);
        nper(0.034, -500, 0, 1_f64/0_f64);
    }

    #[should_panic]
    #[test]
    fn test_nper_3() {
        // positive pmt
        nper(1_f64/0_f64, 500, 1000, 20_000);
    }

    #[should_panic]
    #[test]
    fn test_nper_3() {
        // positive pmt
        nper(1_f64/0_f64, 0, 1000, 20_000);
    }

    #[should_panic]
    #[test]
    fn test_nper_3() {
        // negative 0
        nper(1_f64/0_f64, -0, 1000, 20_000);
    }

}