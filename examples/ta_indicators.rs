//! # Technical analysis demo (quant pattern + teaching solutions)
//!
//! Shows the **recommended production pattern**:
//!
//! 1. `const` parameter packs (FastStoch(9,3), MACD(12,26,9), …)
//! 2. `Validated*::new` once
//! 3. `.compute` on bar batches
//!
//! …plus `*_solution` tables for teaching (warm-up printed as `n/a`).
//!
//! ```bash
//! cargo run --example ta_indicators
//! ```
//!
//! ## Sample output (truncated; numbers depend on synthetic fixture)
//!
//! ```text
//! === Quant pattern: const packs + Validated* + .compute ===
//!
//! FAST_9_3 pack: StochasticParams { k_period: 9, k_smooth: 1, d_period: 3 }
//! last %K/%D (if defined): Some((…, …))
//!
//! === Stochastic solution table (tail) ===
//! period   close      k      d
//! ------  ------  -----  -----
//!     …     …      n/a    n/a
//!     …     …    72.00    n/a
//!     …     …    68.00  70.00
//!
//! === MACD solution (formula) ===
//! macd = EMA(12) - EMA(26); signal = EMA(9)(macd); hist = macd - signal
//!
//! === Bollinger / Keltner / VWAP / RVOL ===
//! (print_table rows with n/a warm-up, then defined bands / vwap / rvol)
//! ```

use finance_solution::*;

fn main() -> FinanceResult<()> {
    // Synthetic rising path with mild noise — enough length for MACD(26) warm-up.
    let n = 50usize;
    let close: Vec<f64> = (0..n)
        .map(|i| 100.0 + i as f64 * 0.15 + ((i % 5) as f64) * 0.05)
        .collect();
    let high: Vec<f64> = close.iter().map(|c| c + 0.4).collect();
    let low: Vec<f64> = close.iter().map(|c| c - 0.4).collect();
    let volume: Vec<f64> = (0..n)
        .map(|i| 1_000.0 + (i as f64) * 5.0 + if i == n - 1 { 2_000.0 } else { 0.0 })
        .collect();

    // -------------------------------------------------------------------------
    // 1) Strategy definition — const packs (can live at module scope in real apps)
    // -------------------------------------------------------------------------
    const FAST_9_3: StochasticParams = StochasticParams::fast(9, 3);
    const FULL_14_3_3: StochasticParams = StochasticParams::full(14, 3, 3);
    const MACD_STD: MacdParams = MacdParams::standard();
    const BB_20_2: BollingerParams = BollingerParams::standard();
    const KC_STD: KeltnerParams = KeltnerParams::standard();
    const VWAP_CUM: VwapParams = VwapParams::cumulative_typical();
    const RVOL_20: RvolParams = RvolParams::days_20();

    println!("=== Quant pattern: const packs + Validated* + .compute ===\n");
    println!("FAST_9_3 pack: {FAST_9_3:?}");
    println!("FULL_14_3_3 pack: {FULL_14_3_3:?}");
    println!("MACD_STD: {MACD_STD:?}");
    println!("BB_20_2: {BB_20_2:?}\n");

    // -------------------------------------------------------------------------
    // 2) Validate once
    // -------------------------------------------------------------------------
    let stoch = ValidatedStochastic::new(FAST_9_3)?;
    let stoch_full = ValidatedStochastic::new(FULL_14_3_3)?;
    let macd_eng = ValidatedMacd::new(MACD_STD)?;
    let bb = ValidatedBollinger::new(BB_20_2)?;
    let kc = ValidatedKeltner::new(KC_STD)?;
    let vwap_eng = ValidatedVwap::new(VWAP_CUM)?;
    let rvol_eng = ValidatedRvol::new(RVOL_20)?;

    // -------------------------------------------------------------------------
    // 3) Hot path — batch compute (production would loop symbols here)
    // -------------------------------------------------------------------------
    let kd = stoch.compute(&high, &low, &close)?;
    let kd_full = stoch_full.compute(&high, &low, &close)?;
    let m = macd_eng.compute(&close)?;
    let bands = bb.compute(&close)?;
    let channels = kc.compute(&high, &low, &close)?;
    let vw = vwap_eng.compute(&high, &low, &close, &volume)?;
    let rv = rvol_eng.compute(&volume)?;

    println!("last Fast(9,3) %K/%D: {:?}", kd.last_kd());
    println!("last Full(14,3,3) %K/%D: {:?}", kd_full.last_kd());
    println!("last MACD (m,s,h): {:?}", m.last());
    println!(
        "last BB mid/upper: {:?} / {:?}",
        bands.middle.iter().rev().find_map(|x| *x),
        bands.upper.iter().rev().find_map(|x| *x)
    );
    println!(
        "last Keltner mid: {:?}",
        channels.middle.iter().rev().find_map(|x| *x)
    );
    println!("last VWAP: {:?}", vw.vwap.iter().rev().find_map(|x| *x));
    println!("last RVOL: {:?}\n", rv.last());

    // -------------------------------------------------------------------------
    // Teaching path — solutions + tables (observability, not hot path)
    // -------------------------------------------------------------------------
    println!("=== Stochastic solution (Fast 9,3) — print_table ===\n");
    let stoch_sol = stochastics_solution(&high, &low, &close, FAST_9_3)?;
    println!("formula: {}", stoch_sol.formula());
    println!("symbolic: {}\n", stoch_sol.symbolic_formula());
    stoch_sol.print_table();

    println!("\n=== MACD solution ===\n");
    let macd_sol = macd_solution(&close, MACD_STD)?;
    println!("formula: {}", macd_sol.formula());
    macd_sol.print_table();

    println!("\n=== Bollinger solution ===\n");
    let bb_sol = bollinger_solution(&close, BB_20_2)?;
    println!("formula: {}", bb_sol.formula());
    bb_sol.print_table();

    println!("\n=== Keltner solution ===\n");
    let kc_sol = keltner_solution(&high, &low, &close, KC_STD)?;
    println!("formula: {}", kc_sol.formula());
    kc_sol.print_table();

    println!("\n=== VWAP solution ===\n");
    let vwap_sol = vwap_solution(&high, &low, &close, &volume, VWAP_CUM)?;
    println!("formula: {}", vwap_sol.formula());
    vwap_sol.print_table();

    println!("\n=== RVOL solution ===\n");
    let rvol_sol = rvol_solution(&volume, RVOL_20)?;
    println!("formula: {}", rvol_sol.formula());
    rvol_sol.print_table();

    // -------------------------------------------------------------------------
    // Incremental state — same packs, live-style push (parity with last batch values)
    // -------------------------------------------------------------------------
    println!("\n=== Incremental state (push bar-by-bar) ===\n");
    let mut stoch_live = StochState::new(FAST_9_3)?;
    let mut macd_live = MacdState::new(MACD_STD)?;
    let mut vwap_live = VwapState::new(VWAP_CUM)?;
    for i in 0..n {
        let _ = stoch_live.push(high[i], low[i], close[i])?;
        let _ = macd_live.push(close[i])?;
        let _ = vwap_live.push(high[i], low[i], close[i], volume[i])?;
    }
    println!("live stoch last_kd: {:?}", stoch_live.last_kd());
    println!("live macd last:     {:?}", macd_live.last());
    println!("live vwap last:     {:?}", vwap_live.last());
    println!("(compare to batch last values above — should match within float noise)");

    // Caller-owned session reset demo
    vwap_live.reset();
    let first_of_day = vwap_live.push(high[n - 1], low[n - 1], close[n - 1], volume[n - 1])?;
    println!("after vwap.reset() + one bar: {first_of_day:?}");

    Ok(())
}
