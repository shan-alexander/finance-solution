# finance-solution

## finance-solution is a financial library for time-value-of-money problems. 💸

People who will find this crate helpful include:
- **Students of Finance** who want to solve their financial problems using something better than a crude handheld calculator. Using this library also reduces the chance of human error, and provides better output and data displays than Excel.
- **New developers** who want to learn Rust using Finance as a topic.
- **Experienced Rust developers** who want to learn more about finance.
- **Serious Rust developers** who want to build financial software, and prefer to rely on a rigourously tested library instead of reinventing the wheel and spending hundreds of hours to develop and test their own library.

Currently, this library is geared towards the basic financial equations, regarding:
- **Simple Time-Value-of-Money formulas** -- this includes `present_value`, `future_value`, `rate`, and `periods` (known as NPER in Excel).
- **Cashflow Time-Value-of-Money formulas** -- this includes `present_value_annuity`, `future_value_annuity`, `net_present_value`, and `payment` (known as PMT in Excel). 
- **Rate conversions** -- this includes all conversions between `apr`, `ear`, and `epr`, and also includes conversion for continuous compounding (`apr_continuous`, `ear_continuous`).

## Examples

A business partner will give you $4,000 in 3 years.
Your rate-of-return in the market is 5%.
How much is the deal worth to you right now?

> <img src="http://i.upmath.me/svg/%5Cbegin%7Bdocument%7D%5Cbegin%7Btikzpicture%7D%0A%25%20draw%20horizontal%20line%20%20%20%0A%5Cdraw%20(0%2C0)%20--%20(6%2C0)%3B%0A%25%20draw%20vertical%20lines%0A%5Cforeach%20%5Cx%20in%20%7B0%2C2%2C4%2C6%7D%20%5Cdraw%20(%5Cx%20cm%2C3pt)%20--%20(%5Cx%20cm%2C-3pt)%3B%0A%25%20draw%20nodes%0A%5Cdraw%20(0%2C0)%20node%5Bbelow%3D3pt%5D%20%7B%24%200%20%24%7D%20node%5Babove%3D3pt%5D%20%7B%24%20%5C%24%3F%20%20%24%7D%3B%0A%5Cdraw%20(1%2C0)%20node%5Bbelow%3D3pt%5D%20%7B%24%20%24%7D%20node%5Babove%3D3pt%5D%20%7B%24%20%24%7D%3B%0A%5Cdraw%20(2%2C0)%20node%5Bbelow%3D3pt%5D%20%7B%24%201%20%24%7D%20node%5Babove%3D3pt%5D%20%7B%24%20%24%7D%3B%0A%5Cdraw%20(3%2C0)%20node%5Bbelow%3D3pt%5D%20%7B%24%20%20%24%7D%20node%5Babove%3D3pt%5D%20%7B%24%205%5C%25%20%24%7D%3B%0A%5Cdraw%20(4%2C0)%20node%5Bbelow%3D3pt%5D%20%7B%24%202%20%24%7D%20node%5Babove%3D3pt%5D%20%7B%24%20%24%7D%3B%0A%5Cdraw%20(5%2C0)%20node%5Bbelow%3D3pt%5D%20%7B%24%20%20%24%7D%20node%5Babove%3D3pt%5D%20%7B%24%20%20%24%7D%3B%0A%5Cdraw%20(6%2C0)%20node%5Bbelow%3D3pt%5D%20%7B%24%203%20%24%7D%20node%5Babove%3D3pt%5D%20%7B%24%20%5C%244%2C000%20%24%7D%3B%0A%5Cend%7Btikzpicture%7D%0A%5Cend%7Bdocument%7D" />

### uses `present_value()` to return f64 value
```
let future_value = 4_000;
let periods = 3;
let rate = 0.05;
let pv = present_value(rate, periods, future_value);
dbg!(pv);

// PRINTS TO TERMINAL:
// pv = 3455.350394125904 
```
> <small>The crate also provides helper functions for rounding, such as `round_4` to round to four decimals places. See `round` for more details.</small>

