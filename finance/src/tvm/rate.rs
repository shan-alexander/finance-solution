//! **Periodic rate calculations.** Given an initial investment amount, a final amount, and a number
//! of periods what does the rate per period need to be?
//! 
//! For most common usages, we recommend the [`rate_solution`](./fn.rate_solution.html) function to provide the best experience with debugging and additional features.
//!
// ! If you need to calculate the future value given a starting value, a number of periods, and one
// ! or more rates use [`future_value`] or related functions.
// !
// ! If you need to calculate the present value given a future value, a number of periods, and one
// ! or more rates use [`present_value`] or related functions.
// !
// ! If you need to calculate the number of periods given a fixed rate and a present and future value
// ! use [`periods`] or related functions.
//! # Formulas
//!
//! ## Simple Compounding
//!
//! With simple compound interest the rate is calculated with:
//!
//! > <img src="http://i.upmath.me/svg/rate%20%3D%20%5Csqrt%5Bperiods%5D%7B%5Cfrac%7Bfuture%5C_value%7D%7Bpresent%5C_value%7D%7D%20-%201" />
//!
//! Or using a few more common variable names:
//!
//! > <img src="http:i.upmath.me/svg/r%20%3D%20%5Csqrt%5Bn%5D%7B%5Cfrac%7Bfv%7D%7Bpv%7D%7D%20-%201" />
//!
//! `r` is the periodic rate, though this may appear as `i` for interest. `n` is often used for the
//! number of periods, though it may be `t` for time if each period is assumed to be one year as in
//! continuous compounding.
//!
//! Throughout this crate we use `pv` for present value and `fv` for future value. You may see these
//! values called `P` for principal in some references.
//!
//! ## Continuous Compounding
//!
//! With continuous compounding the formula is:
//!
//! > <img src="http://i.upmath.me/svg/rate%20%3D%20%5Cfrac%7B%5Cln%5Cleft(%5Cfrac%7Bfuture%5C_value%7D%7Bpresent%5C_value%7D%5Cright)%7D%7Bperiods%7D" />
//!
//! or:
//!
//! > <img src="http://i.upmath.me/svg/r%20%3D%20%5Cfrac%7B%5Cln%5Cleft(%5Cfrac%7Bfv%7D%7Bpv%7D%5Cright)%7Dn" />
//!
//! With continuous compounding the period is assumed to be years and `t` (time) is often used as
//! the variable name. Within this crate we stick with `n` for the number of periods so that all of
//! the functions use the same variables.

// use log::warn;

// Needed for the Rustdoc comments.
#[allow(unused_imports)]
use crate::*;

/// Returns the periodic rate of an investment given the number of periods along with the present
/// and future values.
///
/// See the [rate](./index.html) module page for the formulas.
///
/// Related functions:
/// * To calculate a periodic rate and return a struct that shows the formula and optionally
/// produces the the period-by-period values use [`rate_solution`].
///
/// # Arguments
/// * `periods` - The number of periods such as quarters or periods. Often appears as `n` or `t`.
/// * `present_value` - The starting value of the investment. May appear as `pv` in formulas, or `C`
/// for cash flow or `P` for principal.
/// * `future_value` - The final value of the investment.
/// * `continuous_compounding` - True for continuous compounding, false for simple compounding.
///
/// If present_value and future_value are both zero then any rate will work so the function returns
/// zero.
///
/// # Panics
/// The call will fail if the present value is zero and the future value is nonzero or vice versa.
/// It will also fail if the number of periods is zero and the present value is not equal to the
/// future value. In both cases this is because there's no periodic rate that could make that work.
///
/// # Examples
/// ```
/// use finance_solution::*;
///
/// // The interest will compound for 365 days.
/// let periods = 365;
///
/// // The starting value is $10,000.
/// let present_value = 10_000.00;
///
/// // The ending value is $11,000.
/// let future_value = 11_000.00;
///
/// let continuous_compounding = false;
///
/// // Calculate the periodic rate needed.
/// let rate = rate(periods, present_value, future_value, continuous_compounding);
/// dbg!(&rate);
/// // The rate is 0.0261% per day.
/// assert_rounded_6(0.000261, rate);
/// ```
pub fn rate<P, F>(periods: u32, present_value: P, future_value: F, continuous_compounding: bool) -> f64
    where
        P: Into<f64> + Copy,
        F: Into<f64> + Copy
{
    rate_internal(periods, present_value.into(), future_value.into(), continuous_compounding)
}

