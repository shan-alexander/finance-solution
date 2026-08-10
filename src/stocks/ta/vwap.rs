//! # VWAP (volume-weighted average price)
//!
//! ```text
//! typical_t = (H + L + C) / 3   // or close-only via VwapPriceSource
//! vwap_t    = sum(typical_i * vol_i) / sum(vol_i)   // over session or rolling window
//! ```
//!
//! ## Word problem
//!
//! > Session opens; bars print (TP=10, V=100) then (TP=11, V=100). What is cumulative VWAP?
//!
//! Expect: first bar 10; second `(10*100 + 11*100) / 200 = 10.5`.
//!
//! ```
//! use finance_solution::stocks::ta::{vwap, VwapParams};
//! let h = [10.0, 11.0];
//! let l = [10.0, 11.0];
//! let c = [10.0, 11.0];
//! let v = [100.0, 100.0];
//! let s = vwap(&h, &l, &c, &v, VwapParams::cumulative_typical()).unwrap();
//! assert!((s.vwap[0].unwrap() - 10.0).abs() < 1e-12);
//! assert!((s.vwap[1].unwrap() - 10.5).abs() < 1e-12);
//! ```
//!
//! ## Modes ([`VwapMode`])
//!
//! - **Cumulative** — from bar 0 (or from last [`VwapState::reset`]) — classic intraday.
//! - **Rolling** — last `period` bars only.
//!
//! **Day reset is your policy:** call `VwapState::reset()` at session open, or rebuild state
//! from the day’s history. The library never invents a calendar.
//!
//! ## Quant pattern
//!
//! ```
//! use finance_solution::stocks::ta::{VwapParams, ValidatedVwap, VwapState};
//!
//! const INTRADAY: VwapParams = VwapParams::cumulative_typical();
//! let eng = ValidatedVwap::new(INTRADAY).unwrap();
//! # let h = [10.0, 11.0, 12.0];
//! # let l = [9.0, 10.0, 11.0];
//! # let c = [9.5, 10.5, 11.5];
//! # let vol = [100.0, 200.0, 150.0];
//! let s = eng.compute(&h, &l, &c, &vol).unwrap();
//! let mut live = VwapState::new(INTRADAY).unwrap();
//! let _ = live.push_bars(&h, &l, &c, &vol).unwrap();
//! // live.reset(); // e.g. regular-session open — you decide
//! assert!(s.vwap[2].unwrap().is_finite());
//! ```
//!
//! ## Sample solution table
//!
//! ```text
//! period  typical  volume     vwap
//! ------  -------  ------  -------
//!      0   9.5000  100.00   9.5000
//!      1  10.5000  200.00  10.1667
//!      2  11.5000  150.00  10.6111
//! ```

use crate::stocks::ta::common::{
    opt_cell, require_hlc, require_same_len, validate_positive_volume,
};
use crate::util::error::FinanceResult;
use crate::util::primitives::PeriodLength;
use crate::{columns_with_strings, print_table_locale_opt};

/// Price input for VWAP numerator.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum VwapPriceSource {
    /// `(high + low + close) / 3`.
    #[default]
    Typical,
    /// Close only.
    Close,
}

/// Cumulative session vs rolling window.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum VwapMode {
    Cumulative,
    Rolling { period: usize },
}

/// VWAP parameter pack.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct VwapParams {
    pub mode: VwapMode,
    pub price_source: VwapPriceSource,
}

impl VwapParams {
    /// Cumulative VWAP on typical price — most common intraday default.
    pub const fn cumulative_typical() -> Self {
        Self {
            mode: VwapMode::Cumulative,
            price_source: VwapPriceSource::Typical,
        }
    }

    pub const fn rolling_typical(period: usize) -> Self {
        Self {
            mode: VwapMode::Rolling { period },
            price_source: VwapPriceSource::Typical,
        }
    }
}

/// Validated VWAP config.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ValidatedVwap {
    params: VwapParams,
}

impl ValidatedVwap {
    pub fn new(params: VwapParams) -> FinanceResult<Self> {
        if let VwapMode::Rolling { period } = params.mode {
            PeriodLength::new(period)?;
        }
        Ok(Self { params })
    }

    pub fn params(self) -> VwapParams {
        self.params
    }

