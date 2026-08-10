//! # MACD (Moving Average Convergence Divergence)
//!
//! ```text
//! MACD   = EMA(fast) − EMA(slow)     // default 12 and 26
//! Signal = EMA(signal) of MACD line  // default 9
//! Hist   = MACD − Signal
//! ```
//!
//! ## Word problem
//!
//! > Using classic (12, 26, 9) settings on daily closes, when is the histogram first defined?
//!
//! After the slow EMA seeds (26 bars) and then the signal EMA seeds on 9 MACD points —
//! teaching tables show `n/a` until then. Live code: hold [`MacdState`] and read
//! [`MacdState::last`] after warm-up.
//!
//! ## Variations (same core)
//!
//! | Pack | Typical use |
//! |------|-------------|
//! | `MacdParams::standard()` → (12,26,9) | Default charting |
//! | `MacdParams::new(8, 17, 9)` | Faster reaction |
//! | `MacdParams::new(5, 35, 5)` | Custom desk setting |
//!
//! ## Quant pattern
//!
//! ```
//! use finance_solution::stocks::ta::{MacdParams, ValidatedMacd, MacdState};
//!
//! const STD: MacdParams = MacdParams::standard();
//! let eng = ValidatedMacd::new(STD).unwrap();
//! # let closes: Vec<f64> = (1..=40).map(|x| 100.0 + x as f64 * 0.1).collect();
//! let s = eng.compute(&closes).unwrap();
//! let mut live = MacdState::new(STD).unwrap();
//! let _ = live.push_bars(&closes).unwrap();
//! assert_eq!(s.macd.len(), closes.len());
//! ```
//!
//! ## Sample solution table
//!
//! ```text
//! period  close   macd  signal  hist
//! ------  ------  ----  ------  ----
//!     25  102.50   n/a     n/a   n/a
//!     26  102.60  0.12     n/a   n/a
//!     34  103.40  0.18    0.15  0.03
//! ```

use crate::stocks::ta::common::{opt_cell, validate_series};
use crate::stocks::ta::moving_average::ema;
use crate::util::error::{FinanceError, FinanceResult};
use crate::util::primitives::PeriodLength;
use crate::{columns_with_strings, print_table_locale_opt};

/// MACD parameter pack: `fast < slow`, all periods ≥ 1.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MacdParams {
    pub fast: usize,
    pub slow: usize,
    pub signal: usize,
}

impl MacdParams {
    /// Classic `(12, 26, 9)`.
    pub const fn standard() -> Self {
        Self {
            fast: 12,
            slow: 26,
            signal: 9,
        }
    }

    pub const fn new(fast: usize, slow: usize, signal: usize) -> Self {
        Self { fast, slow, signal }
    }
}

/// Validated MACD config for reuse across many close series.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ValidatedMacd {
    params: MacdParams,
}

impl ValidatedMacd {
    pub fn new(params: MacdParams) -> FinanceResult<Self> {
        PeriodLength::new(params.fast)?;
        PeriodLength::new(params.slow)?;
        PeriodLength::new(params.signal)?;
        if params.fast >= params.slow {
            return Err(FinanceError::Unsolvable {
                message: "MACD requires fast period < slow period",
            });
        }
        Ok(Self { params })
    }

    pub fn params(self) -> MacdParams {
        self.params
    }

    pub fn compute(self, closes: &[f64]) -> FinanceResult<MacdSeries> {
        macd_validated(closes, self)
    }
}

/// Aligned MACD / signal / histogram series.
#[derive(Clone, Debug, PartialEq)]
pub struct MacdSeries {
    pub macd: Vec<Option<f64>>,
    pub signal: Vec<Option<f64>>,
    pub histogram: Vec<Option<f64>>,
    pub params: MacdParams,
}

impl MacdSeries {
    pub fn last(&self) -> Option<(f64, f64, f64)> {
        let m = self.macd.iter().rev().find_map(|x| *x)?;
        let s = self.signal.iter().rev().find_map(|x| *x)?;
        let h = self.histogram.iter().rev().find_map(|x| *x)?;
        Some((m, s, h))
    }
}

/// Teaching wrapper with formula strings and a printable table.
#[derive(Clone, Debug)]
pub struct MacdSolution {
    series: MacdSeries,
    closes: Vec<f64>,
    formula: String,
    symbolic_formula: String,
}

impl MacdSolution {
    pub fn series(&self) -> &MacdSeries {
        &self.series
    }
    pub fn formula(&self) -> &str {
        &self.formula
    }
    pub fn symbolic_formula(&self) -> &str {
        &self.symbolic_formula
    }
    pub fn params(&self) -> MacdParams {
        self.series.params
    }

