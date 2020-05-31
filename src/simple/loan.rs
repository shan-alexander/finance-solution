//! **Amortized loan calculations.**
//!
//! # Formulas
//!
//! The calculation is:
//! > <img src="http://i.upmath.me/svg/rate%20%3D%20%5Cfrac%7Bapr%7D%7B12%7D" />
//!
//! > <img src="http://i.upmath.me/svg/payment%20%3D%20%5Cfrac%7Bprincipal%20%5Ctimes%20%5Cleft(1%2Brate%5Cright)%5E%7Bmonths%7D%20%5Ctimes%20rate%7D%7B%5Cleft(1%2Brate%5Cright)%5E%7Bmonths%7D%20-%201%7D%7D" />

// Import needed for the function references in the Rustdoc comments.
#[allow(unused_imports)]
use crate::*;

const RUN_INVARIANTS: bool = false;

/// A record of an amortized loan calculation.
#[derive(Clone, Debug)]
pub struct LoanSolution {
    apr: f64,
    monthly_rate: f64,
    months: u32,
    principal: f64,
    payment: f64,
    sum_of_payments: f64,
    sum_of_interest: f64,
    formula: String,
    symbolic_formula: String,
}

impl LoanSolution {
    pub(crate) fn new(
        principal: f64,
        apr: f64,
        months: u32,
        payment: f64,
        formula: &str,
        symbolic_formula: &str,
    ) -> Self {
        assert!(!formula.is_empty());
        let sum_of_payments = payment * months as f64;
        let sum_of_interest = sum_of_payments - principal;
        Self {
            principal,
            apr,
            monthly_rate: apr / 12.0,
            months,
            payment,
            sum_of_payments,
            sum_of_interest,
            formula: formula.to_string(),
            symbolic_formula: symbolic_formula.to_string(),
        }
    }

    /// Returns the annual percentage rate of the loan.
    pub fn apr(&self) -> f64 {
        self.apr
    }

    /// Returns the monthly rate, which is the annual percentage rate (APR) divided by 12.
    pub fn monthly_rate(&self) -> f64 {
        self.monthly_rate
    }

    /// Returns the number of monthly periods in the loan.
    pub fn months(&self) -> u32 {
        self.months
    }

    /// Returns the initial amount of the loan.
    pub fn principal(&self) -> f64 {
        self.principal
    }

    /// Returns the monthly payment which includes varying amounts of interest and principal.
    pub fn payment(&self) -> f64 {
        self.payment
    }

    /// The total amount paid during the life of the loan. This is the principal plus the sum of the
    /// interest.
    pub fn sum_of_payments(&self) -> f64 {
        self.sum_of_payments
    }

    /// The total interest paid over the life of the loan.
    pub fn sum_of_interest(&self) -> f64 {
        self.sum_of_interest
    }

    /// The formula used to calculate the monthly payment.
    pub fn formula(&self) -> &str {
        &self.formula
    }

    /// The formula used to calculate the monthly payment, expressed in variables rather than the
    /// concrete numbers of this particular loan.
    pub fn symbolic_formula(&self) -> &str {
        &self.symbolic_formula
    }

    /// Calculates the month-by-month details of a loan calculation including how the payment
    /// is broken down between principal and interest.
    ///
    /// # Examples
    /// ```
    /// // use finance_solution::simple::*;
    ///
    /// // $20,000 loan at 15% APR for 5 years.
    /// let solution = finance_solution::simple::loan_solution(20_000, 0.15, 5 * 12);
    /// dbg!(&solution);
    ///
    /// // Calculate the month-by-month details including the principal and interest paid every month.
    /// let series = solution.series();
    /// dbg!(&series);
    ///
    /// // Print the monthly detail numbers as a formatted table.
    /// series.print_table();
    ///
    /// // As above but print only the last period for every yeor of the loan, with each entry
    /// // showing a summary of payments, interest, and principal paid down for that year.
    /// series
    ///     .summarize_by_year()
    ///     .print_table();
    /// ```
    /// Output from the last line:
    /// ```text
    /// period   payment  payments_to_date  payments_remaining  principal  principal_to_date  principal_remaining  interest  interest_to_date  interest_remaining
    /// ------  --------  ----------------  ------------------  ---------  -----------------  -------------------  --------  ----------------  ------------------
    ///     12  5_709.58          5_709.58           22_838.33   2_903.85           2_903.85            17_096.15  2_805.73          2_805.73            5_742.18
    ///     24  5_709.58         11_419.17           17_128.75   3_370.66           6_274.51            13_725.49  2_338.92          5_144.66            3_403.26
    ///     36  5_709.58         17_128.75           11_419.17   3_912.51          10_187.02             9_812.98  1_797.08          6_941.73            1_606.18
    ///     48  5_709.58         22_838.33            5_709.58   4_541.46          14_728.48             5_271.52  1_168.12          8_109.85              438.06
    ///     60  5_709.58         28_547.92                0.00   5_271.52          20_000.00                 0.00    438.06          8_547.92                0.00
    /// ```
    pub fn series(&self) -> LoanSeries {
        LoanSeries::new(payment_solution_from_loan_arguments(self.principal, self.apr, self.months).series())
    }

