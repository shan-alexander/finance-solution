use log::{warn};

// Import needed for the function references in the Rustdoc comments.
#[allow(unused_imports)]
use crate::*;

pub fn main() {
}

pub fn payment<P, F>(rate: f64, periods: u32, present_value: P, future_value: F) -> f64
    where
        P: Into<f64> + Copy,
        F: Into<f64> + Copy
{
    payment_internal(rate, periods, present_value.into(), future_value.into(), false)
}

pub fn payment_due<P, F>(rate: f64, periods: u32, present_value: P, future_value: F) -> f64
    where
        P: Into<f64> + Copy,
        F: Into<f64> + Copy
{
    /*
    let pv = present_value.into();
    let fv = future_value.into();
    let payment_original = ((pv * (1.0 + rate).powf(periods as f64)) + fv) * (-1.0 * rate) / ((1.0 + rate).powf(periods as f64) -1.0) / (1.0 + rate);
    let payment_grouped = (((pv * (1.0 + rate).powf(periods as f64)) + fv) * -rate) / (((1.0 + rate).powf(periods as f64) -1.0) * (1.0 + rate));
    dbg!(payment_original, payment_grouped);
    assert_approx_equal!(payment_original, payment_grouped);
    */

    let payment = payment_internal(rate, periods, present_value.into(), future_value.into(), true);

    /*
    dbg!(payment_original, payment_grouped, payment);
    if payment_original.is_finite() {
        assert_approx_equal!(payment_original, payment);
    }
    */

    payment
}

fn payment_internal(rate: f64, periods: u32, present_value: f64, future_value: f64, due_at_beginning: bool) -> f64 {
    //dbg!(rate, periods, present_value, future_value);
    assert!(rate.is_finite(), "The rate must be finite (not NaN or infinity)");
    // assert!(rate >= -1.0, "The rate must be greater than or equal to -1.0 because a rate lower than -100% would mean the investment loses more than its full value in a period.");
    if rate.abs() > 1. {
        warn!("You provided a periodic rate ({}) greater than 1. Are you sure you expect a {}% return?", rate, rate * 100.0);
    }
    assert!(present_value.is_finite(), "The present value must be finite (not NaN or infinity)");
    assert!(future_value.is_finite(), "The present value must be finite (not NaN or infinity)");
    assert!(!(periods == 0 && !is_approx_equal!(present_value, future_value)), "The number of periods is 0 yet the future value doesn't equal the present value. There's no way to calculate payments.");

    if periods == 0 {
        // This is an edge case. It's OK for periods to be zero as long as there's no change between
        // the present and future values.
        assert_approx_equal!(present_value, future_value);
        return 0.0;
    }

    if rate == 0.0 {
        // There is no interest so the payment is simply the total amount to be paid divided by the
        // number of periods.
        return (-present_value - future_value) / periods as f64;
    }

    let rate_mult = 1.0 + rate;
    let num= ((present_value * rate_mult.powf(periods as f64)) + future_value) * -rate;
    // dbg!(num);
    assert!(num.is_finite());
    let mut denom = (rate_mult).powf(periods as f64) - 1.0;
    if due_at_beginning {
        denom *= rate_mult;
    }
    // dbg!(denom);
    assert!(denom.is_finite());
    assert!(denom.is_normal());
    let payment = num / denom;
    // dbg!(payment);
    assert!(payment.is_finite());

    payment
}

pub fn payment_solution<P, F>(rate: f64, periods: u32, present_value: P, future_value: F) -> TvmCashflowSolution
    where
        P: Into<f64> + Copy,
        F: Into<f64> + Copy
{
    let due_at_beginning = false;
    payment_solution_internal(rate, periods, present_value.into(), future_value.into(), due_at_beginning)
}

pub fn payment_due_solution<P, F>(rate: f64, periods: u32, present_value: P, future_value: F) -> TvmCashflowSolution
    where
        P: Into<f64> + Copy,
        F: Into<f64> + Copy
{
    let due_at_beginning = true;
    payment_solution_internal(rate, periods, present_value.into(), future_value.into(), due_at_beginning)
}

fn payment_solution_internal(rate: f64, periods: u32, present_value: f64, future_value: f64, due_at_beginning: bool) -> TvmCashflowSolution {
    let payment = payment_internal(rate, periods, present_value, future_value, due_at_beginning);

    let (formula, formula_symbolic) = payment_formula(rate, periods, present_value, future_value, due_at_beginning);
    let cashflow = payment;
    let cashflow_0 = 0_f64;
    // let total_payment = pmt * periods;
    // let total_interest_amount = total_payment - (fv + pv);
    //sum_payment, total_interest_amount,   <---- consider adding something like this
    let rate_schedule = Schedule::new_repeating(ValueType::Rate, rate, periods);
    TvmCashflowSolution::new(TvmCashflowVariable::Payment, rate_schedule, periods, cashflow, cashflow_0, present_value.into(), future_value, payment, due_at_beginning, &formula, &formula_symbolic)
}

