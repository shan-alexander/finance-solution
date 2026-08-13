//! # Parabolic SAR (Stop and Reverse)
//!
//! Classic Welles Wilder trailing stop:
//!
//! ```text
//! SAR_t = SAR_{t−1} + AF * (EP − SAR_{t−1})
//! ```
//!
//! with acceleration factor `AF` starting at `start`, stepping by `increment` up to `maximum`
//! when EP makes new extremes, and flipping long/short when price crosses SAR.
//!
//! Default: `(0.02, 0.02, 0.20)` ([`SarParams::standard`]).
//!
//! First bar is warm-up (`None`); SAR starts at bar 1.
//!
//! ---
//!
//! ## Engineering
//!
//! [`SarParams`] → [`sar`] / [`SarState`] → [`sar_solution`]. Batch via state.

use crate::stocks::ta::common::{opt_cell, require_hlc};
use crate::util::error::{require_finite, FinanceError, FinanceResult};
use crate::{columns_with_strings, print_table_locale_opt};

/// Parabolic SAR acceleration parameters.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SarParams {
    pub start: f64,
    pub increment: f64,
    pub maximum: f64,
}

impl SarParams {
    pub const fn new(start: f64, increment: f64, maximum: f64) -> Self {
        Self {
            start,
            increment,
            maximum,
        }
    }