    /// Prints a table of the monthly details of the loan using default formatting.
    ///
    /// # Examples
    /// ```
    /// finance_solution::simple::loan_solution(200_000, 0.06, 180)
    ///     .print_table();
    /// ```
    /// Output (showing only the first five periods):
    /// ```text
    /// period   payment  payments_to_date  payments_remaining  principal  principal_to_date  principal_remaining  interest  interest_to_date  interest_remaining
    /// ------  --------  ----------------  ------------------  ---------  -----------------  -------------------  --------  ----------------  ------------------
    ///      1  1_687.71          1_687.71          302_100.74     687.71             687.71           199_312.29  1_000.00          1_000.00          102_788.46
    ///      2  1_687.71          3_375.43          300_413.03     691.15           1_378.87           198_621.13    996.56          1_996.56          101_791.90
    ///      3  1_687.71          5_063.14          298_725.32     694.61           2_073.47           197_926.53    993.11          2_989.67          100_798.79
    ///      4  1_687.71          6_750.85          297_037.60     698.08           2_771.55           197_228.45    989.63          3_979.30           99_809.16
    ///      5  1_687.71          8_438.57          295_349.89     701.57           3_473.13           196_526.87    986.14          4_965.44           98_823.02
    /// ```
    pub fn print_table(&self) {
        self.series().print_table()
    }

    /// Prints a table of the monthly details of the loan using default formatting and options for
    /// which columns appear.
    ///
    /// # Arguments
    /// * `include_running_totals` - If true, include the columns "payments_to_date",
    /// "principal_to_date", and "interest_to_date".
    /// * `include_remaining_amounts` - If true, include the columns "remaining_payments",
    /// "remaining_principal", and "remaining_interest".
    ///
    /// # Examples
    /// ```
    /// // Print a table including running totals columns such as `payments_to_date` but not
    /// // including remaining amounts columns like `principal_remaining`.
    /// let include_running_totals = true;
    /// let include_remaining_amounts = false;
    /// finance_solution::simple::loan_solution(200_000, 0.06, 180)
    ///     .print_table_custom(include_running_totals, include_remaining_amounts);
    /// ```
    /// Output (showing only the first five periods):
    /// ```text
    /// period   payment  payments_to_date  principal  principal_to_date  interest  interest_to_date
    /// ------  --------  ----------------  ---------  -----------------  --------  ----------------
    ///      1  1_687.71          1_687.71     687.71             687.71  1_000.00          1_000.00
    ///      2  1_687.71          3_375.43     691.15           1_378.87    996.56          1_996.56
    ///      3  1_687.71          5_063.14     694.61           2_073.47    993.11          2_989.67
    ///      4  1_687.71          6_750.85     698.08           2_771.55    989.63          3_979.30
    ///      5  1_687.71          8_438.57     701.57           3_473.13    986.14          4_965.44
    /// ```
    pub fn print_table_custom(&self, include_running_totals: bool, include_remaining_amounts: bool) {
        self.series().print_table_custom(include_running_totals, include_remaining_amounts)
    }

