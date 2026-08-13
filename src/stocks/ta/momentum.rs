//! # Rate of change / momentum (ROC, ROCP, MOM)
//!
//! On a close series with lookback \(N\):
//!
//! ```text
//! MOM[i]  = close[i] − close[i−N]
//! ROC[i]  = 100 * (close[i] − close[i−N]) / close[i−N]   // percent
//! ROCP[i] = (close[i] − close[i−N]) / close[i−N]         // fraction
//! ```
//!
//! Warm-up: first \(N\) bars are `None`. If `close[i−N] == 0`, ROC/ROCP are `None`
//! (MOM can still be defined).
//!
//! Defaults: **period 10** ([`MomParams::period_10`]).
//!
//! ---
//!
//! ## Trading perspective
//!
//! | Reading | Habit (classic) |
//! |---------|-----------------|
//! | ROC \> 0 | Price above N-bar-ago close (up momentum) |
//! | ROC zero cross | Momentum flip screen |
//! | Rising ROC while price flat | Acceleration narrative |
//!
//! **MOM** is absolute points (scale depends on price level). **ROC/ROCP** are scale-free —
//! prefer them when comparing names or eras.
//!
//! ## vs RSI / MACD
//!
//! | | MOM/ROC | RSI | MACD |
//! |--|---------|-----|------|
//! | Lag / smooth | None (raw N-bar change) | Wilder smooth | Dual EMA smooth |
//! | Best for | Simple impulse / ranking | Bounded OB/OS | Crossover systems |
//!
//! ## Pairs well with
//!
//! - **SMA/EMA** — only long when ROC \> 0 and price above MA.
//! - **ADX** — strong ROC moves with rising ADX = trend impulse.
//! - **Bollinger** — ROC spike into band tag for exhaustion screens.
//!
//! ---
//!
//! ## Engineering
//!
//! [`MomParams`] → [`mom`] / [`roc`] / [`rocp`] / [`MomState`] → [`mom_solution`].  
//! Batch uses [`MomState`]. After warm-up each push is **O(1)** (ring of lag closes).
//! One state computes all three outputs per bar ([`MomBarOutput`]).

use crate::stocks::ta::common::{opt_cell, validate_series};
use crate::stocks::ta::ring::RingF64;
use crate::util::error::{require_finite, FinanceResult};
use crate::util::primitives::PeriodLength;
use crate::{columns_with_strings, print_table_locale_opt};

/// Lookback for MOM / ROC / ROCP.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MomParams {
    pub period: usize,
}

impl MomParams {
    pub const fn new(period: usize) -> Self {
        Self { period }
    }

    pub const fn period_10() -> Self {
        Self { period: 10 }
    }

