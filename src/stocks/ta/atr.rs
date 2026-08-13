//! # Average True Range (ATR)
//!
//! Wilder ATR of high / low / close:
//!
//! ```text
//! TR[i]  = max( high−low, |high−close_prev|, |low−close_prev| )
//! ATR[i] = Wilder smooth of TR over `period`
//!          (seed = SMA of first `period` true ranges)
//! ```
//!
//! Default pack: **period 14** ([`AtrParams::period_14`]).  
//! Keltner channels reuse this definition internally for their ATR leg.
//!
//! ---
//!
//! ## Trading perspective
//!
//! | Use | Habit |
//! |-----|-------|
//! | Volatility level | Wide ATR → large stops / size down |
//! | Breakout filters | Move in ATR units |
//! | Keltner width | `mult * ATR` around EMA |
//!
//! ---
//!
//! ## Engineering perspective
//!
//! [`AtrParams`] → [`ValidatedAtr`] / [`atr`] → [`AtrState`] → [`atr_solution`].  
//! First ATR at index `period − 1` when bar 0 has TR = high−low only (no prior close).
//!
//! ## Word problem
//!
//! > Constant 2-point range bars, no gaps. What is ATR(14) after warm-up?
//!
//! ≈ **2.0**.
//!
//! ```
//! use finance_solution::stocks::ta::{atr, AtrParams};
//! let n = 30usize;
//! let high: Vec<_> = (0..n).map(|_| 102.0).collect();
//! let low: Vec<_> = (0..n).map(|_| 100.0).collect();
//! let close: Vec<_> = (0..n).map(|_| 101.0).collect();
//! let s = atr(&high, &low, &close, AtrParams::period_14()).unwrap();
//! assert!((s.last().unwrap() - 2.0).abs() < 1e-9);
//! ```

use crate::stocks::ta::common::{opt_cell, require_hlc, true_range};
use crate::util::error::FinanceResult;
use crate::util::primitives::PeriodLength;
use crate::{columns_with_strings, print_table_locale_opt};

/// ATR lookback pack (Wilder).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AtrParams {
    pub period: usize,
}

impl AtrParams {
    pub const fn new(period: usize) -> Self {
        Self { period }
    }

    pub const fn period_14() -> Self {
        Self { period: 14 }
    }

    pub const fn period_10() -> Self {
        Self { period: 10 }
    }
}

/// Validated ATR config.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ValidatedAtr {
    params: AtrParams,
}

impl ValidatedAtr {
    pub fn new(params: AtrParams) -> FinanceResult<Self> {
        PeriodLength::new(params.period)?;
        Ok(Self { params })
    }

    pub fn params(self) -> AtrParams {
        self.params
    }

    pub fn compute(self, high: &[f64], low: &[f64], close: &[f64]) -> FinanceResult<AtrSeries> {
        atr_validated(high, low, close, self)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AtrSeries {
    pub atr: Vec<Option<f64>>,
    pub params: AtrParams,
}

impl AtrSeries {
    pub fn last(&self) -> Option<f64> {
        self.atr.iter().rev().find_map(|x| *x)
    }
}

#[derive(Clone, Debug)]
pub struct AtrSolution {
    series: AtrSeries,
    close: Vec<f64>,
    formula: String,
    symbolic_formula: String,
}

impl AtrSolution {
    pub fn series(&self) -> &AtrSeries {
        &self.series
    }
    pub fn formula(&self) -> &str {
        &self.formula
    }
    pub fn symbolic_formula(&self) -> &str {
        &self.symbolic_formula
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
            ("atr", "f", true),
        ]);
        let data = self
            .close
            .iter()
            .enumerate()
            .map(|(i, c)| vec![i.to_string(), c.to_string(), opt_cell(self.series.atr[i])])
            .collect();
        print_table_locale_opt(&columns, data, locale, precision);
    }
}

/// Incremental Wilder ATR.
#[derive(Clone, Debug, PartialEq)]
pub struct AtrState {
    params: AtrParams,
    prev_close: Option<f64>,
    atr: Option<f64>,
    seed_tr: Vec<f64>,
    last: Option<f64>,
}

impl AtrState {
    pub fn new(params: AtrParams) -> FinanceResult<Self> {
        PeriodLength::new(params.period)?;
        Ok(Self {
            params,
            prev_close: None,
            atr: None,
            seed_tr: Vec::with_capacity(params.period),
            last: None,
        })
    }

    pub fn from_history(
        params: AtrParams,
        high: &[f64],
        low: &[f64],
        close: &[f64],
    ) -> FinanceResult<Self> {
        let mut s = Self::new(params)?;
        let _ = s.push_bars(high, low, close)?;
        Ok(s)
    }

