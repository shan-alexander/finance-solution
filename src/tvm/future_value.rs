//! **Future value calculations.** Given an initial investment amount, a number of periods such as
//! periods, and fixed or varying interest rates, what is the value of the investment at the end?
//!
//! For most common usages, we recommend the [future_value_solution](./fn.future_value_solution.html) function, which provides a better debugging experience and additional features.
//! 
//! For more complex scenarios, which involve varying rates in each period, we recommend the [future_value_schedule_solution](./fn.future_value_schedule_solution.html) function.
//! 
//! To simply return an f64 value of the future value answer, use the [future_value](./fn.future_value.html) function.
//! 
// ! If you need to calculate the present value given a future value, a number of periods, and one
// ! or more rates use [`present_value`] or related functions.
// !
// ! If you need to calculate a fixed rate given a present value, future value, and number of periods
// ! use [`rate`] or related functions.
// !
// ! If you need to calculate the number of periods given a fixed rate and a present and future value
// ! use [`periods`] or related functions.
//!
//! ## Example
//! 
//! ```
//! let (rate, periods, present_value, continuous_compounding) = (0.034, 10, 1_000, false);
//! let fv = finance_solution::future_value_solution(rate, periods, present_value, continuous_compounding);
//! dbg!(fv);
//! ```
//! Outputs to terminal:
//! ```text
//! {
//!     calculated_field: FutureValue,
//!     continuous_compounding: false,
//!     rate: 0.034,
//!     periods: 10,
//!     fractional_periods: 10.0,
//!     present_value: 1000.0,
//!     future_value: 1397.0288910795477,
//!     formula: "1397.0289 = 1000.0000 * (1.034000 ^ 10)",
//!     symbolic_formula: "fv = pv * (1 + r)^n",
//! }
//! ```
//! # Formulas
//!
//! ## Simple Compounding
//!
//! With simple compound interest, the future value is calculated with:
//!
//! > <img src="http://i.upmath.me/svg/future%5C_value%20%3D%20present%5C_value%20%5Ctimes%20(1%2Brate)%5E%7Bperiods%7D" />
//!
//! Or with some more commonly-used variable names:
//!
//! > <img src="http://i.upmath.me/svg/fv%20%3D%20pv%20%5Ctimes%20(1%2Br)%5En" />
//!
//! `n` is often used for the number of periods, though it may be `t` for time if each period is
//! assumed to be one year as in continuous compounding. `r` is the periodic rate, though this may
//! appear as `i` for interest.
//!
//! Throughout this crate we use `pv` for present value and `fv` for future value. You may see these
//! values called `P` for principal in some references.
//!
//! Within the [TvmSolution](./struct.TvmSolution.html) struct we record the formula used for the particular calculation
//! using both concrete values and symbols. For example with $1,000 growing at 3.5% per period for
//! 12 periods using simple compounding the struct contains these fields:
//! ```text
//! formula: "1511.0687 = 1000.0000 * (1.035000 ^ 12)",
//! symbolic_formula: "fv = pv * (1 + r)^n",
//! ```
//!
//! ## Continuous Compounding
//!
//! With continuous compounding the formula is:
//!
//! > <img src="http://i.upmath.me/svg/future%5C_value%20%3D%20%7Bpresent%5C_value%20%5Ctimes%20e%5E%7Brate%20%5Ctimes%20periods%7D" />
//!
//! or:
//!
//! > <img src="http://i.upmath.me/svg/fv%20%3D%20pv%20%5Ctimes%20e%5E%7Br%20%5Ctimes%20n%7D" />
//!
//! With continuous compounding the period is assumed to be years and `t` (time) is often used as
//! the variable name. Within this crate we stick with `n` for the number of periods so that it's
//! easier to compare formulas when they're printed as simple text as part of the [TvmSolution](./struct.TvmSolution.html)
//! struct. For example with $1,000 growing at 3.5% per period for 12 periods using continuous
//! compounding the struct contains these fields:
//! ```text
//! formula: "1521.9616 = 1000.0000 * 2.718282^(0.035000 * 12)",
//! symbolic_formula: "fv = pv * e^(rt)",
//! ```
//! This is the same as the example in the previous section except that it uses continuous
//! compounding.

use log::warn;

use super::tvm::*;

#[allow(unused_imports)]
use crate::{rate::*, periods::*, present_value::*};

