#![warn(missing_docs)]

//! **Payment calculations.** What is the periodic payment needed for an amortized loan and how much
//! of that is interest or principal?
//! 
//! For most common usages, we recommend the [payment_solution](fn.payment_solution.html) function
//! which provides a better debugging experience and additional functionality.
//!
//! ## Example
//! ```
//! let (rate, periods, present_value, future_value, due_at_beginning) = (0.034, 6, -1_000, 0, false);
//! let solution = finance_solution::core::payment_solution(rate, periods, present_value, future_value, due_at_beginning);
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
//! # use finance_solution::core::*;
//! let (rate, periods, present_value, future_value, due_at_beginning) = (0.034, 6, -1_000, 0, false);
//! let solution = finance_solution::core::payment_solution(rate, periods, present_value, future_value, due_at_beginning);
//! dbg!(solution.print_table());
//! ```
//! Outputs to terminal:
//! ```text
//! period   payment  payments_to_date  payments_remaining  principal  principal_to_date  principal_remaining  interest  interest_to_date  interest_remaining
//! ------  --------  ----------------  ------------------  ---------  -----------------  -------------------  --------  ----------------  ------------------
//!      1  187.0522          187.0522            935.2611   153.0522           153.0522             846.9478   34.0000           34.0000             88.3133
//!      2  187.0522          374.1044            748.2089   158.2560           311.3082             688.6918   28.7962           62.7962             59.5171
//!      3  187.0522          561.1567            561.1567   163.6367           474.9449             525.0551   23.4155           86.2117             36.1016
//!      4  187.0522          748.2089            374.1044   169.2003           644.1453             355.8547   17.8519          104.0636             18.2497
//!      5  187.0522          935.2611            187.0522   174.9532           819.0984             180.9016   12.0991          116.1627              6.1507
//!      6  187.0522        1_122.3133              0.0000   180.9016           999.0000               0.0000    6.1507          122.3133              0.0000
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
//! See [print_ab_comparison](struct.PaymentSolution.html#method.print_ab_comparison).
//!
//! # Positive and Negative Amounts
//!
//! This module uses the bookkeeping convention found in Google Sheets and Excel in which flows of
//! money in opposite directions have opposite signs.
//!
//! The example above was an amortized loan for $100,000 at 12% annual interest for ten years. Thus
//! the present value (the original principal of the loan) was $10,000 and the future value (the
//! amount still owed at the end) was zero. Also the rate was positive. In cases like this if the
//! present value is entered as a positive number the payment will be negative, and vice versa:
//! ```
//! use finance_solution::core::*;
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

use std::ops::Deref;

use super::*;

const RUN_PAYMENT_INVARIANTS: bool = true;

/// **A record of an amortized loan calculation** with options for producing the period-by-period details.
///
/// It's produced by calling [payment_solution](fn.payment_solution.html).
#[derive(Clone, Debug)]
pub struct PaymentSolution(CashflowSolution);

impl PaymentSolution {
    pub(crate) fn new(solution: CashflowSolution) -> Self {
        Self {
            0: solution,
        }
    }

