#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use float_cmp::ApproxEq;
use log::Level;
use log::{warn}; // consider how to use: log_enabled   also, error (runtime?) vs assert (runtime)

pub fn main() { 
    // try_convert_apr_to_periodic();
    // try_convert_apr_to_periodic_f64();
    // try_convert_apr_to_ear_f64();
    // try_convert_periodic_to_ear_f64();

    
    dbg!(convert_periodic_to_ear(-1.1, 12));
    dbg!(convert_periodic_to_ear(2.04, 12));
    dbg!(round_to_ulps_4(622991.76529749));


}

// note: APR is also called "standard interest rate" or just "interest rate".
// EAR and APY are the same thing. EAR is more polite than APY because the "Y" for yield reminds people that someone is earning a yield from the deal.
// To remember simply, a monthly periodic rate is the APR/12.

fn try_convert_apr_to_periodic() {
    // expect 0.0045833333 repeating
    let apr = 0.055_f64;
    let num_periods = 12_u32; // monthly periods
    dbg!(convert_apr_to_periodic(apr, num_periods));
}

fn try_convert_apr_to_periodic_f64() {
    // expect 0.0045833333 repeating
    let apr = 0.055_f64;
    let num_periods = 12_u32; // monthly periods
    let periodic_rate = convert_apr_to_periodic_f64(apr, num_periods);
    dbg!(periodic_rate);

    // expect 0.001057692307692307
    let apr = 0.055_f64;
    let num_periods = 52_u32; // weekly periods
    let periodic_rate = convert_apr_to_periodic_f64(apr, num_periods);
    dbg!(periodic_rate);   
    
    // expect 0.001057692307692307
    // get warn
    let apr = 5.5_f64;
    let num_periods = 52_u32; // weekly periods
    let periodic_rate = convert_apr_to_periodic_f64(apr, num_periods);
    dbg!(periodic_rate);  
}

// calculator to check EAR conversions
// https://www.calculatorsoup.com/calculators/financial/effective-annual-rate-calculator.php

fn try_convert_apr_to_ear_f64() {
    // expect 3.2989%  (0.03298851181)
    let apr = 0.0325;
    let compounding_periods_in_year = 12_f64; // monthly
    let apr_to_ear = convert_apr_to_ear_f64(apr, compounding_periods_in_year);
    dbg!(apr_to_ear);

    // expect 0.2503%
    let apr = 0.0025;
    let compounding_periods_in_year = 365_f64; // daily compounding
    let apr_to_ear = convert_apr_to_ear_f64(apr, compounding_periods_in_year);
    dbg!(apr_to_ear);

    // get warns
    let apr = 1.5;
    let compounding_periods_in_year = 730; // twice daily? maybe in forex...
    let apr_to_ear = convert_apr_to_ear_f64(apr, compounding_periods_in_year);
    dbg!(apr_to_ear);

    // expect 1.2578%
    let apr = 0.0125;
    let apr_to_ear = convert_apr_to_ear_continuous_compound_f64(apr);
    dbg!(apr_to_ear);
    
}

fn try_convert_periodic_to_ear_f64() {
    // https://www.investopedia.com/articles/investing/121713/interest-rates-apr-apy-and-ear.asp
    // expect 25.72% EAR (also known as APY)
    let apr: f64 = 0.228964;
    let num_compounding_periods: u32 = 365; // daily compounding
    let periodic_rate = apr / num_compounding_periods as f64;
    let ear_solution = convert_periodic_to_ear(periodic_rate, num_compounding_periods);
    dbg!(ear_solution);
}

#[derive(Debug)]
pub struct AprEarSolution {
    pub annual_percentage_rate: f64,
    pub num_periods_in_year: f64,
    pub effective_annual_rate: f64,
    pub input_in_percent: String,
    pub output_in_percent: String,
}
impl AprEarSolution {
    pub fn new(annual_percentage_rate: f64,num_periods_in_year: f64,effective_annual_rate: f64, input_in_percent: String, output_in_percent: String) -> Self {
        Self {
            annual_percentage_rate,
            num_periods_in_year,
            effective_annual_rate,
            input_in_percent,
            output_in_percent,
        }
    }
}

#[derive(Debug)]
pub struct AprPeriodicSolution {
    pub annual_percentage_rate: f64,
    pub num_periods_in_year: f64,
    pub periodic_rate: f64,
    pub input_in_percent: String,
    pub output_in_percent: String,
}
impl AprPeriodicSolution {
    pub fn new(annual_percentage_rate: f64,num_periods_in_year: f64,periodic_rate: f64, input_in_percent: String, output_in_percent: String) -> Self {
        Self {
            annual_percentage_rate,
            num_periods_in_year,
            periodic_rate,
            input_in_percent,
            output_in_percent,
        }
    }
}

/// Convert APR (annual rate) to periodic rate. Returns a custom solution type.
pub fn convert_apr_to_periodic(apr: f64, num_periods_in_year: u32) -> AprPeriodicSolution {
    let n = num_periods_in_year as f64;
    assert!(n >= 1.);
    assert!(n.is_finite());
    assert!(apr.is_finite());
    if apr > 1. || apr < -1. {
        warn!("You provided an APR of {}%. Are you sure?", apr*100.);
    }
    let periodic_rate = apr / n;
    let input_in_percent = format!("{}%", (apr * 100_000.0).round() / 100_000.0 * 100.);
    let output_in_percent = format!("{:.4}%", (periodic_rate * 100.));
    AprPeriodicSolution::new(apr, n, periodic_rate, input_in_percent, output_in_percent)
}
/// Convert APR (annual rate) to periodic rate. Returns f64.
pub fn convert_apr_to_periodic_f64(apr: f64, num_periods_in_year: u32) -> f64 {
    convert_apr_to_periodic(apr, num_periods_in_year).periodic_rate
}

