//! # Simple & exponential moving averages (SMA / EMA)
//!
//! Teaching + production building blocks for price smoothers. **Batch** APIs
//! ([`sma`], [`ema`]) and **incremental** APIs ([`SmaState`], [`EmaState`]) share one
//! implementation path: batch is “create state → [`push_bars`](SmaState::push_bars)”.
//!
//! ## Word problem
//!
//! > A stock closed at 10, 11, 12, 13, 14 over five days. What is the 3-day SMA on
//! > day 5?
//!
//! Expect: `(12 + 13 + 14) / 3 = 13`.
//!
//! ```
//! use finance_solution::stocks::ta::sma;
//! let closes = [10.0, 11.0, 12.0, 13.0, 14.0];
//! let s = sma(&closes, 3).unwrap();
//! // period index:     0     1     2     3     4
//! // warm-up:        None  None  Some  Some  Some
//! assert_eq!(s[0], None);
//! assert_eq!(s[1], None);
//! assert!((s[2].unwrap() - 11.0).abs() < 1e-12); // (10+11+12)/3
//! assert!((s[4].unwrap() - 13.0).abs() < 1e-12); // (12+13+14)/3
//! ```
//!
//! ## Quant pattern — one pack, many symbols
//!
//! ```
//! use finance_solution::stocks::ta::{SmaState, EmaState};
//!
//! // Live: hold state per symbol (your engine's HashMap)
//! let mut sma20 = SmaState::new(20).unwrap();
//! let mut ema20 = EmaState::new(20).unwrap();
//! # let payload = [100.0, 100.5, 101.0];
//! // One streaming payload with several bars:
//! let _ = sma20.push_bars(&payload).unwrap();
//! let _ = ema20.push_bars(&payload).unwrap();
//! // Or single bar:
//! let last_sma = sma20.push(101.2).unwrap(); // Option after warm-up
//! ```
//!
//! ## Formulas
//!
//! **SMA** over window of length `n`:
//!
//! ```text
//! SMA_t = (P_{t-n+1} + … + P_t) / n
//! ```
//!
//! **EMA** with span `n` (α = 2/(n+1)), seed = SMA of first `n` closes:
//!
//! ```text
//! EMA_seed = SMA(P_0..P_{n-1})
//! EMA_t    = α * P_t + (1-α) * EMA_{t-1}
//! ```
//!
//! ## Warm-up
//!
//! Output length = input length. Indices `0 .. n-2` are `None` until the window is full.
//!
//! ## Also here
//!
//! - **WMA** — linear weighted MA (newest bar has highest weight)
//! - **HMA** — [Hull](https://alanhull.com/) moving average: `WMA(2·WMA(n/2) − WMA(n), √n)`
//!
//! ## Related
//!
//! - Incremental: [`SmaState`], [`EmaState`], [`WmaState`], [`HmaState`]
//! - Used by: Bollinger (SMA mid), Keltner/MACD (EMA)

use crate::stocks::ta::ring::RingF64;
use crate::util::error::{require_finite, FinanceError, FinanceResult};
use crate::util::primitives::PeriodLength;

// ---------------------------------------------------------------------------
// Incremental state (canonical math path for SMA/EMA)
// ---------------------------------------------------------------------------

/// Incremental SMA. After warm-up, each [`SmaState::push`] is O(1).
///
/// Batch [`sma`] is implemented as `SmaState::new` + [`SmaState::push_bars`].
///
/// # Examples
/// ```
/// use finance_solution::stocks::ta::SmaState;
/// let mut s = SmaState::new(3).unwrap();
/// assert_eq!(s.push(1.0).unwrap(), None);
/// assert_eq!(s.push(2.0).unwrap(), None);
/// assert!((s.push(3.0).unwrap().unwrap() - 2.0).abs() < 1e-12);
/// assert!((s.push(6.0).unwrap().unwrap() - 3.666666666666).abs() < 1e-9);
/// ```
#[derive(Clone, Debug)]
pub struct SmaState {
    period: usize,
    ring: RingF64,
}

