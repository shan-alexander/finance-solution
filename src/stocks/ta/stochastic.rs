//! Stochastic oscillator — **one core**, many packs via [`StochasticParams`].
//!
//! # Fast vs Full
//!
//! Not two formulas: **Full** is Fast with extra `%K` smoothing.
//!
//! | Style | Params | Meaning |
//! |-------|--------|---------|
//! | Fast | `k_smooth = 1` | Raw %K; %D = SMA(%K, d) |
//! | Full | `k_smooth > 1` | %K = SMA(raw %K, k_smooth); %D = SMA(%K, d) |
//!
//! # Quant pattern — `const` pack + validated engine + `.compute`
//!
//! This is the **recommended** way for production code that repeatedly runs the same
//! stochastic variation. Build the pack once (often as a `const`), validate once into
//! [`ValidatedStochastic`], then call [`.compute`](ValidatedStochastic::compute) on each
//! new H/L/C batch. Construction is O(1); the O(n) work is only the series math.
//!
//! ```
//! use finance_solution::stocks::ta::{StochasticParams, ValidatedStochastic};
//!
//! // 1) Strategy definition — fixed pack, zero heap, can live at module scope:
//! const FAST_9_3: StochasticParams = StochasticParams::fast(9, 3);
//! // Other common packs:
//! // const FAST_14_3: StochasticParams = StochasticParams::fast(14, 3);
//! // const FULL_14_3_3: StochasticParams = StochasticParams::full(14, 3, 3);
//! // const FULL_60_10_1: StochasticParams = StochasticParams::full(60, 10, 1);
//!
//! // 2) Validate once at startup (period ≥ 1 checks):
//! let stoch = ValidatedStochastic::new(FAST_9_3).unwrap();
//!
//! // 3) Hot path — many batches / symbols reuse `stoch`:
//! # let h = vec![10.0; 20];
//! # let l = vec![9.0; 20];
//! # let c = vec![9.5; 20];
//! let series = stoch.compute(&h, &l, &c).unwrap();
//! assert_eq!(series.k.len(), h.len());
//! // series.k / series.d are Option<f64> with warm-up = None
//! ```
//!
//! Free function form (scripts / one-offs) is fine too — still uses the same `Copy` pack:
//!
//! ```
//! use finance_solution::stocks::ta::{stochastics, StochasticParams};
//! const FAST_9_3: StochasticParams = StochasticParams::fast(9, 3);
//! # let h = [11.0_f64; 15];
//! # let l = [10.0; 15];
//! # let c = [10.5; 15];
//! let _ = stochastics(&h, &l, &c, FAST_9_3).unwrap();
//! ```
//!
//! Sample [`stochastics_solution`] table (illustrative):
//!
//! ```text
//! period   close      k      d
//! ------  ------  -----  -----
//!      7   19.50    n/a    n/a
//!      8   19.60  72.00    n/a
//!     10   19.80  68.00  70.00
//! ```
//!
//! ## Flat window (highest high == lowest low)
//!
//! When the lookback range is zero, `%K = 100 * (C − LL) / (HH − LL)` is undefined.
//!
//! | Policy | Pros | Cons |
//! |--------|------|------|
//! | Always **50** | Simple | Fake “neutral” every flat bar; can invent mean-reversion noise |
//! | **`None` / skip** | Honest | Holes in the series after warm-up; breaks some smoothers |
//! | **Carry previous raw %K**, else **50** on the first flat | Continuous series; no spurious 50 flip-flops | Still conventional when no history |
//!
//! **This crate uses carry-forward (else 50).** Batch and [`StochState`] share the rule so live
//! and research match. Documented so you can wrap with a different policy if your desk requires it.
//!
//! Batch [`stochastics`] / [`ValidatedStochastic::compute`] is **the same path** as live state:
//! `StochState::new` + [`StochState::push_bars`] (amortized O(1) HH/LL + ring SMAs).
//!
use crate::stocks::ta::common::opt_cell;
use crate::stocks::ta::state::StochState;
use crate::util::error::FinanceResult;
use crate::util::primitives::PeriodLength;
use crate::{columns_with_strings, print_table_locale_opt};

/// Unvalidated (but `Copy`) stochastic parameter pack.
///
/// Build with [`StochasticParams::fast`], [`StochasticParams::full`], or struct update.
/// Prefer validating once via [`ValidatedStochastic::new`] for hot paths.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct StochasticParams {
    /// Lookback for highest high / lowest low.
    pub k_period: usize,
    /// SMA length on raw %K (`1` = Fast stochastic).
    pub k_smooth: usize,
    /// SMA length on smoothed %K → %D line.
    pub d_period: usize,
}

