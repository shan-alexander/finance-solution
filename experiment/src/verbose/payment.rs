#![allow(dead_code)]

pub fn main() {
    // try_payment_debug();
    // try_payment_due_debug();
    // try_formulas();
    generate_scenarios_for_excel();
    // find_numerator_failures();
    // dbg!(finance::payment(0.23, 3000, -123_456.7, -12_345.67));
}

fn try_payment_debug() {
    let pv_positive_fv_zero = finance::payment_solution(0.034, 10, 100.0, 0.0);
    dbg!(pv_positive_fv_zero);
    println!();

    let pv_zero_fv_positive = finance::payment_solution(0.034, 10, 0.0, 100.0);
    dbg!(pv_zero_fv_positive);
    println!();

    let pv_positive_fv_positive = finance::payment_solution(0.034, 10, 100.0, 25.0);
    dbg!(pv_positive_fv_positive);
    println!();

    let pv_negative_fv_zero = finance::payment_solution(0.034, 10, -100.0, 0.0);
    dbg!(pv_negative_fv_zero);
    println!();

    let pv_negative_fv_negative = finance::payment_solution(0.034, 10, -100.0, -25.0);
    dbg!(pv_negative_fv_negative);
    println!();

    let rate_zero_pv_positive_fv_zero = finance::payment_solution(0.0, 10, 100.0, 0.0);
    dbg!(rate_zero_pv_positive_fv_zero);
    println!();

    let rate_zero_pv_positive_fv_positive = finance::payment_solution(0.0, 10, 100.0, 25.0);
    dbg!(rate_zero_pv_positive_fv_positive);
    println!();

    let rate_zero_pv_negative_fv_zero = finance::payment_solution(0.0, 10, -100.0, 0.0);
    dbg!(rate_zero_pv_negative_fv_zero);
    println!();

    let rate_zero_pv_negative_fv_negative = finance::payment_solution(0.0, 10, -100.0, -25.0);
    dbg!(rate_zero_pv_negative_fv_negative);
    println!();

    let rate_negative = finance::payment_solution(-0.034, 10, 100.0, 200.0);
    dbg!(rate_negative);
    println!();

    let periods_zero = finance::payment_solution(0.034, 0, 100.0, 100.0);
    dbg!(periods_zero);
    println!();
}

fn try_payment_due_debug() {
    let pv_positive_fv_zero = finance::payment_due_solution(0.034, 10, 100.0, 0.0);
    dbg!(pv_positive_fv_zero);
    println!();

    let pv_zero_fv_positive = finance::payment_due_solution(0.034, 10, 0.0, 100.0);
    dbg!(pv_zero_fv_positive);
    println!();

    let pv_positive_fv_positive = finance::payment_due_solution(0.034, 10, 100.0, 25.0);
    dbg!(pv_positive_fv_positive);
    println!();

    let pv_negative_fv_zero = finance::payment_due_solution(0.034, 10, -100.0, 0.0);
    dbg!(pv_negative_fv_zero);
    println!();

    let pv_negative_fv_negative = finance::payment_due_solution(0.034, 10, -100.0, -25.0);
    dbg!(pv_negative_fv_negative);
    println!();

    let rate_zero_pv_positive_fv_zero = finance::payment_due_solution(0.0, 10, 100.0, 0.0);
    dbg!(rate_zero_pv_positive_fv_zero);
    println!();

    let rate_zero_pv_positive_fv_positive = finance::payment_due_solution(0.0, 10, 100.0, 25.0);
    dbg!(rate_zero_pv_positive_fv_positive);
    println!();

    let rate_zero_pv_negative_fv_zero = finance::payment_due_solution(0.0, 10, -100.0, 0.0);
    dbg!(rate_zero_pv_negative_fv_zero);
    println!();

    let rate_zero_pv_negative_fv_negative = finance::payment_due_solution(0.0, 10, -100.0, -25.0);
    dbg!(rate_zero_pv_negative_fv_negative);
    println!();

    let rate_negative = finance::payment_due_solution(-0.034, 10, 100.0, 200.0);
    dbg!(rate_negative);
    println!();

    let periods_zero = finance::payment_due_solution(0.034, 0, 100.0, 100.0);
    dbg!(periods_zero);
    println!();
}

fn try_formulas() {
    let pv_positive_fv_positive = finance::payment_solution(0.034, 10, 100.0, 25.0);
    dbg!(&pv_positive_fv_positive);
    // The formula is "(((100.0000 * 1.034000^10) + 25.0000) * -0.034000) / (1.034000^10 - 1)".
    let formula_result = (((100.0000 * 1.034000f64.powi(10)) + 25.0000) * -0.034000) / (1.034000f64.powi(10) - 1.0);
    dbg!(formula_result);
    finance::assert_rounded_6!(formula_result, pv_positive_fv_positive.payment);
    println!();

    let pv_positive_fv_zero = finance::payment_solution(0.034, 10, 100.0, 0.0);
    dbg!(&pv_positive_fv_zero);
    // The formula is "((100.0000 * 1.034000^10) * -0.034000) / (1.034000^10 - 1)".
    let formula_result = ((100.0000 * 1.034000f64.powi(10)) * -0.034000) / (1.034000f64.powi(10) - 1.0);
    dbg!(formula_result);
    finance::assert_rounded_6!(formula_result, pv_positive_fv_zero.payment);
    println!();
}

fn generate_scenarios_for_excel() {
    let ratio = 5;
    let rate_list = [-0.1, -0.01, -0.001, 0.0, 0.0023, 0.023, 0.23];
    let periods_list = [0, 1, 2, 5, 10, 50, 250];
    let value_list = [-12_345.67, -123.4567, -1.234567, 0.0, 1.234567, 123.4567, 12_345.67];
    let mut counter = 0;
    for rate in rate_list.iter() {
        for periods in periods_list.iter() {
            for present_value in value_list.iter() {
                for future_value in value_list.iter() {
                    if !(*periods == 0 && *present_value != *future_value) {
                        // dbg!(finance::payment_solution(*rate, *periods, *present_value, *future_value));
                        for due_at_beginning in [0, 1].iter() {
                            if counter % ratio == 0 {
                                println!("{}\t{}\t{}\t{}\t{}", *rate, *periods, *present_value, *future_value, *due_at_beginning);
                            }
                            counter += 1;
                        }
                    }
                }
            }
        }
    }
}

fn find_numerator_failures() {
    let rate_list = [-0.9999, -0.5, -0.1, -0.01, -0.001, 0.0, 0.0023, 0.023, 0.23, 2.3, 23.0];
    let periods_list = [0, 1, 2, 5, 10, 20, 50, 100, 200, 500, 1_000, 2_000, 5_000, 10_000];
    let present_value = 100.0;
    let future_value = 0.0;
    for rate in rate_list.iter() {
        let rate_mult: f64 = 1.0 + *rate;
        for periods in periods_list.iter() {
            let num = ((present_value * rate_mult.powf(*periods as f64)) + future_value) * -*rate;
            if !num.is_finite() {
                println!("rate = {}, periods = {}", rate, periods);
            }
        }
    }
}

