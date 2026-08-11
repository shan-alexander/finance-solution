//! # Real 1m bars: underlier TA + path metrics + BSM snapshot
//!
//! Loads the GitHub fixture
//! [`examples/data/NVDA_FCX_XOM_1m_ohlcv.csv`](../data/NVDA_FCX_XOM_1m_ohlcv.csv)
//! (multi-symbol 1-minute OHLCV), filters **one** symbol, then runs the same
//! quant pattern as production math would:
//!
//! 1. **Path metrics** on closes (total return, max drawdown, realized vol)
//! 2. **Incremental TA state** seeded with `from_history` (EMA / stoch / MACD / RVOL)
//! 3. **Teaching table** on a short **tail** of bars only (never print 200k rows)
//! 4. **BSM** ATM European call at the last close, using annualized realized vol as σ
//!
//! CSV parsing lives in **this example** — the library stays pure math.
//!
//! ```bash
//! # Prefer release: ~200k NVDA bars after filter
//! cargo run --example real_bars_ta_bsm --release
//!
//! # Another symbol in the same file
//! REAL_BARS_SYMBOL=SPY cargo run --example real_bars_ta_bsm --release
//! ```
//!
//! ## Sample output (abbreviated; numbers depend on the fixture)
//!
//! ```text
//! === Load ===
//! symbol=NVDA  bars=…  first=…  last=…
//! close first→last: … → …
//!
//! === Path metrics (1m closes) ===
//! total_return / max_drawdown / realized vol (ann.) …
//!
//! === TA state (from_history) ===
//! ema20 / stoch %K%D / macd / rvol …
//!
//! === Stoch solution table (last 20 bars) ===
//! …
//!
//! === BSM ATM call at last close ===
//! price, delta, vega/pt, theta/day …
//! ```

use finance_solution::*;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

/// Regular-session 1m bars/year approximation (252 × 6.5h × 60).
const MINUTES_PER_YEAR: f64 = 252.0 * 6.5 * 60.0;

/// Bars printed in the stoch teaching table (tail only).
const TAIL_BARS: usize = 20;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let symbol = env::var("REAL_BARS_SYMBOL").unwrap_or_else(|_| "NVDA".to_string());
    let path = bars_csv_path();

    println!("=== Load ===\n");
    println!("file:   {}", path.display());
    println!("symbol: {symbol}  (override with REAL_BARS_SYMBOL)\n");

    let bars = load_symbol_bars(&path, &symbol)?;
    if bars.is_empty() {
        return Err(format!("no rows for symbol {symbol:?} in {}", path.display()).into());
    }

    let n = bars.len();
    let first = &bars[0];
    let last = &bars[n - 1];
    println!(
        "bars={n}  first={} {}  last={} {}",
        first.date, first.time, last.date, last.time
    );
    println!("close first→last: {:.4} → {:.4}\n", first.close, last.close);

    let high: Vec<f64> = bars.iter().map(|b| b.high).collect();
    let low: Vec<f64> = bars.iter().map(|b| b.low).collect();
    let close: Vec<f64> = bars.iter().map(|b| b.close).collect();
    let volume: Vec<f64> = bars.iter().map(|b| b.volume).collect();

    // -------------------------------------------------------------------------
    // Path metrics (scalars — do not print 200k-row series tables)
    // -------------------------------------------------------------------------
    println!("=== Path metrics (1m closes) ===\n");
    let years = n as f64 / MINUTES_PER_YEAR;
    let tot = total_return(close[0], close[n - 1])?;
    let mdd = max_drawdown(&close)?;
    let rets = simple_returns(&close)?;
    let vol_1m = volatility(&rets)?;
    let vol_ann = volatility_annualized(&rets, MINUTES_PER_YEAR)?;
    let cagr_v = cagr(close[0], close[n - 1], years)?;

    println!("approx years (RTH 1m): {years:.3}");
    println!("total_return:          {tot:>10.4}  ({:.2}%)", tot * 100.0);
    println!(
        "CAGR:                  {cagr_v:>10.4}  ({:.2}%)",
        cagr_v * 100.0
    );
    println!("max_drawdown:          {mdd:>10.4}  ({:.2}%)", mdd * 100.0);
    println!("vol per 1m bar (std):  {vol_1m:>10.6}");
    println!(
        "vol annualized:        {vol_ann:>10.4}  ({:.1}%  — used as BSM σ below)\n",
        vol_ann * 100.0
    );

    // -------------------------------------------------------------------------
    // Live-style TA: seed from full history once (engine reconnect pattern)
    // -------------------------------------------------------------------------
    println!("=== TA state (from_history on full series) ===\n");

    const FAST_9_3: StochasticParams = StochasticParams::fast(9, 3);
    const MACD_STD: MacdParams = MacdParams::standard();
    const RVOL_20: RvolParams = RvolParams::days_20();

    let ema20 = EmaState::from_history(20, &close)?;
    let stoch = StochState::from_history(FAST_9_3, &high, &low, &close)?;
    let macd = MacdState::from_history(MACD_STD, &close)?;
    let rvol = RvolState::from_history(RVOL_20, &volume)?;

    println!("EMA(20) last:     {:?}", ema20.last());
    println!("Stoch Fast(9,3):  {:?}", stoch.last_kd());
    println!("MACD(12,26,9):    {:?}", macd.last());
    println!("RVOL(20) last:    {:?}", rvol.last());
    println!(
        "note: RvolParams::days_20 is lookback=20 **bars of this series** \
         (here 20 one-minute bars, not 20 sessions)."
    );
    println!(
        "\n(Engineering: keep these *State objects per symbol; on each new bar call push / push_bars.)\n"
    );

    // -------------------------------------------------------------------------
    // Teaching: solution table on a short tail only
    // -------------------------------------------------------------------------
    let tail_n = TAIL_BARS.min(n);
    let start = n - tail_n;
    println!("=== Stoch solution table (last {tail_n} bars only) ===\n");
    let stoch_sol = stochastics_solution(&high[start..], &low[start..], &close[start..], FAST_9_3)?;
    println!("formula: {}\n", stoch_sol.formula());
    stoch_sol.print_table();

    // -------------------------------------------------------------------------
    // BSM at last close: ATM, short-dated, σ = realized annualized
    // -------------------------------------------------------------------------
    println!("\n=== BSM ATM European call at last close ===\n");
    let spot = last.close;
    let sigma = vol_ann.max(1e-6); // floor pathological zero-vol paths
    let p = BsmParams {
        spot,
        strike: spot, // ATM on last print
        time_years: 21.0 / 365.25,
        rate: 0.05,
        dividend_yield: 0.0,
        vol: sigma,
    };
    println!("inputs: S=K={spot:.4}  T=21/365.25d  r=5%  q=0  σ={sigma:.4} (realized ann.)\n");

    let sol = bsm_solution(p, OptionType::Call)?;
    let g = sol.greeks;
    println!("model price:     {:>10.4}", sol.price);
    println!("intrinsic:       {:>10.4}", sol.intrinsic);
    println!("time value:      {:>10.4}", sol.time_value);
    println!(
        "delta:           {:>10.4}  // hedge ≈ short this many shares / long call",
        g.delta
    );
    println!(
        "vega / vol-pt:   {:>10.4}  // if IV moves 1 percentage point",
        g.vega_per_vol_point()
    );
    println!(
        "theta / day:     {:>10.4}  // calendar-day carry (365.25)",
        g.theta_per_calendar_day()
    );
    println!();
    sol.print_table();

    // Live join sketch: same spot into BsmState
    let mut opt = BsmState::new(p, OptionType::Call)?;
    let prior = spot * 0.99;
    opt.set_spot(prior)?;
    let d_down = opt.greeks()?.delta;
    opt.set_spot(spot)?;
    let d_now = opt.greeks()?.delta;
    println!("\nBsmState spot stress: S={prior:.4} Δ={d_down:.4}  →  S={spot:.4} Δ={d_now:.4}");
    println!(
        "\nJoin story: StochState/EmaState on 1m bars + BsmState on the option chain; \
         both are sync math — your engine owns the HashMaps and the tape."
    );

    Ok(())
}

