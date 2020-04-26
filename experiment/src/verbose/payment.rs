#![allow(dead_code)]

pub fn main() {
    // try_payment_debug();
    // try_payment_due_debug();
    // try_formulas();
    // try_payment_series();
    // try_payment_due_series();
    // try_test_combination_examples();
    // try_test_against_excel_ipmt_month_1();
    // try_test_against_excel_ipmt_month_2();
    // generate_scenarios_for_excel();
    // find_numerator_failures();
    // find_calculation_failure_curve();
    // dbg!(finance::payment(0.23, 3000, -123_456.7, -12_345.67));
    try_specify_type_1();
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

fn try_payment_series() {
    let years = 1;
    let rate = 0.11 / 12.0;
    let periods = years * 12;
    let present_value = 100_000.0;
    let future_value = 0.0;
    let solution = finance::payment_solution(rate, periods, present_value, future_value);
    dbg!(&solution);
    dbg!(&solution.series());
}

fn try_payment_due_series() {
    let rate =  0.0056;
    let periods = 12;
    let present_value = 20_000.0;
    let future_value = 0.0;
    let solution = finance::payment_due_solution(rate, periods, present_value, future_value);
    dbg!(&solution);
    dbg!(&solution.series());
}

fn try_test_combination_examples() {
    let rate =  -0.99;
    let periods = 1;
    let present_value = -10;
    let future_value = -10;
    let solution = finance::payment_solution(rate, periods, present_value, future_value);
    dbg!(&solution);
    dbg!(&solution.series());
}

fn try_test_against_excel_ipmt_month_1() {
    // First case in test_payment_nominal() in src/payment.rs in the finance project:
    /*
    let rate = 0.034;
    let periods = 10;
    let present_value = 100.0;
    let future_value = 0.0;
    let exp_payment = -11.9636085342686f64;
    */
    // Case in test_against_excel_ipmt_month_1() in tests/payment.rs in the finance project:
    let rate = 0.0056;
    let periods = 12;
    let present_value = 20_000.0;
    let future_value = 0.0;
    let exp_payment = -1_727.95439349254;

    let payment_1 = finance::payment(rate, periods, present_value, future_value);
    finance::assert_approx_equal!(exp_payment, payment_1);
    let solution = finance::payment_solution(rate, periods, present_value, future_value);
    dbg!(&solution);
    finance::assert_approx_equal!(exp_payment, solution.payment);
}

fn try_test_against_excel_ipmt_month_2() {
    let rate = 0.0056;
    let periods = 12;
    let present_value = 20_000.0;
    let future_value = 0.0;
    let exp_payment = -1_718.11298960604;
    let solution = finance::payment_due_solution(rate, periods, present_value, future_value);
    dbg!(&solution);
    finance::assert_approx_equal!(exp_payment, solution.payment);
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

fn find_calculation_failure_curve() {

    let mut periods = 100;
    while periods < 10_000 {
        let mut rate: f64 = 0.00001;
        // let mut rate: f64 = 0.5;
        let mut last_good_rate: f64 = std::f64::NAN;
        while (rate + 1.0).powi(periods).is_finite() {
            last_good_rate = rate;
            //dbg!(periods, last_good_rate, (rate + 1.0).powi(periods));
            rate *= 2.0;
        }
        // println!();
        // dbg!(periods, last_good_rate);
        // println!();

        let mut low = last_good_rate;
        assert!(low.is_finite());
        let mut high = rate;
        if high.is_finite() && low < 100.0 {
            assert!(low < high);
            assert!((low + 1.0).powi(periods).is_finite());
            assert!(!(high + 1.0).powi(periods).is_finite());
            while high - low > 0.000001 {
                let mid = (low + high) / 2.0;
                assert!(low.is_finite());
                assert!(mid.is_finite());
                assert!(high.is_finite());
                // dbg!(low, mid, high, (mid + 1.0).powi(periods));
                // if (mid + 1.0).powi(periods).is_finite() {
                // if (mid + 1.0).powf(periods as f64).is_finite() {
                if limit_calculation(periods, mid, false).is_finite() {
                    // Continue testing above the midpoint.
                    low = mid;
                } else {
                    // Continue testing below the midpoint.
                    high = mid;
                }
            }
            // println!();
            // println!("periods = {}, rate = {}", periods, low);
            println!("{}\t{}", periods, low);
            // println!();
            assert!((low + 1.0).powi(periods).is_finite());
            // break;
        }
        // periods *= 2;
        periods += 100;
    }
}

fn limit_calculation(periods: i32, rate: f64, due_at_beginning: bool) -> f64 {
    let present_value = 100.0;
    let future_value = 10.0;
    let rate_mult = 1.0 + rate;
    let num= ((present_value * rate_mult.powf(periods as f64)) + future_value) * -rate;
    if !num.is_finite() {
        return num;
    }
    let mut denom = (rate_mult).powf(periods as f64) - 1.0;
    if !denom.is_finite() {
        return denom;
    }
    if due_at_beginning {
        denom *= rate_mult;
    }
    if !denom.is_finite() {
        return denom;
    }
    let payment = num / denom;
    payment
}

macro_rules! specify_type_1 {
    ($e:expr, f64) => {
        println!("try_specify_type: f64 = {}", $e as f64);
    };
    ($e:expr, i32) => {
        println!("try_specify_type: i32 = {}", $e);
    };
}

fn try_specify_type_1() {
    specify_type_1!(123.45, f64);
    specify_type_1!(555, f64);
}

