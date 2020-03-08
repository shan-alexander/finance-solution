#![allow(dead_code)]
#![allow(unused_imports)]

use float_cmp::ApproxEq;
use log::Level;
use log::{info, warn, log_enabled};

// https://exceljet.net/excel-functions/excel-pmt-function
// https://exceljet.net/formula/estimate-mortgage-payment

pub fn main() { 
    try_payment();
    try_payment_b();
}

fn try_payment() {
    // expect $3629.40 in Excel currency format
    let loan_amount = 200_000_f64;
    let annual_percentage_rate = 0.034_f64; // APR
    let periodic_rate = annual_percentage_rate / 12_f64; // monthly periods/payments
    let num_periods = 60_u16; // 5 years of monthly payments
    let payment_solution = payment(loan_amount, periodic_rate, num_periods);
    dbg!(&payment_solution);
    println!("Monthly payment: {}", payment_solution.period_payment);
}

// not working... attempt to duplicate excel function with this (feel free to overwrite)
fn try_payment_b() {
    // expect $3629.40 in Excel currency format
    let present_value = 200_000_f64; // loan_amount
    let future_value = 0_f64;
    let annual_percentage_rate = 0.034_f64; // APR
    let periodic_rate = annual_percentage_rate / 12_f64; // monthly periods/payments
    let num_periods = 60_u16; // 5 years of monthly payments
    let payment_solution = payment_b(periodic_rate, num_periods, present_value, future_value, false);
    dbg!(&payment_solution);
    println!("Monthly payment: {}", payment_solution.period_payment);
}

#[derive(Debug)]
pub struct PaymentSolution {
    pub loan_amount: f64,
    pub periodic_rate: f64,
    pub num_periods: u16,
    pub period_payment: f64,
    pub total_payment: f64,
    pub total_interest_amount: f64,
}
impl PaymentSolution {
    pub fn new(loan_amount: f64, periodic_rate: f64, num_periods: u16, period_payment: f64, total_payment: f64, total_interest_amount: f64) -> Self {
        Self {
            loan_amount,
            periodic_rate,
            num_periods,
            period_payment,
            total_payment,
            total_interest_amount,
        }
    }
}
// https://superuser.com/questions/871404/what-would-be-the-the-mathematical-equivalent-of-this-excel-formula-pmt
// https://www.quora.com/What-is-the-actual-formula-behind-PMT-function-in-Excel-and-how-is-it-used
/// Returns a Payment solution. Typically, the rate of a loan is quoted as an APR (annual percentage rate), and must be converted into a periodic rate (APR/num_periods_in_year) for this function. Payments due at end of period. Currenttly, this function does not support Payments due at beginning of the period.
pub fn payment(loan_amount: f64, periodic_rate: f64, num_periods: u16) -> PaymentSolution {
    // PV*((1+Rate)^nper) + pmt(1+rate*type) * (((1+rate)^Nper) -1 )/ rate) + FV = 0
    // ( PV*((1+Rate)^nper) + FV ) / (((1+rate)^Nper) -1 )/ rate) / (1+rate*type)   = -pmt
    // If rate =0 then (Pmt * Nper)+PV+FV=0
    // https://www.techrepublic.com/forums/discussions/the-real-math-behind-excel-formulas/
    let period_payment = (loan_amount * periodic_rate) / (1. - (1. + periodic_rate).powi(-1 * (num_periods) as i32));
    let total_payment = period_payment * num_periods as f64;
    let total_interest_amount = total_payment - loan_amount;
    PaymentSolution::new(loan_amount, periodic_rate, num_periods, period_payment, total_payment, total_interest_amount)
}

#[derive(Debug)]
pub struct PaymentSolution_b {
    pub periodic_rate: f64,
    pub num_periods: u16,
    pub present_value: f64,
    pub future_value: f64,
    pub period_payment: f64,
    pub total_payment: f64,
    pub total_interest_amount: f64,
    pub due: bool,
}
impl PaymentSolution_b {
    pub fn new(periodic_rate: f64, num_periods: u16, present_value: f64, future_value: f64, period_payment: f64, total_payment: f64, total_interest_amount: f64, due: bool) -> Self {
        Self {
            periodic_rate,
            num_periods, 
            present_value, 
            future_value,
            period_payment,
            total_payment,
            total_interest_amount,
            due,
        }
    }
}
// not working... attempt to duplicate excel function with this (feel free to overwrite)
pub fn payment_b(periodic_rate: f64, num_periods: u16, present_value: f64, future_value: f64, due: bool) -> PaymentSolution_b {
    // PV*((1+Rate)^nper) + pmt(1+rate*type) * (((1+rate)^Nper) -1 )/ rate) + FV = 0
    // ( PV*((1+Rate)^nper) + FV ) / (((1+rate)^Nper) -1 )/ rate) / (1+rate*type)   = -pmt
    // If rate =0 then (Pmt * Nper)+PV+FV=0
    // https://www.techrepublic.com/forums/discussions/the-real-math-behind-excel-formulas/
    
    // let period_payment = (loan_amount * periodic_rate) / (1. - (1. + periodic_rate).powi(-1 * (num_periods) as i32));
    let pmt = present_value * ( (1. + periodic_rate).powi(num_periods as i32) * future_value) / (((1. + periodic_rate).powi(num_periods as i32) - 1.) / periodic_rate) / (1. + (periodic_rate * due as i32 as f64));
    let total_payment = pmt * num_periods as f64;
    let total_interest_amount = total_payment - (future_value + present_value);
    PaymentSolution_b::new(periodic_rate, num_periods, present_value, future_value, pmt, total_payment, total_interest_amount, due)
}