// -----------------------------------------------------------------------------
// Minimal CSV I/O (example-local; not part of the library)
// -----------------------------------------------------------------------------

struct Bar {
    date: String,
    time: String,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
}

fn bars_csv_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/data/NVDA_FCX_XOM_1m_ohlcv.csv")
}

fn load_symbol_bars(
    path: &std::path::Path,
    symbol: &str,
) -> Result<Vec<Bar>, Box<dyn std::error::Error>> {
    let file = File::open(path).map_err(|e| {
        format!(
            "open {}: {e} (clone the repo with examples/data/ or restore the fixture)",
            path.display()
        )
    })?;
    let reader = BufReader::with_capacity(1 << 20, file);
    let mut out = Vec::with_capacity(200_000);
    let mut lines = reader.lines();
    let header = lines.next().ok_or("empty CSV")??.to_ascii_lowercase();
    if !header.contains("symbol") || !header.contains("close") {
        return Err(format!("unexpected CSV header: {header}").into());
    }

    for (i, line) in lines.enumerate() {
        let line = line?;
        if line.is_empty() {
            continue;
        }
        // date,time,symbol,open,high,low,close,volume
        let mut parts = line.split(',');
        let date = parts.next().unwrap_or("");
        let time = parts.next().unwrap_or("");
        let sym = parts.next().unwrap_or("");
        if sym != symbol {
            continue;
        }
        let _open = parts.next();
        let high = parse_f64(parts.next(), i, "high")?;
        let low = parse_f64(parts.next(), i, "low")?;
        let close = parse_f64(parts.next(), i, "close")?;
        let volume = parse_f64(parts.next(), i, "volume")?;
        out.push(Bar {
            date: date.to_string(),
            time: time.to_string(),
            high,
            low,
            close,
            volume,
        });
    }
    Ok(out)
}

fn parse_f64(
    field: Option<&str>,
    row: usize,
    name: &str,
) -> Result<f64, Box<dyn std::error::Error>> {
    let s = field.ok_or_else(|| format!("row {row}: missing {name}"))?;
    s.trim()
        .parse::<f64>()
        .map_err(|e| format!("row {row}: bad {name} {s:?}: {e}").into())
}
