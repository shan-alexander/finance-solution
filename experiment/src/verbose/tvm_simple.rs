#![allow(dead_code)]

use finance::TimeValueOfMoneySolution;

pub fn main() {
    try_doc_example_time_value_of_money_tvm_solution_1();
}

fn try_doc_example_time_value_of_money_tvm_solution_1() {
    // Set up inputs for a variety of calculations that should all be equivalent.
    let rate = 0.015;
    let periods = 24;
    let present_value = 10_000.0;
    let future_value = finance::future_value(rate, periods, present_value);

    // Create a list of solution structs. For simplicity, instead of holding references to a
    // `RateSolution`, a `PeriodsSolution`, and so on turn each result into a more general
    // `TvmSolution`.
    let list = vec![
        finance::rate_solution(periods, present_value, future_value).tvm_solution(),
        finance::periods_solution(rate, present_value, future_value).tvm_solution(),
        finance::present_value_solution(rate, periods, future_value).tvm_solution(),
        finance::future_value_solution(rate, periods, present_value).tvm_solution(),
    ];
    dbg!(&list);

    // Print the formulas used by the four types of calculations.
    for solution in list.iter() {
        println!("{}:\n\t{}\n\t{}", solution.calculated_field(), solution.symbolic_formula(), solution.formula());
    }

    // An alternative would be to create a vector of references to the solutions.
    let _list: Vec<& dyn TimeValueOfMoneySolution> = vec![
        &finance::rate_solution(periods, present_value, future_value),
        &finance::periods_solution(rate, present_value, future_value),
        &finance::present_value_solution(rate, periods, future_value),
        &finance::future_value_solution(rate, periods, present_value),
    ];

}

fn check_formulas() {
    let solution = finance::rate_solution(4, 100, 200);
    dbg!(&solution, &solution.series());
}

