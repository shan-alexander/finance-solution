#![allow(dead_code)]

use log::{warn};

// Import needed for the function references in the Rustdoc comments.
#[allow(unused_imports)]
use crate::*;

const RUN_PAYMENT_INVARIANTS: bool = false;

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
    //bg!(rate, periods, present_value, future_value, due_at_beginning);
    assert!(rate.is_finite(), "The rate must be finite (not NaN or infinity)");
    assert!(rate > -1.0, "The rate must be greater than -1.0 (-100%).");
    if rate > 1.0 {
        warn!("You provided a periodic rate ({}) greater than 1. Are you sure you expect a {}% return?", rate, rate * 100.0);
    }
    assert!(present_value.is_finite(), "The present value must be finite (not NaN or infinity)");
    assert!(future_value.is_finite(), "The present value must be finite (not NaN or infinity)");
    assert!(!(periods == 0 && present_value + future_value != 0.0), "There are no periods and the present value + future value is not zero so there is no way to calculate payments.");

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
    //bg!(num);
    assert!(num.is_finite());
    let mut denom = (rate_mult).powf(periods as f64) - 1.0;
    assert!(denom.is_finite());
    assert!(denom.is_normal());
    if due_at_beginning {
        denom *= rate_mult;
    }
    //bg!(denom);
    assert!(denom.is_finite());
    assert!(denom.is_normal());
    let payment = num / denom;
    //bg!(payment);
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
    let solution = TvmCashflowSolution::new(calculated_field, rate, periods, present_value.into(), future_value, due_at_beginning, payment, &formula, &formula_symbolic);
    if RUN_PAYMENT_INVARIANTS {
        payment_solution_invariant(&solution);
    }
    solution
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

pub(crate) fn payment_series(solution: &TvmCashflowSolution) -> Vec<TvmCashflowPeriod> {
    assert!(solution.calculated_field().is_payment() || solution.calculated_field().is_payment_due());
    let mut series = vec![];
    if solution.future_value() != 0.0 {
        return series;
    }
    let rate = solution.rate();
    let periods = solution.periods();
    let present_value = solution.present_value();
    let due_at_beginning = solution.due_at_beginning();
    let payment = solution.payment();
    let sum_of_payments = solution.sum_of_payments();
    let sum_of_interest = solution.sum_of_interest();
    let mut payments_to_date = 0.0;
    let mut principal_to_date = 0.0;
    let mut interest_to_date = 0.0;
    for period in 1..=periods {
        let principal_remaining_at_start_of_period = present_value + principal_to_date;
        let interest = if solution.due_at_beginning() && period == 1 {
            0.0
        } else {
            -principal_remaining_at_start_of_period * rate
        };
        let principal = payment - interest;
        payments_to_date += payment;
        principal_to_date += principal;
        interest_to_date += interest;
        let payments_remaining = sum_of_payments - payments_to_date;
        let principal_remaining = -(present_value + principal_to_date);
        let interest_remaining = sum_of_interest - interest_to_date;
        let (formula, formula_symbolic) = if due_at_beginning && period == 1 {
            ("0".to_string(), "interest = 0".to_string())
        } else {
            let formula = format!("{:.4} = -({:.4} * {:.6})", interest, principal_remaining_at_start_of_period, rate);
            let formula_symbolic = "interest = -(principal * rate)".to_string();
            (formula, formula_symbolic)
        };
        let entry = TvmCashflowPeriod::new(period, rate, due_at_beginning, payment, payments_to_date,
            payments_remaining, principal, principal_to_date, principal_remaining, interest,
            interest_to_date, interest_remaining, formula, formula_symbolic);
        series.push(entry);
    }
    if RUN_PAYMENT_INVARIANTS {
        payment_series_invariant(solution, &series);
    }
    series
}

