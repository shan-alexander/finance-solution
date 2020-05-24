//! **Number of periods calculations.** Given a periodic rate, present value, and future value, find the
//! number of periods needed to satisfy the equation.
//! 
//! For most common usages, we recommend the [periods_solution](fn.periods_solution.html) function.
//!
//! # Concepts
//!
//! Suppose we invest $100 at 10% annual interest. After one year the investment is worth $110.
//! After two years it's worth $110 plus 10% or $121, and so on:
//!
//! <img src="http://i.upmath.me/svg/%24%24%5Cbegin%7Btikzpicture%7D%5Bscale%3D1.0544%5D%5Csmall%0A%5Cbegin%7Baxis%7D%5Baxis%20line%20style%3Dgray%2C%0A%09samples%3D12%2C%0A%09width%3D9.0cm%2Cheight%3D6.4cm%2C%0A%09xmin%3D0%2C%20xmax%3D12%2C%0A%09ymin%3D70%2C%20ymax%3D370%2C%0A%09restrict%20y%20to%20domain%3D0%3A1000%2C%0A%09ytick%3D%7B100%2C%20150%2C%20200%2C%20250%2C%20300%2C%20350%7D%2C%0A%09xtick%3D%7B1%2C2%2C3%2C4%2C5%2C6%2C7%2C8%2C9%2C10%2C11%2C12%7D%2C%0A%09axis%20x%20line%3Dcenter%2C%0A%09axis%20y%20line%3Dcenter%2C%0A%09xlabel%3D%24n%24%2Cylabel%3D%24fv%24%5D%0A%5Caddplot%5Bblue%2Cdomain%3D1%3A12%2Csemithick%2Conly%20marks%5D%7B100*((1.1)%5Ex)%7D%3B%0A%5Caddplot%5Bblue%5D%20coordinates%20%7B(5.4%2C110)%7D%20node%7B%24fv%3D100(1.1%5En)%24%7D%3B%0A%5Cpath%20(axis%20cs%3A0%2C122)%20node%20%5Banchor%3Dnorth%20west%2Cyshift%3D-0.07cm%5D%3B%0A%5Cend%7Baxis%7D%0A%5Cend%7Btikzpicture%7D%24%24" />
//!
//! Here `n` is the number of periods, in this case years, and `fv` is the future value, or the
//! value of the investment after some number of years. After 12 years the investment would grow to
//! a little over $300.
//!
//! But suppose our goal is to reach $250 and we need to know exactly how many years that will take.
//! This is where the periods calculations come in. They find the point where an investment reaches
//! some fixed value:
//!
//! <img src="http://i.upmath.me/svg/%24%24%5Cbegin%7Btikzpicture%7D%5Bscale%3D1.0544%5D%5Csmall%0A%5Cbegin%7Baxis%7D%5Baxis%20line%20style%3Dgray%2C%0A%09samples%3D100%2C%0A%09width%3D9.0cm%2Cheight%3D6.4cm%2C%0A%09xmin%3D0%2C%20xmax%3D12%2C%0A%09ymin%3D70%2C%20ymax%3D370%2C%0A%09restrict%20y%20to%20domain%3D0%3A1000%2C%0A%09ytick%3D%7B100%2C%20150%2C%20200%2C%20250%2C%20300%2C%20350%7D%2C%0A%09xtick%3D%7B1%2C2%2C3%2C4%2C5%2C6%2C7%2C8%2C9%2C10%2C11%2C12%7D%2C%0A%09axis%20x%20line%3Dcenter%2C%0A%09axis%20y%20line%3Dcenter%2C%0A%09xlabel%3D%24n%24%2Cylabel%3D%24fv%24%5D%0A%5Caddplot%5Bblue%2Cdomain%3D1%3A9.614%2Cthick%5D%7B100*((1.1)%5Ex)%7D%3B%0A%5Caddplot%5Bblue%2Cdomain%3D9.614%3A12%2Cthick%2Cdashed%5D%7B100*((1.1)%5Ex)%7D%3B%0A%5Caddplot%5Bblack%2Cdomain%3D1%3A12%5D%7B250%7D%3B%0A%5Caddplot%5B%5D%20coordinates%20%7B(2.1%2C%20270)%7D%20node%7B%24fv%3D250%24%7D%3B%0A%5Caddplot%5Bblue%5D%20coordinates%20%7B(5.5%2C120.3)%7D%20node%7B%24fv%3D100(1.1%5En)%24%7D%3B%0A%5Caddplot%5Bred%5D%20coordinates%20%7B(10.6%2C235)%7D%20node%7B%24n%3D9.61%24%7D%3B%0A%5Cpath%20(axis%20cs%3A0%2C122)%20node%20%5Banchor%3Dnorth%20west%2Cyshift%3D-0.07cm%5D%3B%0A%5Cend%7Baxis%7D%0A%5Cend%7Btikzpicture%7D%24%24" />
//!
//! Here the investment reaches $250 after 9.61 years.
//!
//! The same ideas apply with a negative rate. Suppose we have a value that starts at $100 and
//! declines by 10% per year. At what point does the value fall to $70?
//!
//! <img src="http://i.upmath.me/svg/%24%24%5Cbegin%7Btikzpicture%7D%5Bscale%3D1.0544%5D%5Csmall%0A%5Cbegin%7Baxis%7D%5Baxis%20line%20style%3Dgray%2C%0A%09samples%3D100%2C%0A%09width%3D9.0cm%2Cheight%3D6.4cm%2C%0A%09xmin%3D0%2C%20xmax%3D12%2C%0A%09ymin%3D0%2C%20ymax%3D120%2C%0A%09restrict%20y%20to%20domain%3D0%3A1000%2C%0A%09ytick%3D%7B10%2C%2020%2C%2030%2C%2040%2C%2050%2C%2060%2C%2070%2C80%2C%2090%2C%20100%7D%2C%0A%09xtick%3D%7B1%2C2%2C3%2C4%2C5%2C6%2C7%2C8%2C9%2C10%2C11%2C12%7D%2C%0A%09axis%20x%20line%3Dcenter%2C%0A%09axis%20y%20line%3Dcenter%2C%0A%09xlabel%3D%24n%24%2Cylabel%3D%24fv%24%5D%0A%5Caddplot%5Bblue%2Cdomain%3D1%3A3.385%2Cthick%5D%7B100*((0.9)%5Ex)%7D%3B%0A%5Caddplot%5Bblue%2Cdomain%3D3.385%3A12%2Cthick%2Cdashed%5D%7B100*((0.9)%5Ex)%7D%3B%0A%5Caddplot%5Bblack%2Cdomain%3D1%3A12%5D%7B70%7D%3B%0A%5Caddplot%5B%5D%20coordinates%20%7B(11%2C%2064)%7D%20node%7B%24fv%3D70%24%7D%3B%0A%5Caddplot%5Bblue%5D%20coordinates%20%7B(6.8%2C30)%7D%20node%7B%24fv%3D100(0.9%5En)%24%7D%3B%0A%5Caddplot%5Bred%5D%20coordinates%20%7B(4.5%2C75.1)%7D%20node%7B%24n%3D3.39%24%7D%3B%0A%5Cpath%20(axis%20cs%3A0%2C122)%20node%20%5Banchor%3Dnorth%20west%2Cyshift%3D-0.07cm%5D%3B%0A%5Cend%7Baxis%7D%0A%5Cend%7Btikzpicture%7D%24%24" />
//!
//! After 3.39 periods the value is $70.
//!
//! # Formulas
//!
//! ## Simple Compounding
//!
//! With simple compound interest the number of periods is calculated with:
//!
//! > <img src="http://i.upmath.me/svg/periods%20%3D%20%5Cfrac%7B%5Clog_%7B1%2Brate%7D%5Cleft(%5Cfrac%7Bfuture%5C_value%7D%7Bpresent%5C_value%7D%5Cright)%7D%7Brate%7D" />
//!
//! Or using some more common variable names:
//!
//! > <img src="http://i.upmath.me/svg/n%20%3D%20%5Cfrac%7B%5Clog_%7B1%2Br%7D%5Cleft(%5Cfrac%7Bfv%7D%7Bpv%7D%5Cright)%7Dr" />
//!
//! `n` is often used for the number of periods, though it may be `t` for time if each period is
//! assumed to be one year as in continuous compounding. `r` is the periodic rate, though this may
//! appear as `i` for interest.
//!
//! Throughout this crate we use `pv` for present value and `fv` for future value. You may see these
//! values called `P` for principal in some references.
//!
//! Within the [TvmSolution](./struct.TvmSolution.html) struct we record the formula used for the particular calculation
//! using both concrete values and symbols. For the example above with $100 growing at 10%, where we
//! want to end up with $250 the struct contains:
//! ```text
//! formula: "9.61 = log(250.0000 / 100.0000, base 1.100000)",
//! symbolic_formula: "n = log(-fv / pv, base (1 + r))",
//! ```
//!
//! ## Continuous Compounding
//!
//! With continuous compounding it's:
//!
//! > <img src="http://i.upmath.me/svg/periods%20%3D%20%5Cfrac%7B%5Cln%5Cleft(%5Cfrac%7Bfuture%5C_value%7D%7Bpresent%5C_value%7D%5Cright)%7D%7Brate%7D" />
//!
//! or:
//!
//! > <img src="http://i.upmath.me/svg/n%20%3D%20%5Cfrac%7B%5Cln%5Cleft(%5Cfrac%7Bfv%7D%7Bpv%7D%5Cright)%7Dr" />
//!
//! With continuous compounding the period is assumed to be years and `t` (time) is often used as
//! the variable name. Within this crate we stick with `n` for the number of periods so that it's
//! easier to compare formulas when they're printed as simple text as part of the [TvmSolution](./struct.TvmSolution.html)
//! struct, as in:
//! ```text
//! formula: "9.16 = ln(250.0000 / 100.0000) / 0.100000",
//! symbolic_formula: "n = ln(fv / pv) / r",
//! ```