    pub fn compute(
        self,
        high: &[f64],
        low: &[f64],
        close: &[f64],
        volume: &[f64],
    ) -> FinanceResult<VwapSeries> {
        vwap_validated(high, low, close, volume, self)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct VwapSeries {
    pub typical: Vec<f64>,
    pub vwap: Vec<Option<f64>>,
    pub params: VwapParams,
}

#[derive(Clone, Debug)]
pub struct VwapSolution {
    series: VwapSeries,
    volume: Vec<f64>,
    formula: String,
    symbolic_formula: String,
}

impl VwapSolution {
    pub fn series(&self) -> &VwapSeries {
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
    /// period  typical  volume     vwap
    /// ------  -------  ------  -------
    ///      0   9.5000  100.00   9.5000
    ///      1  10.5000  200.00  10.1667
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
            ("typical", "f", true),
            ("volume", "f", true),
            ("vwap", "f", true),
        ]);
        let data = self
            .series
            .typical
            .iter()
            .enumerate()
            .map(|(i, tp)| {
                vec![
                    i.to_string(),
                    tp.to_string(),
                    self.volume[i].to_string(),
                    opt_cell(self.series.vwap[i]),
                ]
            })
            .collect();
        print_table_locale_opt(&columns, data, locale, precision);
    }
}

pub fn vwap(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    volume: &[f64],
    params: VwapParams,
) -> FinanceResult<VwapSeries> {
    ValidatedVwap::new(params)?.compute(high, low, close, volume)
}

/// # Examples
/// ```
/// use finance_solution::stocks::ta::{vwap_solution, VwapParams};
/// let h = [10.0, 11.0, 12.0];
/// let l = [9.0, 10.0, 11.0];
/// let c = [9.5, 10.5, 11.5];
/// let v = [100.0, 200.0, 150.0];
/// let sol = vwap_solution(&h, &l, &c, &v, VwapParams::cumulative_typical()).unwrap();
/// assert!(sol.series().vwap[0].is_some());
/// ```
pub fn vwap_solution(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    volume: &[f64],
    params: VwapParams,
) -> FinanceResult<VwapSolution> {
    let series = vwap(high, low, close, volume, params)?;
    let formula = match params.mode {
        VwapMode::Cumulative => {
            "vwap_t = sum_{i=0..t}(price_i * vol_i) / sum_{i=0..t}(vol_i)".to_string()
        }
        VwapMode::Rolling { period } => {
            format!("vwap_t = sum(price*vol over last {period}) / sum(vol over last {period})")
        }
    };
    let symbolic = "vwap = sum(price * volume) / sum(volume)".to_string();
    Ok(VwapSolution {
        series,
        volume: volume.to_vec(),
        formula,
        symbolic_formula: symbolic,
    })
}

fn vwap_validated(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    volume: &[f64],
    v: ValidatedVwap,
) -> FinanceResult<VwapSeries> {
    require_hlc(high, low, close)?;
    validate_positive_volume(volume)?;
    require_same_len(close, volume, "close/volume")?;
    let p = v.params;
    let n = close.len();
    let mut typical = vec![0.0; n];
    for i in 0..n {
        typical[i] = match p.price_source {
            VwapPriceSource::Typical => (high[i] + low[i] + close[i]) / 3.0,
            VwapPriceSource::Close => close[i],
        };
    }
    let mut vwap_out = vec![None; n];
    match p.mode {
        VwapMode::Cumulative => {
            let mut cum_pv = 0.0;
            let mut cum_v = 0.0;
            for i in 0..n {
                cum_pv += typical[i] * volume[i];
                cum_v += volume[i];
                if cum_v > 0.0 {
                    vwap_out[i] = Some(cum_pv / cum_v);
                }
            }
        }
        VwapMode::Rolling { period } => {
            for i in 0..n {
                if i + 1 < period {
                    continue;
                }
                let start = i + 1 - period;
                let mut pv = 0.0;
                let mut vv = 0.0;
                for j in start..=i {
                    pv += typical[j] * volume[j];
                    vv += volume[j];
                }
                if vv > 0.0 {
                    vwap_out[i] = Some(pv / vv);
                }
            }
        }
    }
    Ok(VwapSeries {
        typical,
        vwap: vwap_out,
        params: p,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cumulative_flat() {
        let h = [10.0, 10.0];
        let l = [10.0, 10.0];
        let c = [10.0, 10.0];
        let v = [100.0, 100.0];
        let s = vwap(&h, &l, &c, &v, VwapParams::cumulative_typical()).unwrap();
        assert!((s.vwap[1].unwrap() - 10.0).abs() < 1e-12);
    }
}
