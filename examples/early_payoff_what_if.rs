//! What-if: pay off an amortizing loan early (principal-only lump sum).
//!
//! ```bash
//! cargo run --example early_payoff_what_if
//! ```

use finance_solution::*;

fn main() -> FinanceResult<()> {
    // --- Affordability-linked 5-year loan ---
    let principal = present_value(0.05, 10, 50_000.0, false)?.abs();
    let loan = amortization_solution(0.06 / 12.0, 60, principal, 0.0, false)?;

    println!("=== 5-year 6% loan (principal ≈ ${principal:.2}) ===");
    println!("monthly payment:     {:>12.2}", loan.payment());
    println!("total interest:      {:>12.2}", loan.sum_of_interest());
    println!();

    for (label, months_paid) in [("3 years early", 24u32), ("1 year early", 48)] {
        let row = &loan.series()[(months_paid - 1) as usize];
        println!("{label} (after {months_paid} payments):");
        println!("  lump-sum principal due: {:>12.2}", row.balance());
        println!("  interest already paid:  {:>12.2}", row.interest_to_date());
        println!(
            "  interest saved if paid off now: {:>12.2}",
            row.interest_remaining()
        );
        println!(
            "  share of lifetime interest already paid: {:>5.1}%",
            100.0 * row.interest_to_date().abs() / loan.sum_of_interest().abs()
        );
        println!();
    }

    // --- Classic 30-year mortgage ---
    let mortgage = amortization_solution(0.065 / 12.0, 360, 300_000.0, 0.0, false)?;
    let total_int = mortgage.sum_of_interest();

    println!("=== $300k mortgage, 6.5% APR, 30 years ===");
    println!("monthly payment:  {:>12.2}", mortgage.payment());
    println!("total interest:   {:>12.2}", total_int);
    let first_15 = mortgage.cumipmt(1, 180)?;
    let last_15 = mortgage.cumipmt(181, 360)?;
    println!(
        "interest years 1–15:  {:>12.2}  ({:.1}% of total)",
        first_15,
        100.0 * first_15.abs() / total_int.abs()
    );
    println!(
        "interest years 16–30: {:>12.2}  ({:.1}% of total)",
        last_15,
        100.0 * last_15.abs() / total_int.abs()
    );
    println!();

    for year in [10u32, 15, 20, 25] {
        let m = year * 12;
        let row = &mortgage.series()[(m - 1) as usize];
        println!(
            "after year {year:>2}: balance={:>10.0}  int_paid={:>10.0}  int_left_if_payoff={:>10.0}  ({:.0}% interest done)",
            row.balance(),
            row.interest_to_date(),
            row.interest_remaining(),
            100.0 * row.interest_to_date().abs() / total_int.abs()
        );
    }

    println!("\n=== year-end snapshot table (first 5 years) ===");
    mortgage
        .series()
        .filter(|row| row.period() % 12 == 0 && row.period() <= 60)
        .print_table(true, true);

    Ok(())
}
