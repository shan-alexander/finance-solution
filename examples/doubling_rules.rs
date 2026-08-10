//! # Doubling rules: Rule of 72 / 70 / 69 vs exact math
//!
//! Teaching example for the [`finance_solution::returns`] doubling-time APIs.
//! Read this file **without running it** and you should still see every important
//! number and table the program prints — the same documentation style as
//! [`common_word_problems`](../common_word_problems.rs).
//!
//! ## How to run
//!
//! ```bash
//! cargo run --example doubling_rules
//! ```
//!
//! ## What this example covers
//!
//! 1. **Word problem (8%)** — “About how long to double?” with Rule of 72 vs exact.
//! 2. **Solution struct** — formulas + method comparison table at one rate.
//! 3. **Scalar helpers** — `rule_of_72` / `rule_of_70` / `rule_of_69` / `doubling_time` /
//!    `doubling_time_continuous` side by side.
//! 4. **Multi-rate table** — 1%…15%: when is Rule of 72 “good enough”?
//! 5. **Symmetry check** — $1 grown for exact discrete periods ≈ $2.
//! 6. **Error path** — zero / negative rates return `FinanceError`, not a panic.
//!
//! ---
//!
//! ## Sample run (full terminal output)
//!
//! ### Problem 1 — single rate 8%
//!
//! ```text
//! === Problem 1: How long to double at 8%? ===
//!
//! Word problem:
//!   You earn a steady 8% per year. Roughly how many years until money doubles?
//!   Rule-of-thumb answer (Rule of 72): 72 / 8 = 9 years.
//!
//! solution formulas:
//!   exact 9.0065 = ln(2) / ln(1 + 0.080000); rule_72 9.0000 = 72 / (8.0000)
//!   exact = ln(2)/ln(1+r); rule_72 = 72/(100*r); rule_70 = 70/(100*r); rule_69 = 69/(100*r); continuous = ln(2)/r
//!
//!   rule_of_72:         9.0000 years   (error vs exact: -0.0065)
//!   rule_of_70:         8.7500 years
//!   rule_of_69:         8.6250 years
//!   exact_discrete:     9.0065 years   ← ln(2)/ln(1+r)
//!   exact_continuous:   8.6643 years   ← ln(2)/r  (slightly faster)
//!
//! method comparison table (s.print_table()):
//!
//!           method   years  error_vs_exact
//! ----------------  ------  --------------
//!       rule_of_72  9.0000         -0.0065
//!       rule_of_70  8.7500         -0.2565
//!       rule_of_69  8.6250         -0.3815
//!   exact_discrete  9.0065          0.0000
//! exact_continuous  8.6643         -0.3421
//! ```
//!
//! Teaching note: at 8%, Rule of 72 is almost exact (error only about 0.0065 years).
//!
//! ### Problem 2 — multi-rate comparison 1%…15%
//!
//! ```text
//! === Problem 2: When is Rule of 72 good enough? (1% … 15%) ===
//!
//!     rate  rule_72  rule_70  rule_69    exact  continuous   err_72
//! --------  -------  -------  -------  -------  ----------  -------
//! 0.010000  72.0000  70.0000  69.0000  69.6607     69.3147   2.3393
//! 0.020000  36.0000  35.0000  34.5000  35.0028     34.6574   0.9972
//! 0.030000  24.0000  23.3333  23.0000  23.4498     23.1049   0.5502
//! 0.040000  18.0000  17.5000  17.2500  17.6730     17.3287   0.3270
//! 0.050000  14.4000  14.0000  13.8000  14.2067     13.8629   0.1933
//! 0.060000  12.0000  11.6667  11.5000  11.8957     11.5525   0.1043
//! 0.070000  10.2857  10.0000   9.8571  10.2448      9.9021   0.0409
//! 0.080000   9.0000   8.7500   8.6250   9.0065      8.6643  -0.0065
//! 0.090000   8.0000   7.7778   7.6667   8.0432      7.7016  -0.0432
//! 0.100000   7.2000   7.0000   6.9000   7.2725      6.9315  -0.0725
//! 0.110000   6.5455   6.3636   6.2727   6.6419      6.3013  -0.0964
//! 0.120000   6.0000   5.8333   5.7500   6.1163      5.7762  -0.1163
//! 0.130000   5.5385   5.3846   5.3077   5.6714      5.3319  -0.1330
//! 0.140000   5.1429   5.0000   4.9286   5.2901      4.9511  -0.1472
//! 0.150000   4.8000   4.6667   4.6000   4.9595      4.6210  -0.1595
//! ```
//!
//! Teaching note: Rule of 72 **overstates** years at low rates (err_72 > 0 below ~8%)
//! and **understates** slightly above ~8%. Best “quick mental math” near 6%–10%.
//!
//! ### Problem 3 — symmetry at 8%
//!
//! ```text
//! === Problem 3: Symmetry — grow $1 for exact discrete periods ===
//!
//!   (1 + 0.08)^exact = 2.0000000000   (expect 2)
//! ```
//!
//! ### Problem 4 — invalid rates (Result-only API)
//!
//! ```text
//! === Problem 4: Invalid rates are FinanceError, not panics ===
//!
//!   rule_of_72(0.0)  → Err(ZeroValue { field: "rate" })
//!   doubling_time(-0.05) → Err(InvalidRate { rate: -0.05 })
//!   doubling_compare_rates(&[]) → Err(Unsolvable {
//!       message: "doubling_compare_rates requires at least one rate" })
//! ```
//!
//! ---
//!
//! Related APIs: [`doubling_solution`], [`doubling_compare_rates`], [`rule_of_72`],
//! [`doubling_time`], [`doubling_time_continuous`].

