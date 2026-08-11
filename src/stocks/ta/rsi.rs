//! # Relative Strength Index (RSI)
//!
//! Wilder RSI on closes:
//!
//! ```text
//! change[i] = close[i] − close[i−1]
//! avg_gain, avg_loss: Wilder smooth over `period` (first seed = SMA of gains/losses)
//! RS  = avg_gain / avg_loss
//! RSI = 100 − 100 / (1 + RS)
//! ```
//!
//! Default pack: **period 14** ([`RsiParams::period_14`]).
//!
//! ---
//!
//! ## Trading perspective
//!
//! | Region | Habit (classic, not a rule) |
//! |--------|-----------------------------|
//! | RSI \> 70 | “Overbought” screen |
//! | RSI \< 30 | “Oversold” screen |
//! | Divergences | Price vs RSI direction stories |
//!
//! ---
//!
//! ## Engineering perspective
//!
//! Same TA layers: [`RsiParams`] → [`ValidatedRsi`] / [`rsi`] → [`RsiState`] → [`rsi_solution`].
//! Warm-up bars are `None` until the Wilder seed is ready (index `period` first possible).
//!
//! ## Word problem
//!
//! > Fourteen closes are flat then one up-bar. Is RSI defined on the last bar of a 15-long series?
//!
//! Yes after seed: first RSI appears at index `period` (needs `period` changes ⇒ `period+1` closes).
//!
//! ```
//! use finance_solution::stocks::ta::{rsi, RsiParams};
//! let mut c: Vec<f64> = (0..15).map(|i| 100.0 + i as f64).collect();
//! let s = rsi(&c, RsiParams::period_14()).unwrap();
//! assert!(s.rsi[14].is_some());
//! assert!(s.rsi[13].is_none());
//! ```

use crate::stocks::ta::common::{opt_cell, validate_series};
use crate::util::error::FinanceResult;
use crate::util::primitives::PeriodLength;
use crate::{columns_with_strings, print_table_locale_opt};

/// RSI lookback pack (Wilder).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RsiParams {
    pub period: usize,
}

impl RsiParams {
    pub const fn new(period: usize) -> Self {
        Self { period }
    }

    /// Classic 14-period RSI.
    pub const fn period_14() -> Self {
        Self { period: 14 }
    }

    pub const fn period_7() -> Self {
        Self { period: 7 }
    }
}

/// Validated RSI config.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ValidatedRsi {
    params: RsiParams,
}

impl ValidatedRsi {
    pub fn new(params: RsiParams) -> FinanceResult<Self> {
        PeriodLength::new(params.period)?;
        Ok(Self { params })
    }

    pub fn params(self) -> RsiParams {
        self.params
    }

    pub fn compute(self, closes: &[f64]) -> FinanceResult<RsiSeries> {
        rsi_validated(closes, self)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RsiSeries {
    pub rsi: Vec<Option<f64>>,
    pub params: RsiParams,
}

impl RsiSeries {
    pub fn last(&self) -> Option<f64> {
        self.rsi.iter().rev().find_map(|x| *x)
    }
}

#[derive(Clone, Debug)]
pub struct RsiSolution {
    series: RsiSeries,
    closes: Vec<f64>,
    formula: String,
    symbolic_formula: String,
}

impl RsiSolution {
    pub fn series(&self) -> &RsiSeries {
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
            ("rsi", "f", true),
        ]);
        let data = self
            .closes
            .iter()
            .enumerate()
            .map(|(i, c)| vec![i.to_string(), c.to_string(), opt_cell(self.series.rsi[i])])
            .collect();
        print_table_locale_opt(&columns, data, locale, precision);
    }
}

/// Incremental Wilder RSI.
#[derive(Clone, Debug, PartialEq)]
pub struct RsiState {
    params: RsiParams,
    prev_close: Option<f64>,
    avg_gain: Option<f64>,
    avg_loss: Option<f64>,
    /// Gains/losses buffer until seed length == period.
    seed_gains: Vec<f64>,
    seed_losses: Vec<f64>,
    last: Option<f64>,
    bars: usize,
}

impl RsiState {
    pub fn new(params: RsiParams) -> FinanceResult<Self> {
        PeriodLength::new(params.period)?;
        Ok(Self {
            params,
            prev_close: None,
            avg_gain: None,
            avg_loss: None,
            seed_gains: Vec::with_capacity(params.period),
            seed_losses: Vec::with_capacity(params.period),
            last: None,
            bars: 0,
        })
    }

    pub fn from_history(params: RsiParams, closes: &[f64]) -> FinanceResult<Self> {
        let mut s = Self::new(params)?;
        let _ = s.push_bars(closes)?;
        Ok(s)
    }

