//! **Present value calculations.** Given a final amount, a number of periods such as years, and fixed
//! or varying interest rates, what is the current value?
//!
//! For most common usages, we recommend the [`present_value_solution`](./fn.present_value_solution.html) function to provide a better debugging experience and additional features.
//! 
//! If you have a more complicated use case which has varying rates per period, use the [`present_value_schedule_solution`](./fn.present_value_schedule_solution.html) function.
//! 
// ! If you need to calculate the future value given a present value, a number of periods, and one
// ! or more rates use [`future_value`] or related functions.
// !
// ! If you need to calculate a fixed rate given a present value, future value, and number of periods
// ! use [`rate`] or related functions.
// !
// ! If you need to calculate the number of periods given a fixed rate and a present and future value
// ! use [`periods`] or related functions.
//! # Formulas
//!
//! ## Simple Compounding
//!
//! With simple compound interest, the present value is calculated with:
//!
//! > <img src="http://i.upmath.me/svg/present%5C_value%20%3D%20%7Bfuture%5C_value%20%5Cover%20(1%2Brate)%5E%7Bperiods%7D%7D" />
//!
//! Or using some more usual variable names:
//!
//! > <img src="http://i.upmath.me/svg/pv%20%3D%20%7Bfv%20%5Cover%20(1%2Br)%5En%7D" />
//!
//! `n` is often used for the number of periods, though it may be `t` for time if each period is
//! assumed to be one year as in continuous compounding. `r` is the periodic rate, though this may
//! appear as `i` for interest.
//!
//! Throughout this crate we use `pv` for present value and `fv` for future value. You may see these
//! values called `P` for principal in some references.
//!
//! Within the [TvmSolution](././tvm_simple/struct.TvmSolution.html) struct we record the formula used for the particular calculation
//! using both concrete values and symbols. For example if we calculated the present value of an
//! investment that grows by 1.5% per month for 48 months using simple compounding and reaches a
//! future value of $50,000 the solution struct would contain these fields:
//! ```text
//! formula: "24468.0848 = 50000.0000 / (1.015000 ^ 48)",
//! symbolic_formula: "pv = fv / (1 + r)^n",
//! ```
//!
//! ## Continuous Compounding
//!
//! With continuous compounding the formula is:
//!
//! > <img src="http://i.upmath.me/svg/present%5C_value%20%3D%20%7Bfuture%5C_value%20%5Cover%20e%5E%7Brate%20%5Ctimes%20periods%7D%7D" />
//!
//! or:
//!
//! > <img src="http:i.upmath.me/svg/pv%20%3D%20%7Bfv%20%5Cover%20e%5E%7Br%20%5Ctimes%20n%7D%7D" />
//!
//! With continuous compounding the period is assumed to be years and `t` (time) is often used as
//! the variable name. Within this crate we stick with `n` for the number of periods so that it's
//! easier to compare formulas when they're printed as simple text as part of the [TvmSolution](./struct.TvmSolution.html)
//! struct. Taking the example above but switching to continuous compounding the struct would
//! contain these fields:
//! ```text
//! formula: "24337.6128 = 50000.0000 / 2.718282^(0.015000 * 48)",
//! symbolic_formula: "pv = fv / e^(rt)",
//! ```

use log::warn;

use super::tvm::*;

