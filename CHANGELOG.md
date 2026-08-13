# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned (later)

- BAW / other American approximations
- Optional PDE / Monte Carlo
- Futures linear fair-value / basis helpers
- Wave A desk TA breadth (ADX, OBV, …)

## [0.4.2] — 2026-08-12

Performance and API-coherence release: same public indicator/derivative set as 0.4.1, with **faster streaming TA** and **batch = stream** math paths. **No** dual panicking / `try_*` public math.

### Changed

**TA hot windows (after warm-up)**

- **WMA / HMA:** O(1) sliding weighted sum (`W' = W − Σ + n·p_new`)
- **Bollinger:** O(1) via running `Σ` / `Σx²` (sample or population stdev)
- **LinReg:** O(1) sufficient stats (`Σy`, `Σxy`, `Σy²`; fixed `x = 0..N−1`)
- **Stoch** HH/LL and **Donchian** max/min: **amortized O(1)** monotonic deques
- **`RingF64`:** `push` returns evicted oldest; `oldest()` helper for slides

**Batch = stream**

- Free functions / `Validated*::compute` for Stoch, Bollinger, MACD, RVOL, VWAP (and already state-backed SMA/EMA/WMA/HMA, RSI, ATR, LinReg, Donchian) route through the same `*State` machines as live `push`
- **`MacdState::push_detail` / `push_bars_detail` / `MacdBarOutput`:** per-line warm-up (MACD before signal) for batch parity

### Added

- Criterion **`algo_hot_windows`**: before/after wallclock + parity gate  
  (`cargo bench --bench algo_hot_windows`)
- Criterion **`crr_tree_buffers`**: CRR alloc-per-layer vs ping-pong workspace experiment  
  (measured: no production win; crate CRR unchanged)  
  (`cargo bench --bench crr_tree_buffers`)
- README performance table (desk-scale multi-indicator framing)

### Docs

- `stocks::ta` / `state` / crate root: hot-window costs, batch=stream, multi-indicator caller composition
- Linear regression module: O(1) push documentation

## [0.4.1] — 2026-08-11

### Changed

- crates.io `categories`: `finance`, `mathematics` (valid slugs)
- Criterion harness renamed: `benches/my_benchmark.rs` → `benches/suite.rs` (`cargo bench --bench suite`)

## [0.4.0] — 2026-08-11

Additive derivatives models, TA indicators, solvers, and test depth. **No** dual panicking / `try_*` public math. Still a pure math library.

### Added

**European closed-form lane**

- **Cross Greeks (BSM):** `bsm_cross_greeks` / `BsmCrossGreeks` (vanna, volga, charm)
- **Black ’76** + **Garman–Kohlhagen** full stacks (price/greeks/IV/state/solution)
- Shared IV root-finder; Criterion **suite E** benches
- Examples: `closed_form_options`, `bsm_option`

**Trees / American**

- **CRR binomial:** `CrrParams`, `ExerciseStyle`, `crr_price` / `crr_greeks` / `crr_solution` (node tables for \(N\le 12\))
- **American IV:** `american_implied_vol` / `tree_implied_vol`
- Example: `crr_american`

**TA**

- **RSI** / **ATR** (Wilder) with `*State` and solutions
- **WMA** / **Hull HMA** (`wma` / `WmaState`, `hma` / `HmaState`)
- **Donchian channels** (`donchian` / `DonchianState`)
- **Rolling linear regression** (`linear_regression` / `LinRegState`: slope, angle, \(R^2\)) on any series
- Examples: `ta_rsi_atr`, `real_bars_ta_bsm` (GitHub OHLCV fixture)

**Solvers**

- **`brent_root`** — Brent’s method; used as implied-vol fallback after Newton–Raphson

**Quality**

- TVM identity proptests; expanded TA batch↔stream proptests
- CRR audit: N=1 Δ fix, same-tree Γ, reject illegal RN \(p^*\)
- Keltner uses shared Wilder ATR path

## [0.3.0] — 2026-08-11

Additive **European option math** and a real-bar demo joining underlier TA with BSM. **No** dual panicking / `try_*` public math. Still a pure math library (no feeds, chains, or order routing).

### Added

- **`derivatives` module (D1):**
  - European Black–Scholes–Merton `bsm_price`, `bsm_greeks` (Δ Γ Θ ν ρ), `bsm_terms`, `bsm_implied_vol`
  - Put–call parity residual, `intrinsic` / `time_value` / spot & forward moneyness
  - `BsmParams`, `ValidatedBsm`, `BsmSolution` teaching table, `BsmGreeks` desk helpers (`vega_per_vol_point`, `theta_per_calendar_day`)
  - `BsmState` for live `set_spot` / `set_vol` / `set_vol_from_price` / `set_time_years` updates
  - Deep `//!` docs: trading desk view + engineering wire-up (sync math, join with `stocks::ta`)
- **Examples:**
  - `cargo run --example bsm_option`
  - `cargo run --example real_bars_ta_bsm --release` — real 1m OHLCV under `examples/data/` (GitHub only; excluded from crates.io)
- **TA proptests:** MACD / Keltner last-value batch↔stream parity

### Design notes

- Option + underlier concurrency lives in **your** engine (`HashMap` of `BsmState`, optional `rayon` over strikes).

## [0.2.0] — 2026-08-10

Additive technical-analysis and domain-primitive surface for quant consumers. **No** dual panicking / `try_*` public math.

### Added

- **Domain newtypes:** `Rate`, `Periods`, `PeriodLength`, `PositivePrice`, `Money` (validated `new` → `FinanceResult`)
- **`stocks::ta` batch indicators:** SMA, EMA, Stochastic (fast/full params), MACD, Bollinger, Keltner (Wilder ATR), VWAP (cumulative/rolling + price source enums), RVOL
- **Params + `Validated*` + free functions + `*_solution` tables** for teaching (warm-up as `n/a`; deep `//!` docs with word problems and sample tables)
- **Incremental state machines** (live bars, not a market-data engine):
  - `SmaState`, `EmaState`, `StochState`, `MacdState`, `BollingerState`, `KeltnerState`, `VwapState`, `RvolState`
  - `new` / `from_history` / `push` / `push_bars` / `reset` (where applicable)
  - Batch ↔ stream parity unit tests + **proptest** random-path parity
- SMA/EMA **batch implemented via state** (single math path; `sma_last` / `ema_last` use state)
- Bollinger **`StdevKind`**: Sample (`n−1`, default) or Population (`n`)
- Stochastic **flat window** (HH==LL): carry previous raw %K, else 50
- Criterion **suite D** TA benches; README quant-engine sketch (per-symbol pipeline, caller-owned session reset)
- Example: `cargo run --example ta_indicators`

### Changed

- Fallible constructors use **`new` → `FinanceResult`** (not `try_new`), consistent with Result-only crate policy (`Schedule::new_*`, std `File::open` style)

### Design notes

- Streaming **state** is in scope; streaming **engine** (feeds, multi-symbol registry, calendars) is not — the quant app owns those and calls `reset()` / `from_history` as policy.

## [0.1.0] — 2026-08-10

First crates.io release of the Result-only API.

### Added

- Public math returns `FinanceResult` / `FinanceError` only (no dual panicking / `try_*` surface)
- TVM, cashflow, rate conversion, amortization (solution/series/tables + PPMT/IPMT/CUMPRINC/CUMIPMT)
- Doubling rules, equity price-path metrics (returns, vol, Sharpe, Sortino, beta, drawdowns)
- Domain-oriented docs, examples, Criterion benches for core TVM/cashflow
