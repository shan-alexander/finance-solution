//! # Keltner Channels
//!
//! ```text
//! middle = EMA(ema_period) of close
//! atr    = Wilder ATR(atr_period) of high/low/close
//! upper  = middle + atr_mult * atr
//! lower  = middle − atr_mult * atr
//! ```
//!
//! Default pack: **EMA 20, ATR 10, mult 2** ([`KeltnerParams::standard`]).
//!
//! ## Word problem
//!
//! > Compare Bollinger(20, 2) and Keltner(20, 10, 2) on the same closes. Which uses
//! > volatility of closes only, and which uses true range of the bar?
//!
//! Bollinger → close stdev. Keltner → Wilder ATR (high/low/close). Teaching tables for both
//! show warm-up `n/a` until their respective windows fill.
//!
//! ## Quant pattern
//!
//! ```
//! use finance_solution::stocks::ta::{KeltnerParams, ValidatedKeltner, KeltnerState};
//!
//! const KC: KeltnerParams = KeltnerParams::standard();
//! let eng = ValidatedKeltner::new(KC).unwrap();
//! # let n = 40usize;
//! # let high: Vec<_> = (0..n).map(|i| 101.0 + i as f64 * 0.1).collect();
//! # let low: Vec<_> = (0..n).map(|i| 99.0 + i as f64 * 0.1).collect();
//! # let close: Vec<_> = (0..n).map(|i| 100.0 + i as f64 * 0.1).collect();
//! let s = eng.compute(&high, &low, &close).unwrap();
//! let mut live = KeltnerState::new(KC).unwrap();
//! let _ = live.push_bars(&high, &low, &close).unwrap();
//! assert_eq!(s.middle.len(), n);
//! ```
//!
//! ## Sample solution table
//!
//! ```text
//! period   close  middle   upper   lower     atr
//! ------  ------  ------  ------  ------  ------
//!      9  100.90     n/a     n/a     n/a     n/a
//!     19  101.90  101.20  103.00   99.40  0.9000
//! ```

use crate::stocks::ta::common::{opt_cell, require_hlc, true_range};
use crate::stocks::ta::moving_average::ema;
use crate::util::error::{require_finite, FinanceError, FinanceResult};
use crate::util::primitives::PeriodLength;
use crate::{columns_with_strings, print_table_locale_opt};

/// Keltner parameter pack (Wilder ATR).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KeltnerParams {
    pub ema_period: usize,
    pub atr_period: usize,
    pub atr_mult: f64,
}

impl KeltnerParams {
    /// `(20, 10, 2.0)`.
    pub const fn standard() -> Self {
        Self {
            ema_period: 20,
            atr_period: 10,
            atr_mult: 2.0,
        }
    }

    pub const fn new(ema_period: usize, atr_period: usize, atr_mult: f64) -> Self {
        Self {
            ema_period,
            atr_period,
            atr_mult,
        }
    }
}

/// Validated Keltner config.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ValidatedKeltner {
    params: KeltnerParams,
}

impl ValidatedKeltner {
    pub fn new(params: KeltnerParams) -> FinanceResult<Self> {
        PeriodLength::new(params.ema_period)?;
        PeriodLength::new(params.atr_period)?;
        require_finite("atr_mult", params.atr_mult)?;
        if params.atr_mult < 0.0 {
            return Err(FinanceError::Unsolvable {
                message: "Keltner atr_mult must be non-negative",
            });
        }
        Ok(Self { params })
    }

    pub fn params(self) -> KeltnerParams {
        self.params
    }

    pub fn compute(self, high: &[f64], low: &[f64], close: &[f64]) -> FinanceResult<KeltnerSeries> {
        keltner_validated(high, low, close, self)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct KeltnerSeries {
    pub middle: Vec<Option<f64>>,
    pub upper: Vec<Option<f64>>,
    pub lower: Vec<Option<f64>>,
    pub atr: Vec<Option<f64>>,
    pub params: KeltnerParams,
}

#[derive(Clone, Debug)]
pub struct KeltnerSolution {
    series: KeltnerSeries,
    close: Vec<f64>,
    formula: String,
    symbolic_formula: String,
}

impl KeltnerSolution {
    pub fn series(&self) -> &KeltnerSeries {
        &self.series
    }
    pub fn formula(&self) -> &str {
        &self.formula
    }
    pub fn symbolic_formula(&self) -> &str {
        &self.symbolic_formula
    }

    /// # Sample output
    /// ```text
    /// period   close  middle   upper   lower     atr
    /// ------  ------  ------  ------  ------  ------
    ///     19  101.90  101.20  103.00   99.40  0.9000
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
            ("middle", "f", true),
            ("upper", "f", true),
            ("lower", "f", true),
            ("atr", "f", true),
        ]);
        let data = self
            .close
            .iter()
            .enumerate()
            .map(|(i, c)| {
                vec![
                    i.to_string(),
                    c.to_string(),
                    opt_cell(self.series.middle[i]),
                    opt_cell(self.series.upper[i]),
                    opt_cell(self.series.lower[i]),
                    opt_cell(self.series.atr[i]),
                ]
            })
            .collect();
        print_table_locale_opt(&columns, data, locale, precision);
    }
}