/// Returns the current value of a future amount using a fixed rate.
///
/// Related functions:
/// * To calculate a present value with a fixed rate and return a struct that shows the formula and
/// optionally produces the the period-by-period values use [`present_value_solution`](./fn.present_value_solution.html).
/// * To calculate the present value if the rates vary by period use [`present_value_schedule`](./fn.present_value_schedule.html)
/// or [`present_value_schedule_solution`](./fn.present_value_schedule_solution.html).
///
/// See the [present_value](./index.html) module page for the formulas.
///
/// # Arguments
/// * `rate` - The rate at which the investment grows or shrinks per period,
/// expressed as a floating point number. For instance 0.05 would mean 5% growth. Often appears as
/// `r` or `i` in formulas.
/// * `periods` - The number of periods such as quarters or years. Often appears as `n` or `t`.
/// * `future_value` - The final value of the investment.
/// * `continuous_compounding` - True for continuous compounding, false for simple compounding.
///
/// # Panics
/// The call will fail if `rate` is less than -1.0 as this would mean the investment is
/// losing more than its full value every period. It will fail also if the future value is zero as
/// in this case there's no way to determine the present value.
///
/// # Examples
/// Investment that grows month by month.
/// ```
/// use finance_solution::*;
///
/// // The investment will grow by 1.1% per month.
/// let rate = 0.011;
///
/// // The investment will grow for 12 months.
/// let periods = 12;
///
/// // The final value will be $50,000.
/// let future_value = 50_000;
///
/// let continuous_compounding = false;
///
/// // Find the current value.
/// let present_value = present_value(rate, periods, future_value, continuous_compounding);
/// dbg!(&present_value);
///
/// // Confirm that the present value is correct to four decimal places (one hundredth of a cent).
/// assert_rounded_4(-43_848.6409, present_value);
/// ```
/// Error case: The investment loses 105% per year. There's no way to work out what this means so
/// the call to present_value() will panic.
/// ```should_panic
/// let rate = -1.05;
/// let periods = 6;
/// let present_value = -10_000.75;
/// let continuous_compounding = false;
/// let present_value = finance_solution::present_value(rate, periods, present_value, continuous_compounding);
/// ```
pub fn present_value<T>(rate: f64, periods: u32, future_value: T, continuous_compounding: bool) -> f64
    where T: Into<f64> + Copy
{
    present_value_internal(rate, periods as f64, future_value.into(), continuous_compounding)
}

/// Calculates the current value of a future amount using a fixed rate and returns a struct
/// with the inputs and the calculated value. This is used for keeping track of a collection of
/// financial scenarios so that they can be examined later.
///
/// See the [present_value](./index.html) module page for the formulas.
///
/// Related functions:
/// * For simply calculating a single present value using a fixed rate use [`present_value`](./fn.present_value.html).
/// * To calculate the present value if the rates vary by period use [`present_value_schedule`](./fn.present_value_schedule.html)
/// or [`present_value_schedule_solution`](./fn.present_value_schedule_solution.html).
///
/// # Arguments
/// * `rate` - The rate at which the investment grows or shrinks per period,
/// expressed as a floating point number. For instance 0.05 would mean 5% growth. Often appears as
/// `r` or `i` in formulas.
/// * `periods` - The number of periods such as quarters or years. Often appears as `n` or `t`.
/// * `future_value` - The final value of the investment.
/// * `continuous_compounding` - True for continuous compounding, false for simple compounding.
///
/// # Panics
/// The call will fail if `rate` is less than -1.0 as this would mean the investment is
/// losing more than its full value every period. It will fail also if the future value is zero as
/// in this case there's no way to determine the present value.
///
/// # Examples
/// Calculate a present value and examine the period-by-period values.
/// ```
/// use finance_solution::*;
///
/// // The rate is 8.45% per year.
/// let rate = 0.0845;
///
/// // The investment will grow for six years.
/// let periods = 6;
///
/// // The final value is $50,000.
/// let future_value = 50_000;
///
/// let continuous_compounding = false;
///
/// // Calculate the present value and create a struct with the input values and
/// // the formula used.
/// let solution = present_value_solution(rate, periods, future_value, continuous_compounding);
/// dbg!(&solution);
///
/// let present_value = solution.present_value();
/// assert_rounded_4(present_value, -30_732.1303);
///
/// // Examine the formulas.
/// let formula = solution.formula();
/// dbg!(&formula);
/// assert_eq!(formula, "-30732.1303 = -50000.0000 / (1.084500 ^ 6)");
/// let symbolic_formula = solution.symbolic_formula();
/// dbg!(&symbolic_formula);
/// assert_eq!("pv = -fv / (1 + r)^n", symbolic_formula);
///
/// // Calculate the amount at the end of each period.
/// let series = solution.series();
/// dbg!(&series);
/// ```
/// Build a collection of present value calculations where the future value and periodic rate are
/// fixed but the number of periods varies, then filter the results.
/// ```
/// // The rate is 0.9% per month.
/// # use finance_solution::*;
/// let rate = 0.009;
///
/// // The final value is $100,000.
/// let future_value = 100_000;
///
/// let continuous_compounding = false;
///
/// // We'll keep a collection of the calculated present values along with their inputs.
/// let mut scenarios = vec![];
///
/// // Calculate the present value for terms ranging from 1 to 36 months.
/// for periods in 1..=36 {
///     // Calculate the future value for this number of months and add the details to the
///     // collection.
///     scenarios.push(present_value_solution(rate, periods, future_value, continuous_compounding));
/// }
/// dbg!(&scenarios);
/// assert_eq!(36, scenarios.len());
///
/// // Keep only the scenarios where the present value (which is negative) is greater than or
/// // than or equal to -$80,000.
/// scenarios.retain(|x| x.present_value() >= -80_000.00);
/// dbg!(&scenarios);
/// assert_eq!(12, scenarios.len());
///
/// // Find the range of months for the remaining scenarios.
/// let min_months = scenarios.iter().map(|x| x.periods()).min().unwrap();
/// let max_months = scenarios.iter().map(|x| x.periods()).max().unwrap();
/// dbg!(min_months, max_months);
/// assert_eq!(25, min_months);
/// assert_eq!(36, max_months);
///
/// // Check the formulas for the first of the remaining scenarios.
/// let formula = scenarios[0].formula();
/// dbg!(&formula);
/// assert_eq!("-79932.0303 = -100000.0000 / (1.009000 ^ 25)", formula);
/// let symbolic_formula = scenarios[0].symbolic_formula();
/// dbg!(&symbolic_formula);
/// assert_eq!("pv = -fv / (1 + r)^n", symbolic_formula);
///
/// ```
/// Error case: The investment loses 111% per year. There's no way to work out what this means so
/// the call to present_value() will panic.
/// ```should_panic
/// # use finance_solution::*;
/// let rate = -1.11;
/// let periods = 12;
/// let present_value = 100_000.85;
/// let continuous_compounding = false;
/// let present_value = present_value_solution(rate, periods, present_value, continuous_compounding);
/// ```
pub fn present_value_solution<T>(rate: f64, periods: u32, future_value: T, continuous_compounding: bool) -> TvmSolution
    where T: Into<f64> + Copy
{
    present_value_solution_internal(rate, periods as f64, future_value.into(), continuous_compounding)
}

