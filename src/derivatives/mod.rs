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
//! | **Vanna / Volga / Charm** | Surface &amp; Δ-drift risk | Sticky-strike stories; overnight re-hedge |
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
//! futures mark                   ──F────► Black76State::set_forward
//! FX spot                        ──S────► GkState::set_spot
//! ```
//!
//! **Recommended shape (mirrors TA):**
//!
//! | Layer | Type | When |
//! |-------|------|------|
//! | Config | [`BsmParams`] / [`Black76Params`] / [`GkParams`] (`Copy`) | Contract + market inputs |
//! | Validated | [`ValidatedBsm::new`] / … | One-shot research / backtest bar |
//! | Live | [`BsmState`] / [`Black76State`] / [`GkState`] | Per-contract object in `HashMap` |
//! | Teaching | [`bsm_solution`] / [`black76_solution`] / [`gk_solution`] | Formulas + `print_table` |
//!
//! **Concurrency:** keep math **sync**. Your runtime may `rayon` over strikes or
//! `tokio` only to **receive** data — there is no I/O inside these functions.
//!
//! ---
//!
//! ## Models
//!
//! | Model | Underlier | Status |
//! |-------|-----------|--------|
//! | Black–Scholes–Merton | Spot `S`, continuous yield `q` | **available** (+ cross Greeks) |
//! | Black ’76 | Forward / futures `F` | **available** |
//! | Garman–Kohlhagen | FX spot, \(r_d\), \(r_f\) | **available** |
//! | CRR binomial | European / American tree | **available** |
//!
//! Equity **single-name** Europeans with continuous yield ≈ BSM.  
//! **Options on futures** / many index products → Black ’76.  
//! **FX vanillas** → Garman–Kohlhagen.  
//! **American** early exercise / American IV → CRR ([`crr_price`], [`american_implied_vol`]).  
//! **Crypto perps** need funding / mark conventions outside this module.
//!
//! ## Units (read carefully)
//!
//! | Input | Unit |
//! |-------|------|
//! | Spot / forward / strike | same money units |
//! | `time_years` | **years** (`30.0/365.25` for ~30 calendar days) |
//! | rates | continuous, absolute (`0.05` = 5%) |
//! | `vol` | annualized absolute (`0.20` = 20%) |
//! | Vega | per **+1.0** in σ (use `vega_per_vol_point` for per 1%) |
//! | Theta / charm | per **year** (use `*_per_calendar_day` helpers) |
//!
//! ## Quick start
//!
//! ```
//! use finance_solution::derivatives::{
//!     OptionType, BsmParams, ValidatedBsm, bsm_price, bsm_greeks, bsm_cross_greeks,
//!     bsm_implied_vol, Black76Params, black76_price, GkParams, gk_price,
//! };
//!
//! let p = BsmParams::atm_one_year(100.0, 0.05, 0.20);
//! let model = ValidatedBsm::new(p).unwrap();
//! let call = model.price(OptionType::Call).unwrap();
//! let g = model.greeks(OptionType::Call).unwrap();
//! let x = bsm_cross_greeks(p, OptionType::Call).unwrap();
//! assert!(call > 0.0 && g.delta > 0.0 && x.volga.is_finite());
//!
//! let iv = bsm_implied_vol(p, OptionType::Call, call).unwrap();
//! assert!((iv - 0.20).abs() < 1e-4);
//!
//! // Futures-style
//! let f = Black76Params::atm_one_year(100.0, 0.05, 0.20);
//! let _ = black76_price(f, OptionType::Call).unwrap();
//!
//! // FX-style
//! let fx = GkParams::atm_one_year(1.10, 0.05, 0.03, 0.12);
//! let _ = gk_price(fx, OptionType::Call).unwrap();
//! ```
//!
//! Live underlier ticks: [`BsmState`] / [`Black76State`] / [`GkState`].  
//! Teaching: [`bsm_solution`], [`black76_solution`], [`gk_solution`], [`crr_solution`].  
//! American: [`crr_price`] + [`american_implied_vol`].

pub mod black76;
pub mod black_scholes;
pub mod crr;
pub mod garman_kohlhagen;
pub mod implied_vol;
pub mod norm;
pub mod state;
pub mod types;

#[doc(inline)]
pub use black76::*;
#[doc(inline)]
pub use black_scholes::*;
#[doc(inline)]
pub use crr::*;
#[doc(inline)]
pub use garman_kohlhagen::*;
#[doc(inline)]
pub use implied_vol::*;
#[doc(inline)]
pub use state::*;
#[doc(inline)]
pub use types::*;
