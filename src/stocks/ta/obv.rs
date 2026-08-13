//! # On-Balance Volume (OBV)
//!
//! Cumulative volume flow signed by close direction:
//!
//! ```text
//! if close[i] > close[i−1]:  OBV += volume[i]
//! if close[i] < close[i−1]:  OBV −= volume[i]
//! if close[i] == close[i−1]: OBV unchanged
//! ```
//!
//! First bar: **OBV = volume[0]** (common convention; signed flow starts on bar 1).
//!
//! ---
//!
//! ## Trading perspective
//!
//! | Story | Habit |
//! |-------|-------|
//! | Price up, OBV up | Volume confirms trend |
//! | Price up, OBV flat/down | Divergence screen |
//!
//! ---
//!
//! ## Engineering
//!
//! [`ObvParams`] (unit pack) → [`obv`] / [`ObvState`] → [`obv_solution`].  
//! Batch uses [`ObvState`] end-to-end. Each push is **O(1)**.
//!
//! ## Word problem
//!
//! > Closes 10 → 11 → 10 with volumes 100, 200, 50. OBV path?
//!
//! Expect: `100`, then `100+200=300`, then `300−50=250`.
//!
//! ```
//! use finance_solution::stocks::ta::{obv, ObvParams};
//! let c = [10.0, 11.0, 10.0];
//! let v = [100.0, 200.0, 50.0];
//! let s = obv(&c, &v, ObvParams::default_pack()).unwrap();
//! assert!((s.obv[0].unwrap() - 100.0).abs() < 1e-12);
//! assert!((s.obv[1].unwrap() - 300.0).abs() < 1e-12);
//! assert!((s.obv[2].unwrap() - 250.0).abs() < 1e-12);
//! ```

use crate::stocks::ta::common::{
    opt_cell, require_same_len, validate_positive_volume, validate_series,
};
use crate::util::error::{require_finite, FinanceError, FinanceResult};
use crate::{columns_with_strings, print_table_locale_opt};

/// OBV has no lookback; pack is a unit for API consistency.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct ObvParams;

impl ObvParams {
    pub const fn default_pack() -> Self {
        Self
    }
}

/// Validated pack (always succeeds).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ValidatedObv {
    params: ObvParams,
}

impl ValidatedObv {
    pub fn new(params: ObvParams) -> FinanceResult<Self> {
        Ok(Self { params })
    }

    pub fn params(self) -> ObvParams {
        self.params
    }

