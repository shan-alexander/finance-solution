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
    dbg!(rate, periods, present_value, future_value, due_at_beginning);
    assert!(rate.is_finite(), "The rate must be finite (not NaN or infinity)");
    // assert!(rate >= -1.0, "The rate must be greater than or equal to -1.0 because a rate lower than -100% would mean the investment loses more than its full value in a period.");
    if rate.abs() > 1. {
        warn!("You provided a periodic rate ({}) greater than 1. Are you sure you expect a {}% return?", rate, rate * 100.0);
    }
    assert!(present_value.is_finite(), "The present value must be finite (not NaN or infinity)");
    assert!(future_value.is_finite(), "The present value must be finite (not NaN or infinity)");
    assert!(!(periods == 0 && !is_approx_equal!(present_value, future_value)), "The number of periods is 0 yet the future value doesn't equal the present value. There's no way to calculate payments.");

    if periods == 0 {
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
    dbg!(num);
    assert!(num.is_finite());
    let mut denom = (rate_mult).powf(periods as f64) - 1.0;
    if due_at_beginning {
        denom *= rate_mult;
    }
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
    let (formula, formula_symbolic) = payment_formula(rate, periods, present_value, future_value, due_at_beginning, payment);
    let calculated_field = if due_at_beginning { TvmCashflowVariable::PaymentDue } else { TvmCashflowVariable::Payment };
    TvmCashflowSolution::new(calculated_field, rate, periods, present_value.into(), future_value, due_at_beginning, payment, &formula, &formula_symbolic)
}

fn payment_formula(rate: f64, periods: u32, present_value: f64, future_value: f64, due_at_beginning: bool, payment: f64) -> (String, String) {
    let rate_multiplier = 1.0 + rate;
    let (mut formula, mut formula_symbolic) = if periods == 0 {
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
    };
    formula = format!("{:.4} = {}", payment, formula);
    formula_symbolic = format!("pmt = {}", formula_symbolic);
    (formula, formula_symbolic)
}

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
        assert_approx_equal!(-11.9636085342686f64, payment(0.034, 10, 100.0, 0.0));
        assert_approx_equal!(-8.22683411973293f64, payment(-0.034, 10, 100.0, 0.0));
        assert_approx_equal!(11.9636085342686f64, payment(0.034, 10, -100.0, 0.0));
        assert_approx_equal!(-100.097751710655f64, payment(1.0, 10, 100.0, 0.0));
        assert_approx_equal!(-150.01573028944f64, payment(1.5, 10, 100.0, 0.0));
        assert_approx_equal!(-10f64, payment(0.0, 10, 100.0, 0.0));
        assert_approx_equal!(-10.00055000825f64, payment(0.00001, 10, 100.0, 0.0));
        assert_approx_equal!(8.22683411973293f64, payment(-0.034, 10, -100.0, 0.0));
        assert_approx_equal!(9.82270640070143f64, payment(0.034, 10, -100.0, 25.0));
        assert_approx_equal!(14.1045106678357f64, payment(0.034, 10, -100.0, -25.0));
        assert_approx_equal!(-10f64, payment(0.0, 10, 100.0, 0.0));
        assert_approx_equal!(-11f64, payment(0.0, 10, 100.0, 10.0));
        assert_approx_equal!(-9f64, payment(0.0, 10, 100.0, -10.0));
        assert_approx_equal!(-1f64, payment(0.0, 10, 10.0, 0.0));
        assert_approx_equal!(-11f64, payment(0.0, 10, 10.0, 100.0));
        assert_approx_equal!(9f64, payment(0.0, 10, 10.0, -100.0));
        assert_approx_equal!(10f64, payment(0.0, 10, -100.0, 0.0));
        assert_approx_equal!(9f64, payment(0.0, 10, -100.0, 10.0));
        assert_approx_equal!(11f64, payment(0.0, 10, -100.0, -10.0));
        assert_approx_equal!(1f64, payment(0.0, 10, -10.0, 0.0));
        assert_approx_equal!(-9f64, payment(0.0, 10, -10.0, 100.0));
        assert_approx_equal!(11f64, payment(0.0, 10, -10.0, -100.0));
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
        assert_approx_equal!(-11.5702210196021f64, payment_due(0.034, 10, 100.0, 0.0));
        assert_approx_equal!(-8.51639142829496f64, payment_due(-0.034, 10, 100.0, 0.0));
        assert_approx_equal!(-8.51639142829496f64, payment_due(-0.034, 10, 100.0, 0.0));
        assert_approx_equal!(-50.0488758553275f64, payment_due(1.0, 10, 100.0, 0.0));
        assert_approx_equal!(-60.0062921157762f64, payment_due(1.5, 10, 100.0, 0.0));
        assert_approx_equal!(-10f64, payment_due(0.0, 10, 100.0, 0.0));
        assert_approx_equal!(-10.0004500037499f64, payment_due(0.00001, 10, 100.0, 0.0));
        assert_approx_equal!(8.51639142829496f64, payment_due(-0.034, 10, -100.0, 0.0));
        assert_approx_equal!(9.49971605483697f64, payment_due(0.034, 10, -100.0, 25.0));
        assert_approx_equal!(13.6407259843672f64, payment_due(0.034, 10, -100.0, -25.0));
        assert_approx_equal!(-10f64, payment_due(0.0, 10, 100.0, 0.0));
        assert_approx_equal!(-11f64, payment_due(0.0, 10, 100.0, 10.0));
        assert_approx_equal!(-9f64, payment_due(0.0, 10, 100.0, -10.0));
        assert_approx_equal!(-1f64, payment_due(0.0, 10, 10.0, 0.0));
        assert_approx_equal!(-11f64, payment_due(0.0, 10, 10.0, 100.0));
        assert_approx_equal!(9f64, payment_due(0.0, 10, 10.0, -100.0));
        assert_approx_equal!(10f64, payment_due(0.0, 10, -100.0, 0.0));
        assert_approx_equal!(9f64, payment_due(0.0, 10, -100.0, 10.0));
        assert_approx_equal!(11f64, payment_due(0.0, 10, -100.0, -10.0));
        assert_approx_equal!(1f64, payment_due(0.0, 10, -10.0, 0.0));
        assert_approx_equal!(-9f64, payment_due(0.0, 10, -10.0, 100.0));
        assert_approx_equal!(11f64, payment_due(0.0, 10, -10.0, -100.0));
    }

    /*
    #[should_panic]
    #[test]
    fn test_future_value_error_rate_low() {
        future_value(-101.0, 5, 250_000.00);
    }
    */

    #[test]
    fn test_internal_consistency() {
        let rates = vec![-1.0, -0.5, -0.05, -0.005, 0.0, 0.005, 0.05, 0.5, 1.0, 10.0, 100.0];
        let periods: Vec<u32> = vec![0, 1, 2, 5, 10, 36, 100, 1_000];
        let values: Vec<f64> = vec![-1_000_000.0, -1_234.98, -1.0, 0.0, 5.55555, 99_999.99];
        for rate_one in rates.iter() {
            for periods_one in periods.iter() {
                for present_value_one in values.iter() {
                    for future_value_one in values.iter() {
                        // if !(*periods_one > 50 && *rate_one > 0.01) {

                            assert_internal_consistency(&solution);
                        //}
                    }
                }
            }
        }
    }

    fn assert_internal_consistency(solution: &TvmCashflowSolution) {

    }

}
