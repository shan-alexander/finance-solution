#![allow(dead_code)]
#![allow(unused_imports)]

use float_cmp::ApproxEq;
use log::Level;
use log::{info, warn, log_enabled};

pub fn main() { 
    try_present_value_annuity();
}

fn try_present_value_annuity() {
    
    // annuity DUE example
    // For example, ABC International is paying a third party $100,000 at the beginning of each year for the next eight years in exchange for the rights to a key patent.  What would it cost ABC if it were to pay the entire amount immediately, assuming an interest rate of 5%?
    // expect $678,637
    // https://www.accountingtools.com/articles/what-is-the-formula-for-the-present-value-of-an-annuity-due.html
    let annuity_payment = 100_000_f64;
    let rate = 0.05_f64;
    let periods = 8_u16; // 8 years
    let present_value_annuity_due_solution = present_value_annuity_due(annuity_payment, rate, periods);
    dbg!(present_value_annuity_due_solution);
}

#[derive(Debug)]
pub struct PresentValueAnnuitySolution {
    pub annuity_payment_amount: f64,
    pub periodic_rate: f64,
    pub num_periods: f64,
    pub present_value_annuity: f64,
    pub due_at_beginning: bool,
}
impl PresentValueAnnuitySolution {
    pub fn new(annuity_payment_amount: f64, periodic_rate: f64, num_periods: f64, present_value_annuity: f64, due_at_beginning: bool) -> Self {
        Self {
            annuity_payment_amount,
            periodic_rate,
            num_periods,
            present_value_annuity,
            due_at_beginning,
        }
    }
}
pub fn present_value_annuity<T: Into<f64> + Copy, C: Into<f64> + Copy>(annuity_payment_amount: C, periodic_rate: f64, num_periods: T) -> PresentValueAnnuitySolution {
    //  P = (PMT [(1 - (1 / (1 + r)^n)) / r])
    let pmt = annuity_payment_amount.into();
    let n = num_periods.into();
    let pv = pmt * ((1. - (1. / (1. + periodic_rate)).powf(n)) / periodic_rate);

    PresentValueAnnuitySolution::new(pmt, periodic_rate, n, pv, false)
}

pub fn present_value_annuity_due<T: Into<f64> + Copy, C: Into<f64> + Copy>(annuity_payment_amount: C, periodic_rate: f64, num_periods: T) -> PresentValueAnnuitySolution {
    let pmt = annuity_payment_amount.into();
    let n = num_periods.into();
    //  P = (PMT [(1 - (1 / (1 + r)^n)) / r]) x (1+r)
    let pv = pmt * ((1. - (1. / (1. + periodic_rate)).powf(n)) / periodic_rate) * (1. + periodic_rate);

    PresentValueAnnuitySolution::new(pmt, periodic_rate, n, pv, true)
}

