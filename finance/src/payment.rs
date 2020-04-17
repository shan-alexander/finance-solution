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
    // https://www.techrepublic.com/forums/discussions/the-real-math-behind-excel-formulas/
    // If rate =0 then (Pmt * Nper)+PV+FV=0
    // ultimate formula:pv*(1+rate)^nper + pmt(1+rate*type)*((1+rate)^nper-1)/rate +fv = 0
    // (pv*(1+rate)^nper + fv) / ((1+rate)^nper-1)/rate  / (1+rate*type) = - pmt

    let present_value= present_value.into();
    let future_value = future_value.into();
    dbg!(rate, periods, present_value, future_value);
    assert!(rate.is_finite(), "The rate must be finite (not NaN or infinity)");
    assert!(rate >= -1.0, "The rate must be greater than or equal to -1.0 because a rate lower than -100% would mean the investment loses more than its full value in a period.");
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
        // There is no interest so the payment is simply the total amount to be paid (the difference
        // between the present and future value) divided by the number of periods. We subtract
        // present value from future value because if present_value is higher than future value the
        // payments should be negative.
        return (future_value - present_value) / periods as f64;
    }

    let future_value_no_payments = crate::future_value(rate, periods, present_value);
    dbg!(future_value_no_payments);
    let pmt_alt_1 = (future_value_no_payments + future_value) / periods as f64;
    dbg!(pmt_alt_1);

    let pmt_alt_2 = present_value * ( (1. + rate).powf(periods as f64) + future_value) / (((1.0 + rate).powf(periods as f64) - 1.0) / rate);
    dbg!(pmt_alt_2);

    let mut int_alt = crate::future_value(rate, periods, present_value - future_value);
    dbg!(int_alt);
    int_alt *= present_value / (present_value - future_value);
    dbg!(int_alt);
    let pmt_alt_3 = (int_alt + (present_value - future_value)) / periods as f64;
    dbg!(pmt_alt_3);

    let rate_multiplier = 1.0 + rate;
    let num = -1.0 * present_value * (rate_multiplier.powi(periods as i32) + future_value);
    dbg!(num);
    assert!(num.is_finite());
    let denom = (rate_multiplier.powi(periods as i32) - 1.0) / rate;
    dbg!(denom);
    assert!(denom.is_finite());
    assert!(denom.is_normal());
    let payment = num / denom;
    dbg!(payment);
    assert!(payment.is_finite());
    payment
}

pub fn payment_solution<P, F>(rate: f64, periods: u32, present_value: P, future_value: F) -> TvmCashflowSolution
    where
        P: Into<f64> + Copy,
        F: Into<f64> + Copy
{
    let payment = payment(rate, periods, present_value, future_value);

    let rate_multiplier = 1.0 + rate;
    let present_value = present_value.into();
    let future_value = future_value.into();

    let formula = if periods == 0 {
        // This is an edge case. It's OK for periods to be zero as long as there's no change between
        // the present and future values.
        assert_approx_equal!(present_value, future_value);
        format!("{:.4}", 0.0)
    } else if rate == 0.0 {
        // There is no interest so the payment is simply the total amount to be paid (the difference
        // between the present and future value) divided by the number of periods. We subtract
        // present value from future value because if present_value is higher than future value the
        // payments should be negative.
        if future_value == 0.0 {
            format!("{:.4} / {}", 0.0 - present_value, periods)
        } else if present_value == 0.0 {
            format!("{:.4} / {}", future_value, periods)
        } else if present_value < 0.0 {
            format!("({:.4} + {:.4}) / {}", future_value, 0.0 - present_value, periods)
        } else {
            format!("({:.4} - {:.4}) / {}", future_value, present_value, periods)
        }
    } else if future_value == 0.0 {
        // We can slightly simplify the formula by not including the future value term.
        // Reverse the sign of the present value so we can avoid flipping the entire result.
        format!("({:.4} * ({:.6} ^ {})) / (({:.6} ^ {}) - 1) / {:.6})", 0.0 - present_value, rate_multiplier, periods, rate_multiplier, periods, rate)
    } else {
        // The usual case. Reverse the signs of the present and future values so we can avoid
        // flipping the entire result.
        let add_future_value = if future_value > 0.0 {
            // The future value is positive but we'll show it as negative in the formula. Rather
            // than something like "+ -100.00" we'll simply say "- 100.00".
            format!(" - {:.4}", future_value)
        } else {
            // The future value is zero or negative but we're flipping the sign in the formula so
            // it should show as something like "+ 100.00".
            format!(" + {:.4}", 0.0 - future_value)
        };
        format!("({:.4} * (({:.6} ^ {}){})) / (({:.6} ^ {}) - 1) / {:.6})", 0.0 - present_value, rate_multiplier, periods, add_future_value, rate_multiplier, periods, rate)
    };
    let cashflow = payment;
    let cashflow_0 = 0_f64;
    // let total_payment = pmt * periods;
    // let total_interest_amount = total_payment - (fv + pv);
    //sum_payment, total_interest_amount,   <---- consider adding something like this
    let rate_schedule = Schedule::new_repeating(ValueType::Rate, rate, periods);
    TvmCashflowSolution::new(TvmCashflowVariable::Payment, rate_schedule, periods, cashflow, cashflow_0, present_value.into(), future_value, payment, &formula)
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
        /*
        assert_rounded_6!(-11.9636085342686, payment(0.034, 10, 100.0, 0.0));
        assert_rounded_6!(-8.22683411973293, payment(-0.034, 10, 100.0, 0.0));
        assert_rounded_6!(11.9636085342686, payment(0.034, 10, -100.0, 0.0));
        assert_rounded_6!(-100.097751710655, payment(1.0, 10, 100.0, 0.0));
        assert_rounded_6!(-150.01573028944, payment(1.5, 10, 100.0, 0.0));
        assert_rounded_6!(-10.0, payment(0.0, 10, 100.0, 0.0));
        assert_rounded_6!(-10.00055000825, payment(0.00001, 10, 100.0, 0.0));
        assert_rounded_6!(8.22683411973293, payment(-0.034, 10, -100.0, 0.0));
        */
        assert_rounded_6!(9.82270640070143, payment(0.034, 10, -100, -25));



    }

    #[test]
    fn test_payment_edge() {
        // Zero interest.
        assert_rounded_6!(-10.0, payment(0.0, 10, 100.0, 0.0));
        // Zero periods but it's OK because the present and future value are equal.
        assert_rounded_6!(0.0, payment(0.05, 0, 100.0, 100.0));
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