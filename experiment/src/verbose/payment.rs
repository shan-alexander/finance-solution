pub fn main() {
    // try_payment_debug();
    try_payment_due_debug();
    // try_formulas();
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