/// Returns the value of an investment after it has grown or shrunk over time, using a fixed rate.
///
/// See the [future_value](./index.html) module page for the formulas.
///
/// Related functions:
/// * To calculate a future value with a fixed rate and return a struct that shows the formula and
/// optionally produces the the period-by-period values use [`future_value_solution`].
/// * To calculate the future value if the rates vary by period use [`future_value_schedule`] or
/// [`future_value_schedule_solution`].
///
/// # Arguments
/// * `rate` - The rate at which the investment grows or shrinks per period, expressed as a
/// floating point number. For instance 0.05 would mean 5% growth. Often appears as `r` or `i` in
/// formulas.
/// * `periods` - The number of periods such as quarters or periods. Often appears as `n` or `t`.
/// * `present_value` - The starting value of the investment. May appear as `pv` in formulas, or `C`
/// for cash flow or `P` for principal.
/// * `continuous_compounding` - True for continuous compounding, false for simple compounding.
///
/// # Panics
/// The call will fail if `rate` is less than -1.0 as this would mean the investment is
/// losing more than its full value every period.
///
/// # Examples
/// Investment that grows quarter by quarter.
/// ```
/// use finance_solution::*;
///
/// // The investment grows by 3.4% per quarter.
/// let rate = 0.034;
///
/// // The investment will grow for 5 quarters.
/// let periods = 5;
///
/// // The initial investment is $250,000.
/// let present_value = -250_000;
///
/// let continuous_compounding = false;
///
/// let future_value = future_value(rate, periods, present_value, continuous_compounding);
/// // Confirm that the future value is correct to four decimal places (one
/// // hundredth of a cent).
/// assert_rounded_4(295_489.9418, future_value);
/// ```
/// Investment that loses money each year.
/// ```
/// # use finance_solution::*;
/// // The investment loses 5% per year.
/// let rate = -0.05;
///
/// // The investment will shrink for 6 periods.
/// let periods = 6;
///
/// // The initial investment is $10,000.75.
/// let present_value = -10_000.75;
///
/// let continuous_compounding = false;
///
/// let future_value = future_value(rate, periods, present_value, continuous_compounding);
/// // Confirm that the future value is correct to the penny.
/// assert_rounded_2(7351.47, future_value);
/// ```
/// Error case: The investment loses 105% per year. There's no way to work out
/// what this means so the call will panic.
/// ```should_panic
/// # use finance_solution::future_value;
/// let (rate, periods, present_value, continuous_compounding) = (-1.05, 6, 10_000.75, false);
/// let future_value = future_value(rate, periods, present_value, continuous_compounding);
/// ```
pub fn future_value<T>(rate: f64, periods: u32, present_value: T, continuous_compounding: bool) -> f64
    where T: Into<f64> + Copy
{
    future_value_internal(rate, periods as f64, present_value.into(), continuous_compounding)
}