impl StochasticParams {
    /// Fast stochastic: raw %K over `k_period`, %D = SMA(`d_period`) of %K.
    ///
    /// Common packs: `fast(9, 3)`, `fast(14, 3)`.
    pub const fn fast(k_period: usize, d_period: usize) -> Self {
        Self {
            k_period,
            k_smooth: 1,
            d_period,
        }
    }

    /// Full stochastic: smooth raw %K by `k_smooth`, then %D by `d_period`.
    ///
    /// Common packs: `full(14, 3, 3)`, `full(60, 10, 1)`.
    pub const fn full(k_period: usize, k_smooth: usize, d_period: usize) -> Self {
        Self {
            k_period,
            k_smooth,
            d_period,
        }
    }

    /// Minimum bars before both %K and %D can be defined.
    pub const fn warm_up_bars(self) -> usize {
        // first raw %K at k_period-1; need k_smooth-1 more for smooth K; d_period-1 more for D
        self.k_period
            .saturating_add(self.k_smooth.saturating_sub(1))
            .saturating_add(self.d_period.saturating_sub(1))
    }
}

/// Params that passed period validation — safe to use in a tight loop.
///
/// Construction is O(1). [`ValidatedStochastic::compute`] is O(n) pure math.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ValidatedStochastic {
    params: StochasticParams,
}

impl ValidatedStochastic {
    /// Validate all periods `≥ 1`.
    pub fn new(params: StochasticParams) -> FinanceResult<Self> {
        PeriodLength::new(params.k_period)?;
        PeriodLength::new(params.k_smooth)?;
        PeriodLength::new(params.d_period)?;
        Ok(Self { params })
    }

    #[inline]
    pub fn params(self) -> StochasticParams {
        self.params
    }

    /// Compute %K / %D series (same length as inputs; warm-up = `None`).
    pub fn compute(
        self,
        high: &[f64],
        low: &[f64],
        close: &[f64],
    ) -> FinanceResult<StochasticSeries> {
        stochastics_validated(high, low, close, self)
    }
}

/// Aligned %K / %D output.
#[derive(Clone, Debug, PartialEq)]
pub struct StochasticSeries {
    pub k: Vec<Option<f64>>,
    pub d: Vec<Option<f64>>,
    pub params: StochasticParams,
}

impl StochasticSeries {
    /// Last defined %K / %D pair, if both present.
    pub fn last_kd(&self) -> Option<(f64, f64)> {
        let k = self.k.iter().rev().find_map(|x| *x)?;
        let d = self.d.iter().rev().find_map(|x| *x)?;
        Some((k, d))
    }
}

/// Stochastic series with raw (possibly unvalidated) params — validates then computes.
///
/// For repeated calls with the same pack, prefer [`ValidatedStochastic`].
pub fn stochastics(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    params: StochasticParams,
) -> FinanceResult<StochasticSeries> {
    let v = ValidatedStochastic::new(params)?;
    stochastics_validated(high, low, close, v)
}

/// Teaching solution: formulas + printable %K/%D table.
///
/// Prefer [`ValidatedStochastic::compute`] on the hot path; use this for notebooks,
/// audit trails, and classroom demos.
///
/// # Examples
/// ```
/// use finance_solution::stocks::ta::{stochastics_solution, StochasticParams};
/// # let h: Vec<_> = (0..20).map(|i| 20.0 + i as f64).collect();
/// # let l: Vec<_> = (0..20).map(|i| 18.0 + i as f64).collect();
/// # let c: Vec<_> = (0..20).map(|i| 19.0 + i as f64).collect();
/// let sol = stochastics_solution(&h, &l, &c, StochasticParams::fast(9, 3)).unwrap();
/// assert!(sol.formula().contains("9"));
/// // sol.print_table();
/// ```
pub fn stochastics_solution(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    params: StochasticParams,
) -> FinanceResult<StochasticSolution> {
    let series = stochastics(high, low, close, params)?;
    let formula = format!(
        "%K: stoch(k={}, smooth={}); %D: SMA(%K, {})",
        params.k_period, params.k_smooth, params.d_period
    );
    let symbolic =
        "raw_%K = 100 * (C - LL) / (HH - LL); %K = SMA(raw_%K, k_smooth); %D = SMA(%K, d)"
            .to_string();
    Ok(StochasticSolution {
        series,
        close: close.to_vec(),
        formula,
        symbolic_formula: symbolic,
    })
}

/// Teaching wrapper around [`StochasticSeries`].
#[derive(Clone, Debug)]
pub struct StochasticSolution {
    series: StochasticSeries,
    close: Vec<f64>,
    formula: String,
    symbolic_formula: String,
}