#![allow(unused_variables)]
#![allow(dead_code)]

use finance_solution::*;

pub fn main() -> FinanceResult<()> {
    // Uncomment one problem at a time while studying, or run all (default).
    problem_1_eight_percent_solution()?;
    problem_2_multi_rate_table()?;
    problem_3_symmetry_check()?;
    problem_4_invalid_rates()?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Problem 1
// ---------------------------------------------------------------------------

// Word problem:
//   You can invest at a steady 8% per year, compounded once per year.
//   Roughly how many years until your money doubles?
//
// Mental math (Rule of 72): 72 / 8 = 9 years.
// Exact discrete:           ln(2) / ln(1.08) ≈ 9.0065 years.
// Expect Rule of 72 ≈ exact at this rate (error about −0.0065 years).
//
// Sample solution table is in the //! module docs above.
fn problem_1_eight_percent_solution() -> FinanceResult<()> {
    println!("=== Problem 1: How long to double at 8%? ===\n");
    println!("Word problem:");
    println!("  You earn a steady 8% per year. Roughly how many years until money doubles?");
    println!("  Rule-of-thumb answer (Rule of 72): 72 / 8 = 9 years.\n");

    let rate = 0.08;

    // Prefer the solution API for formulas + method comparison table.
    let s = doubling_solution(rate)?;
    // Outputs DoublingSolution { rate: 0.08, rule_of_72: 9.0, rule_of_70: 8.75,
    //   rule_of_69: 8.625, exact: 9.006468…, exact_continuous: 8.6643…, formula: "…", … }

    println!("solution formulas:");
    println!("  {}", s.formula());
    // Outputs: exact 9.0065 = ln(2) / ln(1 + 0.080000); rule_72 9.0000 = 72 / (8.0000)
    println!("  {}\n", s.symbolic_formula());
    // Outputs: exact = ln(2)/ln(1+r); rule_72 = 72/(100*r); …

    println!(
        "  rule_of_72:         {:.4} years   (error vs exact: {:.4})",
        s.rule_of_72(),
        s.error_rule_of_72()
    );
    // Outputs: rule_of_72:         9.0000 years   (error vs exact: -0.0065)
    println!("  rule_of_70:         {:.4} years", s.rule_of_70());
    // Outputs: rule_of_70:         8.7500 years
    println!("  rule_of_69:         {:.4} years", s.rule_of_69());
    // Outputs: rule_of_69:         8.6250 years
    println!(
        "  exact_discrete:     {:.4} years   ← ln(2)/ln(1+r)",
        s.exact()
    );
    // Outputs: exact_discrete:     9.0065 years
    println!(
        "  exact_continuous:   {:.4} years   ← ln(2)/r  (slightly faster)",
        s.exact_continuous()
    );
    // Outputs: exact_continuous:   8.6643 years
    println!();

    // Same numbers via scalar free functions (hot path when you only need one value).
    assert_rounded_2!(rule_of_72(rate)?, 9.0); // 72 / 8
    assert_rounded_2!(rule_of_70(rate)?, 8.75); // 70 / 8
    assert_rounded_2!(rule_of_69(rate)?, 8.625); // 69 / 8
    assert_rounded_4!(doubling_time(rate)?, 9.0065);
    assert!(doubling_time_continuous(rate)? < doubling_time(rate)?);

    println!("method comparison table (s.print_table()):\n");
    s.print_table();
    // Outputs (see //! module docs for full table):
    //           method   years  error_vs_exact
    // ----------------  ------  --------------
    //       rule_of_72  9.0000         -0.0065
    //       rule_of_70  8.7500         -0.2565
    //       rule_of_69  8.6250         -0.3815
    //   exact_discrete  9.0065          0.0000
    // exact_continuous  8.6643         -0.3421

    println!("\nTeaching note: at 8%, Rule of 72 is almost exact (error ≈ −0.0065 years).\n");
    Ok(())
}

// ---------------------------------------------------------------------------
// Problem 2
// ---------------------------------------------------------------------------

// Word problem / teaching question:
//   Across common annual rates (1% … 15%), when is Rule of 72 a good mental
//   approximation of the true (discrete) doubling time?
//
// Expect:
//   - Low rates (1%–5%): Rule of 72 overstates years (err_72 positive, up to ~2+ years at 1%).
//   - Around 8%: error near zero (Rule of 72 is calibrated near here).
//   - Higher rates (10%–15%): Rule of 72 slightly understates years (err_72 negative).
//
// Full multi-rate table is in the //! module docs above.
fn problem_2_multi_rate_table() -> FinanceResult<()> {
    println!("=== Problem 2: When is Rule of 72 good enough? (1% … 15%) ===\n");

    // 1%, 2%, …, 15% as decimal rates.
    let rates: Vec<f64> = (1..=15).map(|i| i as f64 / 100.0).collect();

    let table = doubling_compare_rates(&rates)?;
    // Outputs DoublingRateSeries with 15 rows (one per rate).

    assert_eq!(table.rows().len(), 15);

    // Spot-check endpoints without printing every row in prose again.
    let first = &table.rows()[0]; // 1%
    assert_rounded_2!(first.rate(), 0.01);
    assert_rounded_2!(first.rule_of_72(), 72.0); // 72 / 1
                                                 // err_72 at 1% is about +2.34 years (Rule of 72 too optimistic about waiting).
    assert!(first.error_rule_of_72() > 2.0);

    let at_8 = &table.rows()[7]; // 8%
    assert_rounded_2!(at_8.rate(), 0.08);
    assert!(at_8.error_rule_of_72().abs() < 0.01); // nearly exact

    let last = &table.rows()[14]; // 15%
    assert_rounded_2!(last.rate(), 0.15);
    assert!(last.error_rule_of_72() < 0.0); // understates years slightly

    table.print_table();
    // Outputs full table — see //! module docs for the complete grid.

    println!("\nTeaching note: Rule of 72 overstates years at low rates and understates");
    println!("slightly above ~8%. Best mental math in the ~6%–10% neighborhood.\n");
    Ok(())
}

// ---------------------------------------------------------------------------
// Problem 3
// ---------------------------------------------------------------------------

// Symmetry / sanity check:
//   If exact discrete doubling time is T = ln(2)/ln(1+r), then
//   (1 + r)^T must equal 2 (within floating-point noise).
//
// Expect: (1.08)^9.0065… = 2.0000000000
fn problem_3_symmetry_check() -> FinanceResult<()> {
    println!("=== Problem 3: Symmetry — grow $1 for exact discrete periods ===\n");

    let rate = 0.08;
    let s = doubling_solution(rate)?;
    let grown = (1.0 + s.rate()).powf(s.exact());
    // Outputs: (1 + 0.08)^exact ≈ 2.0

    println!("  (1 + {:.2})^exact = {:.10}   (expect 2)", rate, grown);
    // Outputs: (1 + 0.08)^exact = 2.0000000000   (expect 2)

    assert!((grown - 2.0).abs() < 1e-9);
    println!("  ✓ symmetry holds: exact discrete periods double the principal.\n");
    Ok(())
}

// ---------------------------------------------------------------------------
// Problem 4
// ---------------------------------------------------------------------------

// Error-handling problem (v0.1 Result-only API):
//   What happens if a student plugs in 0% or a negative rate?
//
// Expect: FinanceError variants — never a panic on domain input.
//   rule_of_72(0.0)       → Err(ZeroValue { field: "rate" })
//   doubling_time(-0.05)  → Err(InvalidRate { rate: -0.05 })
//   doubling_compare_rates(&[]) → Err(Unsolvable { … "one rate" … })
fn problem_4_invalid_rates() -> FinanceResult<()> {
    println!("=== Problem 4: Invalid rates are FinanceError, not panics ===\n");

    match rule_of_72(0.0) {
        Err(FinanceError::ZeroValue { field }) => {
            println!("  rule_of_72(0.0)  → Err(ZeroValue {{ field: \"{field}\" }})");
            // Outputs: rule_of_72(0.0)  → Err(ZeroValue { field: "rate" })
            assert_eq!(field, "rate");
        }
        other => panic!("expected ZeroValue, got {other:?}"),
    }

    match doubling_time(-0.05) {
        Err(FinanceError::InvalidRate { rate }) => {
            println!("  doubling_time(-0.05) → Err(InvalidRate {{ rate: {rate} }})");
            // Outputs: doubling_time(-0.05) → Err(InvalidRate { rate: -0.05 })
            assert!(rate < 0.0);
        }
        other => panic!("expected InvalidRate, got {other:?}"),
    }

    match doubling_compare_rates(&[]) {
        Err(FinanceError::Unsolvable { message }) => {
            println!(
                "  doubling_compare_rates(&[]) → Err(Unsolvable {{ message: \"{message}\" }})"
            );
            // Outputs: … requires at least one rate
            assert!(message.contains("one rate"));
        }
        other => panic!("expected Unsolvable, got {other:?}"),
    }

    println!("\n  ✓ fallible API: bad inputs are values you can match on with `?`.\n");
    Ok(())
}
