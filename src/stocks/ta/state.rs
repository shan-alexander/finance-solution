//! Incremental **state machines** for live / streaming bar updates.
//!
//! # What this is
//!
//! Pure math state: [`push`](SmaState::push) one bar at a time, [`push_bars`](SmaState::push_bars)
//! for multi-bar payloads, or seed with [`from_history`](SmaState::from_history).
//! **Not** a market-data engine — your quant system owns the feed, symbols, and calendars.
//!
//! # Constructor naming: `new` → `FinanceResult` (not `try_new`)
//!
//! Fallible construction uses **`new`**, matching this crate’s Result-only style
//! (`Schedule::new_repeating`, `File::open` in the standard library — fallibility lives in the
//! return type, not a `try_` prefix). There is no panicking twin.
//!
//! # What this is not
//!
//! - No sockets, no multi-symbol registry, no auto “session open”
//! - Day reset of VWAP/RVOL is **your** call to [`VwapState::reset`] / rebuild
//!
//! # Quant engine sketch (per symbol)
//!
//! ```
//! use finance_solution::stocks::ta::{
//!     StochasticParams, StochState, VwapParams, VwapState, EmaState,
//! };
//!
//! const FAST: StochasticParams = StochasticParams::fast(9, 3);
//!
//! struct SymbolPipeline {
//!     stoch: StochState,
//!     vwap: VwapState,
//!     ema20: EmaState,
//! }
//!
//! impl SymbolPipeline {
//!     fn new() -> finance_solution::FinanceResult<Self> {
//!         Ok(Self {
//!             stoch: StochState::new(FAST)?,
//!             vwap: VwapState::new(VwapParams::cumulative_typical())?,
//!             ema20: EmaState::new(20)?,
//!         })
//!     }
//!
//!     /// Seed from historical bars, then only push live bars.
//!     fn seed_history(
//!         &mut self,
//!         high: &[f64],
//!         low: &[f64],
//!         close: &[f64],
//!         volume: &[f64],
//!     ) -> finance_solution::FinanceResult<()> {
//!         self.stoch = StochState::from_history(FAST, high, low, close)?;
//!         self.vwap = VwapState::from_history(
//!             VwapParams::cumulative_typical(),
//!             high, low, close, volume,
//!         )?;
//!         self.ema20 = EmaState::from_history(20, close)?;
//!         Ok(())
//!     }
//!
//!     fn on_bar(
//!         &mut self,
//!         high: f64,
//!         low: f64,
//!         close: f64,
//!         volume: f64,
//!     ) -> finance_solution::FinanceResult<()> {
//!         let _kd = self.stoch.push(high, low, close)?;
//!         let _vw = self.vwap.push(high, low, close, volume)?;
//!         let _e = self.ema20.push(close)?;
//!         Ok(())
//!     }
//!
//!     /// Caller owns the calendar — e.g. regular-session open.
//!     fn on_session_open_reset_vwap(&mut self) {
//!         self.vwap.reset();
//!         // stoch/ema often continue; reset only if *your* strategy wants it
//!     }
//! }
//!
//! # let mut p = SymbolPipeline::new().unwrap();
//! # p.on_bar(10.0, 9.0, 9.5, 1000.0).unwrap();
//! ```
//!
//! # Parity with batch
//!
//! Streaming state is defined to match batch series functions on the same path
//! (within floating-point tolerance). Prefer batch [`compute`](super::ValidatedStochastic::compute)
//! for research; prefer state for live multi-symbol updates.
//!
//! | Batch | Incremental |
//! |-------|-------------|
//! | [`sma`](super::sma) | [`SmaState`] |
//! | [`ema`](super::ema) | [`EmaState`] |
//! | [`stochastics`](super::stochastics) | [`StochState`] |
//! | [`macd`](super::macd) | [`MacdState`] |
//! | [`bollinger`](super::bollinger) | [`BollingerState`] |
//! | [`keltner`](super::keltner) | [`KeltnerState`] |
//! | [`vwap`](super::vwap) | [`VwapState`] |
//! | [`rvol`](super::rvol) | [`RvolState`] |
//! | [`rsi`](super::rsi) | [`RsiState`] |
//! | [`atr`](super::atr) | [`AtrState`] |