    /// Print a table of the monthly details of the loan with options for which columns appear and
    /// for how the numbers are formatted.
    ///
    /// # Arguments
    /// * `include_running_totals` - If true, include the columns "payments_to_date",
    /// "principal_to_date", and "interest_to_date".
    /// * `include_remaining_amounts` - If true, include the columns "remaining_payments",
    /// "remaining_principal", and "remaining_interest".
    /// * `locale` - A variant of the num_format::Locale enum which determines the characters used
    /// for thousands separators and the decimal separator
    /// * `precision` - The number of decimal places. Rates always appear with at least six places
    /// regardless of this value.
    ///
    /// # Examples
    /// ```
    /// let include_running_totals = false;
    /// let include_remaining_amounts = true;
    /// let locale = num_format::Locale::en;
    /// let precision = 0; // Round money amounts to whole numbers.
    /// finance_solution::simple::loan_solution(200_000, 0.06, 180)
    ///     .print_table_locale(include_running_totals, include_remaining_amounts, &locale, precision);
    /// ```
    /// Output (showing only the first five periods):
    /// ```text
    /// period  payment  payments_remaining  principal  principal_remaining  interest  interest_remaining
    /// ------  -------  ------------------  ---------  -------------------  --------  ------------------
    ///      1    1,688             302,101        688              199,312     1,000             102,788
    ///      2    1,688             300,413        691              198,621       997             101,792
    ///      3    1,688             298,725        695              197,927       993             100,799
    ///      4    1,688             297,038        698              197,228       990              99,809
    ///      5    1,688             295,350        702              196,527       986              98,823
    /// ```
    pub fn print_table_locale(
        &self,
        include_running_totals: bool,
        include_remaining_amounts: bool,
        locale: &num_format::Locale,
        precision: usize)
    {
        self.series().print_table_locale(include_running_totals, include_remaining_amounts, locale, precision)
    }

    /// Compares two loans, such as loans that are the same except for the APR, and optionally
    /// prints the month-by-month details.
    ///
    /// The first loan is labeled "a" and the second is "b".
    ///
    /// # Arguments
    /// * `other` - The second loan in the comparison which will be labeled "b".
    /// * `include_period_detail` - If true, print the month-by-month details of the two loans,
    /// contrasting their payment amounts, interest, remaining principal, and so on.
    ///
    /// # Examples
    /// ```
    /// // $200,000 at 6% APR for 15 years.
    /// let solution_six_pct = finance_solution::simple::loan_solution(200_000, 0.06, 180);
    ///
    /// // Create a variation on the first loan with a 5% APR.
    /// let solution_five_pct = solution_six_pct.with_apr(0.05);
    ///
    /// // Compare the two loans but don't show the monthly details.
    /// let include_period_detail = false;
    /// solution_six_pct.print_ab_comparison(&solution_five_pct, include_period_detail);
    /// ```
    /// Output:
    /// ```text
    /// principal: 200_000.0000
    /// apr a: 0.060000
    /// apr b: 0.050000
    /// monthly_rate a: 0.005000
    /// monthly_rate b: 0.004167
    /// months: 180
    /// payment a: 1_687.7137
    /// payment b: 1_581.5873
    /// sum_of_payments a: 303_788.4581
    /// sum_of_payments b: 284_685.7056
    /// sum_of_interest a: 103_788.4581
    /// sum_of_interest b:  84_685.7056
    /// formula a: 1687.7137 = (200000.0000 * 1.005000^180 * 0.005000) / (1.005000^180 - 1)
    /// formula b: 1581.5873 = (200000.0000 * 1.004167^180 * 0.004167) / (1.004167^180 - 1)
    /// symbolic_formula: pmt = (P * (1+r)^n * r) / ((1 + r)^n - 1)
    /// ```
    pub fn print_ab_comparison(&self, other: &LoanSolution, include_period_detail: bool) {
        self.print_ab_comparison_custom(other, include_period_detail, true, true);
    }

