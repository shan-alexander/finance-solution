//! # Money Flow Index (MFI)
//!
//! Volume-weighted RSI-style oscillator on typical price:
//!
//! ```text
//! TP = (H + L + C) / 3
//! raw money flow = TP * volume
//! +MF / −MF over period by TP direction vs prior TP
//! MFI = 100 − 100 / (1 + +MF/−MF)
//! ```
//!
//! Warm-up: needs `period` money-flow samples after the first bar (first TP has no prior).
//! First MFI at index `period` (period flows from bars 1..=period).  
//! If −MF = 0 and +MF > 0 → MFI = 100; if both zero → `None`.
//!
//! Default: **period 14** ([`MfiParams::period_14`]).
//!
//! ---
//!
//! ## Engineering
//!
//! [`MfiParams`] → [`mfi`] / [`MfiState`] → [`mfi_solution`]. Batch via state.
//! After warm-up each push is O(1) via rings of +MF/−MF contributions.

use crate::stocks::ta::common::{opt_cell, require_hlc, validate_positive_volume};
use crate::stocks::ta::ring::RingF64;
use crate::util::error::{require_finite, FinanceError, FinanceResult};
use crate::util::primitives::PeriodLength;
use crate::{columns_with_strings, print_table_locale_opt};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MfiParams {
    pub period: usize,
}

impl MfiParams {
    pub const fn new(period: usize) -> Self {
        Self { period }
    }

