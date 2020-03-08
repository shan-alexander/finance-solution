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

    // expect 1.2578%
    let apr = 0.0125;
    let apr_to_ear = convert_apr_to_ear_continuous_compound(apr);
    dbg!(apr_to_ear);
}

fn try_convert_periodic_to_ear() {
    // https://www.investopedia.com/articles/investing/121713/interest-rates-apr-apy-and-ear.asp
    // expect 25.72% EAR (also known as APY)
    let apr: f64 = 0.228964;
    let num_compounding_periods: f64 = 365_f64; // daily compounding
    let periodic_rate = apr / num_compounding_periods;
    let ear_solution = convert_periodic_to_ear(periodic_rate, num_compounding_periods);
    dbg!(ear_solution);
}

fn try_find_apr() {
    let interest_rate: f64 = 0.0475;
    let periods_in_year: u16 = 12;
    let num_total_periods: u16 = 360;
    let present_value = 100_000_f64; // gross principal borrowed. present value?
    let future_value = 0_f64;
    let payment_amount: f64 = -521.65;

    // after calculating... will find that monthly payment is $521.65
    // and APR is 5.02%
    // https://www.investopedia.com/articles/investing/121713/interest-rates-apr-apy-and-ear.asp

    let find_apr_solution = find_apr(num_total_periods, payment_amount, present_value, 0_f64);
}

pub fn convert_apr_to_periodic(apr: f64, num_periods_in_year: f64) -> f64 {
    apr / num_periods_in_year
}

/// Convert a nominal interest rate (Annual rate) to EAR (effective annual rate).
pub fn convert_apr_to_ear(apr: f64, compounding_periods_in_year: f64) -> f64 {
    (1_f64 + (apr/compounding_periods_in_year)).powi(compounding_periods_in_year as i32) - 1_f64
}

/// Convert a nominal interest rate (Annual rate) to EAR (effective annual rate) using continuous compounding.
pub fn convert_apr_to_ear_continuous_compound(apr: f64) -> f64 {
    // formula: e^apr - 1
    let e: f64 = 2.7182818284;
    e.powf(apr) - 1_f64
}

/// Convert a periodic interest rate (APR / num of compounding periods) to EAR (effective annual rate).
pub fn convert_periodic_to_ear(periodic_rate: f64, compounding_periods_in_year: f64) -> f64 {
    (1_f64 + periodic_rate).powi(compounding_periods_in_year as i32) - 1_f64
}

pub fn convert_ear_to_periodic(ear: f64, periods_per_year: u8) -> f64 {
    // EPR = (1 + annual rate)^(1/#ofPeriodsPerYear) 
    (1. + ear).powf(1./periods_per_year as f64) - 1.
}

pub struct FindAprSolution {
    num_periods: u16,
    payment_amount: f64,
    present_value: f64,
    future_value: f64,
    apr: f64,
}
impl FindAprSolution {
    pub fn new(num_periods: u16, payment_amount: f64, present_value: f64, future_value: f64, apr: f64) -> Self {
        Self {
            num_periods,
            payment_amount,
            present_value,
            future_value,
            apr,
        }
    }
}
// still unsolved...
pub fn find_apr(num_periods: u16, payment_amount: f64, present_value: f64, future_value: f64) -> FindAprSolution {
    // in Excel, the formula is RATE (nper, pmt, pv, fv, type, guess)
    // https://www.investopedia.com/articles/investing/121713/interest-rates-apr-apy-and-ear.asp
    
    let apr = 0.005813_f64;
    FindAprSolution::new(num_periods, payment_amount, present_value, future_value, apr) 
}
