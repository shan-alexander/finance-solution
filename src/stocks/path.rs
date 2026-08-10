//! Price-path analysis: solution struct, period series, and pretty tables.
//!
//! # Conventions
//!
//! - Prices must be **strictly positive** for log returns, CAGR, and drawdowns.
//! - Volatility uses **sample** standard deviation (`n − 1`).
//! - Annualization: pass `periods_per_year` explicitly (e.g. `252.0` daily, `12.0` monthly).
//! - Sharpe / Sortino: excess return and volatility share the **same** period units.
//! - Max drawdown is a **positive fraction** (0.25 = 25% peak-to-trough).
use std::ops::Deref;

use crate::stocks::returns::{
    cagr, log_return, log_returns, mean_return, simple_return, simple_returns, total_return,
};
use crate::stocks::risk::{
    drawdown_series, rolling_max_drawdown, sharpe_ratio, sortino_ratio, volatility,
    volatility_annualized,
};
use crate::util::error::{require_finite, FinanceError, FinanceResult};
use crate::{columns_with_strings, print_table_locale_opt};

/// Kind of return used for mean / vol / Sharpe on the path.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum ReturnKind {
    #[default]
    Simple,
    Log,
}

impl std::fmt::Display for ReturnKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReturnKind::Simple => write!(f, "Simple"),
            ReturnKind::Log => write!(f, "Log"),
        }
    }
}

/// Options for [`price_path_solution`].
#[derive(Clone, Copy, Debug)]
pub struct PricePathOptions {
    /// Periods per year for annualizing volatility (e.g. 252, 12, 1).
    pub periods_per_year: f64,
    /// Calendar years spanned by the full path (for CAGR). If `None`, CAGR uses
    /// `(prices.len() - 1) / periods_per_year`.
    pub years: Option<f64>,
    /// Per-period risk-free rate for Sharpe (same units as period returns).
    pub risk_free_rate: f64,
    /// Target / MAR for Sortino (default 0.0).
    pub sortino_target: f64,
    /// Which return series drives mean/vol/Sharpe.
    pub return_kind: ReturnKind,
}

impl Default for PricePathOptions {
    fn default() -> Self {
        Self {
            periods_per_year: 252.0,
            years: None,
            risk_free_rate: 0.0,
            sortino_target: 0.0,
            return_kind: ReturnKind::Simple,
        }
    }
}

impl PricePathOptions {
    pub fn new(periods_per_year: f64) -> Self {
        Self {
            periods_per_year,
            ..Default::default()
        }
    }

    pub fn with_years(mut self, years: f64) -> Self {
        self.years = Some(years);
        self
    }

    pub fn with_risk_free(mut self, risk_free_rate: f64) -> Self {
        self.risk_free_rate = risk_free_rate;
        self
    }

    pub fn with_sortino_target(mut self, target: f64) -> Self {
        self.sortino_target = target;
        self
    }

    pub fn with_return_kind(mut self, kind: ReturnKind) -> Self {
        self.return_kind = kind;
        self
    }
}