    /// Compares two loans, such as loans that are the same except for the APR, and optionally
    /// prints the month-by-month details with further options for which columns appear.
    ///
    /// The first loan is labeled "a" and the second is "b".
    ///
    /// # Arguments
    /// * `other` - The second loan in the comparison which will be labeled "b".
    /// * `include_period_detail` - If true, print the month-by-month details of the two loans,
    /// contrasting their payment amounts, interest, remaining principal, and so on.
    /// * `include_running_totals` - If true, include the columns "payments_to_date",
    /// "principal_to_date", and "interest_to_date".
    /// * `include_remaining_amounts` - If true, include the columns "remaining_payments",
    /// "remaining_principal", and "remaining_interest".
    ///
    /// # Examples
    /// ```
    /// // $200,000 at 6% APR for 15 years.
    /// let solution_six_pct = finance_solution::simple::loan_solution(200_000, 0.06, 180);
    ///
    /// // Create a variation on the first loan with a 5% APR.
    /// let solution_five_pct = solution_six_pct.with_apr(0.05);
    ///
    /// // Compare the two loans and show the monthly details but leave out the remaining amounts
    /// // columns.
    /// let include_period_detail = true;
    /// let include_running_totals = true;
    /// let include_remaining_amounts = false;
    /// solution_six_pct.print_ab_comparison_custom(&solution_five_pct,
    ///         include_period_detail, include_running_totals, include_remaining_amounts);
    /// ```
    /// Output (skipping the header information which is shown above in `print_ab_comparison`, and
    /// showing only the last five months.
    /// ```text
    /// period  payment_a  payment_b  pmt_to_date_a  pmt_to_date_b  principal_a  principal_b  princ_to_date_a  princ_to_date_b  interest_a  interest_b  int_to_date_a  int_to_date_b
    /// ------  ---------  ---------  -------------  -------------  -----------  -----------  ---------------  ---------------  ----------  ----------  -------------  -------------
    ///    176   1_687.71   1_581.59     297_037.60     278_359.36     1_646.15     1_549.05       193_332.69       193_739.01       41.57       32.54     103_704.91      84_620.35
    ///    177   1_687.71   1_581.59     298_725.32     279_940.94     1_654.38     1_555.50       194_987.07       195_294.51       33.34       26.09     103_738.25      84_646.44
    ///    178   1_687.71   1_581.59     300_413.03     281_522.53     1_662.65     1_561.98       196_649.72       196_856.49       25.06       19.61     103_763.31      84_666.05
    ///    179   1_687.71   1_581.59     302_100.74     283_104.12     1_670.96     1_568.49       198_320.68       198_424.98       16.75       13.10     103_780.06      84_679.14
    ///    180   1_687.71   1_581.59     303_788.46     284_685.71     1_679.32     1_575.02       200_000.00       200_000.00        8.40        6.56     103_788.46      84_685.71
    /// ```
    pub fn print_ab_comparison_custom(
        &self,
        other: &LoanSolution,
        include_period_detail: bool,
        include_running_totals: bool,
        include_remaining_amounts: bool)
    {
        self.print_ab_comparison_locale_opt(other, include_period_detail, include_running_totals, include_remaining_amounts, None, Some(2));
    }