    pub const fn period_14() -> Self {
        Self { period: 14 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ValidatedMfi {
    params: MfiParams,
}

impl ValidatedMfi {
    pub fn new(params: MfiParams) -> FinanceResult<Self> {
        PeriodLength::new(params.period)?;
        Ok(Self { params })
    }

    pub fn params(self) -> MfiParams {
        self.params
    }

    pub fn compute(
        self,
        high: &[f64],
        low: &[f64],
        close: &[f64],
        volume: &[f64],
    ) -> FinanceResult<MfiSeries> {
        mfi_validated(high, low, close, volume, self)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MfiSeries {
    pub mfi: Vec<Option<f64>>,
    pub params: MfiParams,
}

impl MfiSeries {
    pub fn last(&self) -> Option<f64> {
        self.mfi.iter().rev().find_map(|x| *x)
    }
}

/// Incremental MFI.
#[derive(Clone, Debug)]
pub struct MfiState {
    params: MfiParams,
    prev_tp: Option<f64>,
    pos: RingF64,
    neg: RingF64,
    last: Option<f64>,
}

impl MfiState {
    pub fn new(params: MfiParams) -> FinanceResult<Self> {
        let _ = ValidatedMfi::new(params)?;
        Ok(Self {
            params,
            prev_tp: None,
            pos: RingF64::with_capacity(params.period),
            neg: RingF64::with_capacity(params.period),
            last: None,
        })
    }

    pub fn from_history(
        params: MfiParams,
        high: &[f64],
        low: &[f64],
        close: &[f64],
        volume: &[f64],
    ) -> FinanceResult<Self> {
        let mut s = Self::new(params)?;
        let _ = s.push_bars(high, low, close, volume)?;
        Ok(s)
    }

    pub fn params(&self) -> MfiParams {
        self.params
    }

    pub fn reset(&mut self) {
        self.prev_tp = None;
        self.pos.clear();
        self.neg.clear();
        self.last = None;
    }

    pub fn push(
        &mut self,
        high: f64,
        low: f64,
        close: f64,
        volume: f64,
    ) -> FinanceResult<Option<f64>> {
        require_finite("high", high)?;
        require_finite("low", low)?;
        require_finite("close", close)?;
        require_finite("volume", volume)?;
        if high < low {
            return Err(FinanceError::InvalidCashflow {
                message: "high must be >= low for each bar",
            });
        }
        if volume < 0.0 {
            return Err(FinanceError::InvalidCashflow {
                message: "volume must be non-negative",
            });
        }
        let tp = (high + low + close) / 3.0;
        let rmf = tp * volume;
        let out = match self.prev_tp {
            None => {
                self.prev_tp = Some(tp);
                self.last = None;
                None
            }
            Some(ptp) => {
                let (p, n) = if tp > ptp {
                    (rmf, 0.0)
                } else if tp < ptp {
                    (0.0, rmf)
                } else {
                    (0.0, 0.0)
                };
                let _ = self.pos.push(p);
                let _ = self.neg.push(n);
                self.prev_tp = Some(tp);
                if !self.pos.is_full() {
                    self.last = None;
                    None
                } else {
                    let pos_sum = self.pos.sum();
                    let neg_sum = self.neg.sum();
                    let mfi = if neg_sum == 0.0 && pos_sum == 0.0 {
                        None
                    } else if neg_sum == 0.0 {
                        Some(100.0)
                    } else {
                        let ratio = pos_sum / neg_sum;
                        Some(100.0 - 100.0 / (1.0 + ratio))
                    };
                    self.last = mfi;
                    mfi
                }
            }
        };
        Ok(out)
    }

    pub fn push_bars(
        &mut self,
        high: &[f64],
        low: &[f64],
        close: &[f64],
        volume: &[f64],
    ) -> FinanceResult<Vec<Option<f64>>> {
        require_hlc(high, low, close)?;
        validate_positive_volume(volume)?;
        if close.len() != volume.len() {
            return Err(FinanceError::LengthMismatch {
                left: close.len(),
                right: volume.len(),
                context: "close/volume",
            });
        }
        let mut out = Vec::with_capacity(close.len());
        for i in 0..close.len() {
            out.push(self.push(high[i], low[i], close[i], volume[i])?);
        }
        Ok(out)
    }

    pub fn last(&self) -> Option<f64> {
        self.last
    }
}

pub fn mfi(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    volume: &[f64],
    params: MfiParams,
) -> FinanceResult<MfiSeries> {
    ValidatedMfi::new(params)?.compute(high, low, close, volume)
}

fn mfi_validated(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    volume: &[f64],
    eng: ValidatedMfi,
) -> FinanceResult<MfiSeries> {
    let mut st = MfiState::new(eng.params)?;
    let mfi = st.push_bars(high, low, close, volume)?;
    Ok(MfiSeries {
        mfi,
        params: eng.params,
    })
}

#[derive(Clone, Debug)]
pub struct MfiSolution {
    series: MfiSeries,
    close: Vec<f64>,
    formula: String,
}

impl MfiSolution {
    pub fn series(&self) -> &MfiSeries {
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
            ("mfi", "f", true),
        ]);
        let data = self
            .close
            .iter()
            .enumerate()
            .map(|(i, c)| vec![i.to_string(), c.to_string(), opt_cell(self.series.mfi[i])])
            .collect();
        print_table_locale_opt(&columns, data, locale, precision);
    }
}

/// # Examples
/// ```
/// use finance_solution::stocks::ta::{mfi_solution, MfiParams};
/// let n = 30usize;
/// let h: Vec<_> = (0..n).map(|i| 101.0 + i as f64 * 0.1).collect();
/// let l: Vec<_> = (0..n).map(|i| 99.0 + i as f64 * 0.1).collect();
/// let c: Vec<_> = (0..n).map(|i| 100.0 + i as f64 * 0.1).collect();
/// let v: Vec<_> = (0..n).map(|i| 1000.0 + i as f64).collect();
/// let sol = mfi_solution(&h, &l, &c, &v, MfiParams::period_14()).unwrap();
/// assert!(sol.formula().contains("14"));
/// ```
pub fn mfi_solution(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    volume: &[f64],
    params: MfiParams,
) -> FinanceResult<MfiSolution> {
    let series = mfi(high, low, close, volume, params)?;
    Ok(MfiSolution {
        series,
        close: close.to_vec(),
        formula: format!(
            "MFI({}) = 100 - 100/(1 + +MF/-MF); TP=(H+L+C)/3; MF=TP*vol",
            params.period
        ),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rising_high_mfi() {
        let n = 40usize;
        let h: Vec<_> = (0..n).map(|i| 101.0 + i as f64).collect();
        let l: Vec<_> = (0..n).map(|i| 99.0 + i as f64).collect();
        let c: Vec<_> = (0..n).map(|i| 100.0 + i as f64).collect();
        let v = vec![1000.0; n];
        let s = mfi(&h, &l, &c, &v, MfiParams::period_14()).unwrap();
        assert!(s.last().unwrap() > 80.0);
    }

    #[test]
    fn state_parity() {
        let n = 35usize;
        let h: Vec<_> = (0..n).map(|i| 12.0 + (i as f64).sin()).collect();
        let l: Vec<_> = (0..n).map(|i| 10.0 + (i as f64).sin()).collect();
        let c: Vec<_> = (0..n).map(|i| 11.0 + (i as f64).sin()).collect();
        let v: Vec<_> = (0..n).map(|i| 500.0 + i as f64).collect();
        let p = MfiParams::period_14();
        let batch = mfi(&h, &l, &c, &v, p).unwrap();
        let mut st = MfiState::new(p).unwrap();
        for i in 0..n {
            let o = st.push(h[i], l[i], c[i], v[i]).unwrap();
            match (o, batch.mfi[i]) {
                (None, None) => {}
                (Some(a), Some(b)) => assert!((a - b).abs() < 1e-9),
                other => panic!("{other:?}"),
            }
        }
    }
}
