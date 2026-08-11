//! # Derivatives math (options & related)
//!
//! Pure **pricing, Greeks, and implied volatility** for engines that also run
//! [`crate::stocks::ta`] on the underlier. This module does **not** subscribe to
//! option chains, manage multi-leg books, or know about OSI / exchange symbols.
//!
//! ---
//!
//! ## How a quant uses these metrics (trading perspective)
//!
//! | Metric | Trading question | Desk habit |
//! |--------|------------------|------------|
//! | **Price** | Fair value vs mid / edge? | Compare model to NBBO; mark inventory |
//! | **Δ Delta** | How much underlier exposure per option? | Hedge: sell ≈ Δ shares per long call |
//! | **Γ Gamma** | How fast does the hedge go wrong? | Scalp gamma; size limits into events |
//! | **ν Vega** | What if IV moves a point? | Vol trades, earnings, event premium |
//! | **Θ Theta** | What does the book bleed overnight? | Carry P&amp;L, calendar spreads |
//! | **ρ Rho** | Rate risk? | Usually second-order for short-dated equity |
//! | **IV** | What vol is the market implying? | Surfaces, relative value, skew stories |
//! | **Intrinsic / time value** | How much is “optionality”? | Early exercise intuition (European here) |
//! | **Parity residual** | Is the quote book consistent? | Sanity / arb alert (within fees) |
//!
//! **Typical workflow on a name (e.g. AAPL):**
//!
//! 1. Trade the **underlier path** with TA (`StochState`, `EmaState`, …) on 1m/5s bars.  
//! 2. For each option of interest, maintain **IV from mid** and **Greeks at live spot**.  
//! 3. Risk: sum Δ/Γ/ν over positions; hedge underlier when net Δ exceeds a band.  
//! 4. Research: reprice a chain on a vol surface assumption; compare to TA regime (e.g. high RVOL + high IV).
//!
//! This crate supplies steps 1–3 **math only**. Order routing, position servers, and
//! “should I sell the 0.30Δ call?” stay in *your* strategy code.
//!
//! ---
//!
//! ## How an engineer wires this (engineering perspective)
//!
//! ```text
//! Market data (async / websockets)          finance-solution (sync, pure)
//! ───────────────────────────────          ─────────────────────────────
//! 1m bars for underlier          ──push──► StochState / EmaState / …
//! option quote (bid/ask/mid)     ──IV───► BsmState::set_vol_from_price
//! underlier tick                 ──spot─► for c in chain { c.set_spot(s); greeks() }
//! ```
//!
//! **Recommended shape (mirrors TA):**
//!
//! | Layer | Type | When |
//! |-------|------|------|
//! | Config | [`BsmParams`] (`Copy`) | Contract + market inputs |
//! | Validated | [`ValidatedBsm::new`] | One-shot research / backtest bar |
//! | Live | [`BsmState`] | Per-contract object in `HashMap` |
//! | Teaching | [`bsm_solution`] | Formulas + `print_table` |
//!
//! **Concurrency:** keep math **sync**. Your runtime may:
//!
//! - `rayon::par_iter` over symbols or strikes when recalculating a chain on a spot move  
//! - `tokio` tasks that only **receive** data then call `set_spot` / `push`  
//!
//! Do **not** put `async` inside these functions — there is no I/O to await.
//!
//! **Joining TA + options for one underlier** (your types, illustrative):
//!
//! ```ignore
//! struct UnderlierBook {
//!     ta: StochState,                    // bars
//!     options: HashMap<StrikeKey, BsmState>, // chain
//! }
//! // on_bar  -> ta.push(...); maybe recompute filters
//! // on_spot -> for opt in options.values_mut() { opt.set_spot(s)?; }
//! // on_opt_quote -> opt.set_vol_from_price(mid)?;
//! ```
//!
//! ---
//!
//! ## Models (phased)
//!
//! | Model | Underlier | Status |
//! |-------|-----------|--------|
//! | Black–Scholes–Merton | Spot `S`, continuous yield `q` | **available** |
//! | Black ’76 | Forward / futures `F` | planned |
//! | Garman–Kohlhagen | FX | planned |
//!
//! Equity **single-name** Europeans with continuous yield ≈ BSM.  
//! **Options on futures** / many index products → Black ’76 (later).  
//! **Crypto perps** need funding / mark conventions outside this module.
//!
//! ## Units (read carefully)
//!
//! | Input | Unit |
//! |-------|------|
//! | Spot / strike | same money units |
//! | `time_years` | **years** (`30.0/365.25` for ~30 calendar days) |
//! | `rate`, `dividend_yield` | continuous, absolute (`0.05` = 5%) |
//! | `vol` | annualized absolute (`0.20` = 20%) |
//! | Vega | per **+1.0** in σ (use [`BsmGreeks::vega_per_vol_point`] for per 1%) |
//! | Theta | per **year** (use [`BsmGreeks::theta_per_calendar_day`] for daily) |
//!
//! ## Quick start
//!
//! ```
//! use finance_solution::derivatives::{
//!     OptionType, BsmParams, ValidatedBsm, bsm_price, bsm_greeks, bsm_implied_vol,
//! };
//!
//! let p = BsmParams::atm_one_year(100.0, 0.05, 0.20);
//! let model = ValidatedBsm::new(p).unwrap();
//! let call = model.price(OptionType::Call).unwrap();
//! let g = model.greeks(OptionType::Call).unwrap();
//! assert!(call > 0.0 && g.delta > 0.0 && g.delta < 1.0);
//!
//! // Market mid → IV
//! let iv = bsm_implied_vol(p, OptionType::Call, call).unwrap();
//! assert!((iv - 0.20).abs() < 1e-4);
//!
//! let _ = bsm_price(p, OptionType::Put).unwrap();
//! let _ = bsm_greeks(p, OptionType::Put).unwrap();
//! ```
//!
//! Live underlier ticks: [`BsmState`]. Teaching: [`bsm_solution`].

pub mod black_scholes;
pub mod implied_vol;
pub mod norm;
pub mod state;
pub mod types;

#[doc(inline)]
pub use black_scholes::*;
#[doc(inline)]
pub use implied_vol::*;
#[doc(inline)]
pub use state::*;
#[doc(inline)]
pub use types::*;
