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

use crate::stocks::ta::atr::{atr, AtrParams};
use crate::stocks::ta::common::{opt_cell, require_hlc};
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
    // Legs are already state-backed (`ema` / `atr`); compose for independent warm-up
    // (mid or ATR may appear before both are ready for the channel).
    let middle = ema(close, p.ema_period)?;
    let atr_series = atr(high, low, close, AtrParams::new(p.atr_period))?;
    let atr_vals = atr_series.atr;
    let n = close.len();
    let mut upper = vec![None; n];
    let mut lower = vec![None; n];
    for i in 0..n {
        match (middle[i], atr_vals[i]) {
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
        atr: atr_vals,
        params: p,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stocks::ta::atr::{atr, AtrParams};

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

    #[test]
    fn atr_leg_matches_public_atr() {
        let n = 50usize;
        let high: Vec<_> = (0..n).map(|i| 12.0 + i as f64 * 0.05).collect();
        let low: Vec<_> = (0..n).map(|i| 10.0 + i as f64 * 0.05).collect();
        let close: Vec<_> = (0..n).map(|i| 11.0 + i as f64 * 0.05).collect();
        let kc = keltner(&high, &low, &close, KeltnerParams::standard()).unwrap();
        let a = atr(&high, &low, &close, AtrParams::period_10()).unwrap();
        for i in 0..n {
            match (kc.atr[i], a.atr[i]) {
                (None, None) => {}
                (Some(x), Some(y)) => assert!((x - y).abs() < 1e-12, "i={i}"),
                other => panic!("mismatch at {i}: {other:?}"),
            }
        }
    }
}
