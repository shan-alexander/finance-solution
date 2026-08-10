# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned

- Property tests (`proptest`) for TVM/payment/NPV identities (see roadmap E5)

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

[Unreleased]: https://github.com/shan-alexander/finance-solution/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/shan-alexander/finance-solution/releases/tag/v0.1.0
