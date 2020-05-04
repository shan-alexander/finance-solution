#![allow(dead_code)]

#[doc(hidden)]
pub fn main() {
    // try_future_value();
    // try_future_value_series();
    // try_future_value_schedule();
    // try_doc_example_solution_1();
    // try_doc_example_solution_2();
    // try_doc_example_series();
    // try_doc_example_schedule();
    // try_doc_example_schedule_solution();
    // try_doc_example_schedule_series();
    // try_find_rate();
    // try_series_print_table();
    // check_formulas();
    // dbg!(finance::future_value_solution(0.012, 8, 200_000));
    try_continuous();
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
    let future_value_1 = finance::future_value_solution(rate_of_return, periods, present_value_1).series();
    dbg!(future_value_1);
    
    // expect 250_000
    let rate_of_return = 0.034;
    let present_value_2 = 211_513.1216;
    let periods = 5;
    let future_value_2 = finance::future_value_solution(rate_of_return, periods, present_value_2).series();
    dbg!(future_value_2);

    // expect 250_000
    let rate_of_return = 1.034;
    let present_value_3 = 7_181.0056;
    let periods = 5;
    let future_value_3 = finance::future_value_solution(rate_of_return, periods, present_value_3).series();
    dbg!(future_value_3);
}

fn try_future_value_schedule() {
    // expect 1147.26
    let rates: Vec<f64> = vec![0.02,0.03,0.04,0.05];
    let present_value = 1_000.;

    let future_value = finance::future_value_schedule(&rates, present_value);
    dbg!(&future_value);

    let solution = finance::future_value_schedule_solution(&rates, present_value);
    dbg!(&solution);
    dbg!(solution.series());
}

fn try_doc_example_solution_1() {

    // The rate is 1.2% per month.
    let rate = 0.012;

    // The investment will grow for 8 months.
    let periods = 8;

    // The initial investment is $200,000.
    let present_value = 200_000;

    let solution = finance::future_value_solution(rate, periods, present_value);
    dbg!(&solution);

    let future_value = solution.future_value();
    finance::assert_rounded_4(future_value, 220_026.0467);

    let formula = solution.formula();
    assert_eq!(formula, "200000.0000 * (1.012000 ^ 8)");

    // Calculate the value at the end of each period.
    let series = solution.series();
    dbg!(&series);
}

fn try_doc_example_solution_2() {

    // The initial investment is $100,000.
    let present_value = 100_000;

    // The investment will grow for 12 years.
    let periods = 12;

    // We'll keep a collection of the calculated future values along with their inputs.
    let mut scenarios = vec![];

    for i in 2..=15 {
        // The rate is between 2% and 15% per year.
        let rate = i as f64 / 100.0;
        // Calculate the future value for this periodic rate and add the details to the collection.
        scenarios.push(finance::future_value_solution(rate, periods, present_value));
    }
    dbg!(&scenarios);
    assert_eq!(14, scenarios.len());

    // Keep only the scenarios where the future value was between $200,000 and $400,000.
    scenarios.retain(|x| x.future_value() >= 200_000.00 && x.future_value() <= 400_000.00);
    dbg!(&scenarios);
    assert_eq!(7, scenarios.len());

    // Check the formula for the first scenario.
    dbg!(scenarios[0].formula());
    assert_eq!("100000.0000 * (1.060000 ^ 12)", scenarios[0].formula());
}

fn try_doc_example_series() {

    // The initial investment is $10,000.12.
    let present_value = 10_000.12;

    // The interest rate is 1.5% per month.
    let interest_rate = 0.015;

    // The investment will grow for 24 months.
    let periods = 24;

    // Calculate the overall solution including the future value, the value at
    // the end of the last period.
    let solution = finance::future_value_solution(interest_rate, periods, present_value);
    dbg!(&solution);

    // Calculate the value at the end of each period.
    let series = solution.series();
    dbg!(&series);

    // Confirm that we have one entry for the initial value and one entry for each period.
    assert_eq!(25, series.len());

    // Create a reduced vector with every fourth period.
    let filtered_series = series
        .iter()
        .filter(|x| x.period() % 4 == 0)
        .collect::<Vec<_>>();
    dbg!(&filtered_series);
    assert_eq!(7, filtered_series.len());
}