// use log::warn;

use super::tvm::*;

/// Returns the number of periods given a periodic rate along with the present and future values,
/// using simple compounding.
///
/// Note that the returned number of periods will be a floating point number representing fractional
/// periods.
///
/// See the [periods](./index.html) module page for the formulas.
///
/// Related functions:
/// * To calculate the periods using simple compounding and return a struct that shows the formula
/// and can be used to produce the the period-by-period values use [periods_solution](fn.periods_solution.html).
/// * To calculate the periods using continuous compounding use [periods_continuous](fn.periods_continuous.html)
/// or [periods_continuous_solution](fn.periods_continuous_solution.html).
///
/// # Arguments
/// * `rate` - The rate at which the investment grows or shrinks per period, expressed as a
/// floating point number. For instance 0.05 would mean 5% growth. Often appears as `r` or `i` in
/// formulas.
/// * `present_value` - The starting value of the investment. May appear as `pv` in formulas, or `C`
/// for cash flow or `P` for principal.
/// * `future_value` - The final value of the investment.
/// * `continuous_compounding` - True for continuous compounding, false for simple compounding.
///
/// # Panics
/// The call will fail if the rate, the present value, or the future value is infinite or not a
/// number (NaN).
///
/// The call will also fail in any of the follwing cases because there is no number of periods that
/// would make the calculation work:
/// * The periodic rate is less than -1.0.
/// * The present value is zero and the future value is nonzero.
/// * The present value is nonzero and the future value is zero, unless the rate is exactly -1.0%.
/// * The present value is negative and the future value is positive or vice versa.
/// * The present value and future value are both negative, the future value is less than the
/// present value, and the periodic rate is zero or negative.
/// * The present value and future value are both negative, the future value is greater than the
/// present value, and the periodic rate is zero or positive.
/// * The present value and future value are both positive, the future value is greater than the
/// present value, and the periodic rate is zero or negative.
/// * The present value and future value are both positive, the future value is less than the
/// present value, and the periodic rate is zero or positive.
///
/// # Examples
/// ```
/// use finance_solution::*;
///
/// // The interest rate is 8% per year.
/// let rate = 0.08;
///
/// // The starting value is $5,000.00.
/// let present_value = -5_000.00;
///
/// // The ending value is $7,000.00.
/// let future_value = 7_000.00;
///
/// let continuous_compounding = false;
///
/// // Calculate the number of years required.
/// let fractional_periods = periods(rate, present_value, future_value, false);
/// dbg!(&fractional_periods);
/// assert_rounded_2(4.37, fractional_periods);
///
/// // Round up to get a whole number of years.
/// let periods = fractional_periods.ceil() as u32;
/// dbg!(&periods);
/// assert_eq!(5, periods);
/// ```
pub fn periods<P, F>(rate: f64, present_value: P, future_value: F, continuous_compounding: bool) -> f64
    where
        P: Into<f64> + Copy,
        F: Into<f64> + Copy
{
    periods_internal(rate, present_value.into(), future_value.into(), continuous_compounding)
}