    pub fn compute(self, closes: &[f64], volume: &[f64]) -> FinanceResult<ObvSeries> {
        obv_validated(closes, volume, self)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ObvSeries {
    pub obv: Vec<Option<f64>>,
    pub params: ObvParams,
}

impl ObvSeries {
    pub fn last(&self) -> Option<f64> {
        self.obv.iter().rev().find_map(|x| *x)
    }
}

/// Incremental OBV. Each [`push`](Self::push) is O(1).
#[derive(Clone, Debug)]
pub struct ObvState {
    params: ObvParams,
    prev_close: Option<f64>,
    obv: Option<f64>,
    last: Option<f64>,
}

impl ObvState {
    pub fn new(params: ObvParams) -> FinanceResult<Self> {
        let _ = ValidatedObv::new(params)?;
        Ok(Self {
            params,
            prev_close: None,
            obv: None,
            last: None,
        })
    }

    pub fn from_history(params: ObvParams, closes: &[f64], volume: &[f64]) -> FinanceResult<Self> {
        let mut s = Self::new(params)?;
        let _ = s.push_bars(closes, volume)?;
        Ok(s)
    }

    pub fn params(&self) -> ObvParams {
        self.params
    }

    pub fn reset(&mut self) {
        self.prev_close = None;
        self.obv = None;
        self.last = None;
    }

    pub fn push(&mut self, close: f64, volume: f64) -> FinanceResult<Option<f64>> {
        require_finite("close", close)?;
        require_finite("volume", volume)?;
        if volume < 0.0 {
            return Err(FinanceError::InvalidCashflow {
                message: "volume must be non-negative",
            });
        }
        let val = match self.prev_close {
            None => volume,
            Some(pc) => {
                let mut o = self.obv.unwrap_or(0.0);
                if close > pc {
                    o += volume;
                } else if close < pc {
                    o -= volume;
                }
                o
            }
        };
        self.prev_close = Some(close);
        self.obv = Some(val);
        self.last = Some(val);
        Ok(Some(val))
    }

    pub fn push_bars(&mut self, closes: &[f64], volume: &[f64]) -> FinanceResult<Vec<Option<f64>>> {
        validate_series("close", closes)?;
        validate_positive_volume(volume)?;
        require_same_len(closes, volume, "close/volume")?;
        let mut out = Vec::with_capacity(closes.len());
        for i in 0..closes.len() {
            out.push(self.push(closes[i], volume[i])?);
        }
        Ok(out)
    }

    pub fn last(&self) -> Option<f64> {
        self.last
    }
}

pub fn obv(closes: &[f64], volume: &[f64], params: ObvParams) -> FinanceResult<ObvSeries> {
    ValidatedObv::new(params)?.compute(closes, volume)
}

fn obv_validated(closes: &[f64], volume: &[f64], eng: ValidatedObv) -> FinanceResult<ObvSeries> {
    let mut st = ObvState::new(eng.params)?;
    let obv = st.push_bars(closes, volume)?;
    Ok(ObvSeries {
        obv,
        params: eng.params,
    })
}

#[derive(Clone, Debug)]
pub struct ObvSolution {
    series: ObvSeries,
    close: Vec<f64>,
    volume: Vec<f64>,
    formula: String,
}

impl ObvSolution {
    pub fn series(&self) -> &ObvSeries {
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
            ("volume", "f", true),
            ("obv", "f", true),
        ]);
        let data = self
            .close
            .iter()
            .enumerate()
            .map(|(i, c)| {
                vec![
                    i.to_string(),
                    c.to_string(),
                    self.volume[i].to_string(),
                    opt_cell(self.series.obv[i]),
                ]
            })
            .collect();
        print_table_locale_opt(&columns, data, locale, precision);
    }
}

/// # Examples
/// ```
/// use finance_solution::stocks::ta::{obv_solution, ObvParams};
/// let c = [10.0, 11.0, 10.5];
/// let v = [100.0, 50.0, 25.0];
/// let sol = obv_solution(&c, &v, ObvParams::default_pack()).unwrap();
/// assert!(sol.series().last().is_some());
/// ```
pub fn obv_solution(
    closes: &[f64],
    volume: &[f64],
    params: ObvParams,
) -> FinanceResult<ObvSolution> {
    let series = obv(closes, volume, params)?;
    Ok(ObvSolution {
        series,
        close: closes.to_vec(),
        volume: volume.to_vec(),
        formula: "OBV: +vol on up close, −vol on down close, flat otherwise; seed = vol[0]"
            .to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_problem() {
        let c = [10.0, 11.0, 10.0];
        let v = [100.0, 200.0, 50.0];
        let s = obv(&c, &v, ObvParams::default_pack()).unwrap();
        assert!((s.obv[0].unwrap() - 100.0).abs() < 1e-12);
        assert!((s.obv[1].unwrap() - 300.0).abs() < 1e-12);
        assert!((s.obv[2].unwrap() - 250.0).abs() < 1e-12);
    }

    #[test]
    fn flat_close_no_volume_change() {
        let c = [10.0, 10.0];
        let v = [100.0, 999.0];
        let s = obv(&c, &v, ObvParams::default_pack()).unwrap();
        assert!((s.obv[1].unwrap() - 100.0).abs() < 1e-12);
    }

    #[test]
    fn state_parity() {
        let c: Vec<_> = (0..40).map(|i| 100.0 + (i as f64).sin()).collect();
        let v: Vec<_> = (0..40).map(|i| 1000.0 + i as f64).collect();
        let p = ObvParams::default_pack();
        let batch = obv(&c, &v, p).unwrap();
        let mut st = ObvState::new(p).unwrap();
        for i in 0..c.len() {
            let o = st.push(c[i], v[i]).unwrap();
            assert!((o.unwrap() - batch.obv[i].unwrap()).abs() < 1e-12);
        }
    }

    #[test]
    fn negative_volume_err() {
        assert!(obv(&[1.0], &[-1.0], ObvParams::default_pack()).is_err());
    }
}
