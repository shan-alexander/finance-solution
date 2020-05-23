#![allow(dead_code)]

pub fn main() {
    // try_edge_cases();
    // try_symmetric_mult_cases();
    // try_numbers_for_module_level_charts();
    // create_formulas_for_docs();
    // try_doc_example_1();
    // try_doc_example_solution_1();
    // try_doc_example_solution_2();
    // check_formulas();
}

/*
fn try_edge_cases() {
    let fractional_periods = finance::periods(-1.0, 10_000.0, 0.0);
    dbg!(&fractional_periods);
}

fn try_symmetric_mult_cases() {
    let solution = finance::periods_solution(-0.05, -1234.987654321, -1173.23827160495);
    dbg!(&solution);
}

fn try_numbers_for_module_level_charts() {
    let rate = 0.1;
    let periods = 12;
    let present_value = 100;
    let fv_solution = finance::future_value_solution(rate, periods, present_value);
    fv_solution.print_series_table();

    let future_value = 250;
    let periods_solution = finance::periods_solution(rate, present_value, future_value);
    dbg!(periods_solution.fractional_periods());

    dbg!(&periods_solution);

    /*
    let rate = -0.1;
    let periods = 12;
    let present_value = 100;
    let fv_solution = finance::future_value_solution(rate, periods, present_value);
    fv_solution.print_series_table();

    let future_value = 70;
    let periods_solution = finance::periods_solution(rate, present_value, future_value);
    dbg!(periods_solution.fractional_periods());
    */
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

    let fractional_periods = solution.fractional_periods();
    dbg!(&fractional_periods);
    finance::assert_rounded_2(20.15, fractional_periods);

    // Get the whole number of periods.
    let periods = solution.periods();
    dbg!(&periods);
    assert_eq!(21, periods);

    // Examine the formula.
    let formula = solution.formula();
    dbg!(formula);
    assert_eq!("log(200000.0000 / 100000.0000) base (1 + 0.035000))", formula);

    let series = solution.series();
    dbg!(&series);

    // The last period is calculated as if it were a full period so the value is higher than the
    // original future value.
    let last_entry = series.last().unwrap();
    dbg!(&last_entry);
    finance::assert_rounded_4(205_943.1474, last_entry.value());

    // Create a reduced series with the value at the end of each year.
    let filtered_series = series
        .iter()
        .filter(|x| x.period() % 4 == 0 && x.period() != 0)
        .collect::<Vec<_>>();
    dbg!(&filtered_series);
    assert_eq!(5, filtered_series.len());
}

fn try_doc_example_solution_2() {
    // The interest rate is -6% per year and the value falls from $15,000.00 to
    // $12,000.00.
    let solution = finance::periods_solution(-0.06, 15_000.00, 12_000.00);
    dbg!(&solution);
    finance::assert_rounded_2(3.61, solution.fractional_periods());
    assert_eq!(4, solution.periods());

    // View the period-by-period values.
    dbg!(solution.series());
}

fn check_formulas() {
    let solution = finance::periods_solution(0.11, 100, 200);
    dbg!(&solution, &solution.series());
}
*/

