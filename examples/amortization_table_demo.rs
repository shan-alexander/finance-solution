//! Demo: amortization solution → series → print_table (12-month sample loan).
//!
//! ```bash
//! cargo run --example amortization_table_demo
//! ```

use finance_solution::*;

fn main() -> FinanceResult<()> {
    // Fixture: $10k, 8% APR monthly, 12 months.
    let rate = 0.08 / 12.0;
    let periods = 12u32;
    let principal = 10_000.0;

    let solution = amortization_solution(rate, periods, principal, 0.0, false)?;

    println!("=== formulas ===");
    println!("{}", solution.formula());
    println!("{}", solution.symbolic_formula());
    println!("payment: {:.4}", solution.payment());
    println!("sum of interest: {:.4}", solution.sum_of_interest());

    println!("\n=== series.print_table(true, true) ===");
    solution.series().print_table(true, true);

    println!("\n=== filter: period % 6 == 0 ===");
    solution
        .series()
        .filter(|r| r.period() % 6 == 0)
        .print_table(true, false);

    println!("\n=== print_table_locale (en, 2 decimals) ===");
    solution
        .series()
        .print_table_locale(true, true, &num_format::Locale::en, 2);

    Ok(())
}