fn payment_formula(rate: f64, periods: u32, present_value: f64, future_value: f64, due_at_beginning: bool) -> (String, String) {
    let rate_multiplier = 1.0 + rate;
    if present_value == future_value {
        // This includes the edge case where periods = 0.
        (format!("{:.4}", 0.0), "0".to_string())
    } else if rate == 0.0 {
        // There is no interest so the payment is simply the total amount to be paid (the difference
        // between the present and future value) divided by the number of periods. We subtract
        // present value from future value because if present_value is higher than future value the
        // payments should be negative.
        if future_value == 0.0 {
            (format!("{:.4} / {}", -present_value, periods), "-pv / n".to_string())
        } else if present_value == 0.0 {
            (format!("{:.4} / {}", -future_value, periods), "-fv / n".to_string())
        } else {
            // We have both a present and future value.
            let add_future_value = if future_value > 0.0 {
                format!(" - {:.4}", future_value)
            } else {
                format!(" + {:.4}", -future_value)
            };
            let formula = format!("({:.4}{}) / {}", -present_value, add_future_value, periods);
            let formula_symbolic = "(-pv - fv) / n".to_string();
            (formula, formula_symbolic)
        }
    } else {
        let (formula_num, formula_symbolic_num) = if future_value == 0.0 {
            // We can slightly simplify the formula by not including the future value term.
            (format!("(({:.4} * {:.6}^{}) * {:.6})", present_value, rate_multiplier, periods, -rate), "((pv * (1 + r)^n) * -r)".to_string())
        } else {
            if present_value == 0.0 {
                // We can simplify the formula by not including the present value term.
                (format!("({:.4} * {:.6})", future_value, -rate), "(fv * -r)".to_string())
            } else {
                // We have both a present and future value.
                let add_future_value = if future_value > 0.0 {
                    format!(" + {:.4}", future_value)
                } else {
                    format!(" - {:.4}", 0.0 - future_value)
                };
                (format!("((({:.4} * {:.6}^{}){}) * {:.6})", present_value, rate_multiplier, periods, add_future_value, -rate), "(((pv * (1 + r)^n) + fv) * -r)".to_string())
            }
        };
        let mut formula_denom = format!("({:.6}^{} - 1)", rate_multiplier, periods);
        let mut formula_symbolic_denom = "((1 + r)^n - 1)".to_string();
        if due_at_beginning {
            formula_denom = format!("({} * {:.6})", formula_denom, rate_multiplier);
            formula_symbolic_denom = format!("({} * (1 + r))", formula_symbolic_denom);
        }
        (format!("{} / {}", formula_num, formula_denom), format!("{} / {}", formula_symbolic_num, formula_symbolic_denom))
    }
}

/*
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
    let cashflow = pmt;
    let cashflow_0 = 0_f64;
    let formula = format!("formula here");
    let input_in_percent: String = format!("{}", periodic_rate * 100.);
    let output = pmt;
    let rate_schedule = Schedule::new_repeating(ValueType::Rate, periodic_rate, periods);
    TvmCashflowSolution::new(TvmCashflowVariable::PaymentDue, rate_schedule, periods, cashflow, cashflow_0, present_value, future_value, payment, &formula)
}
*/