    /// Print period / close / macd / signal / hist (warm-up as `n/a`).
    ///
    /// # Sample output (truncated)
    /// ```text
    /// period   close    macd  signal    hist
    /// ------  ------  ------  ------  ------
    ///      0  100.10     n/a     n/a     n/a
    ///     25  102.60  0.1234     n/a     n/a
    ///     33  103.40  0.1800  0.1500  0.0300
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
            ("macd", "f", true),
            ("signal", "f", true),
            ("hist", "f", true),
        ]);
        let data = self
            .closes
            .iter()
            .enumerate()
            .map(|(i, c)| {
                vec![
                    i.to_string(),
                    c.to_string(),
                    opt_cell(self.series.macd[i]),
                    opt_cell(self.series.signal[i]),
                    opt_cell(self.series.histogram[i]),
                ]
            })
            .collect();
        print_table_locale_opt(&columns, data, locale, precision);
    }
}

/// Free function: validate params then compute.
pub fn macd(closes: &[f64], params: MacdParams) -> FinanceResult<MacdSeries> {
    ValidatedMacd::new(params)?.compute(closes)
}

/// Solution with formulas + table for teaching / audit.
///
/// # Examples
/// ```
/// use finance_solution::stocks::ta::{macd_solution, MacdParams};
/// let closes: Vec<f64> = (1..=40).map(|x| 100.0 + x as f64).collect();
/// let sol = macd_solution(&closes, MacdParams::standard()).unwrap();
/// assert!(sol.formula().contains("12"));
/// // sol.print_table(); // see module docs for sample layout
/// ```
pub fn macd_solution(closes: &[f64], params: MacdParams) -> FinanceResult<MacdSolution> {
    let series = macd(closes, params)?;
    let formula = format!(
        "macd = EMA({}) - EMA({}); signal = EMA({})(macd); hist = macd - signal",
        params.fast, params.slow, params.signal
    );
    let symbolic =
        "macd = ema_fast(close) - ema_slow(close); signal = ema_signal(macd); hist = macd - signal"
            .to_string();
    Ok(MacdSolution {
        series,
        closes: closes.to_vec(),
        formula,
        symbolic_formula: symbolic,
    })
}

fn macd_validated(closes: &[f64], v: ValidatedMacd) -> FinanceResult<MacdSeries> {
    validate_series("close", closes)?;
    let p = v.params;
    let fast = ema(closes, p.fast)?;
    let slow = ema(closes, p.slow)?;
    let n = closes.len();
    let mut macd_line = vec![None; n];
    for i in 0..n {
        match (fast[i], slow[i]) {
            (Some(f), Some(s)) => macd_line[i] = Some(f - s),
            _ => {}
        }
    }
    let signal = ema_on_option_series(&macd_line, p.signal);
    let mut histogram = vec![None; n];
    for i in 0..n {
        match (macd_line[i], signal[i]) {
            (Some(m), Some(s)) => histogram[i] = Some(m - s),
            _ => {}
        }
    }
    Ok(MacdSeries {
        macd: macd_line,
        signal,
        histogram,
        params: p,
    })
}

/// EMA over a series with warm-up `None`s: seed with SMA of first `period` defined values
/// in order of first availability is awkward — instead: collect contiguous once enough
/// `Some` values exist from the left after first Some.
///
/// Practical approach: build dense vector of macd values with index mapping; run EMA on dense
/// then scatter back. First seed = SMA of first `period` dense points.
fn ema_on_option_series(data: &[Option<f64>], period: usize) -> Vec<Option<f64>> {
    let n = data.len();
    let mut out = vec![None; n];
    if period == 0 {
        return out;
    }
    // Gather (index, value) for defined macd points
    let pts: Vec<(usize, f64)> = data
        .iter()
        .enumerate()
        .filter_map(|(i, v)| v.map(|x| (i, x)))
        .collect();
    if pts.len() < period {
        return out;
    }
    let mut sum = 0.0;
    for j in 0..period {
        sum += pts[j].1;
    }
    let mut prev = sum / period as f64;
    let seed_idx = pts[period - 1].0;
    out[seed_idx] = Some(prev);
    let alpha = 2.0 / (period as f64 + 1.0);
    for j in period..pts.len() {
        prev = alpha * pts[j].1 + (1.0 - alpha) * prev;
        out[pts[j].0] = Some(prev);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_runs() {
        let c: Vec<_> = (1..=50).map(|x| 100.0 + x as f64 * 0.2).collect();
        let s = macd(&c, MacdParams::standard()).unwrap();
        assert_eq!(s.macd.len(), 50);
        assert!(s.last().is_some());
    }

    #[test]
    fn fast_not_lt_slow_err() {
        assert!(ValidatedMacd::new(MacdParams::new(26, 12, 9)).is_err());
    }
}
