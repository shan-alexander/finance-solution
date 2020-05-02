// word problems:
// https://www.westga.edu/~rbest/finc3511/sampleprobs/sample5.pdf

// teaching resources:
// https://www.studocu.com/en-us/document/wichita-state-university/financial-management-ii/assignments/time-value-of-money-practice-problems-and-solutions/1915379/view
// http://mupfc.marshall.edu/~brozik/timevalueweb.pdf

// use finance::{present_value_solution, future_value_solution, future_value, periods_solution, periods};
use finance::*;
pub fn main() {
    pv_problem_1();
    fv_problem_1();
    nper_problem_1();
}

// If you wish to accumulate $140,000 in 13 years, 
// how much must you deposit today in an account that
// pays an annual interest rate of 14%? 
// Expect $25,489.71
fn pv_problem_1() {
    let future_value = 140_000;
    let periods = 13;
    let rate = 0.14;
    let present_value_s = present_value_solution(rate, periods, future_value);
    dbg!(&present_value_s); // Outputs solution struct.
    dbg!(present_value_s.present_value()); // Outputs 25489.71433359101 from solution struct.
    dbg!(present_value(rate, periods, future_value)); // Outputs 25489.71433359101 from f64 fn.
}

// What will $247,000 grow to be in 9 years if it is invested today 
// in an account with an annual interest rate of 11%? 
// Expect $631,835.12
fn fv_problem_1() {
    let present_value = 247_000;
    let periods = 9;
    let rate = 0.11;
    let future_value_s = future_value_solution(rate, periods, present_value);
    dbg!(&future_value_s); // Outputs solution struct.
    dbg!(future_value_s.future_value()); // Outputs 631835.1203234661 from solution struct.
    dbg!(future_value(rate, periods, present_value)); // Outputs 631835.1203234661 from f64 fn.
}

// How many years will it take for $136,000 to grow to be $468,000 
// if it is invested in an account with an annual interest rate of 8%? 
// Expect 16.06 years
fn nper_problem_1() {
    let present_value = 136_000;
    let future_value = 468_000;
    let rate = 0.08;
    let periods_s = periods_solution(rate, present_value, future_value);
    dbg!(&periods_s); // Outputs solution struct.
    dbg!(periods_s.periods()); // Outputs 17 from solution struct.
    dbg!(periods_s.fractional_periods()); // Outputs 16.057649324100133 from solution struct.
    dbg!(periods(rate, present_value, future_value)); // Outputs 16.057649324100133 from f64 fn.
}