fn try_doc_example_schedule() {

    // The rates vary by year: 4% followed by -3.9%, 10.6%, and -5.7%.
    let rates = [0.04, -0.039, 0.106, -0.057];

    // The initial investment is $75,000.
    let present_value = 75_000.00;

    let future_value = finance::future_value_schedule(&rates, present_value);
    dbg!(&future_value);
    finance::assert_rounded_4(78_178.0458, future_value);
}

fn try_doc_example_schedule_solution() {
    // The rates vary by year: 8.1% followed by 11%, 4%, and -2.3%.
    let rates = [0.081, 0.11, 0.04, -0.023];

    // The initial investment is $10,000.
    let present_value = 10_000.00;

    let solution = finance::future_value_schedule_solution(&rates, present_value);
    dbg!(&solution);

    let future_value = solution.future_value();
    dbg!(&future_value);
    finance::assert_rounded_4(future_value, 12_192.0455);

    // Calculate the value for each period.
    let series = solution.series();
    dbg!(&series);
}

fn try_doc_example_schedule_series() {

    // The rates vary by year: 11.6% followed by 13.4%, 9%, and 8.6%.
    let rates = [0.116, 0.134, -0.09, 0.086];

    // The initial investment is $50,000.
    let present_value = 50_000.00;

    // Calculate the future value and create a struct with all of the variables
    // and the formula used.
    let solution = finance::future_value_schedule_solution(&rates, present_value);
    dbg!(&solution);
    finance::assert_rounded_4(62534.3257, solution.future_value());

    // Calculate the value at the end of each period.
    let series = solution.series();
    dbg!(&series);

    // Confirm that there are four periods corresponding to the four interest
    // rates as well as one more for period 0 representing the initial value.
    assert_eq!(5, series.len());

    // Confirm that the value of the fourth period is the same as the overall
    // future value.
    finance::assert_rounded_4(solution.future_value(), series.last().unwrap().value());

    // Find the first period where the value of the investment was at least
    // $60,000.
    let entry = series.iter().find(|x| x.value() >= 60_000.00);
    dbg!(&entry);
    assert_eq!(2, entry.unwrap().period());
}

fn try_find_rate() {
    let present_value = 10_000.00f64;
    let interest_rate = 0.05;
    let periods = 100;
    let future_value = finance::future_value(interest_rate, periods, present_value);
    dbg!(&future_value);

    // let calc_rate = (future_value / present_value).log(periods as f64);
    // let calc_rate = (future_value / present_value).cbrt() - 1.0;
    let power = 1.0 / periods as f64;
    let calc_rate = (future_value / present_value).powf(power) - 1.0;
    dbg!(&calc_rate);

    let calc_periods = (future_value / present_value).log(1.0 + interest_rate);
    // let calc_periods = (interest_rate as f64).log(future_value / present_value);
    dbg!(&calc_periods);

    dbg!(25.0f64.log(2.0));
}

fn try_series_print_table() {
    let locale = finance::num_format::Locale::en;
    // let locale = finance::num_format::Locale::vi;
    let precision = 2;

    let rate = 0.05;
    let periods = 12;
    let present_value = 1_000;
    let solution = finance::future_value_solution(rate, periods, present_value);
    let series = solution.series();
    dbg!(&solution, &series);

    series.print_table(&locale, precision);

    finance::future_value_solution(0.05, 12, 1_000)
        .series()
        .filter(|x| x.value() > 1_400.0)
        .print_table(&locale, precision);

    finance::future_value_schedule_solution(&[0.04, 0.07, 0.06, 0.05], 1_000)
        .series()
        .print_table(&locale, precision);

    finance::future_value_schedule_solution(&[0.04, 0.07, 0.06, 0.05], 1_000)
        .print_series_table(&locale, precision);
}

fn check_formulas() {
    let solution = finance::future_value_solution(0.11, 5, 100);
    dbg!(&solution, &solution.series());

    let schedule = finance::future_value_schedule_solution(&[0.11, 0.09, 0.12], 100);
    dbg!(&schedule, &schedule.series());
}

fn try_continuous() {
    let apr = 0.05;
    let years = 5;
    let present_value = 1_000;

    for periods_per_year in [1, 2, 4, 6, 12, 52, 365].iter() {
        let rate = apr / *periods_per_year as f64;
        let periods = periods_per_year * years;
        println!("{}", finance::future_value(rate, periods, present_value));
    }
    // Continuous.
    println!("{}", finance::future_value_continuous(apr, years, present_value));
}