/*
https://i.upmath.me/g/

Simple compounding.
Original:
n = log(fv / pv, base (1 + r))
LaTeX:
periods = \frac{\log_{1+rate}\left(\frac{future\_value}{present\_value}\right)}{rate}
//i.upmath.me/svg/periods%20%3D%20%5Cfrac%7B%5Clog_%7B1%2Brate%7D%5Cleft(%5Cfrac%7Bfuture%5C_value%7D%7Bpresent%5C_value%7D%5Cright)%7D%7Brate%7D
n = \frac{\log_{1+r}\left(\frac{fv}{pv}\right)}r
//i.upmath.me/svg/n%20%3D%20%5Cfrac%7B%5Clog_%7B1%2Br%7D%5Cleft(%5Cfrac%7Bfv%7D%7Bpv%7D%5Cright)%7Dr

Continuous compounding.
Original:
n = log(fv / pv, base e) / r
LaTeX:
periods = \frac{\ln\left(\frac{future\_value}{present\_value}\right)}{rate}
//i.upmath.me/svg/periods%20%3D%20%5Cfrac%7B%5Cln%5Cleft(%5Cfrac%7Bfuture%5C_value%7D%7Bpresent%5C_value%7D%5Cright)%7D%7Brate%7D
n = \frac{\ln\left(\frac{fv}{pv}\right)}r
//i.upmath.me/svg/n%20%3D%20%5Cfrac%7B%5Cln%5Cleft(%5Cfrac%7Bfv%7D%7Bpv%7D%5Cright)%7Dr





Module-level documentation, first chart:

$$\begin{tikzpicture}[scale=1.0544]\small
\begin{axis}[axis line style=gray,
	samples=12,
	width=9.0cm,height=6.4cm,
	xmin=0, xmax=12,
	ymin=70, ymax=370,
	restrict y to domain=0:1000,
	ytick={100, 150, 200, 250, 300, 350},
	xtick={1,2,3,4,5,6,7,8,9,10,11,12},
	axis x line=center,
	axis y line=center,
	xlabel=$n$,ylabel=$fv$]
\addplot[blue,domain=1:12,semithick,only marks]{100*((1.1)^x)};
\addplot[black,domain=1:12]{250};
\addplot[] coordinates {(2.5,122.8)} node{$fv=100e^{0.2}$};
\addplot[blue] coordinates {(5.5,120.3)} node{$fv=100(1.1^n)$};
\path (axis cs:0,122) node [anchor=north west,yshift=-0.07cm];
\end{axis}
\end{tikzpicture}$$

$$\begin{tikzpicture}[scale=1.0544]\small
\begin{axis}[axis line style=gray,
	samples=12,
	width=9.0cm,height=6.4cm,
	xmin=0, xmax=12,
	ymin=70, ymax=370,
	restrict y to domain=0:1000,
	ytick={100, 150, 200, 250, 300, 350},
	xtick={1,2,3,4,5,6,7,8,9,10,11,12},
	axis x line=center,
	axis y line=center,
	xlabel=$n$,ylabel=$fv$]
\addplot[blue,domain=1:12,semithick,only marks]{100*((1.1)^x)};
\addplot[blue] coordinates {(5.4,110)} node{$fv=100(1.1^n)$};
\path (axis cs:0,122) node [anchor=north west,yshift=-0.07cm];
\end{axis}
\end{tikzpicture}$$

//i.upmath.me/svg/%24%24%5Cbegin%7Btikzpicture%7D%5Bscale%3D1.0544%5D%5Csmall%0A%5Cbegin%7Baxis%7D%5Baxis%20line%20style%3Dgray%2C%0A%09samples%3D12%2C%0A%09width%3D9.0cm%2Cheight%3D6.4cm%2C%0A%09xmin%3D0%2C%20xmax%3D12%2C%0A%09ymin%3D70%2C%20ymax%3D370%2C%0A%09restrict%20y%20to%20domain%3D0%3A1000%2C%0A%09ytick%3D%7B100%2C%20150%2C%20200%2C%20250%2C%20300%2C%20350%7D%2C%0A%09xtick%3D%7B1%2C2%2C3%2C4%2C5%2C6%2C7%2C8%2C9%2C10%2C11%2C12%7D%2C%0A%09axis%20x%20line%3Dcenter%2C%0A%09axis%20y%20line%3Dcenter%2C%0A%09xlabel%3D%24n%24%2Cylabel%3D%24fv%24%5D%0A%5Caddplot%5Bblue%2Cdomain%3D1%3A12%2Csemithick%2Conly%20marks%5D%7B100*((1.1)%5Ex)%7D%3B%0A%5Caddplot%5Bblue%5D%20coordinates%20%7B(5.4%2C110)%7D%20node%7B%24fv%3D100(1.1%5En)%24%7D%3B%0A%5Cpath%20(axis%20cs%3A0%2C122)%20node%20%5Banchor%3Dnorth%20west%2Cyshift%3D-0.07cm%5D%3B%0A%5Cend%7Baxis%7D%0A%5Cend%7Btikzpicture%7D%24%24


Second chart:

$$\begin{tikzpicture}[scale=1.0544]\small
\begin{axis}[axis line style=gray,
	samples=100,
	width=9.0cm,height=6.4cm,
	xmin=0, xmax=12,
	ymin=70, ymax=370,
	restrict y to domain=0:1000,
	ytick={100, 150, 200, 250, 300, 350},
	xtick={1,2,3,4,5,6,7,8,9,10,11,12},
	axis x line=center,
	axis y line=center,
	xlabel=$n$,ylabel=$fv$]
\addplot[blue,domain=1:9.614,thick]{100*((1.1)^x)};
\addplot[blue,domain=9.614:12,thick,dashed]{100*((1.1)^x)};
\addplot[black,domain=1:12]{250};
\addplot[] coordinates {(2.1, 270)} node{$fv=250$};
\addplot[blue] coordinates {(5.5,120.3)} node{$fv=100(1.1^n)$};
\addplot[red] coordinates {(10.5,230)} node{$n=9.61$};
\path (axis cs:0,122) node [anchor=north west,yshift=-0.07cm];
\end{axis}
\end{tikzpicture}$$

$$\begin{tikzpicture}[scale=1.0544]\small
\begin{axis}[axis line style=gray,
	samples=100,
	width=9.0cm,height=6.4cm,
	xmin=0, xmax=12,
	ymin=70, ymax=370,
	restrict y to domain=0:1000,
	ytick={100, 150, 200, 250, 300, 350},
	xtick={1,2,3,4,5,6,7,8,9,10,11,12},
	axis x line=center,
	axis y line=center,
	xlabel=$n$,ylabel=$fv$]
\addplot[blue,domain=1:9.614,thick]{100*((1.1)^x)};
\addplot[blue,domain=9.614:12,thick,dashed]{100*((1.1)^x)};
\addplot[black,domain=1:12]{250};
\addplot[] coordinates {(2.1, 270)} node{$fv=250$};
\addplot[blue] coordinates {(5.5,120.3)} node{$fv=100(1.1^n)$};
\addplot[red] coordinates {(10.6,235)} node{$n=9.61$};
\path (axis cs:0,122) node [anchor=north west,yshift=-0.07cm];
\end{axis}
\end{tikzpicture}$$

//i.upmath.me/svg/%24%24%5Cbegin%7Btikzpicture%7D%5Bscale%3D1.0544%5D%5Csmall%0A%5Cbegin%7Baxis%7D%5Baxis%20line%20style%3Dgray%2C%0A%09samples%3D100%2C%0A%09width%3D9.0cm%2Cheight%3D6.4cm%2C%0A%09xmin%3D0%2C%20xmax%3D12%2C%0A%09ymin%3D70%2C%20ymax%3D370%2C%0A%09restrict%20y%20to%20domain%3D0%3A1000%2C%0A%09ytick%3D%7B100%2C%20150%2C%20200%2C%20250%2C%20300%2C%20350%7D%2C%0A%09xtick%3D%7B1%2C2%2C3%2C4%2C5%2C6%2C7%2C8%2C9%2C10%2C11%2C12%7D%2C%0A%09axis%20x%20line%3Dcenter%2C%0A%09axis%20y%20line%3Dcenter%2C%0A%09xlabel%3D%24n%24%2Cylabel%3D%24fv%24%5D%0A%5Caddplot%5Bblue%2Cdomain%3D1%3A9.614%2Cthick%5D%7B100*((1.1)%5Ex)%7D%3B%0A%5Caddplot%5Bblue%2Cdomain%3D9.614%3A12%2Cthick%2Cdashed%5D%7B100*((1.1)%5Ex)%7D%3B%0A%5Caddplot%5Bblack%2Cdomain%3D1%3A12%5D%7B250%7D%3B%0A%5Caddplot%5B%5D%20coordinates%20%7B(2.1%2C%20270)%7D%20node%7B%24fv%3D250%24%7D%3B%0A%5Caddplot%5Bblue%5D%20coordinates%20%7B(5.5%2C120.3)%7D%20node%7B%24fv%3D100(1.1%5En)%24%7D%3B%0A%5Caddplot%5Bred%5D%20coordinates%20%7B(10.6%2C235)%7D%20node%7B%24n%3D9.61%24%7D%3B%0A%5Cpath%20(axis%20cs%3A0%2C122)%20node%20%5Banchor%3Dnorth%20west%2Cyshift%3D-0.07cm%5D%3B%0A%5Cend%7Baxis%7D%0A%5Cend%7Btikzpicture%7D%24%24



Example with a negative interest rate.

$$\begin{tikzpicture}[scale=1.0544]\small
\begin{axis}[axis line style=gray,
	samples=100,
	width=9.0cm,height=6.4cm,
	xmin=0, xmax=12,
	ymin=0, ymax=120,
	restrict y to domain=0:1000,
	ytick={10, 20, 30, 40, 50, 60, 70,80, 90, 100},
	xtick={1,2,3,4,5,6,7,8,9,10,11,12},
	axis x line=center,
	axis y line=center,
	xlabel=$n$,ylabel=$fv$]
\addplot[blue,domain=1:3.385,thick]{100*((0.9)^x)};
\addplot[blue,domain=3.385:12,thick,dashed]{100*((0.9)^x)};
\addplot[black,domain=1:12]{70};
\addplot[] coordinates {(11, 64)} node{$fv=70$};
\addplot[blue] coordinates {(6.8,30)} node{$fv=100(0.9^n)$};
\addplot[red] coordinates {(4.5,75.1)} node{$n=3.39$};
\path (axis cs:0,122) node [anchor=north west,yshift=-0.07cm];
\end{axis}
\end{tikzpicture}$$

//i.upmath.me/svg/%24%24%5Cbegin%7Btikzpicture%7D%5Bscale%3D1.0544%5D%5Csmall%0A%5Cbegin%7Baxis%7D%5Baxis%20line%20style%3Dgray%2C%0A%09samples%3D100%2C%0A%09width%3D9.0cm%2Cheight%3D6.4cm%2C%0A%09xmin%3D0%2C%20xmax%3D12%2C%0A%09ymin%3D0%2C%20ymax%3D120%2C%0A%09restrict%20y%20to%20domain%3D0%3A1000%2C%0A%09ytick%3D%7B10%2C%2020%2C%2030%2C%2040%2C%2050%2C%2060%2C%2070%2C80%2C%2090%2C%20100%7D%2C%0A%09xtick%3D%7B1%2C2%2C3%2C4%2C5%2C6%2C7%2C8%2C9%2C10%2C11%2C12%7D%2C%0A%09axis%20x%20line%3Dcenter%2C%0A%09axis%20y%20line%3Dcenter%2C%0A%09xlabel%3D%24n%24%2Cylabel%3D%24fv%24%5D%0A%5Caddplot%5Bblue%2Cdomain%3D1%3A3.385%2Cthick%5D%7B100*((0.9)%5Ex)%7D%3B%0A%5Caddplot%5Bblue%2Cdomain%3D3.385%3A12%2Cthick%2Cdashed%5D%7B100*((0.9)%5Ex)%7D%3B%0A%5Caddplot%5Bblack%2Cdomain%3D1%3A12%5D%7B70%7D%3B%0A%5Caddplot%5B%5D%20coordinates%20%7B(11%2C%2064)%7D%20node%7B%24fv%3D70%24%7D%3B%0A%5Caddplot%5Bblue%5D%20coordinates%20%7B(6.8%2C30)%7D%20node%7B%24fv%3D100(0.9%5En)%24%7D%3B%0A%5Caddplot%5Bred%5D%20coordinates%20%7B(4.5%2C75.1)%7D%20node%7B%24n%3D3.39%24%7D%3B%0A%5Cpath%20(axis%20cs%3A0%2C122)%20node%20%5Banchor%3Dnorth%20west%2Cyshift%3D-0.07cm%5D%3B%0A%5Cend%7Baxis%7D%0A%5Cend%7Btikzpicture%7D%24%24







*/


