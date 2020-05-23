#![allow(dead_code)]

/*
fv = pv(1+r)^n\\ \\
fv = pv(e^{rt})\\ \\
pmt = {(((pv(1+r)^n)+fv)-r \over (1+r)^n - 1}

//i.upmath.me/svg/fv%20%3D%20pv(1%2Br)%5En%5C%5C%20%5C%5C%0Afv%20%3D%20pv(e%5E%7Brt%7D)%5C%5C%20%5C%5C%20%0Apmt%20%3D%20%7B(((pv(1%2Br)%5En)%2Bfv)-r%20%5Cover%20(1%2Br)%5En%20-%201%7D
http://i.upmath.me/svg/fv%20%3D%20pv(1%2Br)%5En%5C%5C%20%5C%5C%0Afv%20%3D%20pv(e%5E%7Brt%7D)%5C%5C%20%5C%5C%20%0Apmt%20%3D%20%7B(((pv(1%2Br)%5En)%2Bfv)-r%20%5Cover%20(1%2Br)%5En%20-%201%7D
*/

//! **Payment calculations.** What is the periodic payment needed for an amortized loan and how much
//! of that is interest or principal?
//! 
//! For most common usages, we recommend the [`payment_solution`](./fn.payment_solution.html) function, which provides a better debugging experience and additional functionality.
//!
//! ## Example
//! ```
//! let (rate, periods, present_value, future_value, due_at_beginning) = (0.034, 10, 1000, 0, false);
//! let solution = finance::payment_solution(rate, periods, present_value, future_value, due_at_beginning);
//! dbg!(&solution);
//! ```
//! Outputs to terminal:
//! ```text
//! {
//!     calculated_field: Payment,
//!     rate: 0.034,
//!     periods: 10,
//!     present_value: 1000.0,
//!     future_value: 0.0,
//!     due_at_beginning: false,
//!     payment: -119.63608534268569,
//!     sum_of_payments: -1196.360853426857,
//!     sum_of_interest: -196.36085342685692,
//!     formula: "-119.6361 = ((1000.0000 * 1.034000^10) * -0.034000) / (1.034000^10 - 1)",
//!     symbolic_formula: "pmt = ((pv * (1 + r)^n) * -r) / ((1 + r)^n - 1)",
//! }
//! ```
//! ```
//! # use finance::*;
//! # let (rate, periods, present_value, future_value, due_at_beginning) = (0.034, 10, 1000, 0, false);
//! # let solution = payment_solution(rate, periods, present_value, future_value, due_at_beginning);
//! dbg!(solution.print_table());
//! ```
//! Outputs to terminal:
//! ```text
//! period  payments_to_date  payments_remaining  principal  principal_to_date  principal_remaining  interest  interest_to_date  interest_remaining
//! ------  ----------------  ------------------  ---------  -----------------  -------------------  --------  ----------------  ------------------
//!     1         -119.6361         -1_076.7248   -85.6361           -85.6361            -914.3639  -34.0000          -34.0000           -162.3609
//!     2         -239.2722           -957.0887   -88.5477          -174.1838            -825.8162  -31.0884          -65.0884           -131.2725
//!     3         -358.9083           -837.4526   -91.5583          -265.7421            -734.2579  -28.0778          -93.1661           -103.1947
//!     4         -478.5443           -717.8165   -94.6713          -360.4134            -639.5866  -24.9648         -118.1309            -78.2300
//!     5         -598.1804           -598.1804   -97.8901          -458.3036            -541.6964  -21.7459         -139.8768            -56.4840
//!     6         -717.8165           -478.5443  -101.2184          -559.5220            -440.4780  -18.4177         -158.2945            -38.0663
//!     7         -837.4526           -358.9083  -104.6598          -664.1818            -335.8182  -14.9763         -173.2708            -23.0901
//!     8         -957.0887           -239.2722  -108.2183          -772.4001            -227.5999  -11.4178         -184.6886            -11.6723
//!     9       -1_076.7248           -119.6361  -111.8977          -884.2978            -115.7022   -7.7384         -192.4270             -3.9339
//!     10      -1_196.3609             -0.0000  -115.7022          -999.0000              -0.0000   -3.9339         -196.3609              0.0000
//! ```
//! # Payment Due at the Beginning vs. the End of the Period
//!
//! In a typical loan the payment is due at the end of each period. All other factors being equal,
//! if the payment is instead due at the beginning of the period the payment amount will be slightly
//! smaller. This is justified because it means the outstanding principal at any point during the
//! loan will be slightly smaller since it was paid down one period earlier.
//!
//! To make this concrete, suppose we have an amortized loan for $100,000 at 12% annual interest for
//! ten years. We can run an A/B test where scenario A has the payments due at the end of the period
//! and scenario B has them due at the beginning. We'll look at the details of the first five
//! periods:
//! ```text
//! period  payment_a  payment_b  principal_a  principal_b  princ_remaining_a  princ_remaining_b
//! ------  ---------  ---------  -----------  -----------  -----------------  -----------------
//!      1   1,434.71   1,420.50       434.71     1,420.50          99,565.29          98,579.50
//!      2   1,434.71   1,420.50       439.06       434.71          99,126.23          98,144.79
//!      3   1,434.71   1,420.50       443.45       439.06          98,682.79          97,705.73
//!      4   1,434.71   1,420.50       447.88       443.45          98,234.91          97,262.28
//!      5   1,434.71   1,420.50       452.36       447.88          97,782.54          96,814.40
//! ```
//!
//! For scenario A a payment of $1,434.71 is due at the end of the month and for scenario B a
//! lower payment of $1,420.50 is due at the beginning of the month. Yet in both scenarios the loan
//! is fully paid off by the end of the 120<sup>th</sup> period.
//!
//! The reason is that the first monthly payment in the second case went entirely to paying down the
//! principal, and from that point on the principal (the last two columns in this table) was
//! slightly smaller at every point in time. Thus the amount of each payment that went toward the
//! interest was smaller and the amount left to go to principal was larger.
//!
//! The sum of the payments in the first case (due at the end) is $172,165.14 and the sum of the
//! payments in the second case is $170,460.53, a difference of $1,704.61. The relationship between
//! these sums (and the monthly payment amounts) is based on the interest rate. Specifically:
//!
//! > <img src="http://i.upmath.me/svg/payment%5C_due%5C_at%5C_beginning(x)%20%3D%20%7Bpayment%5C_due%5C_at%5C_end(x)%20%5Cover%201%20%2B%20rate%7D" />
//!
//! and indeed it turns out that:
//!
//! > <img src="http://i.upmath.me/svg/170%2C460.53%20%3D%20%7B172%2C165.14%20%5Cover%201.01%7D" />
//!
//! which is a relief. By the way, A/B comparisons like the table above are built into the crate.
//! See [CashflowSolution::print_ab_comparison](././cashflow/struct.CashflowSolution.html#method.print_ab_comparison).
//!
//! # Positive and Negative Amounts
//!
//! The example above was an amortized loan for $100,000 at 12% annual interest for ten years. Thus
//! the present value (the original principal of the loan) was $10,000 and the future value (the
//! amount still owed at the end) was zero. Also the rate was positive. In cases like this if the
//! present value is entered as a positive number the payment will be negative, and vice versa:
//! ```
//! let (rate, periods, present_value, due_at_beginning) = (0.01, 120, 100_000, false);
//!
//! let pmt_positive_present_value = finance::payment(rate, periods, present_value, 0.0, due_at_beginning);
//! finance::assert_rounded_2!(-1_434.71, pmt_positive_present_value);
//!
//! let pmt_negative_present_value = finance::payment(rate, periods, -present_value, 0.0, due_at_beginning);
//! finance::assert_rounded_2!(1_434.71, pmt_negative_present_value);
//! ```
//! Either way is fine. It depends on whether we're thinking of the payment as a stream of money
//! being paid out or coming in. In the period-by-period detail the principal and interest (and
//! remaining principal and interest and so on) for each period will have the same sign as the
//! payment.
//!
//! # Formulas
//!
//! ## Present Value but no Future Value
//!
//! In the typical case of an amortized loan the present value will be nonzero and the future value
//! will be zero. This represents the loan being paid off by the end of the last period.
//!
//! If the payment is due at the end of the period the calculation is:
//!
//! > <img src="http://i.upmath.me/svg/payment%20%3D%20%7B%7Bpresent%5C_value%20%5Ctimes%20%5Cleft(1%2Brate%5Cright)%5E%7Bperiods%7D%20%5Ctimes%20-rate%7D%20%5Cover%20%5Cleft(1%2Brate%5Cright)%5E%7Bperiods%7D%20-%201%7D" />
//!
//! or with some of the more commonly used variables:
//!
//! > <img src="http://i.upmath.me/svg/pmt%20%3D%20%7B%7Bpv%20%5Ctimes%20%5Cleft(1%2Br%5Cright)%5En%20%5Ctimes%20-r%7D%20%5Cover%20%5Cleft(1%2Br%5Cright)%5En%20-%201%7D" />
//!
//! Often the payment is shown as `A` and the present value is `P` for principal. In this crate we
//! use the same symbols for present value, rate, etc. across all of the functions.
//!
//! If the payment is due at the beginning of the period the only difference is that we have to
//! divide the above payment by `(1 + rate)`. So the formula is:
//!
//! > <img src="http://i.upmath.me/svg/pmt%20%3D%20%7B%7Bpv%20%5Ctimes%20%5Cleft(1%2Br%5Cright)%5En%20%5Ctimes%20-r%7D%20%5Cover%20%5Cleft%5B%5Cleft(1%2Br%5Cright)%5En%20-%201%5Cright%5D%20%5Ctextcolor%7Bblue%7D%7B%5Ctimes%20%5Cleft(1%2Br)%7D" />
//!
//! ## Future Value but no Present Value
//!
//! If there's a future value and the present value is zero, and the payment is due at the end of
//! the period, the formula is:
//!
//! > <img src="http://i.upmath.me/svg/payment%20%3D%20%7Bfuture%5C_value%20%5Ctimes%20-rate%20%5Cover%20%7B%5Cleft(1%2Brate%5Cright)%5E%7Bperiods%7D%20-%201%7D" />
//!
//! or:
//!
//! > <img src="http://i.upmath.me/svg/pmt%20%3D%20%7Bfv%20%5Ctimes%20-r%20%5Cover%20%5Cleft(1%2Br%5Cright)%5En%20-%201%5Cright%7D" />
//!
//! If the payment is due at the beginning of the period it's:
//!
//! > <img src="http://i.upmath.me/svg/pmt%20%3D%20%7Bfv%20%5Ctimes%20-r%20%5Cover%20%5Cleft%5B%5Cleft(1%2Br%5Cright)%5En%20-%201%5Cright%5D%20%5Ctextcolor%7Bblue%7D%7B%5Ctimes%20(1%2Br)%7D%7D" />
//!
//! ## Both Present Value and Future Value
//!
//! If both present value and future value are nonzero the formula is:
//!
//! > <img src="http://i.upmath.me/svg/payment%20%3D%20%7B%5Cleft%5C%7B%5Cleft%5Bpresent%5C_value%20%5Ctimes%20%5Cleft(1%2Brate%5Cright)%5E%7Bperiods%7D%5Cright%5D%2Bfuture%5C_value%5Cright%5C%7D%20%5Ctimes%20-rate%20%5Cover%20%5Cleft(1%2Brate%5Cright)%5E%7Bperiods%7D%20-%201%7D" />
//!
//! or:
//!
//! > <img src="http://i.upmath.me/svg/pmt%20%3D%20%7B%5Cleft%5C%7B%5Cleft%5Bpv%20%5Ctimes%20%5Cleft(1%2Br%5Cright)%5En%5Cright%5D%2Bfv%5Cright%5C%7D%20%5Ctimes%20-r%20%5Cover%20%5Cleft(1%2Br%5Cright)%5En%20-%201%7D" />
//!
//! If the payment is due at the beginning of the period it's:
//!
//! > <img src="http://i.upmath.me/svg/pmt%20%3D%20%7B%5Cleft%5C%7B%5Cleft%5Bpv%20%5Ctimes%20%5Cleft(1%2Br%5Cright)%5En%5Cright%5D%2Bfv%5Cright%5C%7D%20%5Ctimes%20-r%20%5Cover%20%5Cleft%5B%5Cleft(1%2Br%5Cright)%5En%20-%201%5Cright%5D%20%5Ctextcolor%7Bblue%7D%7B%5Ctimes%20(1%2Br)%7D%7D" />
//!

