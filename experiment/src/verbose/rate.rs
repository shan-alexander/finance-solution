#![allow(dead_code)]

pub fn main() {
    // try_doc_example();
    // try_doc_example_solution();
    // try_doc_example_series_1();
    // check_formulas();
}

/*
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
    series.print_table_locale(&locale, 2);

    // Print only the periods where the value has grown to at least $120,000.
    series
        .filter(|entry| entry.value() >= 120_000.0)
        .print_table_locale(&locale, 4);
}

fn check_formulas() {
    let solution = finance::rate_solution(4, 100, 200);
    dbg!(&solution, &solution.series());
}
*/

/*
https://i.upmath.me/g/

Simple compounding.
Original:
        (future_value / present_value).powf(1.0 / periods as f64) - 1.0
LaTeX:
rate = \sqrt[periods]{\frac{future\_value}{present\_value}} - 1
//i.upmath.me/svg/rate%20%3D%20%5Csqrt%5Bperiods%5D%7B%5Cfrac%7Bfuture%5C_value%7D%7Bpresent%5C_value%7D%7D%20-%201
r = \sqrt[n]{\frac{fv}{pv}} - 1
//i.upmath.me/svg/r%20%3D%20%5Csqrt%5Bn%5D%7B%5Cfrac%7Bfv%7D%7Bpv%7D%7D%20-%201


Continuous compounding.
Original:
        (future_value / present_value).ln() / periods as f64
LaTeX:
rate = \frac{\ln\left(\frac{future\_value}{present\_value}\right)}{periods}
//i.upmath.me/svg/rate%20%3D%20%5Cfrac%7B%5Cln%5Cleft(%5Cfrac%7Bfuture%5C_value%7D%7Bpresent%5C_value%7D%5Cright)%7D%7Bperiods%7D
r = \frac{\ln\left(\frac{fv}{pv}\right)}n
//i.upmath.me/svg/r%20%3D%20%5Cfrac%7B%5Cln%5Cleft(%5Cfrac%7Bfv%7D%7Bpv%7D%5Cright)%7Dn







        (future_value / present_value).ln() / periods as f64
    } else {



        let symbolic_formula = "r = log(fv / pv, base e) / t";
        (formula, symbolic_formula)
    } else {
        let formula = format!("{:.6} = (({:.4} / {:.4}) ^ (1 / {})) - 1", rate, future_value, present_value, periods);
        let symbolic_formula = "r = ((fv / pv) ^ (1 / n)) - 1";




*/

