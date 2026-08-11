# Example market data (real bars)

OHLCV fixtures for `cargo run --example …` demos that need a real underlier path
(TA streaming, price-path metrics, optional BSM + spot). **Not** shipped on
crates.io (see package `exclude` in `Cargo.toml`).

## Files

| File | Description |
|------|-------------|
| [`NVDA_FCX_XOM_1m_ohlcv.csv`](NVDA_FCX_XOM_1m_ohlcv.csv) | 1-minute OHLCV bars (~67 MB) |

### Schema

```text
date,time,symbol,open,high,low,close,volume
2024-07-16,09:30:00,NVDA,128.44,128.44,127.57,127.625,2976022.0
```

- **Symbols present:** AAL, FCX, ISRG, NVDA, SPY, XOM (filename is historical; more tickers than the three in the name).
- **Range (file ends):** ~2024-07-16 → ~2026-07-15, regular session minutes.
- **Timezone / venue:** as provided by the source dump; this crate does not interpret calendar rules.

### Run the demo example

```bash
cargo run --example real_bars_ta_bsm --release
REAL_BARS_SYMBOL=SPY cargo run --example real_bars_ta_bsm --release
```

Filters one symbol, seeds TA `*State` with `from_history`, prints a short stoch tail table, and marks an ATM BSM call at the last close using annualized realized vol as σ.

### Load from an example

```rust
use std::path::PathBuf;

fn bars_csv() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("examples/data/NVDA_FCX_XOM_1m_ohlcv.csv")
}
```

Filter by `symbol` in your example (e.g. only `NVDA` closes for SMA/stoch).
The library itself does not parse CSV — examples own the I/O.

### Git / package notes

- Intended for **GitHub** so clones can run real-data examples.
- Excluded from the **crates.io** tarball (size + license caution).
- GitHub soft-warns above 50 MB; if push is rejected or slow, track with
  [Git LFS](https://git-lfs.github.com/) (`*.csv` under this directory).