use crate::stocks::ta::bollinger::{BollingerParams, ValidatedBollinger};
use crate::stocks::ta::common::{require_hlc, validate_positive_volume, window_stdev};
use crate::stocks::ta::keltner::{KeltnerParams, ValidatedKeltner};
use crate::stocks::ta::macd::{MacdParams, ValidatedMacd};
use crate::stocks::ta::ring::{RingF64, RingPv};
use crate::stocks::ta::rvol::{RvolParams, ValidatedRvol};
use crate::stocks::ta::stochastic::{StochasticParams, ValidatedStochastic};
use crate::stocks::ta::vwap::{ValidatedVwap, VwapMode, VwapParams, VwapPriceSource};
use crate::util::error::{require_finite, FinanceError, FinanceResult};

/// SMA/EMA state live in [`moving_average`](super::moving_average); re-exported here for the
/// incremental-API index.
pub use crate::stocks::ta::moving_average::{EmaState, SmaState};

// ---------------------------------------------------------------------------
// Stochastic
// ---------------------------------------------------------------------------

/// Incremental stochastic (fast/full via [`StochasticParams`]).
///
/// # Examples
/// ```
/// use finance_solution::stocks::ta::{StochasticParams, StochState, stochastics};
/// const P: StochasticParams = StochasticParams::fast(5, 3);
/// let h: Vec<_> = (0..20).map(|i| 10.0 + i as f64).collect();
/// let l: Vec<_> = (0..20).map(|i| 9.0 + i as f64).collect();
/// let c: Vec<_> = (0..20).map(|i| 9.5 + i as f64).collect();
/// let batch = stochastics(&h, &l, &c, P).unwrap();
/// let mut st = StochState::new(P).unwrap();
/// let mut last = None;
/// for i in 0..20 {
///     last = st.push(h[i], l[i], c[i]).unwrap();
/// }
/// let (bk, bd) = batch.last_kd().unwrap();
/// let (sk, sd) = last.unwrap();
/// assert!((bk - sk).abs() < 1e-9 && (bd - sd).abs() < 1e-9);
/// ```
#[derive(Clone, Debug)]
pub struct StochState {
    params: StochasticParams,
    high: RingF64,
    low: RingF64,
    close: RingF64,
    raw_k: RingF64,
    smooth_k: RingF64,
    last_k: Option<f64>,
    last_d: Option<f64>,
    /// Last raw %K before smoothing (for flat-window carry-forward).
    prev_raw_k: Option<f64>,
}

impl StochState {
    pub fn new(params: StochasticParams) -> FinanceResult<Self> {
        let _ = ValidatedStochastic::new(params)?;
        Ok(Self {
            params,
            high: RingF64::with_capacity(params.k_period),
            low: RingF64::with_capacity(params.k_period),
            close: RingF64::with_capacity(params.k_period),
            raw_k: RingF64::with_capacity(params.k_smooth),
            smooth_k: RingF64::with_capacity(params.d_period),
            last_k: None,
            last_d: None,
            prev_raw_k: None,
        })
    }

    pub fn from_history(
        params: StochasticParams,
        high: &[f64],
        low: &[f64],
        close: &[f64],
    ) -> FinanceResult<Self> {
        let mut s = Self::new(params)?;
        require_hlc(high, low, close)?;
        for i in 0..close.len() {
            s.push(high[i], low[i], close[i])?;
        }
        Ok(s)
    }

    pub fn params(&self) -> StochasticParams {
        self.params
    }

    pub fn reset(&mut self) {
        self.high.clear();
        self.low.clear();
        self.close.clear();
        self.raw_k.clear();
        self.smooth_k.clear();
        self.last_k = None;
        self.last_d = None;
        self.prev_raw_k = None;
    }

