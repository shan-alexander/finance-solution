//! # Commodity Channel Index (CCI)
//!
//! ```text
//! TP  = (high + low + close) / 3
//! SMA = mean(TP over N)
//! MD  = mean( |TP − SMA| over N )
//! CCI = (TP − SMA) / (0.015 × MD)
//! ```
//!
//! When mean deviation is zero (flat typical prices), CCI is **`None`** for that bar
//! (undefined scale), not a fake 0.
//!
//! Default: **period 20** ([`CciParams::period_20`]).
//!
//! ---
//!
//! ## Trading perspective
//!
//! | Region | Habit (classic) |
//! |--------|-----------------|
//! | CCI \> +100 | Extended / breakout screen |
//! | CCI \< −100 | Oversold / mean-reversion screen |
//!
//! ---
//!
//! ## Engineering
//!
//! [`CciParams`] → [`cci`] / [`CciState`] → [`cci_solution`].  
//! Batch uses [`CciState`]. After warm-up each push is **O(1)** via ring `Σ` / `Σ|x−μ|`
//! recomputed from ordered window only when needed… actually MD needs absolute deviations
//! from **current** mean, so naive O(N) per bar unless we store the window.
//!
//! We keep the ring and compute MD in O(N) on the ordered copy **or** O(N) scan of ring.
//! For typical N=20 this is fine; parity with a simple definition is the priority.
//! (Future: not the same as WMA identity — mean-deviation depends on μ each bar.)
//!
//! ## Word problem
//!
//! > Constant H=L=C=100 for 20 bars. Is CCI defined?
//!
//! No: MD = 0 → `None` (zero width).
//!
//! ```
//! use finance_solution::stocks::ta::{cci, CciParams};
//! let x = vec![100.0; 25];
//! let s = cci(&x, &x, &x, CciParams::period_20()).unwrap();
//! assert!(s.cci[19].is_none());
//! ```

use crate::stocks::ta::common::{opt_cell, require_hlc};
use crate::stocks::ta::ring::RingF64;
use crate::util::error::{require_finite, FinanceError, FinanceResult};
use crate::util::primitives::PeriodLength;
use crate::{columns_with_strings, print_table_locale_opt};

/// CCI lookback (on typical price).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CciParams {
    pub period: usize,
}

impl CciParams {
    pub const fn new(period: usize) -> Self {
        Self { period }
    }

    pub const fn period_20() -> Self {
        Self { period: 20 }
    }

