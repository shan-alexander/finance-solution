//! # Bollinger Bands
//!
//! Middle band = SMA(`period`). Upper/lower = middle ± `num_std` × window standard deviation.
//!
//! ## Word problem
//!
//! > A trader plots Bollinger(20, 2) on daily closes. After 20 days of identical closes at 100,
//! > where are the bands?
//!
//! Expect: middle = 100, upper = lower = 100 (zero width → `%B` is `None`, not a fake 0.5).
//!
//! ```
//! use finance_solution::stocks::ta::{bollinger, BollingerParams};
//! let c = vec![100.0; 25];
//! let s = bollinger(&c, BollingerParams::standard()).unwrap();
//! assert_eq!(s.middle[19], Some(100.0));
//! assert_eq!(s.upper[19], Some(100.0));
//! assert_eq!(s.pct_b[19], None);
//! ```
//!
//! ## Stdev: sample vs population
//!
//! | Kind | Denominator | Use |
//! |------|-------------|-----|
//! | [`StdevKind::Sample`] (default) | `n − 1` | **Recommended** for Bollinger in most finance software |
//! | [`StdevKind::Population`] | `n` | Matches some charting packages / “full window” definitions |
//!
//! Sample is slightly wider for small `n`. Both are available so you are not locked in.
//!
//! ```
//! use finance_solution::stocks::ta::{bollinger, BollingerParams, StdevKind};
//! let closes: Vec<f64> = (1..=30).map(|x| 100.0 + x as f64).collect();
//! let sample = bollinger(&closes, BollingerParams::standard()).unwrap();
//! let pop = bollinger(
//!     &closes,
//!     BollingerParams { stdev: StdevKind::Population, ..BollingerParams::standard() },
//! ).unwrap();
//! // Same middle (SMA); upper band: sample ≥ population for n>1 when variance > 0
//! assert!(sample.upper[29].unwrap() >= pop.upper[29].unwrap() - 1e-12);
//! ```
//!
//! ## Quant pattern
//!
//! ```
//! use finance_solution::stocks::ta::{BollingerParams, ValidatedBollinger, BollingerState};
//!
//! const BB20_2: BollingerParams = BollingerParams::standard(); // sample stdev
//! let bb = ValidatedBollinger::new(BB20_2).unwrap();
//! # let closes: Vec<f64> = (1..=40).map(|x| 100.0 + (x as f64).sin()).collect();
//! let s = bb.compute(&closes).unwrap();
//!
//! // Live:
//! let mut st = BollingerState::new(BB20_2).unwrap();
//! let _ = st.push_bars(&closes).unwrap();
//! ```
//!
//! ## Sample solution table
//!
//! ```text
//! period   close  middle   upper   lower     pct_b
//! ------  ------  ------  ------  ------  --------
//!     18  100.50     n/a     n/a     n/a       n/a
//!     19  100.60  100.20  101.10   99.30    0.6667
//! ```
//!
//! ## %B
//!
//! `%B = (close − lower) / (upper − lower)` when width > 0; otherwise `None`.

use crate::stocks::ta::common::{opt_cell, validate_series, window_stdev, StdevKind};
use crate::stocks::ta::moving_average::sma;
use crate::util::error::{require_finite, FinanceError, FinanceResult};
use crate::util::primitives::PeriodLength;
use crate::{columns_with_strings, print_table_locale_opt};

/// Bollinger parameter pack.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BollingerParams {
    pub period: usize,
    /// Multiplier on window stdev (typically `2.0`).
    pub num_std: f64,
    /// Sample (`n−1`) or population (`n`) stdev. Default: sample.
    pub stdev: StdevKind,
}

impl BollingerParams {
    /// `(20, 2.0, Sample)` — common default.
    pub const fn standard() -> Self {
        Self {
            period: 20,
            num_std: 2.0,
            stdev: StdevKind::Sample,
        }
    }

    /// Sample stdev (same as setting `stdev: Sample`).
    pub const fn new(period: usize, num_std: f64) -> Self {
        Self {
            period,
            num_std,
            stdev: StdevKind::Sample,
        }
    }

    pub const fn with_stdev(period: usize, num_std: f64, stdev: StdevKind) -> Self {
        Self {
            period,
            num_std,
            stdev,
        }
    }
}

/// Validated Bollinger config.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ValidatedBollinger {
    params: BollingerParams,
}

impl ValidatedBollinger {
    pub fn new(params: BollingerParams) -> FinanceResult<Self> {
        PeriodLength::new(params.period)?;
        require_finite("num_std", params.num_std)?;
        if params.num_std < 0.0 {
            return Err(FinanceError::Unsolvable {
                message: "Bollinger num_std must be non-negative",
            });
        }
        match params.stdev {
            StdevKind::Sample if params.period < 2 => {
                return Err(FinanceError::Unsolvable {
                    message: "Bollinger sample stdev requires period >= 2",
                });
            }
            StdevKind::Population if params.period < 1 => {
                return Err(FinanceError::Unsolvable {
                    message: "Bollinger population stdev requires period >= 1",
                });
            }
            _ => {}
        }
        Ok(Self { params })
    }

    pub fn params(self) -> BollingerParams {
        self.params
    }