/// Calculates a present value based on rates that change for each period.
///
/// Related functions:
/// * To calculate the present value with varying rates and return a struct that can produce the
/// period-by-period values use [`present_value_schedule_solution`](./fn.present_value_schedule_solution.html).
/// * If there is a single fixed rate use [present_value](./fn.present_value.html) or [present_value_solution](./fn.present_value_solution.html).
///
/// # Arguments
/// * `rates` - A collection of rates, one for each period.
/// * `future_value` - The ending value of the investment.
///
/// # Panics
/// The call will fail if any of the rates is less than -1.0 as this would mean the investment is
/// losing more than its full value every period. It will fail also if the future value is zero as
/// in this case there's no way to determine the present value.
///
/// # Examples
/// Calculate the present value of an investment whose rates vary by year.
/// ```
/// // The annual rate varies from -3.4% to 12.9%.
/// let rates = [0.04, -0.034, 0.0122, 0.129, 8.5];
///
/// // The value of the investment after applying all of these periodic rates
/// // will be $30_000.
/// let future_value = 30_000.00;
///
/// // Calculate the present value.
/// let present_value = finance_solution::present_value_schedule(&rates, future_value);
/// dbg!(&present_value);
/// ```
pub fn present_value_schedule<T>(rates: &[f64], future_value: T) -> f64
    where T: Into<f64> + Copy
{
    let periods = rates.len();
    let future_value = future_value.into();

    // Check the parameters including all of the provided rates.
    for rate in rates {
        check_present_value_parameters(*rate, periods as f64, future_value);
    }

    let mut present_value = -future_value;
    for i in (0..periods).rev() {
        present_value /= 1.0 + rates[i];
    }

    present_value
}