/// Full analysis of an ordered price path.
///
/// Create with [`price_path_solution`].
///
/// # Examples
/// ```
/// use finance_solution::*;
///
/// let prices = [100.0, 110.0, 105.0, 120.0];
/// let opts = PricePathOptions::new(12.0).with_years(3.0 / 12.0);
/// let path = price_path_solution(&prices, opts).unwrap();
///
/// assert!(path.total_return() > 0.0);
/// assert_approx_equal!(path.max_drawdown(), max_drawdown(&prices).unwrap());
///
/// let series = path.series();
/// assert_eq!(series.len(), prices.len() - 1);
/// series.print_table();
/// ```
///
/// Sample `series().print_table()` (default formatting; columns match the live table):
///
/// ```text
/// period  price_start  price_end  simple_return  log_return  wealth_index  drawdown  roll_max_dd
/// ------  -----------  ---------  -------------  ----------  ------------  --------  -----------
///      1     100.0000   110.0000       0.100000    0.095310        1.1000  0.000000     0.000000
///      2     110.0000   105.0000      -0.045455   -0.046520        1.0500  0.045455     0.045455
///      3     105.0000   120.0000       0.142857    0.133531        1.2000  0.000000     0.045455
/// ```
///
/// Paths with only two prices still work for total return / CAGR / drawdown; sample
/// volatility / Sharpe / Sortino are `None` until there are at least two period returns
/// (three prices) and, for Sortino, at least one return below the target.
#[derive(Clone, Debug)]
pub struct PricePathSolution {
    prices: Vec<f64>,
    options: PricePathOptions,
    years: f64,
    total_return: f64,
    cagr: f64,
    mean_return: Option<f64>,
    volatility: Option<f64>,
    volatility_annualized: Option<f64>,
    sharpe_ratio: Option<f64>,
    sortino_ratio: Option<f64>,
    max_drawdown: f64,
    formula: String,
    symbolic_formula: String,
}

impl PricePathSolution {
    pub fn prices(&self) -> &[f64] {
        &self.prices
    }
    pub fn options(&self) -> &PricePathOptions {
        &self.options
    }
    pub fn n_prices(&self) -> usize {
        self.prices.len()
    }
    pub fn n_returns(&self) -> usize {
        self.prices.len().saturating_sub(1)
    }
    /// Years used for CAGR.
    pub fn years(&self) -> f64 {
        self.years
    }
    pub fn total_return(&self) -> f64 {
        self.total_return
    }
    pub fn cagr(&self) -> f64 {
        self.cagr
    }
    /// `None` if fewer than one return (should not happen for valid paths).
    pub fn mean_return(&self) -> Option<f64> {
        self.mean_return
    }
    /// `None` if fewer than two returns (sample vol undefined).
    pub fn volatility(&self) -> Option<f64> {
        self.volatility
    }
    /// `None` if sample volatility is undefined.
    pub fn volatility_annualized(&self) -> Option<f64> {
        self.volatility_annualized
    }
    /// `None` if volatility undefined or zero.
    pub fn sharpe_ratio(&self) -> Option<f64> {
        self.sharpe_ratio
    }
    /// `None` if no downside observations vs target (or vol path too short).
    pub fn sortino_ratio(&self) -> Option<f64> {
        self.sortino_ratio
    }
    pub fn max_drawdown(&self) -> f64 {
        self.max_drawdown
    }
    pub fn formula(&self) -> &str {
        &self.formula
    }
    pub fn symbolic_formula(&self) -> &str {
        &self.symbolic_formula
    }

    /// Simple returns along the path.
    pub fn simple_returns(&self) -> FinanceResult<Vec<f64>> {
        simple_returns(&self.prices)
    }

    /// Log returns along the path.
    pub fn log_returns(&self) -> FinanceResult<Vec<f64>> {
        log_returns(&self.prices)
    }

    /// Period-by-period detail (length `n_prices - 1`).
    pub fn series(&self) -> PricePathSeries {
        // Internal invariant: solution construction already validated all prices.
        build_series(&self.prices).expect("validated PricePathSolution prices")
    }

    /// Summary metrics as a small table (not the period series).
    ///
    /// Optional stats (vol / Sharpe / Sortino) print as `n/a` when undefined — e.g. fewer
    /// than two period returns, or no downside observations for Sortino.
    ///
    /// # Examples
    /// ```
    /// use finance_solution::*;
    ///
    /// let path = price_path_solution(&[100.0, 110.0, 105.0], PricePathOptions::default()).unwrap();
    /// path.print_summary();
    ///
    /// // Two prices: total return works; sample vol is n/a (only one return).
    /// let short = price_path_solution(&[100.0, 110.0], PricePathOptions::default()).unwrap();
    /// assert!(short.volatility().is_none());
    /// short.print_summary(); // must not panic
    /// ```
    ///
    /// Sample output (three+ prices, with vol):
    ///
    /// ```text
    ///         metric    value
    /// --------------  -------
    ///       n_prices       10
    ///          years   0.8333
    ///   total_return   0.2500
    ///           cagr   0.3070
    ///    mean_return   0.0261
    ///     volatility   0.0477
    /// volatility_ann   0.1653
    ///         sharpe   0.5469
    ///        sortino   1.7235
    ///   max_drawdown   0.0278
    /// ```
    pub fn print_summary(&self) {
        self.print_summary_locale_opt(None, None);
    }