For the same problem above, you can use the _solution function to see a better output and provide additional functionality.
### uses `present_value_solution()` to return a custom "solution" struct
```
let future_value = 4_000;
let periods = 3;
let rate = 0.05;
let answer = present_value_solution(rate, periods, future_value);
dbg!(answer);

// PRINTS TO TERMINAL:
// answer = TvmSolution {
//    calculated_field: PresentValue,
//    rate: 0.05,
//    periods: 3,
//    fractional_periods: 3.0,
//    present_value: 3455.350394125904,
//    future_value: 4000.0,
//    formula: "3455.3504 = 4000.0000 / (1.050000 ^ 3)",
//    formula_symbolic: "pv = fv / (1 + r)^n",
//}
```
If you want to explore what happens in each period of the calculation, you can use the `.series()` method on any solution output:
### uses `present_value_solution().series()` to return a vec of each period
```
let future_value = 4_000;
let periods = 3;
let rate = 0.05;
let answer = present_value_solution(rate, periods, future_value);
dbg!(answer.series());

// PRINTS TO TERMINAL:
// answer.series() = TvmSeries(
//     [
//         TvmPeriod {
//             period: 0,
//             rate: 0.0,
//             value: 3455.3503941259037,
//             formula: "3455.3504 = 3628.1179 / 1.050000",
//             formula_symbolic: "value = {next period value} / (1 + r)",
//         },
//         TvmPeriod {
//             period: 1,
//             rate: 0.05,
//             value: 3628.117913832199,
//             formula: "3628.1179 = 3809.5238 / 1.050000",
//             formula_symbolic: "value = {next period value} / (1 + r)",
//         },
//         TvmPeriod {
//             period: 2,
//             rate: 0.05,
//             value: 3809.523809523809,
//             formula: "3809.5238 = 4000.0000 / 1.050000",
//             formula_symbolic: "value = {next period value} / (1 + r)",
//         },
//         TvmPeriod {
//             period: 3,
//             rate: 0.05,
//             value: 4000.0,
//             formula: "4000.0000",
//             formula_symbolic: "value = fv",
//         },
//     ],
// )
```
To view each period in a table format, use the `.print_table()` method.
### uses `present_value_solution().series().print_table()` to return a pretty-print table
```
use num_format::{Locale};

let future_value = 4_000;
let periods = 3;
let rate = 0.05;
let answer = present_value_solution(rate, periods, future_value);
dbg!(answer.series().print_table());
// or use .print_table_locale() to specify your formatting preferences.
dbg!(answer.series().print_table_locale(&Locale::en, 4));

// PRINTS TO TERMINAL:
// period    rate       value
// ------  ------  ----------
//      0  0.0000  3,455.3504
//      1  0.0500  3,628.1179
//      2  0.0500  3,809.5238
//      3  0.0500  4,000.0000
```
In the table above, you can specify the locale, if you prefer a different format for the money values. For example, your country may prefer `8.532,11` instead of `8,532.11`. The pretty-printed tables are easily copy&amp;pasted into a spreadsheet.

The `.print_table()` function can be especially helpful when analyzing `payment` and `cashflow` information.

> <img src="http://i.upmath.me/svg/%5Cbegin%7Bdocument%7D%5Cbegin%7Btikzpicture%7D%0A%25%20draw%20horizontal%20line%20%20%20%0A%5Cdraw%20(0%2C0)%20--%20(5%2C0)%3B%0A%25%20draw%20vertical%20lines%0A%5Cforeach%20%5Cx%20in%20%7B0%2C1%2C2%2C3%2C4%2C5%7D%20%5Cdraw%20(%5Cx%20cm%2C3pt)%20--%20(%5Cx%20cm%2C-3pt)%3B%0A%25%20draw%20nodes%0A%5Cdraw%20(0%2C0)%20node%5Bbelow%3D3pt%5D%20%7B%24%200%20%24%7D%20node%5Babove%3D3pt%5D%20%7B%24%20%5C%2413%2C000%20%20%24%7D%3B%0A%5Cdraw%20(1%2C0)%20node%5Bbelow%3D3pt%5D%20%7B%24%20%24%7D%20node%5Babove%3D3pt%5D%20%7B%24%20%24%7D%3B%0A%5Cdraw%20(2%2C0)%20node%5Bbelow%3D3pt%5D%20%7B%24%20PMT%3D%20%3F%20%24%7D%20node%5Babove%3D3pt%5D%20%7B%24%20%24%7D%3B%0A%5Cdraw%20(3%2C0)%20node%5Bbelow%3D3pt%5D%20%7B%24%20%24%7D%20node%5Babove%3D3pt%5D%20%7B%24%208%5C%25%20%24%7D%3B%0A%5Cdraw%20(4%2C0)%20node%5Bbelow%3D3pt%5D%20%7B%24%20%24%7D%20node%5Babove%3D3pt%5D%20%7B%24%20%24%7D%3B%0A%5Cdraw%20(5%2C0)%20node%5Bbelow%3D3pt%5D%20%7B%24%205%20%24%7D%20node%5Babove%3D3pt%5D%20%7B%24%20%20%24%7D%3B%0A%5Cend%7Btikzpicture%7D%0A%5Cend%7Bdocument%7D" />