    /// Classic `(0.02, 0.02, 0.20)`.
    pub const fn standard() -> Self {
        Self {
            start: 0.02,
            increment: 0.02,
            maximum: 0.20,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ValidatedSar {
    params: SarParams,
}

impl ValidatedSar {
    pub fn new(params: SarParams) -> FinanceResult<Self> {
        require_finite("start", params.start)?;
        require_finite("increment", params.increment)?;
        require_finite("maximum", params.maximum)?;
        if params.start <= 0.0 || params.increment <= 0.0 || params.maximum < params.start {
            return Err(FinanceError::Unsolvable {
                message: "SAR requires start>0, increment>0, maximum>=start",
            });
        }
        Ok(Self { params })
    }

    pub fn params(self) -> SarParams {
        self.params
    }

    pub fn compute(self, high: &[f64], low: &[f64], close: &[f64]) -> FinanceResult<SarSeries> {
        sar_validated(high, low, close, self)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SarSeries {
    pub sar: Vec<Option<f64>>,
    /// `+1` long (SAR below), `−1` short (SAR above) when defined.
    pub direction: Vec<Option<i8>>,
    pub params: SarParams,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SarBarOutput {
    pub sar: f64,
    pub direction: i8,
}

/// Incremental Parabolic SAR.
#[derive(Clone, Debug)]
pub struct SarState {
    params: SarParams,
    // After first bar setup:
    is_long: bool,
    af: f64,
    ep: f64,
    sar: f64,
    prev_high: f64,
    prev_low: f64,
    prev2_high: Option<f64>,
    prev2_low: Option<f64>,
    started: bool,
    last: Option<SarBarOutput>,
}

impl SarState {
    pub fn new(params: SarParams) -> FinanceResult<Self> {
        let _ = ValidatedSar::new(params)?;
        Ok(Self {
            params,
            is_long: true,
            af: params.start,
            ep: 0.0,
            sar: 0.0,
            prev_high: 0.0,
            prev_low: 0.0,
            prev2_high: None,
            prev2_low: None,
            started: false,
            last: None,
        })
    }

    pub fn from_history(
        params: SarParams,
        high: &[f64],
        low: &[f64],
        close: &[f64],
    ) -> FinanceResult<Self> {
        let mut s = Self::new(params)?;
        let _ = s.push_bars(high, low, close)?;
        Ok(s)
    }

    pub fn params(&self) -> SarParams {
        self.params
    }

    pub fn reset(&mut self) {
        *self = Self::new(self.params).expect("params already valid");
    }

    pub fn push(&mut self, high: f64, low: f64, close: f64) -> FinanceResult<Option<SarBarOutput>> {
        require_finite("high", high)?;
        require_finite("low", low)?;
        require_finite("close", close)?;
        if high < low {
            return Err(FinanceError::InvalidCashflow {
                message: "high must be >= low for each bar",
            });
        }

        if !self.started {
            // Seed on first bar only (no SAR yet).
            self.prev_high = high;
            self.prev_low = low;
            self.started = true;
            self.last = None;
            return Ok(None);
        }

        if self.last.is_none() {
            // Initialize SAR at second bar from first bar extremes.
            self.is_long = close >= self.prev_high; // prefer long if ambiguous
            if high > self.prev_high {
                self.is_long = true;
            } else if low < self.prev_low {
                self.is_long = false;
            }
            if self.is_long {
                self.sar = self.prev_low;
                self.ep = high.max(self.prev_high);
            } else {
                self.sar = self.prev_high;
                self.ep = low.min(self.prev_low);
            }
            self.af = self.params.start;
            let dir = if self.is_long { 1 } else { -1 };
            let bar = SarBarOutput {
                sar: self.sar,
                direction: dir,
            };
            self.prev2_high = Some(self.prev_high);
            self.prev2_low = Some(self.prev_low);
            self.prev_high = high;
            self.prev_low = low;
            self.last = Some(bar);
            return Ok(Some(bar));
        }

        // Advance SAR
        let mut sar = self.sar + self.af * (self.ep - self.sar);

        if self.is_long {
            // SAR cannot be above prior two lows
            sar = sar.min(self.prev_low);
            if let Some(l2) = self.prev2_low {
                sar = sar.min(l2);
            }
            if low < sar {
                // flip to short
                self.is_long = false;
                sar = self.ep;
                self.ep = low;
                self.af = self.params.start;
            } else {
                if high > self.ep {
                    self.ep = high;
                    self.af = (self.af + self.params.increment).min(self.params.maximum);
                }
            }
        } else {
            sar = sar.max(self.prev_high);
            if let Some(h2) = self.prev2_high {
                sar = sar.max(h2);
            }
            if high > sar {
                // flip to long
                self.is_long = true;
                sar = self.ep;
                self.ep = high;
                self.af = self.params.start;
            } else if low < self.ep {
                self.ep = low;
                self.af = (self.af + self.params.increment).min(self.params.maximum);
            }
        }

        self.sar = sar;
        let dir = if self.is_long { 1 } else { -1 };
        let bar = SarBarOutput {
            sar,
            direction: dir,
        };
        self.prev2_high = Some(self.prev_high);
        self.prev2_low = Some(self.prev_low);
        self.prev_high = high;
        self.prev_low = low;
        self.last = Some(bar);
        Ok(Some(bar))
    }

    pub fn push_bars(
        &mut self,
        high: &[f64],
        low: &[f64],
        close: &[f64],
    ) -> FinanceResult<Vec<Option<SarBarOutput>>> {
        require_hlc(high, low, close)?;
        let mut out = Vec::with_capacity(close.len());
        for i in 0..close.len() {
            out.push(self.push(high[i], low[i], close[i])?);
        }
        Ok(out)
    }

    pub fn last(&self) -> Option<SarBarOutput> {
        self.last
    }
}

pub fn sar(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    params: SarParams,
) -> FinanceResult<SarSeries> {
    ValidatedSar::new(params)?.compute(high, low, close)
}

fn sar_validated(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    eng: ValidatedSar,
) -> FinanceResult<SarSeries> {
    let mut st = SarState::new(eng.params)?;
    let bars = st.push_bars(high, low, close)?;
    let n = bars.len();
    let mut sar = vec![None; n];
    let mut direction = vec![None; n];
    for (i, b) in bars.into_iter().enumerate() {
        if let Some(bar) = b {
            sar[i] = Some(bar.sar);
            direction[i] = Some(bar.direction);
        }
    }
    Ok(SarSeries {
        sar,
        direction,
        params: eng.params,
    })
}

#[derive(Clone, Debug)]
pub struct SarSolution {
    series: SarSeries,
    close: Vec<f64>,
    formula: String,
}

impl SarSolution {
    pub fn series(&self) -> &SarSeries {
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
            ("sar", "f", true),
            ("dir", "i", true),
        ]);
        let data = self
            .close
            .iter()
            .enumerate()
            .map(|(i, c)| {
                let d = self.series.direction[i]
                    .map(|x| x.to_string())
                    .unwrap_or_else(|| "n/a".to_string());
                vec![
                    i.to_string(),
                    c.to_string(),
                    opt_cell(self.series.sar[i]),
                    d,
                ]
            })
            .collect();
        print_table_locale_opt(&columns, data, locale, precision);
    }
}

/// # Examples
/// ```
/// use finance_solution::stocks::ta::{sar_solution, SarParams};
/// let n = 30usize;
/// let h: Vec<_> = (0..n).map(|i| 101.0 + i as f64 * 0.1).collect();
/// let l: Vec<_> = (0..n).map(|i| 99.0 + i as f64 * 0.1).collect();
/// let c: Vec<_> = (0..n).map(|i| 100.0 + i as f64 * 0.1).collect();
/// let sol = sar_solution(&h, &l, &c, SarParams::standard()).unwrap();
/// assert!(sol.formula().contains("0.02"));
/// ```
pub fn sar_solution(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    params: SarParams,
) -> FinanceResult<SarSolution> {
    let series = sar(high, low, close, params)?;
    Ok(SarSolution {
        series,
        close: close.to_vec(),
        formula: format!(
            "Parabolic SAR AF start={} step={} max={}",
            params.start, params.increment, params.maximum
        ),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn produces_sar() {
        let n = 40usize;
        let h: Vec<_> = (0..n).map(|i| 101.0 + i as f64 * 0.15).collect();
        let l: Vec<_> = (0..n).map(|i| 99.0 + i as f64 * 0.15).collect();
        let c: Vec<_> = (0..n).map(|i| 100.0 + i as f64 * 0.15).collect();
        let s = sar(&h, &l, &c, SarParams::standard()).unwrap();
        assert!(s.sar[0].is_none());
        assert!(s.sar[1].is_some());
        assert!(s.sar.iter().filter(|x| x.is_some()).count() > 20);
    }

    #[test]
    fn state_parity() {
        let n = 30usize;
        let h: Vec<_> = (0..n).map(|i| 12.0 + (i as f64 * 0.1).sin()).collect();
        let l: Vec<_> = (0..n).map(|i| 10.0 + (i as f64 * 0.1).sin()).collect();
        let c: Vec<_> = (0..n).map(|i| 11.0 + (i as f64 * 0.1).sin()).collect();
        let p = SarParams::standard();
        let batch = sar(&h, &l, &c, p).unwrap();
        let mut st = SarState::new(p).unwrap();
        for i in 0..n {
            let o = st.push(h[i], l[i], c[i]).unwrap();
            match (o, batch.sar[i], batch.direction[i]) {
                (None, None, None) => {}
                (Some(bar), Some(v), Some(d)) => {
                    assert!((bar.sar - v).abs() < 1e-9, "i={i}");
                    assert_eq!(bar.direction, d);
                }
                other => panic!("i={i}: {other:?}"),
            }
        }
    }

    #[test]
    fn bad_params_err() {
        assert!(ValidatedSar::new(SarParams::new(0.0, 0.02, 0.2)).is_err());
    }
}
