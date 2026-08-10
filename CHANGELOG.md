# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned

- Optional TVM/payment identity proptests (E5 remainder)
- Optional RSI / more oscillators

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

First “real” public API contract for this crate: **Result-only** finance math with preserved solution/series/table teaching surface.

### Breaking

- Public financial math is **Result-only**: functions return `FinanceResult<T>` / `FinanceError`. There is no dual panicking / `try_*` API for domain failure.
- `Schedule::new_repeating` and `Schedule::new_custom` return `FinanceResult` (non-finite / empty inputs are errors). OOB `get` returns `Option`.

### Added

- `FinanceError` (`#[non_exhaustive]`) with structured variants, `Display`/`Error`, and stable `code()` strings
- `Compounding` and `PaymentTiming` enums (`From<bool>` for Excel-style flags)
- Criterion benches (suites A/B/C) and measured notes in `benches/RESULTS.md`
- CI: tests, doctests, clippy `-D warnings`, fmt, gate against reintroducing `pub fn try_`

### Changed

- Annuity PV/FV formulas match on `PaymentTiming` (ordinary vs due); docs cover enum and `bool`
- NPV schedule module documents input shapes A–D with examples
- Docs and examples use `?` / `FinanceResult` patterns; teaching depth preserved
- Dependencies: `float-cmp` 0.10, `log` 0.4, `num-format` 0.4, `itertools` 0.15; `criterion` 0.8 (dev)

### Preserved

- Solution / series / pretty-table product surface
- Excel-compatible sign conventions and 1-based amortization period helpers
- MSRV **1.70**, edition **2021**

### Not included (YAGNI)

- **Polars / DataFrame** dependency — no in-crate analysis pipeline that benefits from Polars; tables and `Vec` series are sufficient for 0.1. Apps that already use Polars can map series themselves.

[Unreleased]: https://github.com/shan-alexander/finance-solution/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/shan-alexander/finance-solution/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/shan-alexander/finance-solution/releases/tag/v0.1.0