fn payment_solution_invariant(solution: &TvmCashflowSolution) {
    let calculated_field = solution.calculated_field();
    let rate = solution.rate();
    let periods = solution.periods();
    let present_value = solution.present_value();
    let future_value = solution.future_value();
    let due_at_beginning = solution.due_at_beginning();
    let payment = solution.payment();
    let sum_of_payments = solution.sum_of_payments();
    let sum_of_interest = solution.sum_of_interest();
    let formula = solution.formula();
    let formula_symbolic = solution.formula_symbolic();
    let present_and_future_value= present_value + future_value;
    if due_at_beginning {
        assert!(calculated_field.is_payment_due());
    } else {
        assert!(calculated_field.is_payment());
    }
    assert!(rate.is_finite());
    assert!(present_value.is_finite());
    assert!(future_value.is_finite());
    assert!(payment.is_finite());
    if future_value == 0.0 {
        if present_value == 0.0 {
            assert_eq!(0.0, payment);
        } else if present_value.is_sign_positive() {
            assert!(payment.is_sign_negative());
        } else if present_value.is_sign_negative() {
            assert!(payment.is_sign_positive());
        }
    }
    assert!(sum_of_payments.is_finite());
    assert_approx_equal!(sum_of_payments, payment * periods as f64);
    if future_value == 0.0 {
        assert_same_sign_or_zero!(sum_of_interest, sum_of_payments);
    }
    if periods > 0 && rate != 0.0 && future_value == 0.0 && !is_approx_equal!(0.0, present_value) {
        assert!(sum_of_interest.abs() < sum_of_payments.abs());
    }
    assert_approx_equal!(sum_of_interest, sum_of_payments + present_and_future_value);
    assert!(formula.len() > 0);
    assert!(formula_symbolic.len() > 0);
}