    /// Locale-aware [`print_summary`](Self::print_summary).
    pub fn print_summary_locale(&self, locale: &num_format::Locale, precision: usize) {
        self.print_summary_locale_opt(Some(locale), Some(precision));
    }

    fn print_summary_locale_opt(
        &self,
        locale: Option<&num_format::Locale>,
        precision: Option<usize>,
    ) {
        // Value column is type "s" so optional metrics can show "n/a" without parse panics.
        // Numeric cells are pre-formatted when a locale/precision is requested.
        let columns = columns_with_strings(&[("metric", "s", true), ("value", "s", true)]);
        let fmt_num = |v: f64| -> String {
            match (locale, precision) {
                (Some(loc), Some(prec)) => crate::format_float_locale_opt(v, Some(loc), Some(prec)),
                (Some(loc), None) => crate::format_float_locale_opt(v, Some(loc), Some(4)),
                (None, Some(prec)) => crate::format_float_locale_opt(v, None, Some(prec)),
                (None, None) => crate::format_float_locale_opt(v, None, Some(4)),
            }
        };
        let opt_num = |v: Option<f64>| v.map(fmt_num).unwrap_or_else(|| "n/a".into());
        let data = vec![
            vec!["n_prices".into(), self.prices.len().to_string()],
            vec!["years".into(), fmt_num(self.years)],
            vec!["total_return".into(), fmt_num(self.total_return)],
            vec!["cagr".into(), fmt_num(self.cagr)],
            vec!["mean_return".into(), opt_num(self.mean_return)],
            vec!["volatility".into(), opt_num(self.volatility)],
            vec!["volatility_ann".into(), opt_num(self.volatility_annualized)],
            vec!["sharpe".into(), opt_num(self.sharpe_ratio)],
            vec!["sortino".into(), opt_num(self.sortino_ratio)],
            vec!["max_drawdown".into(), fmt_num(self.max_drawdown)],
        ];
        print_table_locale_opt(&columns, data, locale, precision);
    }

    /// Alias: print period series table with running wealth/drawdown columns.
    pub fn print_table(&self) {
        self.series().print_table();
    }
}

/// One step between consecutive prices.
#[derive(Clone, Debug)]
pub struct PricePathPeriod {
    period: u32,
    price_start: f64,
    price_end: f64,
    simple_return: f64,
    log_return: f64,
    /// Cumulative wealth of $1 invested at the start of the path, after this step.
    wealth_index: f64,
    /// Drawdown at `price_end` vs running peak along the full path.
    drawdown: f64,
    /// Running maximum drawdown from the start through `price_end`.
    rolling_max_drawdown: f64,
    formula: String,
    symbolic_formula: String,
}

impl PricePathPeriod {
    pub fn period(&self) -> u32 {
        self.period
    }
    pub fn price_start(&self) -> f64 {
        self.price_start
    }
    pub fn price_end(&self) -> f64 {
        self.price_end
    }
    pub fn simple_return(&self) -> f64 {
        self.simple_return
    }
    pub fn log_return(&self) -> f64 {
        self.log_return
    }
    pub fn wealth_index(&self) -> f64 {
        self.wealth_index
    }
    pub fn drawdown(&self) -> f64 {
        self.drawdown
    }
    pub fn rolling_max_drawdown(&self) -> f64 {
        self.rolling_max_drawdown
    }
    pub fn formula(&self) -> &str {
        &self.formula
    }
    pub fn symbolic_formula(&self) -> &str {
        &self.symbolic_formula
    }
}