use log::{warn};

// Import needed for the function references in the Rustdoc comments.
#[allow(unused_imports)]
use crate::*;
use std::ops::Deref;

const RUN_PAYMENT_INVARIANTS: bool = false;

#[derive(Clone, Debug)]
pub struct PaymentSolution(CashflowSolution);

#[derive(Clone, Debug)]
pub struct PaymentSeries(CashflowSeries);

impl PaymentSolution {
    pub(crate) fn new(solution: CashflowSolution) -> Self {
        Self {
            0: solution,
        }
    }
    pub fn print_table(&self) {
        self.series().print_table(true, true)
    }

    /// Calculates the period-by-period details of a payment calculation including how the payment
    /// is broken down between principal and interest.
    ///
    /// # Examples
    /// An amortized loan. Uses [`payment`].
    /// ```
    /// let years = 5;
    ///
    /// // The annual percentage rate is 15% and the interest will compound monthly.
    /// let rate = finance::convert_apr_to_epr(0.15, 12);
    ///
    /// // Each period will be one month.
    /// let periods = years * 12;
    ///
    /// // The amount of the loan is $20,000.
    /// let present_value = 20_000;
    ///
    /// // The loan will be fully paid off ot the end of the last period.
    /// let future_value = 0;
    ///
    /// // Payments are due at the end of the month.
    /// let due_at_beginning = false;
    ///
    /// // Calculate the payment, creating a struct that contains additional information and the option
    /// // to generate period-by-period details.
    /// let solution = finance::payment_solution(rate, periods, present_value, future_value, due_at_beginning);
    /// dbg!(&solution);
    ///
    /// // Calculate the month-by-month details including the principal and interest paid every month.
    /// let series = solution.series();
    /// dbg!(&series);
    ///
    /// // Confirm that we have one entry for each period.
    /// assert_eq!(periods as usize, series.len());
    ///
    /// // Print the period detail numbers as a formatted table.
    /// let include_running_totals = true;
    /// let include_remaining_amounts = true;
    /// let locale = finance::num_format::Locale::en;
    /// let precision = 2; // Two decimal places.
    /// series.print_table_locale(include_running_totals, include_remaining_amounts, &locale, precision);
    ///
    /// // As above but print only the last period for every yeor of the loan, that is periods 12, 24,
    /// // 36, 48, and 60; and use default formatting.
    /// series
    ///     .filter(|x| x.period() % 12 == 0)
    ///     .print_table(include_running_totals, include_remaining_amounts);
    /// ```
    pub fn series(&self) -> PaymentSeries {
        let mut series = vec![];
        if self.future_value() != 0.0 {
            return PaymentSeries::new(CashflowSeries::new(series));
        }
        let mut payments_to_date = 0.0;
        let mut principal_to_date = 0.0;
        let mut interest_to_date = 0.0;
        for period in 1..=self.periods() {
            let principal_remaining_at_start_of_period = self.present_value() + principal_to_date;
            let interest = if self.due_at_beginning() && period == 1 {
                0.0
            } else {
                -principal_remaining_at_start_of_period * self.rate()
            };
            let principal = self.payment() - interest;
            payments_to_date += self.payment();
            principal_to_date += principal;
            interest_to_date += interest;
            let payments_remaining = self.sum_of_payments() - payments_to_date;
            let principal_remaining = -(self.present_value() + principal_to_date);
            let interest_remaining = self.sum_of_interest() - interest_to_date;
            let (formula, symbolic_formula) = if self.due_at_beginning() && period == 1 {
                ("0".to_string(), "interest = 0".to_string())
            } else {
                let formula = format!("{:.4} = -({:.4} * {:.6})", interest, principal_remaining_at_start_of_period, self.rate());
                let symbolic_formula = "interest = -(principal * rate)".to_string();
                (formula, symbolic_formula)
            };
            let entry = CashflowPeriod::new(period, self.rate(), self.due_at_beginning(), self.payment(), payments_to_date,
                                               payments_remaining, principal, principal_to_date, principal_remaining, interest,
                                               interest_to_date, interest_remaining, formula, symbolic_formula);
            series.push(entry);
        }
        let payment_series = PaymentSeries::new(CashflowSeries::new(series));
        if RUN_PAYMENT_INVARIANTS {
            payment_series.invariant(self);
        }
        payment_series
    }

