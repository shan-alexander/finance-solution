//! # Relative volume (RVOL)
//!
//! ```text
//! rvol[i] = volume[i] / mean(volume[i − lookback + 1 ..= i])
//! ```
//!
//! when the lookback window is full; warm-up bars are `None`.
//!
//! ## Word problem
//!
//! > Twenty bars printed 1,000 volume, then one bar prints 2,000. What is RVOL(20)?
//!
//! The lookback **includes the current bar**, so
//! `mean = (19×1000 + 2000) / 20 = 1050` and `RVOL = 2000/1050 ≈ 1.905`.
//!
//! ```
//! use finance_solution::stocks::ta::{rvol, RvolParams};
//! let mut vol = vec![1_000.0; 20];
//! vol.push(2_000.0);
//! let s = rvol(&vol, RvolParams::days_20()).unwrap();
//! assert!((s.rvol[20].unwrap() - 2000.0 / 1050.0).abs() < 1e-9);
//! ```
//!
//! Constant volume → RVOL = 1 after warm-up (useful unit check).
//!
//! ## Quant pattern
//!
//! ```
//! use finance_solution::stocks::ta::{RvolParams, ValidatedRvol, RvolState};
//!
//! const R20: RvolParams = RvolParams::days_20();
//! let eng = ValidatedRvol::new(R20).unwrap();
//! # let vol: Vec<f64> = (1..=30).map(|x| 1_000.0 + x as f64 * 10.0).collect();
//! let s = eng.compute(&vol).unwrap();
//! let mut live = RvolState::new(R20).unwrap();
//! let _ = live.push_bars(&vol).unwrap();
//! assert_eq!(s.rvol.len(), vol.len());
//! ```
//!
//! ## Sample solution table
//!
//! ```text
//! period   volume    rvol
//! ------  -------  ------
//!     18  1180.00     n/a
//!     19  1190.00  1.0820
//!     20  2000.00  1.7540
//! ```

use crate::stocks::ta::common::{opt_cell, validate_positive_volume};
use crate::util::error::FinanceResult;
use crate::util::primitives::PeriodLength;
use crate::{columns_with_strings, print_table_locale_opt};

/// RVOL lookback pack.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RvolParams {
    pub lookback: usize,
}

impl RvolParams {
    pub const fn new(lookback: usize) -> Self {
        Self { lookback }
    }

    /// Common short lookback.
    pub const fn days_20() -> Self {
        Self { lookback: 20 }
    }

    /// Common longer lookback.
    pub const fn days_50() -> Self {
        Self { lookback: 50 }
    }
}

/// Validated RVOL config.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ValidatedRvol {
    params: RvolParams,
}

impl ValidatedRvol {
    pub fn new(params: RvolParams) -> FinanceResult<Self> {
        PeriodLength::new(params.lookback)?;
        Ok(Self { params })
    }

    pub fn params(self) -> RvolParams {
        self.params
    }

    pub fn compute(self, volume: &[f64]) -> FinanceResult<RvolSeries> {
        rvol_validated(volume, self)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RvolSeries {
    pub rvol: Vec<Option<f64>>,
    pub params: RvolParams,
}

impl RvolSeries {
    pub fn last(&self) -> Option<f64> {
        self.rvol.iter().rev().find_map(|x| *x)
    }
}

#[derive(Clone, Debug)]
pub struct RvolSolution {
    series: RvolSeries,
    volume: Vec<f64>,
    formula: String,
    symbolic_formula: String,
}

impl RvolSolution {
    pub fn series(&self) -> &RvolSeries {
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
    /// period   volume    rvol
    /// ------  -------  ------
    ///     19  1190.00  1.0820
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
            ("volume", "f", true),
            ("rvol", "f", true),
        ]);
        let data = self
            .volume
            .iter()
            .enumerate()
            .map(|(i, v)| vec![i.to_string(), v.to_string(), opt_cell(self.series.rvol[i])])
            .collect();
        print_table_locale_opt(&columns, data, locale, precision);
    }
}

pub fn rvol(volume: &[f64], params: RvolParams) -> FinanceResult<RvolSeries> {
    ValidatedRvol::new(params)?.compute(volume)
}

/// # Examples
/// ```
/// use finance_solution::stocks::ta::{rvol_solution, RvolParams};
/// let vol = vec![100.0; 25];
/// let sol = rvol_solution(&vol, RvolParams::days_20()).unwrap();
/// // Constant volume ⇒ RVOL ≈ 1 after warm-up
/// assert!((sol.series().rvol[19].unwrap() - 1.0).abs() < 1e-12);
/// ```
pub fn rvol_solution(volume: &[f64], params: RvolParams) -> FinanceResult<RvolSolution> {
    let series = rvol(volume, params)?;
    let formula = format!(
        "rvol[i] = volume[i] / mean(volume[i-{}+1 ..= i])",
        params.lookback
    );
    let symbolic = "rvol = volume / sma(volume, lookback)".to_string();
    Ok(RvolSolution {
        series,
        volume: volume.to_vec(),
        formula,
        symbolic_formula: symbolic,
    })
}

fn rvol_validated(volume: &[f64], v: ValidatedRvol) -> FinanceResult<RvolSeries> {
    validate_positive_volume(volume)?;
    let lb = v.params.lookback;
    let n = volume.len();
    let mut out = vec![None; n];
    if n < lb {
        return Ok(RvolSeries {
            rvol: out,
            params: v.params,
        });
    }
    let mut sum: f64 = volume[..lb].iter().sum();
    let mean0 = sum / lb as f64;
    if mean0 > 0.0 {
        out[lb - 1] = Some(volume[lb - 1] / mean0);
    }
    for i in lb..n {
        sum += volume[i] - volume[i - lb];
        let mean = sum / lb as f64;
        if mean > 0.0 {
            out[i] = Some(volume[i] / mean);
        }
    }
    Ok(RvolSeries {
        rvol: out,
        params: v.params,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constant_is_one() {
        let v = vec![500.0; 30];
        let s = rvol(&v, RvolParams::days_20()).unwrap();
        assert!((s.rvol[19].unwrap() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn lookback_one_is_unity_when_positive() {
        let v = [10.0, 20.0, 5.0];
        let s = rvol(&v, RvolParams::new(1)).unwrap();
        assert!((s.rvol[0].unwrap() - 1.0).abs() < 1e-12);
        assert!((s.rvol[1].unwrap() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn zero_mean_window_is_none() {
        let v = vec![0.0; 25];
        let s = rvol(&v, RvolParams::days_20()).unwrap();
        assert!(s.rvol[19].is_none());
    }

    #[test]
    fn spike_matches_doc_formula() {
        let mut vol = vec![1_000.0; 20];
        vol.push(2_000.0);
        let s = rvol(&vol, RvolParams::days_20()).unwrap();
        // mean includes current: (19*1000 + 2000)/20 = 1050
        assert!((s.rvol[20].unwrap() - 2000.0 / 1050.0).abs() < 1e-12);
    }

    #[test]
    fn negative_volume_err() {
        assert!(rvol(&[1.0, -1.0], RvolParams::new(2)).is_err());
    }
}
