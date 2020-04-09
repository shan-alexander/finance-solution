pub fn main() {
    try_doc_example();
    try_doc_example_solution();
}

fn try_doc_example() {

    // The interest will compound for 365 days.
    let periods = 365;

    // The starting value is $10,000.
    let present_value = 10_000.00;

    // The ending value is $11,000.
    let future_value = 11_000.00;

    // Calculate the periodic rate needed.
    let rate = finance::rate(periods, present_value, future_value);
    dbg!(&rate);
    // The rate is 0.0261% per day.
    finance::assert_rounded_6(0.000261, rate);
}

fn try_doc_example_solution() {

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

    let rate = solution.rate;
    dbg!(&rate);
    // The rate is 4.138% per year.
    finance::assert_rounded_6(0.041380, rate);

    // Examine the formula.
    let formula = solution.formula.clone();
    dbg!(&formula);
    assert_eq!("((15000.0000 / 10000.0000) ^ (1 / 10)) - 1", &formula);

    // Calculate the period-by-period values.
    let series = solution.series();
    dbg!(&series);
}