    pub const fn period_14() -> Self {
        Self { period: 14 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ValidatedCci {
    params: CciParams,
}

impl ValidatedCci {
    pub fn new(params: CciParams) -> FinanceResult<Self> {
        PeriodLength::new(params.period)?;
        Ok(Self { params })
    }

    pub fn params(self) -> CciParams {
        self.params
    }

    pub fn compute(self, high: &[f64], low: &[f64], close: &[f64]) -> FinanceResult<CciSeries> {
        cci_validated(high, low, close, self)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CciSeries {
    pub cci: Vec<Option<f64>>,
    pub params: CciParams,
}

impl CciSeries {
    pub fn last(&self) -> Option<f64> {
        self.cci.iter().rev().find_map(|x| *x)
    }
}

/// Incremental CCI. After warm-up each push is O(period) mean-deviation (typical N≤20).
#[derive(Clone, Debug)]
pub struct CciState {
    params: CciParams,
    tp: RingF64,
    scratch: Vec<f64>,
    last: Option<f64>,
}

impl CciState {
    pub fn new(params: CciParams) -> FinanceResult<Self> {
        let _ = ValidatedCci::new(params)?;
        Ok(Self {
            params,
            tp: RingF64::with_capacity(params.period),
            scratch: Vec::with_capacity(params.period),
            last: None,
        })
    }

    pub fn from_history(
        params: CciParams,
        high: &[f64],
        low: &[f64],
        close: &[f64],
    ) -> FinanceResult<Self> {
        let mut s = Self::new(params)?;
        let _ = s.push_bars(high, low, close)?;
        Ok(s)
    }

    pub fn params(&self) -> CciParams {
        self.params
    }

    pub fn reset(&mut self) {
        self.tp.clear();
        self.scratch.clear();
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
        let tp = (high + low + close) / 3.0;
        let _ = self.tp.push(tp);
        if !self.tp.is_full() {
            self.last = None;
            return Ok(None);
        }
        self.tp.copy_ordered(&mut self.scratch);
        let n = self.scratch.len() as f64;
        let mean = self.tp.sum() / n;
        let mut md = 0.0;
        for &x in &self.scratch {
            md += (x - mean).abs();
        }
        md /= n;
        let out = if md <= 0.0 {
            None
        } else {
            Some((tp - mean) / (0.015 * md))
        };
        self.last = out;
        Ok(out)
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

pub fn cci(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    params: CciParams,
) -> FinanceResult<CciSeries> {
    ValidatedCci::new(params)?.compute(high, low, close)
}

fn cci_validated(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    eng: ValidatedCci,
) -> FinanceResult<CciSeries> {
    let mut st = CciState::new(eng.params)?;
    let cci = st.push_bars(high, low, close)?;
    Ok(CciSeries {
        cci,
        params: eng.params,
    })
}

#[derive(Clone, Debug)]
pub struct CciSolution {
    series: CciSeries,
    close: Vec<f64>,
    formula: String,
}

impl CciSolution {
    pub fn series(&self) -> &CciSeries {
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
            ("cci", "f", true),
        ]);
        let data = self
            .close
            .iter()
            .enumerate()
            .map(|(i, c)| vec![i.to_string(), c.to_string(), opt_cell(self.series.cci[i])])
            .collect();
        print_table_locale_opt(&columns, data, locale, precision);
    }
}

/// # Examples
/// ```
/// use finance_solution::stocks::ta::{cci_solution, CciParams};
/// let n = 30usize;
/// let h: Vec<_> = (0..n).map(|i| 101.0 + i as f64 * 0.1).collect();
/// let l: Vec<_> = (0..n).map(|i| 99.0 + i as f64 * 0.1).collect();
/// let c: Vec<_> = (0..n).map(|i| 100.0 + i as f64 * 0.1).collect();
/// let sol = cci_solution(&h, &l, &c, CciParams::period_20()).unwrap();
/// assert!(sol.formula().contains("0.015"));
/// ```
pub fn cci_solution(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    params: CciParams,
) -> FinanceResult<CciSolution> {
    let series = cci(high, low, close, params)?;
    Ok(CciSolution {
        series,
        close: close.to_vec(),
        formula: format!(
            "CCI = (TP - SMA(TP,{})) / (0.015 * MD); TP=(H+L+C)/3",
            params.period
        ),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constant_is_none() {
        let x = vec![100.0; 25];
        let s = cci(&x, &x, &x, CciParams::period_20()).unwrap();
        assert!(s.cci[19].is_none());
        assert!(s.cci[24].is_none());
    }

    #[test]
    fn rising_path_positive() {
        let n = 40usize;
        let h: Vec<_> = (0..n).map(|i| 101.0 + i as f64).collect();
        let l: Vec<_> = (0..n).map(|i| 99.0 + i as f64).collect();
        let c: Vec<_> = (0..n).map(|i| 100.0 + i as f64).collect();
        let s = cci(&h, &l, &c, CciParams::period_20()).unwrap();
        assert!(s.cci[39].unwrap() > 0.0);
    }

    #[test]
    fn state_parity() {
        let n = 50usize;
        let h: Vec<_> = (0..n).map(|i| 12.0 + (i as f64) * 0.05).collect();
        let l: Vec<_> = (0..n).map(|i| 10.0 + (i as f64) * 0.05).collect();
        let c: Vec<_> = (0..n).map(|i| 11.0 + (i as f64) * 0.05).collect();
        let p = CciParams::period_14();
        let batch = cci(&h, &l, &c, p).unwrap();
        let mut st = CciState::new(p).unwrap();
        for i in 0..n {
            let o = st.push(h[i], l[i], c[i]).unwrap();
            match (o, batch.cci[i]) {
                (None, None) => {}
                (Some(a), Some(b)) => assert!((a - b).abs() < 1e-9, "{a} vs {b}"),
                other => panic!("{other:?}"),
            }
        }
    }
}
