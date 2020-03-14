#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use float_cmp::ApproxEq;
use log::Level;
use log::{info, warn, log_enabled};

pub fn main() { 
    try_convert_apr_to_periodic();
    try_convert_apr_to_ear();
    try_convert_periodic_to_ear();

}

// note: APR is also called "standard interest rate" or just "interest rate"
// EAR and APY are the same thing. EAR is more polite than APY because the "Y" for yield reminds people that someone is earning a yield from the deal
// To remember simply, a monthly periodic rate is the APR/12.

fn try_convert_apr_to_periodic() {
    // expect 0.0045833333 repeating
    let apr = 0.055_f64;
    let num_periods = 12_f64; // monthly periods
    let periodic_rate = convert_apr_to_periodic(apr, num_periods);
    dbg!(periodic_rate);

    // expect 0.001057692307692307
    let apr = 0.055_f64;
    let num_periods = 52_f64; // weekly periods
    let periodic_rate = convert_apr_to_periodic(apr, num_periods);
    dbg!(periodic_rate);   
    
    // expect 0.001057692307692307
    // get warn
    let apr = 5.5_f64;
    let num_periods = 52_f64; // weekly periods
    let periodic_rate = convert_apr_to_periodic(apr, num_periods);
    dbg!(periodic_rate);  
}

// calculator to check EAR conversions
// https://www.calculatorsoup.com/calculators/financial/effective-annual-rate-calculator.php

fn try_convert_apr_to_ear() {
    // expect 3.2989%
    let apr = 0.0325;
    let compounding_periods_in_year = 12_f64; // monthly
    let apr_to_ear = convert_apr_to_ear(apr, compounding_periods_in_year);
    dbg!(apr_to_ear);

    // expect 0.2503%
    let apr = 0.0025;
    let compounding_periods_in_year = 365_f64; // daily compounding
    let apr_to_ear = convert_apr_to_ear(apr, compounding_periods_in_year);
    dbg!(apr_to_ear);

    // get warns
    let apr = 1.5;
    let compounding_periods_in_year = 730; // twice daily? maybe in forex...
    let apr_to_ear = convert_apr_to_ear(apr, compounding_periods_in_year);
    dbg!(apr_to_ear);

    // expect 1.2578%
    let apr = 0.0125;
    let apr_to_ear = convert_apr_to_ear_continuous_compound(apr);
    dbg!(apr_to_ear);
    
}

fn try_convert_periodic_to_ear() {
    // https://www.investopedia.com/articles/investing/121713/interest-rates-apr-apy-and-ear.asp
    // expect 25.72% EAR (also known as APY)
    let apr: f64 = 0.228964;
    let num_compounding_periods: u16 = 365; // daily compounding
    let periodic_rate = apr / num_compounding_periods as f64;
    let ear_solution = convert_periodic_to_ear(periodic_rate, num_compounding_periods);
    dbg!(ear_solution);
}



/// Convert APR (annual rate) to periodic rate.
pub fn convert_apr_to_periodic<T: Into<f64> + Copy>(apr: f64, num_periods_in_year: T) -> f64 {
    let n = num_periods_in_year.into();
    assert!(n >= 1.);
    if apr > 1. || apr < -1. {
        warn!("You provided an APR of {}%. Are you sure?", apr*100.);
    }
    apr / n
}

/// Convert periodic rate to APR (aka Annual rate, nominal interest rate, Annual Percentage Rate).
pub fn convert_periodic_to_apr<T: Into<f64> + Copy>(periodic_rate: f64, periods_per_year: T) -> f64 {
    let n = periods_per_year.into();
    if periodic_rate > 1. || periodic_rate < -1. {
        warn!("You provided a periodic rate of {}%. Are you sure?", periodic_rate*100.);
    }
    periodic_rate * n
}

/// Convert a nominal interest rate (Annual rate, APR) to EAR (effective annual rate).
pub fn convert_apr_to_ear<T: Into<f64> + Copy>(apr: f64, compounding_periods_in_year: T) -> f64 {
    let n = compounding_periods_in_year.into();
    assert!(n >= 1.);
    if apr > 1. || apr < -1. {
        warn!("You provided an APR of {}%. Are you sure?", apr*100.);
    }
    if n > 366. {
        warn!("You provided more than 366 compounding periods in a year (You provided {}). Are you sure?", n);
    }
    if n == 1. { return apr }
    (1_f64 + (apr/n)).powf(n) - 1_f64
}

/// Convert a nominal interest rate (Annual rate) to EAR (effective annual rate) using continuous compounding.
pub fn convert_apr_to_ear_continuous_compound(apr: f64) -> f64 {
    if apr > 1. || apr < -1. {
        warn!("You provided an APR of {}%. Are you sure?", apr*100.);
    }
    // formula: e^apr - 1
    let e: f64 = 2.71828182845904; 
    e.powf(apr) - 1_f64
}

/// Convert an EAR (effective annual rate) to a nominal interest rate (Annual rate, APR). This involves converting the EAR to periodic rate first, and then APR = periodic rate * number of periods.
pub fn convert_ear_to_apr<T: Into<f64> + Copy>(ear: f64, compounding_periods_in_year: T) -> f64 {
    let n = compounding_periods_in_year.into();
    assert!(n >= 1.);
    if ear > 1. || ear < -1. {
        warn!("You provided an EAR of {}%. Are you sure?", ear*100.);
    }
    if n > 366. {
        warn!("You provided more than 366 compounding periods in a year (You provided {}). Are you sure?", n);
    }
    if n == 1. { return ear }
    // APR is the periodic rate * number of periods, so convert EAR to periodic, then * n
    convert_ear_to_periodic(ear, n) * n
}

/// Convert a periodic interest rate (APR / num of compounding periods) to EAR (effective annual rate).
pub fn convert_periodic_to_ear<T: Into<f64> + Copy>(periodic_rate: f64, compounding_periods_in_year: T) -> f64 {
    let n = compounding_periods_in_year.into();
    assert!(n > 0.);
    if periodic_rate > 1. || periodic_rate < -1. {
        warn!("You provided an periodic rate of {}%. Are you sure?", periodic_rate*100.);
    }
    if n == 1. { return periodic_rate }
    (1_f64 + periodic_rate).powf(n) - 1_f64
}

/// Convert an EAR (Effective Annual Rate) to periodic rate.
pub fn convert_ear_to_periodic<T: Into<f64> + Copy>(ear: f64, periods_per_year: T) -> f64 {
    let n = periods_per_year.into();
    assert!(n > 0.);
    if ear > 1. || ear < -1. {
        warn!("You provided an Effective Annual Rate of {}%. Are you sure?", ear*100.);
    }
    if n == 1. { return ear }
    // EPR = (1 + annual rate)^(1/#ofPeriodsPerYear) 
    (1_f64 + ear).powf(1_f64/n) - 1_f64
}


#[cfg(test)]
mod tests {
    use super::*;

    #[should_panic]
    pub fn test_negative_period_convert_apr_to_periodic() {
        convert_apr_to_periodic(0.03, -3.4);
    }
    #[should_panic]
    pub fn test_fractional_period() {
        convert_apr_to_periodic(0.03, 0.4);
    }
}