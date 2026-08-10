# Benchmark notes (P7 / E3)

Criterion suite: `cargo bench` → `benches/my_benchmark.rs`.

## Suites

| Suite | Functions | Purpose |
|-------|-----------|---------|
| **A** | `future_value`, `present_value`, `payment`, `rate`, `periods` | Core math hot path (Result API) |
| **B** | `future_value_solution`, `payment_solution`, `amortization_solution` + `series` (N=12) | Solution / series overhead |
| **C** | `simple_returns` / `volatility` length 252 | Stocks-style path |

All benches use **valid fixtures** and `.expect("bench fixture")` only inside the harness (not product code).

## Expected outcome

Validation is O(1) field checks. Result wrapping is **noise-level** vs `powf` and series loops. Do not gate CI on absolute criterion numbers for 0.1.0.

## Recorded results (0.1.0 Result API)

Measured locally with Criterion (`cargo bench --bench my_benchmark`), same machine, post-migration Result-only API. Times are **mean point estimates** from Criterion `estimates.json` (`new` run).

| Benchmark | Mean | Notes |
|-----------|------|-------|
| `present_value` | **~27.3 ns** | Suite A hot path |
| `future_value` | **~24.8 ns** | Suite A |
| `payment` | **~31.9 ns** | Suite A |
| `rate` | **~24.9 ns** | Suite A |
| `periods` | **~26.5 ns** | Suite A |
| `future_value_solution` | **~0.89 µs** | Suite B — formula strings + struct |
| `payment_solution` | **~1.54 µs** | Suite B |
| `amortization_solution_series_12` | **~18.7 µs** | Suite B — N=12 schedule build |
| `simple_returns_252` | **~0.65 µs** | Suite C |
| `volatility_252` | **~1.42 µs** | Suite C |

### Pre-migration comparison (v0.0.0 panicking dual API)

Measured from git history worktree (`a2a0beb`-era panicking path), same machine, short Criterion runs:

| Function | v0.0.0 (panic) | v0.1.0 (Result) |
|----------|----------------|-----------------|
| `present_value` | ~26.3 ns | ~27.3 ns |
| `future_value` | ~24.0 ns | ~24.8 ns |
| `payment` | ~32.4 ns | ~31.9 ns |

**Conclusion:** Result + validation cost is within noise of the panicking path for scalar TVM/payment. Wallclock is dominated by **solution/series** construction (string formulas, `Vec` periods), not by `FinanceResult`.

### Cost hierarchy (guidance)

1. **Hot path (~ns):** scalar `future_value` / `present_value` / `payment` / … returning `FinanceResult<f64>` — use in production loops.
2. **Solution path (~µs):** `*_solution` — teaching, debugging, observability, one-off CLI.
3. **Series path (~µs–ms):** amortization / payment series of length N — O(N) allocation + formulas.

## How to re-measure

```text
cargo bench --bench my_benchmark
```

Inspect HTML reports under `target/criterion/*/report/index.html`, or `estimates.json` under `target/criterion/<bench>/new/`.