    pub const fn period_12() -> Self {
        Self { period: 12 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ValidatedMom {
    params: MomParams,
}

impl ValidatedMom {
    pub fn new(params: MomParams) -> FinanceResult<Self> {
        PeriodLength::new(params.period)?;
        Ok(Self { params })
    }

    pub fn params(self) -> MomParams {
        self.params
    }

    pub fn compute_mom(self, closes: &[f64]) -> FinanceResult<MomSeries> {
        mom_validated(closes, self)
    }

    pub fn compute_roc(self, closes: &[f64]) -> FinanceResult<RocSeries> {
        roc_validated(closes, self)
    }

    pub fn compute_rocp(self, closes: &[f64]) -> FinanceResult<RocpSeries> {
        rocp_validated(closes, self)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MomSeries {
    pub mom: Vec<Option<f64>>,
    pub params: MomParams,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RocSeries {
    pub roc: Vec<Option<f64>>,
    pub params: MomParams,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RocpSeries {
    pub rocp: Vec<Option<f64>>,
    pub params: MomParams,
}

/// One-bar momentum pack.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MomBarOutput {
    pub mom: Option<f64>,
    pub roc: Option<f64>,
    pub rocp: Option<f64>,
}

/// Incremental MOM/ROC/ROCP (shared ring of last `period` closes + current).
///
/// Ring capacity is `period` holding the **lag** close (oldest = close\[i−period\] when full).
#[derive(Clone, Debug)]
pub struct MomState {
    params: MomParams,
    /// Holds the last `period` closes (oldest is lag for current after push of new).
    lag: RingF64,
    last: Option<MomBarOutput>,
}

impl MomState {
    pub fn new(params: MomParams) -> FinanceResult<Self> {
        let _ = ValidatedMom::new(params)?;
        Ok(Self {
            params,
            lag: RingF64::with_capacity(params.period),
            last: None,
        })
    }

    pub fn from_history(params: MomParams, closes: &[f64]) -> FinanceResult<Self> {
        let mut s = Self::new(params)?;
        let _ = s.push_bars(closes)?;
        Ok(s)
    }

    pub fn params(&self) -> MomParams {
        self.params
    }

    pub fn reset(&mut self) {
        self.lag.clear();
        self.last = None;
    }

    pub fn push(&mut self, close: f64) -> FinanceResult<Option<MomBarOutput>> {
        require_finite("close", close)?;
        let out = if self.lag.is_full() {
            let lag = self.lag.oldest().unwrap();
            let mom = close - lag;
            let (roc, rocp) = if lag == 0.0 {
                (None, None)
            } else {
                let frac = (close - lag) / lag;
                (Some(100.0 * frac), Some(frac))
            };
            let bar = MomBarOutput {
                mom: Some(mom),
                roc,
                rocp,
            };
            self.last = Some(bar);
            Some(bar)
        } else {
            self.last = None;
            None
        };
        let _ = self.lag.push(close);
        Ok(out)
    }

    pub fn push_bars(&mut self, closes: &[f64]) -> FinanceResult<Vec<Option<MomBarOutput>>> {
        validate_series("close", closes)?;
        let mut out = Vec::with_capacity(closes.len());
        for &c in closes {
            out.push(self.push(c)?);
        }
        Ok(out)
    }

    pub fn last(&self) -> Option<MomBarOutput> {
        self.last
    }
}

pub fn mom(closes: &[f64], params: MomParams) -> FinanceResult<MomSeries> {
    ValidatedMom::new(params)?.compute_mom(closes)
}

pub fn roc(closes: &[f64], params: MomParams) -> FinanceResult<RocSeries> {
    ValidatedMom::new(params)?.compute_roc(closes)
}

pub fn rocp(closes: &[f64], params: MomParams) -> FinanceResult<RocpSeries> {
    ValidatedMom::new(params)?.compute_rocp(closes)
}

fn mom_validated(closes: &[f64], eng: ValidatedMom) -> FinanceResult<MomSeries> {
    let mut st = MomState::new(eng.params)?;
    let bars = st.push_bars(closes)?;
    Ok(MomSeries {
        mom: bars.iter().map(|b| b.and_then(|x| x.mom)).collect(),
        params: eng.params,
    })
}

fn roc_validated(closes: &[f64], eng: ValidatedMom) -> FinanceResult<RocSeries> {
    let mut st = MomState::new(eng.params)?;
    let bars = st.push_bars(closes)?;
    Ok(RocSeries {
        roc: bars.iter().map(|b| b.and_then(|x| x.roc)).collect(),
        params: eng.params,
    })
}

fn rocp_validated(closes: &[f64], eng: ValidatedMom) -> FinanceResult<RocpSeries> {
    let mut st = MomState::new(eng.params)?;
    let bars = st.push_bars(closes)?;
    Ok(RocpSeries {
        rocp: bars.iter().map(|b| b.and_then(|x| x.rocp)).collect(),
        params: eng.params,
    })
}

#[derive(Clone, Debug)]
pub struct MomSolution {
    series: MomSeries,
    roc: RocSeries,
    close: Vec<f64>,
    formula: String,
}

impl MomSolution {
    pub fn series(&self) -> &MomSeries {
        &self.series
    }
    pub fn roc_series(&self) -> &RocSeries {
        &self.roc
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
            ("mom", "f", true),
            ("roc", "f", true),
        ]);
        let data = self
            .close
            .iter()
            .enumerate()
            .map(|(i, c)| {
                vec![
                    i.to_string(),
                    c.to_string(),
                    opt_cell(self.series.mom[i]),
                    opt_cell(self.roc.roc[i]),
                ]
            })
            .collect();
        print_table_locale_opt(&columns, data, locale, precision);
    }
}

/// # Examples
/// ```
/// use finance_solution::stocks::ta::{mom_solution, MomParams};
/// let c: Vec<_> = (0..20).map(|i| 100.0 + i as f64).collect();
/// let sol = mom_solution(&c, MomParams::period_10()).unwrap();
/// assert!(sol.formula().contains("10"));
/// ```
pub fn mom_solution(closes: &[f64], params: MomParams) -> FinanceResult<MomSolution> {
    let series = mom(closes, params)?;
    let roc = roc(closes, params)?;
    Ok(MomSolution {
        series,
        roc,
        close: closes.to_vec(),
        formula: format!(
            "MOM = C - C[n-{}]; ROC% = 100*(C-C_lag)/C_lag",
            params.period
        ),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mom_word_problem() {
        let c = [1.0, 2.0, 3.0, 4.0, 10.0];
        let s = mom(&c, MomParams::new(2)).unwrap();
        assert!(s.mom[0].is_none());
        assert!(s.mom[1].is_none());
        assert!((s.mom[2].unwrap() - 2.0).abs() < 1e-12); // 3-1
        assert!((s.mom[4].unwrap() - 7.0).abs() < 1e-12); // 10-3 (lag = i−period)
    }

    #[test]
    fn roc_percent() {
        let c = [100.0, 100.0, 110.0];
        let s = roc(&c, MomParams::new(2)).unwrap();
        assert!((s.roc[2].unwrap() - 10.0).abs() < 1e-12);
    }

    #[test]
    fn zero_lag_roc_none() {
        let c = [0.0, 1.0, 2.0];
        let s = roc(&c, MomParams::new(2)).unwrap();
        assert!(s.roc[2].is_none());
    }

    #[test]
    fn state_parity() {
        let c: Vec<_> = (0..30).map(|i| 100.0 + i as f64 * 0.5).collect();
        let p = MomParams::period_10();
        let batch = mom(&c, p).unwrap();
        let mut st = MomState::new(p).unwrap();
        for i in 0..c.len() {
            let o = st.push(c[i]).unwrap();
            match (o.and_then(|b| b.mom), batch.mom[i]) {
                (None, None) => {}
                (Some(a), Some(b)) => assert!((a - b).abs() < 1e-12),
                other => panic!("{other:?}"),
            }
        }
    }
}
