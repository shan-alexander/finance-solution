//! # Advanced moving averages: DEMA, TEMA, RMA (Wilder), KAMA
//!
//! | Name | Formula sketch |
//! |------|----------------|
//! | **RMA / Wilder** | α = 1/n; seed = SMA; then α·P + (1−α)·prev |
//! | **DEMA** | `2·EMA − EMA(EMA)` |
//! | **TEMA** | `3·EMA − 3·EMA(EMA) + EMA(EMA(EMA))` |
//! | **KAMA** | Kaufman adaptive: efficiency ratio scales SC between fast/slow |
//!
//! Defaults: RMA/DEMA/TEMA period **20**; KAMA **(10, 2, 30)**.
//!
//! Batch free functions use the matching `*State` end-to-end.

use crate::stocks::ta::common::validate_series;
use crate::stocks::ta::moving_average::EmaState;
use crate::stocks::ta::ring::RingF64;
use crate::util::error::{require_finite, FinanceError, FinanceResult};
use crate::util::primitives::PeriodLength;

// ---------------------------------------------------------------------------
// RMA (Wilder / SMMA)
// ---------------------------------------------------------------------------

/// Incremental Wilder RMA (also called SMMA). α = 1/`period`.
#[derive(Clone, Debug)]
pub struct RmaState {
    period: usize,
    alpha: f64,
    seed: RingF64,
    value: Option<f64>,
}

impl RmaState {
    pub fn new(period: usize) -> FinanceResult<Self> {
        let period = PeriodLength::new(period)?.get();
        Ok(Self {
            period,
            alpha: 1.0 / period as f64,
            seed: RingF64::with_capacity(period),
            value: None,
        })
    }

    pub fn from_history(period: usize, closes: &[f64]) -> FinanceResult<Self> {
        let mut s = Self::new(period)?;
        s.push_bars(closes)?;
        Ok(s)
    }

    pub fn period(&self) -> usize {
        self.period
    }

    pub fn reset(&mut self) {
        self.seed.clear();
        self.value = None;
    }

    pub fn push(&mut self, close: f64) -> FinanceResult<Option<f64>> {
        require_finite("close", close)?;
        if let Some(prev) = self.value {
            let next = self.alpha * close + (1.0 - self.alpha) * prev;
            self.value = Some(next);
            return Ok(Some(next));
        }
        let _ = self.seed.push(close);
        if self.seed.is_full() {
            let seed = self.seed.sum() / self.period as f64;
            self.value = Some(seed);
            Ok(Some(seed))
        } else {
            Ok(None)
        }
    }

    pub fn push_bars(&mut self, closes: &[f64]) -> FinanceResult<Vec<Option<f64>>> {
        let mut out = Vec::with_capacity(closes.len());
        for &c in closes {
            out.push(self.push(c)?);
        }
        Ok(out)
    }

    pub fn last(&self) -> Option<f64> {
        self.value
    }
}

/// Wilder RMA / SMMA of `period` closes.
pub fn rma(closes: &[f64], period: usize) -> FinanceResult<Vec<Option<f64>>> {
    validate_series("close", closes)?;
    let mut st = RmaState::new(period)?;
    st.push_bars(closes)
}

pub fn rma_last(closes: &[f64], period: usize) -> FinanceResult<Option<f64>> {
    let mut st = RmaState::new(period)?;
    for &c in closes {
        st.push(c)?;
    }
    Ok(st.last())
}

// ---------------------------------------------------------------------------
// DEMA
// ---------------------------------------------------------------------------

/// Double exponential moving average: `2·EMA − EMA(EMA)`.
#[derive(Clone, Debug)]
pub struct DemaState {
    e1: EmaState,
    e2: EmaState,
}

impl DemaState {
    pub fn new(period: usize) -> FinanceResult<Self> {
        Ok(Self {
            e1: EmaState::new(period)?,
            e2: EmaState::new(period)?,
        })
    }

    pub fn from_history(period: usize, closes: &[f64]) -> FinanceResult<Self> {
        let mut s = Self::new(period)?;
        s.push_bars(closes)?;
        Ok(s)
    }

