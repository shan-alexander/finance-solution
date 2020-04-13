use log::{warn};

// Import needed for the function references in the Rustdoc comments.
#[allow(unused_imports)]
use crate::*;
use crate::verbose::tvm_cashflow::*;

pub fn main() { 
    dbg!(payment_solution(0.034,12, 3000, 0));
    // self::tests::test_convert_rates_1();

}

fn assert_inputs(rate:f64, periods:u32, fn_type: TvmCashflowVariable) {
    assert!(periods >= 1);
    assert!(rate.is_finite());
    if rate > 1. || rate < -1. {
        warn!("You provided an rate of {}%. Are you sure?", rate*100.);
    }
}

pub fn payment_solution<P: Into<f64> + Copy, F: Into<f64> + Copy>(periodic_rate: f64, periods: u32, present_value: P, future_value: F) -> TvmCashflowSolution {
    
    // https://www.techrepublic.com/forums/discussions/the-real-math-behind-excel-formulas/   
    // If rate =0 then (Pmt * Nper)+PV+FV=0


    // ultimate formula:pv*(1+rate)^nper + pmt(1+rate*type)*((1+rate)^nper-1)/rate +fv = 0
    // (pv*(1+rate)^nper + fv) / ((1+rate)^nper-1)/rate  / (1+rate*type) = - pmt
    let pv = present_value.into();
    let fv = future_value.into();
    let pmt = pv * ( (1. + periodic_rate).powf(periods as f64) + fv) / (((1. + periodic_rate).powf(periods as f64) - 1.) / periodic_rate);
    let input_in_percent: String = format!("{}", periodic_rate * 100.);
    let formula = format!("formula here");
    let cashflow_0 = 0_f64;
    let output = pmt;
    // let total_payment = pmt * periods;
    // let total_interest_amount = total_payment - (fv + pv);
    //sum_payment, total_interest_amount,   <---- consider adding something like this
    TvmCashflowSolution::new(TvmCashflowVariable::Payment, periodic_rate, periods, pmt, pv, fv, &formula, cashflow_0, input_in_percent, output)
}

pub fn payment_due_solution<P: Into<f64> + Copy, F: Into<f64> + Copy>(periodic_rate: f64, periods: u32, present_value: P, future_value: F) -> TvmCashflowSolution {
    
    // https://www.techrepublic.com/forums/discussions/the-real-math-behind-excel-formulas/   
    // If rate =0 then (Pmt * Nper)+PV+FV=0


    // ultimate formula:pv*(1+rate)^nper + pmt(1+rate*type)*((1+rate)^nper-1)/rate +fv = 0
    // (pv*(1+rate)^nper + fv) / ((1+rate)^nper-1)/rate  / (1+rate*type) = - pmt

    let pv = present_value.into();
    let fv = future_value.into();
    let pmt = pv * ( (1. + periodic_rate).powf(periods as f64) + fv) / (((1. + periodic_rate).powf(periods as f64) - 1.) / periodic_rate) / (1. + periodic_rate);
    // let total_payment = pmt * periods;
    // let total_interest_amount = total_payment - (fv + pv);
    let formula = format!("formula here");
    let cashflow_0 = 0_f64;
    let input_in_percent: String = format!("{}", periodic_rate * 100.);
    let output = pmt;
    TvmCashflowSolution::new(TvmCashflowVariable::PaymentDue, periodic_rate, periods, pmt, pv, fv, &formula, cashflow_0, input_in_percent, output)
}