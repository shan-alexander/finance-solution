//! Approximate years to double (Rule of 72 / 69 / 70) and exact doubling time.
//!
//! Prefer [`doubling_solution`] for formulas, comparison tables, and multi-rate scenarios.
//! Prefer [`doubling_time`] when you only need the exact discrete result.
//!
//! # Error handling (v0.1+)
//!
//! All public entry points return [`FinanceResult`]. Zero or negative rates and non-finite
//! inputs yield [`FinanceError`] — they do not panic.
use crate::util::error::{require_finite, FinanceError, FinanceResult};
use crate::{columns_with_strings, print_table_locale_opt};

// ---------------------------------------------------------------------------
// Scalars
// ---------------------------------------------------------------------------

/// Approximate years to double using the Rule of 72: `72 / (100 * rate)`.
///
/// `rate` is a decimal rate (e.g. `0.08` for 8%).
///
/// # Errors
/// [`FinanceError::ZeroValue`] if `rate == 0`, [`FinanceError::InvalidRate`] if `rate < 0`,
/// or [`FinanceError::NonFinite`] if non-finite.
///
/// # Examples
/// ```
/// use finance_solution::{rule_of_72, FinanceError};
///
/// assert_eq!((rule_of_72(0.08).unwrap() * 100.0).round() / 100.0, 9.0); // 72/8
///
/// match rule_of_72(0.0) {
///     Err(FinanceError::ZeroValue { field }) => assert_eq!(field, "rate"),
///     other => panic!("expected ZeroValue, got {other:?}"),
/// }
/// ```
pub fn rule_of_72(rate: f64) -> FinanceResult<f64> {
    require_positive_rate(rate)?;
    Ok(72.0 / (rate * 100.0))
}

/// Approximate years to double using the Rule of 70: `70 / (100 * rate)`.
///
/// # Errors
/// Same domain as [`rule_of_72`].
///
/// # Examples
/// ```
/// use finance_solution::{rule_of_70, FinanceError};
///
/// assert!((rule_of_70(0.08).unwrap() - 8.75).abs() < 1e-12); // 70/8
/// assert!(matches!(rule_of_70(0.0), Err(FinanceError::ZeroValue { .. })));
/// ```
pub fn rule_of_70(rate: f64) -> FinanceResult<f64> {
    require_positive_rate(rate)?;
    Ok(70.0 / (rate * 100.0))
}

/// Approximate years to double using the Rule of 69: `69 / (100 * rate)`.
///
/// Often closer to continuous compounding than the Rule of 72.
///
/// # Errors
/// Same domain as [`rule_of_72`].
///
/// # Examples
/// ```
/// use finance_solution::{rule_of_69, FinanceError};
///
/// assert!((rule_of_69(0.08).unwrap() - 8.625).abs() < 1e-12); // 69/8
/// match rule_of_69(-0.01) {
///     Err(FinanceError::InvalidRate { rate }) => assert!(rate < 0.0),
///     other => panic!("expected InvalidRate, got {other:?}"),
/// }
/// ```
pub fn rule_of_69(rate: f64) -> FinanceResult<f64> {
    require_positive_rate(rate)?;
    Ok(69.0 / (rate * 100.0))
}

/// Exact periods to double under discrete compounding: `ln(2) / ln(1 + rate)`.
///
/// # Errors
/// [`FinanceError::InvalidRate`] if `rate <= 0`, or non-finite / unsolvable rates.
///
/// # Examples
/// ```
/// use finance_solution::{doubling_time, rule_of_72, FinanceError};
///
/// let exact = doubling_time(0.08).unwrap();
/// let approx = rule_of_72(0.08).unwrap();
/// assert!((exact - approx).abs() < 0.5);
/// assert!((exact - 9.0065).abs() < 1e-3);
///
/// match doubling_time(-0.05) {
///     Err(FinanceError::InvalidRate { rate }) => assert!(rate < 0.0),
///     other => panic!("expected InvalidRate, got {other:?}"),
/// }
/// ```
pub fn doubling_time(rate: f64) -> FinanceResult<f64> {
    require_finite("rate", rate)?;
    if rate <= 0.0 {
        return Err(FinanceError::InvalidRate { rate });
    }
    let denom = (1.0 + rate).ln();
    if denom == 0.0 || !denom.is_finite() {
        return Err(FinanceError::Unsolvable {
            message: "cannot compute doubling time for this rate",
        });
    }
    Ok(2.0_f64.ln() / denom)
}