    pub fn print_ab_comparison(
        &self,
        other: &PaymentSolution,
        include_running_totals: bool,
        include_remaining_amounts: bool)
    {
        self.print_ab_comparison_locale_opt(other, include_running_totals, include_remaining_amounts, None, None);
    }

    pub fn print_ab_comparison_locale(
        &self,
        other: &PaymentSolution,
        include_running_totals: bool,
        include_remaining_amounts: bool,
        locale: &num_format::Locale,
        precision: usize)
    {
        self.print_ab_comparison_locale_opt(other, include_running_totals, include_remaining_amounts, Some(locale), Some(precision));
    }

    fn print_ab_comparison_locale_opt(
        &self,
        other: &PaymentSolution,
        include_running_totals: bool,
        include_remaining_amounts: bool,
        locale: Option<&num_format::Locale>,
        precision: Option<usize>)
    {
        println!();
        // print_ab_comparison_values_string("calculated_field", &self.calculated_field().to_string(), &other.calculated_field.to_string());
        print_ab_comparison_values_rate("rate", self.rate(), other.rate(), locale, precision);
        print_ab_comparison_values_int("periods", self.periods() as i128, other.periods() as i128, locale);
        print_ab_comparison_values_float("present_value", self.present_value(), other.present_value(), locale, precision);
        print_ab_comparison_values_float("future_value", self.future_value(), other.future_value(), locale, precision);
        print_ab_comparison_values_bool("due_at_beginning", self.due_at_beginning(), other.due_at_beginning());
        print_ab_comparison_values_float("payment", self.payment(), other.payment(), locale, precision);
        print_ab_comparison_values_float("sum_of_payments", self.sum_of_payments(), other.sum_of_payments(), locale, precision);
        print_ab_comparison_values_float("sum_of_interest", self.sum_of_interest(), other.sum_of_interest(), locale, precision);
        print_ab_comparison_values_string("formula", &self.formula(), &other.formula());
        print_ab_comparison_values_string("symbolic_formula", &self.symbolic_formula(), &other.symbolic_formula());

        self.series().print_ab_comparison_locale_opt(&other.series(), include_running_totals, include_remaining_amounts, locale, precision);
    }