    pub fn compute(self, closes: &[f64]) -> FinanceResult<BollingerSeries> {
        bollinger_validated(closes, self)
    }
}

/// Middle / upper / lower / %B series.
#[derive(Clone, Debug, PartialEq)]
pub struct BollingerSeries {
    pub middle: Vec<Option<f64>>,
    pub upper: Vec<Option<f64>>,
    pub lower: Vec<Option<f64>>,
    /// `(close - lower) / (upper - lower)` when width > 0.
    pub pct_b: Vec<Option<f64>>,
    pub params: BollingerParams,
}

/// Teaching solution + table.
#[derive(Clone, Debug)]
pub struct BollingerSolution {
    series: BollingerSeries,
    closes: Vec<f64>,
    formula: String,
    symbolic_formula: String,
}

impl BollingerSolution {
    pub fn series(&self) -> &BollingerSeries {
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
    /// period   close  middle   upper   lower   pct_b
    /// ------  ------  ------  ------  ------  ------
    ///     19  100.60  100.20  101.10   99.30  0.6667
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
            ("pct_b", "f", true),
        ]);
        let data = self
            .closes
            .iter()
            .enumerate()
            .map(|(i, c)| {
                vec![
                    i.to_string(),
                    c.to_string(),
                    opt_cell(self.series.middle[i]),
                    opt_cell(self.series.upper[i]),
                    opt_cell(self.series.lower[i]),
                    opt_cell(self.series.pct_b[i]),
                ]
            })
            .collect();
        print_table_locale_opt(&columns, data, locale, precision);
    }
}

pub fn bollinger(closes: &[f64], params: BollingerParams) -> FinanceResult<BollingerSeries> {
    ValidatedBollinger::new(params)?.compute(closes)
}

/// Teaching solution with formulas + table.
///
/// # Examples
/// ```
/// use finance_solution::stocks::ta::{bollinger_solution, BollingerParams, StdevKind};
/// let closes: Vec<f64> = (1..=30).map(|x| 100.0 + (x as f64) * 0.1).collect();
/// let sol = bollinger_solution(&closes, BollingerParams::standard()).unwrap();
/// assert!(sol.formula().contains("20"));
/// let sol_pop = bollinger_solution(
///     &closes,
///     BollingerParams::with_stdev(20, 2.0, StdevKind::Population),
/// ).unwrap();
/// assert!(sol_pop.formula().contains("population") || sol_pop.formula().contains("Population")
///     || sol_pop.symbolic_formula().contains("population"));
/// ```
pub fn bollinger_solution(
    closes: &[f64],
    params: BollingerParams,
) -> FinanceResult<BollingerSolution> {
    let series = bollinger(closes, params)?;
    let stdev_label = match params.stdev {
        StdevKind::Sample => "sample_stdev(n-1)",
        StdevKind::Population => "population_stdev(n)",
    };
    let formula = format!(
        "mid = SMA({}); upper/lower = mid ± {} * {}; %B = (close-lower)/(upper-lower)",
        params.period, params.num_std, stdev_label
    );
    let symbolic = format!(
        "mid = sma(close,n); band = k * stdev_{:?}(window); upper = mid+band; lower = mid-band",
        params.stdev
    );
    Ok(BollingerSolution {
        series,
        closes: closes.to_vec(),
        formula,
        symbolic_formula: symbolic,
    })
}

fn bollinger_validated(closes: &[f64], v: ValidatedBollinger) -> FinanceResult<BollingerSeries> {
    validate_series("close", closes)?;
    let p = v.params;
    let mid = sma(closes, p.period)?;
    let n = closes.len();
    let mut upper = vec![None; n];
    let mut lower = vec![None; n];
    let mut pct_b = vec![None; n];
    for i in 0..n {
        if let Some(m) = mid[i] {
            let start = i + 1 - p.period;
            let window = &closes[start..=i];
            let sd = window_stdev(window, p.stdev).unwrap_or(0.0);
            let band = p.num_std * sd;
            let u = m + band;
            let l = m - band;
            upper[i] = Some(u);
            lower[i] = Some(l);
            let width = u - l;
            if width > 0.0 {
                pct_b[i] = Some((closes[i] - l) / width);
            }
        }
    }
    Ok(BollingerSeries {
        middle: mid,
        upper,
        lower,
        pct_b,
        params: p,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constant_series_zero_width() {
        let c = vec![10.0; 25];
        let s = bollinger(&c, BollingerParams::standard()).unwrap();
        assert_eq!(s.middle[19], Some(10.0));
        assert_eq!(s.upper[19], Some(10.0));
        assert_eq!(s.pct_b[19], None);
    }

    #[test]
    fn sample_wider_than_population() {
        let closes: Vec<f64> = (1..=30).map(|x| x as f64).collect();
        let s = bollinger(&closes, BollingerParams::standard()).unwrap();
        let p = bollinger(
            &closes,
            BollingerParams::with_stdev(20, 2.0, StdevKind::Population),
        )
        .unwrap();
        let su = s.upper[29].unwrap();
        let pu = p.upper[29].unwrap();
        assert!(su + 1e-12 >= pu);
    }
}
