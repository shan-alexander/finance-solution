#![allow(dead_code)]

//! **Payment calculations.** What is the periodic payment needed for an amortized loan and how much
//! of that is interest or principal?

use log::{warn};

// Import needed for the function references in the Rustdoc comments.
#[allow(unused_imports)]
use crate::*;

const RUN_PAYMENT_INVARIANTS: bool = false;

/// Returns the payment needed at the end of every period for an amortized loan.
///
/// Related functions:
/// * For the case where the payment is due at the beginning of each period use [`payment_due`].
/// * To calculate the payment needed at the end of each period and return a struct that shows the
/// interest, the formula, and optionally the period-by-period values use [`payment_solution`].
/// * To produce that struct when the payment is due at the beginning of the period use
/// [`payment_due_solution`].
///
/// In the typical case where there's a present value and the future value is zero, the formula is:
/// > payment = ((present_value * (1 + rate)<sup>periods</sup>) * -rate) / ((1 + rate)<sup>periods</sup> - 1)
///
/// or with the more commonly used variables:
/// > pmt = ((pv * (1 + r)<sup>n</sup>) * -r) / ((1 + r)<sup>n</sup> - 1)
///
/// Often the payment is shown as `A` and the present value is `P` for principal.
///
/// If there's a future value and the present value is zero, the formula is:
/// > payment = (future_value * -rate) / ((1 + rate)<sup>periods</sup> - 1)
///
/// or:
/// > pmt = (fv * -r) / ((1 + r)<sup>n</sup> - 1)
///
/// If both present value and future value are nonzero the formula is:
/// > payment = (((present_value * (1 + rate)<sup>periods</sup>) + future_value) * -rate) / ((1 + rate)<sup>periods</sup> - 1)
///
/// or:
/// > pmt = (((pv * (1 + r)<sup>n</sup>) + fv) * -r) / ((1 + r)<sup>n</sup> - 1)
///
/// # Arguments
/// * `rate` - The interest rate per period, expressed as a floating point number. For instance
/// 0.01 would mean 1% interest per period. The rate must match the period type. For instance if the
/// periods are months and we're starting with an annual interest rate, the rate must be divided by
/// 12 when calling the function as shown in the example below. The rate often appears as `r` or `i`
/// (interest) in formulas.
/// * `periods` - The number of periods such as quarters or periods. Often appears as `n` or `t`
/// where `t` typically implies years.
/// * `present_value` - In the case of an amortized loan this is the the principal. It may appear as
/// `pv` in formulas, or `C` for cash flow or `P` for principal. Assuming that the future value is
/// zero, the payment will be negative if the present value is positive and vice versa.
/// * `future_value` - The value at the end of the last period. For a typical amortized loan this
/// will be zero. It appears as `fv` in formulas.
///
/// # Panics
/// The call will fail if `rate` is less than -1.0.
///
/// # Examples
/// A simple amortized loan with the payment due at the end of the month.
/// ```
/// // The loan will be paid off in five years.
/// let years = 5;
///
/// // The interest rate is 10% per year. Each period is one month so we need to divide the rate
/// // by the number of months in a year.
/// let rate = 0.10 / 12.0;
///
/// // Each period is one month so we need to multiply the
/// // years by the number of months in a year.
/// let periods = years * 12;
///
/// // The principal is $10,000.
/// let present_value = 10_000;
///
/// // The loan will be fully paid off by the end of the last period.
/// let future_value = 0;
///
/// // Call payment() instead of payment_due() because the payment is due at the end of the month.
/// let payment = finance::payment(rate, periods, present_value, future_value);
/// dbg!(payment);
///
/// // The payment is $212.47/month. Since the principal/present value was positive the payment is
/// // negative.
/// finance::assert_rounded_4(payment, -212.4704);
/// ```
pub fn payment<P, F>(rate: f64, periods: u32, present_value: P, future_value: F) -> f64
    where
        P: Into<f64> + Copy,
        F: Into<f64> + Copy
{
    payment_internal(rate, periods, present_value.into(), future_value.into(), false)
}