    /// Returns `Some((%K, %D))` when both are defined; `%K`-only warm-up yields `None`
    /// until `%D` is ready (same as batch last_kd semantics for the pair).
    ///
    /// For finer control use [`StochState::push_detail`].
    pub fn push(&mut self, high: f64, low: f64, close: f64) -> FinanceResult<Option<(f64, f64)>> {
        let d = self.push_detail(high, low, close)?;
        match (d.k, d.d) {
            (Some(k), Some(dd)) => Ok(Some((k, dd))),
            _ => Ok(None),
        }
    }

    /// Full detail for this bar (either line may still be warming up).
    pub fn push_detail(
        &mut self,
        high: f64,
        low: f64,
        close: f64,
    ) -> FinanceResult<StochBarOutput> {
        require_finite("high", high)?;
        require_finite("low", low)?;
        require_finite("close", close)?;
        if high < low {
            return Err(FinanceError::InvalidCashflow {
                message: "high must be >= low for each bar",
            });
        }
        self.high.push(high);
        self.low.push(low);
        self.close.push(close);

        let mut k_out = None;
        let mut d_out = None;

        if self.high.is_full() {
            let hh = self.high.max().unwrap();
            let ll = self.low.min().unwrap();
            let range = hh - ll;
            // Flat window: carry previous raw %K, else 50 (matches batch stochastics).
            let raw = if range == 0.0 {
                self.prev_raw_k.unwrap_or(50.0)
            } else {
                100.0 * (close - ll) / range
            };
            self.prev_raw_k = Some(raw);
            self.raw_k.push(raw);
            if self.raw_k.is_full() {
                let sk = self.raw_k.sum() / self.params.k_smooth as f64;
                self.last_k = Some(sk);
                k_out = Some(sk);
                self.smooth_k.push(sk);
                if self.smooth_k.is_full() {
                    let d = self.smooth_k.sum() / self.params.d_period as f64;
                    self.last_d = Some(d);
                    d_out = Some(d);
                }
            }
        }
        Ok(StochBarOutput { k: k_out, d: d_out })
    }

    pub fn last_kd(&self) -> Option<(f64, f64)> {
        Some((self.last_k?, self.last_d?))
    }

    /// Push aligned H/L/C slices (same length) from one streaming payload.
    pub fn push_bars(
        &mut self,
        high: &[f64],
        low: &[f64],
        close: &[f64],
    ) -> FinanceResult<Vec<StochBarOutput>> {
        require_hlc(high, low, close)?;
        let mut out = Vec::with_capacity(close.len());
        for i in 0..close.len() {
            out.push(self.push_detail(high[i], low[i], close[i])?);
        }
        Ok(out)
    }
}

/// One-bar stochastic output (warm-up allowed as `None`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StochBarOutput {
    pub k: Option<f64>,
    pub d: Option<f64>,
}

// ---------------------------------------------------------------------------
// MACD
// ---------------------------------------------------------------------------

/// Incremental MACD (fast/slow/signal EMAs).
#[derive(Clone, Debug)]
pub struct MacdState {
    params: MacdParams,
    fast: EmaState,
    slow: EmaState,
    signal: EmaState,
    last: Option<(f64, f64, f64)>,
}

impl MacdState {
    pub fn new(params: MacdParams) -> FinanceResult<Self> {
        let _ = ValidatedMacd::new(params)?;
        Ok(Self {
            params,
            fast: EmaState::new(params.fast)?,
            slow: EmaState::new(params.slow)?,
            signal: EmaState::new(params.signal)?,
            last: None,
        })
    }

    pub fn from_history(params: MacdParams, closes: &[f64]) -> FinanceResult<Self> {
        let mut s = Self::new(params)?;
        for &c in closes {
            s.push(c)?;
        }
        Ok(s)
    }

    pub fn params(&self) -> MacdParams {
        self.params
    }

    pub fn reset(&mut self) {
        self.fast.reset();
        self.slow.reset();
        self.signal.reset();
        self.last = None;
    }

    /// `Some((macd, signal, hist))` when all three are defined.
    pub fn push(&mut self, close: f64) -> FinanceResult<Option<(f64, f64, f64)>> {
        let f = self.fast.push(close)?;
        let s = self.slow.push(close)?;
        let macd_line = match (f, s) {
            (Some(a), Some(b)) => a - b,
            _ => return Ok(None),
        };
        let sig = self.signal.push(macd_line)?;
        match sig {
            Some(signal) => {
                let hist = macd_line - signal;
                self.last = Some((macd_line, signal, hist));
                Ok(Some((macd_line, signal, hist)))
            }
            None => {
                self.last = None;
                Ok(None)
            }
        }
    }