/// Returns the periodic rate of an investment given the number of periods along with the present
/// and future values.
///
/// See the [rate](./index.html) module page for the formulas.
///
/// Related functions:
/// * To calculate a periodic rate returning an f64 value instead of solution object, use [`rate`](./fn.rate.html).
///
/// # Arguments
/// * `periods` - The number of periods such as quarters or periods. Often appears as `n` or `t`.
/// * `present_value` - The starting value of the investment. May appear as `pv` in formulas, or `C`
/// for cash flow or `P` for principal.
/// * `future_value` - The final value of the investment.
/// * `continuous_compounding` - True for continuous compounding, false for simple compounding.
///
/// If present_value and future_value are both zero then any rate will work so the function returns
/// zero.
///
/// # Panics
/// The call will fail if the present value is zero and the future value is nonzero or vice versa.
/// It will also fail if the number of periods is zero and the present value is not equal to the
/// future value. In both cases this is because there's no periodic rate that could make that work.
///
/// # Examples
/// Calculate a periodic rate and examine the period-by-period values.
/// ```
/// use finance_solution::*;
/// // The interest will compound for ten periods.
/// // The starting value is $10,000.
/// // The ending value is $15,000.
/// let periods = 10;
/// let present_value = 10_000.00;
/// let future_value = 15_000.00;
/// let continuous_compounding = false;
/// /// // Calculate the periodic rate and create a struct with a record of the
/// // inputs, a description of the formula, and an option to calculate the
/// // period-by-period values.
/// let solution = rate_solution(periods, present_value, future_value, continuous_compounding);
/// dbg!(&solution);
///
/// let rate = solution.rate();
/// dbg!(&rate);
/// // The rate is 4.138% per period.
/// assert_rounded_6(0.041380, rate);
///
/// // Examine the formulas.
/// let formula = solution.formula();
/// dbg!(&formula);
/// assert_eq!("0.041380 = ((15000.0000 / 10000.0000) ^ (1 / 10)) - 1", formula);
/// let symbolic_formula = solution.symbolic_formula();
/// dbg!(&symbolic_formula);
/// assert_eq!("r = ((fv / pv) ^ (1 / n)) - 1", symbolic_formula);
///
/// // Calculate the period-by-period values.
/// let series = solution.series();
/// dbg!(&series);
/// ```
pub fn rate_solution<P, F>(periods: u32, present_value: P, future_value: F, continuous_compounding: bool) -> TvmSolution
    where
        P: Into<f64> + Copy,
        F: Into<f64> + Copy
{
    rate_solution_internal(periods, present_value.into(), future_value.into(), continuous_compounding)
}

fn rate_internal(periods: u32, present_value: f64, future_value: f64, continuous_compounding: bool) -> f64 {
    if present_value + future_value == 0.0 {
        // This is a special case where any rate will work.
        return 0.0;
    }
    if future_value == 0.0 {
        // This is a special case where the rate must be -100% because present value is nonzero.
        return -1.0;
    }
    check_rate_parameters(periods, present_value, future_value);

    let rate = if continuous_compounding {
        // http://www.edmichaelreggie.com/TMVContent/APR.htm
        (future_value / present_value).ln() / periods as f64
    } else {
        (future_value / present_value).powf(1.0 / periods as f64) - 1.0
    };

    if !rate.is_finite() {
        dbg!(periods, present_value, future_value, continuous_compounding, rate);
    }

    assert!(rate.is_finite());
    rate
}