/// Calculates the value of an investment after it has grown or shrunk over time and returns a
/// struct with the inputs and the calculated value. This is used for keeping track of a collection
/// of financial scenarios so that they can be examined later.
///
/// See the [future_value](./index.html) module page for the formulas.
///
/// Related functions:
/// * For simply calculating a single future value using a fixed rate use [`future_value`].
/// * To calculate the future value if the rates vary by period use [`future_value_schedule`].
/// * To calculate the future value with varying rates and return a struct that can produce the
/// period-by-period values use [`future_value_schedule_solution`].
///
/// # Arguments
/// * `rate` - The rate at which the investment grows or shrinks per period, expressed as a
/// floating point number. For instance 0.05 would mean 5% growth. Often appears as `r` or `i` in
/// formulas.
/// * `periods` - The number of periods such as quarters or periods. Often appears as `n` or `t`.
/// * `present_value` - The starting value of the investment. May appear as `pv` in formulas, or `C`
/// for cash flow or `P` for principal.
/// * `continuous_compounding` - True for continuous compounding, false for simple compounding.
///
/// # Panics
/// The call will fail if `rate` is less than -1.0 as this would mean the investment is
/// losing more than its full value every period.
///
/// # Examples
/// Calculate a future value and examine the period-by-period values.
/// ```
/// use finance_solution::*;
/// // The rate is 1.2% per month.
/// let rate = 0.012;
///
/// // The investment will grow for 8 months.
/// let periods = 8;
///
/// // The initial investment is $200,000.
/// let present_value = -200_000;
///
/// let continuous_compounding = false;
///
/// let solution = future_value_solution(rate, periods, present_value, continuous_compounding);
/// dbg!(&solution);
///
/// let future_value = solution.future_value();
/// assert_rounded_4(future_value, 220_026.0467);
///
/// // Examine the formulas.
/// let formula = solution.formula();
/// dbg!(&formula);
/// assert_eq!(formula, "220026.0467 = 200000.0000 * (1.012000 ^ 8)");
/// let symbolic_formula = solution.symbolic_formula();
/// dbg!(&symbolic_formula);
/// assert_eq!(symbolic_formula, "fv = -pv * (1 + r)^n");
///
/// // Calculate the value at the end of each period.
/// let series = solution.series();
/// dbg!(&series);
/// ```
/// Create a collection of future value calculations ranging over several interest rates.
/// ```
/// # use finance_solution::*;
///
/// // The initial investment is $100,000.
/// let present_value = -100_000;
///
/// // The investment will grow for 12 periods.
/// let periods = 12;
///
/// let continuous_compounding = false;
///
/// // We'll keep a collection of the calculated future values along with their inputs.
/// let mut scenarios = vec![];
///
/// for i in 2..=15 {
///     // The rate is between 2% and 15% per year.
///     let rate = f64::from(i) / 100.0;
///     // Calculate the future value for this periodic rate and add the details to the collection.
///     scenarios.push(future_value_solution(rate, periods, present_value, continuous_compounding));
/// }
/// dbg!(&scenarios);
/// assert_eq!(14, scenarios.len());
///
/// // Keep only the scenarios where the future value was between $200,000 and $400,000.
/// scenarios.retain(|x| x.future_value() >= 200_000.00 && x.future_value() <= 400_000.00);
/// dbg!(&scenarios);
/// assert_eq!(7, scenarios.len());
///
/// // Check the formulas for the first of the remainingc scenarios.
/// let formula = scenarios[0].formula();
/// dbg!(&formula);
/// assert_eq!("201219.6472 = 100000.0000 * (1.060000 ^ 12)", formula);
/// let symbolic_formula = scenarios[0].symbolic_formula();
/// dbg!(&symbolic_formula);
/// assert_eq!("fv = -pv * (1 + r)^n", symbolic_formula);
/// ```
pub fn future_value_solution<T>(rate: f64, periods: u32, present_value: T, continuous_compounding: bool) -> TvmSolution
    where T: Into<f64> + Copy
{
    future_value_solution_internal(rate, periods as f64, present_value.into(), continuous_compounding)
}

/// Calculates a future value based on rates that change for each period.
///
/// Related functions:
/// * To calculate the future value with varying rates and return a struct that can produce the
/// period-by-period values use [`future_value_schedule_solution`].
/// * If there is a single fixed rate use [`future_value`] or [`future_value_solution`].
///
/// # Arguments
/// * `rates` - A collection of rates, one for each period.
/// * `present_value` - The starting value of the investment.
///
/// # Panics
/// The call will fail if any of the rates is less than -1.0 as this would mean the investment is
/// losing more than its full value.
///
/// # Examples
/// Calculate the value of an investment whose rates vary by year.
/// ```
/// use finance_solution::*;
/// // The rates vary by year: 4% followed by -3.9%, 10.6%, and -5.7%.
/// let rates = [0.04, -0.039, 0.106, -0.057];
///
/// // The initial investment is $75,000.
/// let present_value = -75_000.00;
///
/// let future_value = future_value_schedule(&rates, present_value);
/// dbg!(&future_value);
/// assert_rounded_4(78_178.0458, future_value);
/// ```
/// Error case: One of the rates shows a drop of over 100%. There's no way to work out what this
/// means so the call will panic.
/// ```should_panic
/// # use finance_solution::future_value_schedule;
/// let rates = [0.116, -100.134, -0.09, 0.086];
/// let present_value = -4_000.00;
/// let schedule = future_value_schedule(&rates, present_value);
/// ```
pub fn future_value_schedule<T>(rates: &[f64], present_value: T) -> f64
    where T: Into<f64> + Copy
{
    let present_value= present_value.into();
    let periods = rates.len();

    // Check the parameters including all of the provided rates.
    for rate in rates {
        check_future_value_parameters(*rate, periods as f64, present_value);
    }

    let mut future_value = -present_value;
    for i in 0..periods {
        future_value *= 1.0 + rates[i];
    }

    future_value
}

