#![allow(dead_code)]

pub fn main() {
    // try_edge_cases();
    // try_doc_example_1();
    // try_doc_example_solution_1();
    try_doc_example_solution_2();
}

fn try_edge_cases() {
    let fractional_periods = finance::periods(-1.0, 10_000.0, 0.0);
    dbg!(&fractional_periods);
}

fn try_doc_example() {

    // The interest rate is 8% per year.
    let periodic_rate = 0.08;

    // The starting value is $5,000.00.
    let present_value = 5_000.00;

    // The ending value is $7,000.00.
    let future_value = 7_000.00;

    // Calculate the number of years required.
    let fractional_periods = finance::periods(periodic_rate, present_value, future_value);
    dbg!(&fractional_periods);
    finance::assert_rounded_2(4.37, fractional_periods);

    // Round up to get a whole number of years.
    let periods = fractional_periods.ceil() as u32;
    dbg!(&periods);
    assert_eq!(5, periods);
}

fn try_doc_example_solution_1() {

    // The interest rate is 3.5% per quarter.
    let periodic_rate = 0.035;

    // The starting value is $100,000.00.
    let present_value = 100_000.00;

    // The ending value is $200,000.00.
    let future_value = 200_000.00;

    // Calculate the number of quarters required and build a struct with the
    // input values, an explanation of the formula, and an option to calculate
    // the period-by-period values.
    let solution = finance::periods_solution(periodic_rate, present_value, future_value);

    let fractional_periods = solution.fractional_periods;
    dbg!(&fractional_periods);
    finance::assert_rounded_2(20.15, fractional_periods);

    // Get the whole number of periods.
    let periods = solution.periods;
    dbg!(&periods);
    assert_eq!(21, periods);

    // Examine the formula.
    let formula = solution.formula.clone();
    dbg!(&formula);
    assert_eq!("log(200000.0000 / 100000.0000) base (1 + 0.035000))", formula);

    let series = solution.series();
    dbg!(&series);

    // The last period is calculated as if it were a full period so the value is higher than the
    // original future value.
    let last_entry = series.last().unwrap();
    dbg!(&last_entry);
    finance::assert_rounded_4(205_943.1474, last_entry.value);

    // Create a reduced series with the value at the end of each year.
    let filtered_series = series
        .iter()
        .filter(|x| x.period % 4 == 0 && x.period != 0)
        .collect::<Vec<_>>();
    dbg!(&filtered_series);
    assert_eq!(5, filtered_series.len());
}

fn try_doc_example_solution_2() {
    // The interest rate is -6% per year and the value falls from $15,000.00 to
    // $12,000.00.
    let solution = finance::periods_solution(-0.06, 15_000.00, 12_000.00);
    dbg!(&solution);
    finance::assert_rounded_2(3.61, solution.fractional_periods);
    assert_eq!(4, solution.periods);

    // View the period-by-period values.
    dbg!(solution.series());
}