    /// Compares two loans, such as loans that are the same except for the APR, and optionally
    /// prints the month-by-month details with further options for which columns appear and how
    /// numbers are formatted.
    ///
    /// The first loan is labeled "a" and the second is "b".
    ///
    /// # Arguments
    /// * `other` - The second loan in the comparison which will be labeled "b".
    /// * `include_period_detail` - If true, print the month-by-month details of the two loans,
    /// contrasting their payment amounts, interest, remaining principal, and so on.
    /// * `include_running_totals` - If true, include the columns "payments_to_date",
    /// "principal_to_date", and "interest_to_date".
    /// * `include_remaining_amounts` - If true, include the columns "remaining_payments",
    /// "remaining_principal", and "remaining_interest".
    /// * `locale` - A variant of the num_format::Locale enum which determines the characters used
    /// for thousands separators and the decimal separator
    /// * `precision` - The number of decimal places. Rates always appear with at least six places
    /// regardless of this value.
    ///
    /// # Examples
    /// ```
    /// // $200,000 at 6% APR for 15 years.
    /// let solution_six_pct = finance_solution::simple::loan_solution(200_000, 0.06, 180);
    ///
    /// // Create a variation on the first loan with a 5% APR.
    /// let solution_five_pct = solution_six_pct.with_apr(0.05);
    ///
    /// // Compare the two loans and show the monthly details with custom formatting.
    /// let include_period_detail = true;
    /// let include_running_totals = false;
    /// let include_remaining_amounts = false;
    /// let locale = num_format::Locale::en;
    /// let precision = 0; // Round money amounts to whole numbers.
    /// solution_six_pct.print_ab_comparison_locale(&solution_five_pct,
    ///         include_period_detail, include_running_totals, include_remaining_amounts,
    ///         &locale, precision);
    /// ```
    /// Output (skipping the header information which is shown above in `print_ab_comparison`, and
    /// showing only the last five months.
    /// ```text
    /// period  payment_a  payment_b  principal_a  principal_b  interest_a  interest_b
    /// ------  ---------  ---------  -----------  -----------  ----------  ----------
    ///      1      1,688      1,582          688          748       1,000         833
    ///      2      1,688      1,582          691          751         997         830
    ///      3      1,688      1,582          695          755         993         827
    ///      4      1,688      1,582          698          758         990         824
    ///      5      1,688      1,582          702          761         986         821
    /// ```
    pub fn print_ab_comparison_locale(
        &self,
        other: &LoanSolution,
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
        other: &LoanSolution,
        include_period_detail: bool,
        include_running_totals: bool,
        include_remaining_amounts: bool,
        locale: Option<&num_format::Locale>,
        precision: Option<usize>)
    {
        println!();
        print_ab_comparison_values_float("principal", self.principal, other.principal, locale, precision);
        print_ab_comparison_values_rate("apr", self.apr, other.apr, locale, precision);
        print_ab_comparison_values_rate("monthly_rate", self.monthly_rate, other.monthly_rate, locale, precision);
        print_ab_comparison_values_int("months", i128::from(self.months), i128::from(other.months), locale);
        print_ab_comparison_values_float("payment", self.payment, other.payment, locale, precision);
        print_ab_comparison_values_float("sum_of_payments", self.sum_of_payments, other.sum_of_payments, locale, precision);
        print_ab_comparison_values_float("sum_of_interest", self.sum_of_interest, other.sum_of_interest, locale, precision);
        print_ab_comparison_values_string("formula", &self.formula, &other.formula);
        print_ab_comparison_values_string("symbolic_formula", &self.symbolic_formula, &other.symbolic_formula);
        if include_period_detail {
            self.series().payment_series.print_ab_comparison_locale_opt(&other.series().payment_series, include_running_totals, include_remaining_amounts, locale, precision);
        }
    }

    /*
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
    */

    /// Creates a variation on a loan which is the same except for the principal. This might
    /// represent a different down payment.
    ///
    /// # Examples
    /// ```
    /// // $200,000 at 6% APR for 15 years.
    /// let solution_200 = finance_solution::simple::loan_solution(200_000, 0.06, 180);
    ///
    /// // Create a variation on the first loan with a lower principal.
    /// let solution_180 = solution_200.with_principal(180_000);
    ///
    /// // Compare the two loans but don't show the monthly details.
    /// solution_200.print_ab_comparison(&solution_180, false);
    /// ```
    /// Output:
    /// ```text
    /// principal a: 200_000.00
    /// principal b: 180_000.00
    /// apr: 0.060000
    /// monthly_rate: 0.005000
    /// months: 180
    /// payment a: 1_687.71
    /// payment b: 1_518.94
    /// sum_of_payments a: 303_788.46
    /// sum_of_payments b: 273_409.61
    /// sum_of_interest a: 103_788.46
    /// sum_of_interest b:  93_409.61
    /// formula a: 1687.7137 = (200000.0000 * 1.005000^180 * 0.005000) / (1.005000^180 - 1)
    /// formula b: 1518.9423 = (180000.0000 * 1.005000^180 * 0.005000) / (1.005000^180 - 1)
    /// symbolic_formula: pmt = (P * (1+r)^n * r) / ((1 + r)^n - 1)
    /// ```
    pub fn with_principal<T>(&self, principal: T) -> LoanSolution
        where T: Into<f64> + Copy
    {
        loan_solution(principal, self.apr, self.months)
    }