fn payment_series_invariant(solution: &TvmCashflowSolution, series: &[TvmCashflowPeriod]) {
    let periods = solution.periods();
    if solution.future_value() != 0.0 {
        assert_eq!(0, series.len());
    } else {
        assert_eq!(periods as usize, series.len());
    }
    if series.len() == 0 {
        return;
    }
    let rate = solution.rate();
    let present_value = solution.present_value();
    let due_at_beginning = solution.due_at_beginning();
    if due_at_beginning {
        assert!(solution.calculated_field().is_payment_due());
    } else {
        assert!(solution.calculated_field().is_payment());
    }
    let payment = solution.payment();
    let sum_of_payments = solution.sum_of_payments();
    let sum_of_interest = solution.sum_of_interest();
    let mut running_sum_of_payments = 0.0;
    let mut running_sum_of_principal = 0.0;
    let mut running_sum_of_interest = 0.0;
    let mut previous_principal: Option<f64> = None;
    let mut previous_interest: Option<f64> = None;
    for (index, entry) in series.iter().enumerate() {
        running_sum_of_payments += entry.payment();
        running_sum_of_principal += entry.principal();
        running_sum_of_interest += entry.interest();
        assert_eq!(rate, entry.rate());
        assert_eq!(index + 1, entry.period() as usize);
        assert_eq!(payment, entry.payment());
        assert_approx_equal!(running_sum_of_payments, entry.payments_to_date());
        assert_approx_equal!(sum_of_payments - running_sum_of_payments, entry.payments_remaining());
        if present_value == 0.0 || rate == 0.0 || (due_at_beginning && index == 0) {
            assert_eq!(payment, entry.principal());
            assert_eq!(0.0, entry.interest());
        } else {
            if present_value > 0.0 {
                assert!(entry.principal() < 0.0);
                assert!(entry.interest() < 0.0);
            } else {
                if entry.principal() <= 0.0 {
                    dbg!(&solution, &series[..10]);
                }
                assert!(entry.principal() > 0.0);
                assert!(entry.interest() > 0.0);
            }
        }
        if index > 0 && previous_interest.unwrap() != 0.0 {
            // Compared to the previous period the principal should be further from zero and the
            // interest should be further toward zero. There's a special case where payments are due
            // at the beginning and we're currently on the second entry. In this case the previous
            // entry will have had zero interest and a principal matching the full payment amount,
            // so the two assertions below wouldn't make sense.
            assert!(entry.principal().abs() > previous_principal.unwrap().abs());
            assert!(entry.interest().abs() < previous_interest.unwrap().abs());
        }
        assert_approx_equal!(running_sum_of_principal, entry.principal_to_date());
        assert_approx_equal!(-present_value - running_sum_of_principal, entry.principal_remaining());
        assert_approx_equal!(running_sum_of_interest, entry.interest_to_date());
        assert_approx_equal!(sum_of_interest - running_sum_of_interest, entry.interest_remaining());
        assert_approx_equal!(payment, entry.principal() + entry.interest());
        if index == periods as usize - 1 {
            // This is the entry for the last period.
            assert_approx_equal!(0.0, entry.payments_remaining());
            if !is_approx_equal!(0.0, entry.principal_remaining()) {
                dbg!(&solution, &series[..10], &series[250], &series[490..500]);
            }
            assert_approx_equal!(0.0, entry.principal_remaining());
            assert_approx_equal!(0.0, entry.interest_remaining());
        }
        assert!(entry.formula().len() > 0);
        assert!(entry.formula_symbolic().len() > 0);

        previous_principal = Some(entry.principal());
        previous_interest = Some(entry.interest());
    }
    assert_approx_equal!(running_sum_of_payments, sum_of_payments);
    assert_approx_equal!(running_sum_of_principal, -present_value);
    assert_approx_equal!(running_sum_of_interest, sum_of_interest);
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

    /*
    #[test]
    fn test_payment_edge() {
        // Zero interest.
        assert_rounded_6!(-10.0, payment(0.0, 10, 100.0, 0.0));
        // Zero periods but it's OK because the present and future value are equal.
        assert_rounded_6!(0.0, payment(0.05, 0, 100.0, 100.0));
    }
    */

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

    /*
    #[test]
    fn test_combinations() {
        // let rates = vec![-0.99, -0.5, -0.05, -0.005, 0.0, 0.005, 0.05, 0.5, 1.0, 10.0, 100.0];
        let rates = vec![0.0, 0.005, 0.05, 0.5, 1.0, 10.0, 100.0];
        let periods: Vec<u32> = vec![0, 1, 2, 5, 10, 36, 100, 500];
        let values: Vec<f64> = vec![-1_000_000.0, -1_234.98, -1.0, 0.0, 5.55555, 99_999.99];
        for rate_one in rates.iter() {
            for periods_one in periods.iter() {
                for present_value_one in values.iter() {
                    for future_value_one in values.iter() {
                        for due_at_beginning_one in [false, true].iter() {
                            println!();
                            dbg!(rate_one, periods_one, present_value_one, future_value_one, due_at_beginning_one);
                            if !(*periods_one == 0 && *present_value_one + *future_value_one != 0.0) {
                                let solution = if *due_at_beginning_one {
                                    payment_solution(*rate_one, *periods_one, *present_value_one, *future_value_one)
                                } else {
                                    payment_due_solution(*rate_one, *periods_one, *present_value_one, *future_value_one)
                                };
                                let series = solution.series();
                                //bg!(&solution, &series);
                                // If we're already calling the invariant functions at the end of
                                // payment_solution_internal() and payment_series() there's no point in
                                // running them again.
                                if !RUN_PAYMENT_INVARIANTS {
                                    run_payment_invariants(&solution, &series);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    */

    fn run_payment_invariants(solution: &TvmCashflowSolution, series: &[TvmCashflowPeriod]) {
        // Display the solution and series only if either one fails its invariant.
        let result = std::panic::catch_unwind(|| {
            payment_solution_invariant(&solution);
            payment_series_invariant(&solution, &series);
        });
        //bg!(&result);
        if result.is_err() {
            dbg!(&solution, &series);
            payment_solution_invariant(&solution);
            payment_series_invariant(&solution, &series);
        }
    }

}
