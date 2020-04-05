#![allow(dead_code)]

pub fn main() {
    // try_present_value_solution();
    // try_present_value_series();
    // try_present_value();
    // try_present_value_schedule();
    // try_doc_example();
    // try_doc_example_solution();
    // try_doc_example_series();
    try_doc_example_schedule();
}

fn try_present_value_solution() {
    // expect 1047.6190
    let rate_of_return = 0.05f64;
    let periods = 1;
    let future_value_solution_1 = 1_100f64;
    let present_value_solution_1 = finance::present_value_solution(rate_of_return, periods, future_value_solution_1);
    dbg!(present_value_solution_1);
    
    // expect 211_513.1216
    let rate_of_return = 0.034f64;
    let periods = 5;
    let future_value_solution_1 = 250_000f64;
    let present_value_solution_2 = finance::present_value_solution(rate_of_return, periods, future_value_solution_1);
    dbg!(present_value_solution_2);

    // expect 7181.0056
    let rate_of_return = 1.034f64;
    let periods = 5;
    let future_value_solution_1 = 250_000f64;
    let present_value_solution_3 = finance::present_value_solution(rate_of_return, periods, future_value_solution_1);
    dbg!(&present_value_solution_3);
    // println!("{:?}", present_value_solution_3); 
    // dbg!(present_value_solution_3.present_value_solution_series());
    
    // expect 7181.0056
    let rate_of_return = 3.034f64;
    let periods = 12;
    let future_value_solution_1 = 250_000f64;
    let present_value_solution_3 = finance::present_value_solution(rate_of_return, periods, future_value_solution_1);
    dbg!(&present_value_solution_3);
}

fn try_present_value() {
    // expect 211_513.1216
    let rate_of_return = 0.034;
    let periods = 5;
    let future_value_1 = 250_000;
    let present_value_1 = finance::present_value(rate_of_return, periods, future_value_1);
    dbg!(present_value_1);
}

fn try_present_value_series() {
    // expect 1047.6190
    let rate_of_return = 0.05;
    let periods = 1;
    let future_value_1 = 1_100;
    let present_value_1 = finance::present_value_series(rate_of_return, periods, future_value_1);
    dbg!(present_value_1);
    
    // expect 211_513.1216
    let rate_of_return = 0.034;
    let future_value_1 = 250_000;
    let periods = 5;
    let present_value_2 = finance::present_value_series(rate_of_return, periods, future_value_1);
    dbg!(present_value_2);

    // expect 7181.0056
    let rate_of_return = 1.034;
    let periods = 5;
    let future_value_1 = 250_000;
    let present_value_3 = finance::present_value_series(rate_of_return, periods, future_value_1);
    dbg!(present_value_3);
}

fn try_present_value_schedule() {
    let rates = [0.04, 0.07, -0.12, -0.03, 0.11];
    let future_value = 100_000.25;
    let schedule = finance::present_value_schedule(&rates, future_value);
    dbg!(&schedule);
}

fn try_doc_example() {
    // The investment will grow by 1.1% per month.
    let periodic_rate = 0.011;

    // The investment will grow for 12 months.
    let periods = 12;

    // The final value will be $50,000.
    let future_value = 50_000;

    // Find the current value.
    let present_value = finance::present_value(periodic_rate, periods, future_value);

    dbg!(&present_value);

    // Confirm that the present value is correct to four decimal places (one hundredth of a cent).
    assert_eq!(43848.6409, finance::round_to_fraction_of_cent(present_value));
}

fn try_doc_example_solution() {

    // The rate is 0.9% per month.
    let periodic_rate = 0.009;

    // The final value is $100,000.
    let future_value = 100_000;

    // We'll keep a collection of the calculated present values along with their inputs.
    let mut scenarios = vec![];
    // Calculate the present value for terms ranging from 1 to 36 months.
    for periods in 1..=36 {
        // Calculate the future value for this number of months and add the details to the
        // collection.
        scenarios.push(finance::present_value_solution(periodic_rate, periods, future_value));
    }
    dbg!(&scenarios);
    assert_eq!(36, scenarios.len());

    // Keep only the scenarios where the present value is less than or equal to $80,000.
    scenarios.retain(|x| x.present_value <= 80_000.00);
    dbg!(&scenarios);
    assert_eq!(12, scenarios.len());

    // Find the range of months for the remaining scenarios.
    let min_months = scenarios.iter().map(|x| x.periods).min().unwrap();
    let max_months = scenarios.iter().map(|x| x.periods).max().unwrap();
    dbg!(min_months, max_months);
    assert_eq!(25, min_months);
    assert_eq!(36, max_months);

    // Check the formula for the first scenario.
    dbg!(&scenarios[0].formula);
    assert_eq!("100000 / (1 + 0.009)^25", scenarios[0].formula);
}

fn try_doc_example_series() {

    // The interest rate is 7.8% per year.
    let interest_rate = 0.078;

    // The investment will grow for 10 years.
    let periods = 10;

    // The final value is $8112.75.
    let future_value = 8_112.75;

    // Calculate the present value as well as the value at the end of each year.
    let series = finance::present_value_series(interest_rate, periods, future_value);
    dbg!(&series);

    // Confirm that we have one entry for the present value, that is the initial value before any
    // interest is applied, and one entry for each period.
    assert_eq!(11, series.len());

    // Create a reduced vector with every other period not including period 0,
    // the initial state.
    let filtered_series = series
        .iter()
        .filter(|x| x.period % 2 == 0 && x.period != 0)
        .collect::<Vec<_>>();
    dbg!(&filtered_series);
    assert_eq!(5, filtered_series.len());
}

fn try_doc_example_schedule() {

    // The annual rate varies from -12% to 11%.
    let rates = [0.04, 0.07, -0.12, -0.03, 0.11];

    // The value of the investment after applying all of these periodic rates will be $100_000.25.
    let future_value = 100_000.25;

    // Calculate the present value as well as the value at the end of each period.
    let schedule = finance::present_value_schedule(&rates, future_value);
    dbg!(&schedule);
    assert_eq!(5, schedule.schedule_periods.len());

    // Create a filtered list of periods, only those with a negative rate.
    let filtered_periods = schedule.schedule_periods
        .iter()
        .filter(|x| x.periodic_rate < 0.0)
        .collect::<Vec<_>>();
    dbg!(&filtered_periods);
    assert_eq!(2, filtered_periods.len());
}