    /// Creates a variation on a loan which is the same except for the APR.
    /// # Examples
    /// ```
    /// // $200,000 at 6% APR for 15 years.
    /// let solution_six_pct = finance_solution::simple::loan_solution(200_000, 0.06, 180);
    ///
    /// // Create a variation on the first loan with a 5% APR.
    /// let solution_five_pct = solution_six_pct.with_apr(0.05);
    ///
    /// // Compare the two loans but don't show the monthly details.
    /// solution_six_pct.print_ab_comparison(&solution_five_pct, false);
    /// ```
    /// Output:
    /// ```text
    /// principal: 200_000.0000
    /// apr a: 0.060000
    /// apr b: 0.050000
    /// monthly_rate a: 0.005000
    /// monthly_rate b: 0.004167
    /// months: 180
    /// payment a: 1_687.7137
    /// payment b: 1_581.5873
    /// sum_of_payments a: 303_788.4581
    /// sum_of_payments b: 284_685.7056
    /// sum_of_interest a: 103_788.4581
    /// sum_of_interest b:  84_685.7056
    /// formula a: 1687.7137 = (200000.0000 * 1.005000^180 * 0.005000) / (1.005000^180 - 1)
    /// formula b: 1581.5873 = (200000.0000 * 1.004167^180 * 0.004167) / (1.004167^180 - 1)
    /// symbolic_formula: pmt = (P * (1+r)^n * r) / ((1 + r)^n - 1)
    /// ```
    pub fn with_apr(&self, apr: f64) -> LoanSolution {
        loan_solution(self.principal, apr, self.months)
    }

    /// Creates a variation on a loan which is the same except that it runs for a different length
    /// of time.
    ///
    /// # Examples
    /// ```
    /// // $200,000 at 6% APR for 15 years.
    /// let solution_15_years = finance_solution::simple::loan_solution(200_000, 0.06, 15 * 12);
    ///
    /// // Create a variation on the first loan that runs for 20 years.
    /// let solution_20_years = solution_15_years.with_months(20 * 12);
    ///
    /// // Compare the two loans but don't show the monthly details.
    /// solution_15_years.print_ab_comparison(&solution_20_years, false);
    /// ```
    /// Output (note that the second loan has a lower monthly payment but a higher sum of payments):
    /// ```text
    /// principal: 200_000.00
    /// apr: 0.060000
    /// monthly_rate: 0.005000
    /// months a: 180
    /// months b: 240
    /// payment a: 1_687.71
    /// payment b: 1_432.86
    /// sum_of_payments a: 303_788.46
    /// sum_of_payments b: 343_886.91
    /// sum_of_interest a: 103_788.46
    /// sum_of_interest b: 143_886.91
    /// formula a: 1687.7137 = (200000.0000 * 1.005000^180 * 0.005000) / (1.005000^180 - 1)
    /// formula b: 1432.8621 = (200000.0000 * 1.005000^240 * 0.005000) / (1.005000^240 - 1)
    /// symbolic_formula: pmt = (P * (1+r)^n * r) / ((1 + r)^n - 1)
    /// ```
    pub fn with_months(&self, months: u32) -> LoanSolution {
        loan_solution(self.principal, self.apr, months)
    }

    /// Creates a variation on a loan which is the same except that each month an extra payment goes
    /// directly toward paying down the remaining principal. This results in the loan being paid off
    /// earlier and with a smaller total amount of interest.
    pub fn with_extra_payment<T>(&self, _amount: T) -> LoanSolution
        where T: Into<f64> + Copy
    {
        unimplemented!();
    }

}

#[derive(Clone, Debug)]
pub struct LoanSeries {
    payment_series: PaymentSeries,
}

impl LoanSeries {
    pub(crate) fn new(payment_series: PaymentSeries) -> Self {
        Self {
            payment_series,
        }
    }

    /// Cuts down the month-by-month entries for the loan, usually for the sake of illustrating a
    /// result.
    ///
    /// # Examples
    /// ```
    /// let principal = 200_000;
    /// let series = finance_solution::simple::loan_solution(principal, 0.06, 180)
    ///     .series();
    ///
    /// // Print the first five and last five monthly entries.
    /// series.filter(|entry| entry.period() <= 5 || entry.period() > 175)
    ///     .print_table();
    ///
    /// // Print the monthly entries starting from the point where 95% of the principal is paid off.
    /// series.filter(|entry| entry.principal_to_date() >= principal as f64 * 0.95)
    ///     .print_table();
    /// ```
    pub fn filter<P>(&self, predicate: P) -> Self
        where P: Fn(&&CashflowPeriod) -> bool
    {
        Self::new(PaymentSeries::new(self.payment_series.filter(predicate)))
    }