/// Calculates a present value based on rates that change for each period and returns a struct
/// with the inputs and the calculated value.
///
/// Related functions:
/// * To calculate the present value as a single number if the rates vary by period use
/// [present_value_schedule](./fn.present_value_schedule.html).
/// * If there is a single fixed rate use [present_value](./fn.present_value.html) or
/// [present_value_solution](./fn.present_value_solution.html).
///
/// # Arguments
/// * `rates` - A collection of rates, one for each period.
/// * `future_value` - The ending value of the investment.
///
/// # Panics
/// The call will fail if any of the rates is less than -1.0 as this would mean the investment is
/// losing more than its full value every period. It will fail also if the future value is zero as
/// in this case there's no way to determine the present value.
///
/// # Examples
/// Calculate the value of an investment whose rates vary by year, then view only those periods
/// where the rate is negative.
/// ```
/// use finance_solution::*;
///
/// // The quarterly rate varies from -0.5% to 4%.
/// let rates = [0.04, 0.008, 0.0122, -0.005];
///
/// // The value of the investment after applying all of these periodic rates
/// // will be $25_000.
/// let future_value = 25_000.00;
///
/// // Calculate the present value and keep track of the inputs and the formula
/// // in a struct.
/// let solution = present_value_schedule_solution(&rates, future_value);
/// dbg!(&solution);
///
/// let present_value = solution.present_value();
/// assert_rounded_4(present_value, -23_678.6383);
///
/// // Calculate the value for each period.
/// let series = solution.series();
/// dbg!(&series);
/// ```
pub fn present_value_schedule_solution<T>(rates: &[f64], future_value: T) -> TvmScheduleSolution
    where T: Into<f64> + Copy
{
    let present_value = present_value_schedule(rates, future_value);
    TvmScheduleSolution::new(TvmVariable::PresentValue, rates, present_value, future_value.into())
}

pub(crate) fn present_value_internal(rate: f64, periods: f64, future_value: f64, continuous_compounding: bool) -> f64 {
    check_present_value_parameters(rate, periods, future_value);

    let present_value = if is_approx_equal!(0.0, rate) || is_approx_equal!(0.0, periods) {
        // The rate is zero and/or there are no periods, so there's no compounding going on. The
        // present value must be the same as the future value (with signs reversed).
        -future_value
    } else if continuous_compounding {
        -future_value / std::f64::consts::E.powf(rate * periods as f64)
    } else {
        -future_value / (1. + rate).powf(periods)
    };
    assert!(present_value.is_finite());
    present_value
}

pub(crate) fn present_value_solution_internal(rate: f64, periods: f64, future_value: f64, continuous_compounding: bool) -> TvmSolution {
    let present_value = present_value_internal(rate, periods, future_value, continuous_compounding);
    let rate_multiplier = 1.0 + rate;
    assert!(rate_multiplier >= 0.0);
    let (formula, symbolic_formula) = if continuous_compounding {
        let formula = format!("{:.4} = {:.4} / {:.6}^({:.6} * {})", present_value, -future_value, std::f64::consts::E, rate, periods);
        let symbolic_formula = "pv = -fv / e^(rt)";
        (formula, symbolic_formula)
    } else {
        let formula = format!("{:.4} = {:.4} / ({:.6} ^ {})", present_value, -future_value, rate_multiplier, periods);
        let symbolic_formula = "pv = -fv / (1 + r)^n";
        (formula, symbolic_formula)
    };
    TvmSolution::new_fractional_periods(TvmVariable::PresentValue, TvmCalculationType::Excel, continuous_compounding, rate, periods, present_value, future_value, &formula, symbolic_formula)
}

