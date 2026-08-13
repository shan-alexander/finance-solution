//! Technical analysis indicators on price / volume series.
//!
//! **Scope of this module:** pure **batch** building blocks a quant *engine* or notebook
//! consumes. This crate does **not** run an event loop, subscribe to market data, or own
//! portfolio state. See the crate README (“Quant pattern” and “Why not a streaming engine?”).
//!
//! # Layers (performance)
//!
//! | Layer | API | Cost class | Use |
//! |-------|-----|------------|-----|
//! | **Config** | `*Params` / `Validated*` (`Copy`) | O(1) validate once | Build at startup / `const` |
//! | **Hot path (batch)** | `sma`, `ema`, `stochastics`, `macd`, … | O(n) pure math, no `String` | Research, backtests |
//! | **Hot path (live)** | `SmaState` / `StochState` / … `push` / `push_bars` | O(1) / amortized O(1) per bar | Streaming payloads |
//! | **Solution** | `*_solution` | O(n) + formulas + tables | Teaching, audit, observability |
//!
//! # Quant ergonomics — recommended pattern (`const` + validate + `.compute`)
//!
//! Production code should **not** invent a new parameter list on every bar. Define the
//! indicator variation once, validate once, reuse forever:
//!
//! ```
//! use finance_solution::stocks::ta::{
//!     StochasticParams, ValidatedStochastic,
//!     MacdParams, ValidatedMacd,
//!     BollingerParams, ValidatedBollinger,
//! };
//!
//! // --- Strategy knobs (module-level const packs) ---
//! const FAST_STOCH_9_3: StochasticParams = StochasticParams::fast(9, 3);
//! const MACD_12_26_9: MacdParams = MacdParams::standard();
//! const BB_20_2: BollingerParams = BollingerParams::standard();
//!
//! // --- Startup: O(1) validation ---
//! let stoch = ValidatedStochastic::new(FAST_STOCH_9_3).unwrap();
//! let macd_eng = ValidatedMacd::new(MACD_12_26_9).unwrap();
//! let bb = ValidatedBollinger::new(BB_20_2).unwrap();
//!
//! // --- Hot path: many symbols / many days ---
//! # let high = [11.0_f64, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0, 20.0];
//! # let low  = [10.0, 10.5, 11.0, 11.5, 12.0, 12.5, 13.0, 13.5, 14.0, 14.5];
//! # let close= [10.5, 11.5, 12.5, 13.5, 14.5, 15.5, 16.5, 17.5, 18.5, 19.5];
//! # let closes: Vec<f64> = (1..=40).map(|x| 100.0 + x as f64 * 0.1).collect();
//! let kd = stoch.compute(&high, &low, &close).unwrap();
//! let m = macd_eng.compute(&closes).unwrap();
//! let bands = bb.compute(&closes).unwrap();
//! assert_eq!(kd.k.len(), high.len());
//! assert_eq!(m.macd.len(), closes.len());
//! assert_eq!(bands.middle.len(), closes.len());
//! ```
//!
//! **Why this shape?**
//!
//! 1. **Clarity** — `FAST_STOCH_9_3` documents the strategy; no magic positional args.
//! 2. **Safety** — period=0 fails at `new`, not mid-batch.
//! 3. **Speed** — validation is noise vs O(n) windows (see Criterion suite D).
//! 4. **Variations** — Fast(9,3), Full(14,3,3), MACD(8,17,9) are just different `const` packs
//!    on the **same** functions — no combinatorial API explosion.
//!
//! Free functions (`stochastics(...)`, `macd(...)`, …) remain for scripts and doctests.
//!
//! # Warm-up policy
//!
//! Output length equals input length. Bars before a window is full are [`None`].
//! Solution tables print warm-up as `n/a`.
//!
//! # Indicators
//!
//! | Indicator | Params / presets | Series | Solution + table |
//! |-----------|------------------|--------|------------------|
//! | SMA / EMA / WMA / HMA | `period` | [`sma`], [`ema`], [`wma`], [`hma`] (+ `*State`) | — |
//! | Stochastic | [`StochasticParams::fast`] / [`full`](StochasticParams::full) | [`stochastics`] | [`stochastics_solution`] |
//! | MACD | [`MacdParams::standard`] (12,26,9) | [`macd`] | [`macd_solution`] |
//! | Bollinger | [`BollingerParams::standard`] (20,2, sample stdev) | [`bollinger`] | [`bollinger_solution`] |
//! | Keltner | [`KeltnerParams::standard`] (20,10,2) | [`keltner`] | [`keltner_solution`] |
//! | Donchian | [`DonchianParams::period_20`] | [`donchian`] | [`donchian_solution`] |
//! | VWAP | [`VwapParams::cumulative_typical`] | [`vwap`] | [`vwap_solution`] |
//! | RVOL | [`RvolParams::days_20`] | [`rvol`] | [`rvol_solution`] |
//! | RSI | [`RsiParams::period_14`] | [`rsi`] | [`rsi_solution`] |
//! | ATR | [`AtrParams::period_14`] | [`atr`] | [`atr_solution`] |
//! | LinReg | [`LinRegParams::period_20`] | [`linear_regression`] | [`linear_regression_solution`] |
//! | Williams %R | [`WillrParams::period_14`] | [`willr`] | [`willr_solution`] |
//! | OBV | [`ObvParams::default_pack`] | [`obv`] | [`obv_solution`] |
//! | CCI | [`CciParams::period_20`] | [`cci`] | [`cci_solution`] |
//! | ADX / DI / DX | [`AdxParams::period_14`] | [`adx`] | [`adx_solution`] |
//!
//! # Incremental / streaming state (live bars)
//!
//! For tick/5s/1m **payloads**, use `*State` types: `push` one bar at a time, `push_bars` for
//! multi-bar messages, or `from_history` then only push live updates. See [`state`] module docs
//! and the README quant-engine sketch. **You** call `reset()` on VWAP when your calendar says so.
//!
//! **Every public TA indicator has a matching `*State`**. Running *all* of them on one symbol
//! each 5s bar is supported: hold one struct of states per symbol and call `push` sequentially.
//! That is **caller-owned composition** (and multi-symbol parallelism is caller-side `rayon`).
//! The crate does **not** ship a multi-indicator “run everything” engine.
//!
//! Hot-window slides (after warm-up): SMA/EMA/WMA/HMA/BB/LinReg/RSI/ATR/OBV/ADX/… are O(1);
//! Stoch/WillR HH/LL and Donchian max/min are **amortized O(1)** via monotonic deques.
//!
//! Conventions worth knowing:
//!
//! - **Stochastic flat window** (HH == LL): carry previous raw %K, else 50.
//! - **Williams %R flat window**: carry previous %R, else −50.
//! - **Bollinger stdev**: [`StdevKind::Sample`] (`n−1`) default; optional population (`n`).
//! - **Batch = stream path:** free functions / `Validated*::compute` route through the same
//!   `*State` machines as live `push` (parity by construction).