    /// Calculates the period-by-period details of a payment calculation including how the payment
    /// is broken down between principal and interest.
    ///
    /// # Panics
    /// For now this method is not implemented for the case where the future value is nonzero.
    ///
    /// # Examples
    /// An amortized loan. Uses [payment](fn.payment.html).
    /// ```
    /// use finance_solution::core::*;
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
    /// let include_remaining_amounts = false;
    /// let locale = num_format::Locale::en;
    /// let precision = 2; // Two decimal places.
    /// series.print_table_locale(include_running_totals, include_remaining_amounts, &locale, precision);
    ///
    /// // As above but print only the last period for every yeor of the loan, that is periods 12, 24,
    /// // 36, 48, and 60; and show all columns with default number formatting.
    /// series
    ///     .filter(|x| x.period() % 12 == 0)
    ///     .print_table();
    /// ```
    pub fn series(&self) -> CashflowSeries {
        assert!(is_approx_equal!(0.0, self.future_value()), "This method cannot be called for a loan with a nonzero future value.");

        let mut series = vec![];
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
                                               interest_to_date, interest_remaining, &formula, &symbolic_formula);
            series.push(entry);
        }
        let series = CashflowSeries::new(series);
        if RUN_PAYMENT_INVARIANTS {
            payment_series_invariant(self, &series);
        }
        series
    }

    /// Prints a formatted table with the period-by-period details of a loan calculation.
    ///
    /// For more control over which columns appear use
    /// [print_table_custom](#method.print_table_custom).
    ///
    /// Money amounts are rounded to four decimal places, rates to six places, and numbers are
    /// formatted similar to Rust constants such as "10_000.0322". For more control over formatting
    /// use [print_table_locale](#method.print_table_locale) which also includes options for which
    /// columns appear.
    ///
    /// # Examples
    /// ```
    /// let (rate, periods, present_value, future_value, due_at_beginning) = (0.01, 60, -10_000, 0.0, false);
    /// finance_solution::core::payment_solution(rate, periods, present_value, future_value, due_at_beginning)
    ///     .print_table();
    /// ```
    /// Output (only the first three and last three rows shown):
    /// ```text
    /// period   payment  payments_to_date  payments_remaining  principal  principal_to_date  principal_remaining  interest  interest_to_date  interest_remaining
    /// ------  --------  ----------------  ------------------  ---------  -----------------  -------------------  --------  ----------------  ------------------
    ///      1  222.4445          222.4445         13_124.2241   122.4445           122.4445           9_877.5555  100.0000          100.0000          3_246.6686
    ///      2  222.4445          444.8890         12_901.7797   123.6689           246.1134           9_753.8866   98.7756          198.7756          3_147.8931
    ///      3  222.4445          667.3334         12_679.3352   124.9056           371.0190           9_628.9810   97.5389          296.3144          3_050.3542
    /// ...
    ///     58  222.4445       12_901.7797            444.8890   215.9024         9_561.6965             438.3035    6.5421        3_340.0832              6.5855
    ///     59  222.4445       13_124.2241            222.4445   218.0614         9_779.7579             220.2421    4.3830        3_344.4662              2.2024
    ///     60  222.4445       13_346.6686              0.0000   220.2421         9_999.0000               0.0000    2.2024        3_346.6686              0.0000
    /// ```
    pub fn print_table(&self) {
        self.series().print_table()
    }

    /// Prints a formatted table with the period-by-period details of a loan calculation with
    /// options for which columns appear.
    ///
    /// For a simpler method that includes all columns use [print_table](#method.print_table). To
    /// control number formatting use [print_table_locale](#method.print_table_locale).
    ///
    /// # Arguments
    /// * `include_running_totals` - If true include the columns "payments_to_date",
    /// "principal_to_date", and "interest_to_date".
    /// * `include_remaining_amounts` - If true include the columns "payments_remaining",
    /// "principal_remaining", and "interest_remaining".
    ///
    /// # Examples
    /// ```
    /// let include_running_totals = true;
    /// let include_remaining_amounts = false;
    /// let (rate, periods, present_value, future_value, due_at_beginning) = (0.01, 60, -10_000, 0.0, false);
    /// finance_solution::core::payment_solution(rate, periods, present_value, future_value, due_at_beginning)
    ///     .print_table_custom(include_running_totals, include_remaining_amounts);
    /// ```
    /// Output (only the first three and last three rows shown):
    /// ```text
    /// period   payment  payments_to_date  principal  principal_to_date  interest  interest_to_date
    /// ------  --------  ----------------  ---------  -----------------  --------  ----------------
    ///      1  222.4445          222.4445   122.4445           122.4445  100.0000          100.0000
    ///      2  222.4445          444.8890   123.6689           246.1134   98.7756          198.7756
    ///      3  222.4445          667.3334   124.9056           371.0190   97.5389          296.3144
    /// ...
    ///     58  222.4445       12_901.7797   215.9024         9_561.6965    6.5421        3_340.0832
    ///     59  222.4445       13_124.2241   218.0614         9_779.7579    4.3830        3_344.4662
    ///     60  222.4445       13_346.6686   220.2421         9_999.0000    2.2024        3_346.6686
    /// ```
    pub fn print_table_custom(&self, include_running_totals: bool, include_remaining_amounts: bool) {
        self.series().print_table_custom(include_running_totals, include_remaining_amounts);
    }

    /// Prints a formatted table with the period-by-period details of a loan calculation with
    /// options for which columns appear and how numbers are formatted.
    ///
    /// For a simpler method that doesn't require a locale but still has optional columns use
    /// [print_table_custom](#method.print_table_custom). The simplest table method is
    /// [print_table](#method.print_table) which prints all columns with default formatting.
    ///
    /// # Arguments
    /// * `include_running_totals` - If true include the columns "payments_to_date",
    /// "principal_to_date", and "interest_to_date".
    /// * `include_remaining_amounts` - If true include the columns "payments_remaining",
    /// "principal_remaining", and "interest_remaining".
    /// * `locale` - A locale constant from the `num-format` crate such as `Locale::en` for English
    /// or `Locale::vi` for Vietnamese. The locale determines the thousands separator and decimal
    /// separator.
    /// * `precision` - The number of decimal places for money amounts. Rates will appear with at
    /// least six places regardless of this argument.
    ///
    /// # Examples
    /// ```
    /// // Exclude running totals columns but include remaining amounts columns.
    /// let include_running_totals = false;
    /// let include_remaining_amounts = true;
    ///
    /// // English formatting with "," for the thousands separator and "." for the decimal
    /// // separator.
    /// let locale = finance_solution::core::num_format::Locale::en;
    ///
    /// // Show money amounts to two decimal places.
    /// let precision = 2;
    ///
    /// let (rate, periods, present_value, future_value, due_at_beginning) = (0.01, 60, -10_000, 0.0, false);
    /// finance_solution::core::payment_solution(rate, periods, present_value, future_value, due_at_beginning)
    ///     .print_table_locale(include_running_totals, include_remaining_amounts, &locale, precision);
    /// ```
    /// Output (only the first three and last three rows shown):
    /// ```text
    /// period  payment  payments_remaining  principal  principal_remaining  interest  interest_remaining
    /// ------  -------  ------------------  ---------  -------------------  --------  ------------------
    ///      1   222.44           13,124.22     122.44             9,877.56    100.00            3,246.67
    ///      2   222.44           12,901.78     123.67             9,753.89     98.78            3,147.89
    ///      3   222.44           12,679.34     124.91             9,628.98     97.54            3,050.35
    /// ...
    ///     58   222.44              444.89     215.90               438.30      6.54                6.59
    ///     59   222.44              222.44     218.06               220.24      4.38                2.20
    ///     60   222.44                0.00     220.24                 0.00      2.20                0.00
    /// ```
    pub fn print_table_locale(
        &self,
        include_running_totals: bool,
        include_remaining_amounts: bool,
        locale: &num_format::Locale,
        precision: usize)
    {
        self.series().print_table_locale(include_running_totals, include_remaining_amounts, locale, precision);
    }

    /// Compares the results of two loan calculations such as from two calls to
    /// [payment_solution](fn.payment_solution.html) with different rates.
    ///
    /// The values from the first calculation are labeled "a" and those from the second calculation
    /// are labeled "b". To control which columns appear in the table use
    /// [print_ab_comparison_custom](#method.print_ab_comparison_custom).
    ///
    /// Money amounts are rounded to four decimal places, rates to six places, and numbers are
    /// formatted similar to Rust constants such as "10_000.0322". For more control over formatting
    /// use [print_ab_comparison_locale](#method.print_ab_comparison_locale).
    ///
    /// # Arguments
    /// * `other` - The second `PaymentSolution` in the comparison which will be labeled "b".
    /// * `include_period_detail` - If true, print the month-by-month details of both loans.
    ///
    /// # Examples
    /// ```
    /// use finance_solution::core::*;
    ///
    /// // Both loan calculations will have the same rate, periods, present value, and future value.
    /// let (rate, periods, present_value, future_value) = (0.01, 60, -10_000, 0.0);
    ///
    /// // The first loan has the payment due at the end of the period, which is the usual case.
    /// let due_at_beginning = false;
    /// let solution_due_at_end = payment_solution(rate, periods, present_value, future_value, due_at_beginning);
    ///
    /// // The second loan has the payment due at the beginning of the period, and is otherwise the same
    /// // as the first loan.
    /// let due_at_beginning = true;
    /// let solution_due_at_beginning = payment_solution(rate, periods, present_value, future_value, due_at_beginning);
    ///
    /// let include_period_detail = true;
    /// solution_due_at_end.print_ab_comparison(&solution_due_at_beginning, include_period_detail);
    /// ```
    /// Output (in the table only the first three and last three rows are shown):
    /// ```text
    /// rate: 0.010000
    /// periods: 60
    /// present_value: -10_000.0000
    /// future_value: 0.0000
    /// due_at_beginning a: false
    /// due_at_beginning b: true
    /// payment a: 222.4445
    /// payment b: 220.2421
    /// sum_of_payments a: 13_346.6686
    /// sum_of_payments b: 13_214.5234
    /// sum_of_interest a: 3_346.6686
    /// sum_of_interest b: 3_214.5234
    /// formula a: 222.4445 = (-10000.0000 * 1.010000^60 * -0.010000) / (1.010000^60 - 1)
    /// formula b: 220.2421 = (-10000.0000 * 1.010000^60 * -0.010000) / ((1.010000^60 - 1) * 1.010000)
    /// symbolic_formula a: pmt = (pv * (1 + r)^n * -r) / ((1 + r)^n - 1)
    /// symbolic_formula b: pmt = (pv * (1 + r)^n * -r) / (((1 + r)^n - 1) * (1 + r))
    ///
    /// period  payment_a  payment_b  pmt_to_date_a  pmt_to_date_b  pmt_remaining_a  pmt_remaining_b  principal_a  principal_b  princ_to_date_a  princ_to_date_b  princ_remaining_a  princ_remaining_b  interest_a  interest_b  int_to_date_a  int_to_date_b  int_remaining_a  int_remaining_b
    /// ------  ---------  ---------  -------------  -------------  ---------------  ---------------  -----------  -----------  ---------------  ---------------  -----------------  -----------------  ----------  ----------  -------------  -------------  ---------------  ---------------
    ///      1   222.4445   220.2421       222.4445       220.2421      13_124.2241      12_994.2813     122.4445     220.2421         122.4445         220.2421         9_877.5555         9_779.7579    100.0000      0.0000       100.0000         0.0000       3_246.6686       3_214.5234
    ///      2   222.4445   220.2421       444.8890       440.4841      12_901.7797      12_774.0393     123.6689     122.4445         246.1134         342.6865         9_753.8866         9_657.3135     98.7756     97.7976       198.7756        97.7976       3_147.8931       3_116.7258
    ///      3   222.4445   220.2421       667.3334       660.7262      12_679.3352      12_553.7972     124.9056     123.6689         371.0190         466.3555         9_628.9810         9_533.6445     97.5389     96.5731       296.3144       194.3707       3_050.3542       3_020.1527
    /// ...
    ///     58   222.4445   220.2421    12_901.7797    12_774.0393         444.8890         440.4841     215.9024     213.7648       9_561.6965       9_566.0361           438.3035           433.9639      6.5421      6.4773     3_340.0832     3_208.0031           6.5855           6.5203
    ///     59   222.4445   220.2421    13_124.2241    12_994.2813         222.4445         220.2421     218.0614     215.9024       9_779.7579       9_781.9386           220.2421           218.0614      4.3830      4.3396     3_344.4662     3_212.3428           2.2024           2.1806
    ///     60   222.4445   220.2421    13_346.6686    13_214.5234           0.0000           0.0000     220.2421     218.0614       9_999.0000       9_999.0000             0.0000             0.0000      2.2024      2.1806     3_346.6686     3_214.5234           0.0000           0.0000
    /// ```
    pub fn print_ab_comparison(&self, other: &PaymentSolution, include_period_detail: bool)
    {
        self.print_ab_comparison_locale_opt(other, include_period_detail, true, true, None, None);
    }

    /// Compares the results of two loan calculations with options for which columns appear in the
    /// table. For a simpler method that includes all columns use
    /// [print_ab_comparison](#method.print_ab_comparison).
    ///
    /// The values from the first calculation are labeled "a" and those from the second calculation
    /// are labeled "b".
    ///
    /// Money amounts are rounded to four decimal places, rates to six places, and numbers are
    /// formatted similar to Rust constants such as "10_000.0322". For more control over formatting
    /// use [print_ab_comparison_locale](#method.print_ab_comparison_locale).
    ///
    /// # Arguments
    /// * `other` - The second `PaymentSolution` in the comparison which will be labeled "b".
    /// * `include_period_detail` - If true, print the month-by-month details of both loans.
    /// * `include_running_totals` - If true include "payments_to_date_a" (from the first
    /// calculation), "payments_to_date_b" (from the second calculation), and similar columns
    /// in the table.
    /// * `include_remaining_amounts` - If true include "principal_remaining_a" (from the first
    /// calculation), "principal_remaining_b" (from the second calculation), and similar columns
    /// in the table.
    ///
    /// # Examples
    /// For a more detailed example including more of the output text see
    /// [print_ab_comparison](#method.print_ab_comparison).
    /// ```
    /// use finance_solution::core::*;
    ///
    /// // The first loan has the payment due at the end of the period, and the second loan has the
    /// // payment due at the beginning. All other inputs are the same.
    /// let (rate, periods, present_value, future_value) = (0.01, 60, -10_000, 0.0);
    /// let solution_due_at_end = payment_solution(rate, periods, present_value, future_value, false);
    /// let solution_due_at_beginning = payment_solution(rate, periods, present_value, future_value, true);
    ///
    /// // In the table don't show the running totals or remaining amounts columns.
    /// let include_period_detail = true;
    /// let include_running_totals = false;
    /// let include_remaining_amounts = false;
    /// solution_due_at_end.print_ab_comparison_custom(&solution_due_at_beginning, include_period_detail, include_running_totals, include_remaining_amounts);
    /// ```
    /// In the output below only the first three and last three rows of the table are shown. For a
    /// more complete example see [print_ab_comparison](#method.print_ab_comparison). Note that in
    /// the first period there's no interest due for the second loan that has payments at the
    /// beginning of the period (the "interest_b" column on the right ) and after that the interest
    /// is always slightly lower than for the first loan with payments at the end of the period.
    /// ```text
    /// period  payment_a  payment_b  principal_a  principal_b  interest_a  interest_b
    /// ------  ---------  ---------  -----------  -----------  ----------  ----------
    ///      1   222.4445   220.2421     122.4445     220.2421    100.0000      0.0000
    ///      2   222.4445   220.2421     123.6689     122.4445     98.7756     97.7976
    ///      3   222.4445   220.2421     124.9056     123.6689     97.5389     96.5731
    /// ...
    ///     58   222.4445   220.2421     215.9024     213.7648      6.5421      6.4773
    ///     59   222.4445   220.2421     218.0614     215.9024      4.3830      4.3396
    ///     60   222.4445   220.2421     220.2421     218.0614      2.2024      2.1806
    /// ```
    pub fn print_ab_comparison_custom(&self, other: &PaymentSolution, include_period_detail: bool, include_running_totals: bool, include_remaining_amounts: bool)
    {
        self.print_ab_comparison_locale_opt(other, include_period_detail, include_running_totals, include_remaining_amounts, None, None);
    }

    /// Compares the results of two loan calculations with options for which columns appear in the
    /// table and for how numbers should be formatted. For a simpler method that doesn't require a
    /// locale use [print_ab_comparison_custom](#method.print_ab_comparison_custom). The simplest
    /// A/B comparison method that displays all columns with default formatting is
    /// [print_ab_comparison](#method.print_ab_comparison).
    ///
    /// The values from the first calculation are labeled "a" and those from the second calculation
    /// are labeled "b".
    ///
    /// # Arguments
    /// * `other` - The second `PaymentSolution` in the comparison which will be labeled "b".
    /// * `include_period_detail` - If true, print the month-by-month details of both loans.
    /// * `include_running_totals` - If true include "payments_to_date_a" (from the first
    /// calculation), "payments_to_date_b" (from the second calculation), and similar columns
    /// in the table.
    /// * `include_remaining_amounts` - If true include "principal_remaining_a" (from the first
    /// calculation), "principal_remaining_b" (from the second calculation), and similar columns
    /// in the table.
    /// * `locale` - A locale constant from the `num-format` crate such as `Locale::en` for English
    /// or `Locale::vi` for Vietnamese. The locale determines the thousands separator and decimal
    /// separator.
    /// * `precision` - The number of decimal places for money amounts. Rates will appear with at
    /// least six places regardless of this argument.
    ///
    /// # Examples
    /// ```
    /// use finance_solution::core::*;
    ///
    /// // The first loan has the payment due at the end of the period, and the second loan has the
    /// // payment due at the beginning. All other inputs are the same.
    /// let (rate, periods, present_value, future_value) = (0.01, 60, -10_000, 0.0);
    /// let solution_end = payment_solution(rate, periods, present_value, future_value, false);
    /// let solution_beginning = payment_solution(rate, periods, present_value, future_value, true);
    ///
    /// // Show the table and include the remaining amounts columns but but don't include the running
    /// // totals columns.
    /// let period_detail = true;
    /// let running_totals = false;
    /// let remaining_amounts = true;
    ///
    /// // English formatting with "," for the thousands separator and "." for the decimal
    /// // separator.
    /// let locale = finance_solution::core::num_format::Locale::en;
    ///
    /// // Show money amounts with two decimal places.
    /// let precision = 2;
    ///
    /// solution_end.print_ab_comparison_locale(&solution_beginning, period_detail, running_totals, remaining_amounts, &locale, precision);
    /// ```
    /// Output (in the table only the first three and last three rows are shown):
    /// ```text
    /// rate: 0.010000
    /// periods: 60
    /// present_value: -10,000.00
    /// future_value: 0.00
    /// due_at_beginning a: false
    /// due_at_beginning b: true
    /// payment a: 222.44
    /// payment b: 220.24
    /// sum_of_payments a: 13,346.67
    /// sum_of_payments b: 13,214.52
    /// sum_of_interest a: 3,346.67
    /// sum_of_interest b: 3,214.52
    /// formula a: 222.4445 = (-10000.0000 * 1.010000^60 * -0.010000) / (1.010000^60 - 1)
    /// formula b: 220.2421 = (-10000.0000 * 1.010000^60 * -0.010000) / ((1.010000^60 - 1) * 1.010000)
    /// symbolic_formula a: pmt = (pv * (1 + r)^n * -r) / ((1 + r)^n - 1)
    /// symbolic_formula b: pmt = (pv * (1 + r)^n * -r) / (((1 + r)^n - 1) * (1 + r))
    ///
    /// period  payment_a  payment_b  pmt_remaining_a  pmt_remaining_b  principal_a  principal_b  princ_remaining_a  princ_remaining_b  interest_a  interest_b  int_remaining_a  int_remaining_b
    /// ------  ---------  ---------  ---------------  ---------------  -----------  -----------  -----------------  -----------------  ----------  ----------  ---------------  ---------------
    ///      1     222.44     220.24        13,124.22        12,994.28       122.44       220.24           9,877.56           9,779.76      100.00        0.00         3,246.67         3,214.52
    ///      2     222.44     220.24        12,901.78        12,774.04       123.67       122.44           9,753.89           9,657.31       98.78       97.80         3,147.89         3,116.73
    ///      3     222.44     220.24        12,679.34        12,553.80       124.91       123.67           9,628.98           9,533.64       97.54       96.57         3,050.35         3,020.15
    /// ...
    ///     58     222.44     220.24           444.89           440.48       215.90       213.76             438.30             433.96        6.54        6.48             6.59             6.52
    ///     59     222.44     220.24           222.44           220.24       218.06       215.90             220.24             218.06        4.38        4.34             2.20             2.18
    ///     60     222.44     220.24             0.00             0.00       220.24       218.06               0.00               0.00        2.20        2.18             0.00             0.00
    /// ```
    pub fn print_ab_comparison_locale(
        &self,
        other: &PaymentSolution,
        include_period_detail: bool,
        include_running_totals: bool,
        include_remaining_amounts: bool,
        locale: &num_format::Locale,
        precision: usize)
    {
        self.print_ab_comparison_locale_opt(other, include_period_detail, include_running_totals, include_remaining_amounts, Some(locale), Some(precision));
    }

    fn print_ab_comparison_locale_opt(
        &self,
        other: &PaymentSolution,
        include_period_detail: bool,
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

        if include_period_detail {
            self.series().print_ab_comparison_locale_opt(&other.series(), include_running_totals, include_remaining_amounts, locale, precision);
        }
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

fn payment_series_invariant(solution: &PaymentSolution, series: &CashflowSeries) {
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
    assert!(solution.calculated_field().is_payment());
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

/// Returns the payment needed at the end of every period for an amortized loan.
///
/// Related functions:
/// * To calculate the payment and return a struct that shows the interest, the formula, and
/// optionally the period-by-period values use [payment_solution](fn.payment_solution.html).
///
/// # Formulas
///
/// See the [payment module](index.html#formulas) documentation.
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
/// # use finance_solution::core::*;
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
/// assert_rounded_4!(pmt, -212.4704);
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
/// assert_rounded_4!(pmt, -210.7145);
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
    assert!(!(periods == 0 && !is_approx_equal!(0.0, present_value + future_value)), "There are no periods and the present value + future value is not zero so there is no way to calculate payments.");

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
/// # Formulas
///
/// See the [payment module](index.html#formulas) documentation.
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
/// # use finance_solution::core::*;
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
/// assert_rounded_4!(solution.payment(), 327.6538);
///
/// // The sum of payments is simply the monthly payment times the number of months.
/// assert_rounded_4!(solution.sum_of_payments(), 15_727.3820);
///
/// // The sum of interest is the portion of the sum of payments that is over and above the original
/// // loan amount. Here we add the present value since it has the opposite sign of the payments.
/// assert_rounded_4!(solution.sum_of_interest(), solution.sum_of_payments() + solution.present_value());
/// assert_rounded_4!(solution.sum_of_interest(), 3_226.8820);
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
///     .print_table_custom(include_running_totals, include_remaining_amounts);
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

    #[allow(dead_code)]
    fn run_payment_invariants(solution: &PaymentSolution, series: &CashflowSeries) {
        // Display the solution and series only if either one fails its invariant.
        let result = std::panic::catch_unwind(|| {
            solution.invariant();
            payment_series_invariant(solution, series);
        });
        //bg!(&result);
        if result.is_err() {
            dbg!(&solution, &series);
            solution.invariant();
            payment_series_invariant(solution, series);
        }
    }

    fn compare_to_excel (
        test_case: usize,
        r: f64,
        n: u32,
        pv: f64,
        fv: f64,
        pmt_excel_end: f64,
        pmt_excel_beginning: f64,
        pmt_manual_end: f64,
        pmt_manual_beginning: f64
    ) {
        let display = false;

        if display {
            println!("test_case = {}, r = {}, n = {}, pv = {}, fv = {}, pmt_excel_end: {}, pmt_excel_beginning: {}, pmt_manual_end: {}, pmt_manual_beginning: {}",
                    test_case, r, n, pv, fv, pmt_excel_end, pmt_excel_beginning, pmt_manual_end, pmt_manual_beginning);
        }
        assert_approx_equal!(pmt_excel_end, pmt_manual_end);
        assert_approx_equal!(pmt_excel_beginning, pmt_manual_beginning);

        let pmt_calc_end = payment(r, n, pv, fv,false);
        if display { println!("pmt_calc_end = {}", pmt_calc_end) }
        assert_approx_equal!(pmt_excel_end, pmt_calc_end);

        let pmt_calc_beginning = payment(r, n, pv, fv, true);
        if display { println!("pmt_calc_beginning = {}", pmt_calc_beginning); }
        assert_approx_equal!(pmt_excel_beginning, pmt_calc_beginning);

        // Solution with payment at the end of the period.
        let solution = payment_solution(r, n, pv, fv, false);
        if display { dbg!(&solution); }
        solution.invariant();
        assert!(solution.calculated_field().is_payment());
        assert_approx_equal!(r, solution.rate());
        assert_eq!(n, solution.periods());
        assert_approx_equal!(pv, solution.present_value());
        assert_approx_equal!(fv, solution.future_value());
        assert_eq!(false, solution.due_at_beginning());
        assert_approx_equal!(pmt_excel_end, solution.payment());

        // Solution with payment at the beginning of the period.
        let solution = payment_solution(r, n, pv, fv, true);
        if display { dbg!(&solution); }
        solution.invariant();
        assert!(solution.calculated_field().is_payment());
        assert_approx_equal!(r, solution.rate());
        assert_eq!(n, solution.periods());
        assert_approx_equal!(pv, solution.present_value());
        assert_approx_equal!(fv, solution.future_value());
        assert!(solution.due_at_beginning());
        assert_approx_equal!(pmt_excel_beginning, solution.payment());
    }

    #[test]
    fn test_payment_against_excel() {
        compare_to_excel(1, 0.01f64, 90, -0.1f64, 1f64, -0.00521275507414874f64, -0.00516114363777103f64, -0.00521275507414872f64, -0.00516114363777101f64);
        compare_to_excel(2, 0.07f64, 85, 1.05f64, -1.5f64, -0.073399521636348f64, -0.0685976837722879f64, -0.073399521636348f64, -0.0685976837722879f64);
        compare_to_excel(3, 0.05f64, 80, -2.25f64, 2.25f64, 0.1125f64, 0.107142857142857f64, 0.1125f64, 0.107142857142857f64);
        compare_to_excel(4, -0.01f64, 75, 4.3875f64, -3.375f64, 0.0247500575725451f64, 0.0250000581540859f64, 0.0247500575725451f64, 0.025000058154086f64);
        compare_to_excel(5, -0.07f64, 70, -10.125f64, 5.0625f64, -0.352156909187232f64, -0.378663343212077f64, -0.352156909187232f64, -0.378663343212077f64);
        compare_to_excel(6, 0.011f64, 65, 0.759375f64, -7.59375f64, 0.0641970901801655f64, 0.0634986055194515f64, 0.0641970901801662f64, 0.0634986055194522f64);
        compare_to_excel(7, 0.077f64, 60, -7.9734375f64, 11.390625f64, 0.610847873854429f64, 0.567175370338374f64, 0.610847873854429f64, 0.567175370338374f64);
        compare_to_excel(8, 0.055f64, 55, 17.0859375f64, -17.0859375f64, -0.9397265625f64, -0.890736078199052f64, -0.9397265625f64, -0.890736078199052f64);
        compare_to_excel(9, -0.011f64, 50, -33.317578125f64, 25.62890625f64, -0.167401416265857f64, -0.169263312705619f64, -0.167401416265857f64, -0.169263312705618f64);
        compare_to_excel(10, -0.077f64, 45, 76.88671875f64, -38.443359375f64, 2.87746803518998f64, 3.11751683119175f64, 2.87746803518998f64, 3.11751683119175f64);
        compare_to_excel(11, 0.0121f64, 40, -5.76650390625f64, 57.6650390625f64, -0.946616080514215f64, -0.935298963061174f64, -0.946616080514218f64, -0.935298963061177f64);
        compare_to_excel(12, 0.0847f64, 35, 60.548291015625f64, -86.49755859375f64, -4.99286966179272f64, -4.60299590835505f64, -4.99286966179272f64, -4.60299590835504f64);
        compare_to_excel(13, 0.0605f64, 30, -129.746337890625f64, 129.746337890625f64, 7.84965344238281f64, 7.40184200130393f64, 7.84965344238281f64, 7.40184200130393f64);
        compare_to_excel(14, -0.0121f64, 25, 253.005358886719f64, -194.619506835937f64, 0.368964477134417f64, 0.373483629045872f64, 0.368964477134412f64, 0.373483629045867f64);
        compare_to_excel(15, -0.0847f64, 20, -583.858520507812f64, 291.929260253906f64, -19.6504225039078f64, -21.468832627453f64, -19.6504225039078f64, -21.468832627453f64);
        compare_to_excel(16, 0.01331f64, 15, 43.7893890380859f64, -437.893890380859f64, 23.3291369500755f64, 23.0227047498549f64, 23.3291369500757f64, 23.0227047498552f64);
        compare_to_excel(17, 0.09317f64, 12, -459.788584899902f64, 656.840835571289f64, 33.2383843797982f64, 30.4055036085863f64, 33.2383843797982f64, 30.4055036085862f64);
        compare_to_excel(18, 0.06655f64, 10, 985.261253356933f64, -985.261253356933f64, -65.5691364109039f64, -61.477789518451f64, -65.569136410904f64, -61.477789518451f64);
        compare_to_excel(19, -0.01331f64, 7, -1921.25944404602f64, 1477.8918800354f64, 40.340533116546f64, 40.8847085878503f64, 40.3405331165459f64, 40.8847085878502f64);
        compare_to_excel(20, -0.09317f64, 5, 4433.6756401062f64, -2216.8378200531f64, -120.945793012958f64, -133.372068648984f64, -120.945793012958f64, -133.372068648984f64);
        compare_to_excel(21, 0.014641f64, 4, -332.525673007965f64, 3325.25673007965f64, -727.082049320411f64, -716.590448563f64, -727.082049320416f64, -716.590448563005f64);
        compare_to_excel(22, 0.102487f64, 3, 3491.51956658363f64, -4987.88509511947f64, 93.1536336476088f64, 84.4940880460349f64, 93.153633647609f64, 84.494088046035f64);
        compare_to_excel(23, 0.073205f64, 2, -7481.82764267921f64, 7481.82764267921f64, 547.707192582332f64, 510.347224046042f64, 547.707192582332f64, 510.347224046042f64);
        compare_to_excel(24, -0.014641f64, 1, 14589.5639032245f64, -11222.7414640188f64, -3153.21663409854f64, -3200.06884201447f64, -3153.21663409853f64, -3200.06884201447f64);
    }


}

