//! # Williams %R
//!
//! Oscillator on high/low/close over lookback \(N\):
//!
//! ```text
//! %R = -100 * (HH − close) / (HH − LL)
//! ```
//!
//! where \(HH = \max(high)\) and \(LL = \min(low)\) over the last \(N\) bars.
//! Range is typically **\[−100, 0\]**. Flat window (\(HH = LL\)): carry previous %R, else **−50**.
//!
//! Default: **period 14** ([`WillrParams::period_14`]).
//!
//! ---
//!
//! ## Trading perspective
//!
//! | Region | Habit (classic, not a rule) |
//! |--------|-----------------------------|
//! | %R \> −20 | “Overbought” screen |
//! | %R \< −80 | “Oversold” screen |
//! | Cross back from extreme | Momentum resume / mean-reversion exit screen |
//!
//! ## vs other oscillators
//!
//! | | Williams %R | Stochastic | RSI |
//! |--|-------------|------------|-----|
//! | Scale | \[−100, 0\] | \[0, 100\] | \[0, 100\] |
//! | Inputs | H/L/C window | H/L/C + smooth | Closes only |
//! | Feel | Fast, inverted Stoch-like | Smoothed %K/%D | Wilder smooth, slower |
//!
//! Rough map: raw Stoch %K ≈ `100 + %R` (same HH/LL idea; sign/offset differ). Prefer
//! **Stochastic** when you want %D signal line; **%R** when you want a single fast line.
//!
//! ## Pairs well with
//!
//! - **ADX / DI** — only fade %R extremes when ADX is low (range); avoid fading when ADX is high.
//! - **SMA/EMA trend filter** — long setups only above rising MA, etc.
//! - **Volume (OBV/MFI)** — confirm oversold bounce with rising money flow.
//! - **ATR stops** — oscillator does not size risk; ATR does.
//!
//! ---
//!
//! ## Engineering
//!
//! [`WillrParams`] → [`willr`] / [`WillrState`] → [`willr_solution`].  
//! Batch uses [`WillrState`] end-to-end. HH/LL are **amortized O(1)** (sliding max/min).
//!
//! ## Word problem
//!
//! > Highs 12, lows 10, close 11 for three bars with \(N=3\). What is %R on bar 2?
//!
//! Expect: \(HH=12\), \(LL=10\), %R = \(-100 \times (12-11)/(12-10) = -50\).
//!
//! ```
//! use finance_solution::stocks::ta::{willr, WillrParams};
//! let h = [12.0, 12.0, 12.0];
//! let l = [10.0, 10.0, 10.0];
//! let c = [11.0, 11.0, 11.0];
//! let s = willr(&h, &l, &c, WillrParams::new(3)).unwrap();
//! assert!((s.willr[2].unwrap() - (-50.0)).abs() < 1e-12);
//! ```

use crate::stocks::ta::common::{opt_cell, require_hlc};
use crate::stocks::ta::ring::{SlidingMax, SlidingMin};
use crate::util::error::{require_finite, FinanceError, FinanceResult};
use crate::util::primitives::PeriodLength;
use crate::{columns_with_strings, print_table_locale_opt};

/// Williams %R lookback.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WillrParams {
    pub period: usize,
}

impl WillrParams {
    pub const fn new(period: usize) -> Self {
        Self { period }
    }

    /// Classic 14-bar Williams %R.
    pub const fn period_14() -> Self {
        Self { period: 14 }
    }
}

/// Validated pack.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ValidatedWillr {
    params: WillrParams,
}

impl ValidatedWillr {
    pub fn new(params: WillrParams) -> FinanceResult<Self> {
        PeriodLength::new(params.period)?;
        Ok(Self { params })
    }

    pub fn params(self) -> WillrParams {
        self.params
    }