pub (crate) fn rate_solution_internal(periods: u32, present_value: f64, future_value: f64, continuous_compounding: bool) -> TvmSolution {
    if present_value == 0.0 && future_value == 0.0 {
        // This is a special case where any rate will work.
        let formula = "{special case}";
        let symbolic_formula = "***";
        let rate = 0.0;
        return TvmSolution::new(TvmVariable::Rate, continuous_compounding, rate, periods, present_value, future_value, formula, symbolic_formula);
    }

    let rate = rate_internal(periods, present_value, future_value, continuous_compounding);
    let (formula, symbolic_formula) = if continuous_compounding {
        let formula = format!("{:.6} = ln({:.4} / {:.4}) / {}", rate, future_value, present_value, periods);
        let symbolic_formula = "r = ln(fv / pv) / t";
        (formula, symbolic_formula)
    } else {
        let formula = format!("{:.6} = (({:.4} / {:.4}) ^ (1 / {})) - 1", rate, future_value, present_value, periods);
        let symbolic_formula = "r = ((fv / pv) ^ (1 / n)) - 1";
        (formula, symbolic_formula)
    };
    TvmSolution::new(TvmVariable::Rate, continuous_compounding, rate, periods, present_value.into(), future_value, &formula, symbolic_formula)
}

fn check_rate_parameters(periods: u32, present_value: f64, future_value: f64) {
    assert!(present_value.is_finite(), "The present value must be finite (not NaN or infinity)");
    assert!(future_value.is_finite(), "The future value must be finite (not NaN or infinity)");
    assert!(!(present_value == 0.0 && future_value != 0.0), "The present value is zero and the future value is nonzero so there's no way to solve for rate.");
    assert!(!(periods == 0 && present_value + future_value != 0.0), "The number of periods is zero and the present value plus the future value is nonzero so there's no way to solve for rate.");
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::*;

    #[test]
    fn test_rate_nominal() {
        // The test values come from the Excel rate function.
        assert_rounded_6(0.028436, rate(12, 5_000, 7_000, false));
        assert_rounded_6(-0.027650, rate(12, 7_000, 5_000, false));
        assert_rounded_6(0.100000, rate(1, 10_000, 11_000, false));
        assert_rounded_6(-0.090909, rate(1, 11_000, 10_000, false));
        assert_rounded_6(0.001127, rate(360, 8_000, 12_000, false));
        assert_rounded_6(-0.001126, rate(360, 12_000, 8_000, false));
    }

    #[test]
    fn test_rate_edge() {
        // Zero periods, values add up to zero.
        assert_rounded_6(0.0, rate(0, 10_000.0, -10_000.0, false));

        // Nonzero periods, values the same.
        assert_rounded_6(0.0, rate(12, 10_000.0, 10_000.0, false));
    }

    #[should_panic]
    #[test]
    fn test_rate_err_present_value_nan() {
        // The present value is not a number.
        rate(12, std::f64::NAN, 1_000.0, false);
    }

    #[should_panic]
    #[test]
    fn test_rate_err_present_value_inf() {
        // The present value is infinite.
        rate(12, std::f64::INFINITY, 1_000.0, false);
    }

    #[should_panic]
    #[test]
    fn test_rate_err_future_value_nan() {
        // The future value is not a number.
        rate(12, 1_000.0, std::f64::NAN, false);
    }

    #[should_panic]
    #[test]
    fn test_rate_err_future_value_inf() {
        // The future value is infinite.
        rate(12, 1_000.0, std::f64::NEG_INFINITY, false);
    }

    #[should_panic]
    #[test]
    fn test_rate_err_zero_periods() {
        // Zero periods, values don't add up to zero.
        rate(0, 10_000.0, 10_000.0, false);
    }

}

