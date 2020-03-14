#![allow(dead_code)]
#![allow(unused_imports)]

use float_cmp::ApproxEq;
use log::Level;
use log::{info, warn, log_enabled};

pub fn main() {
    try_rate();
}

fn try_rate() {

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