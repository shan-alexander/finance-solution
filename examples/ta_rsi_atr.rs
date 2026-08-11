//! # RSI + ATR demo (Wilder)
//!
//! ```bash
//! cargo run --example ta_rsi_atr
//! ```

use finance_solution::*;

fn main() -> FinanceResult<()> {
    let n = 40usize;
    let close: Vec<f64> = (0..n)
        .map(|i| 100.0 + (i as f64) * 0.3 + ((i % 4) as f64 - 1.5) * 0.2)
        .collect();
    let high: Vec<f64> = close.iter().map(|c| c + 0.5).collect();
    let low: Vec<f64> = close.iter().map(|c| c - 0.5).collect();

    const RSI14: RsiParams = RsiParams::period_14();
    const ATR14: AtrParams = AtrParams::period_14();

    println!("=== Quant pattern: const + Validated* + compute ===\n");
    let rsi_eng = ValidatedRsi::new(RSI14)?;
    let atr_eng = ValidatedAtr::new(ATR14)?;
    let r = rsi_eng.compute(&close)?;
    let a = atr_eng.compute(&high, &low, &close)?;
    println!("RSI(14) last: {:?}", r.last());
    println!("ATR(14) last: {:?}\n", a.last());

    println!("=== RSI solution table (tail) ===\n");
    let start = n.saturating_sub(15);
    let rsi_sol = rsi_solution(&close[start..], RSI14)?;
    println!("{}\n", rsi_sol.formula());
    rsi_sol.print_table();

    println!("\n=== ATR solution table (tail) ===\n");
    let atr_sol = atr_solution(&high[start..], &low[start..], &close[start..], ATR14)?;
    println!("{}\n", atr_sol.formula());
    atr_sol.print_table();

    println!("\n=== Incremental state ===\n");
    let rs = RsiState::from_history(RSI14, &close)?;
    let at = AtrState::from_history(ATR14, &high, &low, &close)?;
    println!("RsiState last: {:?}", rs.last());
    println!("AtrState last: {:?}", at.last());

    Ok(())
}