/// Exact time to double under continuous compounding: `ln(2) / rate`.
///
/// Slightly shorter than discrete [`doubling_time`] for the same nominal rate.
///
/// # Errors
/// Same domain as [`rule_of_72`] (strictly positive finite rate).
///
/// # Examples
/// ```
/// use finance_solution::{doubling_time, doubling_time_continuous, FinanceError};
///
/// let cont = doubling_time_continuous(0.08).unwrap();
/// let disc = doubling_time(0.08).unwrap();
/// assert!(cont < disc);
/// assert!(matches!(
///     doubling_time_continuous(0.0),
///     Err(FinanceError::ZeroValue { .. })
/// ));
/// ```
pub fn doubling_time_continuous(rate: f64) -> FinanceResult<f64> {
    require_positive_rate(rate)?;
    Ok(2.0_f64.ln() / rate)
}

// ---------------------------------------------------------------------------
// Solution
// ---------------------------------------------------------------------------

/// Comparison of doubling-time methods at a single rate.
///
/// Create with [`doubling_solution`].
///
/// # Examples
/// ```
/// use finance_solution::*;
///
/// let s = doubling_solution(0.08).unwrap();
/// assert_rounded_2!(s.rule_of_72(), 9.0);
/// assert_rounded_4!(s.exact(), 9.0065);
/// // Table of method vs years (and error vs exact):
/// s.print_table();
/// ```
///
/// Sample `print_table()` output at 8%:
///
/// ```text
/// method               years      error_vs_exact
/// ---------------  ---------  ----------------
/// rule_of_72          9.0000           -0.0065
/// rule_of_70          8.7500           -0.2565
/// rule_of_69          8.6250           -0.3815
/// exact_discrete      9.0065            0.0000
/// exact_continuous    8.6643           -0.3421
/// ```
#[derive(Clone, Debug)]
pub struct DoublingSolution {
    rate: f64,
    rule_of_72: f64,
    rule_of_70: f64,
    rule_of_69: f64,
    exact: f64,
    exact_continuous: f64,
    formula: String,
    symbolic_formula: String,
}

impl DoublingSolution {
    pub fn rate(&self) -> f64 {
        self.rate
    }
    pub fn rule_of_72(&self) -> f64 {
        self.rule_of_72
    }
    pub fn rule_of_70(&self) -> f64 {
        self.rule_of_70
    }
    pub fn rule_of_69(&self) -> f64 {
        self.rule_of_69
    }
    /// Exact discrete compounding: `ln(2) / ln(1+r)`.
    pub fn exact(&self) -> f64 {
        self.exact
    }
    /// Exact continuous compounding: `ln(2) / r`.
    pub fn exact_continuous(&self) -> f64 {
        self.exact_continuous
    }
    pub fn formula(&self) -> &str {
        &self.formula
    }
    pub fn symbolic_formula(&self) -> &str {
        &self.symbolic_formula
    }

    /// Absolute error of Rule of 72 vs exact discrete doubling time.
    pub fn error_rule_of_72(&self) -> f64 {
        self.rule_of_72 - self.exact
    }

    /// Print method comparison at this rate.
    pub fn print_table(&self) {
        self.print_table_locale_opt(None, None);
    }

    pub fn print_table_locale(&self, locale: &num_format::Locale, precision: usize) {
        self.print_table_locale_opt(Some(locale), Some(precision));
    }