/*
fn check_payment_parameters(rate: f64, periods: u32, present_value: f64, future_value: f64) {
    assert!(rate.is_finite(), "The rate must be finite (not NaN or infinity)");
    assert!(rate >= -1.0, "The rate must be greater than or equal to -1.0 because a rate lower than -100% would mean the investment loses more than its full value in a period.");
    if rate.abs() > 1. {
        warn!("You provided a periodic rate ({}) greater than 1. Are you sure you expect a {}% return?", rate, rate * 100.0);
    }
    assert!(present_value.is_finite(), "The present value must be finite (not NaN or infinity)");
    assert!(future_value.is_finite(), "The future value must be finite (not NaN or infinity)");
    assert!(!(present_value == 0.0 && future_value != 0.0), "The present value is zero and the future value is nonzero so there's no way to solve for the payments.");
    assert!(!(present_value != 0.0 && future_value == 0.0 && rate != -1.0), "The present value is nonzero, the future value is zero, and the rate is not -100% so there's no way to solve for the payments.");
    assert!(!(present_value < 0.0 && future_value > 0.0), "The present value is negative and the future value is positive so there's no way to solve for the payments.");
    assert!(!(present_value > 0.0 && future_value < 0.0), "The present value is positive and the future value is negative so there's no way to solve for the payments.");
    assert!(!(present_value < 0.0 && future_value < present_value && rate <= 0.0), "The present value and future value are both negative, the future value is less than the present value, and the periodic rate is zero or negative. There's no way to solve for the payments.");
    assert!(!(present_value < 0.0 && future_value > present_value && rate >= 0.0), "The present value and future value are both negative, the future value is greater than the present value, and the periodic rate is zero or positive. There's no way to solve for the payments.");
    assert!(!(present_value > 0.0 && future_value > present_value && rate <= 0.0), "The present value and future value are both positive, the future value is greater than the present value, and the periodic rate is zero or negative. There's no way to solve for the payments.");
    assert!(!(present_value > 0.0 && future_value < present_value && rate >= 0.0), "The present value and future value are both positive, the future value is less than the present value, and the periodic rate is zero or positive. There's no way to solve for the payments.");
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::*;

    #[test]
    fn test_payment_nominal() {
        assert_rounded_6!(-11.9636085342686, payment(0.034, 10, 100.0, 0.0));
        assert_rounded_6!(-8.22683411973293, payment(-0.034, 10, 100.0, 0.0));
        assert_rounded_6!(11.9636085342686, payment(0.034, 10, -100.0, 0.0));
        assert_rounded_6!(-100.097751710655, payment(1.0, 10, 100.0, 0.0));
        assert_rounded_6!(-150.01573028944, payment(1.5, 10, 100.0, 0.0));
        assert_rounded_6!(-10.0, payment(0.0, 10, 100.0, 0.0));
        assert_rounded_6!(-10.00055000825, payment(0.00001, 10, 100.0, 0.0));
        assert_rounded_6!(8.22683411973293, payment(-0.034, 10, -100.0, 0.0));
        assert_rounded_6!(9.82270640070143, payment(0.034, 10, -100, 25));
        assert_rounded_6!(14.1045106678357, payment(0.034, 10, -100, -25));
    }

    #[test]
    fn test_payment_edge() {
        // Zero interest.
        assert_rounded_6!(-10.0, payment(0.0, 10, 100.0, 0.0));
        // Zero periods but it's OK because the present and future value are equal.
        assert_rounded_6!(0.0, payment(0.05, 0, 100.0, 100.0));
    }


    #[test]
    fn test_payment_due_nominal() {
        assert_rounded_6!(-11.5702210196021, payment_due(0.034, 10, 100.0, 0.0));
        assert_rounded_6!(-8.51639142829496, payment_due(-0.034, 10, 100.0, 0.0));
        assert_rounded_6!(-8.51639142829496, payment_due(-0.034, 10, 100.0, 0.0));
        assert_rounded_6!(-50.0488758553275, payment_due(1.0, 10, 100.0, 0.0));
        assert_rounded_6!(-60.0062921157762, payment_due(1.5, 10, 100.0, 0.0));
        assert_rounded_6!(-10.0, payment_due(0.0, 10, 100.0, 0.0));
        assert_rounded_6!(-10.0004500037499, payment_due(0.00001, 10, 100.0, 0.0));
        assert_rounded_6!(9.49971605483697, payment_due(0.034, 10, -100, 25));
        assert_rounded_6!(13.6407259843672, payment_due(0.034, 10, -100, -25));
        assert_rounded_6!(-10.0, payment_due(0.0, 10, 100, 0));
        assert_rounded_6!(-11.0, payment_due(0.0, 10, 100, 10));
        assert_rounded_6!(-9.0, payment_due(0.0, 10, 100, -10));
        assert_rounded_6!(-1.0, payment_due(0.0, 10, 10, 0));
        assert_rounded_6!(-11.0, payment_due(0.0, 10, 10, 100));
        assert_rounded_6!(9.0, payment_due(0.0, 10, 10, -100));
        assert_rounded_6!(10.0, payment_due(0.0, 10, -100, 0));
        assert_rounded_6!(9.0, payment_due(0.0, 10, -100, 10));
        assert_rounded_6!(11.0, payment_due(0.0, 10, -100, -10));
        assert_rounded_6!(1.0, payment_due(0.0, 10, -10, 0));
        assert_rounded_6!(-9.0, payment_due(0.0, 10, -10, 100));
        assert_rounded_6!(11.0, payment_due(0.0, 10, -10, -100));
    }

    /*
    #[should_panic]
    #[test]
    fn test_future_value_error_rate_low() {
        future_value(-101.0, 5, 250_000.00);
    }
    */
//assert_rounded_6(-11.5702210196021, payment_due(0.034, 10, 100.0, 0.0));
//assert_rounded_6(-8.51639142829496, payment_due(-0.034, 10, 100.0, 0.0));
}