    pub fn last(&self) -> Option<(f64, f64, f64)> {
        self.last
    }

    /// Push many closes from one payload.
    pub fn push_bars(&mut self, closes: &[f64]) -> FinanceResult<Vec<Option<(f64, f64, f64)>>> {
        let mut out = Vec::with_capacity(closes.len());
        for &c in closes {
            out.push(self.push(c)?);
        }
        Ok(out)
    }
}

// ---------------------------------------------------------------------------
// Bollinger
// ---------------------------------------------------------------------------

/// Incremental Bollinger Bands (sample stdev on the window).
#[derive(Clone, Debug)]
pub struct BollingerState {
    params: BollingerParams,
    ring: RingF64,
    scratch: Vec<f64>,
    last: Option<BollingerBarOutput>,
}

/// One-bar Bollinger output.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BollingerBarOutput {
    pub middle: f64,
    pub upper: f64,
    pub lower: f64,
    pub pct_b: Option<f64>,
}

impl BollingerState {
    pub fn new(params: BollingerParams) -> FinanceResult<Self> {
        let _ = ValidatedBollinger::new(params)?;
        Ok(Self {
            params,
            ring: RingF64::with_capacity(params.period),
            scratch: Vec::with_capacity(params.period),
            last: None,
        })
    }

    pub fn from_history(params: BollingerParams, closes: &[f64]) -> FinanceResult<Self> {
        let mut s = Self::new(params)?;
        for &c in closes {
            s.push(c)?;
        }
        Ok(s)
    }

    pub fn params(&self) -> BollingerParams {
        self.params
    }

    pub fn reset(&mut self) {
        self.ring.clear();
        self.last = None;
    }

    pub fn push(&mut self, close: f64) -> FinanceResult<Option<BollingerBarOutput>> {
        require_finite("close", close)?;
        self.ring.push(close);
        if !self.ring.is_full() {
            self.last = None;
            return Ok(None);
        }
        self.ring.copy_ordered(&mut self.scratch);
        let mid = self.ring.sum() / self.params.period as f64;
        let sd = window_stdev(&self.scratch, self.params.stdev).unwrap_or(0.0);
        let band = self.params.num_std * sd;
        let upper = mid + band;
        let lower = mid - band;
        let width = upper - lower;
        let pct_b = if width > 0.0 {
            Some((close - lower) / width)
        } else {
            None
        };
        let out = BollingerBarOutput {
            middle: mid,
            upper,
            lower,
            pct_b,
        };
        self.last = Some(out);
        Ok(Some(out))
    }

    pub fn last(&self) -> Option<BollingerBarOutput> {
        self.last
    }

    /// Push many closes from one payload.
    pub fn push_bars(&mut self, closes: &[f64]) -> FinanceResult<Vec<Option<BollingerBarOutput>>> {
        let mut out = Vec::with_capacity(closes.len());
        for &c in closes {
            out.push(self.push(c)?);
        }
        Ok(out)
    }
}

// ---------------------------------------------------------------------------
// Keltner
// ---------------------------------------------------------------------------

/// Incremental Keltner (EMA mid + Wilder ATR).
#[derive(Clone, Debug)]
pub struct KeltnerState {
    params: KeltnerParams,
    mid: EmaState,
    atr: crate::stocks::ta::atr::AtrState,
    last: Option<KeltnerBarOutput>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KeltnerBarOutput {
    pub middle: f64,
    pub upper: f64,
    pub lower: f64,
    pub atr: f64,
}

impl KeltnerState {
    pub fn new(params: KeltnerParams) -> FinanceResult<Self> {
        let _ = ValidatedKeltner::new(params)?;
        Ok(Self {
            params,
            mid: EmaState::new(params.ema_period)?,
            atr: crate::stocks::ta::atr::AtrState::new(crate::stocks::ta::atr::AtrParams::new(
                params.atr_period,
            ))?,
            last: None,
        })
    }

