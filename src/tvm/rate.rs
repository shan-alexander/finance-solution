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
/// let present_value = -10_000.00;
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
/// let present_value = -10_000.00;
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
/// assert_eq!("0.041380 = ((-15000.0000 / -10000.0000) ^ (1 / 10)) - 1", formula);
/// let symbolic_formula = solution.symbolic_formula();
/// dbg!(&symbolic_formula);
/// assert_eq!("r = ((-fv / pv) ^ (1 / n)) - 1", symbolic_formula);
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
    if is_approx_equal!(0.0, present_value + future_value) {
        // This is a special case where any rate will work.
        return 0.0;
    }
    if future_value == 0.0 {
        // This is a special case where the rate must be -100% because the present value is nonzero.
        return -1.0;
    }
    check_rate_parameters(periods, present_value, future_value);

    let rate = if continuous_compounding {
        // http://www.edmichaelreggie.com/TMVContent/APR.htm
        (-future_value / present_value).ln() / periods as f64
    } else {
        (-future_value / present_value).powf(1.0 / periods as f64) - 1.0
    };

    if !rate.is_finite() {
        dbg!(periods, present_value, future_value, continuous_compounding, rate);
    }

    assert!(rate.is_finite());
    //rintln!("\nrate_internal(): periods = {}, present_value = {}, future_value = {}, continuous_compounding = {},\n\trate = {}", periods, present_value, future_value, continuous_compounding, rate);
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
        let formula = format!("{:.6} = ln({:.4} / {:.4}) / {}", rate, -future_value, present_value, periods);
        let symbolic_formula = "r = ln(-fv / pv) / t";
        (formula, symbolic_formula)
    } else {
        let formula = format!("{:.6} = (({:.4} / {:.4}) ^ (1 / {})) - 1", rate, -future_value, present_value, periods);
        let symbolic_formula = "r = ((-fv / pv) ^ (1 / n)) - 1";
        (formula, symbolic_formula)
    };
    TvmSolution::new(TvmVariable::Rate, continuous_compounding, rate, periods, present_value, future_value, &formula, symbolic_formula)
}

