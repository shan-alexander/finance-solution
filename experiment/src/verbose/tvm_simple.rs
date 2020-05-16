#![allow(dead_code)]

pub fn main() {
    try_doc_example_time_value_of_money_tvm_solution_1();
}

fn try_doc_example_time_value_of_money_tvm_solution_1() {

    // The interest will compound for ten years.
    let periods = 10;

    // The starting value is $10,000.
    let present_value = 10_000.00;

    // The ending value is $15,000.
    let future_value = 15_000.00;

    // Calculate the periodic rate and create a struct with a record of the
    // inputs, a description of the formula, and an option to calculate the
    // period-by-period values.
    let solution = finance::rate_solution(periods, present_value, future_value);
    dbg!(&solution);

    let rate = solution.rate();
    dbg!(&rate);
    // The rate is 4.138% per year.
    finance::assert_rounded_6(0.041380, rate);

    // Examine the formula.
    let formula = solution.formula();
    dbg!(formula);
    assert_eq!("((15000.0000 / 10000.0000) ^ (1 / 10)) - 1", formula);

    // Calculate the period-by-period values.
    let series = solution.series();
    dbg!(&series);
}

fn try_doc_example_series_1() {

    // The interest will compound monthly for two years.
    let periods = 24;

    // The starting value is $100,000.
    let present_value = 100_000.00;

    // The ending value is $15,000.
    let future_value = 125_000.00;

    // Calculate the periodic rate and create a struct that supports further operations.
    let solution = finance::rate_solution(periods, present_value, future_value);
    dbg!(&solution);

    // Calculate the period-by-period values.
    let series = solution.series();
    dbg!(&series);

    // Print the period-by-period details in a formatted table using 2 decimal places.
    let locale = finance::num_format::Locale::en;
    series.print_table(&locale, 2);

    // Print only the periods where the value has grown to at least $120,000.
    series
        .filter(|entry| entry.value() >= 120_000.0)
        .print_table(&locale, 4);
}

fn check_formulas() {
    let solution = finance::rate_solution(4, 100, 200);
    dbg!(&solution, &solution.series());
}

