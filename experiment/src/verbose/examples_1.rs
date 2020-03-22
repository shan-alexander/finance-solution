#![allow(unused_imports)]
// #![allow(dead_code)]

use super::convert_rates;
use super::present_value::{present_value, PresentValueSolution};
use super::future_value::{future_value, FutureValueSolution};
use super::nper::{nper, NperSolution};


pub fn main() {
    try_problem_1();
    try_problem_2();
    try_problem_3();
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

/* PROBLEM 3
Suppose you want to have $0.5 million saved by the time you are age 40 
and you are 30 years old today. You can earn 5% on your funds. 
How much do you you need to invest today, to reach your goal?
PV = 500,000 / (1 + 0.05)^10
PV = $306,959.63
*/
fn try_problem_3() {
    let apr = 0.05;
    let num_periods = 10;
    let fv_amount = 500_000;
    let pv_answer = present_value(apr, fv_amount, num_periods);
    // expect $306,959.63
    dbg!(pv_answer);
}

/* PROBLEM 4
Suppose you want to be able to withdraw $5,000 at the end of five years
and withdraw $6,000 at the end of 6 years, leaving a zero balance in the
account after the last withdrawal. If you can earn 5%, how much must you
deposit today to satisfy your withdrawals needs?
Answer: $8,394.92
*/



/* PROBLEM 5
Suppose you deposit $100,000 in an account today
that pays 6% interest, compounded annually.
How long before your balance is $500,000?
Answer: 28 
*/