/// Calculates a future value based on rates that change for each period, returning a struct with
/// all of the inputs and results.
///
/// Related functions:
/// * For simply calculating a single future value using a fixed rate use [`future_value`].
/// * To calculate a future value with a fixed rate and return a struct that shows the formula and
/// optionally produces the the period-by-period values use [`future_value_solution`].
/// * To calculate the future value if the rates vary by period use [`future_value_schedule`].
///
/// # Arguments
/// * `rates` - A collection of rates, one for each period.
/// * `present_value` - The starting value of the investment.
///
/// # Panics
/// The call will fail if any of the rates is less than -1.0 as this would mean the investment is
/// losing more than its full value.
///
/// # Examples
/// Calculate the value of an investment whose rates vary by year.
/// ```
/// use finance_solution::*;
/// // The rates vary by year: 8.1% followed by 11%, 4%, and -2.3%.
/// let rates = [0.081, 0.11, 0.04, -0.023];
///
/// // The initial investment is $10,000.
/// let present_value = -10_000.00;
///
/// let solution = future_value_schedule_solution(&rates, present_value);
/// dbg!(&solution);
///
/// let future_value = solution.future_value();
/// dbg!(&future_value);
/// assert_rounded_4(future_value, 12_192.0455);
///
/// // Calculate the value for each period.
/// let series = solution.series();
/// dbg!(&series);
/// ```
/// Error case: One of the rates shows a drop of over 100%. There's no way to work out what this
/// means so the call will panic.
/// ```should_panic
/// # use finance_solution::*;
/// let rates = [0.116, -100.134, -0.09, 0.086];
/// let present_value = -4_000.00;
/// let schedule = future_value_schedule(&rates, present_value);
/// ```
pub fn future_value_schedule_solution<T>(rates: &[f64], present_value: T) -> TvmScheduleSolution
    where T: Into<f64> + Copy
{
    let future_value = future_value_schedule(rates, present_value);
    TvmScheduleSolution::new(TvmVariable::FutureValue, rates, present_value.into(), future_value)
}

pub(crate) fn future_value_internal(rate: f64, periods: f64, present_value: f64, continuous_compounding: bool) -> f64 {
    check_future_value_parameters(rate, periods, present_value);
    let future_value = if continuous_compounding {
        // http://www.edmichaelreggie.com/TMVContent/rate.htm
        -present_value * std::f64::consts::E.powf(rate * periods)
    } else {
        -present_value * (1.0 + rate).powf(periods)
    };
    assert!(future_value.is_finite());
    future_value
}

pub(crate) fn future_value_solution_internal(rate: f64, periods: f64, present_value: f64, continuous_compounding: bool) -> TvmSolution {
    let future_value = future_value_internal(rate, periods, present_value, continuous_compounding);
    let (formula, symbolic_formula) = if continuous_compounding {
        let formula = format!("{:.4} = {:.4} * {:.6}^({:.6} * {})", future_value, -present_value, std::f64::consts::E, rate, periods);
        let symbolic_formula = "fv = -pv * e^(rt)";
        (formula, symbolic_formula)
    } else {
        let rate_multiplier = 1.0 + rate;
        assert!(rate_multiplier >= 0.0);
        let formula = format!("{:.4} = {:.4} * ({:.6} ^ {})", future_value, -present_value, rate_multiplier, periods);
        let symbolic_formula = "fv = -pv * (1 + r)^n";
        (formula, symbolic_formula)
    };
    TvmSolution::new_fractional_periods(TvmVariable::FutureValue, continuous_compounding, rate, periods, present_value, future_value, &formula, symbolic_formula)
}