    pub fn compute(self, high: &[f64], low: &[f64], close: &[f64]) -> FinanceResult<WillrSeries> {
        willr_validated(high, low, close, self)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WillrSeries {
    pub willr: Vec<Option<f64>>,
    pub params: WillrParams,
}

impl WillrSeries {
    pub fn last(&self) -> Option<f64> {
        self.willr.iter().rev().find_map(|x| *x)
    }
}

/// Incremental Williams %R.
///
/// After warm-up each [`push`](Self::push) is amortized O(1).
#[derive(Clone, Debug)]
pub struct WillrState {
    params: WillrParams,
    high_max: SlidingMax,
    low_min: SlidingMin,
    prev: Option<f64>,
    last: Option<f64>,
}

impl WillrState {
    pub fn new(params: WillrParams) -> FinanceResult<Self> {
        let _ = ValidatedWillr::new(params)?;
        Ok(Self {
            params,
            high_max: SlidingMax::with_window(params.period),
            low_min: SlidingMin::with_window(params.period),
            prev: None,
            last: None,
        })
    }

    pub fn from_history(
        params: WillrParams,
        high: &[f64],
        low: &[f64],
        close: &[f64],
    ) -> FinanceResult<Self> {
        let mut s = Self::new(params)?;
        let _ = s.push_bars(high, low, close)?;
        Ok(s)
    }

    pub fn params(&self) -> WillrParams {
        self.params
    }

    pub fn reset(&mut self) {
        self.high_max.clear();
        self.low_min.clear();
        self.prev = None;
        self.last = None;
    }

    pub fn push(&mut self, high: f64, low: f64, close: f64) -> FinanceResult<Option<f64>> {
        require_finite("high", high)?;
        require_finite("low", low)?;
        require_finite("close", close)?;
        if high < low {
            return Err(FinanceError::InvalidCashflow {
                message: "high must be >= low for each bar",
            });
        }
        let hh = self.high_max.push(high).unwrap();
        let ll = self.low_min.push(low).unwrap();
        if !self.high_max.is_full() {
            self.last = None;
            return Ok(None);
        }
        let range = hh - ll;
        let raw = if range == 0.0 {
            self.prev.unwrap_or(-50.0)
        } else {
            -100.0 * (hh - close) / range
        };
        self.prev = Some(raw);
        self.last = Some(raw);
        Ok(Some(raw))
    }

    pub fn push_bars(
        &mut self,
        high: &[f64],
        low: &[f64],
        close: &[f64],
    ) -> FinanceResult<Vec<Option<f64>>> {
        require_hlc(high, low, close)?;
        let mut out = Vec::with_capacity(close.len());
        for i in 0..close.len() {
            out.push(self.push(high[i], low[i], close[i])?);
        }
        Ok(out)
    }

    pub fn last(&self) -> Option<f64> {
        self.last
    }
}

pub fn willr(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    params: WillrParams,
) -> FinanceResult<WillrSeries> {
    ValidatedWillr::new(params)?.compute(high, low, close)
}

fn willr_validated(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    eng: ValidatedWillr,
) -> FinanceResult<WillrSeries> {
    let mut st = WillrState::new(eng.params)?;
    let willr = st.push_bars(high, low, close)?;
    Ok(WillrSeries {
        willr,
        params: eng.params,
    })
}

#[derive(Clone, Debug)]
pub struct WillrSolution {
    series: WillrSeries,
    close: Vec<f64>,
    formula: String,
}

impl WillrSolution {
    pub fn series(&self) -> &WillrSeries {
        &self.series
    }
    pub fn formula(&self) -> &str {
        &self.formula
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
            ("close", "f", true),
            ("willr", "f", true),
        ]);
        let data = self
            .close
            .iter()
            .enumerate()
            .map(|(i, c)| vec![i.to_string(), c.to_string(), opt_cell(self.series.willr[i])])
            .collect();
        print_table_locale_opt(&columns, data, locale, precision);
    }
}

/// # Examples
/// ```
/// use finance_solution::stocks::ta::{willr_solution, WillrParams};
/// let h: Vec<_> = (0..20).map(|i| 11.0 + i as f64).collect();
/// let l: Vec<_> = (0..20).map(|i| 9.0 + i as f64).collect();
/// let c: Vec<_> = (0..20).map(|i| 10.0 + i as f64).collect();
/// let sol = willr_solution(&h, &l, &c, WillrParams::period_14()).unwrap();
/// assert!(sol.formula().contains("14"));
/// ```
pub fn willr_solution(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    params: WillrParams,
) -> FinanceResult<WillrSolution> {
    let series = willr(high, low, close, params)?;
    Ok(WillrSolution {
        series,
        close: close.to_vec(),
        formula: format!("%R = -100 * (HH - C) / (HH - LL), period={}", params.period),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flat_mid_is_neg_50() {
        let h = vec![12.0; 5];
        let l = vec![10.0; 5];
        let c = vec![11.0; 5];
        let s = willr(&h, &l, &c, WillrParams::new(3)).unwrap();
        assert!((s.willr[2].unwrap() - (-50.0)).abs() < 1e-12);
    }

    #[test]
    fn at_high_is_zero() {
        let h = [10.0, 11.0, 12.0];
        let l = [8.0, 9.0, 10.0];
        let c = [10.0, 11.0, 12.0]; // close at HH
        let s = willr(&h, &l, &c, WillrParams::new(3)).unwrap();
        assert!((s.willr[2].unwrap() - 0.0).abs() < 1e-12);
    }

    #[test]
    fn at_low_is_neg_100() {
        let h = [10.0, 11.0, 12.0];
        let l = [8.0, 9.0, 10.0];
        // Window LL = 8, HH = 12; close at LL → %R = −100
        let c = [9.0, 9.5, 8.0];
        let s = willr(&h, &l, &c, WillrParams::new(3)).unwrap();
        assert!((s.willr[2].unwrap() - (-100.0)).abs() < 1e-12);
    }

    #[test]
    fn state_parity() {
        let h: Vec<_> = (0..30).map(|i| 101.0 + (i as f64) * 0.1).collect();
        let l: Vec<_> = (0..30).map(|i| 99.0 + (i as f64) * 0.1).collect();
        let c: Vec<_> = (0..30).map(|i| 100.0 + (i as f64) * 0.1).collect();
        let p = WillrParams::period_14();
        let batch = willr(&h, &l, &c, p).unwrap();
        let mut st = WillrState::new(p).unwrap();
        for i in 0..c.len() {
            let o = st.push(h[i], l[i], c[i]).unwrap();
            match (o, batch.willr[i]) {
                (None, None) => {}
                (Some(a), Some(b)) => assert!((a - b).abs() < 1e-12),
                other => panic!("{other:?}"),
            }
        }
    }

    #[test]
    fn high_lt_low_err() {
        assert!(willr(&[1.0], &[2.0], &[1.5], WillrParams::new(1)).is_err());
    }
}