/// Convert periodic rate to APR (aka Annual rate, nominal interest rate, Annual Percentage Rate). Returns a custom solution type.
pub fn convert_periodic_to_apr<T: Into<f64> + Copy>(periodic_rate: f64, periods_per_year: T) -> AprPeriodicSolution {
    let n = periods_per_year.into();
    assert!(n >= 0.);
    assert!(n.is_finite());
    assert!(periodic_rate.is_finite());
    if periodic_rate > 1. || periodic_rate < -1. {
        warn!("You provided a periodic rate of {}%. Are you sure?", periodic_rate*100.);
    }
    let apr = periodic_rate * n;
    let input_in_percent = format!("{}%", (periodic_rate * 100_000.0).round() / 100_000.0 * 100.);
    let output_in_percent = format!("{:.4}%", (apr * 100.));
    AprPeriodicSolution::new(apr, n, periodic_rate, input_in_percent, output_in_percent)
}
/// Convert periodic rate to APR (aka Annual rate, nominal interest rate, Annual Percentage Rate). Returns f64.
pub fn convert_periodic_to_apr_f64<T: Into<f64> + Copy>(periodic_rate: f64, periods_per_year: T) -> f64 {
    let n = periods_per_year.into();
    assert!(n >= 1.);
    assert!(n.is_finite());
    assert!(periodic_rate.is_finite());
    if periodic_rate > 1. || periodic_rate < -1. {
        warn!("You provided a periodic rate of {}%. Are you sure?", periodic_rate*100.);
    }
    periodic_rate * n
}

/// Convert a nominal interest rate (Annual rate, APR) to EAR (effective annual rate). Returns a custom solution type.
pub fn convert_apr_to_ear<T: Into<f64> + Copy>(apr: f64, compounding_periods_in_year: T) -> AprEarSolution {
    let n = compounding_periods_in_year.into();
    assert!(n >= 1.);
    assert!(n.is_finite());
    assert!(apr.is_finite());
    if apr > 1. || apr < -1. {
        warn!("You provided an APR of {}%. Are you sure?", apr*100.);
    }
    if apr < 0.  {
        warn!("You provided a negative APR of {}%. Are you sure?", apr*100.);
    }
    if n > 366. {
        warn!("You provided more than 366 compounding periods in a year (You provided {}). Are you sure?", n);
    }
    let ear: f64;
    if n == 1. { 
        ear = apr;
    } else if apr < 0. {
        // for the rare case of negative nominal rates...
        let pos_apr = apr * -1.0;
        ear = ((1_f64 + (pos_apr/n)).powf(n) - 1_f64) * -1.0;
    } else {
        ear = (1_f64 + (apr/n)).powf(n) - 1_f64;
    }
    let input_in_percent = format!("{}%", (apr * 100_000.0).round() / 100_000.0 * 100.);
    let output_in_percent = format!("{:.4}%", (ear * 100.));
    AprEarSolution::new(apr, n, ear, input_in_percent, output_in_percent)
}
/// Convert a nominal interest rate (Annual rate, APR) to EAR (effective annual rate). Returns f64.
pub fn convert_apr_to_ear_f64<T: Into<f64> + Copy>(apr: f64, compounding_periods_in_year: T) -> f64 {
    let n = compounding_periods_in_year.into();
    assert!(n >= 1.);
    assert!(n.is_finite());
    assert!(apr.is_finite());
    if apr > 1. || apr < -1. {
        warn!("You provided an APR of {}%. Are you sure?", apr*100.);
    }
    if apr < 0.  {
        warn!("You provided a negative APR of {}%. Are you sure?", apr*100.);
    }
    if n > 366. {
        warn!("You provided more than 366 compounding periods in a year (You provided {}). Are you sure?", n);
    }
    if n == 1. { 
        return apr 
    } else if apr < 0. {
        // for the rare case of negative nominal rates...
        let pos_apr = apr * -1.0;
        return ((1_f64 + (pos_apr/n)).powf(n) - 1_f64) * -1.0
    } else {
        return (1_f64 + (apr/n)).powf(n) - 1_f64
    }
}

/// Convert an EAR (effective annual rate) to a nominal interest rate (Annual rate, APR). This involves converting the EAR to periodic rate first, and then APR = periodic rate * number of periods. Returns f64.
pub fn convert_ear_to_apr<T: Into<f64> + Copy>(ear: f64, compounding_periods_in_year: T) -> AprEarSolution {
    let n = compounding_periods_in_year.into();
    assert!(n >= 1., "Periods provided must be greater than or equal to 1.0");
    assert!(n.is_finite());
    assert!(ear.is_finite());
    assert!(ear > -1., "Effective Annual Rate must be greater than -1.0, or else the formula breaks down and creates an imaginary number");
    if ear > 1. || ear < -1. {
        warn!("You provided an EAR of {}%. Are you sure?", ear*100.);
    }
    if ear < 0.  {
        warn!("You provided a negative EAR of {}%. Are you sure?", ear*100.);
    }
    if n > 366. {
        warn!("You provided more than 366 compounding periods in a year (You provided {}). Are you sure?", n);
    }
    
    // APR is the periodic rate * number of periods, so convert EAR to periodic, then * n
    // convert_ear_to_periodic_f64(ear, n) * n
    let apr: f64;
    if n == 1. { 
        apr = ear;
    } else {
        apr = ((1_f64 + ear).powf(1_f64/n) - 1_f64) * n;
    }
    let input_in_percent = format!("{}%", (ear * 100_000.0).round() / 100_000.0 * 100.);
    let output_in_percent = format!("{:.4}%", (apr * 100.));
    AprEarSolution::new(apr, n, ear, input_in_percent, output_in_percent)
}
/// Convert an EAR (effective annual rate) to a nominal interest rate (Annual rate, APR). This involves converting the EAR to periodic rate first, and then APR = periodic rate * number of periods. Returns f64.
pub fn convert_ear_to_apr_f64<T: Into<f64> + Copy>(ear: f64, compounding_periods_in_year: T) -> f64 {
    let n = compounding_periods_in_year.into();
    assert!(n >= 1., "Periods provided must be greater than or equal to 1.0");
    assert!(n.is_finite());
    assert!(ear.is_finite());
    assert!(ear > -1., "Effective Annual Rate must be greater than -1.0, or else the formula breaks down and creates an imaginary number");
    if ear > 1. || ear < -1. {
        warn!("You provided an EAR of {}%. Are you sure?", ear*100.);
    }
    if ear < 0.  {
        warn!("You provided a negative EAR of {}%. Are you sure?", ear*100.);
    }
    if n > 366. {
        warn!("You provided more than 366 compounding periods in a year (You provided {}). Are you sure?", n);
    }
    if n == 1. { return ear }
    // APR is the periodic rate * number of periods, so convert EAR to periodic, then * n
    // convert_ear_to_periodic_f64(ear, n) * n
    ((1_f64 + ear).powf(1_f64/n) - 1_f64) * n
}