    pub fn print_table(&self)
    {
        self.payment_series.print_table_locale_opt(true, true, None, Some(2));
    }

    pub fn print_table_custom(&self, include_running_totals: bool, include_remaining_amounts: bool)
    {
        self.payment_series.print_table_locale_opt(include_running_totals, include_remaining_amounts, None, Some(2));
    }

    pub fn print_table_locale(
        &self,
        include_running_totals: bool,
        include_remaining_amounts: bool,
        locale: &num_format::Locale,
        precision: usize) {
        self.payment_series.print_table_locale(include_running_totals, include_remaining_amounts, locale, precision);
    }

    pub fn print_ab_comparison(&self, other: &LoanSeries) {
        self.print_ab_comparison_custom(other, true, true);
    }

    pub fn print_ab_comparison_custom(
        &self,
        other: &LoanSeries,
        include_running_totals: bool,
        include_remaining_amounts: bool)
    {
        self.payment_series.print_ab_comparison_locale_opt(&other.payment_series, include_running_totals, include_remaining_amounts, None, Some(2));
    }

    pub fn print_ab_comparison_locale(
        &self,
        other: &LoanSeries,
        include_running_totals: bool,
        include_remaining_amounts: bool,
        locale: &num_format::Locale,
        precision: usize) {
        self.payment_series.print_ab_comparison_locale(&other.payment_series, include_running_totals, include_remaining_amounts, locale, precision);
    }

    /// Groups the month-by-month details of the loan into one entry per year. This is mostly used
    /// to print a summary of the loan as a double check.
    ///
    /// # Examples
    /// ```
    /// finance_solution::simple::loan_solution(200_000, 0.06, 180)
    ///     .series()
    ///     .summarize_by_year()
    ///     .print_table();
    /// ```
    pub fn summarize_by_year(&self) -> Self {
        LoanSeries::new(PaymentSeries::new(self.payment_series.summarize(12)))
    }

}