    pub fn push(&mut self, high: f64, low: f64, close: f64) -> FinanceResult<Option<f64>> {
        crate::util::error::require_finite("high", high)?;
        crate::util::error::require_finite("low", low)?;
        crate::util::error::require_finite("close", close)?;
        if high < low {
            return Err(crate::util::error::FinanceError::InvalidCashflow {
                message: "high must be >= low",
            });
        }
        let period = self.params.period;
        let tr = true_range(high, low, self.prev_close);
        let out = if self.atr.is_none() {
            let _ = self.seed_tr.push(tr);
            if self.seed_tr.len() == period {
                let a = self.seed_tr.iter().sum::<f64>() / period as f64;
                self.atr = Some(a);
                self.last = Some(a);
                self.last
            } else {
                None
            }
        } else {
            let a = self.atr.unwrap();
            let a = (a * (period as f64 - 1.0) + tr) / period as f64;
            self.atr = Some(a);
            self.last = Some(a);
            self.last
        };
        self.prev_close = Some(close);
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

    pub fn reset(&mut self) {
        self.prev_close = None;
        self.atr = None;
        self.seed_tr.clear();
        self.last = None;
    }
}

pub fn atr(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    params: AtrParams,
) -> FinanceResult<AtrSeries> {
    ValidatedAtr::new(params)?.compute(high, low, close)
}

fn atr_validated(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    eng: ValidatedAtr,
) -> FinanceResult<AtrSeries> {
    require_hlc(high, low, close)?;
    let mut state = AtrState::new(eng.params)?;
    let atr = state.push_bars(high, low, close)?;
    Ok(AtrSeries {
        atr,
        params: eng.params,
    })
}

/// # Examples
/// ```
/// use finance_solution::stocks::ta::{atr_solution, AtrParams};
/// let high = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0];
/// let low  = vec![ 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0];
/// let close= vec![ 9.5, 10.5, 11.5, 12.5, 13.5, 14.5, 15.5, 16.5, 17.5, 18.5, 19.5, 20.5, 21.5, 22.5, 23.5];
/// let sol = atr_solution(&high, &low, &close, AtrParams::period_14()).unwrap();
/// assert!(sol.series().last().is_some());
/// ```
pub fn atr_solution(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    params: AtrParams,
) -> FinanceResult<AtrSolution> {
    let series = atr(high, low, close, params)?;
    Ok(AtrSolution {
        series,
        close: close.to_vec(),
        formula: format!("ATR({}) = Wilder smooth of true range", params.period),
        symbolic_formula: "ATR = Wilder(TR); TR = max(H-L, |H-Cprev|, |L-Cprev|)".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constant_range() {
        let n = 30usize;
        let high: Vec<_> = (0..n).map(|_| 102.0).collect();
        let low: Vec<_> = (0..n).map(|_| 100.0).collect();
        let close: Vec<_> = (0..n).map(|_| 101.0).collect();
        let s = atr(&high, &low, &close, AtrParams::period_14()).unwrap();
        assert!((s.last().unwrap() - 2.0).abs() < 1e-9);
    }

    #[test]
    fn state_parity() {
        let n = 40usize;
        let high: Vec<_> = (0..n).map(|i| 101.0 + i as f64 * 0.1).collect();
        let low: Vec<_> = (0..n).map(|i| 99.0 + i as f64 * 0.1).collect();
        let close: Vec<_> = (0..n).map(|i| 100.0 + i as f64 * 0.1).collect();
        let batch = atr(&high, &low, &close, AtrParams::period_10()).unwrap();
        let st = AtrState::from_history(AtrParams::period_10(), &high, &low, &close).unwrap();
        assert!((batch.last().unwrap() - st.last().unwrap()).abs() < 1e-9);
    }

    #[test]
    fn high_lt_low_err() {
        let high = vec![10.0, 9.0];
        let low = vec![9.0, 10.0]; // bar 1 inverted
        let close = vec![9.5, 9.5];
        assert!(atr(&high, &low, &close, AtrParams::period_14()).is_err());
    }

    #[test]
    fn first_atr_at_period_minus_one() {
        let n = 20usize;
        let high: Vec<_> = (0..n).map(|_| 102.0).collect();
        let low: Vec<_> = (0..n).map(|_| 100.0).collect();
        let close: Vec<_> = (0..n).map(|_| 101.0).collect();
        let s = atr(&high, &low, &close, AtrParams::period_14()).unwrap();
        assert!(s.atr[12].is_none());
        assert!(s.atr[13].is_some()); // period=14 → index 13
    }

    #[test]
    fn gap_increases_atr_vs_no_gap() {
        // Same ranges but with a large gap mid-path
        let n = 30usize;
        let mut high: Vec<f64> = (0..n).map(|_| 102.0).collect();
        let mut low: Vec<f64> = (0..n).map(|_| 100.0).collect();
        let mut close: Vec<f64> = (0..n).map(|_| 101.0).collect();
        let base = atr(&high, &low, &close, AtrParams::period_14())
            .unwrap()
            .last()
            .unwrap();
        // Introduce gap open after bar 15
        high[16] = 110.0;
        low[16] = 108.0;
        close[16] = 109.0;
        let gapped = atr(&high, &low, &close, AtrParams::period_14())
            .unwrap()
            .last()
            .unwrap();
        assert!(gapped > base);
    }
}