pub struct AprEarContinous {
    pub apr: f64,
    pub ear: f64,
    pub formula: String
}
impl AprEarContinous {
    pub fn new(apr: f64, ear: f64) -> Self {
        let formula = format!("{}^{} - 1", 2.71828182845904, apr);
        Self {
            apr,
            ear,
            formula,
        }
    }
}
/// Convert a nominal interest rate (Annual rate) to EAR (effective annual rate) using continuous compounding.
pub fn convert_apr_to_ear_continuous_compound(apr: f64) -> AprEarContinous {
    assert!(apr.is_finite());
    if apr > 1. || apr < -1. {
        warn!("You provided an APR of {}%. Are you sure?", apr*100.);
    }
    // formula: e^apr - 1
    let e: f64 = 2.71828182845904; 
    let ear: f64;
    if apr < 0.0 {
        // when apr is negative...
        ear = (e.powf(apr.abs()) - 1_f64) * -1_f64;
    } else {
        ear = e.powf(apr) - 1_f64;
    }
    AprEarContinous::new(apr, ear)
}

/// Convert a nominal interest rate (Annual rate) to EAR (effective annual rate) using continuous compounding.
pub fn convert_apr_to_ear_continuous_compound_f64(apr: f64) -> f64 {
    convert_apr_to_ear_continuous_compound(apr).ear
}


#[derive(Debug)]
pub struct PeriodicToEarSolution {
    pub periodic_rate: f64,
    pub compounding_periods_in_year: f64,
    pub effective_annual_rate: f64,
    pub input_in_percent: String,
    pub output_in_percent: String,
    pub formula: String,
}
impl PeriodicToEarSolution {
    pub fn new(periodic_rate: f64,compounding_periods_in_year: f64,effective_annual_rate: f64, input_in_percent: String, output_in_percent: String) -> Self {
        let formula = format!("(1 + {})^{} - 1", periodic_rate, compounding_periods_in_year);
        Self {
            periodic_rate,
            compounding_periods_in_year,
            effective_annual_rate,
            input_in_percent,
            output_in_percent,
            formula,
        }
    }
}

/// Convert a periodic interest rate (APR / num of compounding periods) to EAR (effective annual rate).
pub fn convert_periodic_to_ear_solution(periodic_rate: f64, compounding_periods_in_year: u32) -> PeriodicToEarSolution {
    let n = compounding_periods_in_year as f64;
    assert!(n >= 0.);
    assert!(periodic_rate.is_finite());
    if periodic_rate > 1. || periodic_rate < -1. {
        warn!("You provided an periodic rate of {}%. Are you sure?", periodic_rate*100.);
    }
    let ear: f64;
    if n == 1. { 
        ear = periodic_rate;
    } else if periodic_rate < 0.0 {
        ear = ((1_f64 + periodic_rate.abs()).powf(n) - 1_f64) * -1_f64;
    } else {
        ear = (1_f64 + periodic_rate).powf(n) - 1_f64;
    }

    let input_string = (periodic_rate*100.).to_string();
    let output_string = (ear*100.).to_string();
    PeriodicToEarSolution::new(periodic_rate, n, ear, input_string, output_string)
}

/// Convert a periodic interest rate (APR / num of compounding periods) to EAR (effective annual rate).
pub fn convert_periodic_to_ear(periodic_rate: f64, compounding_periods_in_year: u32) -> f64 {
    convert_periodic_to_ear_solution(periodic_rate, compounding_periods_in_year).effective_annual_rate
}

#[derive(Debug)]
pub struct EarToPeriodicSolution {
    pub effective_annual_rate: f64,
    pub compounding_periods_in_year: f64,
    pub periodic_rate: f64,
    pub input_in_percent: String,
    pub output_in_percent: String,
    pub formula: String,
}
impl EarToPeriodicSolution {
    pub fn new(effective_annual_rate: f64, compounding_periods_in_year: f64, periodic_rate: f64, input_in_percent: String, output_in_percent: String) -> Self {
        let formula = format!("(1 + {})^(1/{}) - 1", effective_annual_rate, compounding_periods_in_year);
        Self {
            effective_annual_rate,
            compounding_periods_in_year,
            periodic_rate,
            input_in_percent,
            output_in_percent,
            formula,
        }
    }
}

/// Convert an EAR (Effective Annual Rate) to periodic rate.
pub fn convert_ear_to_periodic_solution(ear: f64, periods_per_year: u32) -> EarToPeriodicSolution {
    let n = periods_per_year as f64;
    assert!(n >= 0.);
    if ear > 1. || ear < -1. {
        warn!("You provided an Effective Annual Rate of {}%. Are you sure?", ear*100.);
    }
    let periodic_rate: f64;
    if n == 1. { 
         periodic_rate = ear;
    } else {
        // EPR = (1 + annual rate)^(1/#ofPeriodsPerYear) 
        periodic_rate = (1_f64 + ear).powf(1_f64/n) - 1_f64;
    }
    let input_string = (ear*100.).to_string();
    let output_string = (periodic_rate*100.).to_string();
    EarToPeriodicSolution::new(ear, n, periodic_rate, input_string, output_string)
}

/// Convert an EAR (Effective Annual Rate) to periodic rate.
pub fn convert_ear_to_periodic_f64(ear: f64, periods_per_year: u32) -> f64 {
    convert_ear_to_periodic_solution(ear, periods_per_year).periodic_rate
}

