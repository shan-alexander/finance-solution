//! # Black–Scholes: trading + engineering walkthrough
//!
//! Shows how a quant might **think** about a European option (price, Greeks, IV, hedge
//! sketch) and how an engineer might **wire** live updates with [`BsmState`] alongside
//! underlier TA (pattern only — no sockets here).
//!
//! ```bash
//! cargo run --example bsm_option
//! ```
//!
//! ## Sample output (abbreviated)
//!
//! ```text
//! === Trading view: ATM 1y call ===
//! model price ≈ 10.45
//! delta ≈ 0.64  → hedge: short ~0.64 shares per long call
//! vega/vol-pt ≈ 0.375  → +1% IV ≈ +0.375 value
//! theta/day ≈ -0.0176  → overnight bleed if long
//!
//! === Engineering: BsmState underlier tick ===
//! spot 100 → 105: delta and price rise (ITM-er call)
//!
//! === Join pattern (pseudocode in comments) ===
//! // on_1m_bar  → ta.push(...)
//! // on_spot    → for opt in chain { opt.set_spot(s) }
//! ```

use finance_solution::*;

fn main() -> FinanceResult<()> {
    // -------------------------------------------------------------------------
    // Contract + market snapshot (what a research notebook starts with)
    // -------------------------------------------------------------------------
    let p = BsmParams::atm_one_year(100.0, 0.05, 0.20);

    println!("=== Trading view: ATM 1y European call ===\n");
    println!("Inputs: S=K=100, T=1y, r=5% cont., q=0, σ=20%\n");

    let sol = bsm_solution(p, OptionType::Call)?;
    println!("model price:     {:>10.4}", sol.price);
    println!("intrinsic:       {:>10.4}  (OTM ATM → 0)", sol.intrinsic);
    println!(
        "time value:      {:>10.4}  (all premium is optionality here)",
        sol.time_value
    );
    println!("fwd moneyness:   {:>10.4}", sol.forward_moneyness);
    println!(
        "parity residual: {:>10.2e}  (model self-check; expect ~0)\n",
        sol.parity_residual
    );

    let g = sol.greeks;
    println!("Greeks (raw model units):");
    println!(
        "  delta = {:>8.4}   // share equivalent per long call",
        g.delta
    );
    println!("  gamma = {:>8.6}   // convexity of delta", g.gamma);
    println!("  vega  = {:>8.4}   // per +1.0 absolute vol", g.vega);
    println!("  theta = {:>8.4}   // per year", g.theta);
    println!("  rho   = {:>8.4}   // per +1.0 absolute rate", g.rho);
    println!();
    println!("Desk-scaled helpers:");
    println!(
        "  vega per vol-point (1%): {:>8.4}  // P&L if IV 20%→21%",
        g.vega_per_vol_point()
    );
    println!(
        "  theta per calendar day:  {:>8.4}  // rough overnight bleed",
        g.theta_per_calendar_day()
    );
    println!();
    println!(
        "Hedge sketch: long 1 call → short ≈ {:.2} shares to flatten delta.\n",
        g.delta
    );

    println!("Solution table:");
    sol.print_table();

    // -------------------------------------------------------------------------
    // IV: market speaks prices; desks speak vol
    // -------------------------------------------------------------------------
    println!("\n=== Trading view: implied vol from mid ===\n");
    let mid = sol.price; // pretend model price is the mid
    let iv = bsm_implied_vol(p, OptionType::Call, mid)?;
    println!("mid {mid:.4} → IV {iv:.4} (expect 0.20 if mid is fair BSM)\n");

    // -------------------------------------------------------------------------
    // Engineering: live state for one contract
    // -------------------------------------------------------------------------
    println!("=== Engineering: BsmState on underlier ticks ===\n");
    let mut opt = BsmState::new(p, OptionType::Call)?;
    println!(
        "spot={:.1}  price={:.4}  delta={:.4}",
        opt.params().spot,
        opt.price()?,
        opt.greeks()?.delta
    );
    opt.set_spot(105.0)?;
    println!(
        "spot={:.1}  price={:.4}  delta={:.4}  (call deeper ITM → higher Δ)",
        opt.params().spot,
        opt.price()?,
        opt.greeks()?.delta
    );
    opt.set_time_years(0.5)?;
    println!(
        "T=0.5y     price={:.4}  theta/day={:.4}",
        opt.price()?,
        opt.greeks()?.theta_per_calendar_day()
    );

    // -------------------------------------------------------------------------
    // Engineering: how this coexists with TA (pattern only)
    // -------------------------------------------------------------------------
    println!("\n=== Engineering: TA + options in one process (pattern) ===\n");
    println!(
        "Your engine owns maps, not this crate:\n\
         \n\
           underlier_ta:  StochState / EmaState / …   // 1m / 5s bars\n\
           option_book:   HashMap<Key, BsmState>     // chain\n\
         \n\
         on_1m_bar(bars)  → underlier_ta.push_bars(...)\n\
         on_spot(s)       → for o in option_book.values_mut() {{ o.set_spot(s)?; }}\n\
         on_opt_quote(k, mid) → option_book[k].set_vol_from_price(mid)?;\n\
         \n\
         Parallelism: rayon over option_book keys on spot moves (optional).\n\
         Async: only for I/O; math stays sync.\n"
    );

    // Tiny demo that both APIs compile in one binary
    let mut ema = EmaState::new(5)?;
    for px in [100.0, 101.0, 102.0, 101.5, 103.0] {
        let _ = ema.push(px)?;
    }
    println!(
        "Demo underlier EMA(5) last={:?} while option delta={:.4}",
        ema.last(),
        opt.greeks()?.delta
    );

    Ok(())
}