    pub fn from_history(
        params: KeltnerParams,
        high: &[f64],
        low: &[f64],
        close: &[f64],
    ) -> FinanceResult<Self> {
        let mut s = Self::new(params)?;
        require_hlc(high, low, close)?;
        for i in 0..close.len() {
            s.push(high[i], low[i], close[i])?;
        }
        Ok(s)
    }

    pub fn params(&self) -> KeltnerParams {
        self.params
    }

    pub fn reset(&mut self) {
        self.mid.reset();
        self.atr.reset();
        self.last = None;
    }

    pub fn push(
        &mut self,
        high: f64,
        low: f64,
        close: f64,
    ) -> FinanceResult<Option<KeltnerBarOutput>> {
        let atr_val = self.atr.push(high, low, close)?;
        let mid = self.mid.push(close)?;
        match (mid, atr_val) {
            (Some(m), Some(a)) => {
                let out = KeltnerBarOutput {
                    middle: m,
                    upper: m + self.params.atr_mult * a,
                    lower: m - self.params.atr_mult * a,
                    atr: a,
                };
                self.last = Some(out);
                Ok(Some(out))
            }
            _ => {
                self.last = None;
                Ok(None)
            }
        }
    }

    pub fn last(&self) -> Option<KeltnerBarOutput> {
        self.last
    }

    /// Push aligned H/L/C bars from one payload.
    pub fn push_bars(
        &mut self,
        high: &[f64],
        low: &[f64],
        close: &[f64],
    ) -> FinanceResult<Vec<Option<KeltnerBarOutput>>> {
        require_hlc(high, low, close)?;
        let mut out = Vec::with_capacity(close.len());
        for i in 0..close.len() {
            out.push(self.push(high[i], low[i], close[i])?);
        }
        Ok(out)
    }
}

// ---------------------------------------------------------------------------
// VWAP
// ---------------------------------------------------------------------------

/// Incremental VWAP (cumulative or rolling). Call [`VwapState::reset`] at session open if desired.
///
/// # Examples
/// ```
/// use finance_solution::stocks::ta::{VwapParams, VwapState};
/// let mut v = VwapState::new(VwapParams::cumulative_typical()).unwrap();
/// let a = v.push(10.0, 9.0, 9.5, 100.0).unwrap().unwrap();
/// let b = v.push(11.0, 10.0, 10.5, 200.0).unwrap().unwrap();
/// assert!(b > a || (b - a).abs() < 1.0); // volume-weighted drift
/// v.reset(); // e.g. new regular trading day — caller decides
/// assert!(v.push(10.0, 10.0, 10.0, 50.0).unwrap().unwrap() - 10.0 < 1e-12);
/// ```
#[derive(Clone, Debug)]
pub struct VwapState {
    params: VwapParams,
    // cumulative
    cum_pv: f64,
    cum_v: f64,
    // rolling
    rolling: Option<RingPv>,
    last: Option<f64>,
}

impl VwapState {
    pub fn new(params: VwapParams) -> FinanceResult<Self> {
        let _ = ValidatedVwap::new(params)?;
        let rolling = match params.mode {
            VwapMode::Cumulative => None,
            VwapMode::Rolling { period } => Some(RingPv::with_capacity(period)),
        };
        Ok(Self {
            params,
            cum_pv: 0.0,
            cum_v: 0.0,
            rolling,
            last: None,
        })
    }

    pub fn from_history(
        params: VwapParams,
        high: &[f64],
        low: &[f64],
        close: &[f64],
        volume: &[f64],
    ) -> FinanceResult<Self> {
        let mut s = Self::new(params)?;
        require_hlc(high, low, close)?;
        validate_positive_volume(volume)?;
        if close.len() != volume.len() {
            return Err(FinanceError::LengthMismatch {
                left: close.len(),
                right: volume.len(),
                context: "close/volume",
            });
        }
        for i in 0..close.len() {
            s.push(high[i], low[i], close[i], volume[i])?;
        }
        Ok(s)
    }

