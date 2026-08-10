//! Price-path solution: summary metrics + period series table.
//!
//! ```bash
//! cargo run --example price_path_analysis
//! ```

use finance_solution::*;

fn main() -> FinanceResult<()> {
    // Strictly positive price fixture (monthly-ish path).
    let prices = [
        100.0, 102.0, 101.0, 108.0, 105.0, 112.0, 110.0, 118.0, 115.0, 125.0,
    ];
    let opts = PricePathOptions::new(12.0)
        .with_years(prices.len() as f64 / 12.0)
        .with_risk_free(0.0);

    let path = price_path_solution(&prices, opts)?;
    println!("=== formulas ===");
    println!("{}", path.formula());
    println!("{}\n", path.symbolic_formula());

    println!("=== print_summary ===");
    path.print_summary();

    println!("\n=== series.print_table ===");
    path.series().print_table();

    println!("\n=== scalar cross-checks ===");
    println!("max_drawdown = {:.6}", path.max_drawdown());
    println!("max_drawdown (scalar) = {:.6}", max_drawdown(&prices)?);
    let rets = simple_returns(&prices)?;
    println!("n returns = {}", rets.len());
    println!("vol = {:.6}", volatility(&rets)?);
    Ok(())
}
