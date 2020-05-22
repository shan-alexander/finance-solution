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
    // try_doc_example_print_table();
    // try_find_rate();
    // try_series_print_table();
    // try_ab_comparison();
    create_formulas_for_docs();
    // check_formulas();
    // dbg!(finance::future_value_solution(0.012, 8, 200_000));
    // try_continuous();
    /*
    try_vary_compounding_periods();
    try_simple_to_continuous(&finance::TvmVariable::Rate);
    try_simple_to_continuous(&finance::TvmVariable::Periods);
    try_simple_to_continuous(&finance::TvmVariable::PresentValue);
    try_simple_to_continuous(&finance::TvmVariable::FutureValue);
    */
    // try_values_for_diagram_1();
}

/*
fn try_doc_example_print_table() {
    // finance::future_value_solution(0.045, 5, 10_000).print_series_table();
    let locale = finance::num_format::Locale::en;
    let precision = 2;
    finance::future_value_solution(0.11, 4, 5_000).print_series_table_locale(&locale, precision);

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

fn try_ab_comparison() {
    let locale = finance::num_format::Locale::en;
    let precision = 4;

    let years = 1;
    let rate = 0.20 / 12.0;
    let periods = years * 12;
    let present_value = 5_000.0;

    let solution_a = finance::future_value_solution(rate, periods, present_value -500.0);
    let solution_b = finance::future_value_solution(rate, periods -3, present_value);
    solution_a.print_ab_comparison_locale(&solution_b, &locale, precision);

    let solution_a = finance::future_value_solution(rate, periods, present_value);
    let solution_b = solution_a.rate_solution(true, None);
    solution_a.print_ab_comparison_locale(&solution_b, &locale, 8);

    let solution_a = finance::future_value_solution(rate, periods, present_value);
    let solution_b = finance::future_value_solution(rate + 0.001, periods, present_value);
    solution_a.print_ab_comparison(&solution_b);
}

fn check_formulas() {
    let solution = finance::future_value_solution(0.11, 5, 100);
    dbg!(&solution, &solution.series());

    let schedule = finance::future_value_schedule_solution(&[0.11, 0.09, 0.12], 100);
    dbg!(&schedule, &schedule.series());
}
*/

fn create_formulas_for_docs() {
    let (rate, periods, present_value) = (0.035, 12, 1_000);
    dbg!(finance::future_value_solution(rate, periods, present_value, false));
    dbg!(finance::future_value_solution(rate, periods, present_value, true));
}

/*
fn solution_for_transformations() -> finance::FutureValueSolution {
    let rate = 0.10;
    let periods = 4;
    let present_value = 5_000.00;
    finance::future_value_solution(rate, periods, present_value)
}

fn compounding_periods() -> Vec<u32> {
    vec![1, 2, 4, 6, 12, 24, 52, 365]
}

fn try_continuous() {
    let apr = 0.05;
    let years= 5;
    let present_value = 1_000;

    for periods_per_year in compounding_periods().iter() {
        let rate = apr / *periods_per_year as f64;
        let periods = periods_per_year * years;
        println!("{}", finance::future_value(rate, periods, present_value));
    }
    // Continuous.
    println!("{}", finance::future_value_continuous(apr, years, present_value));
}

fn print_header_for_try(header: &str) {
    println!("\n{}\n\n{}\n", "=".repeat(120), header);
}

fn try_vary_compounding_periods() {
    print_header_for_try("try_vary_compounding_periods()");
    let solution = solution_for_transformations();
    dbg!(&solution);

    println!("\nFuture values for a variety of compounding periods.\n");
    dbg!(solution.future_value_vary_compounding_periods(&compounding_periods()));

    println!("\nPresent values for a variety of compounding periods.\n");
    dbg!(solution.present_value_vary_compounding_periods(&compounding_periods()));
}

fn try_simple_to_continuous(vary_field: &finance::TvmVariable) {
    print_header_for_try(&format!("try_simple_to_continuous() varying {} and keeping the other three values constant.", vary_field.to_string().to_lowercase()));

    let simple_solution = solution_for_transformations();
    dbg!(&simple_solution);

    let (continuous_solution, simple_solution_round_trip) = match vary_field {
        finance::TvmVariable::Rate => {
            let continuous = simple_solution.rate_solution(true, None).tvm_solution();
            let round_trip = continuous.rate_solution(false, None).tvm_solution();
            (continuous, round_trip)
        },
        finance::TvmVariable::Periods => {
            let continuous = simple_solution.periods_solution(true).tvm_solution();
            let round_trip = continuous.periods_solution(false).tvm_solution();
            (continuous, round_trip)
        },
        finance::TvmVariable::PresentValue => {
            let continuous = simple_solution.present_value_solution(true, None).tvm_solution();
            let round_trip = continuous.present_value_solution(false, None).tvm_solution();
            (continuous, round_trip)
        },
        finance::TvmVariable::FutureValue => {
            let continuous = simple_solution.future_value_solution(true, None).tvm_solution();
            let round_trip = continuous.future_value_solution(false, None).tvm_solution();
            (continuous, round_trip)
        },
    };

    println!("\nChanged to continuous compounding, varying {} to keep the other three values constant.\n", vary_field.to_string().to_lowercase());
    dbg!(&continuous_solution);

    println!("\nBack to simple compounding, varying {} to keep the other three values constant.\n", vary_field.to_string().to_lowercase());
    dbg!(&simple_solution_round_trip);
}
*/