/// Returns the payment needed at the start of every period for an amortized loan.
///
/// Related functions:
/// * For the more usual case where the payment is due at the end of each period use [`payment`].
/// * To calculate the payment needed at the beginning of each period and return a struct that shows
/// the, interest, the formula, and optionally the period-by-period values use
/// [`payment_due_solution`].
/// * To produce that struct when the payment is due at the end of the period use
/// [`payment_solution`].
///
/// In the typical case where there's a present value and the future value is zero, the formula is:
/// > payment = ((present_value * (1 + rate)<sup>periods</sup>) * -rate) / (((1 + rate)<sup>periods</sup> - 1) * (1 + rate))
///
/// or with the more commonly used variables:
/// > pmt = ((pv * (1 + r)<sup>n</sup>) * -r) / (((1 + r)<sup>n</sup> - 1) * (1 + r))",
///
/// Often the payment is shown as `A` and the present value is `P` for principal.
///
/// This is nearly the same formula as the one for payments due at the end of the month. The
/// relationship between the two formulas is that:
/// > payment_due(x) = payment(x) / (1 + rate)
///
/// Thus the payment is slightly smaller if it's due at the beginning of the month since the
/// principal is paid down a bit faster.
///
/// If there's a future value and the present value is zero, the formula is:
/// > payment = (future_value * -rate) / (((1 + rate)<sup>periods</sup> - 1) * (1 + rate))
///
/// or:
/// > pmt = (fv * -r) / (((1 + r)<sup>n</sup> - 1) * (1 + r))
///
/// If both present value and future value are nonzero the formula is:
/// > payment = (((present_value * (1 + rate)<sup>periods</sup>) + future_value) * -rate) / (((1 + rate)<sup>periods</sup> - 1) * (1 + rate))
///
/// or:
/// > pmt = (((pv * (1 + r)<sup>n</sup>) + fv) * -r) / (((1 + r)<sup>n</sup> - 1) * (1 + r))
///
/// # Arguments
/// * `rate` - The interest rate per period, expressed as a floating point number. For instance
/// 0.01 would mean 1% interest per period. The rate must match the period type. For instance if the
/// periods are months and we're starting with an annual interest rate, the rate must be divided by
/// 12 when calling the function as shown in the example below. The rate often appears as `r` or `i`
/// (interest) in formulas.
/// * `periods` - The number of periods such as quarters or periods. Often appears as `n` or `t`
/// where `t` typically implies years.
/// * `present_value` - In the case of an amortized loan this is the the principal. It may appear as
/// `pv` in formulas, or `C` for cash flow or `P` for principal. Assuming that the future value is
/// zero, the payment will be negative if the present value is positive and vice versa.
/// * `future_value` - The value at the end of the last period. For a typical amortized loan this
/// will be zero. It appears as `fv` in formulas.
///
/// # Panics
/// The call will fail if `rate` is less than -1.0.
///
/// # Examples
/// A simple amortized loan with the payment due at the beginning of the month.
/// ```
/// // The loan will be paid off in ten years.
/// let years = 10;
///
/// // The interest rate is 8% per year. Each period is one month so we need to divide the rate
/// // by the number of months in a year.
/// let rate = 0.08 / 12.0;
///
/// // Each period is one month so we need to multiply the
/// // years by the number of months in a year.
/// let periods = years * 12;
///
/// // The principal is $25,000.
/// let present_value = 25_000;
///
/// // The loan will be fully paid off by the end of the last period.
/// let future_value = 0;
///
/// // Call payment_due() instead of payment() because the payment is due at the beginning of the
/// // month.
/// let payment_due_at_beginning = finance::payment_due(rate, periods, present_value, future_value);
/// dbg!(payment_due_at_beginning);
///
/// // The payment is $-301.31/month. Since the principal/present value was positive the payment is
/// // negative.
/// finance::assert_rounded_4(payment_due_at_beginning, -301.3103);
///
/// // Contrast this amount with the payment if it were due at the end of the month, the more usual
/// // case.
/// let payment_due_at_end = finance::payment(rate, periods, present_value, future_value);
/// dbg!(payment_due_at_end);
///
/// // The payment is slightly different if it's due at the end of the month.
/// finance::assert_rounded_4(payment_due_at_end, -303.3190);
/// ```
pub fn payment_due<P, F>(rate: f64, periods: u32, present_value: P, future_value: F) -> f64
    where
        P: Into<f64> + Copy,
        F: Into<f64> + Copy
{
    payment_internal(rate, periods, present_value.into(), future_value.into(), true)
}

fn payment_internal(rate: f64, periods: u32, present_value: f64, future_value: f64, due_at_beginning: bool) -> f64 {
    //bg!(rate, periods, present_value, future_value, due_at_beginning);
    assert!(rate.is_finite(), "The rate must be finite (not NaN or infinity)");
    assert!(rate > -1.0, "The rate must be greater than -1.0 (-100%).");
    if rate > 1.0 {
        warn!("You provided a periodic rate ({}) greater than 1. Are you sure you expect a {}% return?", rate, rate * 100.0);
    }
    assert!(present_value.is_finite(), "The present value must be finite (not NaN or infinity)");
    assert!(future_value.is_finite(), "The present value must be finite (not NaN or infinity)");
    assert!(!(periods == 0 && present_value + future_value != 0.0), "There are no periods and the present value + future value is not zero so there is no way to calculate payments.");

    if periods == 0 {
        assert_approx_equal!(present_value, -future_value);
        return 0.0;
    }

    if rate == 0.0 {
        // There is no interest so the payment is simply the total amount to be paid divided by the
        // number of periods.
        return (-present_value - future_value) / periods as f64;
    }

    let rate_mult = 1.0 + rate;
    let num= ((present_value * rate_mult.powf(periods as f64)) + future_value) * -rate;
    //bg!(num);
    assert!(num.is_finite());
    let mut denom = (rate_mult).powf(periods as f64) - 1.0;
    assert!(denom.is_finite());
    assert!(denom.is_normal());
    if due_at_beginning {
        denom *= rate_mult;
    }
    //bg!(denom);
    assert!(denom.is_finite());
    assert!(denom.is_normal());
    let payment = num / denom;
    //bg!(payment);
    assert!(payment.is_finite());

    payment
}