    fn invariant(&self) {
        let rate = self.rate();
        let periods = self.periods();
        let present_value = self.present_value();
        let future_value = self.future_value();
        let payment = self.payment();
        let sum_of_payments = self.sum_of_payments();
        let sum_of_interest = self.sum_of_interest();
        let formula = self.formula();
        let symbolic_formula = self.symbolic_formula();
        let present_and_future_value= present_value + future_value;
        assert!(self.calculated_field().is_payment());
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
        assert!(symbolic_formula.len() > 0);
    }

}

impl Deref for PaymentSolution {
    type Target = CashflowSolution;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PaymentSeries {
    pub(crate) fn new(series: CashflowSeries) -> Self {
        Self {
            0: series,
        }
    }

    fn invariant(&self, solution: &CashflowSolution) {
        let periods = solution.periods();
        if solution.future_value() != 0.0 {
            assert_eq!(0, self.len());
        } else {
            assert_eq!(periods as usize, self.len());
        }
        if self.len() == 0 {
            return;
        }
        let rate = solution.rate();
        let present_value = solution.present_value();
        let due_at_beginning = solution.due_at_beginning();
        assert!(solution.calculated_field().is_payment());
        let payment = solution.payment();
        let sum_of_payments = solution.sum_of_payments();
        let sum_of_interest = solution.sum_of_interest();
        let mut running_sum_of_payments = 0.0;
        let mut running_sum_of_principal = 0.0;
        let mut running_sum_of_interest = 0.0;
        let mut previous_principal: Option<f64> = None;
        let mut previous_interest: Option<f64> = None;
        for (index, entry) in self.iter().enumerate() {
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
                    // if entry.principal() <= 0.0 {
                    //     bg!(&solution, &series[..10]);
                    // }
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
                // if !is_approx_equal!(0.0, entry.principal_remaining()) {
                //     bg!(&solution, &series[..10], &series[250], &series[490..500]);
                //}
                assert_approx_equal!(0.0, entry.principal_remaining());
                assert_approx_equal!(0.0, entry.interest_remaining());
            }
            assert!(entry.formula().len() > 0);
            assert!(entry.symbolic_formula().len() > 0);

            previous_principal = Some(entry.principal());
            previous_interest = Some(entry.interest());
        }
        assert_approx_equal!(running_sum_of_payments, sum_of_payments);
        assert_approx_equal!(running_sum_of_principal, -present_value);
        assert_approx_equal!(running_sum_of_interest, sum_of_interest);
    }
}

