#![allow(dead_code)]

use crate::fin::TimeValueOfMoneySolution;

pub fn main() {
    // try_present_value_solution();
    // try_present_value_series();
    // try_present_value();
    // try_present_value_schedule();
    // try_doc_example_solution_1();
    // try_doc_example_solution_2();
    // try_doc_example_series();
    // try_doc_example_schedule();
    // try_doc_example_schedule_solution();
    // try_doc_example_schedule_series();
    try_values_for_diagram_1();
    // dbg!(finance::present_value_solution(0.009, 1, 100_000));
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
    let present_value_1 = finance::present_value_solution(rate_of_return, periods, future_value_1).series();
    dbg!(present_value_1);
    
    // expect 211_513.1216
    let rate_of_return = 0.034;
    let future_value_1 = 250_000;
    let periods = 5;
    let present_value_2 = finance::present_value_solution(rate_of_return, periods, future_value_1).series();
    dbg!(present_value_2);

    // expect 7181.0056
    let rate_of_return = 1.034;
    let periods = 5;
    let future_value_1 = 250_000;
    let present_value_3 = finance::present_value_solution(rate_of_return, periods, future_value_1).series();
    dbg!(present_value_3);
}

fn try_present_value_schedule() {
    let rates = [0.04, 0.07, -0.12, -0.03, 0.11];
    let future_value = 100_000.25;

    let present_value = finance::present_value_schedule(&rates, future_value);
    dbg!(&present_value);

    let solution = finance::present_value_schedule_solution(&rates, future_value);
    dbg!(&solution);
    dbg!(&solution.series());
}

fn try_doc_example() {
    // The investment will grow by 1.1% per month.
    let rate = 0.011;

    // The investment will grow for 12 months.
    let periods = 12;

    // The final value will be $50,000.
    let future_value = 50_000;

    // Find the current value.
    let present_value = finance::present_value(rate, periods, future_value);

    dbg!(&present_value);

    // Confirm that the present value is correct to four decimal places (one hundredth of a cent).
    finance::assert_rounded_4(43_848.6409, present_value);
}

fn try_doc_example_solution_1() {

    // The rate is 8.45% per year.
    let rate = 0.0845;

    // The investment will grow for six years.
    let periods = 6;

    // The final value is $50,000.
    let future_value = 50_000;

    // Calculate the present value and create a struct with the input values and
    // the formula used.
    let solution= finance::present_value_solution(rate, periods, future_value);
    dbg!(&solution);

    let present_value = solution.present_value();
    finance::assert_rounded_4(present_value, 30_732.1303);

    // Examine the formula.
    let formula = solution.formula();
    dbg!(&formula);
    assert_eq!(formula, "50000.0000 / (1.084500 ^ 6)");

    // Calculate the amount at the end of each period.
    let series = solution.series();
    dbg!(&series);
}

fn try_doc_example_solution_2() {

    // The rate is 0.9% per month.
    let rate = 0.009;

    // The final value is $100,000.
    let future_value = 100_000;

    // We'll keep a collection of the calculated present values along with their inputs.
    let mut scenarios = vec![];
    // Calculate the present value for terms ranging from 1 to 36 months.
    for periods in 1..=36 {
        // Calculate the future value for this number of months and add the details to the
        // collection.
        scenarios.push(finance::present_value_solution(rate, periods, future_value));
    }
    dbg!(&scenarios);
    assert_eq!(36, scenarios.len());

    // Keep only the scenarios where the present value is less than or equal to $80,000.
    scenarios.retain(|x| x.present_value() <= 80_000.00);
    dbg!(&scenarios);
    assert_eq!(12, scenarios.len());

    // Find the range of months for the remaining scenarios.
    let min_months = scenarios.iter().map(|x| x.periods()).min().unwrap();
    let max_months = scenarios.iter().map(|x| x.periods()).max().unwrap();
    dbg!(min_months, max_months);
    assert_eq!(25, min_months);
    assert_eq!(36, max_months);

    // Check the formula for the first scenario.
    dbg!(scenarios[0].formula());
    assert_eq!("100000.0000 / (1.009000 ^ 25)", scenarios[0].formula());
}

fn try_doc_example_series() {

    // The interest rate is 7.8% per year.
    let interest_rate = 0.078;

    // The investment will grow for 10 years.
    let periods = 10;

    // The final value is $8112.75.
    let future_value = 8_112.75;

    // Calculate the present value, the value at the beginning of the first
    // period.
    let solution = finance::present_value_solution(interest_rate, periods, future_value);
    dbg!(&solution);

    // Calculate the value at the end of each period.
    let series = solution.series();
    dbg!(&series);

    // Confirm that we have one entry for the present value, that is the
    // initial value before any interest is applied, and one entry for each
    // period.
    assert_eq!(11, series.len());

    // Create a reduced vector with every other period not including period 0,
    // the initial state.
    let filtered_series = series
        .iter()
        .filter(|x| x.period() % 2 == 0 && x.period() != 0)
        .collect::<Vec<_>>();
    dbg!(&filtered_series);
    assert_eq!(5, filtered_series.len());
}

fn try_doc_example_schedule() {

    // The annual rate varies from -3.4% to 12.9%.
    let rates = [0.04, -0.034, 0.0122, 0.129, 8.5];

    // The value of the investment after applying all of these periodic rates
    // will be $30_000.
    let future_value = 30_000.00;

    // Calculate the present value.
    let present_value = finance::present_value_schedule(&rates, future_value);
    dbg!(&present_value);
}

