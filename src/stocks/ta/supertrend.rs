//! # Supertrend
//!
//! ATR-based trailing band (common breakout / stop overlay):
//!
//! ```text
//! mid = (high + low) / 2
//! basic_upper = mid + mult * ATR
//! basic_lower = mid − mult * ATR
//! final bands stick until price closes through the opposite band
//! Supertrend = final_lower in uptrend, final_upper in downtrend
//! ```
//!
//! Default: ATR period **10**, mult **3.0** ([`SupertrendParams::standard`]).
//!
//! Output: [`SupertrendBar`] with `value` and `direction` (`+1` uptrend / `−1` downtrend).
//!
//! ---
//!
//! ## Trading perspective
//!
//! | Event | Habit (classic) |
//! |-------|-----------------|
//! | Direction flips to +1 | Long regime / trail under price |
//! | Direction flips to −1 | Short regime / trail over price |
//! | Price pulls to ST line | Support/resistance screen in trend |
//!
//! Supertrend is a **stop + regime** tool, not an oscillator. Mult↑ → fewer flips, wider trail.
//!
//! ## vs Parabolic SAR / Donchian / Keltner
//!
//! | | Supertrend | SAR | Donchian | Keltner |
//! |--|------------|-----|----------|---------|
//! | Driver | ATR × mid | Acceleration on extremes | Pure HH/LL | EMA + ATR |
//! | Flip style | Close through band | Touch SAR | Channel break | Band tag |
//! | Best for | ATR-scaled trail | Fast reverse systems | Breakout structure | Channel mean reversion |
//!
//! ## Pairs well with
//!
//! - **ADX** — enter Supertrend flips only when ADX rising / above threshold.
//! - **Volume (OBV/RVOL)** — confirm breakout volume.
//! - **Higher-timeframe MA** — only long Supertrend with HTF trend.
//!
//! ---
//!
//! ## Engineering
//!
//! [`SupertrendParams`] → [`supertrend`] / [`SupertrendState`] → [`supertrend_solution`].  
//! Uses shared [`AtrState`] (Wilder). Batch via state.  
//! First defined bar seeds direction from close vs mid (implementation convention).

use crate::stocks::ta::atr::{AtrParams, AtrState};
use crate::stocks::ta::common::{opt_cell, require_hlc};
use crate::util::error::{require_finite, FinanceError, FinanceResult};
use crate::{columns_with_strings, print_table_locale_opt};

/// Supertrend pack: Wilder ATR period + band multiplier.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SupertrendParams {
    pub atr_period: usize,
    pub multiplier: f64,
}

impl SupertrendParams {
    pub const fn new(atr_period: usize, multiplier: f64) -> Self {
        Self {
            atr_period,
            multiplier,
        }
    }