impl Deref for PaymentSeries {
    type Target = CashflowSeries;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Returns the payment needed at the end of every period for an amortized loan.
///
/// Related functions:
/// * To calculate the payment needed at the end of each period and return a struct that shows the
/// interest, the formula, and optionally the period-by-period values use [`payment_solution`].
///
/// In the typical case where there's a present value and the future value is zero, and the payment
/// is due at the end of the period, the formula is:
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
/// If the payment is due at the beginning of the period, the only difference is that the payment
/// is divided by (1 + rate). In our formulas this means multiplying the denominator by (1 + rate)
/// so in the typical case where there's a present value and the future value is zero, the formula
/// is:
/// > payment = ((present_value * (1 + rate)<sup>periods</sup>) * -rate) / (((1 + rate)<sup>periods</sup> - 1) * (1 + rate))
///
/// or with the more commonly used variables:
/// > pmt = ((pv * (1 + r)<sup>n</sup>) * -r) / (((1 + r)<sup>n</sup> - 1) * (1 + r))",
///
/// This is nearly the same formula as the one for payments due at the end of the period. The
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
/// * `due_at_beginning` - True if the payment is due at the beginning of the period. Typically the
/// payment will be due at the end of the period so this will be false.
///
/// # Panics
/// The call will fail if `rate` is less than -1.0.
///
/// # Examples
/// A simple amortized loan with the payment due at the end of the month.
/// ```
/// # use finance::*;
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
/// let due_at_beginning = false;
///
/// let pmt = payment(rate, periods, present_value, future_value, due_at_beginning);
/// dbg!(pmt);
///
/// // The payment is $212.47/month. Since the principal/present value was positive the payment is
/// // negative.
/// assert_rounded_4(pmt, -212.4704);
///
///
/// // As above except this time the payment is due at the beginning of the month. This will reduce
/// // the payment slightly.
/// let due_at_beginning = true;
/// let pmt = payment(rate, periods, present_value, future_value, due_at_beginning);
/// dbg!(pmt);
///
/// // The payment is $210.7145, shown as negative since the present value was positive. It's
/// // slightly smaller (that is, closer to zero) than the payment in the case above where the
/// // payment was due at the end of the month.
/// assert_rounded_4(pmt, -210.7145);
///
/// ```
pub fn payment<P, F>(rate: f64, periods: u32, present_value: P, future_value: F, due_at_beginning: bool) -> f64
    where
        P: Into<f64> + Copy,
        F: Into<f64> + Copy
{
    let present_value = present_value.into();
    let future_value = future_value.into();

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

/// Calculates the payment needed for each period for an amortized loan and creates a  struct
/// showing the interest, the formula, and optionally the period-by-period values.
///
/// Related functions:
/// * To calculate the payment as a simple number instead of a struct when the payment is due at the
/// end of each period use [payment](./fn.payment.html).
///
/// In the typical case where there's a present value and the future value is zero, and the payment
/// is due at the end of the period, the formula is:
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
/// If the payment is due at the beginning of the period, the only difference is that the payment
/// is divided by (1 + rate). In our formulas this means multiplying the denominator by (1 + rate)
/// so in the typical case where there's a present value and the future value is zero, the formula
/// is:
/// > payment = ((present_value * (1 + rate)<sup>periods</sup>) * -rate) / (((1 + rate)<sup>periods</sup> - 1) * (1 + rate))
///
/// or with the more commonly used variables:
/// > pmt = ((pv * (1 + r)<sup>n</sup>) * -r) / (((1 + r)<sup>n</sup> - 1) * (1 + r))",
///
/// This is nearly the same formula as the one for payments due at the end of the period. The
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
/// * `due_at_beginning` - True if the payment is due at the beginning of the period. Typically the
/// payment will be due at the end of the period so this will be false.
///
/// # Panics
/// The call will fail if `rate` is less than -1.0.
///
/// # Examples
/// Calculate the payment for a simple amortized loan with the payment due at the end of the month,
/// then examine the formulas and the period-by-period details such as the amount of the payment
/// that goes to principal and interest.
/// ```
/// # use finance::*;
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
/// let due_at_beginning = false;
///
/// let solution = payment_solution(rate, periods, present_value, future_value, due_at_beginning);
/// // Display the inputs, payment amount, formulas, sum of interest, etc.
/// dbg!(&solution);
///
/// // The payment is $327.65/month. Since the principal/present value was negative the payment is
/// // positive.
/// assert_rounded_4(solution.payment(), 327.6538);
///
/// // The sum of payments is simply the monthly payment times the number of months.
/// assert_rounded_4(solution.sum_of_payments(), 15_727.3820);
///
/// // The sum of interest is the portion of the sum of payments that is over and above the original
/// // loan amount. Here we add the present value since it has the opposite sign of the payments.
/// assert_rounded_4(solution.sum_of_interest(), solution.sum_of_payments() + solution.present_value());
/// assert_rounded_4(solution.sum_of_interest(), 3_226.8820);
///
/// // Examine the formulas. Since the future value is zero we expect to see a slightly simplified
/// // formula.
/// let formula = solution.formula();
/// println!();
/// dbg!(&formula);
/// assert_eq!(formula, "327.6538 = (-12500.5000 * 1.009792^48 * -0.009792) / (1.009792^48 - 1)");
/// let symbolic_formula = solution.symbolic_formula();
/// println!();
/// dbg!(&symbolic_formula);
/// assert_eq!(symbolic_formula, "pmt = (pv * (1 + r)^n * -r) / ((1 + r)^n - 1)");
///
/// // Calculate the period-by-period values including the amount of the payment that goes toward
/// // interest and principle as well as the running tally of the remaining amounts.
/// let series = solution.series();
/// // Note that all of the period-by-period values are shown as of the end of the period after that
/// // period's payment has been made.
/// println!();
/// dbg!(&series);
///
/// // Print the period-by-period values in a table with two decimal places and the numbers aligned,
/// // with Vietnamese formatting for the thousands and decimal separators. Show all columns
/// // including running totals and remaining amounts.
/// let locale = &finance::num_format::Locale::vi;
/// let include_running_totals = true;
/// let include_remaining_amounts = true;
/// println!();
/// series.print_table_locale(include_running_totals, include_remaining_amounts, &locale, 2);
///
/// // Print a table with only the last period of each year, that is all of the periods that can be
/// // divided by 12. Include the running totals columns but not remaining amounts. Also this time
/// // use default formatting without specifying the locale or number of decimal places.
/// let include_running_totals = true;
/// let include_remaining_amounts = false;
/// println!();
/// series
///     .filter(|entry| entry.period() % 12 == 0)
///     .print_table(include_running_totals, include_remaining_amounts);
///
/// // Print a table starting at the first period where at least 95% of the interest has been paid
/// // off, and round all dollar amounts to whole numbers by passing zero as the second argument to
/// // print_table_locale(). Include the remanining amounts columns but not the running totals.
/// let include_running_totals = false;
/// let include_remaining_amounts = true;
/// println!();
/// series
///     .filter(|entry| entry.interest_to_date() >= solution.sum_of_interest() * 0.95)
///     .print_table_locale(include_running_totals, include_remaining_amounts, locale, 0);
/// ```
pub fn payment_solution<P, F>(rate: f64, periods: u32, present_value: P, future_value: F, due_at_beginning: bool) -> PaymentSolution
    where
        P: Into<f64> + Copy,
        F: Into<f64> + Copy
{
    let present_value = present_value.into();
    let future_value = future_value.into();
    let payment = payment(rate, periods, present_value, future_value, due_at_beginning);
    let (formula, symbolic_formula) = payment_formula(rate, periods, present_value, future_value, due_at_beginning, payment);
    let solution = PaymentSolution::new(CashflowSolution::new(CashflowVariable::Payment, rate, periods, present_value, future_value, due_at_beginning, payment, &formula, &symbolic_formula));
    if RUN_PAYMENT_INVARIANTS {
        solution.invariant();
    }
    solution
}

fn payment_formula(rate: f64, periods: u32, present_value: f64, future_value: f64, due_at_beginning: bool, payment: f64) -> (String, String) {
    let rate_multiplier = 1.0 + rate;
    let (mut formula, mut symbolic_formula) = if periods == 0 {
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
            let symbolic_formula = "(-pv - fv) / n".to_string();
            (formula, symbolic_formula)
        }
    } else {
        let (formula_num, symbolic_formula_num) = if future_value == 0.0 {
            // We can slightly simplify the formula by not including the future value term.
            (format!("({:.4} * {:.6}^{} * {:.6})", present_value, rate_multiplier, periods, -rate), "(pv * (1 + r)^n * -r)".to_string())
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
        let mut symbolic_formula_denom = "((1 + r)^n - 1)".to_string();
        if due_at_beginning {
            formula_denom = format!("({} * {:.6})", formula_denom, rate_multiplier);
            symbolic_formula_denom = format!("({} * (1 + r))", symbolic_formula_denom);
        }
        (format!("{} / {}", formula_num, formula_denom), format!("{} / {}", symbolic_formula_num, symbolic_formula_denom))
    };
    formula = format!("{:.4} = {}", payment, formula);
    symbolic_formula = format!("pmt = {}", symbolic_formula);
    (formula, symbolic_formula)
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
    fn test_payment_due_at_end_nominal() {
        assert_approx_equal!(-11.9636085342686f64, payment(0.034, 10, 100.0, 0.0, false));
        assert_approx_equal!(-8.22683411973293f64, payment(-0.034, 10, 100.0, 0.0, false));
        assert_approx_equal!(11.9636085342686f64, payment(0.034, 10, -100.0, 0.0, false));
        assert_approx_equal!(-100.097751710655f64, payment(1.0, 10, 100.0, 0.0, false));
        assert_approx_equal!(-150.01573028944f64, payment(1.5, 10, 100.0, 0.0, false));
        assert_approx_equal!(-10f64, payment(0.0, 10, 100.0, 0.0, false));
        assert_approx_equal!(-10.00055000825f64, payment(0.00001, 10, 100.0, 0.0, false));
        assert_approx_equal!(8.22683411973293f64, payment(-0.034, 10, -100.0, 0.0, false));
        assert_approx_equal!(9.82270640070143f64, payment(0.034, 10, -100.0, 25.0, false));
        assert_approx_equal!(14.1045106678357f64, payment(0.034, 10, -100.0, -25.0, false));
        assert_approx_equal!(-10f64, payment(0.0, 10, 100.0, 0.0, false));
        assert_approx_equal!(-11f64, payment(0.0, 10, 100.0, 10.0, false));
        assert_approx_equal!(-9f64, payment(0.0, 10, 100.0, -10.0, false));
        assert_approx_equal!(-1f64, payment(0.0, 10, 10.0, 0.0, false));
        assert_approx_equal!(-11f64, payment(0.0, 10, 10.0, 100.0, false));
        assert_approx_equal!(9f64, payment(0.0, 10, 10.0, -100.0, false));
        assert_approx_equal!(10f64, payment(0.0, 10, -100.0, 0.0, false));
        assert_approx_equal!(9f64, payment(0.0, 10, -100.0, 10.0, false));
        assert_approx_equal!(11f64, payment(0.0, 10, -100.0, -10.0, false));
        assert_approx_equal!(1f64, payment(0.0, 10, -10.0, 0.0, false));
        assert_approx_equal!(-9f64, payment(0.0, 10, -10.0, 100.0, false));
        assert_approx_equal!(11f64, payment(0.0, 10, -10.0, -100.0, false));
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
    fn test_payment_due_at_beginning_nominal() {
        assert_approx_equal!(-11.5702210196021f64, payment(0.034, 10, 100.0, 0.0, true));
        assert_approx_equal!(-8.51639142829496f64, payment(-0.034, 10, 100.0, 0.0, true));
        assert_approx_equal!(-8.51639142829496f64, payment(-0.034, 10, 100.0, 0.0, true));
        assert_approx_equal!(-50.0488758553275f64, payment(1.0, 10, 100.0, 0.0, true));
        assert_approx_equal!(-60.0062921157762f64, payment(1.5, 10, 100.0, 0.0, true));
        assert_approx_equal!(-10f64, payment(0.0, 10, 100.0, 0.0, true));
        assert_approx_equal!(-10.0004500037499f64, payment(0.00001, 10, 100.0, 0.0, true));
        assert_approx_equal!(8.51639142829496f64, payment(-0.034, 10, -100.0, 0.0, true));
        assert_approx_equal!(9.49971605483697f64, payment(0.034, 10, -100.0, 25.0, true));
        assert_approx_equal!(13.6407259843672f64, payment(0.034, 10, -100.0, -25.0, true));
        assert_approx_equal!(-10f64, payment(0.0, 10, 100.0, 0.0, true));
        assert_approx_equal!(-11f64, payment(0.0, 10, 100.0, 10.0, true));
        assert_approx_equal!(-9f64, payment(0.0, 10, 100.0, -10.0, true));
        assert_approx_equal!(-1f64, payment(0.0, 10, 10.0, 0.0, true));
        assert_approx_equal!(-11f64, payment(0.0, 10, 10.0, 100.0, true));
        assert_approx_equal!(9f64, payment(0.0, 10, 10.0, -100.0, true));
        assert_approx_equal!(10f64, payment(0.0, 10, -100.0, 0.0, true));
        assert_approx_equal!(9f64, payment(0.0, 10, -100.0, 10.0, true));
        assert_approx_equal!(11f64, payment(0.0, 10, -100.0, -10.0, true));
        assert_approx_equal!(1f64, payment(0.0, 10, -10.0, 0.0, true));
        assert_approx_equal!(-9f64, payment(0.0, 10, -10.0, 100.0, true));
        assert_approx_equal!(11f64, payment(0.0, 10, -10.0, -100.0, true));
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

    fn run_payment_invariants(solution: &PaymentSolution, series: &PaymentSeries) {
        // Display the solution and series only if either one fails its invariant.
        let result = std::panic::catch_unwind(|| {
            solution.invariant();
            series.invariant(solution);
        });
        //bg!(&result);
        if result.is_err() {
            dbg!(&solution, &series);
            solution.invariant();
            series.invariant(solution);
        }
    }

}

