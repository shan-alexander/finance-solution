use log::{warn};

// Import needed for the function references in the Rustdoc comments.
#[allow(unused_imports)]
use crate::*;
use crate::verbose::convert_rate_solution::*;

pub fn main() { 
    dbg!(convert_apr_to_ear_solution(0.034,12));
    dbg!(convert_ear_to_apr_solution(0.03453486936028982,12));
    dbg!(convert_ear_to_epr_solution(0.03453486936028982,12));
    dbg!(convert_epr_to_ear_solution(0.0028333333333332433,12));
    // self::tests::test_convert_rates_1();

}

fn assert_inputs(rate:f64, periods:u32, fn_type: ConvertRateVariable) {
    assert!(periods >= 1);
    assert!(rate.is_finite());
    if fn_type.is_ear_to_apr() { 
        assert!(rate > -1.0, "The Effective Annual Rate (EAR) must be greater than -100% or the exponential formula will create an imaginary number.");
    }
    if rate > 1. || rate < -1. {
        warn!("You provided an rate of {}%. Are you sure?", rate*100.);
    }
    if periods > 366 {
        warn!("You provided more than 366 compounding periods in a year (You provided {}). Are you sure?", periods);
    }
}

/// Convert a nominal interest rate (Annual rate, APR) to EAR (effective annual rate). Returns a custom solution type.
pub fn convert_apr_to_ear_solution(apr: f64, compounding_periods_in_year: u32) -> ConvertRateSolution {
    assert_inputs(apr, compounding_periods_in_year, ConvertRateVariable::AprToEar);
    let ear = (1_f64 + (apr/compounding_periods_in_year as f64)).powf(compounding_periods_in_year as f64) - 1_f64;
    let input_in_percent = format!("{:.4}%", apr * 100.);
    let output_in_percent = format!("{:.4}%", ear * 100.);
    let formula = format!("(1 + ({}/{}))^{} - 1", apr, compounding_periods_in_year, compounding_periods_in_year);
    let epr= convert_apr_to_epr(apr, compounding_periods_in_year);
    ConvertRateSolution::new(ConvertRateVariable::AprToEar, apr, compounding_periods_in_year, ear, input_in_percent, output_in_percent, &formula, apr, epr, ear)
}
/// Convert a nominal interest rate (Annual rate, APR) to EAR (effective annual rate). Returns f64.
pub fn convert_apr_to_ear(apr: f64, compounding_periods_in_year: u32) -> f64 {
    assert_inputs(apr, compounding_periods_in_year, ConvertRateVariable::AprToEar);
    (1_f64 + (apr/compounding_periods_in_year as f64)).powf(compounding_periods_in_year as f64) - 1_f64
}

/// Convert APR (annual rate) to periodic rate. Returns a custom solution type.
pub fn convert_apr_to_epr_solution(apr: f64, compounding_periods_in_year: u32) -> ConvertRateSolution {
    assert_inputs(apr, compounding_periods_in_year, ConvertRateVariable::AprToEpr);
    let epr = apr / compounding_periods_in_year as f64;
    let input_in_percent = format!("{}%", (apr * 100_000.0).round() / 100_000.0 * 100.);
    let output_in_percent = format!("{:.4}%", (epr * 100.));
    let formula = format!("{} / {}", apr, compounding_periods_in_year);
    let ear = convert_apr_to_ear(apr, compounding_periods_in_year);
    ConvertRateSolution::new(ConvertRateVariable::AprToEpr, apr, compounding_periods_in_year, epr, input_in_percent, output_in_percent, &formula, apr, epr, ear)
}
/// Convert APR (annual rate) to periodic rate. Returns f64.
pub fn convert_apr_to_epr(apr: f64, compounding_periods_in_year: u32) -> f64 {
    assert_inputs(apr, compounding_periods_in_year, ConvertRateVariable::AprToEpr);
    apr / compounding_periods_in_year as f64
}

