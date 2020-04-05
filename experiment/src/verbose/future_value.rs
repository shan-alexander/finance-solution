#![allow(dead_code)]

#[doc(hidden)]
pub fn main() {
    // try_future_value();
    // try_future_value_series();
    // try_future_value_schedule();
    // dbg!(finance::future_value(-0.002, 5000.33, 250));
    // try_doc_example_solution();
    try_doc_example_series();
    // try_doc_example_schedule();
}

fn try_future_value() {
    // expect 1100
    let rate_of_return = 0.05;
    let present_value_1 = 1_047.6190;
    let periods = 1;
    let future_value_1 = finance::future_value(rate_of_return, periods, present_value_1);
    dbg!(&future_value_1);

    // expect 250_000
    let rate_of_return = 0.034;
    let present_value_2 = 211_513.1216;
    let periods = 5;
    let future_value_2 = finance::future_value(rate_of_return, periods, present_value_2);
    dbg!(&future_value_2);

    let rate_of_return = 1.034;
    let present_value_3 = 7_181.0056;
    let periods = 5;
    let future_value_3 = finance::future_value(rate_of_return, periods, present_value_3);
    dbg!(&future_value_3);

    let rate_of_return = 0.03;
    let present_value_4 = 7_181;
    let periods = 5;
    let future_value_4 = finance::future_value(rate_of_return, periods, present_value_4);
    dbg!(&future_value_4);
}

fn try_future_value_series() {
    // expect 1100
    let rate_of_return = 0.05;
    let present_value_1 = 1_047.6190;
    let periods = 1;
    let future_value_1 = finance::future_value_series(rate_of_return, periods, present_value_1);
    dbg!(future_value_1);
    
    // expect 250_000
    let rate_of_return = 0.034;
    let present_value_2 = 211_513.1216;
    let periods = 5;
    let future_value_2 = finance::future_value_series(rate_of_return, periods, present_value_2);
    dbg!(future_value_2);

    // expect 250_000
    let rate_of_return = 1.034;
    let present_value_3 = 7_181.0056;
    let periods = 5;
    let future_value_3 = finance::future_value_series(rate_of_return, periods, present_value_3);
    dbg!(future_value_3);
}

fn try_future_value_schedule() {
    // expect 1147.26
    let rates: Vec<f64> = vec![0.02,0.03,0.04,0.05];
    let present_value_1 = 1_000.;
    let future_value_1 = finance::future_value_schedule(&rates, present_value_1);
    dbg!(future_value_1);
}

fn try_doc_example_solution() {
    // The initial investment is $100,000.
    let present_value = 100_000;
    // The investment will grow for 12 years.
    let periods = 12;
    // We'll keep a collection of the calculated future values along with their inputs.
    let mut scenarios = vec![];
    for i in 2..=15 {
        // The rate is between 2% and 15% per year.
        let periodic_rate = i as f64 / 100.0;
        // Calculate the future value for this periodic rate and add the details to the collection.
        scenarios.push(finance::future_value_solution(periodic_rate, present_value, periods));
    }
    dbg!(&scenarios);
    assert_eq!(14, scenarios.len());
    // Keep only the scenarios where the future value was between $200,000 and $400,000.
    scenarios.retain(|x| x.future_value >= 200_000.00 && x.future_value <= 400_000.00);
    dbg!(&scenarios);
    assert_eq!(7, scenarios.len());
    // Check the formula for the first scenario.
    dbg!(&scenarios[0].formula);
    assert_eq!("100000 * (1 + 0.06)^12", scenarios[0].formula);
}

fn try_doc_example_series() {

    // The initial investment is $10,000.12.
    let present_value = 10_000.12;

    // The interest rate is 1.5% per month.
    let interest_rate = 0.015;

    // The investment will grow for 24 months.
    let periods = 24;
    let periods = finance::future_value_series(interest_rate, present_value, periods);
    dbg!(&periods);

    // Confirm that we have one entry for the initial value and one entry for each period.
    assert_eq!(25, periods.len());

    // Create a reduced vector with every fourth period.
    let filtered_periods = periods
        .iter()
        .filter(|x| x.period % 4 == 0)
        .collect::<Vec<_>>();
    dbg!(&filtered_periods);
    assert_eq!(7, filtered_periods.len());
}

fn try_doc_example_schedule() {

    // The rates vary by year: 11.6% followed by 13.4%, 9%, and 8.6%.
    let rates = [0.116, 0.134, -0.09, 0.086];

    // The initial investment is $50,000.
    let present_value = 50_000.00;

    let schedule = finance::future_value_schedule(&rates, present_value);
    dbg!(&schedule);

    assert_eq!(62534.3257, finance::round_to_fraction_of_cent(schedule.future_value));

    // Confirm that there are four periods, corresponding to the four interest
    // rates.
    assert_eq!(4, schedule.periods.len());

    // Confirm that the value of the fourth period is the same as the overall
    // future value.
    assert_eq!(schedule.future_value, schedule.periods.last().unwrap().value);

    // Find the first period where the value of the investment was at least
    // $60,000.
    let period = schedule.periods.iter().find(|x| x.value >= 60_000.00);
    dbg!(&period);
    assert_eq!(2, period.unwrap().period);
}
