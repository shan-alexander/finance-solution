#![allow(dead_code)]
#![allow(unused_imports)]

use float_cmp::ApproxEq;
use log::Level;
use log::{info, warn, log_enabled};

// https://exceljet.net/excel-functions/excel-pmt-function
// https://exceljet.net/formula/estimate-mortgage-payment

pub fn main() { 
    try_payment();
}

// fn try_payment() {
//     // expect $3629.40 in Excel currency format
//     let loan_amount = 200_000_f64;
//     let annual_percentage_rate = 0.034_f64; // APR
//     let periodic_rate = annual_percentage_rate / 12_f64; // monthly periods/payments
//     let num_periods = 60_u16; // 5 years of monthly payments
//     let payment_solution = payment(loan_amount, periodic_rate, num_periods);
//     dbg!(&payment_solution);
//     println!("Monthly payment: {}", payment_solution.period_payment);
// }

fn try_payment() {
    // expect $3629.40 in Excel currency format
    let present_value = 200_000_f64; // loan_amount
    let future_value = 0_f64;
    let annual_percentage_rate = 0.034_f64; // APR
    let periodic_rate = annual_percentage_rate / 12_f64; // monthly periods/payments
    let num_periods = 60_u16; // 5 years of monthly payments
    let payment_solution = payment(periodic_rate, num_periods, present_value, future_value);
    dbg!(&payment_solution);
    println!("Monthly payment: {}", payment_solution.period_payment);
}

#[derive(Debug)]
pub struct PaymentSolution {
    pub periodic_rate: f64,
    pub num_periods: f64,
    pub present_value: f64,
    pub future_value: f64,
    pub period_payment: f64,
    pub total_payment: f64,
    pub total_interest_amount: f64,
    pub due: bool,
}
impl PaymentSolution {
    pub fn new(periodic_rate: f64, num_periods: f64, present_value: f64, future_value: f64, period_payment: f64, total_payment: f64, total_interest_amount: f64, due: bool) -> Self {
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

pub fn payment<T: Into<f64> + Copy, P: Into<f64> + Copy, F: Into<f64> + Copy>(periodic_rate: f64, num_periods: T, present_value: P, future_value: F) -> PaymentSolution {
    
    // https://www.techrepublic.com/forums/discussions/the-real-math-behind-excel-formulas/   
    // If rate =0 then (Pmt * Nper)+PV+FV=0


    // ultimate formula:pv*(1+rate)^nper + pmt(1+rate*type)*((1+rate)^nper-1)/rate +fv = 0
    // (pv*(1+rate)^nper + fv) / ((1+rate)^nper-1)/rate  / (1+rate*type) = - pmt
    let n = num_periods.into();
    let pv = present_value.into();
    let fv = future_value.into();
    let pmt = pv * ( (1. + periodic_rate).powf(n) + fv) / (((1. + periodic_rate).powf(n) - 1.) / periodic_rate);
    let total_payment = pmt * n;
    let total_interest_amount = total_payment - (fv + pv);
    PaymentSolution::new(periodic_rate, n, pv, fv, pmt, total_payment, total_interest_amount, true)
}

pub fn payment_due<T: Into<f64> + Copy, P: Into<f64> + Copy, F: Into<f64> + Copy>(periodic_rate: f64, num_periods: T, present_value: P, future_value: F) -> PaymentSolution {
    
    // https://www.techrepublic.com/forums/discussions/the-real-math-behind-excel-formulas/   
    // If rate =0 then (Pmt * Nper)+PV+FV=0


    // ultimate formula:pv*(1+rate)^nper + pmt(1+rate*type)*((1+rate)^nper-1)/rate +fv = 0
    // (pv*(1+rate)^nper + fv) / ((1+rate)^nper-1)/rate  / (1+rate*type) = - pmt
    let n = num_periods.into();
    let pv = present_value.into();
    let fv = future_value.into();
    let pmt = pv * ( (1. + periodic_rate).powf(n) + fv) / (((1. + periodic_rate).powf(n) - 1.) / periodic_rate) / (1. + periodic_rate);
    let total_payment = pmt * n;
    let total_interest_amount = total_payment - (fv + pv);
    PaymentSolution::new(periodic_rate, n, pv, fv, pmt, total_payment, total_interest_amount, true)
}