    pub fn period(&self) -> usize {
        self.e1.period()
    }

    pub fn reset(&mut self) {
        self.e1.reset();
        self.e2.reset();
    }

    pub fn push(&mut self, close: f64) -> FinanceResult<Option<f64>> {
        let e1 = match self.e1.push(close)? {
            Some(v) => v,
            None => return Ok(None),
        };
        match self.e2.push(e1)? {
            Some(e2) => Ok(Some(2.0 * e1 - e2)),
            None => Ok(None),
        }
    }

    pub fn push_bars(&mut self, closes: &[f64]) -> FinanceResult<Vec<Option<f64>>> {
        let mut out = Vec::with_capacity(closes.len());
        for &c in closes {
            out.push(self.push(c)?);
        }
        Ok(out)
    }

    pub fn last(&self) -> Option<f64> {
        match (self.e1.last(), self.e2.last()) {
            (Some(e1), Some(e2)) => Some(2.0 * e1 - e2),
            _ => None,
        }
    }
}

/// DEMA of `period` closes.
pub fn dema(closes: &[f64], period: usize) -> FinanceResult<Vec<Option<f64>>> {
    validate_series("close", closes)?;
    let mut st = DemaState::new(period)?;
    st.push_bars(closes)
}

pub fn dema_last(closes: &[f64], period: usize) -> FinanceResult<Option<f64>> {
    let mut st = DemaState::new(period)?;
    for &c in closes {
        st.push(c)?;
    }
    Ok(st.last())
}

// ---------------------------------------------------------------------------
// TEMA
// ---------------------------------------------------------------------------

/// Triple exponential moving average: `3·e1 − 3·e2 + e3`.
#[derive(Clone, Debug)]
pub struct TemaState {
    e1: EmaState,
    e2: EmaState,
    e3: EmaState,
}

impl TemaState {
    pub fn new(period: usize) -> FinanceResult<Self> {
        Ok(Self {
            e1: EmaState::new(period)?,
            e2: EmaState::new(period)?,
            e3: EmaState::new(period)?,
        })
    }

    pub fn from_history(period: usize, closes: &[f64]) -> FinanceResult<Self> {
        let mut s = Self::new(period)?;
        s.push_bars(closes)?;
        Ok(s)
    }

    pub fn period(&self) -> usize {
        self.e1.period()
    }

    pub fn reset(&mut self) {
        self.e1.reset();
        self.e2.reset();
        self.e3.reset();
    }

    pub fn push(&mut self, close: f64) -> FinanceResult<Option<f64>> {
        let e1 = match self.e1.push(close)? {
            Some(v) => v,
            None => return Ok(None),
        };
        let e2 = match self.e2.push(e1)? {
            Some(v) => v,
            None => return Ok(None),
        };
        match self.e3.push(e2)? {
            Some(e3) => Ok(Some(3.0 * e1 - 3.0 * e2 + e3)),
            None => Ok(None),
        }
    }

    pub fn push_bars(&mut self, closes: &[f64]) -> FinanceResult<Vec<Option<f64>>> {
        let mut out = Vec::with_capacity(closes.len());
        for &c in closes {
            out.push(self.push(c)?);
        }
        Ok(out)
    }

    pub fn last(&self) -> Option<f64> {
        match (self.e1.last(), self.e2.last(), self.e3.last()) {
            (Some(e1), Some(e2), Some(e3)) => Some(3.0 * e1 - 3.0 * e2 + e3),
            _ => None,
        }
    }
}

/// TEMA of `period` closes.
pub fn tema(closes: &[f64], period: usize) -> FinanceResult<Vec<Option<f64>>> {
    validate_series("close", closes)?;
    let mut st = TemaState::new(period)?;
    st.push_bars(closes)
}

pub fn tema_last(closes: &[f64], period: usize) -> FinanceResult<Option<f64>> {
    let mut st = TemaState::new(period)?;
    for &c in closes {
        st.push(c)?;
    }
    Ok(st.last())
}