/*
https://i.upmath.me/g/

Simple:
future\_value = present\_value \times (1+rate)^{periods}
//i.upmath.me/svg/future%5C_value%20%3D%20present%5C_value%20%5Ctimes%20(1%2Brate)%5E%7Bperiods%7D
fv = pv \times (1+r)^n
//i.upmath.me/svg/fv%20%3D%20pv%20%5Ctimes%20(1%2Br)%5En

Continuous:
future\_value = present\_value \times e^{rate \times periods}
//i.upmath.me/svg/future%5C_value%20%3D%20%7Bpresent%5C_value%20%5Ctimes%20e%5E%7Brate%20%5Ctimes%20periods%7D
fv = pv \times e^{r \times n}
//i.upmath.me/svg/fv%20%3D%20pv%20%5Ctimes%20e%5E%7Br%20%5Ctimes%20n%7D

Plot showing how the future value with increasing numbers of periods approaches the future value
with continuous compounding.

$$\begin{tikzpicture}[scale=1.0544]\small
\begin{axis}[axis line style=gray,
	samples=100,
	width=9.0cm,height=6.4cm,
	xmin=0, xmax=12,
	ymin=120, ymax=123,
	restrict y to domain=0:1000,
	ytick={120, 121, 122},
	xtick={1,2,3,4,5,6,7,8,9,10,11,12},
	axis equal,
	axis x line=center,
	axis y line=center,
	xlabel=$n$,ylabel=$fv$]
\addplot[red,domain=1:12,semithick]{100*((1+(0.2/x))^x)};
\addplot[black,domain=1:12]{100*(e^(0.2))};
\addplot[] coordinates {(2.5,122.8)} node{$fv=100e^{0.2}$};
\addplot[red] coordinates {(4,120.3)} node{$fv=100(1+{0.2 \over n})^n$};
\path (axis cs:0,122) node [anchor=north west,yshift=-0.07cm];
\end{axis}
\end{tikzpicture}$$

//i.upmath.me/svg/%24%24%5Cbegin%7Btikzpicture%7D%5Bscale%3D1.0544%5D%5Csmall%0A%5Cbegin%7Baxis%7D%5Baxis%20line%20style%3Dgray%2C%0A%09samples%3D100%2C%0A%09width%3D9.0cm%2Cheight%3D6.4cm%2C%0A%09xmin%3D0%2C%20xmax%3D12%2C%0A%09ymin%3D120%2C%20ymax%3D123%2C%0A%09restrict%20y%20to%20domain%3D0%3A1000%2C%0A%09ytick%3D%7B120%2C%20121%2C%20122%7D%2C%0A%09xtick%3D%7B1%2C2%2C3%2C4%2C5%2C6%2C7%2C8%2C9%2C10%2C11%2C12%7D%2C%0A%09axis%20equal%2C%0A%09axis%20x%20line%3Dcenter%2C%0A%09axis%20y%20line%3Dcenter%2C%0A%09xlabel%3D%24n%24%2Cylabel%3D%24fv%24%5D%0A%5Caddplot%5Bred%2Cdomain%3D1%3A12%2Csemithick%5D%7B100*((1%2B(0.2%2Fx))%5Ex)%7D%3B%0A%5Caddplot%5Bblack%2Cdomain%3D1%3A12%5D%7B100*(e%5E(0.2))%7D%3B%0A%5Caddplot%5B%5D%20coordinates%20%7B(2.5%2C122.8)%7D%20node%7B%24fv%3D100e%5E%7B0.2%7D%24%7D%3B%0A%5Caddplot%5Bred%5D%20coordinates%20%7B(4%2C120.3)%7D%20node%7B%24fv%3D100(1%2B%7B0.2%20%5Cover%20n%7D)%5En%24%7D%3B%0A%5Cpath%20(axis%20cs%3A0%2C122)%20node%20%5Banchor%3Dnorth%20west%2Cyshift%3D-0.07cm%5D%3B%0A%5Cend%7Baxis%7D%0A%5Cend%7Btikzpicture%7D%24%24

$$\begin{tikzpicture}[scale=1.0544]\small
\begin{axis}[axis line style=gray,
	samples=12,
	width=9.0cm,height=6.4cm,
	xmin=0, xmax=12,
	ymin=119, ymax=123,
	restrict y to domain=0:1000,
	ytick={120, 121, 122},
	xtick={1,2,3,4,5,6,7,8,9,10,11,12},
	axis x line=center,
	axis y line=center,
	xlabel=$n$,ylabel=$fv$]
\addplot[blue,domain=1:12,thick, only marks]{100*((1+(0.2/x))^x)};
\addplot[black,domain=1:12,thick]{100*(e^(0.2))};
\addplot[] coordinates {(2.5,122.4)} node{$fv=100e^{0.2}$};
\addplot[blue] coordinates {(4.8,120.7)} node{$fv=100(1+{0.2 \over n})^n$};
\path (axis cs:0,122) node [anchor=north west,yshift=-0.07cm];
\end{axis}
\end{tikzpicture}$$

//i.upmath.me/svg/%24%24%5Cbegin%7Btikzpicture%7D%5Bscale%3D1.0544%5D%5Csmall%0A%5Cbegin%7Baxis%7D%5Baxis%20line%20style%3Dgray%2C%0A%09samples%3D12%2C%0A%09width%3D9.0cm%2Cheight%3D6.4cm%2C%0A%09xmin%3D0%2C%20xmax%3D12%2C%0A%09ymin%3D119%2C%20ymax%3D123%2C%0A%09restrict%20y%20to%20domain%3D0%3A1000%2C%0A%09ytick%3D%7B120%2C%20121%2C%20122%7D%2C%0A%09xtick%3D%7B1%2C2%2C3%2C4%2C5%2C6%2C7%2C8%2C9%2C10%2C11%2C12%7D%2C%0A%09axis%20x%20line%3Dcenter%2C%0A%09axis%20y%20line%3Dcenter%2C%0A%09xlabel%3D%24n%24%2Cylabel%3D%24fv%24%5D%0A%5Caddplot%5Bblue%2Cdomain%3D1%3A12%2Cthick%2C%20only%20marks%5D%7B100*((1%2B(0.2%2Fx))%5Ex)%7D%3B%0A%5Caddplot%5Bblack%2Cdomain%3D1%3A12%2Cthick%5D%7B100*(e%5E(0.2))%7D%3B%0A%5Caddplot%5B%5D%20coordinates%20%7B(2.5%2C122.4)%7D%20node%7B%24fv%3D100e%5E%7B0.2%7D%24%7D%3B%0A%5Caddplot%5Bblue%5D%20coordinates%20%7B(4.8%2C120.7)%7D%20node%7B%24fv%3D100(1%2B%7B0.2%20%5Cover%20n%7D)%5En%24%7D%3B%0A%5Cpath%20(axis%20cs%3A0%2C122)%20node%20%5Banchor%3Dnorth%20west%2Cyshift%3D-0.07cm%5D%3B%0A%5Cend%7Baxis%7D%0A%5Cend%7Btikzpicture%7D%24%24

*/