    fn print_table_locale_opt(
        &self,
        locale: Option<&num_format::Locale>,
        precision: Option<usize>,
    ) {
        let columns = columns_with_strings(&[
            ("method", "s", true),
            ("years", "f", true),
            ("error_vs_exact", "f", true),
        ]);
        let rows = [
            ("rule_of_72", self.rule_of_72),
            ("rule_of_70", self.rule_of_70),
            ("rule_of_69", self.rule_of_69),
            ("exact_discrete", self.exact),
            ("exact_continuous", self.exact_continuous),
        ];
        let data = rows
            .iter()
            .map(|(name, years)| {
                vec![
                    name.to_string(),
                    years.to_string(),
                    (years - self.exact).to_string(),
                ]
            })
            .collect();
        print_table_locale_opt(&columns, data, locale, precision);
    }
}

/// Build a [`DoublingSolution`] comparing Rule of 72/70/69 with exact discrete and continuous times.
///
/// # Errors
/// Propagates validation failures from the underlying doubling formulas.
///
/// # Examples
/// ```
/// use finance_solution::{doubling_solution, FinanceError};
///
/// let s = doubling_solution(0.10).unwrap();
/// // Symmetry: $1 grown at r for exact periods ≈ $2
/// let grown = (1.0_f64 + s.rate()).powf(s.exact());
/// assert!((grown - 2.0).abs() < 1e-9);
///
/// match doubling_solution(-0.05) {
///     Err(FinanceError::InvalidRate { .. }) => {}
///     other => panic!("expected InvalidRate, got {other:?}"),
/// }
/// ```
pub fn doubling_solution(rate: f64) -> FinanceResult<DoublingSolution> {
    let r72 = rule_of_72(rate)?;
    let r70 = rule_of_70(rate)?;
    let r69 = rule_of_69(rate)?;
    let exact = doubling_time(rate)?;
    let exact_c = doubling_time_continuous(rate)?;
    let formula = format!(
        "exact {:.4} = ln(2) / ln(1 + {:.6}); rule_72 {:.4} = 72 / ({:.4})",
        exact,
        rate,
        r72,
        rate * 100.0
    );
    let symbolic =
        "exact = ln(2)/ln(1+r); rule_72 = 72/(100*r); rule_70 = 70/(100*r); rule_69 = 69/(100*r); continuous = ln(2)/r";
    Ok(DoublingSolution {
        rate,
        rule_of_72: r72,
        rule_of_70: r70,
        rule_of_69: r69,
        exact,
        exact_continuous: exact_c,
        formula,
        symbolic_formula: symbolic.to_string(),
    })
}

// ---------------------------------------------------------------------------
// Multi-rate scenario table
// ---------------------------------------------------------------------------

/// One row of a multi-rate doubling comparison.
#[derive(Clone, Debug)]
pub struct DoublingRateRow {
    rate: f64,
    rule_of_72: f64,
    rule_of_70: f64,
    rule_of_69: f64,
    exact: f64,
    exact_continuous: f64,
}

impl DoublingRateRow {
    pub fn rate(&self) -> f64 {
        self.rate
    }
    pub fn rule_of_72(&self) -> f64 {
        self.rule_of_72
    }
    pub fn rule_of_70(&self) -> f64 {
        self.rule_of_70
    }
    pub fn rule_of_69(&self) -> f64 {
        self.rule_of_69
    }
    pub fn exact(&self) -> f64 {
        self.exact
    }
    pub fn exact_continuous(&self) -> f64 {
        self.exact_continuous
    }
    pub fn error_rule_of_72(&self) -> f64 {
        self.rule_of_72 - self.exact
    }
}

/// Table of doubling estimates across many rates (teaching: when is Rule of 72 “good enough?”).
#[derive(Clone, Debug)]
pub struct DoublingRateSeries(Vec<DoublingRateRow>);

impl DoublingRateSeries {
    pub fn rows(&self) -> &[DoublingRateRow] {
        &self.0
    }

    pub fn print_table(&self) {
        self.print_table_locale_opt(None, None);
    }

    pub fn print_table_locale(&self, locale: &num_format::Locale, precision: usize) {
        self.print_table_locale_opt(Some(locale), Some(precision));
    }