impl SmaState {
    /// Fallible constructor (`period >= 1`). Named `new` → [`FinanceResult`] (not `try_new`).
    pub fn new(period: usize) -> FinanceResult<Self> {
        let period = PeriodLength::new(period)?.get();
        Ok(Self {
            period,
            ring: RingF64::with_capacity(period),
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
        self.ring.clear();
    }

    /// Push one close. `None` until `period` samples seen.
    pub fn push(&mut self, close: f64) -> FinanceResult<Option<f64>> {
        require_finite("close", close)?;
        self.ring.push(close);
        if self.ring.is_full() {
            Ok(Some(self.ring.sum() / self.period as f64))
        } else {
            Ok(None)
        }
    }

    /// Push many closes (one streaming payload). One output per input.
    pub fn push_bars(&mut self, closes: &[f64]) -> FinanceResult<Vec<Option<f64>>> {
        let mut out = Vec::with_capacity(closes.len());
        for &c in closes {
            out.push(self.push(c)?);
        }
        Ok(out)
    }

    pub fn last(&self) -> Option<f64> {
        if self.ring.is_full() {
            Some(self.ring.sum() / self.period as f64)
        } else {
            None
        }
    }
}

/// Incremental EMA (α = 2/(period+1), seed = SMA of first `period` closes).
///
/// Batch [`ema`] uses this state end-to-end.
///
/// # Examples
/// ```
/// use finance_solution::stocks::ta::{EmaState, ema};
/// let closes: Vec<f64> = (1..=20).map(|x| x as f64).collect();
/// let batch = ema(&closes, 5).unwrap();
/// let mut st = EmaState::new(5).unwrap();
/// let mut last = None;
/// for &c in &closes {
///     last = st.push(c).unwrap();
/// }
/// assert!((last.unwrap() - batch[19].unwrap()).abs() < 1e-9);
/// ```
#[derive(Clone, Debug)]
pub struct EmaState {
    period: usize,
    alpha: f64,
    seed: RingF64,
    value: Option<f64>,
}

impl EmaState {
    pub fn new(period: usize) -> FinanceResult<Self> {
        let period = PeriodLength::new(period)?.get();
        Ok(Self {
            period,
            alpha: 2.0 / (period as f64 + 1.0),
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
        self.seed.push(close);
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

/// SMA of `period` closes. Leading `period - 1` values are `None`.
///
/// Implemented via [`SmaState::push_bars`] so batch and streaming stay bit-identical.
///
/// # Errors
/// Empty input, non-finite values, or `period == 0`.
///
/// # Examples
/// ```
/// use finance_solution::stocks::ta::sma;
/// let c = [1.0, 2.0, 3.0, 4.0, 5.0];
/// let s = sma(&c, 3).unwrap();
/// assert_eq!(s[0], None);
/// assert_eq!(s[1], None);
/// assert!((s[2].unwrap() - 2.0).abs() < 1e-12); // (1+2+3)/3
/// assert!((s[4].unwrap() - 4.0).abs() < 1e-12); // (3+4+5)/3
/// ```
///
/// Short series (shorter than period) — all `None`, still `Ok`:
/// ```
/// use finance_solution::stocks::ta::sma;
/// let s = sma(&[1.0, 2.0], 5).unwrap();
/// assert!(s.iter().all(|x| x.is_none()));
/// ```
pub fn sma(closes: &[f64], period: usize) -> FinanceResult<Vec<Option<f64>>> {
    validate_closes(closes)?;
    let mut st = SmaState::new(period)?;
    st.push_bars(closes)
}

/// EMA with span `period` (α = 2 / (period + 1)). Seed = SMA of the first `period` closes.
///
/// Implemented via [`EmaState::push_bars`].
///
/// # Errors
/// Same domain as [`sma`].
///
/// # Examples
/// ```
/// use finance_solution::stocks::ta::ema;
/// let c: Vec<f64> = (1..=20).map(|x| x as f64).collect();
/// let e = ema(&c, 5).unwrap();
/// assert!(e[3].is_none());
/// assert!(e[4].is_some());
/// assert!(e[19].unwrap().is_finite());
/// ```
pub fn ema(closes: &[f64], period: usize) -> FinanceResult<Vec<Option<f64>>> {
    validate_closes(closes)?;
    let mut st = EmaState::new(period)?;
    st.push_bars(closes)
}

/// Last defined SMA value, if any.
///
/// Uses [`SmaState`] end-to-end (no intermediate full `Vec` of options beyond the push loop).
/// Prefer holding an [`SmaState`] across live bars instead of calling this on growing history.
///
/// # Examples
/// ```
/// use finance_solution::stocks::ta::sma_last;
/// assert_eq!(sma_last(&[1.0, 2.0, 3.0], 3).unwrap(), Some(2.0));
/// assert_eq!(sma_last(&[1.0, 2.0], 3).unwrap(), None);
/// ```
#[inline]
pub fn sma_last(closes: &[f64], period: usize) -> FinanceResult<Option<f64>> {
    validate_closes(closes)?;
    let mut st = SmaState::new(period)?;
    for &c in closes {
        st.push(c)?;
    }
    Ok(st.last())
}

/// Last defined EMA value, if any (via [`EmaState`]).
///
/// # Examples
/// ```
/// use finance_solution::stocks::ta::{ema, ema_last};
/// let c: Vec<f64> = (1..=15).map(|x| x as f64).collect();
/// let series = ema(&c, 5).unwrap();
/// let last = ema_last(&c, 5).unwrap();
/// assert_eq!(last, series[14]);
/// ```
#[inline]
pub fn ema_last(closes: &[f64], period: usize) -> FinanceResult<Option<f64>> {
    validate_closes(closes)?;
    let mut st = EmaState::new(period)?;
    for &c in closes {
        st.push(c)?;
    }
    Ok(st.last())
}

// ---------------------------------------------------------------------------
// WMA (weighted moving average)
// ---------------------------------------------------------------------------

/// Incremental WMA: newest sample weight = `period`, oldest weight = 1.
///
/// \[
/// \mathrm{WMA} = \frac{\sum_{i=1}^{n} i\, P_{t-n+i}}{\sum_{i=1}^{n} i}
/// \]
#[derive(Clone, Debug)]
pub struct WmaState {
    period: usize,
    ring: RingF64,
    weight_sum: f64,
    ordered: Vec<f64>,
}

impl WmaState {
    pub fn new(period: usize) -> FinanceResult<Self> {
        let period = PeriodLength::new(period)?.get();
        let weight_sum = (period * (period + 1)) as f64 / 2.0;
        Ok(Self {
            period,
            ring: RingF64::with_capacity(period),
            weight_sum,
            ordered: Vec::with_capacity(period),
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
        self.ring.clear();
        self.ordered.clear();
    }

    pub fn push(&mut self, close: f64) -> FinanceResult<Option<f64>> {
        require_finite("close", close)?;
        self.ring.push(close);
        if !self.ring.is_full() {
            return Ok(None);
        }
        self.ring.copy_ordered(&mut self.ordered);
        let mut num = 0.0;
        for (i, &p) in self.ordered.iter().enumerate() {
            num += (i + 1) as f64 * p;
        }
        Ok(Some(num / self.weight_sum))
    }

    pub fn push_bars(&mut self, closes: &[f64]) -> FinanceResult<Vec<Option<f64>>> {
        let mut out = Vec::with_capacity(closes.len());
        for &c in closes {
            out.push(self.push(c)?);
        }
        Ok(out)
    }

    pub fn last(&self) -> Option<f64> {
        if !self.ring.is_full() {
            return None;
        }
        // recompute from ring (state may have been cloned)
        let mut ordered = Vec::with_capacity(self.period);
        self.ring.copy_ordered(&mut ordered);
        let mut num = 0.0;
        for (i, &p) in ordered.iter().enumerate() {
            num += (i + 1) as f64 * p;
        }
        Some(num / self.weight_sum)
    }
}

/// Weighted moving average (newest weight = `period`).
pub fn wma(closes: &[f64], period: usize) -> FinanceResult<Vec<Option<f64>>> {
    validate_closes(closes)?;
    let mut st = WmaState::new(period)?;
    st.push_bars(closes)
}

pub fn wma_last(closes: &[f64], period: usize) -> FinanceResult<Option<f64>> {
    validate_closes(closes)?;
    let mut st = WmaState::new(period)?;
    for &c in closes {
        st.push(c)?;
    }
    Ok(st.last())
}

// ---------------------------------------------------------------------------
// Hull moving average (HMA)
// ---------------------------------------------------------------------------

/// Hull moving average state.
///
/// ```text
/// raw = 2 * WMA(price, n/2) - WMA(price, n)
/// HMA = WMA(raw, floor(sqrt(n)))
/// ```
///
/// Requires `period >= 2`.
#[derive(Clone, Debug)]
pub struct HmaState {
    period: usize,
    half: WmaState,
    full: WmaState,
    sqrt_wma: WmaState,
    last: Option<f64>,
}

impl HmaState {
    pub fn new(period: usize) -> FinanceResult<Self> {
        let period = PeriodLength::new(period)?.get();
        if period < 2 {
            return Err(FinanceError::Unsolvable {
                message: "HMA period must be >= 2",
            });
        }
        let half_n = (period / 2).max(1);
        let sqrt_n = ((period as f64).sqrt().floor() as usize).max(1);
        Ok(Self {
            period,
            half: WmaState::new(half_n)?,
            full: WmaState::new(period)?,
            sqrt_wma: WmaState::new(sqrt_n)?,
            last: None,
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
        self.half.reset();
        self.full.reset();
        self.sqrt_wma.reset();
        self.last = None;
    }

    pub fn push(&mut self, close: f64) -> FinanceResult<Option<f64>> {
        let wh = self.half.push(close)?;
        let wf = self.full.push(close)?;
        let out = match (wh, wf) {
            (Some(h), Some(f)) => {
                let raw = 2.0 * h - f;
                self.sqrt_wma.push(raw)?
            }
            _ => None,
        };
        self.last = out;
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
        // Prefer cached last; fall back to sqrt WMA after clone without re-push.
        self.last.or_else(|| self.sqrt_wma.last())
    }
}

/// Hull moving average of `period` (must be ≥ 2).
///
/// # Examples
/// ```
/// use finance_solution::stocks::ta::hma;
/// let c: Vec<f64> = (1..=40).map(|x| x as f64).collect();
/// let h = hma(&c, 9).unwrap();
/// assert!(h.iter().any(|x| x.is_some()));
/// assert!(h.last().unwrap().unwrap().is_finite());
/// ```
pub fn hma(closes: &[f64], period: usize) -> FinanceResult<Vec<Option<f64>>> {
    validate_closes(closes)?;
    let mut st = HmaState::new(period)?;
    st.push_bars(closes)
}

pub fn hma_last(closes: &[f64], period: usize) -> FinanceResult<Option<f64>> {
    validate_closes(closes)?;
    let mut st = HmaState::new(period)?;
    for &c in closes {
        st.push(c)?;
    }
    Ok(st.last())
}

fn validate_closes(closes: &[f64]) -> FinanceResult<()> {
    if closes.is_empty() {
        return Err(FinanceError::EmptyInput { what: "closes" });
    }
    for &c in closes {
        require_finite("close", c)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sma_constant() {
        let c = [10.0; 5];
        let s = sma(&c, 3).unwrap();
        assert_eq!(s[2], Some(10.0));
        assert_eq!(s[4], Some(10.0));
    }

    #[test]
    fn ema_runs() {
        let c: Vec<_> = (1..=30).map(|x| x as f64).collect();
        let e = ema(&c, 10).unwrap();
        assert!(e[8].is_none());
        assert!(e[9].is_some());
    }

    #[test]
    fn rejects_zero_period() {
        assert!(sma(&[1.0, 2.0], 0).is_err());
    }

    #[test]
    fn sma_period_one_is_identity() {
        let c = [1.0, 2.0, 3.0];
        let s = sma(&c, 1).unwrap();
        assert_eq!(s[0], Some(1.0));
        assert_eq!(s[2], Some(3.0));
    }

    #[test]
    fn ema_seed_is_sma() {
        let c = [1.0, 2.0, 3.0, 4.0, 5.0];
        let e = ema(&c, 3).unwrap();
        // First EMA value at index 2 = SMA(1,2,3) = 2
        assert!((e[2].unwrap() - 2.0).abs() < 1e-12);
    }

    #[test]
    fn empty_series_err() {
        assert!(sma(&[], 3).is_err());
        assert!(ema(&[], 3).is_err());
    }

    #[test]
    fn nan_close_err() {
        assert!(sma(&[1.0, f64::NAN], 2).is_err());
    }

    #[test]
    fn last_matches_series_tail() {
        let c: Vec<_> = (1..=25).map(|x| x as f64 * 0.5).collect();
        let s = sma(&c, 7).unwrap();
        assert_eq!(sma_last(&c, 7).unwrap(), s[24]);
        let e = ema(&c, 7).unwrap();
        assert_eq!(ema_last(&c, 7).unwrap(), e[24]);
    }

    #[test]
    fn wma_weights_newest_heavier() {
        // window [1,2,3]: WMA = (1*1+2*2+3*3)/(1+2+3) = 14/6
        let s = wma(&[1.0, 2.0, 3.0], 3).unwrap();
        assert!((s[2].unwrap() - 14.0 / 6.0).abs() < 1e-12);
    }

    #[test]
    fn hma_state_parity() {
        let c: Vec<f64> = (1..=50).map(|x| 100.0 + x as f64 * 0.1).collect();
        let batch = hma(&c, 16).unwrap();
        let st = HmaState::from_history(16, &c).unwrap();
        assert!((batch.last().unwrap().unwrap() - st.last().unwrap()).abs() < 1e-9);
    }

    #[test]
    fn hma_rejects_period_one() {
        assert!(HmaState::new(1).is_err());
    }

    #[test]
    fn hma_tracks_rising_path() {
        let c: Vec<f64> = (1..=60).map(|x| x as f64).collect();
        let h = hma(&c, 9).unwrap();
        let last = h.iter().rev().find_map(|x| *x).unwrap();
        // Rising line: HMA should sit near the recent levels (well above early prices).
        assert!(last > 50.0, "hma last={last}");
    }

    #[test]
    fn wma_last_matches_series() {
        let c: Vec<f64> = (1..=20).map(|x| x as f64).collect();
        let s = wma(&c, 5).unwrap();
        assert_eq!(wma_last(&c, 5).unwrap(), s[19]);
    }

    #[test]
    fn hma_reset_clears() {
        let c: Vec<f64> = (1..=30).map(|x| x as f64).collect();
        let mut st = HmaState::from_history(9, &c).unwrap();
        assert!(st.last().is_some());
        st.reset();
        assert!(st.last().is_none());
    }
}