### uses `payment_solution().series().print_table()` to return a pretty-print table
```
let present_value = 13_000;
let periods = 5;
let rate = 0.08;
let answer = payment_solution(rate, periods, present_value, 0);
dbg!(answer.series().print_table());

// PRINTS TO TERMINAL:
// period  payments_to_date  payments_remaining    principal  principal_to_date  principal_remaining     interest  interest_to_date  interest_remaining
// ------  ----------------  ------------------  -----------  -----------------  -------------------  -----------  ----------------  ------------------
//      1       -3,255.9339        -13,023.7356  -2,215.9339        -2,215.9339         -10,784.0661  -1,040.0000       -1,040.0000         -2,239.6695
//      2       -6,511.8678         -9,767.8017  -2,393.2086        -4,609.1425          -8,390.8575    -862.7253       -1,902.7253         -1,376.9443
//      3       -9,767.8017         -6,511.8678  -2,584.6653        -7,193.8078          -5,806.1922    -671.2686       -2,573.9939           -705.6757
//      4      -13,023.7356         -3,255.9339  -2,791.4385        -9,985.2464          -3,014.7536    -464.4954       -3,038.4893           -241.1803
//      5      -16,279.6695              0.0000  -3,014.7536       -12,999.0000              -0.0000    -241.1803       -3,279.6695              0.0000
```
As you can see in the table above, the `finance-solution` library offers more than "just the answer" to the problem (-3,255.9339) but also provides a detailed visual of what is happening in each period.

## Benefits of using finance-solution
This library has undergone hundreds of hours spent designing the library to be ergonomic, rustic, and accurate.

Bonus highlights include:
- finance-solution provides both f64 functions almost all functions can add `_solution` to the function name to provide a more helpful output, with additional functionality.  We highly recommend you use the `_solution` functions when you can!
- the `_solution` structs add a trivial amount of time to the code execution, 12-30 nanoseconds... as shown in our `benches` section.
- all the formulas have been rigorously tested, both in unit tests, integration tests, and "symmetry" tests.
- function parameters follow consistent ordering when possible, for example "rate, periods, ..." so the user can almost guess the ordering of parameters.
- table output of `_solution().series().print_table()` allows the user to specify locale for specific currency formatting, and the output can easily be copy&pasted into spreadsheets (Excel, Google Sheet, etc).
- Sample word problems are provided in the `examples` folder of the repo, which allows users to see how finance-solution can be used to solve financial problems.
- Functions have built-in asserts, panics, and even `warn!` logs to prevent users from making common mistakes, especially with rates.
- Functions are built for ease-of-use, so the user can provide f64 or f32 or u32 or i8... Any numeric format for money is accepted into the function and converted into f64, for user convenience.
- Functional parameters like rate and periods are strictly enforced to f64 for rate and u32 for periods. Periods are not allowed to be negative, and periods larger than 2000 will likely lead to computer-inherent floating point representation errors, so we provide a `warn!` for situations when we believe the inputs may create inaccuracies in the final output. It is up to the user to enable warn logs.

## To-do items for this crate
We have a backlog of items we intend to include in this crate:
- IRR
- MIRR
- WACC
- PV and FV annuity schedules (varying rates and cashflows)
- Payback Period
- Profitability Index
- Return on investment
- Amortization
- PPMT
- IPMT
- Perpetuities
- CUMPRINC
- CUMIPMT
- Bonds
- ROI
- Rule of 72 (and related fns)
- SLN
- XNPV
- XIRR