/// Calculates the payment needed at the end of every period for an amortized loan and creates a
/// struct showing the interest, the formula, and optionally the period-by-period values.
///
/// Related functions:
/// * For the case where the payment is due at the beginning of each period use
/// [`payment_due_solution`].
/// * To calculate the payment as a simple number instead of a struct when the payment is due at the
/// end of each period use [`payment`].
/// * For a simple number when the payment is due at the beginning of each period use
/// [`payment_due`].
///
/// In the typical case where there's a present value and the future value is zero, the formula is:
/// > payment = ((present_value * (1 + rate)<sup>periods</sup>) * -rate) / ((1 + rate)<sup>periods</sup> - 1)
///
/// or with the more commonly used variables:
/// > pmt = ((pv * (1 + r)<sup>n</sup>) * -r) / ((1 + r)<sup>n</sup> - 1)
///
/// Often the payment is shown as `A` and the present value is `P` for principal.
///
/// If there's a future value and the present value is zero, the formula is:
/// > payment = (future_value * -rate) / ((1 + rate)<sup>periods</sup> - 1)
///
/// or:
/// > pmt = (fv * -r) / ((1 + r)<sup>n</sup> - 1)
///
/// If both present value and future value are nonzero the formula is:
/// > payment = (((present_value * (1 + rate)<sup>periods</sup>) + future_value) * -rate) / ((1 + rate)<sup>periods</sup> - 1)
///
/// or:
/// > pmt = (((pv * (1 + r)<sup>n</sup>) + fv) * -r) / ((1 + r)<sup>n</sup> - 1)
///
/// # Arguments
/// * `rate` - The interest rate per period, expressed as a floating point number. For instance
/// 0.01 would mean 1% interest per period. The rate must match the period type. For instance if the
/// periods are months and we're starting with an annual interest rate, the rate must be divided by
/// 12 when calling the function as shown in the example below. The rate often appears as `r` or `i`
/// (interest) in formulas.
/// * `periods` - The number of periods such as quarters or periods. Often appears as `n` or `t`
/// where `t` typically implies years.
/// * `present_value` - In the case of an amortized loan this is the the principal. It may appear as
/// `pv` in formulas, or `C` for cash flow or `P` for principal. Assuming that the future value is
/// zero, the payment will be negative if the present value is positive and vice versa.
/// * `future_value` - The value at the end of the last period. For a typical amortized loan this
/// will be zero. It appears as `fv` in formulas.
///
/// # Panics
/// The call will fail if `rate` is less than -1.0.
///
/// # Examples
/// Calculate the payment for a simple amortized loan with the payment due at the end of the month,
/// then examine the formulas and the period-by-period details such as the amount of the payment
/// that goes to principal and interest.
/// ```
/// // The interest rate is 11.75% per year. Each period is one month so we need to divide the rate
/// // by the number of months in a year.
/// let rate = 0.1175 / 12.0;
///
/// // The loan will be paid off in 48 months.
/// let periods = 48;
///
/// // The principal is $12,500.50. Here we'll express it as a negative number so that the payment,
/// // interest, and principal are all positive numbers for simplicity.
/// let present_value = -12_500.5;
///
/// // The loan will be fully paid off by the end of the last period.
/// let future_value = 0.0;
///
/// // Call payment_solution() instead of payment_due_solution() because the payment is due at the
/// // end of each month.
/// let solution = finance::payment_solution(rate, periods, present_value, future_value);
/// // Display the inputs, payment amount, formulas, sum of interest, etc.
/// dbg!(&solution);
///
/// // The payment is $327.65/month. Since the principal/present value was negative the payment is
/// // positive.
/// finance::assert_rounded_4(solution.payment(), 327.6538);
///
/// // The sum of payments is simply the monthly payment times the number of months.
/// finance::assert_rounded_4(solution.sum_of_payments(), 15_727.3820);
///
/// // The sum of interest is the portion of the sum of payments that is over and above the original
/// // loan amount. Here we add the present value since it has the opposite sign of the payments.
/// finance::assert_rounded_4(solution.sum_of_interest(), solution.sum_of_payments() + solution.present_value());
/// finance::assert_rounded_4(solution.sum_of_interest(), 3_226.8820);
///
/// // Examine the formulas. Since the future value is zero we expect to see a slightly simplified
/// // formula.
/// let formula = solution.formula();
/// println!();
/// dbg!(&formula);
/// assert_eq!(formula, "327.6538 = ((-12500.5000 * 1.009792^48) * -0.009792) / (1.009792^48 - 1)");
/// let symbolic_formula = solution.formula_symbolic();
/// println!();
/// dbg!(&symbolic_formula);
/// assert_eq!(symbolic_formula, "pmt = ((pv * (1 + r)^n) * -r) / ((1 + r)^n - 1)");
///
/// // Calculate the period-by-period values including the amount of the payment that goes toward
/// // interest and principle as well as the running tally of the remaining amounts.
/// let series = solution.series();
/// // Note that all of the period-by-period values are shown as of the end of the period after that
/// // period's payment has been made.
/// println!();
/// dbg!(&series);
///
/// // Print the period-by-period values in a table with two decimal places and the numbers aligned.
/// // Show all columns including running totals and remaining amounts.
/// let include_running_totals = true;
/// let include_remaining_amounts = true;
/// println!();
/// series.print_table(include_running_totals, include_remaining_amounts, &finance::num_format::Locale::en, 2);
///
/// // Print a table with only the last period of each year, that is all of the periods that can be
/// // divided by 12. Include the running totals columns but not remaining amounts.
/// let include_running_totals = true;
/// let include_remaining_amounts = false;
/// println!();
/// series
///     .filter(|entry| entry.period() % 12 == 0)
///     .print_table(include_running_totals, include_remaining_amounts, &finance::num_format::Locale::en, 2);
///
/// // Print a table starting at the first period where at least 95% of the interest has been paid
/// // off, and round all dollar amounts to whole numbers by passing zero as the second argument to
/// // print_table(). Include the remanining amounts columns but not the running totals.
/// let include_running_totals = false;
/// let include_remaining_amounts = true;
/// println!();
/// series
///     .filter(|entry| entry.interest_to_date() >= solution.sum_of_interest() * 0.95)
///     .print_table(include_running_totals, include_remaining_amounts, &finance::num_format::Locale::en, 0);
/// ```
pub fn payment_solution<P, F>(rate: f64, periods: u32, present_value: P, future_value: F) -> TvmCashflowSolution
    where
        P: Into<f64> + Copy,
        F: Into<f64> + Copy
{
    let due_at_beginning = false;
    payment_solution_internal(rate, periods, present_value.into(), future_value.into(), due_at_beginning)
}

