#![allow(dead_code)]

#[doc(hidden)]
pub fn main() {
    try_future_value();
    try_future_value_series();
    try_future_value_schedule();
    // dbg!(finance::future_value(-0.002, 5000.33, 250));
    
}

fn try_future_value() {
    // expect 1100
    let rate_of_return = 0.05;
    let present_value_1 = 1_047.6190;
    let periods = 1;
    let future_value_1 = finance::future_value(rate_of_return, present_value_1, periods);
    dbg!(&future_value_1);

    // expect 250_000
    let rate_of_return = 0.034;
    let present_value_2 = 211_513.1216;
    let periods = 5;
    let future_value_2 = finance::future_value(rate_of_return, present_value_2, periods);
    dbg!(&future_value_2);

    let rate_of_return = 1.034;
    let present_value_3 = 7_181.0056;
    let periods = 5;
    let future_value_3 = finance::future_value(rate_of_return, present_value_3, periods);
    dbg!(&future_value_3);

    let rate_of_return = 0.03;
    let present_value_4 = 7_181;
    let periods = 5;
    let future_value_4 = finance::future_value(rate_of_return, present_value_4, periods);
    dbg!(&future_value_4);
}

fn try_future_value_series() {
    // expect 1100
    let rate_of_return = 0.05;
    let present_value_1 = 1_047.6190;
    let periods = 1;
    let future_value_1 = finance::future_value_series(rate_of_return, present_value_1, periods);
    dbg!(future_value_1);
    
    // expect 250_000
    let rate_of_return = 0.034;
    let present_value_2 = 211_513.1216;
    let periods = 5;
    let future_value_2 = finance::future_value_series(rate_of_return, present_value_2, periods);
    dbg!(future_value_2);

    // expect 250_000
    let rate_of_return = 1.034;
    let present_value_3 = 7_181.0056;
    let periods = 5;
    let future_value_3 = finance::future_value_series(rate_of_return, present_value_3, periods);
    dbg!(future_value_3);
}

fn try_future_value_schedule() {
    // expect 1147.26
    let rates: Vec<f64> = vec![0.02,0.03,0.04,0.05];
    let present_value_1 = 1_000.;
    let future_value_1 = finance::future_value_schedule(&rates, present_value_1);
    dbg!(future_value_1);
}