pub fn keltner(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    params: KeltnerParams,
) -> FinanceResult<KeltnerSeries> {
    ValidatedKeltner::new(params)?.compute(high, low, close)
}

/// # Examples
/// ```
/// use finance_solution::stocks::ta::{keltner_solution, KeltnerParams};
/// let n = 30usize;
/// let high: Vec<_> = (0..n).map(|i| 11.0 + i as f64).collect();
/// let low: Vec<_> = (0..n).map(|i| 9.0 + i as f64).collect();
/// let close: Vec<_> = (0..n).map(|i| 10.0 + i as f64).collect();
/// let sol = keltner_solution(&high, &low, &close, KeltnerParams::standard()).unwrap();
/// assert!(sol.formula().contains("Wilder"));
/// ```
pub fn keltner_solution(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    params: KeltnerParams,
) -> FinanceResult<KeltnerSolution> {
    let series = keltner(high, low, close, params)?;
    let formula = format!(
        "mid = EMA({})(close); atr = WilderATR({}); upper/lower = mid ± {} * atr",
        params.ema_period, params.atr_period, params.atr_mult
    );
    let symbolic =
        "mid = ema(close); atr = wilder_atr(high,low,close); bands = mid ± mult * atr".to_string();
    Ok(KeltnerSolution {
        series,
        close: close.to_vec(),
        formula,
        symbolic_formula: symbolic,
    })
}

fn keltner_validated(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    v: ValidatedKeltner,
) -> FinanceResult<KeltnerSeries> {
    require_hlc(high, low, close)?;
    let p = v.params;
    let middle = ema(close, p.ema_period)?;
    let atr = wilder_atr(high, low, close, p.atr_period)?;
    let n = close.len();
    let mut upper = vec![None; n];
    let mut lower = vec![None; n];
    for i in 0..n {
        match (middle[i], atr[i]) {
            (Some(m), Some(a)) => {
                upper[i] = Some(m + p.atr_mult * a);
                lower[i] = Some(m - p.atr_mult * a);
            }
            _ => {}
        }
    }
    Ok(KeltnerSeries {
        middle,
        upper,
        lower,
        atr,
        params: p,
    })
}

/// Wilder ATR series. First ATR at index `period-1` = SMA of first `period` true ranges.
fn wilder_atr(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    period: usize,
) -> FinanceResult<Vec<Option<f64>>> {
    let n = close.len();
    let mut out = vec![None; n];
    if n == 0 || period == 0 {
        return Ok(out);
    }
    let mut trs = Vec::with_capacity(n);
    for i in 0..n {
        let prev = if i == 0 { None } else { Some(close[i - 1]) };
        trs.push(true_range(high[i], low[i], prev));
    }
    if n < period {
        return Ok(out);
    }
    let sum: f64 = trs[..period].iter().sum();
    let mut prev_atr = sum / period as f64;
    out[period - 1] = Some(prev_atr);
    for i in period..n {
        prev_atr = (prev_atr * (period as f64 - 1.0) + trs[i]) / period as f64;
        out[i] = Some(prev_atr);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs() {
        let n = 40;
        let high: Vec<_> = (0..n).map(|i| 12.0 + i as f64 * 0.1).collect();
        let low: Vec<_> = (0..n).map(|i| 10.0 + i as f64 * 0.1).collect();
        let close: Vec<_> = (0..n).map(|i| 11.0 + i as f64 * 0.1).collect();
        let s = keltner(&high, &low, &close, KeltnerParams::standard()).unwrap();
        assert!(s.middle[19].is_some());
        assert!(s.atr[9].is_some());
    }
}