/// Period series for a price path. Derefs to `[PricePathPeriod]`.
#[derive(Clone, Debug)]
pub struct PricePathSeries(Vec<PricePathPeriod>);

impl PricePathSeries {
    pub fn filter<P>(&self, predicate: P) -> Self
    where
        P: Fn(&&PricePathPeriod) -> bool,
    {
        Self(self.iter().filter(|x| predicate(x)).cloned().collect())
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
            ("period", "i", true),
            ("price_start", "f", true),
            ("price_end", "f", true),
            ("simple_return", "r", true),
            ("log_return", "r", true),
            ("wealth_index", "f", true),
            ("drawdown", "r", true),
            ("roll_max_dd", "r", true),
        ]);
        let data = self
            .iter()
            .map(|e| {
                vec![
                    e.period.to_string(),
                    e.price_start.to_string(),
                    e.price_end.to_string(),
                    e.simple_return.to_string(),
                    e.log_return.to_string(),
                    e.wealth_index.to_string(),
                    e.drawdown.to_string(),
                    e.rolling_max_drawdown.to_string(),
                ]
            })
            .collect();
        print_table_locale_opt(&columns, data, locale, precision);
    }
}

impl Deref for PricePathSeries {
    type Target = Vec<PricePathPeriod>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Build a [`PricePathSolution`] summarizing returns, risk, and period detail for a price path.
///
/// # Examples
/// ```
/// use finance_solution::{price_path_solution, PricePathOptions, FinanceError};
///
/// match price_path_solution(&[100.0, 110.0], PricePathOptions::default()) {
///     Ok(path) => assert!(path.total_return() > 0.0),
///     Err(FinanceError::Unsolvable { message }) => panic!("{message}"),
///     Err(e) => panic!("{e}"),
/// }
///
/// assert!(matches!(
///     price_path_solution(&[100.0], PricePathOptions::default()),
///     Err(FinanceError::Unsolvable { .. })
/// ));
/// ```
pub fn price_path_solution(
    prices: &[f64],
    options: PricePathOptions,
) -> FinanceResult<PricePathSolution> {
    require_finite("periods_per_year", options.periods_per_year)?;
    if options.periods_per_year <= 0.0 {
        return Err(FinanceError::Unsolvable {
            message: "periods_per_year must be positive",
        });
    }
    require_finite("risk_free_rate", options.risk_free_rate)?;
    require_finite("sortino_target", options.sortino_target)?;

    if prices.len() < 2 {
        return Err(FinanceError::Unsolvable {
            message: "price_path_solution requires at least two prices",
        });
    }
    for &p in prices {
        require_finite("prices", p)?;
        if p <= 0.0 {
            return Err(FinanceError::InvalidCashflow {
                message: "price_path_solution requires strictly positive prices",
            });
        }
    }

    let start = prices[0];
    let end = prices[prices.len() - 1];
    let n_steps = (prices.len() - 1) as f64;
    let years = match options.years {
        Some(y) => {
            require_finite("years", y)?;
            if y <= 0.0 {
                return Err(FinanceError::Unsolvable {
                    message: "years must be positive when provided",
                });
            }
            y
        }
        None => n_steps / options.periods_per_year,
    };

    let total_return = total_return(start, end)?;
    let cagr = cagr(start, end, years)?;

    let simple = simple_returns(prices)?;
    let log = log_returns(prices)?;
    let series_for_stats = match options.return_kind {
        ReturnKind::Simple => &simple[..],
        ReturnKind::Log => &log[..],
    };

    let mean_return = mean_return(series_for_stats).ok();
    let volatility = volatility(series_for_stats).ok();
    let volatility_annualized = volatility
        .and_then(|_| volatility_annualized(series_for_stats, options.periods_per_year).ok());
    let sharpe_ratio = sharpe_ratio(series_for_stats, options.risk_free_rate).ok();
    let sortino_ratio = sortino_ratio(series_for_stats, options.sortino_target).ok();
    let max_drawdown = drawdown_series(prices)?.into_iter().fold(0.0_f64, f64::max);

    let vol_s = volatility
        .map(|v| format!("{v:.6}"))
        .unwrap_or_else(|| "n/a".into());
    let formula = format!(
        "total_return {:.6} = ({:.4} - {:.4}) / {:.4}; cagr {:.6} over {:.4}y; vol {}; max_dd {:.6}",
        total_return, end, start, start, cagr, years, vol_s, max_drawdown
    );
    let symbolic = "total_return = (P_n - P_0)/P_0; cagr = (P_n/P_0)^(1/years)-1; vol = sample_stdev(r); max_dd = max((peak-p)/peak)";

    Ok(PricePathSolution {
        prices: prices.to_vec(),
        options,
        years,
        total_return,
        cagr,
        mean_return,
        volatility,
        volatility_annualized,
        sharpe_ratio,
        sortino_ratio,
        max_drawdown,
        formula,
        symbolic_formula: symbolic.to_string(),
    })
}

fn build_series(prices: &[f64]) -> FinanceResult<PricePathSeries> {
    let drawdowns = drawdown_series(prices)?;
    let rolling = rolling_max_drawdown(prices)?;
    let mut rows = Vec::with_capacity(prices.len() - 1);
    let mut wealth = 1.0_f64;

    for i in 0..prices.len() - 1 {
        let p0 = prices[i];
        let p1 = prices[i + 1];
        let sr = simple_return(p0, p1)?;
        let lr = log_return(p0, p1)?;
        wealth *= 1.0 + sr;
        let formula = format!("{:.6} = ({:.4} - {:.4}) / {:.4}", sr, p1, p0, p0);
        rows.push(PricePathPeriod {
            period: (i + 1) as u32,
            price_start: p0,
            price_end: p1,
            simple_return: sr,
            log_return: lr,
            wealth_index: wealth,
            drawdown: drawdowns[i + 1],
            rolling_max_drawdown: rolling[i + 1],
            formula,
            symbolic_formula: "r = (P_t - P_{t-1}) / P_{t-1}".to_string(),
        });
    }
    Ok(PricePathSeries(rows))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_path_solution_basic() {
        let prices = [100.0, 110.0, 105.0, 120.0];
        let path =
            price_path_solution(&prices, PricePathOptions::new(12.0).with_years(0.25)).unwrap();
        assert_approx_equal!(path.total_return(), 0.20);
        // Peak 110 → 105 is the only drawdown.
        assert_approx_equal!(path.max_drawdown(), 5.0 / 110.0);

        let series = path.series();
        assert_eq!(series.len(), 3);
        assert_approx_equal!(series[0].simple_return(), 0.10);
        assert_approx_equal!(series[2].wealth_index(), 1.20);
        assert!(!path.formula().is_empty());
    }

    #[test]
    fn test_path_rejects_short_or_nonpositive() {
        assert!(matches!(
            price_path_solution(&[100.0], PricePathOptions::default()),
            Err(FinanceError::Unsolvable { .. })
        ));
        assert!(matches!(
            price_path_solution(&[100.0, -1.0], PricePathOptions::default()),
            Err(FinanceError::InvalidCashflow { .. })
        ));
    }

    #[test]
    fn test_round_trip_wealth() {
        let prices = [50.0, 55.0, 52.25, 60.0];
        let path = price_path_solution(&prices, PricePathOptions::default()).unwrap();
        let last_w = path.series().last().unwrap().wealth_index();
        assert_approx_equal!(last_w, prices[prices.len() - 1] / prices[0]);
    }
}