/// Calculates an amortized loan.
///
/// # Arguments
/// * `principal` - The amount of the loan given as a positive number.
/// * `apr` - The annual percentage rate. For instance 12% would be entered as `0.12`. This must be
/// a positive number.
/// * `months` - The number of periods for the loan. This must be at least one.
///
/// # Panics
/// The call will fail if `principal` or `apr` is not positive, or if `months` is zero.
///
/// # Examples
/// Calculate the payment and other details for an amortized loan then examine the formulas and the
/// month-by-month details such as the amount of the payment that goes to principal and interest.
/// ```
/// use finance_solution::*;
/// use finance_solution::simple::*;
///
/// // This is a mortgage loan for $200,000.
/// let principal = 200_000;
///
/// // The annual percentage rate is 6%
/// let apr = 0.06;
///
/// // The loan will be paid off in 15 years.
/// let months = 15 * 12;
///
/// let solution = loan_solution(principal, apr, months);
///
/// // Display the inputs, payment amount, formulas, sum of interest, etc.
/// dbg!(&solution);
/// ```
/// Output:
/// ```text
/// &solution = LoanSolution {
///     apr: 0.06,
///     monthly_rate: 0.005,
///     months: 180,
///     principal: 200000.0,
///     payment: 1687.7136560969248,
///     sum_of_payments: 303788.4580974465,
///     sum_of_interest: 103788.45809744648,
///     formula: "1687.7137 = (200000.0000 * 1.005000^180 * 0.005000) / (1.005000^180 - 1)",
///     symbolic_formula: "pmt = (P * (1+r)^n * r) / ((1 + r)^n - 1)",
/// }
/// ```
/// ```
/// # use finance_solution::*;
/// # use finance_solution::simple::*;
/// # let principal = 200_000;
/// # let months = 15 * 12;
/// # let solution = finance_solution::simple::loan_solution(principal, 0.06, months);
/// // The payment is $1,687.71/month.
/// assert_rounded_2!(solution.payment(), 1687.71);
///
/// // The sum of payments is simply the monthly payment times the number of months.
/// assert_rounded_2!(solution.sum_of_payments(), solution.payment() * months as f64);
/// assert_rounded_2!(solution.sum_of_payments(), 303_788.46);
///
/// // The sum of interest is the portion of the sum of payments that is over and above the original
/// // loan amount.
/// assert_rounded_2!(solution.sum_of_interest(), solution.sum_of_payments() - principal as f64);
/// assert_rounded_2!(solution.sum_of_interest(), 103_788.46);
///
/// // Calculate the month-by-month values including the amount of the payment that goes toward
/// // interest and principle as well as the running tally of the remaining amounts.
/// let series = solution.series();
/// // Note that all of the month-by-month values are shown as of the end of the period after that
/// // month's payment has been made.
/// dbg!(&series);
/// ```
/// Output (showing only the first period out of 180):
/// ```text
/// CashflowPeriod {
///     period: 1,
///     rate: 0.005,
///     due_at_beginning: false,
///     payment: 1687.7136560969248,
///     payments_to_date: 1687.7136560969248,
///     payments_remaining: 302100.7444413495,
///     principal: 687.7136560969248,
///     principal_to_date: 687.7136560969248,
///     principal_remaining: 199312.28634390308,
///     interest: 1000.0,
///     interest_to_date: 1000.0,
///     interest_remaining: 102788.45809744648,
///     formula: "1000.0000 = -(-200000.0000 * 0.005000)",
///     symbolic_formula: "interest = -(principal * rate)",
/// }
/// ```
/// ```
/// # use finance_solution::*;
/// # use finance_solution::simple::*;
/// # let solution = finance_solution::simple::loan_solution(200_000, 0.06, 180);
/// // Print the period-by-period values in a formatted table.
/// solution.print_table();
/// ```
/// Output (showing only the first five months):
/// ```text
/// period   payment  payments_to_date  payments_remaining  principal  principal_to_date  principal_remaining  interest  interest_to_date  interest_remaining
/// ------  --------  ----------------  ------------------  ---------  -----------------  -------------------  --------  ----------------  ------------------
///      1  1_687.71          1_687.71          302_100.74     687.71             687.71           199_312.29  1_000.00          1_000.00          102_788.46
///      2  1_687.71          3_375.43          300_413.03     691.15           1_378.87           198_621.13    996.56          1_996.56          101_791.90
///      3  1_687.71          5_063.14          298_725.32     694.61           2_073.47           197_926.53    993.11          2_989.67          100_798.79
///      4  1_687.71          6_750.85          297_037.60     698.08           2_771.55           197_228.45    989.63          3_979.30           99_809.16
///      5  1_687.71          8_438.57          295_349.89     701.57           3_473.13           196_526.87    986.14          4_965.44           98_823.02
/// ```
pub fn loan_solution<T>(principal: T, apr: f64, months: u32) -> LoanSolution
    where T: Into<f64> + Copy
{
    let principal = principal.into();

    assert!(principal > 0.0, "The principal must be a positive number.");
    assert!(apr > 0.0, "The APR must be a positive number.");
    assert!(months >= 1, "The number of months cannot be zero.");

    let payment = payment_solution_from_loan_arguments(principal, apr, months).payment();
    let rate = apr / 12.0;
    let rate_multiplier = 1.0 + rate;
    let formula = format!("{:.4} = ({:.4} * {:.6}^{} * {:.6}) / ({:.6}^{} - 1)", payment, principal, rate_multiplier, months, rate, rate_multiplier, months);
    let symbolic_formula = "pmt = (P * (1+r)^n * r) / ((1 + r)^n - 1)";
    let solution = LoanSolution::new(principal, apr, months, payment, &formula, symbolic_formula);
    // if RUN_PAYMENT_INVARIANTS {
    //    solution.invariant();
    //}
    solution
}

fn payment_solution_from_loan_arguments(principal: f64, apr: f64, months: u32) -> PaymentSolution {
    let rate = apr / 12.0;
    let periods = months;
    let present_value = -principal;
    let future_value = 0.0;
    let due_at_beginning= false;
    payment_solution(rate, periods, present_value, future_value, due_at_beginning)
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use crate::*;

}