    pub fn params(&self) -> VwapParams {
        self.params
    }

    /// Clear cumulative / rolling window (typical: start of session).
    pub fn reset(&mut self) {
        self.cum_pv = 0.0;
        self.cum_v = 0.0;
        if let Some(r) = self.rolling.as_mut() {
            r.clear();
        }
        self.last = None;
    }

    pub fn push(
        &mut self,
        high: f64,
        low: f64,
        close: f64,
        volume: f64,
    ) -> FinanceResult<Option<f64>> {
        require_finite("high", high)?;
        require_finite("low", low)?;
        require_finite("close", close)?;
        require_finite("volume", volume)?;
        if high < low {
            return Err(FinanceError::InvalidCashflow {
                message: "high must be >= low for each bar",
            });
        }
        if volume < 0.0 {
            return Err(FinanceError::InvalidCashflow {
                message: "volume must be non-negative",
            });
        }
        let price = match self.params.price_source {
            VwapPriceSource::Typical => (high + low + close) / 3.0,
            VwapPriceSource::Close => close,
        };
        let out = match self.params.mode {
            VwapMode::Cumulative => {
                self.cum_pv += price * volume;
                self.cum_v += volume;
                if self.cum_v > 0.0 {
                    Some(self.cum_pv / self.cum_v)
                } else {
                    None
                }
            }
            VwapMode::Rolling { period } => {
                let ring = self.rolling.as_mut().unwrap();
                ring.push(price, volume);
                if ring.len() >= period {
                    ring.vwap()
                } else {
                    None
                }
            }
        };
        self.last = out;
        Ok(out)
    }

    pub fn last(&self) -> Option<f64> {
        self.last
    }

    /// Push aligned OHLC+V bars from one payload.
    pub fn push_bars(
        &mut self,
        high: &[f64],
        low: &[f64],
        close: &[f64],
        volume: &[f64],
    ) -> FinanceResult<Vec<Option<f64>>> {
        require_hlc(high, low, close)?;
        validate_positive_volume(volume)?;
        if close.len() != volume.len() {
            return Err(FinanceError::LengthMismatch {
                left: close.len(),
                right: volume.len(),
                context: "close/volume",
            });
        }
        let mut out = Vec::with_capacity(close.len());
        for i in 0..close.len() {
            out.push(self.push(high[i], low[i], close[i], volume[i])?);
        }
        Ok(out)
    }
}

// ---------------------------------------------------------------------------
// RVOL
// ---------------------------------------------------------------------------

/// Incremental relative volume.
#[derive(Clone, Debug)]
pub struct RvolState {
    params: RvolParams,
    ring: RingF64,
    last: Option<f64>,
}

impl RvolState {
    pub fn new(params: RvolParams) -> FinanceResult<Self> {
        let _ = ValidatedRvol::new(params)?;
        Ok(Self {
            params,
            ring: RingF64::with_capacity(params.lookback),
            last: None,
        })
    }

    pub fn from_history(params: RvolParams, volume: &[f64]) -> FinanceResult<Self> {
        let mut s = Self::new(params)?;
        for &v in volume {
            s.push(v)?;
        }
        Ok(s)
    }

    pub fn params(&self) -> RvolParams {
        self.params
    }

    pub fn reset(&mut self) {
        self.ring.clear();
        self.last = None;
    }

    pub fn push(&mut self, volume: f64) -> FinanceResult<Option<f64>> {
        require_finite("volume", volume)?;
        if volume < 0.0 {
            return Err(FinanceError::InvalidCashflow {
                message: "volume must be non-negative",
            });
        }
        self.ring.push(volume);
        if !self.ring.is_full() {
            self.last = None;
            return Ok(None);
        }
        let mean = self.ring.sum() / self.params.lookback as f64;
        let out = if mean > 0.0 {
            Some(volume / mean)
        } else {
            None
        };
        self.last = out;
        Ok(out)
    }

    pub fn last(&self) -> Option<f64> {
        self.last
    }