impl StochasticSolution {
    pub fn series(&self) -> &StochasticSeries {
        &self.series
    }
    pub fn formula(&self) -> &str {
        &self.formula
    }
    pub fn symbolic_formula(&self) -> &str {
        &self.symbolic_formula
    }
    pub fn params(&self) -> StochasticParams {
        self.series.params
    }

    /// # Sample output
    /// ```text
    /// period   close      k      d
    /// ------  ------  -----  -----
    ///      8   19.60  72.00    n/a
    ///     10   19.80  68.00  70.00
    /// ```
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
            ("close", "f", true),
            ("k", "f", true),
            ("d", "f", true),
        ]);
        let data = self
            .close
            .iter()
            .enumerate()
            .map(|(i, c)| {
                vec![
                    i.to_string(),
                    c.to_string(),
                    opt_cell(self.series.k[i]),
                    opt_cell(self.series.d[i]),
                ]
            })
            .collect();
        print_table_locale_opt(&columns, data, locale, precision);
    }
}

fn stochastics_validated(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    v: ValidatedStochastic,
) -> FinanceResult<StochasticSeries> {
    let p = v.params;
    let mut st = StochState::new(p)?;
    let bars = st.push_bars(high, low, close)?;
    let mut k = Vec::with_capacity(bars.len());
    let mut d = Vec::with_capacity(bars.len());
    for b in bars {
        k.push(b.k);
        d.push(b.d);
    }
    Ok(StochasticSeries { k, d, params: p })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fast_const_and_validate() {
        let p = StochasticParams::fast(9, 3);
        assert_eq!(p.k_smooth, 1);
        let v = ValidatedStochastic::new(p).unwrap();
        assert_eq!(v.params().k_period, 9);
    }

    #[test]
    fn full_presets() {
        let p = StochasticParams::full(14, 3, 3);
        assert_eq!(p.warm_up_bars(), 14 + 2 + 2);
    }

    #[test]
    fn series_length_and_warmup() {
        let n = 30;
        let high: Vec<_> = (0..n).map(|i| 100.0 + i as f64).collect();
        let low: Vec<_> = (0..n).map(|i| 90.0 + i as f64).collect();
        let close: Vec<_> = (0..n).map(|i| 95.0 + i as f64).collect();
        let out = stochastics(&high, &low, &close, StochasticParams::fast(14, 3)).unwrap();
        assert_eq!(out.k.len(), n);
        assert!(out.k[12].is_none()); // before k_period
        assert!(out.k[13].is_some());
        // %D needs 3 %K values
        assert!(out.d[13 + 2].is_some());
    }

    #[test]
    fn zero_period_err() {
        assert!(ValidatedStochastic::new(StochasticParams {
            k_period: 0,
            k_smooth: 1,
            d_period: 3
        })
        .is_err());
    }

    #[test]
    fn flat_window_carries_previous_raw() {
        // i=2 first full window (range>0); i=4 window of three 12s is flat → carry i=3 raw.
        let high = [10.0, 11.0, 12.0, 12.0, 12.0];
        let low = [9.0, 10.0, 12.0, 12.0, 12.0];
        let close = [9.5, 10.5, 12.0, 12.0, 12.0];
        let p = StochasticParams::fast(3, 1);
        let s = stochastics(&high, &low, &close, p).unwrap();
        let k3 = s.k[3].unwrap();
        // Fast k_smooth=1 → %K is raw; pure-flat bar carries previous raw.
        assert!((s.k[4].unwrap() - k3).abs() < 1e-12);
        // First flat-only bar would be 50 if no history; here we have history so not forced to 50
        // unless prior raw happened to be 50.
        assert!(s.k[4].is_some());
    }

    #[test]
    fn k_in_unit_interval_when_range_positive() {
        let n = 40;
        let high: Vec<_> = (0..n).map(|i| 100.0 + (i % 5) as f64).collect();
        let low: Vec<_> = (0..n).map(|i| 90.0 + (i % 5) as f64).collect();
        let close: Vec<_> = (0..n).map(|i| 95.0 + (i % 5) as f64 * 0.5).collect();
        let s = stochastics(&high, &low, &close, StochasticParams::full(14, 3, 3)).unwrap();
        for k in s.k.iter().flatten() {
            assert!(*k >= -1e-9 && *k <= 100.0 + 1e-9, "k={k}");
        }
    }

    #[test]
    fn high_lt_low_err() {
        let h = [10.0, 9.0];
        let l = [9.0, 10.0];
        let c = [9.5, 9.5];
        assert!(stochastics(&h, &l, &c, StochasticParams::fast(2, 1)).is_err());
    }
}