fn check_future_value_parameters(rate: f64, _periods: f64, present_value: f64) {
    assert!(rate.is_finite(), "The rate must be finite (not NaN or infinity)");
    assert!(rate >= -1.0, "The rate must be greater than or equal to -1.0 because a rate lower than -100% would mean the investment loses more than its full value in a period.");
    if rate.abs() > 1. {
        warn!("You provided a periodic rate ({}) greater than 1. Are you sure you expect a {}% return?", rate, rate * 100.0);
    }
    assert!(present_value.is_finite(), "The present value must be finite (not NaN or infinity)");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::initialized_vector;

    #[should_panic]
    #[test]
    fn test_future_value_error_rate_low() {
        future_value(-101.0, 5, 250_000.00, false);
    }

    #[should_panic]
    #[test]
    fn test_future_value_solution_6() {
        // test infinity on rate
        let rate_of_return = 1.0f64 / 0.0f64;
        let periods = 6;
        let present_value = 5_000.00;
        let _should_panic = future_value_solution(rate_of_return, periods, present_value, false);
    }

    #[should_panic]
    #[test]
    fn test_future_value_solution_7() {
        // test infinity on fv
        let rate_of_return = 0.03;
        let periods = 6;
        let present_value = 1.0f64 / 0.0f64;
        let _should_panic = future_value_solution(rate_of_return, periods, present_value, false);
    }

    /*
    macro_rules! compare_to_excel {
        ( $r:expr, $n:expr, $pv:expr, $fv_excel:expr, $fv_manual_simple:expr, $fv_manual_cont:expr ) => {
            println!("$r = {}, $n = {}, $pv = {}, $fv_excel: {}, $fv_manual_simple = {}, $fv_manual_cont = {}", $r, $n, $pv, $fv_excel, $fv_manual_simple, $fv_manual_cont);
            assert_approx_equal!($fv_excel, $fv_manual_simple);

            let fv_calc_simple = future_value($r, $n, $pv, false);
            println!("fv_calc_simple = {}", fv_calc_simple);
            assert_approx_equal!($fv_excel, fv_calc_simple);

            let fv_calc_cont = future_value($r, $n, $pv, true);
            println!("fv_calc_cont = {}", fv_calc_cont);
            assert_approx_equal!($fv_manual_cont, fv_calc_cont);

            let ratio = fv_calc_cont / fv_calc_simple;
            println!("ratio = {}", ratio);
            assert!(ratio >= 1.0);
            assert!(ratio < 2.0);
        }
    }
    */
    
    fn compare_to_excel (test_case: usize, r: f64, n: u32, pv: f64, fv_excel: f64, fv_manual_simple: f64, fv_manual_cont: f64) {
        let display = false;

        if display { println!("test_case = {}, r = {}, n = {}, pv = {}, fv_excel: {}, fv_manual_simple = {}, fv_manual_cont = {}", test_case, r, n, pv, fv_excel, fv_manual_simple, fv_manual_cont); }
        assert_approx_equal!(fv_excel, fv_manual_simple);

        let fv_calc_simple = future_value(r, n, pv, false);
        if display { println!("fv_calc_simple = {}", fv_calc_simple) };
        assert_approx_equal!(fv_excel, fv_calc_simple);

        let fv_calc_cont = future_value(r, n, pv, true);
        if display { println!("fv_calc_cont = {}", fv_calc_cont) };
        assert_approx_equal!(fv_manual_cont, fv_calc_cont);

        let ratio = fv_calc_cont / fv_calc_simple;
        if display { println!("ratio = {}", ratio) };
        assert!(ratio >= 1.0);
        assert!(ratio < 2.0);

        // Solution with simple compounding.
        let solution = future_value_solution(r, n, pv, false);
        if display { dbg!(&solution); }
        solution.invariant();
        assert!(solution.calculated_field().is_future_value());
        assert_eq!(false, solution.continuous_compounding());
        assert_approx_equal!(r, solution.rate());
        assert_eq!(n, solution.periods());
        assert_approx_equal!(n as f64, solution.fractional_periods());
        assert_approx_equal!(pv, solution.present_value());
        assert_approx_equal!(fv_excel, solution.future_value());

        // Solution with continuous compounding.
        let solution = future_value_solution(r, n, pv, true);
        if display { dbg!(&solution); }
        solution.invariant();
        assert!(solution.calculated_field().is_future_value());
        assert!(solution.continuous_compounding());
        assert_approx_equal!(r, solution.rate());
        assert_eq!(n, solution.periods());
        assert_approx_equal!(n as f64, solution.fractional_periods());
        assert_approx_equal!(pv, solution.present_value());
        assert_approx_equal!(fv_manual_cont, solution.future_value());

        let rates = initialized_vector(n as usize, r);

        // Schedule solution.
        let solution = future_value_schedule_solution(&rates, pv);
        if display { dbg!(&solution); }
        solution.invariant();
        assert!(solution.calculated_field().is_future_value());
        assert_eq!(n, solution.periods());
        assert_approx_equal!(pv, solution.present_value());
        assert_approx_equal!(fv_excel, solution.future_value());
    }

    #[test]
    fn test_future_value_against_excel() {
        compare_to_excel(1, 0.01f64, 90, 1f64, -2.44863267464848f64, -2.44863267464848f64, -2.45960311115695f64);
        compare_to_excel(2, -0.01f64, 85, -1.5f64, 0.638385185082981f64, 0.638385185082981f64, 0.64112239792309f64);
        compare_to_excel(3, 0f64, 80, 2.25f64, -2.25f64, -2.25f64, -2.25f64);
        compare_to_excel(4, 0.05f64, 75, -3.375f64, 131.060314992675f64, 131.060314992675f64, 143.508651750212f64);
        compare_to_excel(5, -0.05f64, 70, 5.0625f64, -0.139642432836174f64, -0.139642432836174f64, -0.152874253575487f64);
        compare_to_excel(6, 0.01f64, 65, -7.59375f64, 14.499251769574f64, 14.499251769574f64, 14.5461381703243f64);
        compare_to_excel(7, -0.01f64, 60, 11.390625f64, -6.23245612973226f64, -6.23245612973226f64, -6.25130754238352f64);
        compare_to_excel(8, 0f64, 55, -17.0859375f64, 17.0859375f64, 17.0859375f64, 17.0859375f64);
        compare_to_excel(9, 0.05f64, 50, 25.62890625f64, -293.896914040351f64, -293.896914040351f64, -312.223995610061f64);
        compare_to_excel(10, -0.05f64, 45, -38.443359375f64, 3.82281753569715f64, 3.82281753569715f64, 4.05190026767808f64);
        compare_to_excel(11, 0.01f64, 40, 57.6650390625f64, -85.8553853561044f64, -85.8553853561044f64, -86.0261294638861f64);
        compare_to_excel(12, -0.01f64, 35, -86.49755859375f64, 60.8465082158636f64, 60.8465082158636f64, 60.9537993307622f64);
        compare_to_excel(13, 0f64, 30, 129.746337890625f64, -129.746337890625f64, -129.746337890625f64, -129.746337890625f64);
        compare_to_excel(14, 0.05f64, 25, -194.619506835937f64, 659.050728569279f64, 659.050728569279f64, 679.288825069511f64);
        compare_to_excel(15, -0.05f64, 20, 291.929260253906f64, -104.652530140165f64, -104.652530140165f64, -107.3947731238f64);
        compare_to_excel(16, 0.01f64, 15, -437.893890380859f64, 508.381212478371f64, 508.381212478371f64, 508.760116525988f64);
        compare_to_excel(17, -0.01f64, 12, 656.840835571289f64, -582.213779775772f64, -582.213779775772f64, -582.56556073855f64);
        compare_to_excel(18, 0f64, 10, -985.261253356933f64, 985.261253356933f64, 985.261253356933f64, 985.261253356933f64);
        compare_to_excel(19, 0.05f64, 7, 1477.8918800354f64, -2079.54228903805f64, -2079.54228903805f64, -2097.22840728772f64);
        compare_to_excel(20, -0.05f64, 5, -2216.8378200531f64, 1715.34684668614f64, 1715.34684668614f64, 1726.47503019966f64);
        compare_to_excel(21, 0.01f64, 4, 3325.25673007965f64, -3460.27548760037f64, -3460.27548760037f64, -3460.96303162265f64);
        compare_to_excel(22, -0.01f64, 3, -4987.88509511947f64, 4839.73991990933f64, 4839.73991990933f64, 4840.47081241187f64);
        compare_to_excel(23, 0f64, 2, 7481.82764267921f64, -7481.82764267921f64, -7481.82764267921f64, -7481.82764267921f64);
        compare_to_excel(24, 0.05f64, 1, -11222.7414640188f64, 11783.8785372198f64, 11783.8785372198f64, 11798.1437232237f64);
        compare_to_excel(25, -0.05f64, 0, 16834.1121960282f64, -16834.1121960282f64, -16834.1121960282f64, -16834.1121960282f64);
    }

}

// Comment should be on dev branch.
// Second comment should be on dev branch.