/// Calculates the payment needed at the beginning of every period for an amortized loan and creates
/// a struct showing the interest, the formula, and optionally the period-by-period values.
///
/// Related functions:
/// * For the case where the payment is due at the end of each period use [`payment_solution`].
/// * To calculate the payment as a simple number instead of a struct when the payment is due at the
/// beginning of each period use [`payment_due`].
/// * For a simple number when the payment is due at the end of each period use [`payment`].
///
/// In the typical case where there's a present value and the future value is zero, the formula is:
/// > payment = ((present_value * (1 + rate)<sup>periods</sup>) * -rate) / (((1 + rate)<sup>periods</sup> - 1) * (1 + rate))
///
/// or with the more commonly used variables:
/// > pmt = ((pv * (1 + r)<sup>n</sup>) * -r) / (((1 + r)<sup>n</sup> - 1) * (1 + r))",
///
/// Often the payment is shown as `A` and the present value is `P` for principal.
///
/// This is nearly the same formula as the one for payments due at the end of the month. The
/// relationship between the two formulas is that:
/// > payment_due(x) = payment(x) / (1 + rate)
///
/// Thus the payment is slightly smaller if it's due at the beginning of the month since the
/// principal is paid down a bit faster.
///
/// If there's a future value and the present value is zero, the formula is:
/// > payment = (future_value * -rate) / (((1 + rate)<sup>periods</sup> - 1) * (1 + rate))
///
/// or:
/// > pmt = (fv * -r) / (((1 + r)<sup>n</sup> - 1) * (1 + r))
///
/// If both present value and future value are nonzero the formula is:
/// > payment = (((present_value * (1 + rate)<sup>periods</sup>) + future_value) * -rate) / (((1 + rate)<sup>periods</sup> - 1) * (1 + rate))
///
/// or:
/// > pmt = (((pv * (1 + r)<sup>n</sup>) + fv) * -r) / (((1 + r)<sup>n</sup> - 1) * (1 + r))
///
/// # Arguments
/// * `rate` - The interest rate per period, expressed as a floating point number. For instance
/// 0.01 would mean 1% interest per period. The rate must match the period type. For instance if the
/// periods are months and we're starting with an annual interest rate, the rate must be divided by
/// 12 when calling the function as shown in the example below. The rate often appears as `r` or `i`
/// (interest) in formulas.
/// * `periods` - The number of periods such as quarters or periods. Often appears as `n` or `t`
/// where `t` typically implies years.
/// * `present_value` - In the case of an amortized loan this is the the principal. It may appear as
/// `pv` in formulas, or `C` for cash flow or `P` for principal. Assuming that the future value is
/// zero, the payment will be negative if the present value is positive and vice versa.
/// * `future_value` - The value at the end of the last period. For a typical amortized loan this
/// will be zero. It appears as `fv` in formulas.
///
/// # Panics
/// The call will fail if `rate` is less than -1.0.
///
/// # Examples
/// A simple amortized loan with the payment due at the beginning of the month.
/// ```
/// ```
pub fn payment_due_solution<P, F>(rate: f64, periods: u32, present_value: P, future_value: F) -> TvmCashflowSolution
    where
        P: Into<f64> + Copy,
        F: Into<f64> + Copy
{
    let due_at_beginning = true;
    payment_solution_internal(rate, periods, present_value.into(), future_value.into(), due_at_beginning)
}

