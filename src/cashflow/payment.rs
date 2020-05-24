//! **Payment calculations.** What is the periodic payment needed for an amortized loan and how much
//! of that is interest or principal?
//! 
//! For most common usages, we recommend the [`payment_solution`](./fn.payment_solution.html) function, which provides a better debugging experience and additional functionality.
//!
//! ## Example
//! ```
//! let (rate, periods, present_value, future_value, due_at_beginning) = (0.034, 10, 1000, 0, false);
//! let solution = finance_solution::payment_solution(rate, periods, present_value, future_value, due_at_beginning);
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
//! # use finance_solution::*;
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
//! use finance_solution::*;
//!
//! let (rate, periods, present_value, due_at_beginning) = (0.01, 120, 100_000, false);
//!
//! let pmt_positive_present_value = payment(rate, periods, present_value, 0.0, due_at_beginning);
//! assert_rounded_2!(-1_434.71, pmt_positive_present_value);
//!
//! let pmt_negative_present_value = payment(rate, periods, -present_value, 0.0, due_at_beginning);
//! assert_rounded_2!(1_434.71, pmt_negative_present_value);
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
    /// use finance_solution::*;
    ///
    /// let years = 5;
    ///
    /// // The annual percentage rate is 15% and the interest will compound monthly.
    /// let rate = convert_apr_to_epr(0.15, 12);
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
    /// let solution = payment_solution(rate, periods, present_value, future_value, due_at_beginning);
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
    /// let locale = num_format::Locale::en;
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
        print_ab_comparison_values_int("periods", i128::from(self.periods()), i128::from(other.periods()), locale);
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
        assert!(!formula.is_empty());
        assert!(!symbolic_formula.is_empty());
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
            } else if present_value > 0.0 {
                assert!(entry.principal() < 0.0);
                assert!(entry.interest() < 0.0);
            } else {
                // if entry.principal() <= 0.0 {
                //     bg!(&solution, &series[..10]);
                // }
                assert!(entry.principal() > 0.0);
                assert!(entry.interest() > 0.0);
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
            assert!(!entry.formula().is_empty());
            assert!(!entry.symbolic_formula().is_empty());

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
/// # use finance_solution::*;
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
/// # use finance_solution::*;
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
/// let locale = &num_format::Locale::vi;
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
        } else if present_value == 0.0 {
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

    #[test]
    fn test_against_excel_ipmt_month_1() {
        // Payments at the end of the period.
        let solution = payment_solution(0.0056, 12, 20000.0, 0.0, false);
        assert_approx_equal!(-1727.95439349254, solution.payment());
        let series = solution.series();
        assert_approx_equal!(-112.0, series[0].interest());
        assert_approx_equal!(-102.950655396442, series[1].interest());
        assert_approx_equal!(-93.8506344631036, series[2].interest());
        assert_approx_equal!(-84.6996534125387, series[3].interest());
        assert_approx_equal!(-75.4974268680907, series[4].interest());
        assert_approx_equal!(-66.2436678549938, series[5].interest());
        assert_approx_equal!(-56.9380877914235, series[6].interest());
        assert_approx_equal!(-47.5803964794972, series[7].interest());
        assert_approx_equal!(-38.1703020962241, series[8].interest());
        assert_approx_equal!(-28.7075111844048, series[9].interest());
        assert_approx_equal!(-19.1917286434792, series[10].interest());
        assert_approx_equal!(-9.62265772032443, series[11].interest());
    }

    #[test]
    fn test_against_excel_ipmt_month_2() {
        // Payments at the beginning of the period.
        let solution = payment_solution(0.0056, 12, 20000.0, 0.0, true);
        assert_approx_equal!(-1718.33173577222, solution.payment());
        let series = solution.series();
        assert_approx_equal!(0.0, series[0].interest());
        assert_approx_equal!(-102.377342279676, series[1].interest());
        assert_approx_equal!(-93.3279976761173, series[2].interest());
        assert_approx_equal!(-84.2279767427791, series[3].interest());
        assert_approx_equal!(-75.0769956922143, series[4].interest());
        assert_approx_equal!(-65.8747691477663, series[5].interest());
        assert_approx_equal!(-56.6210101346693, series[6].interest());
        assert_approx_equal!(-47.315430071099, series[7].interest());
        assert_approx_equal!(-37.9577387591728, series[8].interest());
        assert_approx_equal!(-28.5476443758997, series[9].interest());
        assert_approx_equal!(-19.0848534640803, series[10].interest());
        assert_approx_equal!(-9.56907092315476, series[11].interest());
    }

    #[test]
    fn test_against_excel_pmt_mod_5() {
        assert_approx_equal!(26063.0811111111f64, payment(-0.1, 1, -12345.67, -12345.67, true));
        assert_approx_equal!(11111.103f64, payment(-0.1, 1, -12345.67, 0.0, false));
        assert_approx_equal!(12208.4958888889f64, payment(-0.1, 1, -12345.67, 123.4567, true));
        assert_approx_equal!(234.56773f64, payment(-0.1, 1, -123.4567, -123.4567, false));
        assert_approx_equal!(123.4567f64, payment(-0.1, 1, -123.4567, 0.0, true));
        assert_approx_equal!(-12234.55897f64, payment(-0.1, 1, -123.4567, 12345.67, false));
        assert_approx_equal!(138.408678111111f64, payment(-0.1, 1, -1.234567, -123.4567, true));
        assert_approx_equal!(-0.1234567f64, payment(-0.1, 1, -1.234567, 1.234567, false));
        assert_approx_equal!(-13716.1765441111f64, payment(-0.1, 1, -1.234567, 12345.67, true));
        assert_approx_equal!(1.234567f64, payment(-0.1, 1, 0.0, -1.234567, false));
        assert_approx_equal!(-1.37174111111111f64, payment(-0.1, 1, 0.0, 1.234567, true));
        assert_approx_equal!(12344.5588897f64, payment(-0.1, 1, 1.234567, -12345.67, false));
        assert_approx_equal!(0.137174111111111f64, payment(-0.1, 1, 1.234567, -1.234567, true));
        assert_approx_equal!(-124.5678103f64, payment(-0.1, 1, 1.234567, 123.4567, false));
        assert_approx_equal!(13593.9544111111f64, payment(-0.1, 1, 123.4567, -12345.67, true));
        assert_approx_equal!(-111.11103f64, payment(-0.1, 1, 123.4567, 0.0, false));
        assert_approx_equal!(-260.630811111111f64, payment(-0.1, 1, 123.4567, 123.4567, true));
        assert_approx_equal!(-10987.6463f64, payment(-0.1, 1, 12345.67, -123.4567, false));
        assert_approx_equal!(-12345.67f64, payment(-0.1, 1, 12345.67, 0.0, true));
        assert_approx_equal!(-23456.773f64, payment(-0.1, 1, 12345.67, 12345.67, false));
        assert_approx_equal!(5920.14584795322f64, payment(-0.1, 2, -12345.67, -123.4567, true));
        assert_approx_equal!(5262.50428052632f64, payment(-0.1, 2, -12345.67, 1.234567, false));
        assert_approx_equal!(-1371.74111111111f64, payment(-0.1, 2, -12345.67, 12345.67, true));
        assert_approx_equal!(53.2813126315789f64, payment(-0.1, 2, -123.4567, -1.234567, false));
        assert_approx_equal!(57.7575204678363f64, payment(-0.1, 2, -123.4567, 1.234567, true));
        assert_approx_equal!(6498.24736803684f64, payment(-0.1, 2, -1.234567, -12345.67, false));
        assert_approx_equal!(1.3067639005848f64, payment(-0.1, 2, -1.234567, -1.234567, true));
        assert_approx_equal!(-64.4508951210526f64, payment(-0.1, 2, -1.234567, 123.4567, false));
        assert_approx_equal!(7219.69005847953f64, payment(-0.1, 2, 0.0, -12345.67, true));
        assert_approx_equal!(0f64, payment(-0.1, 2, 0.0, 0.0, false));
        assert_approx_equal!(-72.1969005847953f64, payment(-0.1, 2, 0.0, 123.4567, true));
        assert_approx_equal!(64.4508951210526f64, payment(-0.1, 2, 1.234567, -123.4567, false));
        assert_approx_equal!(-0.584794894736842f64, payment(-0.1, 2, 1.234567, 0.0, true));
        assert_approx_equal!(-6498.24736803684f64, payment(-0.1, 2, 1.234567, 12345.67, false));
        assert_approx_equal!(13.7174111111111f64, payment(-0.1, 2, 123.4567, -123.4567, true));
        assert_approx_equal!(-53.2813126315789f64, payment(-0.1, 2, 123.4567, 1.234567, false));
        assert_approx_equal!(-7278.16954795322f64, payment(-0.1, 2, 123.4567, 12345.67, true));
        assert_approx_equal!(-5262.50428052632f64, payment(-0.1, 2, 12345.67, -1.234567, false));
        assert_approx_equal!(-5848.67091637427f64, payment(-0.1, 2, 12345.67, 1.234567, true));
        assert_approx_equal!(4794.91701748431f64, payment(-0.1, 5, -12345.67, -12345.67, false));
        assert_approx_equal!(1978.30720327003f64, payment(-0.1, 5, -12345.67, -1.234567, true));
        assert_approx_equal!(1750.02758865473f64, payment(-0.1, 5, -12345.67, 123.4567, false));
        assert_approx_equal!(3369.4930653662f64, payment(-0.1, 5, -123.4567, -12345.67, true));
        assert_approx_equal!(17.8017500874216f64, payment(-0.1, 5, -123.4567, 0.0, false));
        assert_approx_equal!(-13.7174111111111f64, payment(-0.1, 5, -123.4567, 123.4567, true));
        assert_approx_equal!(30.3254375882958f64, payment(-0.1, 5, -1.234567, -123.4567, false));
        assert_approx_equal!(0.197797223193573f64, payment(-0.1, 5, -1.234567, 0.0, true));
        assert_approx_equal!(-3014.56399124128f64, payment(-0.1, 5, -1.234567, 12345.67, false));
        assert_approx_equal!(33.4971334304684f64, payment(-0.1, 5, 0.0, -123.4567, true));
        assert_approx_equal!(-0.301474200874216f64, payment(-0.1, 5, 0.0, 1.234567, false));
        assert_approx_equal!(-3349.71334304684f64, payment(-0.1, 5, 0.0, 12345.67, true));
        assert_approx_equal!(0.1234567f64, payment(-0.1, 5, 1.234567, -1.234567, false));
        assert_approx_equal!(-0.532768557498257f64, payment(-0.1, 5, 1.234567, 1.234567, true));
        assert_approx_equal!(2996.94025865473f64, payment(-0.1, 5, 123.4567, -12345.67, false));
        assert_approx_equal!(-19.4447509850526f64, payment(-0.1, 5, 123.4567, -1.234567, true));
        assert_approx_equal!(-47.9491701748431f64, payment(-0.1, 5, 123.4567, 123.4567, false));
        assert_approx_equal!(1371.74111111111f64, payment(-0.1, 5, 12345.67, -12345.67, true));
        assert_approx_equal!(-1780.17500874216f64, payment(-0.1, 5, 12345.67, 0.0, false));
        assert_approx_equal!(-2011.4693653662f64, payment(-0.1, 5, 12345.67, 123.4567, true));
        assert_approx_equal!(679.867814949844f64, payment(-0.1, 10, -12345.67, -123.4567, false));
        assert_approx_equal!(734.34779422425f64, payment(-0.1, 10, -12345.67, 0.0, true));
        assert_approx_equal!(-1234.567f64, payment(-0.1, 10, -12345.67, 12345.67, false));
        assert_approx_equal!(28.4043669955961f64, payment(-0.1, 10, -123.4567, -123.4567, true));
        assert_approx_equal!(6.41958214653807f64, payment(-0.1, 10, -123.4567, 1.234567, false));
        assert_approx_equal!(-2098.74542739312f64, payment(-0.1, 10, -123.4567, 12345.67, true));
        assert_approx_equal!(0.255639302960365f64, payment(-0.1, 10, -1.234567, -1.234567, false));
        assert_approx_equal!(-0.137174111111111f64, payment(-0.1, 10, -1.234567, 1.234567, true));
        assert_approx_equal!(1895.48001480183f64, payment(-0.1, 10, 0.0, -12345.67, false));
        assert_approx_equal!(0.210608890533536f64, payment(-0.1, 10, 0.0, -1.234567, true));
        assert_approx_equal!(-18.9548001480183f64, payment(-0.1, 10, 0.0, 123.4567, false));
        assert_approx_equal!(2106.01547055594f64, payment(-0.1, 10, 1.234567, -12345.67, true));
        assert_approx_equal!(-0.0660913014801825f64, payment(-0.1, 10, 1.234567, 0.0, false));
        assert_approx_equal!(-21.134323832776f64, payment(-0.1, 10, 1.234567, 123.4567, true));
        assert_approx_equal!(12.34567f64, payment(-0.1, 10, 123.4567, -123.4567, false));
        assert_approx_equal!(-7.3434779422425f64, payment(-0.1, 10, 123.4567, 0.0, true));
        assert_approx_equal!(-1902.08914494984f64, payment(-0.1, 10, 123.4567, 12345.67, false));
        assert_approx_equal!(-713.286905170897f64, payment(-0.1, 10, 12345.67, -123.4567, true));
        assert_approx_equal!(-661.102562803305f64, payment(-0.1, 10, 12345.67, 1.234567, false));
        assert_approx_equal!(-2840.43669955961f64, payment(-0.1, 10, 12345.67, 12345.67, true));
        assert_approx_equal!(6.51973876437762f64, payment(-0.1, 50, -12345.67, -1.234567, false));
        assert_approx_equal!(6.96838470653066f64, payment(-0.1, 50, -12345.67, 1.234567, true));
        assert_approx_equal!(1241.02659892513f64, payment(-0.1, 50, -123.4567, -12345.67, false));
        assert_approx_equal!(0.208947432501432f64, payment(-0.1, 50, -123.4567, -1.234567, true));
        assert_approx_equal!(-12.34567f64, payment(-0.1, 50, -123.4567, 123.4567, false));
        assert_approx_equal!(1378.84809118264f64, payment(-0.1, 50, -1.234567, -12345.67, true));
        assert_approx_equal!(0.000639564250012761f64, payment(-0.1, 50, -1.234567, 0.0, false));
        assert_approx_equal!(-13.7877631786125f64, payment(-0.1, 50, -1.234567, 123.4567, true));
        assert_approx_equal!(12.4096264250013f64, payment(-0.1, 50, 0.0, -123.4567, false));
        assert_approx_equal!(0f64, payment(-0.1, 50, 0.0, 0.0, true));
        assert_approx_equal!(-1240.96264250013f64, payment(-0.1, 50, 0.0, 12345.67, false));
        assert_approx_equal!(13.7877631786125f64, payment(-0.1, 50, 1.234567, -123.4567, true));
        assert_approx_equal!(-0.124735828500026f64, payment(-0.1, 50, 1.234567, 1.234567, false));
        assert_approx_equal!(-1378.84809118264f64, payment(-0.1, 50, 1.234567, 12345.67, true));
        assert_approx_equal!(0.0601398392487366f64, payment(-0.1, 50, 123.4567, -1.234567, false));
        assert_approx_equal!(-0.208947432501432f64, payment(-0.1, 50, 123.4567, 1.234567, true));
        assert_approx_equal!(1234.567f64, payment(-0.1, 50, 12345.67, -12345.67, false));
        assert_approx_equal!(-6.96838470653066f64, payment(-0.1, 50, 12345.67, -1.234567, true));
        assert_approx_equal!(-18.8052689251289f64, payment(-0.1, 50, 12345.67, 123.4567, false));
        assert_approx_equal!(1371.74111112109f64, payment(-0.1, 250, -12345.67, -12345.67, true));
        assert_approx_equal!(4.48892163617149E-09f64, payment(-0.1, 250, -12345.67, 0.0, false));
        assert_approx_equal!(-13.7174111061733f64, payment(-0.1, 250, -12345.67, 123.4567, true));
        assert_approx_equal!(12.3456700000898f64, payment(-0.1, 250, -123.4567, -123.4567, false));
        assert_approx_equal!(4.98769070685721E-11f64, payment(-0.1, 250, -123.4567, 0.0, true));
        assert_approx_equal!(-1234.56700000444f64, payment(-0.1, 250, -123.4567, 12345.67, false));
        assert_approx_equal!(13.7174111111615f64, payment(-0.1, 250, -1.234567, -123.4567, true));
        assert_approx_equal!(-0.1234567f64, payment(-0.1, 250, -1.234567, 1.234567, false));
        assert_approx_equal!(-1371.7411111161f64, payment(-0.1, 250, -1.234567, 12345.67, true));
        assert_approx_equal!(0.123456700000449f64, payment(-0.1, 250, 0.0, -1.234567, false));
        assert_approx_equal!(-0.13717411111161f64, payment(-0.1, 250, 0.0, 1.234567, true));
        assert_approx_equal!(1234.56700000449f64, payment(-0.1, 250, 1.234567, -12345.67, false));
        assert_approx_equal!(0.137174111111111f64, payment(-0.1, 250, 1.234567, -1.234567, true));
        assert_approx_equal!(-12.3456700000453f64, payment(-0.1, 250, 1.234567, 123.4567, false));
        assert_approx_equal!(1371.74111111605f64, payment(-0.1, 250, 123.4567, -12345.67, true));
        assert_approx_equal!(-4.48892163617149E-11f64, payment(-0.1, 250, 123.4567, 0.0, false));
        assert_approx_equal!(-13.7174111112109f64, payment(-0.1, 250, 123.4567, 123.4567, true));
        assert_approx_equal!(12.345669995556f64, payment(-0.1, 250, 12345.67, -123.4567, false));
        assert_approx_equal!(-4.98769070685721E-09f64, payment(-0.1, 250, 12345.67, 0.0, true));
        assert_approx_equal!(-1234.56700000898f64, payment(-0.1, 250, 12345.67, 12345.67, false));
        assert_approx_equal!(12223.447867f64, payment(-0.01, 1, -12345.67, -1.234567, false));
        assert_approx_equal!(12344.4229626263f64, payment(-0.01, 1, -12345.67, 1.234567, true));
        assert_approx_equal!(12467.892133f64, payment(-0.01, 1, -123.4567, -12345.67, false));
        assert_approx_equal!(124.703737373737f64, payment(-0.01, 1, -123.4567, -1.234567, true));
        assert_approx_equal!(-1.234567f64, payment(-0.01, 1, -123.4567, 123.4567, false));
        assert_approx_equal!(12471.6083043737f64, payment(-0.01, 1, -1.234567, -12345.67, true));
        assert_approx_equal!(1.22222133f64, payment(-0.01, 1, -1.234567, 0.0, false));
        assert_approx_equal!(-123.469170373737f64, payment(-0.01, 1, -1.234567, 123.4567, true));
        assert_approx_equal!(123.4567f64, payment(-0.01, 1, 0.0, -123.4567, false));
        assert_approx_equal!(0f64, payment(-0.01, 1, 0.0, 0.0, true));
        assert_approx_equal!(-12345.67f64, payment(-0.01, 1, 0.0, 12345.67, false));
        assert_approx_equal!(123.469170373737f64, payment(-0.01, 1, 1.234567, -123.4567, true));
        assert_approx_equal!(-2.45678833f64, payment(-0.01, 1, 1.234567, 1.234567, false));
        assert_approx_equal!(-12471.6083043737f64, payment(-0.01, 1, 1.234567, 12345.67, true));
        assert_approx_equal!(-120.987566f64, payment(-0.01, 1, 123.4567, -1.234567, false));
        assert_approx_equal!(-124.703737373737f64, payment(-0.01, 1, 123.4567, 1.234567, true));
        assert_approx_equal!(123.4567f64, payment(-0.01, 1, 12345.67, -12345.67, false));
        assert_approx_equal!(-12344.4229626263f64, payment(-0.01, 1, 12345.67, -1.234567, true));
        assert_approx_equal!(-12345.67f64, payment(-0.01, 1, 12345.67, 123.4567, false));
        assert_approx_equal!(12408.3351946602f64, payment(-0.01, 2, -12345.67, -12345.67, true));
        assert_approx_equal!(6080.39757135678f64, payment(-0.01, 2, -12345.67, 0.0, false));
        assert_approx_equal!(6079.15053398305f64, payment(-0.01, 2, -12345.67, 123.4567, true));
        assert_approx_equal!(122.842518427136f64, payment(-0.01, 2, -123.4567, -123.4567, false));
        assert_approx_equal!(61.4181572864322f64, payment(-0.01, 2, -123.4567, 0.0, true));
        assert_approx_equal!(-6143.05029564322f64, payment(-0.01, 2, -123.4567, 12345.67, false));
        assert_approx_equal!(63.2793762330339f64, payment(-0.01, 2, -1.234567, -123.4567, true));
        assert_approx_equal!(-0.01234567f64, payment(-0.01, 2, -1.234567, 1.234567, false));
        assert_approx_equal!(-6265.90528444409f64, payment(-0.01, 2, -1.234567, 12345.67, true));
        assert_approx_equal!(0.620385427135678f64, payment(-0.01, 2, 0.0, -1.234567, false));
        assert_approx_equal!(-0.626651946601695f64, payment(-0.01, 2, 0.0, 1.234567, true));
        assert_approx_equal!(6203.24623159965f64, payment(-0.01, 2, 1.234567, -12345.67, false));
        assert_approx_equal!(0.0124703737373737f64, payment(-0.01, 2, 1.234567, -1.234567, true));
        assert_approx_equal!(-62.6465824707035f64, payment(-0.01, 2, 1.234567, 123.4567, false));
        assert_approx_equal!(6205.10130873052f64, payment(-0.01, 2, 123.4567, -12345.67, true));
        assert_approx_equal!(-60.8039757135678f64, payment(-0.01, 2, 123.4567, 0.0, false));
        assert_approx_equal!(-124.083351946602f64, payment(-0.01, 2, 123.4567, 123.4567, true));
        assert_approx_equal!(-6018.35902864322f64, payment(-0.01, 2, 12345.67, -123.4567, false));
        assert_approx_equal!(-6141.81572864322f64, payment(-0.01, 2, 12345.67, 0.0, true));
        assert_approx_equal!(-12284.2518427136f64, payment(-0.01, 2, 12345.67, 12345.67, false));
        assert_approx_equal!(2445.19838434817f64, payment(-0.01, 5, -12345.67, -123.4567, true));
        assert_approx_equal!(2395.30436949964f64, payment(-0.01, 5, -12345.67, 1.234567, false));
        assert_approx_equal!(-124.703737373737f64, payment(-0.01, 5, -12345.67, 12345.67, true));
        assert_approx_equal!(24.2074640050469f64, payment(-0.01, 5, -123.4567, -1.234567, false));
        assert_approx_equal!(23.9430923342298f64, payment(-0.01, 5, -123.4567, 1.234567, true));
        assert_approx_equal!(2519.2525264238f64, payment(-0.01, 5, -1.234567, -12345.67, false));
        assert_approx_equal!(0.496421135514489f64, payment(-0.01, 5, -1.234567, -1.234567, true));
        assert_approx_equal!(-24.9505740808875f64, payment(-0.01, 5, -1.234567, 123.4567, false));
        assert_approx_equal!(2544.45754625931f64, payment(-0.01, 5, 0.0, -12345.67, true));
        assert_approx_equal!(0f64, payment(-0.01, 5, 0.0, 0.0, false));
        assert_approx_equal!(-25.4445754625931f64, payment(-0.01, 5, 0.0, 123.4567, true));
        assert_approx_equal!(24.9505740808875f64, payment(-0.01, 5, 1.234567, -123.4567, false));
        assert_approx_equal!(-0.241975380888558f64, payment(-0.01, 5, 1.234567, 0.0, true));
        assert_approx_equal!(-2519.2525264238f64, payment(-0.01, 5, 1.234567, 12345.67, false));
        assert_approx_equal!(1.24703737373737f64, payment(-0.01, 5, 123.4567, -123.4567, true));
        assert_approx_equal!(-24.2074640050469f64, payment(-0.01, 5, 123.4567, 1.234567, false));
        assert_approx_equal!(-2568.65508434817f64, payment(-0.01, 5, 123.4567, 12345.67, true));
        assert_approx_equal!(-2395.30436949964f64, payment(-0.01, 5, 12345.67, -1.234567, false));
        assert_approx_equal!(-2420.0082546402f64, payment(-0.01, 5, 12345.67, 1.234567, true));
        assert_approx_equal!(2458.83527112085f64, payment(-0.01, 10, -12345.67, -12345.67, false));
        assert_approx_equal!(1179.61454561513f64, payment(-0.01, 10, -12345.67, -1.234567, true));
        assert_approx_equal!(1154.77782570482f64, payment(-0.01, 10, -12345.67, 123.4567, false));
        assert_approx_equal!(1315.98270547074f64, payment(-0.01, 10, -123.4567, -12345.67, true));
        assert_approx_equal!(11.6768928556043f64, payment(-0.01, 10, -123.4567, 0.0, false));
        assert_approx_equal!(-1.24703737373737f64, payment(-0.01, 10, -123.4567, 123.4567, true));
        assert_approx_equal!(13.0282287841603f64, payment(-0.01, 10, -1.234567, -123.4567, false));
        assert_approx_equal!(0.117948412682871f64, payment(-0.01, 10, -1.234567, 0.0, true));
        assert_approx_equal!(-1291.02921663187f64, payment(-0.01, 10, -1.234567, 12345.67, false));
        assert_approx_equal!(13.0418786420245f64, payment(-0.01, 10, 0.0, -123.4567, true));
        assert_approx_equal!(-0.129114598556043f64, payment(-0.01, 10, 0.0, 1.234567, false));
        assert_approx_equal!(-1304.18786420245f64, payment(-0.01, 10, 0.0, 12345.67, true));
        assert_approx_equal!(0.01234567f64, payment(-0.01, 10, 1.234567, -1.234567, false));
        assert_approx_equal!(-0.248367199103116f64, payment(-0.01, 10, 1.234567, 1.234567, true));
        assert_approx_equal!(1279.46909270482f64, payment(-0.01, 10, 123.4567, -12345.67, false));
        assert_approx_equal!(-11.6644224818669f64, payment(-0.01, 10, 123.4567, -1.234567, true));
        assert_approx_equal!(-24.5883527112085f64, payment(-0.01, 10, 123.4567, 123.4567, false));
        assert_approx_equal!(124.703737373737f64, payment(-0.01, 10, 12345.67, -12345.67, true));
        assert_approx_equal!(-1167.68928556043f64, payment(-0.01, 10, 12345.67, 0.0, false));
        assert_approx_equal!(-1192.52600547074f64, payment(-0.01, 10, 12345.67, 123.4567, true));
        assert_approx_equal!(192.222242449522f64, payment(-0.01, 50, -12345.67, -123.4567, false));
        assert_approx_equal!(191.006776127135f64, payment(-0.01, 50, -12345.67, 0.0, true));
        assert_approx_equal!(-123.4567f64, payment(-0.01, 50, -12345.67, 12345.67, false));
        assert_approx_equal!(5.06717289628007f64, payment(-0.01, 50, -123.4567, -123.4567, true));
        assert_approx_equal!(1.85971174282205f64, payment(-0.01, 50, -123.4567, 1.234567, false));
        assert_approx_equal!(-313.800445739601f64, payment(-0.01, 50, -123.4567, 12345.67, true));
        assert_approx_equal!(0.0501650116731727f64, payment(-0.01, 50, -1.234567, -1.234567, false));
        assert_approx_equal!(-0.0124703737373737f64, payment(-0.01, 50, -1.234567, 1.234567, true));
        assert_approx_equal!(312.553408365864f64, payment(-0.01, 50, 0.0, -12345.67, false));
        assert_approx_equal!(0.0315710513500872f64, payment(-0.01, 50, 0.0, -1.234567, true));
        assert_approx_equal!(-3.12553408365864f64, payment(-0.01, 50, 0.0, 123.4567, false));
        assert_approx_equal!(315.69141282326f64, payment(-0.01, 50, 1.234567, -12345.67, true));
        assert_approx_equal!(-0.0189096708365864f64, payment(-0.01, 50, 1.234567, 0.0, false));
        assert_approx_equal!(-3.17620581262144f64, payment(-0.01, 50, 1.234567, 123.4567, true));
        assert_approx_equal!(1.234567f64, payment(-0.01, 50, 123.4567, -123.4567, false));
        assert_approx_equal!(-1.91006776127135f64, payment(-0.01, 50, 123.4567, 0.0, true));
        assert_approx_equal!(-314.444375449522f64, payment(-0.01, 50, 123.4567, 12345.67, false));
        assert_approx_equal!(-187.849670992126f64, payment(-0.01, 50, 12345.67, -123.4567, true));
        assert_approx_equal!(-189.1279637067f64, payment(-0.01, 50, 12345.67, 1.234567, false));
        assert_approx_equal!(-506.717289628007f64, payment(-0.01, 50, 12345.67, 12345.67, true));
        assert_approx_equal!(10.9033738910495f64, payment(-0.01, 250, -12345.67, -1.234567, false));
        assert_approx_equal!(10.9863682456607f64, payment(-0.01, 250, -12345.67, 1.234567, true));
        assert_approx_equal!(134.455538619398f64, payment(-0.01, 250, -123.4567, -12345.67, false));
        assert_approx_equal!(0.123569753731294f64, payment(-0.01, 250, -123.4567, -1.234567, true));
        assert_approx_equal!(-1.234567f64, payment(-0.01, 250, -123.4567, 123.4567, false));
        assert_approx_equal!(135.704775980858f64, payment(-0.01, 250, -1.234567, -12345.67, true));
        assert_approx_equal!(0.00108899392271268f64, payment(-0.01, 250, -1.234567, 0.0, false));
        assert_approx_equal!(-1.35593676600864f64, payment(-0.01, 250, -1.234567, 123.4567, true));
        assert_approx_equal!(1.34346639227127f64, payment(-0.01, 250, 0.0, -123.4567, false));
        assert_approx_equal!(0f64, payment(-0.01, 250, 0.0, 0.0, true));
        assert_approx_equal!(-134.346639227127f64, payment(-0.01, 250, 0.0, 12345.67, false));
        assert_approx_equal!(1.35593676600864f64, payment(-0.01, 250, 1.234567, -123.4567, true));
        assert_approx_equal!(-0.0145236578454254f64, payment(-0.01, 250, 1.234567, 1.234567, false));
        assert_approx_equal!(-135.704775980858f64, payment(-0.01, 250, 1.234567, 12345.67, true));
        assert_approx_equal!(-0.0954647283485555f64, payment(-0.01, 250, 123.4567, -1.234567, false));
        assert_approx_equal!(-0.123569753731294f64, payment(-0.01, 250, 123.4567, 1.234567, true));
        assert_approx_equal!(123.4567f64, payment(-0.01, 250, 12345.67, -12345.67, false));
        assert_approx_equal!(-10.9863682456607f64, payment(-0.01, 250, 12345.67, -1.234567, true));
        assert_approx_equal!(-12.2334056193981f64, payment(-0.01, 250, 12345.67, 123.4567, false));
        assert_approx_equal!(12456.78103f64, payment(-0.001, 1, -12345.67, -123.4567, false));
        assert_approx_equal!(12345.67f64, payment(-0.001, 1, -12345.67, 0.0, true));
        assert_approx_equal!(-12.34567f64, payment(-0.001, 1, -12345.67, 12345.67, false));
        assert_approx_equal!(247.03698028028f64, payment(-0.001, 1, -123.4567, -123.4567, true));
        assert_approx_equal!(122.0986763f64, payment(-0.001, 1, -123.4567, 1.234567, false));
        assert_approx_equal!(-12234.571328028f64, payment(-0.001, 1, -123.4567, 12345.67, true));
        assert_approx_equal!(2.467899433f64, payment(-0.001, 1, -1.234567, -1.234567, false));
        assert_approx_equal!(-0.0012358028028028f64, payment(-0.001, 1, -1.234567, 1.234567, true));
        assert_approx_equal!(12345.67f64, payment(-0.001, 1, 0.0, -12345.67, false));
        assert_approx_equal!(1.2358028028028f64, payment(-0.001, 1, 0.0, -1.234567, true));
        assert_approx_equal!(-123.4567f64, payment(-0.001, 1, 0.0, 123.4567, false));
        assert_approx_equal!(12356.793461028f64, payment(-0.001, 1, 1.234567, -12345.67, true));
        assert_approx_equal!(-1.233332433f64, payment(-0.001, 1, 1.234567, 0.0, false));
        assert_approx_equal!(-124.81484728028f64, payment(-0.001, 1, 1.234567, 123.4567, true));
        assert_approx_equal!(0.1234567f64, payment(-0.001, 1, 123.4567, -123.4567, false));
        assert_approx_equal!(-123.4567f64, payment(-0.001, 1, 123.4567, 0.0, true));
        assert_approx_equal!(-12469.0032433f64, payment(-0.001, 1, 123.4567, 12345.67, false));
        assert_approx_equal!(-12222.0897197197f64, payment(-0.001, 1, 12345.67, -123.4567, true));
        assert_approx_equal!(-12334.558897f64, payment(-0.001, 1, 12345.67, 1.234567, false));
        assert_approx_equal!(-24703.698028028f64, payment(-0.001, 1, 12345.67, 12345.67, true));
        assert_approx_equal!(6164.19488377689f64, payment(-0.001, 2, -12345.67, -1.234567, false));
        assert_approx_equal!(6169.12882801261f64, payment(-0.001, 2, -12345.67, 1.234567, true));
        assert_approx_equal!(6237.55873439555f64, payment(-0.001, 2, -123.4567, -12345.67, false));
        assert_approx_equal!(62.3156808918473f64, payment(-0.001, 2, -123.4567, -1.234567, true));
        assert_approx_equal!(-0.1234567f64, payment(-0.001, 2, -123.4567, 123.4567, false));
        assert_approx_equal!(6182.72204125114f64, payment(-0.001, 2, -1.234567, -12345.67, true));
        assert_approx_equal!(0.616357729148074f64, payment(-0.001, 2, -1.234567, 0.0, false));
        assert_approx_equal!(-61.204075961621f64, payment(-0.001, 2, -1.234567, 123.4567, true));
        assert_approx_equal!(61.7592296148074f64, payment(-0.001, 2, 0.0, -123.4567, false));
        assert_approx_equal!(0f64, payment(-0.001, 2, 0.0, 0.0, true));
        assert_approx_equal!(-6175.92296148074f64, payment(-0.001, 2, 0.0, 12345.67, false));
        assert_approx_equal!(61.204075961621f64, payment(-0.001, 2, 1.234567, -123.4567, true));
        assert_approx_equal!(-1.23395002529615f64, payment(-0.001, 2, 1.234567, 1.234567, false));
        assert_approx_equal!(-6182.72204125114f64, payment(-0.001, 2, 1.234567, 12345.67, true));
        assert_approx_equal!(-61.0181806186593f64, payment(-0.001, 2, 123.4567, -1.234567, false));
        assert_approx_equal!(-62.3156808918473f64, payment(-0.001, 2, 123.4567, 1.234567, true));
        assert_approx_equal!(12.34567f64, payment(-0.001, 2, 12345.67, -12345.67, false));
        assert_approx_equal!(-6169.12882801261f64, payment(-0.001, 2, 12345.67, -1.234567, true));
        assert_approx_equal!(-6225.33652109555f64, payment(-0.001, 2, 12345.67, 123.4567, false));
        assert_approx_equal!(4940.74949697025f64, payment(-0.001, 5, -12345.67, -12345.67, true));
        assert_approx_equal!(2461.73153873664f64, payment(-0.001, 5, -12345.67, 0.0, false));
        assert_approx_equal!(2439.43019684612f64, payment(-0.001, 5, -12345.67, 123.4567, true));
        assert_approx_equal!(49.3580874747328f64, payment(-0.001, 5, -123.4567, -123.4567, false));
        assert_approx_equal!(24.6419573447111f64, payment(-0.001, 5, -123.4567, 0.0, true));
        assert_approx_equal!(-2449.45989334927f64, payment(-0.001, 5, -123.4567, 12345.67, false));
        assert_approx_equal!(25.0119571984385f64, payment(-0.001, 5, -1.234567, -123.4567, true));
        assert_approx_equal!(-0.001234567f64, payment(-0.001, 5, -1.234567, 1.234567, false));
        assert_approx_equal!(-2476.30734292569f64, payment(-0.001, 5, -1.234567, 12345.67, true));
        assert_approx_equal!(0.247407720873664f64, payment(-0.001, 5, 0.0, -1.234567, false));
        assert_approx_equal!(-0.247655376249914f64, payment(-0.001, 5, 0.0, 1.234567, true));
        assert_approx_equal!(2473.83103558276f64, payment(-0.001, 5, 1.234567, -12345.67, false));
        assert_approx_equal!(0.0012358028028028f64, payment(-0.001, 5, 1.234567, -1.234567, true));
        assert_approx_equal!(-24.98694524124f64, payment(-0.001, 5, 1.234567, 123.4567, false));
        assert_approx_equal!(2451.91180515443f64, payment(-0.001, 5, 123.4567, -12345.67, true));
        assert_approx_equal!(-24.6173153873664f64, payment(-0.001, 5, 123.4567, 0.0, false));
        assert_approx_equal!(-49.4074949697025f64, payment(-0.001, 5, 123.4567, 123.4567, true));
        assert_approx_equal!(-2436.99076664927f64, payment(-0.001, 5, 12345.67, -123.4567, false));
        assert_approx_equal!(-2464.19573447111f64, payment(-0.001, 5, 12345.67, 0.0, true));
        assert_approx_equal!(-4935.80874747328f64, payment(-0.001, 5, 12345.67, 12345.67, false));
        assert_approx_equal!(1241.42982900313f64, payment(-0.001, 10, -12345.67, -123.4567, true));
        assert_approx_equal!(1227.66305848239f64, payment(-0.001, 10, -12345.67, 1.234567, false));
        assert_approx_equal!(-12.358028028028f64, payment(-0.001, 10, -12345.67, 12345.67, true));
        assert_approx_equal!(12.4018839917413f64, payment(-0.001, 10, -123.4567, -1.234567, false));
        assert_approx_equal!(12.1660234668569f64, payment(-0.001, 10, -123.4567, 1.234567, true));
        assert_approx_equal!(1240.25552046374f64, payment(-0.001, 10, -1.234567, -12345.67, false));
        assert_approx_equal!(0.247039020371685f64, payment(-0.001, 10, -1.234567, -1.234567, true));
        assert_approx_equal!(-12.27854871039f64, payment(-0.001, 10, -1.234567, 123.4567, false));
        assert_approx_equal!(1241.37411587244f64, payment(-0.001, 10, 0.0, -12345.67, true));
        assert_approx_equal!(0f64, payment(-0.001, 10, 0.0, 0.0, false));
        assert_approx_equal!(-12.4137411587244f64, payment(-0.001, 10, 0.0, 123.4567, true));
        assert_approx_equal!(12.27854871039f64, payment(-0.001, 10, 1.234567, -123.4567, false));
        assert_approx_equal!(-0.122901608784441f64, payment(-0.001, 10, 1.234567, 0.0, true));
        assert_approx_equal!(-1240.25552046374f64, payment(-0.001, 10, 1.234567, 12345.67, false));
        assert_approx_equal!(0.12358028028028f64, payment(-0.001, 10, 123.4567, -123.4567, true));
        assert_approx_equal!(-12.4018839917413f64, payment(-0.001, 10, 123.4567, 1.234567, false));
        assert_approx_equal!(-1253.66427675088f64, payment(-0.001, 10, 123.4567, 12345.67, true));
        assert_approx_equal!(-1227.66305848239f64, payment(-0.001, 10, 12345.67, -1.234567, false));
        assert_approx_equal!(-1229.140225256f64, payment(-0.001, 10, 12345.67, 1.234567, true));
        assert_approx_equal!(493.682773192249f64, payment(-0.001, 50, -12345.67, -12345.67, false));
        assert_approx_equal!(240.93478780609f64, payment(-0.001, 50, -12345.67, -1.234567, true));
        assert_approx_equal!(238.138409380163f64, payment(-0.001, 50, -12345.67, 123.4567, false));
        assert_approx_equal!(255.676583695782f64, payment(-0.001, 50, -123.4567, -12345.67, true));
        assert_approx_equal!(2.40668551596125f64, payment(-0.001, 50, -123.4567, 0.0, false));
        assert_approx_equal!(-0.12358028028028f64, payment(-0.001, 50, -123.4567, 123.4567, true));
        assert_approx_equal!(2.55420907112086f64, payment(-0.001, 50, -1.234567, -123.4567, false));
        assert_approx_equal!(0.0240909461057182f64, payment(-0.001, 50, -1.234567, 0.0, true));
        assert_approx_equal!(-252.990154740965f64, payment(-0.001, 50, -1.234567, 12345.67, false));
        assert_approx_equal!(2.5326748908521f64, payment(-0.001, 50, 0.0, -123.4567, true));
        assert_approx_equal!(-0.0253014221596125f64, payment(-0.001, 50, 0.0, 1.234567, false));
        assert_approx_equal!(-253.26748908521f64, payment(-0.001, 50, 0.0, 12345.67, true));
        assert_approx_equal!(0.001234567f64, payment(-0.001, 50, 1.234567, -1.234567, false));
        assert_approx_equal!(-0.0494176950142391f64, payment(-0.001, 50, 1.234567, 1.234567, true));
        assert_approx_equal!(250.607536080163f64, payment(-0.001, 50, 123.4567, -12345.67, false));
        assert_approx_equal!(-2.3837678616633f64, payment(-0.001, 50, 123.4567, -1.234567, true));
        assert_approx_equal!(-4.93682773192249f64, payment(-0.001, 50, 123.4567, 123.4567, false));
        assert_approx_equal!(12.358028028028f64, payment(-0.001, 50, 12345.67, -12345.67, true));
        assert_approx_equal!(-240.668551596125f64, payment(-0.001, 50, 12345.67, 0.0, false));
        assert_approx_equal!(-243.442135948034f64, payment(-0.001, 50, 12345.67, 123.4567, true));
        assert_approx_equal!(44.0000905837691f64, payment(-0.001, 250, -12345.67, -123.4567, false));
        assert_approx_equal!(43.4856974635716f64, payment(-0.001, 250, -12345.67, 0.0, true));
        assert_approx_equal!(-12.34567f64, payment(-0.001, 250, -12345.67, 12345.67, false));
        assert_approx_equal!(0.993294229551712f64, payment(-0.001, 250, -123.4567, -123.4567, true));
        assert_approx_equal!(0.42884332948447f64, payment(-0.001, 250, -123.4567, 1.234567, false));
        assert_approx_equal!(-55.4088685169639f64, payment(-0.001, 250, -123.4567, 12345.67, true));
        assert_approx_equal!(0.00992300935322161f64, payment(-0.001, 250, -1.234567, -1.234567, false));
        assert_approx_equal!(-0.0012358028028028f64, payment(-0.001, 250, -1.234567, 1.234567, true));
        assert_approx_equal!(55.787881766108f64, payment(-0.001, 250, 0.0, -12345.67, false));
        assert_approx_equal!(0.00558437254915996f64, payment(-0.001, 250, 0.0, -1.234567, true));
        assert_approx_equal!(-0.55787881766108f64, payment(-0.001, 250, 0.0, 123.4567, false));
        assert_approx_equal!(55.8393769218533f64, payment(-0.001, 250, 1.234567, -12345.67, true));
        assert_approx_equal!(-0.0043442211766108f64, payment(-0.001, 250, 1.234567, 0.0, false));
        assert_approx_equal!(-0.562785824662353f64, payment(-0.001, 250, 1.234567, 123.4567, true));
        assert_approx_equal!(0.1234567f64, payment(-0.001, 250, 123.4567, -123.4567, false));
        assert_approx_equal!(-0.434856974635716f64, payment(-0.001, 250, 123.4567, 0.0, true));
        assert_approx_equal!(-56.2223038837691f64, payment(-0.001, 250, 123.4567, 12345.67, false));
        assert_approx_equal!(-42.9272602086556f64, payment(-0.001, 250, 12345.67, -123.4567, true));
        assert_approx_equal!(-43.4477905542846f64, payment(-0.001, 250, 12345.67, 1.234567, false));
        assert_approx_equal!(-99.3294229551712f64, payment(-0.001, 250, 12345.67, 12345.67, true));
        assert_approx_equal!(24691.34f64, payment(0.0, 1, -12345.67, -12345.67, false));
        assert_approx_equal!(12346.904567f64, payment(0.0, 1, -12345.67, -1.234567, true));
        assert_approx_equal!(12222.2133f64, payment(0.0, 1, -12345.67, 123.4567, false));
        assert_approx_equal!(12469.1267f64, payment(0.0, 1, -123.4567, -12345.67, true));
        assert_approx_equal!(123.4567f64, payment(0.0, 1, -123.4567, 0.0, false));
        assert_approx_equal!(0f64, payment(0.0, 1, -123.4567, 123.4567, true));
        assert_approx_equal!(124.691267f64, payment(0.0, 1, -1.234567, -123.4567, false));
        assert_approx_equal!(1.234567f64, payment(0.0, 1, -1.234567, 0.0, true));
        assert_approx_equal!(-12344.435433f64, payment(0.0, 1, -1.234567, 12345.67, false));
        assert_approx_equal!(123.4567f64, payment(0.0, 1, 0.0, -123.4567, true));
        assert_approx_equal!(-1.234567f64, payment(0.0, 1, 0.0, 1.234567, false));
        assert_approx_equal!(-12345.67f64, payment(0.0, 1, 0.0, 12345.67, true));
        assert_approx_equal!(0f64, payment(0.0, 1, 1.234567, -1.234567, false));
        assert_approx_equal!(-2.469134f64, payment(0.0, 1, 1.234567, 1.234567, true));
        assert_approx_equal!(12222.2133f64, payment(0.0, 1, 123.4567, -12345.67, false));
        assert_approx_equal!(-122.222133f64, payment(0.0, 1, 123.4567, -1.234567, true));
        assert_approx_equal!(-246.9134f64, payment(0.0, 1, 123.4567, 123.4567, false));
        assert_approx_equal!(0f64, payment(0.0, 1, 12345.67, -12345.67, true));
        assert_approx_equal!(-12345.67f64, payment(0.0, 1, 12345.67, 0.0, false));
        assert_approx_equal!(-12469.1267f64, payment(0.0, 1, 12345.67, 123.4567, true));
        assert_approx_equal!(6234.56335f64, payment(0.0, 2, -12345.67, -123.4567, false));
        assert_approx_equal!(6172.835f64, payment(0.0, 2, -12345.67, 0.0, true));
        assert_approx_equal!(0f64, payment(0.0, 2, -12345.67, 12345.67, false));
        assert_approx_equal!(123.4567f64, payment(0.0, 2, -123.4567, -123.4567, true));
        assert_approx_equal!(61.1110665f64, payment(0.0, 2, -123.4567, 1.234567, false));
        assert_approx_equal!(-6111.10665f64, payment(0.0, 2, -123.4567, 12345.67, true));
        assert_approx_equal!(1.234567f64, payment(0.0, 2, -1.234567, -1.234567, false));
        assert_approx_equal!(0f64, payment(0.0, 2, -1.234567, 1.234567, true));
        assert_approx_equal!(6172.835f64, payment(0.0, 2, 0.0, -12345.67, false));
        assert_approx_equal!(0.6172835f64, payment(0.0, 2, 0.0, -1.234567, true));
        assert_approx_equal!(-61.72835f64, payment(0.0, 2, 0.0, 123.4567, false));
        assert_approx_equal!(6172.2177165f64, payment(0.0, 2, 1.234567, -12345.67, true));
        assert_approx_equal!(-0.6172835f64, payment(0.0, 2, 1.234567, 0.0, false));
        assert_approx_equal!(-62.3456335f64, payment(0.0, 2, 1.234567, 123.4567, true));
        assert_approx_equal!(0f64, payment(0.0, 2, 123.4567, -123.4567, false));
        assert_approx_equal!(-61.72835f64, payment(0.0, 2, 123.4567, 0.0, true));
        assert_approx_equal!(-6234.56335f64, payment(0.0, 2, 123.4567, 12345.67, false));
        assert_approx_equal!(-6111.10665f64, payment(0.0, 2, 12345.67, -123.4567, true));
        assert_approx_equal!(-6173.4522835f64, payment(0.0, 2, 12345.67, 1.234567, false));
        assert_approx_equal!(-12345.67f64, payment(0.0, 2, 12345.67, 12345.67, true));
        assert_approx_equal!(2469.3809134f64, payment(0.0, 5, -12345.67, -1.234567, false));
        assert_approx_equal!(2468.8870866f64, payment(0.0, 5, -12345.67, 1.234567, true));
        assert_approx_equal!(2493.82534f64, payment(0.0, 5, -123.4567, -12345.67, false));
        assert_approx_equal!(24.9382534f64, payment(0.0, 5, -123.4567, -1.234567, true));
        assert_approx_equal!(0f64, payment(0.0, 5, -123.4567, 123.4567, false));
        assert_approx_equal!(2469.3809134f64, payment(0.0, 5, -1.234567, -12345.67, true));
        assert_approx_equal!(0.2469134f64, payment(0.0, 5, -1.234567, 0.0, false));
        assert_approx_equal!(-24.4444266f64, payment(0.0, 5, -1.234567, 123.4567, true));
        assert_approx_equal!(24.69134f64, payment(0.0, 5, 0.0, -123.4567, false));
        assert_approx_equal!(0f64, payment(0.0, 5, 0.0, 0.0, true));
        assert_approx_equal!(-2469.134f64, payment(0.0, 5, 0.0, 12345.67, false));
        assert_approx_equal!(24.4444266f64, payment(0.0, 5, 1.234567, -123.4567, true));
        assert_approx_equal!(-0.4938268f64, payment(0.0, 5, 1.234567, 1.234567, false));
        assert_approx_equal!(-2469.3809134f64, payment(0.0, 5, 1.234567, 12345.67, true));
        assert_approx_equal!(-24.4444266f64, payment(0.0, 5, 123.4567, -1.234567, false));
        assert_approx_equal!(-24.9382534f64, payment(0.0, 5, 123.4567, 1.234567, true));
        assert_approx_equal!(0f64, payment(0.0, 5, 12345.67, -12345.67, false));
        assert_approx_equal!(-2468.8870866f64, payment(0.0, 5, 12345.67, -1.234567, true));
        assert_approx_equal!(-2493.82534f64, payment(0.0, 5, 12345.67, 123.4567, false));
        assert_approx_equal!(2469.134f64, payment(0.0, 10, -12345.67, -12345.67, true));
        assert_approx_equal!(1234.567f64, payment(0.0, 10, -12345.67, 0.0, false));
        assert_approx_equal!(1222.22133f64, payment(0.0, 10, -12345.67, 123.4567, true));
        assert_approx_equal!(24.69134f64, payment(0.0, 10, -123.4567, -123.4567, false));
        assert_approx_equal!(12.34567f64, payment(0.0, 10, -123.4567, 0.0, true));
        assert_approx_equal!(-1222.22133f64, payment(0.0, 10, -123.4567, 12345.67, false));
        assert_approx_equal!(12.4691267f64, payment(0.0, 10, -1.234567, -123.4567, true));
        assert_approx_equal!(0f64, payment(0.0, 10, -1.234567, 1.234567, false));
        assert_approx_equal!(-1234.4435433f64, payment(0.0, 10, -1.234567, 12345.67, true));
        assert_approx_equal!(0.1234567f64, payment(0.0, 10, 0.0, -1.234567, false));
        assert_approx_equal!(-0.1234567f64, payment(0.0, 10, 0.0, 1.234567, true));
        assert_approx_equal!(1234.4435433f64, payment(0.0, 10, 1.234567, -12345.67, false));
        assert_approx_equal!(0f64, payment(0.0, 10, 1.234567, -1.234567, true));
        assert_approx_equal!(-12.4691267f64, payment(0.0, 10, 1.234567, 123.4567, false));
        assert_approx_equal!(1222.22133f64, payment(0.0, 10, 123.4567, -12345.67, true));
        assert_approx_equal!(-12.34567f64, payment(0.0, 10, 123.4567, 0.0, false));
        assert_approx_equal!(-24.69134f64, payment(0.0, 10, 123.4567, 123.4567, true));
        assert_approx_equal!(-1222.22133f64, payment(0.0, 10, 12345.67, -123.4567, false));
        assert_approx_equal!(-1234.567f64, payment(0.0, 10, 12345.67, 0.0, true));
        assert_approx_equal!(-2469.134f64, payment(0.0, 10, 12345.67, 12345.67, false));
        assert_approx_equal!(249.382534f64, payment(0.0, 50, -12345.67, -123.4567, true));
        assert_approx_equal!(246.88870866f64, payment(0.0, 50, -12345.67, 1.234567, false));
        assert_approx_equal!(0f64, payment(0.0, 50, -12345.67, 12345.67, true));
        assert_approx_equal!(2.49382534f64, payment(0.0, 50, -123.4567, -1.234567, false));
        assert_approx_equal!(2.44444266f64, payment(0.0, 50, -123.4567, 1.234567, true));
        assert_approx_equal!(246.93809134f64, payment(0.0, 50, -1.234567, -12345.67, false));
        assert_approx_equal!(0.04938268f64, payment(0.0, 50, -1.234567, -1.234567, true));
        assert_approx_equal!(-2.44444266f64, payment(0.0, 50, -1.234567, 123.4567, false));
        assert_approx_equal!(246.9134f64, payment(0.0, 50, 0.0, -12345.67, true));
        assert_approx_equal!(0f64, payment(0.0, 50, 0.0, 0.0, false));
        assert_approx_equal!(-2.469134f64, payment(0.0, 50, 0.0, 123.4567, true));
        assert_approx_equal!(2.44444266f64, payment(0.0, 50, 1.234567, -123.4567, false));
        assert_approx_equal!(-0.02469134f64, payment(0.0, 50, 1.234567, 0.0, true));
        assert_approx_equal!(-246.93809134f64, payment(0.0, 50, 1.234567, 12345.67, false));
        assert_approx_equal!(0f64, payment(0.0, 50, 123.4567, -123.4567, true));
        assert_approx_equal!(-2.49382534f64, payment(0.0, 50, 123.4567, 1.234567, false));
        assert_approx_equal!(-249.382534f64, payment(0.0, 50, 123.4567, 12345.67, true));
        assert_approx_equal!(-246.88870866f64, payment(0.0, 50, 12345.67, -1.234567, false));
        assert_approx_equal!(-246.93809134f64, payment(0.0, 50, 12345.67, 1.234567, true));
        assert_approx_equal!(98.76536f64, payment(0.0, 250, -12345.67, -12345.67, false));
        assert_approx_equal!(49.387618268f64, payment(0.0, 250, -12345.67, -1.234567, true));
        assert_approx_equal!(48.8888532f64, payment(0.0, 250, -12345.67, 123.4567, false));
        assert_approx_equal!(49.8765068f64, payment(0.0, 250, -123.4567, -12345.67, true));
        assert_approx_equal!(0.4938268f64, payment(0.0, 250, -123.4567, 0.0, false));
        assert_approx_equal!(0f64, payment(0.0, 250, -123.4567, 123.4567, true));
        assert_approx_equal!(0.498765068f64, payment(0.0, 250, -1.234567, -123.4567, false));
        assert_approx_equal!(0.004938268f64, payment(0.0, 250, -1.234567, 0.0, true));
        assert_approx_equal!(-49.377741732f64, payment(0.0, 250, -1.234567, 12345.67, false));
        assert_approx_equal!(0.4938268f64, payment(0.0, 250, 0.0, -123.4567, true));
        assert_approx_equal!(-0.004938268f64, payment(0.0, 250, 0.0, 1.234567, false));
        assert_approx_equal!(-49.38268f64, payment(0.0, 250, 0.0, 12345.67, true));
        assert_approx_equal!(0f64, payment(0.0, 250, 1.234567, -1.234567, false));
        assert_approx_equal!(-0.009876536f64, payment(0.0, 250, 1.234567, 1.234567, true));
        assert_approx_equal!(48.8888532f64, payment(0.0, 250, 123.4567, -12345.67, false));
        assert_approx_equal!(-0.488888532f64, payment(0.0, 250, 123.4567, -1.234567, true));
        assert_approx_equal!(-0.9876536f64, payment(0.0, 250, 123.4567, 123.4567, false));
        assert_approx_equal!(0f64, payment(0.0, 250, 12345.67, -12345.67, true));
        assert_approx_equal!(-49.38268f64, payment(0.0, 250, 12345.67, 0.0, false));
        assert_approx_equal!(-49.8765068f64, payment(0.0, 250, 12345.67, 123.4567, true));
        assert_approx_equal!(12468.8434011773f64, payment(0.0023, 1, -12345.67, -123.4567, true));
        assert_approx_equal!(12372.830474f64, payment(0.0023, 1, -12345.67, 1.234567, false));
        assert_approx_equal!(28.3298822707772f64, payment(0.0023, 1, -12345.67, 12345.67, true));
        assert_approx_equal!(124.97521741f64, payment(0.0023, 1, -123.4567, -1.234567, false));
        assert_approx_equal!(122.224965988227f64, payment(0.0023, 1, -123.4567, 1.234567, true));
        assert_approx_equal!(12346.9074065041f64, payment(0.0023, 1, -1.234567, -12345.67, false));
        assert_approx_equal!(2.46630101177292f64, payment(0.0023, 1, -1.234567, -1.234567, true));
        assert_approx_equal!(-122.2192934959f64, payment(0.0023, 1, -1.234567, 123.4567, false));
        assert_approx_equal!(12317.3401177292f64, payment(0.0023, 1, 0.0, -12345.67, true));
        assert_approx_equal!(0f64, payment(0.0023, 1, 0.0, 0.0, false));
        assert_approx_equal!(-123.173401177292f64, payment(0.0023, 1, 0.0, 123.4567, true));
        assert_approx_equal!(122.2192934959f64, payment(0.0023, 1, 1.234567, -123.4567, false));
        assert_approx_equal!(-1.234567f64, payment(0.0023, 1, 1.234567, 0.0, true));
        assert_approx_equal!(-12346.9074065041f64, payment(0.0023, 1, 1.234567, 12345.67, false));
        assert_approx_equal!(-0.283298822707772f64, payment(0.0023, 1, 123.4567, -123.4567, true));
        assert_approx_equal!(-124.97521741f64, payment(0.0023, 1, 123.4567, 1.234567, false));
        assert_approx_equal!(-12440.7968177292f64, payment(0.0023, 1, 123.4567, 12345.67, true));
        assert_approx_equal!(-12372.830474f64, payment(0.0023, 1, 12345.67, -1.234567, false));
        assert_approx_equal!(-12346.9017340118f64, payment(0.0023, 1, 12345.67, 1.234567, true));
        assert_approx_equal!(12359.8838288939f64, payment(0.0023, 2, -12345.67, -12345.67, false));
        assert_approx_equal!(6180.54076562542f64, payment(0.0023, 2, -12345.67, -1.234567, true));
        assert_approx_equal!(6132.48199100749f64, payment(0.0023, 2, -12345.67, 123.4567, false));
        assert_approx_equal!(6213.39497984279f64, payment(0.0023, 2, -123.4567, -12345.67, true));
        assert_approx_equal!(61.9413943494696f64, payment(0.0023, 2, -123.4567, 0.0, false));
        assert_approx_equal!(0.283298822707772f64, payment(0.0023, 2, -123.4567, 123.4567, true));
        assert_approx_equal!(62.2768578829643f64, payment(0.0023, 2, -1.234567, -123.4567, false));
        assert_approx_equal!(0.617992560605304f64, payment(0.0023, 2, -1.234567, 0.0, true));
        assert_approx_equal!(-6165.12498000346f64, payment(0.0023, 2, -1.234567, 12345.67, false));
        assert_approx_equal!(61.5159572378226f64, payment(0.0023, 2, 0.0, -123.4567, true));
        assert_approx_equal!(-0.616574439394696f64, payment(0.0023, 2, 0.0, 1.234567, false));
        assert_approx_equal!(-6151.59572378226f64, payment(0.0023, 2, 0.0, 12345.67, true));
        assert_approx_equal!(-0.0028395041f64, payment(0.0023, 2, 1.234567, -1.234567, false));
        assert_approx_equal!(-1.23315213298353f64, payment(0.0023, 2, 1.234567, 1.234567, true));
        assert_approx_equal!(6103.80299959749f64, payment(0.0023, 2, 123.4567, -12345.67, false));
        assert_approx_equal!(-61.1840964881522f64, payment(0.0023, 2, 123.4567, -1.234567, true));
        assert_approx_equal!(-123.598838288939f64, payment(0.0023, 2, 123.4567, 123.4567, false));
        assert_approx_equal!(-28.3298822707772f64, payment(0.0023, 2, 12345.67, -12345.67, true));
        assert_approx_equal!(-6194.13943494696f64, payment(0.0023, 2, 12345.67, 0.0, false));
        assert_approx_equal!(-6241.44156329086f64, payment(0.0023, 2, 12345.67, 123.4567, true));
        assert_approx_equal!(2510.7751387519f64, payment(0.0023, 5, -12345.67, -123.4567, false));
        assert_approx_equal!(2480.49198641332f64, payment(0.0023, 5, -12345.67, 0.0, true));
        assert_approx_equal!(28.395041f64, payment(0.0023, 5, -12345.67, 12345.67, false));
        assert_approx_equal!(49.3265409055587f64, payment(0.0023, 5, -123.4567, -123.4567, true));
        assert_approx_equal!(24.6161909721225f64, payment(0.0023, 5, -123.4567, 1.234567, false));
        assert_approx_equal!(-2427.35718427841f64, payment(0.0023, 5, -123.4567, 12345.67, true));
        assert_approx_equal!(0.494399919496415f64, payment(0.0023, 5, -1.234567, -1.234567, false));
        assert_approx_equal!(0.00283298822707772f64, payment(0.0023, 5, -1.234567, 1.234567, true));
        assert_approx_equal!(2457.80207698207f64, payment(0.0023, 5, 0.0, -12345.67, false));
        assert_approx_equal!(0.245216210414255f64, payment(0.0023, 5, 0.0, -1.234567, true));
        assert_approx_equal!(-24.5780207698207f64, payment(0.0023, 5, 0.0, 123.4567, false));
        assert_approx_equal!(2451.91405494391f64, payment(0.0023, 5, 1.234567, -12345.67, true));
        assert_approx_equal!(-0.248619711798207f64, payment(0.0023, 5, 1.234567, 0.0, false));
        assert_approx_equal!(-24.7696702400668f64, payment(0.0023, 5, 1.234567, 123.4567, true));
        assert_approx_equal!(-0.28395041f64, payment(0.0023, 5, 123.4567, -123.4567, false));
        assert_approx_equal!(-24.8049198641332f64, payment(0.0023, 5, 123.4567, 0.0, true));
        assert_approx_equal!(-2482.6640481619f64, payment(0.0023, 5, 123.4567, 12345.67, false));
        assert_approx_equal!(-2455.9703653719f64, payment(0.0023, 5, 12345.67, -123.4567, true));
        assert_approx_equal!(-2486.44289818977f64, payment(0.0023, 5, 12345.67, 1.234567, false));
        assert_approx_equal!(-4932.65409055587f64, payment(0.0023, 5, 12345.67, 12345.67, true));
        assert_approx_equal!(1250.36027410036f64, payment(0.0023, 10, -12345.67, -1.234567, false));
        assert_approx_equal!(1247.24723684586f64, payment(0.0023, 10, -12345.67, 1.234567, true));
        assert_approx_equal!(1234.34542969344f64, payment(0.0023, 10, -123.4567, -12345.67, false));
        assert_approx_equal!(12.5955953335671f64, payment(0.0023, 10, -123.4567, -1.234567, true));
        assert_approx_equal!(0.28395041f64, payment(0.0023, 10, -123.4567, 123.4567, false));
        assert_approx_equal!(1219.16399541501f64, payment(0.0023, 10, -1.234567, -12345.67, true));
        assert_approx_equal!(0.125023808979548f64, payment(0.0023, 10, -1.234567, 0.0, false));
        assert_approx_equal!(-12.0656556709321f64, payment(0.0023, 10, -1.234567, 123.4567, true));
        assert_approx_equal!(12.2184304879548f64, payment(0.0023, 10, 0.0, -123.4567, false));
        assert_approx_equal!(0f64, payment(0.0023, 10, 0.0, 0.0, true));
        assert_approx_equal!(-1221.84304879548f64, payment(0.0023, 10, 0.0, 12345.67, false));
        assert_approx_equal!(12.0656556709321f64, payment(0.0023, 10, 1.234567, -123.4567, true));
        assert_approx_equal!(-0.247208113859096f64, payment(0.0023, 10, 1.234567, 1.234567, false));
        assert_approx_equal!(-1219.16399541501f64, payment(0.0023, 10, 1.234567, 12345.67, true));
        assert_approx_equal!(-12.3801965930753f64, payment(0.0023, 10, 123.4567, -1.234567, false));
        assert_approx_equal!(-12.5955953335671f64, payment(0.0023, 10, 123.4567, 1.234567, true));
        assert_approx_equal!(-28.395041f64, payment(0.0023, 10, 12345.67, -12345.67, false));
        assert_approx_equal!(-1247.24723684586f64, payment(0.0023, 10, 12345.67, -1.234567, true));
        assert_approx_equal!(-1262.45652028344f64, payment(0.0023, 10, 12345.67, 123.4567, false));
        assert_approx_equal!(493.80223210177f64, payment(0.0023, 50, -12345.67, -12345.67, true));
        assert_approx_equal!(261.666509117802f64, payment(0.0023, 50, -12345.67, 0.0, false));
        assert_approx_equal!(258.738695437118f64, payment(0.0023, 50, -12345.67, 123.4567, true));
        assert_approx_equal!(4.94937977235604f64, payment(0.0023, 50, -123.4567, -123.4567, false));
        assert_approx_equal!(2.61066057186273f64, payment(0.0023, 50, -123.4567, 0.0, true));
        assert_approx_equal!(-230.654803026624f64, payment(0.0023, 50, -123.4567, 12345.67, false));
        assert_approx_equal!(2.35346835487359f64, payment(0.0023, 50, -1.234567, -123.4567, true));
        assert_approx_equal!(0.0028395041f64, payment(0.0023, 50, -1.234567, 1.234567, false));
        assert_approx_equal!(-232.710068309778f64, payment(0.0023, 50, -1.234567, 12345.67, true));
        assert_approx_equal!(0.0233271468117802f64, payment(0.0023, 50, 0.0, -1.234567, false));
        assert_approx_equal!(-0.0232736174915496f64, payment(0.0023, 50, 0.0, 1.234567, true));
        assert_approx_equal!(233.24530146689f64, payment(0.0023, 50, 1.234567, -12345.67, false));
        assert_approx_equal!(-0.00283298822707772f64, payment(0.0023, 50, 1.234567, -1.234567, true));
        assert_approx_equal!(-2.3588813320898f64, payment(0.0023, 50, 1.234567, 123.4567, false));
        assert_approx_equal!(230.125514343633f64, payment(0.0023, 50, 123.4567, -12345.67, true));
        assert_approx_equal!(-2.61666509117802f64, payment(0.0023, 50, 123.4567, 0.0, false));
        assert_approx_equal!(-4.9380223210177f64, payment(0.0023, 50, 123.4567, 123.4567, true));
        assert_approx_equal!(-259.333794436624f64, payment(0.0023, 50, 12345.67, -123.4567, false));
        assert_approx_equal!(-261.066057186273f64, payment(0.0023, 50, 12345.67, 0.0, true));
        assert_approx_equal!(-494.937977235604f64, payment(0.0023, 50, 12345.67, 12345.67, false));
        assert_approx_equal!(65.204553816131f64, payment(0.0023, 250, -12345.67, -123.4567, true));
        assert_approx_equal!(64.984929457009f64, payment(0.0023, 250, -12345.67, 1.234567, false));
        assert_approx_equal!(28.3298822707772f64, payment(0.0023, 250, -12345.67, 12345.67, true));
        assert_approx_equal!(0.653545242899081f64, payment(0.0023, 250, -123.4567, -1.234567, false));
        assert_approx_equal!(0.644743623003814f64, payment(0.0023, 250, -123.4567, 1.234567, true));
        assert_approx_equal!(36.6000466706714f64, payment(0.0023, 250, -1.234567, -12345.67, false));
        assert_approx_equal!(0.0101349033845735f64, payment(0.0023, 250, -1.234567, -1.234567, true));
        assert_approx_equal!(-0.359436619236723f64, payment(0.0023, 250, -1.234567, 123.4567, false));
        assert_approx_equal!(36.509575787479f64, payment(0.0023, 250, 0.0, -12345.67, true));
        assert_approx_equal!(0f64, payment(0.0023, 250, 0.0, 0.0, false));
        assert_approx_equal!(-0.36509575787479f64, payment(0.0023, 250, 0.0, 123.4567, true));
        assert_approx_equal!(0.359436619236723f64, payment(0.0023, 250, 1.234567, -123.4567, false));
        assert_approx_equal!(-0.00648394580582562f64, payment(0.0023, 250, 1.234567, 0.0, true));
        assert_approx_equal!(-36.6000466706714f64, payment(0.0023, 250, 1.234567, 12345.67, false));
        assert_approx_equal!(-0.283298822707772f64, payment(0.0023, 250, 123.4567, -123.4567, true));
        assert_approx_equal!(-0.653545242899081f64, payment(0.0023, 250, 123.4567, 1.234567, false));
        assert_approx_equal!(-37.1579703680616f64, payment(0.0023, 250, 123.4567, 12345.67, true));
        assert_approx_equal!(-64.984929457009f64, payment(0.0023, 250, 12345.67, -1.234567, false));
        assert_approx_equal!(-64.843109015835f64, payment(0.0023, 250, 12345.67, 1.234567, true));
        assert_approx_equal!(24413.7736168133f64, payment(0.023, 1, -12345.67, -12345.67, true));
        assert_approx_equal!(12629.62041f64, payment(0.023, 1, -12345.67, 0.0, false));
        assert_approx_equal!(12224.9889638319f64, payment(0.023, 1, -12345.67, 123.4567, true));
        assert_approx_equal!(249.7529041f64, payment(0.023, 1, -123.4567, -123.4567, false));
        assert_approx_equal!(123.4567f64, payment(0.023, 1, -123.4567, 0.0, true));
        assert_approx_equal!(-12219.3737959f64, payment(0.023, 1, -123.4567, 12345.67, false));
        assert_approx_equal!(121.915603168133f64, payment(0.023, 1, -1.234567, -123.4567, true));
        assert_approx_equal!(0.028395041f64, payment(0.023, 1, -1.234567, 1.234567, false));
        assert_approx_equal!(-12066.8690498133f64, payment(0.023, 1, -1.234567, 12345.67, true));
        assert_approx_equal!(1.234567f64, payment(0.023, 1, 0.0, -1.234567, false));
        assert_approx_equal!(-1.20681036168133f64, payment(0.023, 1, 0.0, 1.234567, true));
        assert_approx_equal!(12344.407037959f64, payment(0.023, 1, 1.234567, -12345.67, false));
        assert_approx_equal!(-0.0277566383186706f64, payment(0.023, 1, 1.234567, -1.234567, true));
        assert_approx_equal!(-124.719662041f64, payment(0.023, 1, 1.234567, 123.4567, false));
        assert_approx_equal!(11944.6469168133f64, payment(0.023, 1, 123.4567, -12345.67, true));
        assert_approx_equal!(-126.2962041f64, payment(0.023, 1, 123.4567, 0.0, false));
        assert_approx_equal!(-244.137736168133f64, payment(0.023, 1, 123.4567, 123.4567, true));
        assert_approx_equal!(-12506.16371f64, payment(0.023, 1, 12345.67, -123.4567, false));
        assert_approx_equal!(-12345.67f64, payment(0.023, 1, 12345.67, 0.0, true));
        assert_approx_equal!(-24975.29041f64, payment(0.023, 1, 12345.67, 12345.67, false));
        assert_approx_equal!(6302.67001787847f64, payment(0.023, 2, -12345.67, -123.4567, true));
        assert_approx_equal!(6385.99461810677f64, payment(0.023, 2, -12345.67, 1.234567, false));
        assert_approx_equal!(277.566383186706f64, payment(0.023, 2, -12345.67, 12345.67, true));
        assert_approx_equal!(64.4763142828967f64, payment(0.023, 2, -123.4567, -1.234567, false));
        assert_approx_equal!(61.8336103501328f64, payment(0.023, 2, -123.4567, 1.234567, true));
        assert_approx_equal!(6103.29313404248f64, payment(0.023, 2, -1.234567, -12345.67, false));
        assert_approx_equal!(1.2208464669705f64, payment(0.023, 2, -1.234567, -1.234567, true));
        assert_approx_equal!(-60.3878842471859f64, payment(0.023, 2, -1.234567, 123.4567, false));
        assert_approx_equal!(5965.44914325917f64, payment(0.023, 2, 0.0, -12345.67, true));
        assert_approx_equal!(0f64, payment(0.023, 2, 0.0, 0.0, false));
        assert_approx_equal!(-59.6544914325917f64, payment(0.023, 2, 0.0, 123.4567, true));
        assert_approx_equal!(60.3878842471859f64, payment(0.023, 2, 1.234567, -123.4567, false));
        assert_approx_equal!(-0.624301552644587f64, payment(0.023, 2, 1.234567, 0.0, true));
        assert_approx_equal!(-6103.29313404248f64, payment(0.023, 2, 1.234567, 12345.67, false));
        assert_approx_equal!(-2.77566383186706f64, payment(0.023, 2, 123.4567, -123.4567, true));
        assert_approx_equal!(-64.4763142828967f64, payment(0.023, 2, 123.4567, 1.234567, false));
        assert_approx_equal!(-6027.87929852363f64, payment(0.023, 2, 123.4567, 12345.67, true));
        assert_approx_equal!(-6385.99461810677f64, payment(0.023, 2, 12345.67, -1.234567, false));
        assert_approx_equal!(-6243.6120713602f64, payment(0.023, 2, 12345.67, 1.234567, true));
        assert_approx_equal!(5000.22243424033f64, payment(0.023, 5, -12345.67, -12345.67, false));
        assert_approx_equal!(2582.91518643341f64, payment(0.023, 5, -12345.67, -1.234567, true));
        assert_approx_equal!(2618.50506199896f64, payment(0.023, 5, -12345.67, 123.4567, false));
        assert_approx_equal!(2330.9451381636f64, payment(0.023, 5, -123.4567, -12345.67, true));
        assert_approx_equal!(26.4208642212016f64, payment(0.023, 5, -123.4567, 0.0, false));
        assert_approx_equal!(2.77566383186706f64, payment(0.023, 5, -123.4567, 123.4567, true));
        assert_approx_equal!(23.8455687634136f64, payment(0.023, 5, -1.234567, -123.4567, false));
        assert_approx_equal!(0.258268467460426f64, payment(0.023, 5, -1.234567, 0.0, true));
        assert_approx_equal!(-2357.87180347795f64, payment(0.023, 5, -1.234567, 12345.67, false));
        assert_approx_equal!(23.0511829141756f64, payment(0.023, 5, 0.0, -123.4567, true));
        assert_approx_equal!(-0.235813601212016f64, payment(0.023, 5, 0.0, 1.234567, false));
        assert_approx_equal!(-2305.11829141756f64, payment(0.023, 5, 0.0, 12345.67, true));
        assert_approx_equal!(-0.028395041f64, payment(0.023, 5, 1.234567, -1.234567, false));
        assert_approx_equal!(-0.488780296602182f64, payment(0.023, 5, 1.234567, 1.234567, true));
        assert_approx_equal!(2331.71514789896f64, payment(0.023, 5, 123.4567, -12345.67, false));
        assert_approx_equal!(-25.5963349169009f64, payment(0.023, 5, 123.4567, -1.234567, true));
        assert_approx_equal!(-50.0022243424032f64, payment(0.023, 5, 123.4567, 123.4567, false));
        assert_approx_equal!(-277.566383186706f64, payment(0.023, 5, 12345.67, -12345.67, true));
        assert_approx_equal!(-2642.08642212016f64, payment(0.023, 5, 12345.67, 0.0, false));
        assert_approx_equal!(-2605.73585751844f64, payment(0.023, 5, 12345.67, 123.4567, true));
        assert_approx_equal!(1407.18314215102f64, payment(0.023, 10, -12345.67, -123.4567, false));
        assert_approx_equal!(1364.67451221027f64, payment(0.023, 10, -12345.67, 0.0, true));
        assert_approx_equal!(283.95041f64, payment(0.023, 10, -12345.67, 12345.67, false));
        assert_approx_equal!(24.5178264123383f64, payment(0.023, 10, -123.4567, -123.4567, true));
        assert_approx_equal!(13.8494090983119f64, payment(0.023, 10, -123.4567, 1.234567, false));
        assert_approx_equal!(-1073.46138390146f64, payment(0.023, 10, -123.4567, 12345.67, true));
        assert_approx_equal!(0.250817364198221f64, payment(0.023, 10, -1.234567, -1.234567, false));
        assert_approx_equal!(0.0277566383186706f64, payment(0.023, 10, -1.234567, 1.234567, true));
        assert_approx_equal!(1112.1116159911f64, payment(0.023, 10, 0.0, -12345.67, false));
        assert_approx_equal!(0.108710812902356f64, payment(0.023, 10, 0.0, -1.234567, true));
        assert_approx_equal!(-11.121116159911f64, payment(0.023, 10, 0.0, 123.4567, false));
        assert_approx_equal!(1086.97166157234f64, payment(0.023, 10, 1.234567, -12345.67, true));
        assert_approx_equal!(-0.13960620259911f64, payment(0.023, 10, 1.234567, 0.0, false));
        assert_approx_equal!(-11.0075487414567f64, payment(0.023, 10, 1.234567, 123.4567, true));
        assert_approx_equal!(-2.8395041f64, payment(0.023, 10, 123.4567, -123.4567, false));
        assert_approx_equal!(-13.6467451221027f64, payment(0.023, 10, 123.4567, 0.0, true));
        assert_approx_equal!(-1126.07223625102f64, payment(0.023, 10, 123.4567, 12345.67, false));
        assert_approx_equal!(-1353.80343092003f64, payment(0.023, 10, 12345.67, -123.4567, true));
        assert_approx_equal!(-1396.1732371527f64, payment(0.023, 10, 12345.67, 1.234567, false));
        assert_approx_equal!(-2451.78264123383f64, payment(0.023, 10, 12345.67, 12345.67, true));
        assert_approx_equal!(418.072090469851f64, payment(0.023, 50, -12345.67, -1.234567, false));
        assert_approx_equal!(408.646401579592f64, payment(0.023, 50, -12345.67, 1.234567, true));
        assert_approx_equal!(138.288856439316f64, payment(0.023, 50, -123.4567, -12345.67, false));
        assert_approx_equal!(4.09970442169419f64, payment(0.023, 50, -123.4567, -1.234567, true));
        assert_approx_equal!(2.8395041f64, payment(0.023, 50, -123.4567, 123.4567, false));
        assert_approx_equal!(131.133993656746f64, payment(0.023, 50, -1.234567, -12345.67, true));
        assert_approx_equal!(0.0418058679642887f64, payment(0.023, 50, -1.234567, 0.0, false));
        assert_approx_equal!(-1.27006532596733f64, payment(0.023, 50, -1.234567, 123.4567, true));
        assert_approx_equal!(1.34108269642887f64, payment(0.023, 50, 0.0, -123.4567, false));
        assert_approx_equal!(0f64, payment(0.023, 50, 0.0, 0.0, true));
        assert_approx_equal!(-134.108269642887f64, payment(0.023, 50, 0.0, 12345.67, false));
        assert_approx_equal!(1.27006532596733f64, payment(0.023, 50, 1.234567, -123.4567, true));
        assert_approx_equal!(-0.0552166949285774f64, payment(0.023, 50, 1.234567, 1.234567, false));
        assert_approx_equal!(-131.133993656746f64, payment(0.023, 50, 1.234567, 12345.67, true));
        assert_approx_equal!(-4.16717596946458f64, payment(0.023, 50, 123.4567, -1.234567, false));
        assert_approx_equal!(-4.09970442169419f64, payment(0.023, 50, 123.4567, 1.234567, true));
        assert_approx_equal!(-283.95041f64, payment(0.023, 50, 12345.67, -12345.67, false));
        assert_approx_equal!(-408.646401579592f64, payment(0.023, 50, 12345.67, -1.234567, true));
        assert_approx_equal!(-419.399762339316f64, payment(0.023, 50, 12345.67, 123.4567, false));
        assert_approx_equal!(279.458579666982f64, payment(0.023, 250, -12345.67, -12345.67, true));
        assert_approx_equal!(284.918268499661f64, payment(0.023, 250, -12345.67, 0.0, false));
        assert_approx_equal!(278.503020444443f64, payment(0.023, 250, -12345.67, 123.4567, true));
        assert_approx_equal!(2.85886126999323f64, payment(0.023, 250, -123.4567, -123.4567, false));
        assert_approx_equal!(2.78512481426844f64, payment(0.023, 250, -123.4567, 0.0, true));
        assert_approx_equal!(1.8813241853352f64, payment(0.023, 250, -123.4567, 12345.67, false));
        assert_approx_equal!(0.0373122305440668f64, payment(0.023, 250, -1.234567, -123.4567, true));
        assert_approx_equal!(0.028395041f64, payment(0.023, 250, -1.234567, 1.234567, false));
        assert_approx_equal!(-0.918246991995591f64, payment(0.023, 250, -1.234567, 12345.67, true));
        assert_approx_equal!(9.67858499661454E-05f64, payment(0.023, 250, 0.0, -1.234567, false));
        assert_approx_equal!(-9.46098240138273E-05f64, payment(0.023, 250, 0.0, 1.234567, true));
        assert_approx_equal!(0.939366672811489f64, payment(0.023, 250, 1.234567, -12345.67, false));
        assert_approx_equal!(-0.0277566383186706f64, payment(0.023, 250, 1.234567, -1.234567, true));
        assert_approx_equal!(-0.0381704118465804f64, payment(0.023, 250, 1.234567, 123.4567, false));
        assert_approx_equal!(-1.83902657413021f64, payment(0.023, 250, 123.4567, -12345.67, true));
        assert_approx_equal!(-2.84918268499661f64, payment(0.023, 250, 123.4567, 0.0, false));
        assert_approx_equal!(-2.79458579666982f64, payment(0.023, 250, 123.4567, 123.4567, true));
        assert_approx_equal!(-284.908589914665f64, payment(0.023, 250, 12345.67, -123.4567, false));
        assert_approx_equal!(-278.512481426844f64, payment(0.023, 250, 12345.67, 0.0, true));
        assert_approx_equal!(-285.886126999323f64, payment(0.023, 250, 12345.67, 12345.67, false));
        assert_approx_equal!(15186.408667f64, payment(0.23, 1, -12345.67, -1.234567, false));
        assert_approx_equal!(12344.6662869919f64, payment(0.23, 1, -12345.67, 1.234567, true));
        assert_approx_equal!(12497.521741f64, payment(0.23, 1, -123.4567, -12345.67, false));
        assert_approx_equal!(124.46041300813f64, payment(0.23, 1, -123.4567, -1.234567, true));
        assert_approx_equal!(28.395041f64, payment(0.23, 1, -123.4567, 123.4567, false));
        assert_approx_equal!(10038.3646483008f64, payment(0.23, 1, -1.234567, -12345.67, true));
        assert_approx_equal!(1.51851741f64, payment(0.23, 1, -1.234567, 0.0, false));
        assert_approx_equal!(-99.1367338130081f64, payment(0.23, 1, -1.234567, 123.4567, true));
        assert_approx_equal!(123.4567f64, payment(0.23, 1, 0.0, -123.4567, false));
        assert_approx_equal!(0f64, payment(0.23, 1, 0.0, 0.0, true));
        assert_approx_equal!(-12345.67f64, payment(0.23, 1, 0.0, 12345.67, false));
        assert_approx_equal!(99.1367338130081f64, payment(0.23, 1, 1.234567, -123.4567, true));
        assert_approx_equal!(-2.75308441f64, payment(0.23, 1, 1.234567, 1.234567, false));
        assert_approx_equal!(-10038.3646483008f64, payment(0.23, 1, 1.234567, 12345.67, true));
        assert_approx_equal!(-150.617174f64, payment(0.23, 1, 123.4567, -1.234567, false));
        assert_approx_equal!(-124.46041300813f64, payment(0.23, 1, 123.4567, 1.234567, true));
        assert_approx_equal!(-2839.5041f64, payment(0.23, 1, 12345.67, -12345.67, false));
        assert_approx_equal!(-12344.6662869919f64, payment(0.23, 1, 12345.67, -1.234567, true));
        assert_approx_equal!(-15308.6308f64, payment(0.23, 1, 12345.67, 123.4567, false));
        assert_approx_equal!(11310.4503055161f64, payment(0.23, 2, -12345.67, -12345.67, true));
        assert_approx_equal!(8375.67898789238f64, payment(0.23, 2, -12345.67, 0.0, false));
        assert_approx_equal!(6764.48556017354f64, payment(0.23, 2, -12345.67, 123.4567, true));
        assert_approx_equal!(139.118538757848f64, payment(0.23, 2, -123.4567, -123.4567, false));
        assert_approx_equal!(68.0949511210762f64, payment(0.23, 2, -123.4567, 0.0, true));
        assert_approx_equal!(-5452.41809801345f64, payment(0.23, 2, -123.4567, 12345.67, false));
        assert_approx_equal!(45.6905014452951f64, payment(0.23, 2, -1.234567, -123.4567, true));
        assert_approx_equal!(0.28395041f64, payment(0.23, 2, -1.234567, 1.234567, false));
        assert_approx_equal!(-4500.27424389723f64, payment(0.23, 2, -1.234567, 12345.67, true));
        assert_approx_equal!(0.553617488789238f64, payment(0.23, 2, 0.0, -1.234567, false));
        assert_approx_equal!(-0.450095519340844f64, payment(0.23, 2, 0.0, 1.234567, true));
        assert_approx_equal!(5535.33731999359f64, payment(0.23, 2, 1.234567, -12345.67, false));
        assert_approx_equal!(-0.230853991869919f64, payment(0.23, 2, 1.234567, -1.234567, true));
        assert_approx_equal!(-56.199316777713f64, payment(0.23, 2, 1.234567, 123.4567, false));
        assert_approx_equal!(4432.86024228736f64, payment(0.23, 2, 123.4567, -12345.67, true));
        assert_approx_equal!(-83.7567898789238f64, payment(0.23, 2, 123.4567, 0.0, false));
        assert_approx_equal!(-113.104503055161f64, payment(0.23, 2, 123.4567, 123.4567, true));
        assert_approx_equal!(-8320.31723901346f64, payment(0.23, 2, 12345.67, -123.4567, false));
        assert_approx_equal!(-6809.49511210762f64, payment(0.23, 2, 12345.67, 0.0, true));
        assert_approx_equal!(-13911.8538757848f64, payment(0.23, 2, 12345.67, 12345.67, false));
        assert_approx_equal!(3592.96564272554f64, payment(0.23, 5, -12345.67, -123.4567, true));
        assert_approx_equal!(4403.54930414689f64, payment(0.23, 5, -12345.67, 1.234567, false));
        assert_approx_equal!(2308.53991869919f64, payment(0.23, 5, -12345.67, 12345.67, true));
        assert_approx_equal!(44.1934774055241f64, payment(0.23, 5, -123.4567, -1.234567, false));
        assert_approx_equal!(35.6753146997254f64, payment(0.23, 5, -123.4567, 1.234567, true));
        assert_approx_equal!(1564.64199488175f64, payment(0.23, 5, -1.234567, -12345.67, false));
        assert_approx_equal!(0.485195719399889f64, payment(0.23, 5, -1.234567, -1.234567, true));
        assert_approx_equal!(-15.2016456706623f64, payment(0.23, 5, -1.234567, 123.4567, false));
        assert_approx_equal!(1271.70863764985f64, payment(0.23, 5, 0.0, -12345.67, true));
        assert_approx_equal!(0f64, payment(0.23, 5, 0.0, 0.0, false));
        assert_approx_equal!(-12.7170863764985f64, payment(0.23, 5, 0.0, 123.4567, true));
        assert_approx_equal!(15.2016456706623f64, payment(0.23, 5, 1.234567, -123.4567, false));
        assert_approx_equal!(-0.358024855634904f64, payment(0.23, 5, 1.234567, 0.0, true));
        assert_approx_equal!(-1564.64199488175f64, payment(0.23, 5, 1.234567, 12345.67, false));
        assert_approx_equal!(-23.0853991869919f64, payment(0.23, 5, 123.4567, -123.4567, true));
        assert_approx_equal!(-44.1934774055241f64, payment(0.23, 5, 123.4567, 1.234567, false));
        assert_approx_equal!(-1307.51112321334f64, payment(0.23, 5, 123.4567, 12345.67, true));
        assert_approx_equal!(-4403.54930414689f64, payment(0.23, 5, 12345.67, -1.234567, false));
        assert_approx_equal!(-3580.3757272128f64, payment(0.23, 5, 12345.67, 1.234567, true));
        assert_approx_equal!(3659.46546285804f64, payment(0.23, 10, -12345.67, -12345.67, false));
        assert_approx_equal!(2641.89087763997f64, payment(0.23, 10, -12345.67, -1.234567, true));
        assert_approx_equal!(3245.38497461473f64, payment(0.23, 10, -12345.67, 123.4567, false));
        assert_approx_equal!(359.736202636836f64, payment(0.23, 10, -123.4567, -12345.67, true));
        assert_approx_equal!(32.4948478142902f64, payment(0.23, 10, -123.4567, 0.0, false));
        assert_approx_equal!(23.0853991869919f64, payment(0.23, 10, -123.4567, 123.4567, true));
        assert_approx_equal!(4.42475529243308f64, payment(0.23, 10, -1.234567, -123.4567, false));
        assert_approx_equal!(0.264185754587725f64, payment(0.23, 10, -1.234567, 0.0, true));
        assert_approx_equal!(-409.655732950875f64, payment(0.23, 10, -1.234567, 12345.67, false));
        assert_approx_equal!(3.33317627178063f64, payment(0.23, 10, 0.0, -123.4567, true));
        assert_approx_equal!(-0.0409980681429018f64, payment(0.23, 10, 0.0, 1.234567, false));
        assert_approx_equal!(-333.317627178063f64, payment(0.23, 10, 0.0, 12345.67, true));
        assert_approx_equal!(-0.28395041f64, payment(0.23, 10, 1.234567, -1.234567, false));
        assert_approx_equal!(-0.297517517305531f64, payment(0.23, 10, 1.234567, 1.234567, true));
        assert_approx_equal!(377.485833614727f64, payment(0.23, 10, 123.4567, -12345.67, false));
        assert_approx_equal!(-26.3852436960547f64, payment(0.23, 10, 123.4567, -1.234567, true));
        assert_approx_equal!(-36.5946546285804f64, payment(0.23, 10, 123.4567, 123.4567, false));
        assert_approx_equal!(-2308.53991869919f64, payment(0.23, 10, 12345.67, -12345.67, true));
        assert_approx_equal!(-3249.48478142902f64, payment(0.23, 10, 12345.67, 0.0, false));
        assert_approx_equal!(-2645.19072214903f64, payment(0.23, 10, 12345.67, 123.4567, true));
        assert_approx_equal!(2839.59579004515f64, payment(0.23, 50, -12345.67, -123.4567, false));
        assert_approx_equal!(2308.61372538449f64, payment(0.23, 50, -12345.67, 0.0, true));
        assert_approx_equal!(2839.5041f64, payment(0.23, 50, -12345.67, 12345.67, false));
        assert_approx_equal!(23.0868753206979f64, payment(0.23, 50, -123.4567, -123.4567, true));
        assert_approx_equal!(28.3959397440069f64, payment(0.23, 50, -123.4567, 1.234567, false));
        assert_approx_equal!(23.0123305685425f64, payment(0.23, 50, -123.4567, 12345.67, true));
        assert_approx_equal!(0.283968566444584f64, payment(0.23, 50, -1.234567, -1.234567, false));
        assert_approx_equal!(0.230853991869919f64, payment(0.23, 50, -1.234567, 1.234567, true));
        assert_approx_equal!(0.0907822229220255f64, payment(0.23, 50, 0.0, -12345.67, false));
        assert_approx_equal!(7.38066853025501E-06f64, payment(0.23, 50, 0.0, -1.234567, true));
        assert_approx_equal!(-0.000907822229219732f64, payment(0.23, 50, 0.0, 123.4567, false));
        assert_approx_equal!(-0.157054687235803f64, payment(0.23, 50, 1.234567, -12345.67, true));
        assert_approx_equal!(-0.283959488222292f64, payment(0.23, 50, 1.234567, 0.0, false));
        assert_approx_equal!(-0.231599439391473f64, payment(0.23, 50, 1.234567, 123.4567, true));
        assert_approx_equal!(-28.395041f64, payment(0.23, 50, 123.4567, -123.4567, false));
        assert_approx_equal!(-23.0861372538449f64, payment(0.23, 50, 123.4567, 0.0, true));
        assert_approx_equal!(-28.4867310451513f64, payment(0.23, 50, 123.4567, 12345.67, false));
        assert_approx_equal!(-2308.61298731764f64, payment(0.23, 50, 12345.67, -123.4567, true));
        assert_approx_equal!(-2839.59489130114f64, payment(0.23, 50, 12345.67, 1.234567, false));
        assert_approx_equal!(-2308.68753206979f64, payment(0.23, 50, 12345.67, 12345.67, true));
        assert_approx_equal!(2839.5041f64, payment(0.23, 250, -12345.67, -1.234567, false));
        assert_approx_equal!(2308.53991869919f64, payment(0.23, 250, -12345.67, 1.234567, true));
        assert_approx_equal!(28.3950410000001f64, payment(0.23, 250, -123.4567, -12345.67, false));
        assert_approx_equal!(23.0853991869919f64, payment(0.23, 250, -123.4567, -1.234567, true));
        assert_approx_equal!(28.395041f64, payment(0.23, 250, -123.4567, 123.4567, false));
        assert_approx_equal!(0.230853991869828f64, payment(0.23, 250, -1.234567, -12345.67, true));
        assert_approx_equal!(0.28395041f64, payment(0.23, 250, -1.234567, 0.0, false));
        assert_approx_equal!(0.230853991869918f64, payment(0.23, 250, -1.234567, 123.4567, true));
        assert_approx_equal!(0f64, payment(0.23, 250, 0.0, -123.4567, false));
        assert_approx_equal!(0f64, payment(0.23, 250, 0.0, 0.0, true));
        assert_approx_equal!(0f64, payment(0.23, 250, 0.0, 12345.67, false));
        assert_approx_equal!(-0.230853991869918f64, payment(0.23, 250, 1.234567, -123.4567, true));
        assert_approx_equal!(-0.28395041f64, payment(0.23, 250, 1.234567, 1.234567, false));
        assert_approx_equal!(-0.230853991869828f64, payment(0.23, 250, 1.234567, 12345.67, true));
        assert_approx_equal!(-28.395041f64, payment(0.23, 250, 123.4567, -1.234567, false));
        assert_approx_equal!(-23.0853991869919f64, payment(0.23, 250, 123.4567, 1.234567, true));
        assert_approx_equal!(-2839.5041f64, payment(0.23, 250, 12345.67, -12345.67, false));
        assert_approx_equal!(-2308.53991869919f64, payment(0.23, 250, 12345.67, -1.234567, true));
        assert_approx_equal!(-2839.5041f64, payment(0.23, 250, 12345.67, 123.4567, false));
    }

}

