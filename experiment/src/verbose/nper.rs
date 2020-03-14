#![allow(dead_code)]
#![allow(unused_imports)]

use float_cmp::ApproxEq;
use log::Level;
use log::{info, warn, log_enabled};
use libm::log10;

// https://math.stackexchange.com/questions/820094/what-is-the-best-way-to-calculate-log-without-a-calculator

pub fn main() { 
    try_nper();
}

fn try_nper() {
    // https://www.techonthenet.com/excel/formulas/nper.php
    // This example returns the number of monthly payments (monthly payments are $150) for a $5,000 investment that earns 7.5% annually. Payments are due at the end of the period.
    // expect 37.495062 periods
    let apr = 0.075_f64;
    let periodic_rate = apr / 12.;
    let payment_amount = -150_f64;
    let present_value_total = 5_000_f64;
    let future_value_total =0_f64;
    let nper_solution = nper(periodic_rate, payment_amount, present_value_total, future_value_total);
    dbg!(nper_solution);

    // This example returns the number of annual payments (annual payments are $200) for a $1,500 investment that earns 5.25% annually. Payments are due at the end of each year.
    // expect 9.780722988
    let apr = 0.0525_f64;
    let periodic_rate = apr / 1.;
    let payment_amount = -200_f64;
    let present_value_total = 1_500_f64;
    let future_value_total =0_f64;
    let nper_solution = nper(periodic_rate, payment_amount, present_value_total, future_value_total);
    dbg!(nper_solution);

    // expect 55.903044 periods
    let apr = 0.075_f64;
    let periodic_rate = apr / 12.;
    let payment_amount = -150_f64;
    let present_value_total = 0_f64;
    let future_value_total = 10_000_f64;
    let nper_solution = nper(periodic_rate, payment_amount, present_value_total, future_value_total);
    dbg!(nper_solution);
}

#[derive(Debug)]
pub struct NperSolution {
    pub periodic_rate: f64,
    pub periods: f64,
    pub payment_amount: f64,
    pub present_value_total: f64,
    pub future_value_total: f64,
    
}
impl NperSolution {
    pub fn new(periodic_rate: f64, periods: f64, payment_amount: f64, present_value_total: f64, future_value_total: f64) -> Self {
        Self {
            periodic_rate,
            periods,
            payment_amount,
            present_value_total,
            future_value_total,
        }
    }
}

//To calculate the number of periods needed for an annuity to reach a given future value, you can use the NPER function.
pub fn nper<C: Into<f64> + Copy, P: Into<f64> + Copy, F: Into<f64> + Copy>(periodic_rate: f64, payment_amount: C, present_value: P, future_value: F) -> NperSolution {
    let pmt = payment_amount.into();
    let pv = present_value.into();
    let fv = future_value.into();
    assert!(pmt < 0_f64); // payment must be negative, same as Excel.

    // LN((pmt - fv*r_)/(pmt + pv*r_))/LN(1 + r_)
    // =Log10(Payment/(Payment+Capital+Rate))/Log10(1+Rate)
    let numerator = libm::log10( (pmt - fv * periodic_rate) / (pmt +  pv * periodic_rate) );
    let num_periods = numerator / libm::log10(1. + periodic_rate); 
    NperSolution::new(periodic_rate, num_periods, pmt, pv, fv)
}