// APR is the periodic rate * number of periods, so convert EAR to periodic, then * n
// convert_ear_to_periodic_f64(ear, n) * n
/// Convert an EAR (effective annual rate) to a nominal interest rate (Annual rate, APR). This involves converting the EAR to periodic rate first, and then APR = periodic rate * number of periods. Returns f64.
pub fn convert_ear_to_apr_solution(ear: f64, compounding_periods_in_year: u32) -> ConvertRateSolution {
    assert_inputs(ear, compounding_periods_in_year, ConvertRateVariable::EarToApr);
    let epr= (1_f64 + ear).powf(1_f64/compounding_periods_in_year as f64) - 1_f64;
    let apr = epr* compounding_periods_in_year as f64;
    let input_in_percent = format!("{}%", (ear * 100_000.0).round() / 100_000.0 * 100.);
    let output_in_percent = format!("{:.4}%", (apr * 100.));
    let formula = format!("((1 + {})^(1/{}) - 1) * {}", ear, compounding_periods_in_year, compounding_periods_in_year);
    ConvertRateSolution::new(ConvertRateVariable::EarToApr, ear, compounding_periods_in_year, apr, input_in_percent, output_in_percent, &formula, apr, epr, ear)
}
/// Convert an EAR (effective annual rate) to a nominal interest rate (Annual rate, APR). This involves converting the EAR to periodic rate first, and then APR = periodic rate * number of periods. Returns f64.
pub fn convert_ear_to_apr(ear: f64, compounding_periods_in_year: u32) -> f64 {
    assert_inputs(ear, compounding_periods_in_year, ConvertRateVariable::EarToApr);
    ((1_f64 + ear).powf(1_f64/compounding_periods_in_year as f64) - 1_f64) * compounding_periods_in_year as f64
}

/// Convert an EAR (Effective Annual Rate) to periodic rate (also known as EPR).
pub fn convert_ear_to_epr_solution(ear: f64, compounding_periods_in_year: u32) -> ConvertRateSolution {
    assert_inputs(ear, compounding_periods_in_year, ConvertRateVariable::EarToEpr);
    // EPR = (1 + annual rate)^(1/#ofPeriodsPerYear) 
    let epr = (1_f64 + ear).powf(1_f64/compounding_periods_in_year as f64) - 1_f64;
    let input_string = (ear*100.).to_string();
    let output_string = (epr*100.).to_string();
    let formula = format!("(1 + {})^(1 / {}) - 1", ear, compounding_periods_in_year);
    let apr = epr * compounding_periods_in_year as f64;
    ConvertRateSolution::new(ConvertRateVariable::EarToEpr, ear, compounding_periods_in_year, epr, input_string, output_string, &formula, apr, epr, ear)
}
/// Convert an EAR (Effective Annual Rate) to periodic rate.
pub fn convert_ear_to_epr(ear: f64, compounding_periods_in_year: u32) -> f64 {
    assert_inputs(ear, compounding_periods_in_year, ConvertRateVariable::EarToEpr);
    (1_f64 + ear).powf(1_f64/compounding_periods_in_year as f64) - 1_f64
}

/// Convert a periodic interest rate (APR / num of compounding periods) to EAR (effective annual rate).
pub fn convert_epr_to_ear_solution(epr: f64, compounding_periods_in_year: u32) -> ConvertRateSolution {
    assert_inputs(epr, compounding_periods_in_year, ConvertRateVariable::EprToEar);
    let ear = (1_f64 + epr).powf(compounding_periods_in_year as f64) - 1_f64;
    let input_string = format!("{:.4}%", epr*100.);
    let output_string = format!("{:.4}%", ear*100.);
    let formula = format!("(1 + {})^{} - 1", epr, compounding_periods_in_year);
    let apr = epr * compounding_periods_in_year as f64;
    ConvertRateSolution::new(ConvertRateVariable::EprToEar, epr, compounding_periods_in_year, ear, input_string, output_string, &formula, apr, epr, ear)
}
/// Convert a periodic interest rate (APR / num of compounding periods) to EAR (effective annual rate).
pub fn convert_epr_to_ear(epr: f64, compounding_periods_in_year: u32) -> f64 {
    assert_inputs(epr, compounding_periods_in_year, ConvertRateVariable::EprToEar);
    (1_f64 + epr).powf(compounding_periods_in_year as f64) - 1_f64
}

