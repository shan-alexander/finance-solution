#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use float_cmp::ApproxEq;
use log::Level;
use log::{warn}; // consider how to use: log_enabled   also, error (runtime?) vs assert (runtime)

pub fn main() { 
    try_convert_apr_to_periodic();
    try_convert_apr_to_periodic_f64();
    try_convert_apr_to_ear_f64();
    try_convert_periodic_to_ear_f64();

}

// note: APR is also called "standard interest rate" or just "interest rate"
// EAR and APY are the same thing. EAR is more polite than APY because the "Y" for yield reminds people that someone is earning a yield from the deal
// To remember simply, a monthly periodic rate is the APR/12.

fn try_convert_apr_to_periodic() {
    // expect 0.0045833333 repeating
    let apr = 0.055_f64;
    let num_periods = 12_f64; // monthly periods
    dbg!(convert_apr_to_periodic(apr, num_periods));
}

fn try_convert_apr_to_periodic_f64() {
    // expect 0.0045833333 repeating
    let apr = 0.055_f64;
    let num_periods = 12_f64; // monthly periods
    let periodic_rate = convert_apr_to_periodic_f64(apr, num_periods);
    dbg!(periodic_rate);

    // expect 0.001057692307692307
    let apr = 0.055_f64;
    let num_periods = 52_f64; // weekly periods
    let periodic_rate = convert_apr_to_periodic_f64(apr, num_periods);
    dbg!(periodic_rate);   
    
    // expect 0.001057692307692307
    // get warn
    let apr = 5.5_f64;
    let num_periods = 52_f64; // weekly periods
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
    let num_compounding_periods: u16 = 365; // daily compounding
    let periodic_rate = apr / num_compounding_periods as f64;
    let ear_solution = convert_periodic_to_ear_f64(periodic_rate, num_compounding_periods);
    dbg!(ear_solution);
}