    fn print_table_locale_opt(
        &self,
        locale: Option<&num_format::Locale>,
        precision: Option<usize>,
    ) {
        let columns = columns_with_strings(&[
            ("rate", "r", true),
            ("rule_72", "f", true),
            ("rule_70", "f", true),
            ("rule_69", "f", true),
            ("exact", "f", true),
            ("continuous", "f", true),
            ("err_72", "f", true),
        ]);
        let data = self
            .0
            .iter()
            .map(|row| {
                vec![
                    row.rate.to_string(),
                    row.rule_of_72.to_string(),
                    row.rule_of_70.to_string(),
                    row.rule_of_69.to_string(),
                    row.exact.to_string(),
                    row.exact_continuous.to_string(),
                    row.error_rule_of_72().to_string(),
                ]
            })
            .collect();
        print_table_locale_opt(&columns, data, locale, precision);
    }
}

/// Compare doubling rules across a list of rates (teaching: when is Rule of 72 “good enough?”).
///
/// # Examples
/// ```
/// use finance_solution::{doubling_compare_rates, FinanceError};
///
/// let table = doubling_compare_rates(&[0.05, 0.08]).unwrap();
/// assert_eq!(table.rows().len(), 2);
///
/// match doubling_compare_rates(&[]) {
///     Err(FinanceError::Unsolvable { message }) => assert!(message.contains("one rate")),
///     other => panic!("expected Unsolvable, got {other:?}"),
/// }
/// match doubling_compare_rates(&[0.05, -0.01]) {
///     Err(FinanceError::InvalidRate { .. }) => {}
///     other => panic!("expected InvalidRate, got {other:?}"),
/// }
/// ```
pub fn doubling_compare_rates(rates: &[f64]) -> FinanceResult<DoublingRateSeries> {
    if rates.is_empty() {
        return Err(FinanceError::Unsolvable {
            message: "doubling_compare_rates requires at least one rate",
        });
    }
    let mut rows = Vec::with_capacity(rates.len());
    for &rate in rates {
        let s = doubling_solution(rate)?;
        rows.push(DoublingRateRow {
            rate: s.rate,
            rule_of_72: s.rule_of_72,
            rule_of_70: s.rule_of_70,
            rule_of_69: s.rule_of_69,
            exact: s.exact,
            exact_continuous: s.exact_continuous,
        });
    }
    Ok(DoublingRateSeries(rows))
}

fn require_positive_rate(rate: f64) -> FinanceResult<()> {
    require_finite("rate", rate)?;
    if rate == 0.0 {
        return Err(FinanceError::ZeroValue { field: "rate" });
    }
    if rate < 0.0 {
        return Err(FinanceError::InvalidRate { rate });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_rule_of_72_eight_percent() {
        assert_rounded_2!(rule_of_72(0.08).unwrap(), 9.0);
        assert_rounded_2!(rule_of_70(0.08).unwrap(), 8.75);
        assert_rounded_2!(rule_of_69(0.08).unwrap(), 8.625);
    }

    #[test]
    fn test_doubling_time_positive() {
        let t = doubling_time(0.10).unwrap();
        assert!(t > 7.0 && t < 8.0);
        assert!(doubling_time_continuous(0.10).unwrap() < t);
    }

    #[test]
    fn test_rule_rejects_zero_and_negative() {
        assert!(matches!(
            rule_of_72(0.0),
            Err(FinanceError::ZeroValue { .. })
        ));
        assert!(matches!(
            doubling_time(-0.05),
            Err(FinanceError::InvalidRate { .. })
        ));
        assert!(doubling_compare_rates(&[]).is_err());
    }

    #[test]
    fn test_solution_and_symmetry() {
        let s = doubling_solution(0.08).unwrap();
        assert_rounded_2!(s.rule_of_72(), 9.0);
        assert_rounded_4!(s.exact(), 9.0065);
        let grown = (1.0 + s.rate()).powf(s.exact());
        assert!((grown - 2.0).abs() < 1e-9);
        assert!(!s.formula().is_empty());
    }

    #[test]
    fn test_compare_rates() {
        let t = doubling_compare_rates(&[0.01, 0.08, 0.12]).unwrap();
        assert_eq!(t.rows().len(), 3);
        // At low rates rule_72 overstates years vs exact.
        assert!(t.rows()[0].error_rule_of_72() > 0.0);
    }
}