fn check_rate_parameters(periods: u32, present_value: f64, future_value: f64) {
    //bg!(periods, present_value, future_value);
    assert!(present_value.is_finite(), "The present value must be finite (not NaN or infinity)");
    assert!(future_value.is_finite(), "The future value must be finite (not NaN or infinity)");
    assert!(!(present_value < 0.0 && future_value < 0.0), "The present value and future value are both negative. They must have opposite signs.");
    assert!(!(present_value > 0.0 && future_value > 0.0), "The present value and future value are both positive. They must have opposite signs.");
    assert!(!(is_approx_equal!(0.0, present_value) && !is_approx_equal!(0.0, future_value)), "The present value is zero and the future value is nonzero so there's no way to solve for rate.");
    assert!(!(periods == 0 && is_approx_equal!(0.0, present_value + future_value)), "The number of periods is zero and the present value plus the future value is nonzero so there's no way to solve for rate.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_edge() {
        // Zero periods, values add up to zero.
        assert_rounded_6(0.0, rate(0, 10_000.0, -10_000.0, false));

        // Nonzero periods, values add up to zero.
        assert_rounded_6(0.0, rate(12, -10_000.0, 10_000.0, false));
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

    /*
    macro_rules! compare_to_excel {
        ( $n:expr, $pv:expr, $fv:expr, $r_excel:expr, $r_manual_simple:expr, $r_manual_cont:expr ) => {
            println!("$n = {}, $pv = {}, $fv = {}, $r_excel: {}, $r_manual_simple = {}, $r_manual_cont = {}", $n, $pv, $fv, $r_excel, $r_manual_simple, $r_manual_cont);
            assert_approx_equal!($r_excel, $r_manual_simple);

            let r_calc_simple = rate($n, $pv, $fv, false);
            println!("r_calc_simple = {}", r_calc_simple);
            assert_approx_equal!($r_excel, r_calc_simple);

            let r_calc_cont = rate($n, $pv, $fv, true);
            println!("r_calc_cont = {}", r_calc_cont);
            assert_approx_equal!($r_manual_cont, r_calc_cont);

            if is_approx_equal!(0.0, r_calc_simple) {
                assert_approx_equal!(0.0, r_calc_cont);
            } else {
                let ratio = r_calc_cont / r_calc_simple;
                println!("ratio = {}", ratio);
                if $r_excel < 0.0 {
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

    fn compare_to_excel (test_case: usize, n: u32, pv: f64, fv: f64, r_excel: f64, r_manual_simple: f64, r_manual_cont: f64) {
        let display = false;

        if display { println!("test_case = {}, n = {}, pv = {}, fv = {}, r_excel: {}, r_manual_simple = {}, r_manual_cont = {}", test_case, n, pv, fv, r_excel, r_manual_simple, r_manual_cont) }
        assert_approx_equal!(r_excel, r_manual_simple);

        let r_calc_simple = rate(n, pv, fv,false);
        if display { println!("r_calc_simple = {}", r_calc_simple) }
        assert_approx_equal!(r_excel, r_calc_simple);

        let r_calc_cont = rate(n, pv, fv, true);
        if display { println!("r_calc_cont = {}", r_calc_cont); }
        assert_approx_equal!(r_manual_cont, r_calc_cont);

        if is_approx_equal!(0.0, r_calc_simple) {
            assert_approx_equal!(0.0, r_calc_cont);
        } else {
            let ratio = r_calc_cont / r_calc_simple;
            if display { println!("ratio = {}", ratio) };
            if r_excel < 0.0 {
                assert!(ratio >= 1.0);
                assert!(ratio <= 2.0);
            } else {
                assert!(ratio >= 0.0);
                assert!(ratio <= 1.0);
            }
        }

        // Solution with simple compounding.
        let solution = rate_solution(n, pv, fv, false);
        if display { dbg!(&solution); }
        solution.invariant();
        assert!(solution.calculated_field().is_rate());
        assert_eq!(false, solution.continuous_compounding());
        assert_approx_equal!(r_excel, solution.rate());
        assert_eq!(n, solution.periods());
        assert_approx_equal!(n as f64, solution.fractional_periods());
        assert_approx_equal!(pv, solution.present_value());
        assert_approx_equal!(fv, solution.future_value());

        // Solution with continuous compounding.
        let solution = rate_solution(n, pv, fv, true);
        if display { dbg!(&solution); }
        solution.invariant();
        assert!(solution.calculated_field().is_rate());
        assert!(solution.continuous_compounding());
        assert_approx_equal!(r_manual_cont, solution.rate());
        assert_eq!(n, solution.periods());
        assert_approx_equal!(n as f64, solution.fractional_periods());
        assert_approx_equal!(pv, solution.present_value());
        assert_approx_equal!(fv, solution.future_value());
    }

    #[test]
    fn test_rate_against_excel() {
        compare_to_excel(1, 90, -0.1f64, 1f64, 0.0259143654700119f64, 0.0259143654700098f64, 0.025584278811045f64);
        compare_to_excel(2, 85, 1.05f64, -1.5f64, 0.00420499208399443f64, 0.00420499208399305f64, 0.00419617581104391f64);
        compare_to_excel(3, 80, -2.25f64, 2.25f64, 8.10490132135311E-16f64, 0f64, 0f64);
        compare_to_excel(4, 75, 4.3875f64, -3.375f64, -0.00349207865283533f64, -0.00349207865410572f64, -0.00349819019289988f64);
        compare_to_excel(5, 70, -10.125f64, 5.0625f64, -0.00985323817917932f64, -0.00985323818144335f64, -0.00990210257942779f64);
        compare_to_excel(6, 65, 0.759375f64, -7.59375f64, 0.0360593046264088f64, 0.0360593046256343f64, 0.0354243860460622f64);
        compare_to_excel(7, 60, -7.9734375f64, 11.390625f64, 0.00596228650143506f64, 0.00596228649269048f64, 0.00594458239897887f64);
        compare_to_excel(8, 55, 17.0859375f64, -17.0859375f64, 4.31995919490311E-13f64, 0f64, 0f64);
        compare_to_excel(9, 50, -33.317578125f64, 25.62890625f64, -0.00523354233611077f64, -0.00523354233613538f64, -0.00524728528934982f64);
        compare_to_excel(10, 45, 76.88671875f64, -38.443359375f64, -0.0152852470655657f64, -0.0152852470655688f64, -0.0154032706791099f64);
        compare_to_excel(11, 40, -5.76650390625f64, 57.6650390625f64, 0.0592537251772898f64, 0.0592537251772889f64, 0.0575646273248511f64);
        compare_to_excel(12, 35, 60.548291015625f64, -86.49755859375f64, 0.010242814832087f64, 0.0102428148320715f64, 0.0101907126839638f64);
        compare_to_excel(13, 30, -129.746337890625f64, 129.746337890625f64, 2.53808542775445E-15f64, 0f64, 0f64);
        compare_to_excel(14, 25, 253.005358886719f64, -194.619506835937f64, -0.0104396946981842f64, -0.0104396947068867f64, -0.0104945705786996f64);
        compare_to_excel(15, 20, -583.858520507812f64, 291.929260253906f64, -0.0340636710696604f64, -0.0340636710751544f64, -0.0346573590279973f64);
        compare_to_excel(16, 15, 43.7893890380859f64, -437.893890380859f64, 0.165914401180033f64, 0.165914401179832f64, 0.15350567286627f64);
        compare_to_excel(17, 12, -459.788584899902f64, 656.840835571289f64, 0.0301690469250706f64, 0.0301690469166949f64, 0.0297229119948944f64);
        compare_to_excel(18, 10, 985.261253356933f64, -985.261253356933f64, 3.26110008113999E-16f64, 0f64, 0f64);
        compare_to_excel(19, 7, -1921.25944404602f64, 1477.8918800354f64, -0.0367869049970667f64, -0.0367869049970667f64, -0.0374806092096416f64);
        compare_to_excel(20, 5, 4433.6756401062f64, -2216.8378200531f64, -0.129449436703859f64, -0.129449436703876f64, -0.138629436111989f64);
        compare_to_excel(21, 4, -332.525673007965f64, 3325.25673007965f64, 0.778279410038923f64, 0.778279410038923f64, 0.575646273248511f64);
        compare_to_excel(22, 3, 3491.51956658363f64, -4987.88509511947f64, 0.126247880443697f64, 0.126247880443606f64, 0.118891647979577f64);
        compare_to_excel(23, 2, -7481.82764267921f64, 7481.82764267921f64, -9.19973496824152E-17f64, 0f64, 0f64);
        compare_to_excel(24, 1, 14589.5639032245f64, -11222.7414640188f64, -0.230769230769231f64, -0.230769230769231f64, -0.262364264467491f64);
    }
}