pub fn round_to_ulps_6(val: f64) -> f64 {
    (val * 1_000_000.0).round() / 1_000_000.0
}
pub fn round_to_ulps_4(val: f64) -> f64 {
    (val * 10_000.0).round() / 10_000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apr_to_periodic_f64_1() {
        // normal case
        let input_rate = 0.03;
        let periods = 6;
        let expected_output = 0.00500;
        let actual_output = convert_apr_to_periodic_f64(input_rate, periods);
        assert!( float_cmp::approx_eq!(f64, expected_output, actual_output, ulps = 4) );
    }
    
    #[test]
    fn test_apr_to_periodic_f64_2() {
        // normal case, negative rate
        let input_rate = -0.03;
        let periods = 6;
        let expected_output = -0.00500;
        let actual_output = convert_apr_to_periodic_f64(input_rate, periods);
        assert_eq!(expected_output, actual_output);
    }

    #[test]
    fn test_apr_to_periodic_f64_3() {
        // big numbers, negative rate
        let input_rate = -0.78;
        let periods = 35;
        let expected_output = -0.0222857142;
        let actual_output = convert_apr_to_periodic_f64(input_rate, periods);
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_apr_to_periodic_f64_4() {
        // big rate
        let input_rate = 0.99;
        let periods = 13;
        let expected_output = 0.0761538462;
        let actual_output = convert_apr_to_periodic_f64(input_rate, periods);
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_apr_to_periodic_f64_5() {
        //  rate of 100%
        let input_rate = 1.00;
        let periods = 13;
        let expected_output = 0.0769230769;
        let actual_output = convert_apr_to_periodic_f64(input_rate, periods);
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_apr_to_periodic_f64_6() {
        //  rate over 1.0
        let input_rate = 3.34;
        let periods = 13;
        let expected_output = 0.2569230769;
        let actual_output = convert_apr_to_periodic_f64(input_rate, periods);
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_apr_to_periodic_f64_7() {
        //  rate is -1.0
        let input_rate = -1.0;
        let periods = 13;
        let expected_output = -0.0769230769;
        let actual_output = convert_apr_to_periodic_f64(input_rate, periods);
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_apr_to_periodic_f64_8() {
        //  rate below -1.0
        let input_rate = -2.3334;
        let periods = 13;
        let expected_output = -0.1794923077;
        let actual_output = convert_apr_to_periodic_f64(input_rate, periods);
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_apr_to_periodic_f64_12() {
        //  period = 1
        let input_rate = 0.34;
        let periods = 1;
        let expected_output = 0.34;
        let actual_output = convert_apr_to_periodic_f64(input_rate, periods);
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
        // assert!( float_cmp::approx_eq!(f64, expected_output, actual_output, ulps = 2) );
    }

    #[should_panic]
    #[test]
    fn test_apr_to_periodic_f64_13() {
        //  infinite rate
        let input_rate = 1_f64 / 0_f64;
        let periods = 3;
        let _should_panic = convert_apr_to_periodic_f64(input_rate, periods);
    }
    
    /* 
    ***********************
    ** Solution type tests
    ***********************
    */

    #[test]
    fn test_apr_to_periodic_1() {
        // normal case
        let input_rate = 0.03;
        let periods = 6;
        let expected_output = 0.00500;
        let actual_output = convert_apr_to_periodic(input_rate, periods).periodic_rate;
        assert!( float_cmp::approx_eq!(f64, expected_output, actual_output, ulps = 4) );
    }
    
    #[test]
    fn test_apr_to_periodic_2() {
        // normal case, negative rate
        let input_rate = -0.03;
        let periods = 6;
        let expected_output = -0.00500;
        let actual_output = convert_apr_to_periodic(input_rate, periods).periodic_rate;
        assert_eq!(expected_output, actual_output);
    }

    #[test]
    fn test_apr_to_periodic_3() {
        // big numbers, negative rate
        let input_rate = -0.78;
        let periods = 35;
        let expected_output = -0.0222857142;
        let actual_output = convert_apr_to_periodic(input_rate, periods).periodic_rate;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_apr_to_periodic_4() {
        // big rate
        let input_rate = 0.99;
        let periods = 13;
        let expected_output = 0.0761538462;
        let actual_output = convert_apr_to_periodic(input_rate, periods).periodic_rate;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_apr_to_periodic_5() {
        //  rate of 100%
        let input_rate = 1.00;
        let periods = 13;
        let expected_output = 0.0769230769;
        let actual_output = convert_apr_to_periodic(input_rate, periods).periodic_rate;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_apr_to_periodic_6() {
        //  rate over 1.0
        let input_rate = 3.34;
        let periods = 13;
        let expected_output = 0.2569230769;
        let actual_output = convert_apr_to_periodic(input_rate, periods).periodic_rate;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_apr_to_periodic_7() {
        //  rate is -1.0
        let input_rate = -1.0;
        let periods = 13;
        let expected_output = -0.0769230769;
        let actual_output = convert_apr_to_periodic(input_rate, periods).periodic_rate;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_apr_to_periodic_8() {
        //  rate below -1.0
        let input_rate = -2.3334;
        let periods = 13;
        let expected_output = -0.1794923077;
        let actual_output = convert_apr_to_periodic(input_rate, periods).periodic_rate;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_apr_to_periodic_12() {
        //  period = 1
        let input_rate = 0.34;
        let periods = 1;
        let expected_output = 0.34;
        let actual_output = convert_apr_to_periodic(input_rate, periods).periodic_rate;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
        // assert!( float_cmp::approx_eq!(f64, expected_output, actual_output, ulps = 2) );
    }

    #[should_panic]
    #[test]
    fn test_apr_to_periodic_13() {
        //  infinite rate
        let input_rate = 1_f64 / 0_f64;
        let periods = 3;
        let _should_panic = convert_apr_to_periodic(input_rate, periods);
    }


    /* 
    ****************************************
    ******** periodic to apr ***************
    ****************************************
    */

    #[test]
    fn test_periodic_to_apr_f64_1() {
        // normal case
        let input_rate =  0.00500;
        let periods = 6;
        let expected_output = 0.03;
        let actual_output = convert_periodic_to_apr_f64(input_rate, periods);
        assert!( float_cmp::approx_eq!(f64, expected_output, actual_output, ulps = 4) );
    }
    
    #[test]
    fn test_periodic_to_apr_f64_2() {
        // normal case, negative rate
        let input_rate = -0.00500;
        let periods = 6;
        let expected_output = -0.03;
        let actual_output = convert_periodic_to_apr_f64(input_rate, periods);
        assert_eq!(expected_output, actual_output);
    }

    #[test]
    fn test_periodic_to_apr_f64_3() {
        // big numbers, negative rate
        let input_rate = -2.3;
        let periods = 35;
        let expected_output = -80.50;
        let actual_output = convert_periodic_to_apr_f64(input_rate, periods);
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_periodic_to_apr_f64_4() {
        // long float
        let input_rate = 0.0761538462;
        let periods = 13;
        let expected_output = 0.99;
        let actual_output = convert_periodic_to_apr_f64(input_rate, periods);
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_periodic_to_apr_f64_5() {
        //  rate of 100%
        let input_rate = 1.00;
        let periods = 13;
        let expected_output = 13.00;
        let actual_output = convert_periodic_to_apr_f64(input_rate, periods);
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_periodic_to_apr_f64_6() {
        //  rate over 1.0
        let input_rate = 1.1;
        let periods = 13;
        let expected_output = 14.3;
        let actual_output = convert_periodic_to_apr_f64(input_rate, periods);
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_periodic_to_apr_f64_7() {
        //  rate is -1.0
        let input_rate = -1.0;
        let periods = 13;
        let expected_output = -13.0;
        let actual_output = convert_periodic_to_apr_f64(input_rate, periods);
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_periodic_to_apr_f64_8() {
        //  rate below -1.0
        let input_rate = -1.1794;
        let periods = 13;
        let expected_output = -15.3322;
        let actual_output = convert_periodic_to_apr_f64(input_rate, periods);
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }


    #[test]
    fn test_periodic_to_apr_f64_12() {
        //  period = 1
        let input_rate = 0.34;
        let periods = 1;
        let expected_output = 0.34;
        let actual_output = convert_periodic_to_apr_f64(input_rate, periods);
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
        // assert!( float_cmp::approx_eq!(f64, expected_output, actual_output, ulps = 2) );
    }

    #[should_panic]
    #[test]
    fn test_periodic_to_apr_f64_13() {
        //  infinite rate
        let input_rate = 1_f64 / 0_f64;
        let periods = 3;
        let _should_panic = convert_periodic_to_apr_f64(input_rate, periods);
    }


    #[test]
    fn test_periodic_to_apr_1() {
        // normal case
        let input_rate =  0.00500;
        let periods = 6;
        let expected_output = 0.03;
        let actual_output = convert_periodic_to_apr(input_rate, periods).annual_percentage_rate;
        assert!( float_cmp::approx_eq!(f64, expected_output, actual_output, ulps = 4) );
    }
    
    #[test]
    fn test_periodic_to_apr_2() {
        // normal case, negative rate
        let input_rate = -0.00500;
        let periods = 6;
        let expected_output = -0.03;
        let actual_output = convert_periodic_to_apr(input_rate, periods).annual_percentage_rate;
        assert_eq!(expected_output, actual_output);
    }

    #[test]
    fn test_periodic_to_apr_3() {
        // big numbers, negative rate
        let input_rate = -2.3;
        let periods = 35;
        let expected_output = -80.50;
        let actual_output = convert_periodic_to_apr(input_rate, periods).annual_percentage_rate;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_periodic_to_apr_4() {
        // long float
        let input_rate = 0.0761538462;
        let periods = 13;
        let expected_output = 0.99;
        let actual_output = convert_periodic_to_apr(input_rate, periods).annual_percentage_rate;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_periodic_to_apr_5() {
        //  rate of 100%
        let input_rate = 1.00;
        let periods = 13;
        let expected_output = 13.00;
        let actual_output = convert_periodic_to_apr(input_rate, periods).annual_percentage_rate;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_periodic_to_apr_6() {
        //  rate over 1.0
        let input_rate = 1.1;
        let periods = 13;
        let expected_output = 14.3;
        let actual_output = convert_periodic_to_apr(input_rate, periods).annual_percentage_rate;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_periodic_to_apr_7() {
        //  rate is -1.0
        let input_rate = -1.0;
        let periods = 13;
        let expected_output = -13.0;
        let actual_output = convert_periodic_to_apr(input_rate, periods).annual_percentage_rate;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_periodic_to_apr_8() {
        //  rate below -1.0
        let input_rate = -1.1794;
        let periods = 13;
        let expected_output = -15.3322;
        let actual_output = convert_periodic_to_apr(input_rate, periods).annual_percentage_rate;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_periodic_to_apr_12() {
        //  period = 1
        let input_rate = 0.34;
        let periods = 1;
        let expected_output = 0.34;
        let actual_output = convert_periodic_to_apr(input_rate, periods).annual_percentage_rate;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
        // assert!( float_cmp::approx_eq!(f64, expected_output, actual_output, ulps = 2) );
    }

    #[should_panic]
    #[test]
    fn test_periodic_to_apr_13() {
        //  infinite rate
        let input_rate = 1_f64 / 0_f64;
        let periods = 3;
        let _should_panic = convert_periodic_to_apr(input_rate, periods);
    }

    #[should_panic]
    #[test]
    fn test_periodic_to_apr_15() {
        //  rate = 0
        let input_rate = 0.0;
        let periods = 1_f64 / 0_f64;
        let _should_panic = convert_periodic_to_apr(input_rate, periods);
    }

    /* 
    ****************************************
    ************* apr to ear ***************
    ****************************************
    */

    #[test]
    fn test_apr_to_ear_f64_1() {
        // normal case
        let input_rate = 0.0325;
        let periods = 12;
        let expected_output = 0.03298851181;
        let actual_output = convert_apr_to_ear_f64(input_rate, periods);
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }
    
    #[test]
    fn test_apr_to_ear_f64_2() {
        // rate < 0
        let input_rate = -0.0325;
        let periods = 12;
        let expected_output = -0.03298851181;
        let actual_output = convert_apr_to_ear_f64(input_rate, periods);
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_apr_to_ear_f64_3() {
        // rate over 1.0 
        let input_rate = 1.34;
        let periods = 12;
        let expected_output = 2.562008918;
        let actual_output = convert_apr_to_ear_f64(input_rate, periods);
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_apr_to_ear_f64_4() {
        // rate = 1.0 
        let input_rate = 1.0;
        let periods = 12;
        let expected_output = 1.61303529;
        let actual_output = convert_apr_to_ear_f64(input_rate, periods);
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_apr_to_ear_f64_5() {
        // rate = 0.0 
        let input_rate = 0.0;
        let periods = 12;
        let expected_output = 0.0;
        let actual_output = convert_apr_to_ear_f64(input_rate, periods);
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_apr_to_ear_f64_6() {
        // rate < -1.0
        let input_rate = -3.5;
        let periods = 12;
        let expected_output = -20.56777904;
        let actual_output = convert_apr_to_ear_f64(input_rate, periods);
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_apr_to_ear_f64_7() {
        // rate = -1.0
        let input_rate = -1.0;
        let periods = 12;
        let expected_output = -1.61303529;
        let actual_output = convert_apr_to_ear_f64(input_rate, periods);
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[should_panic]
    #[test]
    fn test_apr_to_ear_f64_8() {
        // rate = infinity
        let input_rate = 1_f64/0_f64;
        let periods = 12;
        let _should_panic = convert_apr_to_ear_f64(input_rate, periods);
    }

    #[test]
    fn test_apr_to_ear_f64_9() {
        // periods = 1
        let input_rate = 0.034;
        let periods = 1;
        let expected_output = 0.034;
        let actual_output = convert_apr_to_ear_f64(input_rate, periods);
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[should_panic]
    #[test]
    fn test_apr_to_ear_f64_12() {
        // - periods = 0 
        let input_rate = 0.034;
        let periods = 0.0;
        let _should_panic = convert_apr_to_ear_f64(input_rate, periods);
    }

    /* 
    ****************************************
    ******* apr to ear Solution type *******
    ****************************************
    */

    #[test]
    fn test_apr_to_ear_1() {
        // normal case
        let input_rate = 0.0325;
        let periods = 12;
        let expected_output = 0.03298851181;
        let actual_output = convert_apr_to_ear(input_rate, periods).effective_annual_rate;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }
    
    #[test]
    fn test_apr_to_ear_2() {
        // rate < 0
        let input_rate = -0.0325;
        let periods = 12;
        let expected_output = -0.03298851181;
        let actual_output = convert_apr_to_ear(input_rate, periods).effective_annual_rate;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_apr_to_ear_3() {
        // rate over 1.0 
        let input_rate = 1.34;
        let periods = 12;
        let expected_output = 2.562008918;
        let actual_output = convert_apr_to_ear(input_rate, periods).effective_annual_rate;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_apr_to_ear_4() {
        // rate = 1.0 
        let input_rate = 1.0;
        let periods = 12;
        let expected_output = 1.61303529;
        let actual_output = convert_apr_to_ear(input_rate, periods).effective_annual_rate;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_apr_to_ear_5() {
        // rate = 0.0 
        let input_rate = 0.0;
        let periods = 12;
        let expected_output = 0.0;
        let actual_output = convert_apr_to_ear(input_rate, periods).effective_annual_rate;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_apr_to_ear_6() {
        // rate < -1.0
        let input_rate = -3.5;
        let periods = 12;
        let expected_output = -20.56777904;
        let actual_output = convert_apr_to_ear(input_rate, periods).effective_annual_rate;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_apr_to_ear_7() {
        // rate = -1.0
        let input_rate = -1.0;
        let periods = 12;
        let expected_output = -1.61303529;
        let actual_output = convert_apr_to_ear(input_rate, periods).effective_annual_rate;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[should_panic]
    #[test]
    fn test_apr_to_ear_8() {
        // rate = infinity
        let input_rate = 1_f64/0_f64;
        let periods = 12;
        let _should_panic = convert_apr_to_ear(input_rate, periods);
    }

    #[test]
    fn test_apr_to_ear_9() {
        // periods = 1
        let input_rate = 0.034;
        let periods = 1;
        let expected_output = 0.034;
        let actual_output = convert_apr_to_ear(input_rate, periods).effective_annual_rate;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[should_panic]
    #[test]
    fn test_apr_to_ear_12() {
        // - periods = 0 
        let input_rate = 0.034;
        let periods = 0.0;
        let _should_panic = convert_apr_to_ear(input_rate, periods);
    }

    #[test]
    fn test_apr_to_ear_15() {
        // periods as various types
        let input_rate = 0.0325;
        let periods = 12_u8;
        let expected_output = 0.03298851181;
        let actual_output = convert_apr_to_ear(input_rate, periods).effective_annual_rate;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_apr_to_ear_16() {
        // periods as various types
        let input_rate = 0.0325;
        let periods = 12_i8;
        let expected_output = 0.03298851181;
        let actual_output = convert_apr_to_ear(input_rate, periods).effective_annual_rate;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_apr_to_ear_17() {
        // periods as various types 
        let input_rate = 0.0325;
        let periods = 12_f32;
        let expected_output = 0.03298851181;
        let actual_output = convert_apr_to_ear(input_rate, periods).effective_annual_rate;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }


    /* 
    ****************************************
    ************* apr to ear ***************
    ****************************************
    */

    #[test]
    fn test_ear_to_apr_f64_1() {
        // normal case
        let input_rate = 0.03298851181;
        let periods = 12;
        let expected_output = 0.0325;
        let actual_output = convert_ear_to_apr_f64(input_rate, periods);
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }
    
    #[test]
    fn test_ear_to_apr_f64_2() {
        // -1 < rate < 0
        let input_rate = -0.03298851181;
        let periods = 12;
        let expected_output = -0.0325;
        let actual_output = convert_ear_to_apr_f64(input_rate, periods);
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_ear_to_apr_f64_3() {
        // rate over 1.0 
        let input_rate = 2.562008918;
        let periods = 12;
        let expected_output = 1.34;
        let actual_output = convert_ear_to_apr_f64(input_rate, periods);
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_ear_to_apr_f64_4() {
        // rate = 1.0 
        let input_rate = 1.0;
        let periods = 12;
        let expected_output = 0.7135571323;
        let actual_output = convert_ear_to_apr_f64(input_rate, periods);
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_ear_to_apr_f64_5() {
        // rate = 0.0 
        let input_rate = 0.0;
        let periods = 12;
        let expected_output = 0.0;
        let actual_output = convert_ear_to_apr_f64(input_rate, periods);
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[should_panic]
    #[test]
    fn test_ear_to_apr_f64_6() {
        // rate < -1.0
        let input_rate = -1.1;
        let periods = 12;
        let _should_panic = convert_ear_to_apr_f64(input_rate, periods);
    }

    #[should_panic]
    #[test]
    fn test_ear_to_apr_f64_7() {
        // rate = -1.0
        let input_rate = -1.0;
        let periods = 12;
        let actual_output = convert_ear_to_apr_f64(input_rate, periods);
    }

    #[should_panic]
    #[test]
    fn test_ear_to_apr_f64_8() {
        // rate = infinity
        let input_rate = 1_f64/0_f64;
        let periods = 12;
        let _should_panic = convert_ear_to_apr_f64(input_rate, periods);
    }

    #[test]
    fn test_ear_to_apr_f64_9() {
        // periods = 1
        let input_rate = 0.034;
        let periods = 1;
        let expected_output = 0.034;
        let actual_output = convert_ear_to_apr_f64(input_rate, periods);
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_ear_to_apr_f64_10() {
        // - periods as float, > 1 (float)
        let input_rate = 0.03456823374;
        let periods = 36.5;
        let expected_output = 0.034;
        let actual_output = convert_ear_to_apr_f64(input_rate, periods);
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[should_panic]
    #[test]
    fn test_ear_to_apr_f64_11() {
        // - periods as fractional, 0 < n < 1 
        let input_rate = 0.034;
        let periods = 0.5;
        let _should_panic = convert_ear_to_apr_f64(input_rate, periods);
    }

    #[should_panic]
    #[test]
    fn test_ear_to_apr_f64_12() {
        // - periods = 0 
        let input_rate = 0.034;
        let periods = 0.0;
        let _should_panic = convert_ear_to_apr_f64(input_rate, periods);
    }

    #[should_panic]
    #[test]
    fn test_ear_to_apr_f64_13() {
        // - periods < 0 
        let input_rate = 0.034;
        let periods = -3;
        let _should_panic = convert_ear_to_apr_f64(input_rate, periods);
    }

    #[should_panic]
    #[test]
    fn test_ear_to_apr_f64_14() {
        // - periods = infinity 
        let input_rate = 0.034;
        let periods = 1_f64 / 0_f64;
        let _should_panic = convert_ear_to_apr_f64(input_rate, periods);
    }

    #[test]
    fn test_ear_to_apr_f64_15() {
        // periods as u8
        let input_rate = 0.03298851181;
        let periods = 12_u8;
        let expected_output = 0.0325;
        let actual_output = convert_ear_to_apr_f64(input_rate, periods);
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_ear_to_apr_f64_16() {
        // periods as i8
        let input_rate = 0.03298851181;
        let periods = 12_i8;
        let expected_output = 0.0325;
        let actual_output = convert_ear_to_apr_f64(input_rate, periods);
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_ear_to_apr_f64_17() {
        // periods as f32
        let input_rate = 0.03298851181;
        let periods = 12_f32;
        let expected_output = 0.0325;
        let actual_output = convert_ear_to_apr_f64(input_rate, periods);
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    /* 
    ****************************************
    ******* apr to ear Solution type *******
    ****************************************
    */

    #[test]
    fn test_ear_to_apr_1() {
        // normal case
        let input_rate = 0.03298851181;
        let periods = 12;
        let expected_output = 0.0325;
        let actual_output = convert_ear_to_apr(input_rate, periods).annual_percentage_rate;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }
    
    #[test]
    fn test_ear_to_apr_2() {
        // -1 < rate < 0
        let input_rate = -0.03298851181;
        let periods = 12;
        let expected_output = -0.0325;
        let actual_output = convert_ear_to_apr(input_rate, periods).annual_percentage_rate;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_ear_to_apr_3() {
        // rate over 1.0 
        let input_rate = 2.562008918;
        let periods = 12;
        let expected_output = 1.34;
        let actual_output = convert_ear_to_apr(input_rate, periods).annual_percentage_rate;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_ear_to_apr_4() {
        // rate = 1.0 
        let input_rate = 1.0;
        let periods = 12;
        let expected_output = 0.7135571323;
        let actual_output = convert_ear_to_apr(input_rate, periods).annual_percentage_rate;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_ear_to_apr_5() {
        // rate = 0.0 
        let input_rate = 0.0;
        let periods = 12;
        let expected_output = 0.0;
        let actual_output = convert_ear_to_apr(input_rate, periods).annual_percentage_rate;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[should_panic]
    #[test]
    fn test_ear_to_apr_6() {
        // rate < -1.0
        let input_rate = -1.1;
        let periods = 12;
        let _should_panic = convert_ear_to_apr(input_rate, periods).annual_percentage_rate;
    }

    #[should_panic]
    #[test]
    fn test_ear_to_apr_7() {
        // rate = -1.0
        let input_rate = -1.0;
        let periods = 12;
        let actual_output = convert_ear_to_apr(input_rate, periods);
    }

    #[should_panic]
    #[test]
    fn test_ear_to_apr_8() {
        // rate = infinity
        let input_rate = 1_f64/0_f64;
        let periods = 12;
        let _should_panic = convert_ear_to_apr(input_rate, periods);
    }

    #[test]
    fn test_ear_to_apr_9() {
        // periods = 1
        let input_rate = 0.034;
        let periods = 1;
        let expected_output = 0.034;
        let actual_output = convert_ear_to_apr(input_rate, periods).annual_percentage_rate;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_ear_to_apr_10() {
        // - periods as float, > 1 (float)
        let input_rate = 0.03456823374;
        let periods = 36.5;
        let expected_output = 0.034;
        let actual_output = convert_ear_to_apr(input_rate, periods).annual_percentage_rate;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[should_panic]
    #[test]
    fn test_ear_to_apr_11() {
        // - periods as fractional, 0 < n < 1 
        let input_rate = 0.034;
        let periods = 0.5;
        let _should_panic = convert_ear_to_apr(input_rate, periods).annual_percentage_rate;
    }

    #[should_panic]
    #[test]
    fn test_ear_to_apr_12() {
        // - periods = 0 
        let input_rate = 0.034;
        let periods = 0.0;
        let _should_panic = convert_ear_to_apr(input_rate, periods).annual_percentage_rate;
    }

    #[should_panic]
    #[test]
    fn test_ear_to_apr_13() {
        // - periods < 0 
        let input_rate = 0.034;
        let periods = -3;
        let _should_panic = convert_ear_to_apr(input_rate, periods);
    }

    #[should_panic]
    #[test]
    fn test_ear_to_apr_14() {
        // - periods = infinity 
        let input_rate = 0.034;
        let periods = 1_f64 / 0_f64;
        let _should_panic = convert_ear_to_apr(input_rate, periods).annual_percentage_rate;
    }

    #[test]
    fn test_ear_to_apr_15() {
        // periods as u8
        let input_rate = 0.03298851181;
        let periods = 12_u8;
        let expected_output = 0.0325;
        let actual_output = convert_ear_to_apr(input_rate, periods).annual_percentage_rate;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_ear_to_apr_16() {
        // periods as i8
        let input_rate = 0.03298851181;
        let periods = 12_i8;
        let expected_output = 0.0325;
        let actual_output = convert_ear_to_apr(input_rate, periods).annual_percentage_rate;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_ear_to_apr_17() {
        // periods as f32
        let input_rate = 0.03298851181;
        let periods = 12_f32;
        let expected_output = 0.0325;
        let actual_output = convert_ear_to_apr(input_rate, periods).annual_percentage_rate;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }



    /* 
    ************************************************
    ************* apr to ear, continous compound ***
    ************************************************
    */

    #[test]
    fn test_apr_to_ear_continuous_compound_f64_1() {
        // normal case
        let input_rate = 0.034;
        let expected_output = 0.4049475906;
        let actual_output = convert_apr_to_ear_continuous_compound_f64(input_rate);
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }
    
    #[test]
    fn test_apr_to_ear_continuous_compound_f64_2() {
        // -1 < rate < 0
        let input_rate = -0.034;
        let expected_output = -0.4049475906;
        let actual_output = convert_apr_to_ear_continuous_compound_f64(input_rate);
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_apr_to_ear_continuous_compound_f64_3() {
        // rate over 1.0 
        let input_rate = 2.562008918;
        let expected_output = 11.96183043;
        let actual_output = convert_apr_to_ear_continuous_compound_f64(input_rate);
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_apr_to_ear_continuous_compound_f64_4() {
        // rate = 1.0 
        let input_rate = 1.0;
        let expected_output = 1.718281828;
        let actual_output = convert_apr_to_ear_continuous_compound_f64(input_rate);
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_apr_to_ear_continuous_compound_f64_5() {
        // rate = 0.0 
        let input_rate = 0.0;
        let expected_output = 0.0;
        let actual_output = convert_apr_to_ear_continuous_compound_f64(input_rate);
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_apr_to_ear_continuous_compound_f64_6() {
        // rate < -1.0
        let input_rate = -1.1;
        let expected_output = -2.004166024;
        let actual_output = convert_apr_to_ear_continuous_compound_f64(input_rate);
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_apr_to_ear_continuous_compound_f64_7() {
        // rate = -1.0
        let input_rate = -1.0;
        let expected_output = -1.718281828;
        let actual_output = convert_apr_to_ear_continuous_compound_f64(input_rate);
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[should_panic]
    #[test]
    fn test_apr_to_ear_continuous_compound_f64_8() {
        // rate = infinity
        let input_rate = 1_f64/0_f64;
        let _should_panic = convert_apr_to_ear_continuous_compound_f64(input_rate);
    }


    /* 
    ************************************************
    ************* apr to ear, continous compound ***
    ****************** solution type ***************
    ************************************************
    */

    #[test]
    fn test_apr_to_ear_continuous_compound_1() {
        // normal case
        let input_rate = 0.034;
        let expected_output = 0.4049475906;
        let actual_output = convert_apr_to_ear_continuous_compound(input_rate).ear;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }
    
    #[test]
    fn test_apr_to_ear_continuous_compound_2() {
        // -1 < rate < 0
        let input_rate = -0.034;
        let expected_output = -0.4049475906;
        let actual_output = convert_apr_to_ear_continuous_compound(input_rate).ear;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_apr_to_ear_continuous_compound_3() {
        // rate over 1.0 
        let input_rate = 2.562008918;
        let expected_output = 11.96183043;
        let actual_output = convert_apr_to_ear_continuous_compound(input_rate).ear;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_apr_to_ear_continuous_compound_4() {
        // rate = 1.0 
        let input_rate = 1.0;
        let expected_output = 1.718281828;
        let actual_output = convert_apr_to_ear_continuous_compound(input_rate).ear;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_apr_to_ear_continuous_compound_5() {
        // rate = 0.0 
        let input_rate = 0.0;
        let expected_output = 0.0;
        let actual_output = convert_apr_to_ear_continuous_compound(input_rate).ear;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }


    #[test]
    fn test_apr_to_ear_continuous_compound_6() {
        // rate < -1.0
        let input_rate = -1.1;
        let expected_output = -2.004166024;
        let actual_output = convert_apr_to_ear_continuous_compound(input_rate).ear;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_apr_to_ear_continuous_compound_7() {
        // rate = -1.0
        let input_rate = -1.0;
        let expected_output = -1.718281828;
        let actual_output = convert_apr_to_ear_continuous_compound(input_rate).ear;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[should_panic]
    #[test]
    fn test_apr_to_ear_continuous_compound_8() {
        // rate = infinity
        let input_rate = 1_f64/0_f64;
        let _should_panic = convert_apr_to_ear_continuous_compound(input_rate).ear;
    }

    /* 
    ****************************************
    ******* periodic to ear Solution type **
    ****************************************
    */

    #[test]
    fn test_periodic_to_ear_solution_1() {
        // normal case
        let input_rate = 0.0325;
        let periods = 12;
        let expected_output = 0.4678467782;
        let actual_output = convert_periodic_to_ear_solution(input_rate, periods).effective_annual_rate;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }
    
    #[test]
    fn test_periodic_to_ear_solution_2() {
        // -1 < rate < 0
        let input_rate = -0.0325;
        let periods = 12;
        let expected_output = -0.4678467782;
        let actual_output = convert_periodic_to_ear_solution(input_rate, periods).effective_annual_rate;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[should_panic]
    #[test]
    fn test_periodic_to_ear_solution_3() {
        // rate over 1.0 
        let input_rate = 2.04;
        let periods = 12;
        let expected_output = 622991.76529749500;
        let actual_output = convert_periodic_to_ear_solution(input_rate, periods).effective_annual_rate;
        assert_eq!(round_to_ulps_4(expected_output), round_to_ulps_4(actual_output));
    }

    #[test]
    fn test_periodic_to_ear_solution_4() {
        // rate = 1.0 
        let input_rate = 1.0;
        let periods = 12;
        let expected_output = 4095.00;
        let actual_output = convert_periodic_to_ear_solution(input_rate, periods).effective_annual_rate;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_periodic_to_ear_solution_5() {
        // rate = 0.0 
        let input_rate = 0.0;
        let periods = 12;
        let expected_output = 0.0;
        let actual_output = convert_periodic_to_ear_solution(input_rate, periods).effective_annual_rate;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_periodic_to_ear_solution_6() {
        // rate < -1.0
        let input_rate = -1.1;
        let periods = 12;
        let expected_output = -7354.827511386650;
        let actual_output = convert_periodic_to_ear_solution(input_rate, periods).effective_annual_rate;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_periodic_to_ear_solution_7() {
        // rate < -1.0
        let input_rate = -1.0;
        let periods = 12;
        let expected_output = -4095.00;
        let actual_output = convert_periodic_to_ear_solution(input_rate, periods).effective_annual_rate;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[should_panic]
    #[test]
    fn test_periodic_to_ear_solution_8() {
        // rate = infinity
        let input_rate = 1_f64/0_f64;
        let periods = 12;
        let _should_panic = convert_periodic_to_ear_solution(input_rate, periods);
    }

    #[test]
    fn test_periodic_to_ear_solution_9() {
        // periods = 1
        let input_rate = 0.034;
        let periods = 1;
        let expected_output = 0.034;
        let actual_output = convert_periodic_to_ear_solution(input_rate, periods).effective_annual_rate;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

    #[test]
    fn test_periodic_to_ear_solution_10() {
        // periods = 0
        let input_rate = 0.034;
        let periods = 0;
        let expected_output = 0.00;
        let actual_output = convert_periodic_to_ear_solution(input_rate, periods).effective_annual_rate;
        assert_eq!(round_to_ulps_6(expected_output), round_to_ulps_6(actual_output));
    }

// test cases 
// - normal
// - rate over 1.0 
// - rate = 1.0 
// - rate < 0.0
// - rate = 0.0 
// - rate < -1.0 
// - rate = -1.0 
// - rate = infinity
// - periods = 1
// - periods as float, > 1 (float) 
// - periods between 0 and 1 (fractional)
// - periods = 0 
// - periods < 0 
// - periods = infinity
// - periods as different types (float, int)

    
}