/// Convert periodic rate to APR (aka Annual rate, nominal interest rate, Annual Percentage Rate). Returns a custom solution type.
pub fn convert_epr_to_apr_solution(epr: f64, compounding_periods_in_year: u32) -> ConvertRateSolution {
    assert_inputs(epr, compounding_periods_in_year, ConvertRateVariable::EprToApr);
    let apr = epr * compounding_periods_in_year as f64;
    let input_in_percent = format!("{:.4}%", epr * 100.);
    let output_in_percent = format!("{:.4}%", apr * 100.);
    let formula = format!("{} * {}", epr, compounding_periods_in_year);
    let ear = convert_apr_to_ear(apr, compounding_periods_in_year);
    ConvertRateSolution::new(ConvertRateVariable::EprToApr, epr, compounding_periods_in_year, apr, input_in_percent, output_in_percent, &formula, apr, epr, ear)
}
/// Convert periodic rate to APR (aka Annual rate, nominal interest rate, Annual Percentage Rate). Returns f64.
pub fn convert_epr_to_apr(epr: f64, compounding_periods_in_year: u32) -> f64 {
    assert_inputs(epr, compounding_periods_in_year, ConvertRateVariable::EprToApr);
    epr * compounding_periods_in_year as f64
}


fn round_2(val:f64) -> f64 { (val * 100.).round() / 100.}
fn round_4(val:f64) -> f64 { (val * 10_000.).round() / 10_000.}
fn round_6(val:f64) -> f64 { (val * 1_000_000.).round() / 1_000_000.}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    
    #[test]
    fn test_convert_rate_solution_symmetry() {
        let apr_epr_ear_rates = vec!(0.034, 0.00283333333,0.0345348693603,1.0, 2.1, 0.00001);
        let periods = vec![12, 1, 2, 3, 4, 6, 24, 52, 365, 800];
        
        for rates_i in apr_epr_ear_rates {
            for &periods_i in periods.iter() {
                check_rate_conversion_symmetry(rates_i,periods_i);

                fn check_rate_conversion_symmetry(rate:f64, periods:u32) {
                    let ap = convert_apr_to_epr_solution(rate, periods);
                    let ae = convert_apr_to_ear_solution(rate, periods);

                    assert_rounded_4!(ap.output_rate, convert_ear_to_epr(ae.output_rate, periods));
                    assert_rounded_4!(ae.output_rate, convert_epr_to_ear(ap.output_rate, periods));
                    

                    let pa = convert_epr_to_apr_solution(rate, periods);
                    let pe = convert_epr_to_ear_solution(rate, periods);
                    
                    let ea = convert_ear_to_apr_solution(rate, periods);
                    let ep = convert_ear_to_epr_solution(rate, periods);

                    
                }
                
            }
        }
    }

    #[test]
    fn test_convert_rates_simple_1() {
        // test on excel values using 12 periods
        const periods: u32 = 12;
        let apr_epr_ear_rates = vec!((0.034, 0.00283333333, 0.0345348693603),
                                     (-0.034, -0.002833333333, -0.03347513889),
                                     (1.0, 0.08333333333, 1.6130352902247),
                                     (-1.0, -0.083333333333, -0.64800437199),
                                     (2.1, 0.175, 5.9255520766347),
                                     (-2.1, -0.175,-0.90058603794));
        for rate_tupe in apr_epr_ear_rates {
                let ap = convert_apr_to_epr(rate_tupe.0, periods);
                let ae = convert_apr_to_ear(rate_tupe.0, periods);
                let pa = convert_epr_to_apr(rate_tupe.1, periods);
                let pe = convert_epr_to_ear(rate_tupe.1, periods);
                let ea = convert_ear_to_apr(rate_tupe.2, periods);
                let ep = convert_ear_to_epr(rate_tupe.2, periods);
                check_rate_conversion_symmetry(ap, ae, pa, pe, ea, ep);

                fn check_rate_conversion_symmetry(ap:f64, ae:f64, pa:f64, pe:f64, ea:f64, ep:f64) {
                    assert_eq!( round_6(ap), round_6(ep) );
                    assert_eq!( round_6(ae), round_6(pe) );
                    assert_eq!( round_6(pa), round_6(ea) );
                }
        }
    }
}