    pub fn push(&mut self, close: f64) -> FinanceResult<Option<f64>> {
        crate::util::error::require_finite("close", close)?;
        self.bars += 1;
        let period = self.params.period;
        let out = if let Some(prev) = self.prev_close {
            let ch = close - prev;
            let gain = ch.max(0.0);
            let loss = (-ch).max(0.0);
            if self.avg_gain.is_none() {
                self.seed_gains.push(gain);
                self.seed_losses.push(loss);
                if self.seed_gains.len() == period {
                    let ag = self.seed_gains.iter().sum::<f64>() / period as f64;
                    let al = self.seed_losses.iter().sum::<f64>() / period as f64;
                    self.avg_gain = Some(ag);
                    self.avg_loss = Some(al);
                    self.last = Some(rsi_from_avgs(ag, al));
                    self.last
                } else {
                    None
                }
            } else {
                let ag = self.avg_gain.unwrap();
                let al = self.avg_loss.unwrap();
                let ag = (ag * (period as f64 - 1.0) + gain) / period as f64;
                let al = (al * (period as f64 - 1.0) + loss) / period as f64;
                self.avg_gain = Some(ag);
                self.avg_loss = Some(al);
                self.last = Some(rsi_from_avgs(ag, al));
                self.last
            }
        } else {
            None
        };
        self.prev_close = Some(close);
        Ok(out)
    }

    pub fn push_bars(&mut self, closes: &[f64]) -> FinanceResult<Vec<Option<f64>>> {
        let mut out = Vec::with_capacity(closes.len());
        for &c in closes {
            out.push(self.push(c)?);
        }
        Ok(out)
    }

    pub fn last(&self) -> Option<f64> {
        self.last
    }

    pub fn reset(&mut self) {
        self.prev_close = None;
        self.avg_gain = None;
        self.avg_loss = None;
        self.seed_gains.clear();
        self.seed_losses.clear();
        self.last = None;
        self.bars = 0;
    }
}

fn rsi_from_avgs(avg_gain: f64, avg_loss: f64) -> f64 {
    if avg_loss == 0.0 {
        return if avg_gain == 0.0 { 50.0 } else { 100.0 };
    }
    let rs = avg_gain / avg_loss;
    100.0 - 100.0 / (1.0 + rs)
}

pub fn rsi(closes: &[f64], params: RsiParams) -> FinanceResult<RsiSeries> {
    ValidatedRsi::new(params)?.compute(closes)
}

fn rsi_validated(closes: &[f64], eng: ValidatedRsi) -> FinanceResult<RsiSeries> {
    validate_series("close", closes)?;
    let mut state = RsiState::new(eng.params)?;
    let rsi = state.push_bars(closes)?;
    Ok(RsiSeries {
        rsi,
        params: eng.params,
    })
}

/// # Examples
/// ```
/// use finance_solution::stocks::ta::{rsi_solution, RsiParams};
/// let closes: Vec<f64> = (0..30).map(|i| 100.0 + (i as f64) * 0.5).collect();
/// let sol = rsi_solution(&closes, RsiParams::period_14()).unwrap();
/// assert!(sol.series().last().unwrap() > 50.0); // rising path
/// ```
pub fn rsi_solution(closes: &[f64], params: RsiParams) -> FinanceResult<RsiSolution> {
    let series = rsi(closes, params)?;
    Ok(RsiSolution {
        series,
        closes: closes.to_vec(),
        formula: format!(
            "RSI({}) Wilder: 100 - 100/(1 + avg_gain/avg_loss)",
            params.period
        ),
        symbolic_formula: "RSI = 100 - 100/(1+RS); RS = Wilder avg gain / avg loss".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rising_path_high_rsi() {
        let c: Vec<f64> = (0..40).map(|i| 100.0 + i as f64).collect();
        let s = rsi(&c, RsiParams::period_14()).unwrap();
        let last = s.last().unwrap();
        assert!(last > 70.0, "rsi={last}");
    }

    #[test]
    fn flat_is_50_after_warmup() {
        let c = vec![100.0; 30];
        let s = rsi(&c, RsiParams::period_14()).unwrap();
        let last = s.last().unwrap();
        assert!((last - 50.0).abs() < 1e-9);
    }

    #[test]
    fn state_parity() {
        let c: Vec<f64> = (0..50)
            .map(|i| 100.0 + (i % 5) as f64 * 0.2 - 0.3)
            .collect();
        let batch = rsi(&c, RsiParams::period_14()).unwrap();
        let st = RsiState::from_history(RsiParams::period_14(), &c).unwrap();
        assert!((batch.last().unwrap() - st.last().unwrap()).abs() < 1e-9);
    }

    #[test]
    fn falling_path_low_rsi() {
        let c: Vec<f64> = (0..40).map(|i| 140.0 - i as f64).collect();
        let last = rsi(&c, RsiParams::period_14()).unwrap().last().unwrap();
        assert!(last < 30.0, "rsi={last}");
    }

    #[test]
    fn warmup_none_before_period() {
        let c: Vec<f64> = (0..20).map(|i| 100.0 + i as f64 * 0.1).collect();
        let s = rsi(&c, RsiParams::period_14()).unwrap();
        assert!(s.rsi[13].is_none());
        assert!(s.rsi[14].is_some());
    }

    #[test]
    fn empty_series_err() {
        assert!(rsi(&[], RsiParams::period_14()).is_err());
    }

    #[test]
    fn zero_period_err() {
        assert!(RsiState::new(RsiParams::new(0)).is_err());
    }

    #[test]
    fn reset_clears_last() {
        let c: Vec<f64> = (0..30).map(|i| 100.0 + i as f64).collect();
        let mut st = RsiState::from_history(RsiParams::period_14(), &c).unwrap();
        assert!(st.last().is_some());
        st.reset();
        assert!(st.last().is_none());
    }
}
