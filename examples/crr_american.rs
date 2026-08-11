//! # CRR binomial tree: European vs American + American IV
//!
//! ```bash
//! cargo run --example crr_american
//! ```
//!
//! Shows:
//! - Small-N **teaching table** (nodes, early exercise flags)
//! - American put ≥ European put premium
//! - Convergence sketch vs BSM for European call
//! - **American IV** from a model mid (Newton on the tree)

use finance_solution::*;

fn main() -> FinanceResult<()> {
    println!("=== CRR teaching tree (N=4 American put) ===\n");
    let small = CrrParams::new(
        100.0,
        100.0,
        1.0,
        0.05,
        0.0,
        0.25,
        4,
        ExerciseStyle::American,
    );
    let sol = crr_solution(small, OptionType::Put)?;
    println!("{}\n", sol.formula());
    println!("{}\n", sol.symbolic_formula());
    sol.print_table();
    println!(
        "\nprice={:.4}  delta={:.4}  vega/pt={:.4}",
        sol.price,
        sol.greeks.delta,
        sol.greeks.vega_per_vol_point()
    );

    println!("\n=== American vs European put (N=100) ===\n");
    let am_p = CrrParams::new(
        100.0,
        100.0,
        1.0,
        0.05,
        0.0,
        0.25,
        100,
        ExerciseStyle::American,
    );
    let am = crr_price(am_p, OptionType::Put)?;
    let eu = crr_price(am_p.with_style(ExerciseStyle::European), OptionType::Put)?;
    println!("American put:  {am:.4}");
    println!("European put:  {eu:.4}");
    println!("EE premium:    {:.4}\n", am - eu);

    println!("=== European call tree vs BSM (N=200) ===\n");
    let eu_c = CrrParams::atm_one_year(100.0, 0.05, 0.20, 200, ExerciseStyle::European);
    let tree_c = crr_price(eu_c, OptionType::Call)?;
    let bsm_c = bsm_price(BsmParams::atm_one_year(100.0, 0.05, 0.20), OptionType::Call)?;
    println!("CRR European call: {tree_c:.4}");
    println!("BSM call:          {bsm_c:.4}");
    println!("|diff|:            {:.4}\n", (tree_c - bsm_c).abs());

    println!("=== American IV from mid ===\n");
    let mkt = am;
    let iv = american_implied_vol(am_p, OptionType::Put, mkt)?;
    println!("mid={mkt:.4} → American IV={iv:.4} (expect ~0.25)");

    Ok(())
}