/// Calculates the number of periods given a periodic rate along with the present and future values
/// using simple compounding; and builds a struct with the input values, an explanation of the
/// formula, and the option to calculate the period-by-period values.
///
/// Note that the calculated number of periods from [PeriodsSolution::fractional_periods](./struct.PeriodsSolution.html#method.fractional_periods) field will
/// be a floating point number. To get the periods as a whole number (rounded up) use
/// [PeriodsSolution::periods](./struct.PeriodsSolution.html#method.periods).
///
/// See the [periods](./index.html) module page for the formulas.
///
/// Related functions:
/// * To calculate the periods as a single number with simple compounding use [periods](fn.periods.html).
/// * To calculate the periods using continuous compounding use [periods_continuous](fn.periods_continuous.html)
/// or [periods_continuous_solution](fn.periods_continuous_solution.html).
///
/// # Arguments
/// * `rate` - The rate at which the investment grows or shrinks per period, expressed as a
/// floating point number. For instance 0.05 would mean 5% growth. Often appears as `r` or `i` in
/// formulas.
/// * `present_value` - The starting value of the investment. May appear as `pv` in formulas, or `P`
/// for principal.
/// * `future_value` - The final value of the investment.
/// * `continuous_compounding` - True for continuous compounding, false for simple compounding.
///
/// # Panics
/// The call will fail if the rate, the present value, or the future value is infinite or not a
/// number (NaN).
///
/// The call will also fail in any of the follwing cases because there is no number of periods that
/// would make the calculation work:
/// * The periodic rate is less than -1.0.
/// * The present value is zero and the future value is nonzero.
/// * The present value is nonzero and the future value is zero, unless the rate is exactly -1.0%.
/// * The present value is negative and the future value is positive or vice versa.
/// * The present value and future value are both negative, the future value is less than the
/// present value, and the periodic rate is zero or negative.
/// * The present value and future value are both negative, the future value is greater than the
/// present value, and the periodic rate is zero or positive.
/// * The present value and future value are both positive, the future value is greater than the
/// present value, and the periodic rate is zero or negative.
/// * The present value and future value are both positive, the future value is less than the
/// present value, and the periodic rate is zero or positive.
///
/// # Examples
/// ```
/// use finance_solution::*;
///
/// // The interest rate is 3.5% per quarter.
/// let rate = 0.035;
///
/// // The starting value is $100,000.00.
/// let present_value = -100_000.00;
///
/// // The ending value is $200,000.00.
/// let future_value = 200_000.00;
///
/// // Use simple compounding.
/// let continuous_compounding = false;
///
/// // Calculate the number of quarters required and build a struct with the
/// // input values, an explanation of the formula, and an option to calculate
/// // the quarter-by-quarter values.
/// let solution = periods_solution(rate, present_value, future_value, continuous_compounding);
///
/// let fractional_quarters = solution.fractional_periods();
/// dbg!(&fractional_quarters);
/// assert_rounded_2(20.15, fractional_quarters);
///
/// // Get the whole number of quarters.
/// let quarters = solution.periods();
/// dbg!(&quarters);
/// assert_eq!(21, quarters);
///
/// // Examine the formulas.
/// let formula = solution.formula();
/// dbg!(&formula);
/// assert_eq!("20.15 = log(-200000.0000 / -100000.0000, base 1.035000)", formula);
/// let symbolic_formula = solution.symbolic_formula();
/// dbg!(&symbolic_formula);
/// assert_eq!("n = log(-fv / pv, base (1 + r))", symbolic_formula);
///
/// let series = solution.series();
/// dbg!(&series);
///
/// let last_entry = series.last().unwrap();
/// dbg!(&last_entry);
/// assert_rounded_4(200_000.0, last_entry.value());
///
/// // Create a reduced series with the value at the end of each year.
/// let filtered_series = series
///     .iter()
///     .filter(|x| x.period() % 4 == 0 && x.period() != 0)
///     .collect::<Vec<_>>();
/// dbg!(&filtered_series);
/// assert_eq!(5, filtered_series.len());
/// ```
/// Negative interest rate.
/// ```
/// // The interest rate is -6% per year and the value falls from $15,000.00 to
/// // $12,000.00.
/// # use finance_solution::*;
/// let solution = periods_solution(-0.06, -15_000.00, 12_000.00, false);
/// dbg!(&solution);
/// assert_rounded_2(3.61, solution.fractional_periods());
/// assert_eq!(4, solution.periods());
///
/// // Print the period-by-period values as a formatted table.
/// solution.print_series_table();
/// ```
pub fn periods_solution<P, F>(rate: f64, present_value: P, future_value: F, continuous_compounding: bool) -> TvmSolution
    where
        P: Into<f64> + Copy,
        F: Into<f64> + Copy
{
    periods_solution_internal(rate, present_value.into(), future_value.into(), continuous_compounding)
}

