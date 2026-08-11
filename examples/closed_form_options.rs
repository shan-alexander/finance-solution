//! # European closed-form lane: cross Greeks, Black ’76, GK, shared IV
//!
//! Walks the **0.3+ derivatives surface** beyond plain BSM price/Δ:
//!
//! 1. **Cross Greeks** (vanna / volga / charm) on equity BSM  
//! 2. **Black ’76** on a futures-style forward  
//! 3. **Garman–Kohlhagen** FX call with dual rate rhos  
//! 4. **Shared IV** — mid → σ for each model (same Newton + bisection core)
//!
//! ```bash
//! cargo run --example closed_form_options
//! ```
//!
//! ## Sample output (abbreviated)
//!
//! ```text
//! === BSM cross Greeks (ATM equity) ===
//! vanna / volga / charm …
//!
//! === Black ’76 (futures) ===
//! forward delta, parity residual …
//!
//! === Garman–Kohlhagen (FX) ===
//! rho_d / rho_f …
//!
//! === Shared IV round-trips ===
//! BSM / B76 / GK mids → recovered σ
//! ```

use finance_solution::*;

fn main() -> FinanceResult<()> {
    // -------------------------------------------------------------------------
    // 1) Equity BSM + cross Greeks
    // -------------------------------------------------------------------------
    println!("=== BSM cross Greeks (equity ATM 1y) ===\n");
    let bsm = BsmParams::atm_one_year(100.0, 0.05, 0.20);
    let sol = bsm_solution(bsm, OptionType::Call)?;
    let g = sol.greeks;
    let x = sol.cross_greeks;

    println!(
        "price={:.4}  delta={:.4}  vega={:.4}",
        sol.price, g.delta, g.vega
    );
    println!(
        "vanna={:.6}  // ∂Δ/∂σ  (Δ if vol moves +1.0 absolute)",
        x.vanna
    );
    println!(
        "volga={:.4}   // ∂²V/∂σ²  (vega convexity / smile risk)",
        x.volga
    );
    println!(
        "charm={:.4}   // ∂Δ/∂T per year; per day ≈ {:.6}",
        x.charm,
        x.charm_per_calendar_day()
    );
    println!(
        "desk: if IV 20%→21% (Δσ=+0.01), rough Δ change ≈ vanna×0.01 = {:.6}\n",
        x.vanna * 0.01
    );
    sol.print_table();

    // -------------------------------------------------------------------------
    // 2) Black ’76 — options on futures / forward
    // -------------------------------------------------------------------------
    println!("\n=== Black ’76 (F=K=100, T=1y, r=5%, σ=20%) ===\n");
    let f = Black76Params::atm_one_year(100.0, 0.05, 0.20);
    let fsol = black76_solution(f, OptionType::Call)?;
    let fg = fsol.greeks;
    println!(
        "price={:.4}  (BSM q=r map ≈ same; equity q=0 ATM was {:.4})",
        fsol.price, sol.price
    );
    println!(
        "forward Δ={:.4}  gamma={:.6}  vega/pt={:.4}",
        fg.delta,
        fg.gamma,
        fg.vega_per_vol_point()
    );
    println!(
        "parity residual={:.2e}  (model self-check)\n",
        fsol.parity_residual
    );
    fsol.print_table();

    // Engineering: live forward mark
    let mut fut = Black76State::new(f, OptionType::Call)?;
    fut.set_forward(102.0)?;
    println!(
        "Black76State: F=102 → price={:.4}  Δ={:.4}",
        fut.price()?,
        fut.greeks()?.delta
    );

    // -------------------------------------------------------------------------
    // 3) Garman–Kohlhagen FX
    // -------------------------------------------------------------------------
    println!("\n=== Garman–Kohlhagen FX (S=1.10, r_d=5%, r_f=3%, σ=12%) ===\n");
    let fx = GkParams::atm_one_year(1.10, 0.05, 0.03, 0.12);
    let gsol = gk_solution(fx, OptionType::Call)?;
    let gg = gsol.greeks;
    println!("price={:.6}  delta={:.4}", gsol.price, gg.delta);
    println!(
        "rho_domestic={:.6}  // ∂V/∂r_d  (discount)",
        gg.rho_domestic
    );
    println!(
        "rho_foreign={:.6}   // ∂V/∂r_f  (foreign carry; typically <0 for calls)",
        gg.rho_foreign
    );
    println!(
        "cross volga={:.6}  parity={:.2e}\n",
        gsol.cross_greeks.volga, gsol.parity_residual
    );
    gsol.print_table();

    // -------------------------------------------------------------------------
    // 4) Shared IV — market mid → σ for each model
    // -------------------------------------------------------------------------
    println!("\n=== Shared IV (Newton + bisection; same solver core) ===\n");

    let mid_bsm = sol.price;
    let iv_bsm = bsm_implied_vol(bsm, OptionType::Call, mid_bsm)?;
    println!("BSM mid={mid_bsm:.4} → IV={iv_bsm:.6} (expect 0.20)");

    let mid_76 = fsol.price;
    let iv_76 = black76_implied_vol(f, OptionType::Call, mid_76)?;
    println!("B76 mid={mid_76:.4} → IV={iv_76:.6} (expect 0.20)");

    let mid_fx = gsol.price;
    let iv_fx = gk_implied_vol(fx, OptionType::Call, mid_fx)?;
    println!("GK  mid={mid_fx:.6} → IV={iv_fx:.6} (expect 0.12)");

    // Live mark-to-mid
    let mut opt = BsmState::new(bsm, OptionType::Call)?;
    opt.set_vol(0.15)?; // wrong seed
    let recovered = opt.set_vol_from_price(mid_bsm)?;
    println!("\nBsmState::set_vol_from_price(mid) → σ={recovered:.6}");
    println!(
        "\nTrading takeaway: quotes are prices; books speak IV; surfaces need a model.\n\
         Engineering takeaway: one solver, three params packs — pick BSM / B76 / GK by product."
    );

    Ok(())
}