fn check_present_value_parameters(rate: f64, periods: f64, future_value: f64) {
    assert!(rate.is_finite(), "The rate must be finite (not NaN or infinity)");
    assert!(rate >= -1.0, "The rate must be greater than or equal to -1.0 because a rate lower than -100% would mean the investment loses more than its full value in a period.");
    if rate.abs() > 1. {
        warn!("You provided a periodic rate ({}) greater than 1. Are you sure you expect a {}% return?", rate, rate * 100.0);
    }
    assert!(future_value.is_finite(), "The future value must be finite (not NaN or infinity)");
    if !is_approx_equal!(0.0, rate) && periods > 0.0 {
        // rate and periods are both nonzero, so if future value is zero there's no way to work out
        // the present value.
        assert!(!is_approx_equal!(0.0, future_value), "The future value is zero but the rate is nonzero and periods > 0 so there is no way to calculate the present value.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    
    #[test]
    fn test_present_value_schedule() {
        let rates = [0.04, 0.07, -0.12, -0.03, 0.11];
        let future_value = 100_000.25;

        let present_value = present_value_schedule(&rates, future_value);
        assert_rounded_4(-94843.2841, present_value);

        let solution = present_value_schedule_solution(&rates, future_value);
        assert_rounded_4(100000.2500, solution.future_value());
        assert_rounded_4(-94843.2841, solution.present_value());

        let series = solution.series();
        assert_eq!(6, series.len());

        let period = &series[0];
        assert_eq!(0, period.period());
        assert_rounded_6(0.0, period.rate());
        assert_rounded_4(-present_value,period.value());

        let period = &series[1];
        assert_eq!(1, period.period());
        assert_rounded_6(0.04, period.rate());
        assert_rounded_4(98_637.0154,period.value());

        let period = &series[2];
        assert_eq!(2, period.period());
        assert_rounded_6(0.07, period.rate());
        assert_rounded_4(105_541.6065,period.value());

        let period = &series[3];
        assert_eq!(3, period.period());
        assert_rounded_6(-0.12, period.rate());
        assert_rounded_4(92_876.6137,period.value());

        let period = &series[4];
        assert_eq!(4, period.period());
        assert_rounded_6(-0.03, period.rate());
        assert_rounded_4(90_090.3153, period.value());

        let period = &series[5];
        assert_eq!(5, period.period());
        assert_rounded_6(0.11, period.rate());
        assert_rounded_4(100_000.2500, period.value());
    }

    /*
    macro_rules! compare_to_excel {
        ( $r:expr, $n:expr, $fv:expr, $pv_excel:expr, $pv_manual_simple:expr, $pv_manual_cont:expr ) => {
            println!("$r = {}, $n = {}, $fv = {}, $pv_excel: {}, $pv_manual_simple = {}, $pv_manual_cont = {}", $r, $n, $fv, $pv_excel, $pv_manual_simple, $pv_manual_cont);
            assert_approx_equal!($pv_excel, $pv_manual_simple);

            let pv_calc_simple = present_value($r, $n, $fv, false);
            println!("pv_calc_simple = {}", pv_calc_simple);
            assert_approx_equal!($pv_excel, pv_calc_simple);

            let pv_calc_cont = present_value($r, $n, $fv, true);
            println!("pv_calc_cont = {}", pv_calc_cont);
            assert_approx_equal!($pv_manual_cont, pv_calc_cont);

            let ratio = pv_calc_cont / pv_calc_simple;
            println!("ratio = {}", ratio);
            assert!(ratio > 0.0);
            assert!(ratio <= 1.0);
        }
    }
    */

    fn compare_to_excel (test_case: usize, r: f64, n: u32, fv: f64, pv_excel: f64, pv_manual_simple: f64, pv_manual_cont: f64) {
        let display = false;

        if display { println!("test_case = {}, r = {}, n = {}, fv = {}, pv_excel: {}, pv_manual_simple = {}, pv_manual_cont = {}", test_case, r, n, fv, pv_excel, pv_manual_simple, pv_manual_cont) };
        assert_approx_equal!(pv_excel, pv_manual_simple);

        let pv_calc_simple = present_value(r, n, fv, false);
        if display { println!("pv_calc_simple = {}", pv_calc_simple) };
        assert_approx_equal!(pv_excel, pv_calc_simple);

        let pv_calc_cont = present_value(r, n, fv, true);
        if display { println!("pv_calc_cont = {}", pv_calc_cont) };
        assert_approx_equal!(pv_manual_cont, pv_calc_cont);

        let ratio = pv_calc_cont / pv_calc_simple;
        if display { println!("ratio = {}", ratio) };
        assert!(ratio >= 0.0);
        assert!(ratio <= 1.0);

        // Solution with simple compounding.
        let solution = present_value_solution(r, n, fv, false);
        if display { dbg!(&solution); }
        solution.invariant();
        assert!(solution.calculated_field().is_present_value());
        assert_eq!(false, solution.continuous_compounding());
        assert_approx_equal!(r, solution.rate());
        assert_eq!(n, solution.periods());
        assert_approx_equal!(n as f64, solution.fractional_periods());
        assert_approx_equal!(pv_excel, solution.present_value());
        assert_approx_equal!(fv, solution.future_value());

        // Solution with continuous compounding.
        let solution = present_value_solution(r, n, fv, true);
        if display { dbg!(&solution); }
        solution.invariant();
        assert!(solution.calculated_field().is_present_value());
        assert!(solution.continuous_compounding());
        assert_approx_equal!(r, solution.rate());
        assert_eq!(n, solution.periods());
        assert_approx_equal!(n as f64, solution.fractional_periods());
        assert_approx_equal!(pv_manual_cont, solution.present_value());
        assert_approx_equal!(fv, solution.future_value());

        let rates = initialized_vector(n as usize, r);

        // Schedule solution.
        let solution = present_value_schedule_solution(&rates, fv);
        if display { dbg!(&solution); }
        solution.invariant();
        assert!(solution.calculated_field().is_present_value());
        assert_eq!(n, solution.periods());
        assert_approx_equal!(pv_excel, solution.present_value());
        assert_approx_equal!(fv, solution.future_value());
    }

    #[test]
    fn test_present_value_against_excel() {
        compare_to_excel(1, 0.01f64, 90, 1f64, -0.408391185151344f64, -0.408391185151344f64, -0.406569659740599f64);
        compare_to_excel(2, -0.01f64, 85, -1.5f64, 3.52451788132823f64, 3.52451788132823f64, 3.50947027788899f64);
        compare_to_excel(3, 0f64, 80, 2.25f64, -2.25f64, -2.25f64, -2.25f64);
        compare_to_excel(4, 0.05f64, 75, -3.375f64, 0.0869113201859512f64, 0.0869113201859512f64, 0.0793723922640307f64);
        compare_to_excel(5, -0.05f64, 70, 5.0625f64, -183.53236712846f64, -183.53236712846f64, -167.64697554088f64);
        compare_to_excel(6, 0.01f64, 65, -7.59375f64, 3.97710447262579f64, 3.97710447262579f64, 3.96428511727897f64);
        compare_to_excel(7, -0.01f64, 60, 11.390625f64, -20.8178501685176f64, -20.8178501685176f64, -20.7550719606981f64);
        compare_to_excel(8, 0f64, 55, -17.0859375f64, 17.0859375f64, 17.0859375f64, 17.0859375f64);
        compare_to_excel(9, 0.05f64, 50, 25.62890625f64, -2.23493614322574f64, -2.23493614322574f64, -2.10374873426328f64);
        compare_to_excel(10, -0.05f64, 45, -38.443359375f64, 386.597546504632f64, 386.597546504632f64, 364.740438412197f64);
        compare_to_excel(11, 0.01f64, 40, 57.6650390625f64, -38.7309044888379f64, -38.7309044888379f64, -38.6540316390219f64);
        compare_to_excel(12, -0.01f64, 35, -86.49755859375f64, 122.962317182378f64, 122.962317182378f64, 122.745878432934f64);
        compare_to_excel(13, 0f64, 30, 129.746337890625f64, -129.746337890625f64, -129.746337890625f64, -129.746337890625f64);
        compare_to_excel(14, 0.05f64, 25, -194.619506835937f64, 57.4716797951039f64, 57.4716797951039f64, 55.7594222710607f64);
        compare_to_excel(15, -0.05f64, 20, 291.929260253906f64, -814.33953749853f64, -814.33953749853f64, -793.546003343685f64);
        compare_to_excel(16, 0.01f64, 15, -437.893890380859f64, 377.179672510109f64, 377.179672510109f64, 376.898764278606f64);
        compare_to_excel(17, -0.01f64, 12, 656.840835571289f64, -741.033445550103f64, -741.033445550103f64, -740.585974095395f64);
        compare_to_excel(18, 0f64, 10, -985.261253356933f64, 985.261253356933f64, 985.261253356933f64, 985.261253356933f64);
        compare_to_excel(19, 0.05f64, 7, 1477.8918800354f64, -1050.31016709206f64, -1050.31016709206f64, -1041.45280575294f64);
        compare_to_excel(20, -0.05f64, 5, -2216.8378200531f64, 2864.94240503709f64, 2864.94240503709f64, 2846.47610562283f64);
        compare_to_excel(21, 0.01f64, 4, 3325.25673007965f64, -3195.50635796575f64, -3195.50635796575f64, -3194.87154873072f64);
        compare_to_excel(22, -0.01f64, 3, -4987.88509511947f64, 5140.56501667989f64, 5140.56501667989f64, 5139.78881110503f64);
        compare_to_excel(23, 0f64, 2, 7481.82764267921f64, -7481.82764267921f64, -7481.82764267921f64, -7481.82764267921f64);
        compare_to_excel(24, 0.05f64, 1, -11222.7414640188f64, 10688.3252038274f64, 10688.3252038274f64, 10675.4019041389f64);
        compare_to_excel(25, -0.05f64, 0, 16834.1121960282f64, -16834.1121960282f64, -16834.1121960282f64, -16834.1121960282f64);
    }

}