pub mod adx;
pub mod atr;
pub mod bollinger;
pub mod cci;
pub mod common;
pub mod donchian;
pub mod keltner;
pub mod linear_regression;
pub mod macd;
pub mod moving_average;
pub mod obv;
#[cfg(test)]
mod proptests;
pub mod ring;
pub mod rsi;
pub mod rvol;
pub mod state;
pub mod stochastic;
pub mod vwap;
pub mod willr;

#[doc(inline)]
pub use adx::*;
#[doc(inline)]
pub use atr::*;
#[doc(inline)]
pub use bollinger::*;
#[doc(inline)]
pub use cci::*;
#[doc(inline)]
pub use common::StdevKind;
#[doc(inline)]
pub use donchian::*;
#[doc(inline)]
pub use keltner::*;
#[doc(inline)]
pub use linear_regression::*;
#[doc(inline)]
pub use macd::*;
#[doc(inline)]
pub use moving_average::*;
#[doc(inline)]
pub use obv::*;
#[doc(inline)]
pub use rsi::*;
#[doc(inline)]
pub use rvol::*;
#[doc(inline)]
pub use state::*;
#[doc(inline)]
pub use stochastic::*;
#[doc(inline)]
pub use vwap::*;
#[doc(inline)]
pub use willr::*;
