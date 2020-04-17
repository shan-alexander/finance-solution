use log::{warn};

// Import needed for the function references in the Rustdoc comments.
#[allow(unused_imports)]
use crate::*;
use crate::verbose::tvm_cashflow::*;

pub fn main() { 
    // self::tests::test_payment_1();

    // expect -119.636085343
    let pmt = payment_using_future_value_solution(0.034, 10, 1397.0288910796);
    dbg!(pmt);
}

fn assert_inputs(rate:f64, periods:u32, fn_type: TvmCashflowVariable) {
    assert!(periods >= 1);
    assert!(rate.is_finite());
    if rate > 1. || rate < -1. {
        warn!("You provided an rate of {}%. Are you sure?", rate*100.);
    }
}

pub fn payment<P: Into<f64> + Copy, F: Into<f64> + Copy>(periodic_rate: f64, periods: u32, present_value: P, future_value: F) -> f64 {
    assert_inputs(periodic_rate, periods, TvmCashflowVariable::Payment);
    let pv = present_value.into();
    let fv = future_value.into();
    //  -pmt  = (fv + pv*(1+rate)^nper) * rate / ((1+rate)^nper-1) / (1+rate*type)
    // get the FV of the PV, and the add to the original FV, and then multiple times the rate, then divide by the compound rate
    ((pv * (1.0 + periodic_rate).powf(periods as f64)) + fv) * (-1.0 * periodic_rate) / ((1.0 + periodic_rate).powf(periods as f64) -1.0)
}
pub fn payment_due<P: Into<f64> + Copy, F: Into<f64> + Copy>(periodic_rate: f64, periods: u32, present_value: P, future_value: F) -> f64 {
    assert_inputs(periodic_rate, periods, TvmCashflowVariable::Payment);
    let pv = present_value.into();
    let fv = future_value.into();
    //  -pmt  = (fv + pv*(1+rate)^nper) * rate / ((1+rate)^nper-1) / (1+rate*type)
    // get the FV of the PV, and the add to the original FV, and then divide by the effective rate plus an extra compound
    ((pv * (1.0 + periodic_rate).powf(periods as f64)) + fv) * (-1.0 * periodic_rate) / ((1.0 + periodic_rate).powf(periods as f64) -1.0) / (1.0 + periodic_rate)
}
pub fn payment_using_present_value<P: Into<f64> + Copy>(periodic_rate: f64, periods: u32, present_value: P) -> f64 {
     assert_inputs(periodic_rate, periods, TvmCashflowVariable::Payment);
     let pv = present_value.into();
     pv * (1.0 + periodic_rate).powf(periods as f64) * (-1.0 * periodic_rate) / ((1.0 + periodic_rate).powf(periods as f64) -1.0)
}
pub fn payment_due_using_present_value<P: Into<f64> + Copy>(periodic_rate: f64, periods: u32, present_value: P) -> f64 {
    assert_inputs(periodic_rate, periods, TvmCashflowVariable::Payment);
    let pv = present_value.into();
    pv * (1.0 + periodic_rate).powf(periods as f64) * (-1.0 * periodic_rate) / ((1.0 + periodic_rate).powf(periods as f64) -1.0) / (1.0 + periodic_rate)
}
pub fn payment_using_future_value<F: Into<f64> + Copy>(periodic_rate: f64, periods: u32, future_value: F) -> f64 {
     assert_inputs(periodic_rate, periods, TvmCashflowVariable::Payment);
     let fv = future_value.into();
     -1.0 * fv * periodic_rate / ((1.0 + periodic_rate).powf(periods as f64) -1.0)
}
pub fn payment_due_using_future_value<F: Into<f64> + Copy>(periodic_rate: f64, periods: u32, future_value: F) -> f64 {
    assert_inputs(periodic_rate, periods, TvmCashflowVariable::Payment);
    let fv = future_value.into();
    -1.0 * fv * periodic_rate / ((1.0 + periodic_rate).powf(periods as f64) -1.0) / (1.0 + periodic_rate)
}
pub fn payment_solution<P: Into<f64> + Copy, F: Into<f64> + Copy>(periodic_rate: f64, periods: u32, present_value: P, future_value: F) -> TvmCashflowSolution {  
    // https://www.techrepublic.com/forums/discussions/the-real-math-behind-excel-formulas/   
    // If rate =0 then (Pmt * Nper)+PV+FV=0
    // ultimate formula:pv*(1+rate)^nper + pmt(1+rate*type)*((1+rate)^nper-1)/rate +fv = 0
    let pv = present_value.into();
    let fv = future_value.into();
    assert!(pv + fv > 0.0, "Present value and future value cannot both be 0.");
    let pmt = payment_using_present_value(periodic_rate, periods, present_value) + payment_using_future_value(periodic_rate, periods, future_value);
    let input_in_percent: String = format!("{}", periodic_rate * 100.);
    let formula = format!("pmt = pv * ( (1. + periodic_rate).powf(periods as f64) + fv) / (((1. + periodic_rate).powf(periods as f64) - 1.) / periodic_rate);");
    let cashflow_0 = 0_f64;
    let output = pmt;
    TvmCashflowSolution::new(TvmCashflowVariable::Payment, periodic_rate, periods, pmt, pv, fv, &formula, cashflow_0, input_in_percent, output)
}
pub fn payment_due_solution<P: Into<f64> + Copy, F: Into<f64> + Copy>(periodic_rate: f64, periods: u32, present_value: P, future_value: F) -> TvmCashflowSolution {  
    // https://www.techrepublic.com/forums/discussions/the-real-math-behind-excel-formulas/   
    // If rate =0 then (Pmt * Nper)+PV+FV=0
    // ultimate formula:pv*(1+rate)^nper + pmt(1+rate*type)*((1+rate)^nper-1)/rate +fv = 0
    let pv = present_value.into();
    let fv = future_value.into();
    let pmt = payment_due(periodic_rate, periods, present_value, future_value);
    let formula = format!("formula here");
    let cashflow_0 = 0_f64;
    let input_in_percent: String = format!("{}", periodic_rate * 100.);
    let output = pmt;
    TvmCashflowSolution::new(TvmCashflowVariable::PaymentDue, periodic_rate, periods, pmt, pv, fv, &formula, cashflow_0, input_in_percent, output)
}
pub fn payment_using_present_value_solution<P: Into<f64> + Copy>(periodic_rate: f64, periods: u32, present_value: P) -> TvmCashflowSolution {
    assert_inputs(periodic_rate, periods, TvmCashflowVariable::Payment);
    let pv = present_value.into();
    let fv = 0.0;
    let pmt = payment_using_present_value(periodic_rate, periods, present_value);
    let input_in_percent: String = format!("{}", periodic_rate * 100.);
    let formula = format!("= {} * ((1. + {}).pow({})) / (((1. + {}).pow({}) - 1.) / {});", pv, periodic_rate, periods, periodic_rate, periods, periodic_rate);
    let cashflow_0 = 0_f64;
    let output = pmt;
    TvmCashflowSolution::new(TvmCashflowVariable::Payment, periodic_rate, periods, pmt, pv, fv, &formula, cashflow_0, input_in_percent, output)
}
pub fn payment_due_using_present_value_solution<P: Into<f64> + Copy>(periodic_rate: f64, periods: u32, present_value: P) -> TvmCashflowSolution {
    assert_inputs(periodic_rate, periods, TvmCashflowVariable::PaymentDue);
    let pv = present_value.into();
    let fv = 0.0;
    let pmt = payment_due_using_present_value(periodic_rate, periods, pv);
    let input_in_percent: String = format!("{}", periodic_rate * 100.);
    let formula = format!("= {} * ((1. + {}).pow({})) / (((1. + {}).pow({}) - 1.) / {});", pv, periodic_rate, periods, periodic_rate, periods, periodic_rate);
    let cashflow_0 = 0_f64;
    let output = pmt;
    TvmCashflowSolution::new(TvmCashflowVariable::PaymentDue, periodic_rate, periods, pmt, pv, fv, &formula, cashflow_0, input_in_percent, output)
}
pub fn payment_using_future_value_solution<F: Into<f64> + Copy>(periodic_rate: f64, periods: u32, future_value: F) -> TvmCashflowSolution {
    assert_inputs(periodic_rate, periods, TvmCashflowVariable::Payment);
    let pv = 0.0;
    let fv = future_value.into();
    let pmt = payment_using_future_value(periodic_rate, periods, pv);
    let input_in_percent: String = format!("{}", periodic_rate * 100.);
    let formula = format!("= {} * ((1. + {}).pow({})) / (((1. + {}).pow({}) - 1.) / {});", pv, periodic_rate, periods, periodic_rate, periods, periodic_rate);
    let cashflow_0 = 0_f64;
    let output = pmt;
    TvmCashflowSolution::new(TvmCashflowVariable::Payment, periodic_rate, periods, pmt, pv, fv, &formula, cashflow_0, input_in_percent, output)
}
pub fn payment_due_using_future_value_solution<F: Into<f64> + Copy>(periodic_rate: f64, periods: u32, future_value: F) -> TvmCashflowSolution {
    assert_inputs(periodic_rate, periods, TvmCashflowVariable::PaymentDue);
    let pv = 0.0;
    let fv = future_value.into();
    let pmt = payment_due_using_future_value(periodic_rate, periods, pv);
    let input_in_percent: String = format!("{}", periodic_rate * 100.);
    let formula = format!("= {} * ((1. + {}).pow({})) / (((1. + {}).pow({}) - 1.) / {});", pv, periodic_rate, periods, periodic_rate, periods, periodic_rate);
    let cashflow_0 = 0_f64;
    let output = pmt;
    TvmCashflowSolution::new(TvmCashflowVariable::PaymentDue, periodic_rate, periods, pmt, pv, fv, &formula, cashflow_0, input_in_percent, output)
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    
    #[test]
    fn test_payment_1() {
        let payment_s_1_a = payment_solution(0.034, 10, 1000, 0);
        let payment_s_1_b = payment_using_present_value_solution(0.034,10, 1000);
        let payment_s_1_c = payment_using_future_value_solution(0.034,10, 1397.0288910796);
        dbg!(payment_s_1_a);
        dbg!(payment_s_1_b);
        dbg!(payment_s_1_c);
        
    
        let payment_1_a = payment_using_present_value(0.034,10, 1000);
        let payment_1_b = payment_using_future_value(0.034,10, 1397.0288910796);
        let payment_1_c = payment(0.034,10, 1000, 0);
        let payment_1_d = payment(0.034,10, 0, 1397.0288910796);
        println!("Excel gives: -119.63608534269");
        dbg!(payment_1_a);
        dbg!(payment_1_b);
        dbg!(payment_1_c);
        dbg!(payment_1_d);
        println!("This should be the payment * 2:");
        let payment_2_a = payment(0.034,10, 1000, 1397.0288910796);
    
        // from excel
        let pmt_1 = -119.636085343; 
        let pmt_2 = -239.272170685;
    
        assert_approx_equal!(payment_1_a, pmt_1);
        assert_approx_equal!(payment_1_b, pmt_1);
        assert_approx_equal!(payment_1_c, pmt_1);
        assert_approx_equal!(payment_1_d, pmt_1);
        assert_approx_equal!(payment_2_a, pmt_2);
    }

}