#[derive(Debug)]
pub struct AprEarSolution {
    annual_percentage_rate: f64,
    num_periods_in_year: f64,
    effective_annual_rate: f64,
    input_in_percent: String,
    output_in_percent: String,
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
    annual_percentage_rate: f64,
    num_periods_in_year: f64,
    periodic_rate: f64,
    input_in_percent: String,
    output_in_percent: String,
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
pub fn convert_apr_to_periodic<T: Into<f64> + Copy>(apr: f64, num_periods_in_year: T) -> AprPeriodicSolution {
    let n = num_periods_in_year.into();
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
pub fn convert_apr_to_periodic_f64<T: Into<f64> + Copy>(apr: f64, num_periods_in_year: T) -> f64 {
    let n = num_periods_in_year.into();
    assert!(n >= 1.);
    assert!(n.is_finite());
    assert!(apr.is_finite());
    if apr > 1. || apr < -1. {
        warn!("You provided an APR of {}%. Are you sure?", apr*100.);
    }
    apr / n
}

/// Convert periodic rate to APR (aka Annual rate, nominal interest rate, Annual Percentage Rate). Returns a custom solution type.
pub fn convert_periodic_to_apr<T: Into<f64> + Copy>(periodic_rate: f64, periods_per_year: T) -> AprPeriodicSolution {
    let n = periods_per_year.into();
    assert!(n >= 1.);
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
    if n > 366. {
        warn!("You provided more than 366 compounding periods in a year (You provided {}). Are you sure?", n);
    }
    let ear: f64;
    if n == 1. { 
        ear = apr;
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
    assert!(n >= 1.);
    assert!(n.is_finite());
    assert!(ear.is_finite());
    if ear > 1. || ear < -1. {
        warn!("You provided an EAR of {}%. Are you sure?", ear*100.);
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
    assert!(n >= 1.);
    assert!(n.is_finite());
    assert!(ear.is_finite());
    if ear > 1. || ear < -1. {
        warn!("You provided an EAR of {}%. Are you sure?", ear*100.);
    }
    if n > 366. {
        warn!("You provided more than 366 compounding periods in a year (You provided {}). Are you sure?", n);
    }
    if n == 1. { return ear }
    // APR is the periodic rate * number of periods, so convert EAR to periodic, then * n
    // convert_ear_to_periodic_f64(ear, n) * n
    ((1_f64 + ear).powf(1_f64/n) - 1_f64) * n
}

/// Convert a nominal interest rate (Annual rate) to EAR (effective annual rate) using continuous compounding.
pub fn convert_apr_to_ear_continuous_compound_f64(apr: f64) -> f64 {
    if apr > 1. || apr < -1. {
        warn!("You provided an APR of {}%. Are you sure?", apr*100.);
    }
    // formula: e^apr - 1
    let e: f64 = 2.71828182845904; 
    e.powf(apr) - 1_f64
}



/// Convert a periodic interest rate (APR / num of compounding periods) to EAR (effective annual rate).
pub fn convert_periodic_to_ear_f64<T: Into<f64> + Copy>(periodic_rate: f64, compounding_periods_in_year: T) -> f64 {
    let n = compounding_periods_in_year.into();
    assert!(n > 0.);
    if periodic_rate > 1. || periodic_rate < -1. {
        warn!("You provided an periodic rate of {}%. Are you sure?", periodic_rate*100.);
    }
    if n == 1. { return periodic_rate }
    (1_f64 + periodic_rate).powf(n) - 1_f64
}

/// Convert an EAR (Effective Annual Rate) to periodic rate.
pub fn convert_ear_to_periodic_f64<T: Into<f64> + Copy>(ear: f64, periods_per_year: T) -> f64 {
    let n = periods_per_year.into();
    assert!(n > 0.);
    if ear > 1. || ear < -1. {
        warn!("You provided an Effective Annual Rate of {}%. Are you sure?", ear*100.);
    }
    if n == 1. { return ear }
    // EPR = (1 + annual rate)^(1/#ofPeriodsPerYear) 
    (1_f64 + ear).powf(1_f64/n) - 1_f64
}

pub fn round_to_ulps_8(val: f64) -> f64 {
    (val * 100_000_000.0 / 100_000_000.0).floor()
}
pub fn round_to_ulps_4(val: f64) -> f64 {
    (val * 10_000.0 / 10_000.0).floor()
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
        assert_eq!(round_to_ulps_8(expected_output), round_to_ulps_8(actual_output));
    }

    #[test]
    fn test_apr_to_periodic_f64_4() {
        // big rate
        let input_rate = 0.99;
        let periods = 13;
        let expected_output = 0.0761538462;
        let actual_output = convert_apr_to_periodic_f64(input_rate, periods);
        assert_eq!(round_to_ulps_8(expected_output), round_to_ulps_8(actual_output));
    }

    #[test]
    fn test_apr_to_periodic_f64_5() {
        //  rate of 100%
        let input_rate = 1.00;
        let periods = 13;
        let expected_output = 0.0769230769;
        let actual_output = convert_apr_to_periodic_f64(input_rate, periods);
        assert_eq!(round_to_ulps_8(expected_output), round_to_ulps_8(actual_output));
    }

    #[test]
    fn test_apr_to_periodic_f64_6() {
        //  rate over 1.0
        let input_rate = 3.34;
        let periods = 13;
        let expected_output = 0.2569230769;
        let actual_output = convert_apr_to_periodic_f64(input_rate, periods);
        assert_eq!(round_to_ulps_8(expected_output), round_to_ulps_8(actual_output));
    }

    #[test]
    fn test_apr_to_periodic_f64_7() {
        //  rate is -1.0
        let input_rate = -1.0;
        let periods = 13;
        let expected_output = -0.0769230769;
        let actual_output = convert_apr_to_periodic_f64(input_rate, periods);
        assert_eq!(round_to_ulps_8(expected_output), round_to_ulps_8(actual_output));
    }

    #[test]
    fn test_apr_to_periodic_f64_8() {
        //  rate below -1.0
        let input_rate = -2.3334;
        let periods = 13;
        let expected_output = -0.1794923077;
        let actual_output = convert_apr_to_periodic_f64(input_rate, periods);
        assert_eq!(round_to_ulps_8(expected_output), round_to_ulps_8(actual_output));
    }

    #[test]
    fn test_apr_to_periodic_f64_9() {
        //  fractional period
        let input_rate = -2.3334;
        let periods = 13.9;
        let expected_output = -0.1678705036;
        let actual_output = convert_apr_to_periodic_f64(input_rate, periods);
        assert_eq!(round_to_ulps_8(expected_output), round_to_ulps_8(actual_output));
    }

    #[should_panic]
    #[test]
    fn test_apr_to_periodic_f64_10() {
        //  negative period
        let input_rate = -2.3334;
        let periods = -5;
        let expected_output = -0.0;
        let actual_output = convert_apr_to_periodic_f64(input_rate, periods);
        assert_eq!(round_to_ulps_8(expected_output), round_to_ulps_8(actual_output));
    }

    #[should_panic]
    #[test]
    fn test_apr_to_periodic_f64_11() {
        //  period less than 1, greater than 0
        let input_rate = -2.3334;
        let periods = 0.9;
        let expected_output = -0.0;
        let actual_output = convert_apr_to_periodic_f64(input_rate, periods);
        assert_eq!(round_to_ulps_8(expected_output), round_to_ulps_8(actual_output));
    }

    #[test]
    fn test_apr_to_periodic_f64_12() {
        //  period = 1
        let input_rate = 0.34;
        let periods = 1;
        let expected_output = 0.34;
        let actual_output = convert_apr_to_periodic_f64(input_rate, periods);
        assert_eq!(round_to_ulps_8(expected_output), round_to_ulps_8(actual_output));
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

    #[should_panic]
    #[test]
    fn test_apr_to_periodic_f64_14() {
        //  infinite periods
        let input_rate = 0.3;
        let periods = 1_f64 / 0_f64;
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
        assert_eq!(round_to_ulps_8(expected_output), round_to_ulps_8(actual_output));
    }

    #[test]
    fn test_apr_to_periodic_4() {
        // big rate
        let input_rate = 0.99;
        let periods = 13;
        let expected_output = 0.0761538462;
        let actual_output = convert_apr_to_periodic(input_rate, periods).periodic_rate;
        assert_eq!(round_to_ulps_8(expected_output), round_to_ulps_8(actual_output));
    }

    #[test]
    fn test_apr_to_periodic_5() {
        //  rate of 100%
        let input_rate = 1.00;
        let periods = 13;
        let expected_output = 0.0769230769;
        let actual_output = convert_apr_to_periodic(input_rate, periods).periodic_rate;
        assert_eq!(round_to_ulps_8(expected_output), round_to_ulps_8(actual_output));
    }

    #[test]
    fn test_apr_to_periodic_6() {
        //  rate over 1.0
        let input_rate = 3.34;
        let periods = 13;
        let expected_output = 0.2569230769;
        let actual_output = convert_apr_to_periodic(input_rate, periods).periodic_rate;
        assert_eq!(round_to_ulps_8(expected_output), round_to_ulps_8(actual_output));
    }

    #[test]
    fn test_apr_to_periodic_7() {
        //  rate is -1.0
        let input_rate = -1.0;
        let periods = 13;
        let expected_output = -0.0769230769;
        let actual_output = convert_apr_to_periodic(input_rate, periods).periodic_rate;
        assert_eq!(round_to_ulps_8(expected_output), round_to_ulps_8(actual_output));
    }

    #[test]
    fn test_apr_to_periodic_8() {
        //  rate below -1.0
        let input_rate = -2.3334;
        let periods = 13;
        let expected_output = -0.1794923077;
        let actual_output = convert_apr_to_periodic(input_rate, periods).periodic_rate;
        assert_eq!(round_to_ulps_8(expected_output), round_to_ulps_8(actual_output));
    }

    #[test]
    fn test_apr_to_periodic_9() {
        //  fractional period
        let input_rate = -2.3334;
        let periods = 13.9;
        let expected_output = -0.1678705036;
        let actual_output = convert_apr_to_periodic(input_rate, periods).periodic_rate;
        assert_eq!(round_to_ulps_8(expected_output), round_to_ulps_8(actual_output));
    }

    #[should_panic]
    #[test]
    fn test_apr_to_periodic_10() {
        //  negative period
        let input_rate = -2.3334;
        let periods = -5;
        let expected_output = -0.0;
        let _should_panic = convert_apr_to_periodic(input_rate, periods);
    }

    #[should_panic]
    #[test]
    fn test_apr_to_periodic_11() {
        //  period less than 1, greater than 0
        let input_rate = -2.3334;
        let periods = 0.9;
        let expected_output = -0.0;
        let _should_panic = convert_apr_to_periodic(input_rate, periods);
    }

    #[test]
    fn test_apr_to_periodic_12() {
        //  period = 1
        let input_rate = 0.34;
        let periods = 1;
        let expected_output = 0.34;
        let actual_output = convert_apr_to_periodic(input_rate, periods).periodic_rate;
        assert_eq!(round_to_ulps_8(expected_output), round_to_ulps_8(actual_output));
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

    #[should_panic]
    #[test]
    fn test_apr_to_periodic_14() {
        //  infinite periods
        let input_rate = 0.3;
        let periods = 1_f64 / 0_f64;
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
        assert_eq!(round_to_ulps_8(expected_output), round_to_ulps_8(actual_output));
    }

    #[test]
    fn test_periodic_to_apr_f64_4() {
        // long float
        let input_rate = 0.0761538462;
        let periods = 13;
        let expected_output = 0.99;
        let actual_output = convert_periodic_to_apr_f64(input_rate, periods);
        assert_eq!(round_to_ulps_8(expected_output), round_to_ulps_8(actual_output));
    }

    #[test]
    fn test_periodic_to_apr_f64_5() {
        //  rate of 100%
        let input_rate = 1.00;
        let periods = 13;
        let expected_output = 13.00;
        let actual_output = convert_periodic_to_apr_f64(input_rate, periods);
        assert_eq!(round_to_ulps_8(expected_output), round_to_ulps_8(actual_output));
    }

    #[test]
    fn test_periodic_to_apr_f64_6() {
        //  rate over 1.0
        let input_rate = 1.1;
        let periods = 13;
        let expected_output = 14.3;
        let actual_output = convert_periodic_to_apr_f64(input_rate, periods);
        assert_eq!(round_to_ulps_8(expected_output), round_to_ulps_8(actual_output));
    }

    #[test]
    fn test_periodic_to_apr_f64_7() {
        //  rate is -1.0
        let input_rate = -1.0;
        let periods = 13;
        let expected_output = -13.0;
        let actual_output = convert_periodic_to_apr_f64(input_rate, periods);
        assert_eq!(round_to_ulps_8(expected_output), round_to_ulps_8(actual_output));
    }

    #[test]
    fn test_periodic_to_apr_f64_8() {
        //  rate below -1.0
        let input_rate = -1.1794;
        let periods = 13;
        let expected_output = -15.3322;
        let actual_output = convert_periodic_to_apr_f64(input_rate, periods);
        assert_eq!(round_to_ulps_8(expected_output), round_to_ulps_8(actual_output));
    }

    #[test]
    fn test_periodic_to_apr_f64_9() {
        //  fractional period
        let input_rate = -0.1678705036;
        let periods = 13.9;
        let expected_output = -2.3334;
        let actual_output = convert_periodic_to_apr_f64(input_rate, periods);
        dbg!(&actual_output);
        assert_eq!(round_to_ulps_8(expected_output), round_to_ulps_8(actual_output));
    }

    #[should_panic]
    #[test]
    fn test_periodic_to_apr_f64_10() {
        //  negative period
        let input_rate = 0.3334;
        let periods = -5;
        let _should_panic = convert_periodic_to_apr_f64(input_rate, periods);
    }

    #[should_panic]
    #[test]
    fn test_periodic_to_apr_f64_11() {
        //  period less than 1, greater than 0
        let input_rate = 0.3334;
        let periods = 0.9;
        let _should_panic = convert_periodic_to_apr_f64(input_rate, periods);
    }

    #[test]
    fn test_periodic_to_apr_f64_12() {
        //  period = 1
        let input_rate = 0.34;
        let periods = 1;
        let expected_output = 0.34;
        let actual_output = convert_periodic_to_apr_f64(input_rate, periods);
        assert_eq!(round_to_ulps_8(expected_output), round_to_ulps_8(actual_output));
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

    #[should_panic]
    #[test]
    fn test_periodic_to_apr_f64_14() {
        //  infinite periods
        let input_rate = 0.3;
        let periods = 1_f64 / 0_f64;
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
        assert_eq!(round_to_ulps_8(expected_output), round_to_ulps_8(actual_output));
    }

    #[test]
    fn test_periodic_to_apr_4() {
        // long float
        let input_rate = 0.0761538462;
        let periods = 13;
        let expected_output = 0.99;
        let actual_output = convert_periodic_to_apr(input_rate, periods).annual_percentage_rate;
        assert_eq!(round_to_ulps_8(expected_output), round_to_ulps_8(actual_output));
    }

    #[test]
    fn test_periodic_to_apr_5() {
        //  rate of 100%
        let input_rate = 1.00;
        let periods = 13;
        let expected_output = 13.00;
        let actual_output = convert_periodic_to_apr(input_rate, periods).annual_percentage_rate;
        assert_eq!(round_to_ulps_8(expected_output), round_to_ulps_8(actual_output));
    }

    #[test]
    fn test_periodic_to_apr_6() {
        //  rate over 1.0
        let input_rate = 1.1;
        let periods = 13;
        let expected_output = 14.3;
        let actual_output = convert_periodic_to_apr(input_rate, periods).annual_percentage_rate;
        assert_eq!(round_to_ulps_8(expected_output), round_to_ulps_8(actual_output));
    }

    #[test]
    fn test_periodic_to_apr_7() {
        //  rate is -1.0
        let input_rate = -1.0;
        let periods = 13;
        let expected_output = -13.0;
        let actual_output = convert_periodic_to_apr(input_rate, periods).annual_percentage_rate;
        assert_eq!(round_to_ulps_8(expected_output), round_to_ulps_8(actual_output));
    }

    #[test]
    fn test_periodic_to_apr_8() {
        //  rate below -1.0
        let input_rate = -1.1794;
        let periods = 13;
        let expected_output = -15.3322;
        let actual_output = convert_periodic_to_apr(input_rate, periods).annual_percentage_rate;
        assert_eq!(round_to_ulps_8(expected_output), round_to_ulps_8(actual_output));
    }

    #[test]
    fn test_periodic_to_apr_9() {
        //  fractional period
        let input_rate = -0.1678705036;
        let periods = 13.9;
        let expected_output = -2.3334;
        let actual_output = convert_periodic_to_apr(input_rate, periods).annual_percentage_rate;
        dbg!(&actual_output);
        assert_eq!(round_to_ulps_8(expected_output), round_to_ulps_8(actual_output));
    }

    #[should_panic]
    #[test]
    fn test_periodic_to_apr_10() {
        //  negative period
        let input_rate = 0.3334;
        let periods = -5;
        let _should_panic = convert_periodic_to_apr(input_rate, periods);
    }

    #[should_panic]
    #[test]
    fn test_periodic_to_apr_11() {
        //  period less than 1, greater than 0
        let input_rate = 0.3334;
        let periods = 0.9;
        let _should_panic = convert_periodic_to_apr(input_rate, periods);
    }

    #[test]
    fn test_periodic_to_apr_12() {
        //  period = 1
        let input_rate = 0.34;
        let periods = 1;
        let expected_output = 0.34;
        let actual_output = convert_periodic_to_apr(input_rate, periods).annual_percentage_rate;
        assert_eq!(round_to_ulps_8(expected_output), round_to_ulps_8(actual_output));
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
    fn test_periodic_to_apr_14() {
        //  infinite periods
        let input_rate = 0.3;
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
        assert_eq!( round_to_ulps_8(expected_output), round_to_ulps_8(actual_output) );
    }
    
    #[test]
    fn test_apr_to_ear_f64_2() {
        // normal case, negative rate
        let input_rate = -0.0325;
        let periods = 12;
        let expected_output = -0.03298851181;
        let actual_output = convert_apr_to_ear_f64(input_rate, periods);
        assert_eq!(round_to_ulps_8(expected_output), round_to_ulps_8(actual_output));
    }


    
}