fn payment_solution_internal(rate: f64, periods: u32, present_value: f64, future_value: f64, due_at_beginning: bool) -> TvmCashflowSolution {
    let payment = payment_internal(rate, periods, present_value, future_value, due_at_beginning);
    let (formula, formula_symbolic) = payment_formula(rate, periods, present_value, future_value, due_at_beginning, payment);
    let calculated_field = if due_at_beginning { TvmCashflowVariable::PaymentDue } else { TvmCashflowVariable::Payment };
    let solution = TvmCashflowSolution::new(calculated_field, rate, periods, present_value.into(), future_value, due_at_beginning, payment, &formula, &formula_symbolic);
    if RUN_PAYMENT_INVARIANTS {
        payment_solution_invariant(&solution);
    }
    solution
}

fn payment_formula(rate: f64, periods: u32, present_value: f64, future_value: f64, due_at_beginning: bool, payment: f64) -> (String, String) {
    let rate_multiplier = 1.0 + rate;
    let (mut formula, mut formula_symbolic) = if periods == 0 {
        (format!("{:.4}", 0.0), "0".to_string())
    } else if rate == 0.0 {
        // There is no interest so the payment is simply the total amount to be paid (the difference
        // between the present and future value) divided by the number of periods. We subtract
        // present value from future value because if present_value is higher than future value the
        // payments should be negative.
        if future_value == 0.0 {
            (format!("{:.4} / {}", -present_value, periods), "-pv / n".to_string())
        } else if present_value == 0.0 {
            (format!("{:.4} / {}", -future_value, periods), "-fv / n".to_string())
        } else {
            // We have both a present and future value.
            let add_future_value = if future_value > 0.0 {
                format!(" - {:.4}", future_value)
            } else {
                format!(" + {:.4}", -future_value)
            };
            let formula = format!("({:.4}{}) / {}", -present_value, add_future_value, periods);
            let formula_symbolic = "(-pv - fv) / n".to_string();
            (formula, formula_symbolic)
        }
    } else {
        let (formula_num, formula_symbolic_num) = if future_value == 0.0 {
            // We can slightly simplify the formula by not including the future value term.
            (format!("(({:.4} * {:.6}^{}) * {:.6})", present_value, rate_multiplier, periods, -rate), "((pv * (1 + r)^n) * -r)".to_string())
        } else {
            if present_value == 0.0 {
                // We can simplify the formula by not including the present value term.
                (format!("({:.4} * {:.6})", future_value, -rate), "(fv * -r)".to_string())
            } else {
                // We have both a present and future value.
                let add_future_value = if future_value > 0.0 {
                    format!(" + {:.4}", future_value)
                } else {
                    format!(" - {:.4}", 0.0 - future_value)
                };
                (format!("((({:.4} * {:.6}^{}){}) * {:.6})", present_value, rate_multiplier, periods, add_future_value, -rate), "(((pv * (1 + r)^n) + fv) * -r)".to_string())
            }
        };
        let mut formula_denom = format!("({:.6}^{} - 1)", rate_multiplier, periods);
        let mut formula_symbolic_denom = "((1 + r)^n - 1)".to_string();
        if due_at_beginning {
            formula_denom = format!("({} * {:.6})", formula_denom, rate_multiplier);
            formula_symbolic_denom = format!("({} * (1 + r))", formula_symbolic_denom);
        }
        (format!("{} / {}", formula_num, formula_denom), format!("{} / {}", formula_symbolic_num, formula_symbolic_denom))
    };
    formula = format!("{:.4} = {}", payment, formula);
    formula_symbolic = format!("pmt = {}", formula_symbolic);
    (formula, formula_symbolic)
}

pub(crate) fn payment_series(solution: &TvmCashflowSolution) -> TvmCashflowSeries {
    assert!(solution.calculated_field().is_payment() || solution.calculated_field().is_payment_due());
    let mut series = vec![];
    if solution.future_value() != 0.0 {
        return TvmCashflowSeries::new(series);
    }
    let rate = solution.rate();
    let periods = solution.periods();
    let present_value = solution.present_value();
    let due_at_beginning = solution.due_at_beginning();
    let payment = solution.payment();
    let sum_of_payments = solution.sum_of_payments();
    let sum_of_interest = solution.sum_of_interest();
    let mut payments_to_date = 0.0;
    let mut principal_to_date = 0.0;
    let mut interest_to_date = 0.0;
    for period in 1..=periods {
        let principal_remaining_at_start_of_period = present_value + principal_to_date;
        let interest = if solution.due_at_beginning() && period == 1 {
            0.0
        } else {
            -principal_remaining_at_start_of_period * rate
        };
        let principal = payment - interest;
        payments_to_date += payment;
        principal_to_date += principal;
        interest_to_date += interest;
        let payments_remaining = sum_of_payments - payments_to_date;
        let principal_remaining = -(present_value + principal_to_date);
        let interest_remaining = sum_of_interest - interest_to_date;
        let (formula, formula_symbolic) = if due_at_beginning && period == 1 {
            ("0".to_string(), "interest = 0".to_string())
        } else {
            let formula = format!("{:.4} = -({:.4} * {:.6})", interest, principal_remaining_at_start_of_period, rate);
            let formula_symbolic = "interest = -(principal * rate)".to_string();
            (formula, formula_symbolic)
        };
        let entry = TvmCashflowPeriod::new(period, rate, due_at_beginning, payment, payments_to_date,
            payments_remaining, principal, principal_to_date, principal_remaining, interest,
            interest_to_date, interest_remaining, formula, formula_symbolic);
        series.push(entry);
    }
    if RUN_PAYMENT_INVARIANTS {
        payment_series_invariant(solution, &series);
    }
    TvmCashflowSeries::new(series)
}