pub(crate) fn periods_internal(rate: f64, present_value: f64, future_value: f64, continuous_compounding: bool) -> f64 {
    if is_approx_equal!(0.0, present_value + future_value) {
        // This is a special case that doesn't require us to check the parameters and which covers
        // the case where both are zero.
        return 0.0;
    }
    if future_value == 0.0 && rate == -1.0 {
        // This is a special case that we can't run through the log function. Since the rate is
        // -100%, given any present value the future value will be zero and it will take only one
        // period to get there.
        // We already know that the present value is nonzero because that case would have been
        // caught above.
        assert!(present_value != 0.0);
        return 1.0;
    }

    check_periods_parameters(rate, present_value, future_value);

    let fractional_periods = if continuous_compounding {
        // http://www.edmichaelreggie.com/TMVContent/rate.htm
        (-future_value / present_value).ln() / rate
    } else {
        (-future_value / present_value).log(1.0 + rate)
    };
    assert!(fractional_periods >= 0.0);
    fractional_periods
}

pub(crate) fn periods_solution_internal(rate: f64, present_value: f64, future_value: f64, continuous_compounding: bool) -> TvmSolution {
    let fractional_periods = periods_internal(rate, present_value, future_value, continuous_compounding);
    assert!(fractional_periods >= 0.0);
    let (formula, symbolic_formula) = if continuous_compounding {
        let formula = format!("{:.2} = ln({:.4} / {:.4}) / {:.6}", fractional_periods, -future_value, present_value, rate);
        let symbolic_formula = "n = ln(-fv / pv) / r";
        (formula, symbolic_formula)
    } else {
        let rate_multiplier = 1.0 + rate;
        let formula = format!("{:.2} = log({:.4} / {:.4}, base {:.6})", fractional_periods, -future_value, present_value, rate_multiplier);
        let symbolic_formula = "n = log(-fv / pv, base (1 + r))";
        (formula, symbolic_formula)
    };
    TvmSolution::new_fractional_periods(TvmVariable::Periods,continuous_compounding, rate, fractional_periods, present_value, future_value, &formula, symbolic_formula)
}

