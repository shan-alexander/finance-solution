pub fn main() {
    try_debug();
}

fn try_debug() {
    let pv_positive_fv_zero = finance::payment_solution(0.034, 10, 100.0, 0.0);
    println!();
    dbg!(pv_positive_fv_zero);

    let pv_positive_fv_positive = finance::payment_solution(0.034, 10, 100.0, 25.0);
    println!();
    dbg!(pv_positive_fv_positive);

    let pv_negative_fv_zero = finance::payment_solution(0.034, 10, -100.0, 0.0);
    println!();
    dbg!(pv_negative_fv_zero);

    let pv_negative_fv_negative = finance::payment_solution(0.034, 10, -100.0, -25.0);
    println!();
    dbg!(pv_negative_fv_negative);

    let rate_zero_pv_positive_fv_zero = finance::payment_solution(0.0, 10, 100.0, 0.0);
    println!();
    dbg!(rate_zero_pv_positive_fv_zero);

    let rate_zero_pv_positive_fv_positive = finance::payment_solution(0.0, 10, 100.0, 25.0);
    println!();
    dbg!(rate_zero_pv_positive_fv_positive);

    let rate_zero_pv_negative_fv_zero = finance::payment_solution(0.0, 10, -100.0, 0.0);
    println!();
    dbg!(rate_zero_pv_negative_fv_zero);

    let rate_zero_pv_negative_fv_negative = finance::payment_solution(0.0, 10, -100.0, -25.0);
    println!();
    dbg!(rate_zero_pv_negative_fv_negative);

    let rate_negative = finance::payment_solution(-0.034, 10, 100.0, 200.0);
    println!();
    dbg!(rate_negative);

    let periods_zero = finance::payment_solution(0.034, 0, 100.0, 100.0);
    println!();
    dbg!(periods_zero);

    /*
    dbg!(finance::payment_solution(-0.034, 10, 100.0, 0.0));
    dbg!(finance::payment_solution(0.034, 10, -100.0, 0.0));
    dbg!(finance::payment_solution(1.0, 10, 100.0, 0.0));
    dbg!(finance::payment_solution(1.5, 10, 100.0, 0.0));
    dbg!(finance::payment_solution(0.0, 10, 100.0, 0.0));
    dbg!(finance::payment_solution(0.00001, 10, 100.0, 0.0));
    dbg!(finance::payment_solution(-0.034, 10, -100.0, 0.0));
    */
}