// ---------------------------------------------------------------------------
// KAMA
// ---------------------------------------------------------------------------

/// Kaufman Adaptive Moving Average parameters.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct KamaParams {
    /// Efficiency-ratio lookback (typically 10).
    pub period: usize,
    /// Fast EMA-equivalent period (typically 2).
    pub fast: usize,
    /// Slow EMA-equivalent period (typically 30).
    pub slow: usize,
}

impl KamaParams {
    pub const fn new(period: usize, fast: usize, slow: usize) -> Self {
        Self { period, fast, slow }
    }

    /// Classic `(10, 2, 30)`.
    pub const fn standard() -> Self {
        Self {
            period: 10,
            fast: 2,
            slow: 30,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ValidatedKama {
    params: KamaParams,
}

impl ValidatedKama {
    pub fn new(params: KamaParams) -> FinanceResult<Self> {
        PeriodLength::new(params.period)?;
        PeriodLength::new(params.fast)?;
        PeriodLength::new(params.slow)?;
        if params.fast >= params.slow {
            return Err(FinanceError::Unsolvable {
                message: "KAMA requires fast < slow",
            });
        }
        Ok(Self { params })
    }

    pub fn params(self) -> KamaParams {
        self.params
    }

    pub fn compute(self, closes: &[f64]) -> FinanceResult<Vec<Option<f64>>> {
        kama(closes, self.params)
    }
}

/// Incremental KAMA.
#[derive(Clone, Debug)]
pub struct KamaState {
    params: KamaParams,
    fast_sc: f64,
    slow_sc: f64,
    /// Last `period` closes for ER (capacity period+? we need change over period and n diffs)
    closes: RingF64,
    kama: Option<f64>,
    last: Option<f64>,
}

impl KamaState {
    pub fn new(params: KamaParams) -> FinanceResult<Self> {
        let _ = ValidatedKama::new(params)?;
        let fast_sc = 2.0 / (params.fast as f64 + 1.0);
        let slow_sc = 2.0 / (params.slow as f64 + 1.0);
        Ok(Self {
            params,
            fast_sc,
            slow_sc,
            closes: RingF64::with_capacity(params.period + 1),
            kama: None,
            last: None,
        })
    }

    pub fn from_history(params: KamaParams, closes: &[f64]) -> FinanceResult<Self> {
        let mut s = Self::new(params)?;
        s.push_bars(closes)?;
        Ok(s)
    }

    pub fn params(&self) -> KamaParams {
        self.params
    }

    pub fn reset(&mut self) {
        self.closes.clear();
        self.kama = None;
        self.last = None;
    }

    pub fn push(&mut self, close: f64) -> FinanceResult<Option<f64>> {
        require_finite("close", close)?;
        let _ = self.closes.push(close);
        // Need period+1 samples for ER over `period` steps
        if self.closes.len() < self.params.period + 1 {
            self.last = None;
            return Ok(None);
        }
        let mut ordered = Vec::with_capacity(self.params.period + 1);
        self.closes.copy_ordered(&mut ordered);
        let n = ordered.len();
        let change = (ordered[n - 1] - ordered[0]).abs();
        let mut volatility = 0.0;
        for i in 1..n {
            volatility += (ordered[i] - ordered[i - 1]).abs();
        }
        let er = if volatility > 0.0 {
            change / volatility
        } else {
            0.0
        };
        let sc = (er * (self.fast_sc - self.slow_sc) + self.slow_sc).powi(2);
        let kama = match self.kama {
            None => close, // first KAMA = price when ER window first fills
            Some(prev) => prev + sc * (close - prev),
        };
        self.kama = Some(kama);
        self.last = Some(kama);
        Ok(Some(kama))
    }

    pub fn push_bars(&mut self, closes: &[f64]) -> FinanceResult<Vec<Option<f64>>> {
        validate_series("close", closes)?;
        let mut out = Vec::with_capacity(closes.len());
        for &c in closes {
            out.push(self.push(c)?);
        }
        Ok(out)
    }

    pub fn last(&self) -> Option<f64> {
        self.last
    }
}

/// Kaufman adaptive moving average.
pub fn kama(closes: &[f64], params: KamaParams) -> FinanceResult<Vec<Option<f64>>> {
    let mut st = KamaState::new(params)?;
    st.push_bars(closes)
}

pub fn kama_last(closes: &[f64], params: KamaParams) -> FinanceResult<Option<f64>> {
    let mut st = KamaState::new(params)?;
    for &c in closes {
        st.push(c)?;
    }
    Ok(st.last())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stocks::ta::moving_average::ema;

    #[test]
    fn rma_matches_wilder_seed() {
        let c: Vec<_> = (1..=20).map(|x| x as f64).collect();
        let r = rma(&c, 5).unwrap();
        // First RMA at index 4 = SMA(1..5)=3
        assert!((r[4].unwrap() - 3.0).abs() < 1e-12);
        // Next: (3*4 + 6)/5 = 3.6
        assert!((r[5].unwrap() - 3.6).abs() < 1e-12);
    }

    #[test]
    fn dema_related_to_ema() {
        let c: Vec<_> = (1..=40).map(|x| x as f64).collect();
        let d = dema(&c, 5).unwrap();
        let e1 = ema(&c, 5).unwrap();
        // DEMA should be defined later than EMA
        let first_e = e1.iter().position(|x| x.is_some()).unwrap();
        let first_d = d.iter().position(|x| x.is_some()).unwrap();
        assert!(first_d >= first_e);
        assert!(d.last().unwrap().is_some());
    }

    #[test]
    fn tema_runs() {
        let c: Vec<_> = (1..=50).map(|x| 100.0 + x as f64 * 0.1).collect();
        let t = tema(&c, 5).unwrap();
        assert!(t.iter().filter(|x| x.is_some()).count() > 10);
    }

    #[test]
    fn kama_standard_runs() {
        let c: Vec<_> = (0..40).map(|i| 100.0 + (i as f64).sin() * 2.0).collect();
        let k = kama(&c, KamaParams::standard()).unwrap();
        assert!(k[9].is_none() || k[10].is_some()); // first at period (index period)
        assert!(k.last().unwrap().is_some());
    }

    #[test]
    fn dema_tema_state_parity() {
        let c: Vec<_> = (0..45).map(|i| 50.0 + i as f64 * 0.25).collect();
        for (batch, mut st) in [(dema(&c, 8).unwrap(), DemaState::new(8).unwrap())] {
            for i in 0..c.len() {
                let o = st.push(c[i]).unwrap();
                match (o, batch[i]) {
                    (None, None) => {}
                    (Some(a), Some(b)) => assert!((a - b).abs() < 1e-9),
                    other => panic!("{other:?}"),
                }
            }
        }
        let batch = tema(&c, 6).unwrap();
        let mut st = TemaState::new(6).unwrap();
        for i in 0..c.len() {
            let o = st.push(c[i]).unwrap();
            match (o, batch[i]) {
                (None, None) => {}
                (Some(a), Some(b)) => assert!((a - b).abs() < 1e-9),
                other => panic!("{other:?}"),
            }
        }
        let batch = rma(&c, 7).unwrap();
        let mut st = RmaState::new(7).unwrap();
        for i in 0..c.len() {
            let o = st.push(c[i]).unwrap();
            match (o, batch[i]) {
                (None, None) => {}
                (Some(a), Some(b)) => assert!((a - b).abs() < 1e-9),
                other => panic!("{other:?}"),
            }
        }
        let p = KamaParams::standard();
        let batch = kama(&c, p).unwrap();
        let mut st = KamaState::new(p).unwrap();
        for i in 0..c.len() {
            let o = st.push(c[i]).unwrap();
            match (o, batch[i]) {
                (None, None) => {}
                (Some(a), Some(b)) => assert!((a - b).abs() < 1e-9),
                other => panic!("{other:?}"),
            }
        }
    }

    #[test]
    fn kama_fast_ge_slow_err() {
        assert!(ValidatedKama::new(KamaParams::new(10, 30, 2)).is_err());
    }
}
