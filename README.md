# libf financial library for Rust

## Libf is a financial library for time-value-of-money problems.

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
```
let future_value = 4_000;
let periods = 3;
let rate = 0.05;
let pv = present_value(rate, periods, future_value);
dbg!(pv);

// PRINTS TO TERMINAL:
// pv = 3455.350394125904
```
For the same problem above, you can use the _solution function to see a better output.
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
To view each period in a table format, use the `.print_table()` method:
```
use num_format::{Locale};

let future_value = 4_000;
let periods = 3;
let rate = 0.05;
let answer = present_value_solution(rate, periods, future_value);
dbg!(answer.series().print_table(&Locale::en, 4));

// PRINTS TO TERMINAL:
// period    rate       value
// ------  ------  ----------
//      0  0.0000  3,455.3504
//      1  0.0500  3,628.1179
//      2  0.0500  3,809.5238
//      3  0.0500  4,000.0000
```
In the table above, you can specify the locale, if you prefer a different format for the money values. For example, your country may prefer `8.532,11` instead of `8,532.11`.

The `.print_table()` function can be especially helpful when analyzing `payment` information. For example:
```
use num_format::{Locale};

let present_value = 13_000;
let periods = 5;
let rate = 0.08;
let answer = payment_solution(rate, periods, present_value, 0);
dbg!(answer.series().print_table(&Locale::en, 4));

// PRINTS TO TERMINAL:
// period  payments_to_date  payments_remaining    principal  principal_to_date  principal_remaining     interest  interest_to_date  interest_remaining
// ------  ----------------  ------------------  -----------  -----------------  -------------------  -----------  ----------------  ------------------
//      1       -3,255.9339        -13,023.7356  -2,215.9339        -2,215.9339         -10,784.0661  -1,040.0000       -1,040.0000         -2,239.6695
//      2       -6,511.8678         -9,767.8017  -2,393.2086        -4,609.1425          -8,390.8575    -862.7253       -1,902.7253         -1,376.9443
//      3       -9,767.8017         -6,511.8678  -2,584.6653        -7,193.8078          -5,806.1922    -671.2686       -2,573.9939           -705.6757
//      4      -13,023.7356         -3,255.9339  -2,791.4385        -9,985.2464          -3,014.7536    -464.4954       -3,038.4893           -241.1803
//      5      -16,279.6695              0.0000  -3,014.7536       -12,999.0000              -0.0000    -241.1803       -3,279.6695              0.0000
```