fn try_doc_example_schedule_solution() {

    // The quarterly rate varies from -0.5% to 4%.
    let rates = [0.04, 0.008, 0.0122, -0.005];

    // The value of the investment after applying all of these periodic rates
    // will be $25_000.
    let future_value = 25_000.00;

    // Calculate the present value and keep track of the inputs and the formula
    // in a struct.
    let solution = finance::present_value_schedule_solution(&rates, future_value);
    dbg!(&solution);

    let present_value = solution.present_value();
    finance::assert_rounded_4(present_value, 23_678.6383);

    // Calculate the value for each period.
    let series = solution.series();
    dbg!(&series);
}

fn try_doc_example_schedule_series() {

    // The annual rate varies from -12% to 11%.
    let rates = [0.04, 0.07, -0.12, -0.03, 0.11];

    // The value of the investment after applying all of these periodic rates
    // will be $100_000.25.
    let future_value = 100_000.25;

    // Calculate the present value and keep track of the inputs and the formula
    // in a struct.
    let solution = finance::present_value_schedule_solution(&rates, future_value);
    dbg!(&solution);

    // Calculate the value at the end of each period.
    let series = solution.series();
    dbg!(&series);
    // There is one entry for each period and one entry for period 0 containing
    // the present value.
    assert_eq!(6, series.len());

    // Create a filtered list of periods, only those with a negative rate.
    let filtered_series = series
        .iter()
        .filter(|x| x.rate() < 0.0)
        .collect::<Vec<_>>();
    dbg!(&filtered_series);
    assert_eq!(2, filtered_series.len());
}

fn try_values_for_diagram_1() {
    let rate = 0.2;
    let future_value = 100;
    dbg!(finance::present_value_solution(rate, 1, future_value).present_value_vary_compounding_periods(&[1, 2, 4, 6, 12, 24, 52, 365]));

    dbg!(finance::present_value_continuous(rate, 1, future_value));

}

/*
$$\begin{tikzpicture}[scale=1.0544]
\begin{axis}[axis line style=gray,
	samples=100,
	width=9.0cm,height=6.4cm,
	xmin=0, xmax=12,
	ymin=80, ymax=85,
	restrict y to domain=0:1000,
	ytick={81, 82, 83, 84},
	xtick={1,2,3,4,5,6,7,8,9,10,11,12},
	axis equal,
	axis x line=center,
	axis y line=center,
	xlabel=$n$,ylabel=$pv$]
\addplot[red,domain=1:12,semithick]{100/((1+(0.2/x))^x)};
\addplot[black,domain=1:12]{100/(e^(0.2))};
\addplot[] coordinates {(2.5,81.2)} node{$pv={100 \over e^{0.2}}$};
\addplot[red] coordinates {(4.5,83.3)} node{$pv={100 \over (1+{0.2 \over n})^n}$};
\path (axis cs:0,83) node [anchor=north west,yshift=-0.07cm];
\end{axis}
\end{tikzpicture}$$

//i.upmath.me/svg/%24%24%5Cbegin%7Btikzpicture%7D%5Bscale%3D1.0544%5D%0A%5Cbegin%7Baxis%7D%5Baxis%20line%20style%3Dgray%2C%0A%09samples%3D100%2C%0A%09width%3D9.0cm%2Cheight%3D6.4cm%2C%0A%09xmin%3D0%2C%20xmax%3D12%2C%0A%09ymin%3D80%2C%20ymax%3D85%2C%0A%09restrict%20y%20to%20domain%3D0%3A1000%2C%0A%09ytick%3D%7B81%2C%2082%2C%2083%2C%2084%7D%2C%0A%09xtick%3D%7B1%2C2%2C3%2C4%2C5%2C6%2C7%2C8%2C9%2C10%2C11%2C12%7D%2C%0A%09axis%20equal%2C%0A%09axis%20x%20line%3Dcenter%2C%0A%09axis%20y%20line%3Dcenter%2C%0A%09xlabel%3D%24n%24%2Cylabel%3D%24pv%24%5D%0A%5Caddplot%5Bred%2Cdomain%3D1%3A12%2Csemithick%5D%7B100%2F((1%2B(0.2%2Fx))%5Ex)%7D%3B%0A%5Caddplot%5Bblack%2Cdomain%3D1%3A12%5D%7B100%2F(e%5E(0.2))%7D%3B%0A%5Caddplot%5B%5D%20coordinates%20%7B(2.5%2C81.2)%7D%20node%7B%24pv%3D%7B100%20%5Cover%20e%5E%7B0.2%7D%7D%24%7D%3B%0A%5Caddplot%5Bred%5D%20coordinates%20%7B(4.5%2C83.3)%7D%20node%7B%24pv%3D%7B100%20%5Cover%20(1%2B%7B0.2%20%5Cover%20n%7D)%5En%7D%24%7D%3B%0A%5Cpath%20(axis%20cs%3A0%2C83)%20node%20%5Banchor%3Dnorth%20west%2Cyshift%3D-0.07cm%5D%3B%0A%5Cend%7Baxis%7D%0A%5Cend%7Btikzpicture%7D%24%24

*/