    /// Push many volume samples from one payload.
    pub fn push_bars(&mut self, volume: &[f64]) -> FinanceResult<Vec<Option<f64>>> {
        let mut out = Vec::with_capacity(volume.len());
        for &v in volume {
            out.push(self.push(v)?);
        }
        Ok(out)
    }
}

// ---------------------------------------------------------------------------
// Parity tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stocks::ta::bollinger::bollinger;
    use crate::stocks::ta::keltner::keltner;
    use crate::stocks::ta::macd::macd;
    use crate::stocks::ta::moving_average::{ema, sma};
    use crate::stocks::ta::rvol::rvol;
    use crate::stocks::ta::stochastic::stochastics;
    use crate::stocks::ta::vwap::vwap;

    fn path(n: usize) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
        let close: Vec<_> = (0..n)
            .map(|i| 100.0 + i as f64 * 0.13 + ((i % 7) as f64) * 0.04)
            .collect();
        let high: Vec<_> = close.iter().map(|c| c + 0.35).collect();
        let low: Vec<_> = close.iter().map(|c| c - 0.35).collect();
        let vol: Vec<_> = (0..n).map(|i| 800.0 + i as f64 * 3.0).collect();
        (high, low, close, vol)
    }

    fn approx_opt(a: Option<f64>, b: Option<f64>) {
        match (a, b) {
            (None, None) => {}
            (Some(x), Some(y)) => assert!((x - y).abs() < 1e-9, "{x} vs {y}"),
            _ => panic!("Option mismatch {a:?} vs {b:?}"),
        }
    }

    #[test]
    fn sma_parity() {
        let (_, _, c, _) = path(40);
        let batch = sma(&c, 10).unwrap();
        let mut st = SmaState::new(10).unwrap();
        for i in 0..c.len() {
            approx_opt(st.push(c[i]).unwrap(), batch[i]);
        }
    }

    #[test]
    fn ema_parity() {
        let (_, _, c, _) = path(40);
        let batch = ema(&c, 10).unwrap();
        let mut st = EmaState::new(10).unwrap();
        for i in 0..c.len() {
            approx_opt(st.push(c[i]).unwrap(), batch[i]);
        }
    }

    #[test]
    fn stoch_parity() {
        let (h, l, c, _) = path(50);
        let p = StochasticParams::full(14, 3, 3);
        let batch = stochastics(&h, &l, &c, p).unwrap();
        let mut st = StochState::new(p).unwrap();
        for i in 0..c.len() {
            let d = st.push_detail(h[i], l[i], c[i]).unwrap();
            approx_opt(d.k, batch.k[i]);
            approx_opt(d.d, batch.d[i]);
        }
    }

    #[test]
    fn macd_parity() {
        let (_, _, c, _) = path(60);
        let p = MacdParams::standard();
        let batch = macd(&c, p).unwrap();
        let mut st = MacdState::new(p).unwrap();
        for i in 0..c.len() {
            let o = st.push(c[i]).unwrap();
            match (o, batch.signal[i], batch.histogram[i], batch.macd[i]) {
                (Some((m, s, h)), Some(bs), Some(bh), Some(bm)) => {
                    assert!((m - bm).abs() < 1e-8, "macd {i}");
                    assert!((s - bs).abs() < 1e-8, "signal {i}");
                    assert!((h - bh).abs() < 1e-8, "hist {i}");
                }
                (None, None, None, _) => {} // warm-up or macd-only in batch
                other => panic!("macd parity at {i}: {other:?}"),
            }
        }
        let bl = batch.last().unwrap();
        let sl = st.last().unwrap();
        assert!((bl.0 - sl.0).abs() < 1e-8);
        assert!((bl.1 - sl.1).abs() < 1e-8);
        assert!((bl.2 - sl.2).abs() < 1e-8);
    }

    #[test]
    fn bollinger_parity() {
        let (_, _, c, _) = path(40);
        let p = BollingerParams::standard();
        let batch = bollinger(&c, p).unwrap();
        let mut st = BollingerState::new(p).unwrap();
        for i in 0..c.len() {
            let o = st.push(c[i]).unwrap();
            match (o, batch.middle[i]) {
                (None, None) => {}
                (Some(bo), Some(m)) => {
                    assert!((bo.middle - m).abs() < 1e-9);
                    assert!((bo.upper - batch.upper[i].unwrap()).abs() < 1e-9);
                    assert!((bo.lower - batch.lower[i].unwrap()).abs() < 1e-9);
                }
                other => panic!("{other:?}"),
            }
        }
    }

    #[test]
    fn keltner_parity() {
        let (h, l, c, _) = path(45);
        let p = KeltnerParams::standard();
        let batch = keltner(&h, &l, &c, p).unwrap();
        let mut st = KeltnerState::new(p).unwrap();
        for i in 0..c.len() {
            let o = st.push(h[i], l[i], c[i]).unwrap();
            match (o, batch.middle[i], batch.upper[i], batch.atr[i]) {
                (Some(ko), Some(m), Some(u), Some(a)) => {
                    assert!((ko.middle - m).abs() < 1e-8, "mid {i}");
                    assert!((ko.upper - u).abs() < 1e-8, "upper {i}");
                    assert!((ko.atr - a).abs() < 1e-8, "atr {i}");
                }
                (None, _, None, _) | (None, None, _, _) => {} // mid and/or atr still warming
                other => panic!("keltner parity {i}: {other:?}"),
            }
        }
        let sl = st.last().unwrap();
        let bl_m = batch.middle.iter().rev().find_map(|x| *x).unwrap();
        let bl_a = batch.atr.iter().rev().find_map(|x| *x).unwrap();
        assert!((bl_m - sl.middle).abs() < 1e-8);
        assert!((bl_a - sl.atr).abs() < 1e-8);
    }

    #[test]
    fn vwap_cum_parity() {
        let (h, l, c, v) = path(30);
        let p = VwapParams::cumulative_typical();
        let batch = vwap(&h, &l, &c, &v, p).unwrap();
        let mut st = VwapState::new(p).unwrap();
        for i in 0..c.len() {
            approx_opt(st.push(h[i], l[i], c[i], v[i]).unwrap(), batch.vwap[i]);
        }
    }

    #[test]
    fn vwap_reset() {
        let mut st = VwapState::new(VwapParams::cumulative_typical()).unwrap();
        st.push(10.0, 10.0, 10.0, 100.0).unwrap();
        st.reset();
        let x = st.push(20.0, 20.0, 20.0, 50.0).unwrap().unwrap();
        assert!((x - 20.0).abs() < 1e-12);
    }

    #[test]
    fn rvol_parity() {
        let (_, _, _, v) = path(40);
        let p = RvolParams::days_20();
        let batch = rvol(&v, p).unwrap();
        let mut st = RvolState::new(p).unwrap();
        for i in 0..v.len() {
            approx_opt(st.push(v[i]).unwrap(), batch.rvol[i]);
        }
    }

    #[test]
    fn rsi_atr_parity_via_from_history() {
        let (h, l, c, _) = path(60);
        let rsi_b = crate::stocks::ta::rsi::rsi(&c, crate::stocks::ta::rsi::RsiParams::period_14())
            .unwrap();
        let rsi_s = crate::stocks::ta::rsi::RsiState::from_history(
            crate::stocks::ta::rsi::RsiParams::period_14(),
            &c,
        )
        .unwrap();
        approx_opt(rsi_b.last(), rsi_s.last());

        let atr_b =
            crate::stocks::ta::atr::atr(&h, &l, &c, crate::stocks::ta::atr::AtrParams::period_14())
                .unwrap();
        let atr_s = crate::stocks::ta::atr::AtrState::from_history(
            crate::stocks::ta::atr::AtrParams::period_14(),
            &h,
            &l,
            &c,
        )
        .unwrap();
        approx_opt(atr_b.last(), atr_s.last());
    }

    #[test]
    fn from_history_matches_push() {
        let (h, l, c, _) = path(25);
        let p = StochasticParams::fast(9, 3);
        let a = StochState::from_history(p, &h, &l, &c).unwrap();
        let mut b = StochState::new(p).unwrap();
        for i in 0..c.len() {
            b.push(h[i], l[i], c[i]).unwrap();
        }
        assert_eq!(a.last_kd(), b.last_kd());
    }
}
