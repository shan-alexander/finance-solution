#![allow(dead_code)]

pub fn main() {
    try_present_value_solution();
    // try_present_value_series()
    // try_present_value();
    try_doc_example();
}

fn try_present_value_solution() {
    // expect 1047.6190
    let rate_of_return = 0.05f64;
    let future_value_solution_1 = 1_100f64;
    let periods = 1;
    let present_value_solution_1 = finance::present_value_solution(rate_of_return, future_value_solution_1, periods);
    dbg!(present_value_solution_1);
    
    // expect 211_513.1216
    let rate_of_return = 0.034f64;
    let future_value_solution_1 = 250_000f64;
    let periods = 5;
    let present_value_solution_2 = finance::present_value_solution(rate_of_return, future_value_solution_1, periods);
    dbg!(present_value_solution_2);

    // expect 7181.0056
    let rate_of_return = 1.034f64;
    let future_value_solution_1 = 250_000f64;
    let periods = 5;
    let present_value_solution_3 = finance::present_value_solution(rate_of_return, future_value_solution_1, periods);
    dbg!(&present_value_solution_3);
    // println!("{:?}", present_value_solution_3); 
    // dbg!(present_value_solution_3.present_value_solution_series());
    
    // expect 7181.0056
    let rate_of_return = 3.034f64;
    let future_value_solution_1 = 250_000f64;
    let periods = 12;
    let present_value_solution_3 = finance::present_value_solution(rate_of_return, future_value_solution_1, periods);
    dbg!(&present_value_solution_3);
    
    // expect 7181.0056
    let rate_of_return = -3.034f64;
    let future_value_solution_1 = 250_000f64;
    let periods = 12;
    let present_value_solution_4 = finance::present_value_solution(rate_of_return, future_value_solution_1, periods);
    dbg!(&present_value_solution_4);

}

fn try_present_value() {
    // expect 211_513.1216
    let rate_of_return = 0.034;
    let future_value_1 = 250_000;
    let periods = 5;
    let present_value_1 = finance::present_value(rate_of_return, periods, future_value_1);
    dbg!(present_value_1);
}

fn try_present_value_series() {
    // expect 1047.6190
    let rate_of_return = 0.05;
    let future_value_1 = 1_100;
    let periods = 1;
    let present_value_1 = finance::present_value_series(rate_of_return, future_value_1, periods);
    dbg!(present_value_1);
    
    // expect 211_513.1216
    let rate_of_return = 0.034;
    let future_value_1 = 250_000;
    let periods = 5;
    let present_value_2 = finance::present_value_series(rate_of_return, future_value_1, periods);
    dbg!(present_value_2);

    // expect 7181.0056
    let rate_of_return = 1.034;
    let future_value_1 = 250_000;
    let periods = 5;
    let present_value_3 = finance::present_value_series(rate_of_return, future_value_1, periods);
    dbg!(present_value_3);
}

fn try_doc_example() { 
    // The investment grows by 1.1% per month.
    let periodic_rate = 1.1;
    
    // The final value will be $50,000.
    let future_value = 50_000;
        
    // The investment will grow for 12 months.
    let periods = 5;
    
    let present_value = finance::present_value(periodic_rate, future_value, periods);

    dbg!(&present_value);
    // Confirm that the present value is correct to four decimal places (one
    // hundredth of a cent).
    // assert_eq!(295_489.9418, finance::round_to_fraction_of_cent(present_value));
}