fn payment_solution_invariant(solution: &TvmCashflowSolution) {
    let calculated_field = solution.calculated_field();
    let rate = solution.rate();
    let periods = solution.periods();
    let present_value = solution.present_value();
    let future_value = solution.future_value();
    let due_at_beginning = solution.due_at_beginning();
    let payment = solution.payment();
    let sum_of_payments = solution.sum_of_payments();
    let sum_of_interest = solution.sum_of_interest();
    let formula = solution.formula();
    let formula_symbolic = solution.formula_symbolic();
    let present_and_future_value= present_value + future_value;
    if due_at_beginning {
        assert!(calculated_field.is_payment_due());
    } else {
        assert!(calculated_field.is_payment());
    }
    assert!(rate.is_finite());
    assert!(present_value.is_finite());
    assert!(future_value.is_finite());
    assert!(payment.is_finite());
    if future_value == 0.0 {
        if present_value == 0.0 {
            assert_eq!(0.0, payment);
        } else if present_value.is_sign_positive() {
            assert!(payment.is_sign_negative());
        } else if present_value.is_sign_negative() {
            assert!(payment.is_sign_positive());
        }
    }
    assert!(sum_of_payments.is_finite());
    assert_approx_equal!(sum_of_payments, payment * periods as f64);
    if future_value == 0.0 {
        assert_same_sign_or_zero!(sum_of_interest, sum_of_payments);
    }
    if periods > 0 && rate != 0.0 && future_value == 0.0 && !is_approx_equal!(0.0, present_value) {
        assert!(sum_of_interest.abs() < sum_of_payments.abs());
    }
    assert_approx_equal!(sum_of_interest, sum_of_payments + present_and_future_value);
    assert!(formula.len() > 0);
    assert!(formula_symbolic.len() > 0);
}

fn payment_series_invariant(solution: &TvmCashflowSolution, series: &[TvmCashflowPeriod]) {
    let periods = solution.periods();
    if solution.future_value() != 0.0 {
        assert_eq!(0, series.len());
    } else {
        assert_eq!(periods as usize, series.len());
    }
    if series.len() == 0 {
        return;
    }
    let rate = solution.rate();
    let present_value = solution.present_value();
    let due_at_beginning = solution.due_at_beginning();
    if due_at_beginning {
        assert!(solution.calculated_field().is_payment_due());
    } else {
        assert!(solution.calculated_field().is_payment());
    }
    let payment = solution.payment();
    let sum_of_payments = solution.sum_of_payments();
    let sum_of_interest = solution.sum_of_interest();
    let mut running_sum_of_payments = 0.0;
    let mut running_sum_of_principal = 0.0;
    let mut running_sum_of_interest = 0.0;
    let mut previous_principal: Option<f64> = None;
    let mut previous_interest: Option<f64> = None;
    for (index, entry) in series.iter().enumerate() {
        running_sum_of_payments += entry.payment();
        running_sum_of_principal += entry.principal();
        running_sum_of_interest += entry.interest();
        assert_eq!(rate, entry.rate());
        assert_eq!(index + 1, entry.period() as usize);
        assert_eq!(payment, entry.payment());
        assert_approx_equal!(running_sum_of_payments, entry.payments_to_date());
        assert_approx_equal!(sum_of_payments - running_sum_of_payments, entry.payments_remaining());
        if present_value == 0.0 || rate == 0.0 || (due_at_beginning && index == 0) {
            assert_eq!(payment, entry.principal());
            assert_eq!(0.0, entry.interest());
        } else {
            if present_value > 0.0 {
                assert!(entry.principal() < 0.0);
                assert!(entry.interest() < 0.0);
            } else {
                if entry.principal() <= 0.0 {
                    dbg!(&solution, &series[..10]);
                }
                assert!(entry.principal() > 0.0);
                assert!(entry.interest() > 0.0);
            }
        }
        if index > 0 && previous_interest.unwrap() != 0.0 {
            // Compared to the previous period the principal should be further from zero and the
            // interest should be further toward zero. There's a special case where payments are due
            // at the beginning and we're currently on the second entry. In this case the previous
            // entry will have had zero interest and a principal matching the full payment amount,
            // so the two assertions below wouldn't make sense.
            assert!(entry.principal().abs() > previous_principal.unwrap().abs());
            assert!(entry.interest().abs() < previous_interest.unwrap().abs());
        }
        assert_approx_equal!(running_sum_of_principal, entry.principal_to_date());
        assert_approx_equal!(-present_value - running_sum_of_principal, entry.principal_remaining());
        assert_approx_equal!(running_sum_of_interest, entry.interest_to_date());
        assert_approx_equal!(sum_of_interest - running_sum_of_interest, entry.interest_remaining());
        assert_approx_equal!(payment, entry.principal() + entry.interest());
        if index == periods as usize - 1 {
            // This is the entry for the last period.
            assert_approx_equal!(0.0, entry.payments_remaining());
            if !is_approx_equal!(0.0, entry.principal_remaining()) {
                dbg!(&solution, &series[..10], &series[250], &series[490..500]);
            }
            assert_approx_equal!(0.0, entry.principal_remaining());
            assert_approx_equal!(0.0, entry.interest_remaining());
        }
        assert!(entry.formula().len() > 0);
        assert!(entry.formula_symbolic().len() > 0);

        previous_principal = Some(entry.principal());
        previous_interest = Some(entry.interest());
    }
    assert_approx_equal!(running_sum_of_payments, sum_of_payments);
    assert_approx_equal!(running_sum_of_principal, -present_value);
    assert_approx_equal!(running_sum_of_interest, sum_of_interest);
}

