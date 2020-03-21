#![allow(unused_imports)]
// #![allow(dead_code)]

use super::convert_rates;
use super::present_value::{present_value, PresentValueSolution};
use super::present_value_annuity::{present_value_annuity, PresentValueAnnuitySolution};
use super::future_value::{future_value, FutureValueSolution};
use super::future_value_annuity::{future_value_annuity, FutureValueAnnuitySolution};
use super::payment::{payment, PaymentSolution};


pub fn main() {
    try_problem_1();
    try_problem_2();
}

/* PROBLEM 1a, 1b
What is the balance in an account at the end of 10 years 
if $2,500 is deposited today and the account 
earns 4% interest, compounded annually? quarterly?
*/
fn try_problem_1() {
    let t = 10;
    let pv = 2500;
    let r = 0.04;
    let answer_a = future_value(r, pv, t);
    // expect $3,700.61
    dbg!(answer_a);

    // Compounded quarterly, thus 4 compounding periods per year
    let answer_b = future_value(r, pv, t * 4);
    // expect $3,722.16
    dbg!(answer_b);
}

/* PROBLEM 2
How much will be in an account at the end of five years if
the amount deposited today is $10,000 and interest is 8% per year,
compounded semi-annually?
FV = 10,000 * (1 + 0.04)^10
*/
fn try_problem_2() {
    let apr = 0.08;
    let num_compounding_periods_in_year = 2;
    let periodic_rate = convert_rates::convert_apr_to_periodic(apr, num_compounding_periods_in_year).periodic_rate;
    let num_periods = 5 * num_compounding_periods_in_year; // 5 years, 2 compounds per year
    let pv_amount = 10_000;
    let fv_answer = future_value(periodic_rate, pv_amount, num_periods);
    // expect $14,802.44
    dbg!(fv_answer);
}