fn check_periods_parameters(rate: f64, present_value: f64, future_value: f64) {
    assert!(rate.is_finite(), "The rate must be finite (not NaN or infinity)");
    assert!(present_value.is_finite(), "The present value must be finite (not NaN or infinity)");
    assert!(future_value.is_finite(), "The future value must be finite (not NaN or infinity)");
    assert!(rate >= -1.0, "The rate must be greater than or equal to -1.0 because a rate lower than -100% would mean the investment loses more than its full value in a period.");
    assert!(!(present_value == 0.0 && future_value != 0.0), "The present value is zero and the future value is nonzero so there's no way to solve for the number of periods.");
    assert!(!(present_value != 0.0 && future_value == 0.0 && rate != -1.0), "The present value is nonzero, the future value is zero, and the rate is not -100% so there's no way to solve for the number of periods.");
    assert!(!(present_value < 0.0 && future_value < 0.0), "The present value and future value are both negative. They must have opposite signs.");
    assert!(!(present_value > 0.0 && future_value > 0.0), "The present value and future value are both positive. They must have opposite signs.");
    assert!(!(present_value.abs() < future_value.abs() && rate <= 0.0), "The absolute value of the present value is less than the absolute value of the future value and the periodic rate is zero or negative. There's no way to solve for the number of periods because no amount of compounding will reach the future value.");
    assert!(!(present_value.abs() > future_value.abs() && rate >= 0.0), "The absolute value of the present value is greater than the absolute value of the future value and the periodic rate is zero or positive. There's no way to solve for the number of periods because no amount of compounding will reach the future value.");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_periods_edge() {
        // Present and future values add up to zero so no periods are needed.
        assert_rounded_2(0.0, periods(0.04, 10_000.0, -10_000.0, false));

        // The present value is negative and the future value is zero, which works only if the rate
        // is exactly -1.0%.
        assert_rounded_6(1.0, periods(-1.0, -10_000.0, 0.0, false));

        // The present value is positive and the future value is zero, which works only if the rate
        // is exactly -1.0%.
        assert_rounded_6(1.0, periods(-1.0, 10_000.0, 0.0, false));
    }

    #[should_panic]
    #[test]
    fn test_periods_err_rate_nan() {
        periods(std::f64::NAN, 1_000.0, 2_000.0, false);
    }

    #[should_panic]
    #[test]
    fn test_periods_err_rate_inf() {
        periods(std::f64::NEG_INFINITY, 1_000.0, 2_000.0, false);
    }

    #[should_panic]
    #[test]
    fn test_periods_err_present_value_nan() {
        periods(0.04, std::f64::NAN, 1_000.0, false);
    }

    #[should_panic]
    #[test]
    fn test_periods_err_present_value_inf() {
        periods(0.04, std::f64::INFINITY, 1_000.0, false);
    }

    #[should_panic]
    #[test]
    fn test_periods_err_future_value_nan() {
        periods(0.04, 1_000.0, std::f64::NAN, false);
    }

    #[should_panic]
    #[test]
    fn test_periods_err_future_value_inf() {
        periods(0.04, 1_000.0, std::f64::NEG_INFINITY, false);
    }

    #[should_panic]
    #[test]
    fn test_periods_err_future_greater_bad_rate_1() {
        // The future value is greater than the present value and the periodic rate is zero.
        periods(0.0, 1_000.0, 2_000.0, false);
    }

    #[should_panic]
    #[test]
    fn test_periods_err_future_greater_bad_rate_2() {
        // The future value is greater than the present value and the periodic rate is negative.
        periods(-0.04, 1_000.0, 2_000.0, false);
    }

    #[should_panic]
    #[test]
    fn test_periods_err_future_less_bad_rate_1() {
        // The future value is less than the present value and the periodic rate is zero.
        periods(0.0, 2_000.0, 1_000.0, false);
    }

    #[should_panic]
    #[test]
    fn test_periods_err_future_less_bad_rate_2() {
        // The future value is less than the present value and the periodic rate is positive.
        periods(0.04, 2_000.0, 1_000.0, false);
    }

    #[should_panic]
    #[test]
    fn test_periods_err_present_zero_future_negative() {
        // The present value is zero and the future value is negative.
        periods(0.04, 0.0, -1_000.0, false);
    }

    #[should_panic]
    #[test]
    fn test_periods_err_present_zero_future_positive() {
        // The present value is zero and the future value is positive.
        periods(0.04, 0.0, 1_000.0, false);
    }

    #[should_panic]
    #[test]
    fn test_periods_err_present_negative_future_zero() {
        // The present value is negative and the future value is zero.
        periods(0.04, -1_000.0, 0.0, false);
    }

    #[should_panic]
    #[test]
    fn test_periods_err_present_positive_future_zero() {
        // The present value is positive and the future value is zero. This will fail unless the
        // rate is exactly -1.0%.
        periods(-0.04, 1_000.0, 0.0, false);
    }

    #[should_panic]
    #[test]
    fn test_periods_err_present_negative_future_negative() {
        // The present value and future value are both negative.
        periods(0.04, -1_000.0, -1_000.0, false);
    }

    #[should_panic]
    #[test]
    fn test_periods_err_present_positive_future_positive() {
        // The present value and future value are buth positive.
        periods(0.04, 1_000.0, 1_000.0, false);
    }

    /*
    macro_rules! compare_to_excel {
        ( $r:expr, $pv:expr, $fv:expr, $n_excel:expr, $n_manual_simple:expr, $n_manual_cont:expr ) => {
            println!("$r = {}, $pv = {}, $fv = {}, $n_excel: {}, $n_manual_simple = {}, $n_manual_cont = {}", $r, $pv, $fv, $n_excel, $n_manual_simple, $n_manual_cont);
            assert_approx_equal!($n_excel, $n_manual_simple);

            let n_calc_simple = periods($r, $pv, $fv, false);
            println!("n_calc_simple = {}", n_calc_simple);
            assert_approx_equal!($n_excel, n_calc_simple);

            let n_calc_cont = periods($r, $pv, $fv, true);
            println!("n_calc_cont = {}", n_calc_cont);
            assert_approx_equal!($n_manual_cont, n_calc_cont);

            if is_approx_equal!(0.0, n_calc_simple) {
                assert_approx_equal!(0.0, n_calc_cont);
            } else {
                let ratio = n_calc_cont / n_calc_simple;
                println!("ratio = {}", ratio);
                if $r < 0.0 {
                    assert!(ratio >= 1.0);
                    assert!(ratio <= 2.0);
                } else {
                    assert!(ratio >= 0.0);
                    assert!(ratio <= 1.0);
                }
            }
        }
    }
    */

    fn compare_to_excel (test_case: usize, r: f64, pv: f64, fv: f64, n_excel: f64, n_manual_simple: f64, n_manual_cont: f64) {
        let display = false;

        if display { println!("test_case = {}, r = {}, pv = {}, fv = {}, n_excel: {}, n_manual_simple = {}, n_manual_cont = {}", test_case, r, pv, fv, n_excel, n_manual_simple, n_manual_cont) };
        assert_approx_equal!(n_excel, n_manual_simple);

        let n_calc_simple = periods(r, pv, fv,false);
        if display { println!("n_calc_simple = {}", n_calc_simple) };
        assert_approx_equal!(n_excel, n_calc_simple);

        let n_calc_cont = periods(r, pv, fv, true);
        if display { println!("n_calc_cont = {}", n_calc_cont) };
        assert_approx_equal!(n_manual_cont, n_calc_cont);

        if is_approx_equal!(0.0, n_calc_simple) {
            assert_approx_equal!(0.0, n_calc_cont);
        } else {
            let ratio = n_calc_cont / n_calc_simple;
            if display { println!("ratio = {}", ratio) };
            if r < 0.0 {
                assert!(ratio >= 1.0);
                assert!(ratio <= 2.0);
            } else {
                assert!(ratio >= 0.0);
                assert!(ratio <= 1.0);
            }
        }

        // Solution with simple compounding.
        let solution = periods_solution(r, pv, fv, false);
        if display { dbg!(&solution); }
        solution.invariant();
        assert!(solution.calculated_field().is_periods());
        assert_eq!(false, solution.continuous_compounding());
        assert_approx_equal!(r, solution.rate());
        assert_approx_equal!(n_excel, solution.fractional_periods());
        assert_approx_equal!(pv, solution.present_value());
        assert_approx_equal!(fv, solution.future_value());

        // Solution with continuous compounding.
        let solution = periods_solution(r, pv, fv, true);
        if display { dbg!(&solution); }
        solution.invariant();
        assert!(solution.calculated_field().is_periods());
        assert!(solution.continuous_compounding());
        assert_approx_equal!(r, solution.rate());
        assert_approx_equal!(n_manual_cont as f64, solution.fractional_periods());
        assert_approx_equal!(pv, solution.present_value());
        assert_approx_equal!(fv, solution.future_value());
    }
    
    #[test]
    fn test_periods_against_excel() {
        compare_to_excel(1, 0.01f64, -0.1f64, 1f64, 231.407892558761f64, 231.407892558761f64, 230.258509299405f64);
        compare_to_excel(2, 0.07f64, 1.05f64, -1.5f64, 5.27168295531017f64, 5.27168295531017f64, 5.09535634198189f64);
        compare_to_excel(3, 0.05f64, -2.25f64, 2.25f64, 0f64, 0f64, 0f64);
        compare_to_excel(4, -0.01f64, 4.3875f64, -3.375f64, 26.1050245774708f64, 26.1050245774708f64, 26.2364264467491f64);
        compare_to_excel(5, -0.07f64, -10.125f64, 5.0625f64, 9.55133750944734f64, 9.55133750944734f64, 9.90210257942779f64);
        compare_to_excel(6, 0.011f64, 0.759375f64, -7.59375f64, 210.475110917029f64, 210.475110917029f64, 209.325917544913f64);
        compare_to_excel(7, 0.077f64, -7.9734375f64, 11.390625f64, 4.80827497549769f64, 4.80827497549769f64, 4.63214212907445f64);
        compare_to_excel(8, 0.055f64, 17.0859375f64, -17.0859375f64, 0f64, 0f64, 0f64);
        compare_to_excel(9, -0.011f64, -33.317578125f64, 25.62890625f64, 23.7198728049214f64, 23.7198728049214f64, 23.8512967697719f64);
        compare_to_excel(10, -0.077f64, 76.88671875f64, -38.443359375f64, 8.65071007890757f64, 8.65071007890757f64, 9.00191143584344f64);
        compare_to_excel(11, 0.0121f64, -5.76650390625f64, 57.6650390625f64, 191.44527339039f64, 191.44527339039f64, 190.296288677194f64);
        compare_to_excel(12, 0.0847f64, 60.548291015625f64, -86.49755859375f64, 4.38695946200137f64, 4.38695946200137f64, 4.21103829915859f64);
        compare_to_excel(13, 0.0605f64, -129.746337890625f64, 129.746337890625f64, 0f64, 0f64, 0f64);
        compare_to_excel(14, -0.0121f64, 253.005358886719f64, -194.619506835937f64, 21.5515487676588f64, 21.5515487676588f64, 21.682997063429f64);
        compare_to_excel(15, -0.0847f64, -583.858520507812f64, 291.929260253906f64, 7.83187077097481f64, 7.83187077097481f64, 8.18355585076677f64);
        compare_to_excel(16, 0.01331f64, 43.7893890380859f64, -437.893890380859f64, 174.145381520649f64, 174.145381520649f64, 172.996626070176f64);
        compare_to_excel(17, 0.09317f64, -459.788584899902f64, 656.840835571289f64, 4.00390668920358f64, 4.00390668920358f64, 3.82821663559871f64);
        compare_to_excel(18, 0.06655f64, 985.261253356933f64, -985.261253356933f64, 0f64, 0f64, 0f64);
        compare_to_excel(19, -0.01331f64, -1921.25944404602f64, 1477.8918800354f64, 19.5803404211536f64, 19.5803404211536f64, 19.7118155122082f64);
        compare_to_excel(20, -0.09317f64, 4433.6756401062f64, -2216.8378200531f64, 7.08737437249388f64, 7.08737437249388f64, 7.43959622796979f64);
        compare_to_excel(21, 0.014641f64, -332.525673007965f64, 3325.25673007965f64, 158.418163641677f64, 158.418163641677f64, 157.269660063797f64);
        compare_to_excel(22, 0.102487f64, 3491.51956658363f64, -4987.88509511947f64, 3.65563485240142f64, 3.65563485240142f64, 3.48019694145338f64);
        compare_to_excel(23, 0.073205f64, -7481.82764267921f64, 7481.82764267921f64, 0f64, 0f64, 0f64);
        compare_to_excel(24, -0.014641f64, 14589.5639032245f64, -11222.7414640188f64, 17.7883276800301f64, 17.7883276800301f64, 17.9198322838256f64);
    }
}