/*
fn check_payment_parameters(rate: f64, periods: u32, present_value: f64, future_value: f64) {
    assert!(rate.is_finite(), "The rate must be finite (not NaN or infinity)");
    assert!(rate >= -1.0, "The rate must be greater than or equal to -1.0 because a rate lower than -100% would mean the investment loses more than its full value in a period.");
    if rate.abs() > 1. {
        warn!("You provided a periodic rate ({}) greater than 1. Are you sure you expect a {}% return?", rate, rate * 100.0);
    }
    assert!(present_value.is_finite(), "The present value must be finite (not NaN or infinity)");
    assert!(future_value.is_finite(), "The future value must be finite (not NaN or infinity)");
    assert!(!(present_value == 0.0 && future_value != 0.0), "The present value is zero and the future value is nonzero so there's no way to solve for the payments.");
    assert!(!(present_value != 0.0 && future_value == 0.0 && rate != -1.0), "The present value is nonzero, the future value is zero, and the rate is not -100% so there's no way to solve for the payments.");
    assert!(!(present_value < 0.0 && future_value > 0.0), "The present value is negative and the future value is positive so there's no way to solve for the payments.");
    assert!(!(present_value > 0.0 && future_value < 0.0), "The present value is positive and the future value is negative so there's no way to solve for the payments.");
    assert!(!(present_value < 0.0 && future_value < present_value && rate <= 0.0), "The present value and future value are both negative, the future value is less than the present value, and the periodic rate is zero or negative. There's no way to solve for the payments.");
    assert!(!(present_value < 0.0 && future_value > present_value && rate >= 0.0), "The present value and future value are both negative, the future value is greater than the present value, and the periodic rate is zero or positive. There's no way to solve for the payments.");
    assert!(!(present_value > 0.0 && future_value > present_value && rate <= 0.0), "The present value and future value are both positive, the future value is greater than the present value, and the periodic rate is zero or negative. There's no way to solve for the payments.");
    assert!(!(present_value > 0.0 && future_value < present_value && rate >= 0.0), "The present value and future value are both positive, the future value is less than the present value, and the periodic rate is zero or positive. There's no way to solve for the payments.");
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::*;

    #[test]
    fn test_payment_nominal() {
        assert_approx_equal!(-11.9636085342686f64, payment(0.034, 10, 100.0, 0.0));
        assert_approx_equal!(-8.22683411973293f64, payment(-0.034, 10, 100.0, 0.0));
        assert_approx_equal!(11.9636085342686f64, payment(0.034, 10, -100.0, 0.0));
        assert_approx_equal!(-100.097751710655f64, payment(1.0, 10, 100.0, 0.0));
        assert_approx_equal!(-150.01573028944f64, payment(1.5, 10, 100.0, 0.0));
        assert_approx_equal!(-10f64, payment(0.0, 10, 100.0, 0.0));
        assert_approx_equal!(-10.00055000825f64, payment(0.00001, 10, 100.0, 0.0));
        assert_approx_equal!(8.22683411973293f64, payment(-0.034, 10, -100.0, 0.0));
        assert_approx_equal!(9.82270640070143f64, payment(0.034, 10, -100.0, 25.0));
        assert_approx_equal!(14.1045106678357f64, payment(0.034, 10, -100.0, -25.0));
        assert_approx_equal!(-10f64, payment(0.0, 10, 100.0, 0.0));
        assert_approx_equal!(-11f64, payment(0.0, 10, 100.0, 10.0));
        assert_approx_equal!(-9f64, payment(0.0, 10, 100.0, -10.0));
        assert_approx_equal!(-1f64, payment(0.0, 10, 10.0, 0.0));
        assert_approx_equal!(-11f64, payment(0.0, 10, 10.0, 100.0));
        assert_approx_equal!(9f64, payment(0.0, 10, 10.0, -100.0));
        assert_approx_equal!(10f64, payment(0.0, 10, -100.0, 0.0));
        assert_approx_equal!(9f64, payment(0.0, 10, -100.0, 10.0));
        assert_approx_equal!(11f64, payment(0.0, 10, -100.0, -10.0));
        assert_approx_equal!(1f64, payment(0.0, 10, -10.0, 0.0));
        assert_approx_equal!(-9f64, payment(0.0, 10, -10.0, 100.0));
        assert_approx_equal!(11f64, payment(0.0, 10, -10.0, -100.0));
    }

    /*
    #[test]
    fn test_payment_edge() {
        // Zero interest.
        assert_rounded_6!(-10.0, payment(0.0, 10, 100.0, 0.0));
        // Zero periods but it's OK because the present and future value are equal.
        assert_rounded_6!(0.0, payment(0.05, 0, 100.0, 100.0));
    }
    */

    #[test]
    fn test_payment_due_nominal() {
        assert_approx_equal!(-11.5702210196021f64, payment_due(0.034, 10, 100.0, 0.0));
        assert_approx_equal!(-8.51639142829496f64, payment_due(-0.034, 10, 100.0, 0.0));
        assert_approx_equal!(-8.51639142829496f64, payment_due(-0.034, 10, 100.0, 0.0));
        assert_approx_equal!(-50.0488758553275f64, payment_due(1.0, 10, 100.0, 0.0));
        assert_approx_equal!(-60.0062921157762f64, payment_due(1.5, 10, 100.0, 0.0));
        assert_approx_equal!(-10f64, payment_due(0.0, 10, 100.0, 0.0));
        assert_approx_equal!(-10.0004500037499f64, payment_due(0.00001, 10, 100.0, 0.0));
        assert_approx_equal!(8.51639142829496f64, payment_due(-0.034, 10, -100.0, 0.0));
        assert_approx_equal!(9.49971605483697f64, payment_due(0.034, 10, -100.0, 25.0));
        assert_approx_equal!(13.6407259843672f64, payment_due(0.034, 10, -100.0, -25.0));
        assert_approx_equal!(-10f64, payment_due(0.0, 10, 100.0, 0.0));
        assert_approx_equal!(-11f64, payment_due(0.0, 10, 100.0, 10.0));
        assert_approx_equal!(-9f64, payment_due(0.0, 10, 100.0, -10.0));
        assert_approx_equal!(-1f64, payment_due(0.0, 10, 10.0, 0.0));
        assert_approx_equal!(-11f64, payment_due(0.0, 10, 10.0, 100.0));
        assert_approx_equal!(9f64, payment_due(0.0, 10, 10.0, -100.0));
        assert_approx_equal!(10f64, payment_due(0.0, 10, -100.0, 0.0));
        assert_approx_equal!(9f64, payment_due(0.0, 10, -100.0, 10.0));
        assert_approx_equal!(11f64, payment_due(0.0, 10, -100.0, -10.0));
        assert_approx_equal!(1f64, payment_due(0.0, 10, -10.0, 0.0));
        assert_approx_equal!(-9f64, payment_due(0.0, 10, -10.0, 100.0));
        assert_approx_equal!(11f64, payment_due(0.0, 10, -10.0, -100.0));
    }

    /*
    #[should_panic]
    #[test]
    fn test_future_value_error_rate_low() {
        future_value(-101.0, 5, 250_000.00);
    }
    */

    /*
    #[test]
    fn test_combinations() {
        // let rates = vec![-0.99, -0.5, -0.05, -0.005, 0.0, 0.005, 0.05, 0.5, 1.0, 10.0, 100.0];
        let rates = vec![0.0, 0.005, 0.05, 0.5, 1.0, 10.0, 100.0];
        let periods: Vec<u32> = vec![0, 1, 2, 5, 10, 36, 100, 500];
        let values: Vec<f64> = vec![-1_000_000.0, -1_234.98, -1.0, 0.0, 5.55555, 99_999.99];
        for rate_one in rates.iter() {
            for periods_one in periods.iter() {
                for present_value_one in values.iter() {
                    for future_value_one in values.iter() {
                        for due_at_beginning_one in [false, true].iter() {
                            println!();
                            dbg!(rate_one, periods_one, present_value_one, future_value_one, due_at_beginning_one);
                            if !(*periods_one == 0 && *present_value_one + *future_value_one != 0.0) {
                                let solution = if *due_at_beginning_one {
                                    payment_solution(*rate_one, *periods_one, *present_value_one, *future_value_one)
                                } else {
                                    payment_due_solution(*rate_one, *periods_one, *present_value_one, *future_value_one)
                                };
                                let series = solution.series();
                                //bg!(&solution, &series);
                                // If we're already calling the invariant functions at the end of
                                // payment_solution_internal() and payment_series() there's no point in
                                // running them again.
                                if !RUN_PAYMENT_INVARIANTS {
                                    run_payment_invariants(&solution, &series);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    */

    fn run_payment_invariants(solution: &TvmCashflowSolution, series: &[TvmCashflowPeriod]) {
        // Display the solution and series only if either one fails its invariant.
        let result = std::panic::catch_unwind(|| {
            payment_solution_invariant(&solution);
            payment_series_invariant(&solution, &series);
        });
        //bg!(&result);
        if result.is_err() {
            dbg!(&solution, &series);
            payment_solution_invariant(&solution);
            payment_series_invariant(&solution, &series);
        }
    }

}