    /// Common `(10, 3.0)`.
    pub const fn standard() -> Self {
        Self {
            atr_period: 10,
            multiplier: 3.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ValidatedSupertrend {
    params: SupertrendParams,
}

impl ValidatedSupertrend {
    pub fn new(params: SupertrendParams) -> FinanceResult<Self> {
        crate::util::primitives::PeriodLength::new(params.atr_period)?;
        require_finite("multiplier", params.multiplier)?;
        if params.multiplier <= 0.0 {
            return Err(FinanceError::Unsolvable {
                message: "supertrend multiplier must be positive",
            });
        }
        Ok(Self { params })
    }

    pub fn params(self) -> SupertrendParams {
        self.params
    }

    pub fn compute(
        self,
        high: &[f64],
        low: &[f64],
        close: &[f64],
    ) -> FinanceResult<SupertrendSeries> {
        supertrend_validated(high, low, close, self)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SupertrendBar {
    pub value: f64,
    /// `+1` uptrend (line under price), `−1` downtrend (line over price).
    pub direction: i8,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SupertrendSeries {
    pub value: Vec<Option<f64>>,
    pub direction: Vec<Option<i8>>,
    pub params: SupertrendParams,
}

/// Incremental Supertrend.
#[derive(Clone, Debug)]
pub struct SupertrendState {
    params: SupertrendParams,
    atr: AtrState,
    prev_close: Option<f64>,
    final_upper: Option<f64>,
    final_lower: Option<f64>,
    /// Last direction: +1 / −1
    direction: Option<i8>,
    last: Option<SupertrendBar>,
}

impl SupertrendState {
    pub fn new(params: SupertrendParams) -> FinanceResult<Self> {
        let _ = ValidatedSupertrend::new(params)?;
        Ok(Self {
            params,
            atr: AtrState::new(AtrParams::new(params.atr_period))?,
            prev_close: None,
            final_upper: None,
            final_lower: None,
            direction: None,
            last: None,
        })
    }

    pub fn from_history(
        params: SupertrendParams,
        high: &[f64],
        low: &[f64],
        close: &[f64],
    ) -> FinanceResult<Self> {
        let mut s = Self::new(params)?;
        let _ = s.push_bars(high, low, close)?;
        Ok(s)
    }

    pub fn params(&self) -> SupertrendParams {
        self.params
    }

    pub fn reset(&mut self) {
        self.atr.reset();
        self.prev_close = None;
        self.final_upper = None;
        self.final_lower = None;
        self.direction = None;
        self.last = None;
    }

    pub fn push(
        &mut self,
        high: f64,
        low: f64,
        close: f64,
    ) -> FinanceResult<Option<SupertrendBar>> {
        require_finite("high", high)?;
        require_finite("low", low)?;
        require_finite("close", close)?;
        if high < low {
            return Err(FinanceError::InvalidCashflow {
                message: "high must be >= low for each bar",
            });
        }
        let atr_v = self.atr.push(high, low, close)?;
        let out = match atr_v {
            None => {
                self.prev_close = Some(close);
                self.last = None;
                None
            }
            Some(atr) => {
                let mid = 0.5 * (high + low);
                let basic_u = mid + self.params.multiplier * atr;
                let basic_l = mid - self.params.multiplier * atr;
                let prev_c = self.prev_close.unwrap_or(close);

                let fu = match self.final_upper {
                    None => basic_u,
                    Some(prev_u) => {
                        if basic_u < prev_u || prev_c > prev_u {
                            basic_u
                        } else {
                            prev_u
                        }
                    }
                };
                let fl = match self.final_lower {
                    None => basic_l,
                    Some(prev_l) => {
                        if basic_l > prev_l || prev_c < prev_l {
                            basic_l
                        } else {
                            prev_l
                        }
                    }
                };
                self.final_upper = Some(fu);
                self.final_lower = Some(fl);

                let dir = match self.direction {
                    None => {
                        // First defined bar: close above mid → up
                        if close >= mid {
                            1
                        } else {
                            -1
                        }
                    }
                    Some(1) => {
                        if close < fl {
                            -1
                        } else {
                            1
                        }
                    }
                    Some(_) => {
                        if close > fu {
                            1
                        } else {
                            -1
                        }
                    }
                };
                self.direction = Some(dir);
                let value = if dir > 0 { fl } else { fu };
                let bar = SupertrendBar {
                    value,
                    direction: dir,
                };
                self.prev_close = Some(close);
                self.last = Some(bar);
                Some(bar)
            }
        };
        Ok(out)
    }

    pub fn push_bars(
        &mut self,
        high: &[f64],
        low: &[f64],
        close: &[f64],
    ) -> FinanceResult<Vec<Option<SupertrendBar>>> {
        require_hlc(high, low, close)?;
        let mut out = Vec::with_capacity(close.len());
        for i in 0..close.len() {
            out.push(self.push(high[i], low[i], close[i])?);
        }
        Ok(out)
    }

    pub fn last(&self) -> Option<SupertrendBar> {
        self.last
    }
}

pub fn supertrend(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    params: SupertrendParams,
) -> FinanceResult<SupertrendSeries> {
    ValidatedSupertrend::new(params)?.compute(high, low, close)
}

fn supertrend_validated(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    eng: ValidatedSupertrend,
) -> FinanceResult<SupertrendSeries> {
    let mut st = SupertrendState::new(eng.params)?;
    let bars = st.push_bars(high, low, close)?;
    let n = bars.len();
    let mut value = vec![None; n];
    let mut direction = vec![None; n];
    for (i, b) in bars.into_iter().enumerate() {
        if let Some(bar) = b {
            value[i] = Some(bar.value);
            direction[i] = Some(bar.direction);
        }
    }
    Ok(SupertrendSeries {
        value,
        direction,
        params: eng.params,
    })
}

#[derive(Clone, Debug)]
pub struct SupertrendSolution {
    series: SupertrendSeries,
    close: Vec<f64>,
    formula: String,
}

impl SupertrendSolution {
    pub fn series(&self) -> &SupertrendSeries {
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
            ("st", "f", true),
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
                    opt_cell(self.series.value[i]),
                    d,
                ]
            })
            .collect();
        print_table_locale_opt(&columns, data, locale, precision);
    }
}

/// # Examples
/// ```
/// use finance_solution::stocks::ta::{supertrend_solution, SupertrendParams};
/// let n = 40usize;
/// let h: Vec<_> = (0..n).map(|i| 101.0 + i as f64 * 0.2).collect();
/// let l: Vec<_> = (0..n).map(|i| 99.0 + i as f64 * 0.2).collect();
/// let c: Vec<_> = (0..n).map(|i| 100.0 + i as f64 * 0.2).collect();
/// let sol = supertrend_solution(&h, &l, &c, SupertrendParams::standard()).unwrap();
/// assert!(sol.formula().contains("ATR"));
/// ```
pub fn supertrend_solution(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    params: SupertrendParams,
) -> FinanceResult<SupertrendSolution> {
    let series = supertrend(high, low, close, params)?;
    Ok(SupertrendSolution {
        series,
        close: close.to_vec(),
        formula: format!(
            "Supertrend ATR({}) x {}; ST = final lower (up) / upper (down)",
            params.atr_period, params.multiplier
        ),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn produces_values() {
        let n = 40usize;
        let h: Vec<_> = (0..n).map(|i| 101.0 + i as f64 * 0.2).collect();
        let l: Vec<_> = (0..n).map(|i| 99.0 + i as f64 * 0.2).collect();
        let c: Vec<_> = (0..n).map(|i| 100.0 + i as f64 * 0.2).collect();
        let s = supertrend(&h, &l, &c, SupertrendParams::standard()).unwrap();
        assert!(s.value.iter().filter(|x| x.is_some()).count() > 10);
        // Strong uptrend → mostly +1
        let up = s.direction.iter().filter(|d| **d == Some(1)).count();
        assert!(up > 5);
    }

    #[test]
    fn state_parity() {
        let n = 35usize;
        let h: Vec<_> = (0..n).map(|i| 12.0 + i as f64 * 0.05).collect();
        let l: Vec<_> = (0..n).map(|i| 10.0 + i as f64 * 0.05).collect();
        let c: Vec<_> = (0..n).map(|i| 11.0 + i as f64 * 0.05).collect();
        let p = SupertrendParams::standard();
        let batch = supertrend(&h, &l, &c, p).unwrap();
        let mut st = SupertrendState::new(p).unwrap();
        for i in 0..n {
            let o = st.push(h[i], l[i], c[i]).unwrap();
            match (o, batch.value[i], batch.direction[i]) {
                (None, None, None) => {}
                (Some(bar), Some(v), Some(d)) => {
                    assert!((bar.value - v).abs() < 1e-9);
                    assert_eq!(bar.direction, d);
                }
                other => panic!("{other:?}"),
            }
        }
    }

    #[test]
    fn bad_mult_err() {
        assert!(ValidatedSupertrend::new(SupertrendParams::new(10, 0.0)).is_err());
    }
}
