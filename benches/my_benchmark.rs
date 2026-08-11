//! Criterion benches for Result-only hot paths (P7 / E3).
//!
//! Run: `cargo bench`
//! Fixtures are valid by construction; `.expect` is only for bench harness.

use criterion::{criterion_group, criterion_main, Criterion};
use finance_solution::*;
use std::hint::black_box;

fn suite_a_core_math(c: &mut Criterion) {
    c.bench_function("future_value", |b| {
        b.iter(|| {
            future_value(
                black_box(0.05),
                black_box(10),
                black_box(-1_000.0),
                black_box(false),
            )
            .expect("bench fixture")
        })
    });

    c.bench_function("present_value", |b| {
        b.iter(|| {
            present_value(
                black_box(0.034),
                black_box(5),
                black_box(250_000.0),
                black_box(false),
            )
            .expect("bench fixture")
        })
    });

    c.bench_function("payment", |b| {
        b.iter(|| {
            payment(
                black_box(0.01),
                black_box(36),
                black_box(10_000.0),
                black_box(0.0),
                black_box(false),
            )
            .expect("bench fixture")
        })
    });

    c.bench_function("rate", |b| {
        b.iter(|| {
            rate(
                black_box(10),
                black_box(-10_000.0),
                black_box(15_000.0),
                black_box(false),
            )
            .expect("bench fixture")
        })
    });

    c.bench_function("periods", |b| {
        b.iter(|| {
            periods(
                black_box(0.05),
                black_box(-1_000.0),
                black_box(2_000.0),
                black_box(false),
            )
            .expect("bench fixture")
        })
    });
}

fn suite_b_solution_overhead(c: &mut Criterion) {
    c.bench_function("future_value_solution", |b| {
        b.iter(|| {
            future_value_solution(
                black_box(0.05),
                black_box(10),
                black_box(-1_000.0),
                black_box(false),
            )
            .expect("bench fixture")
        })
    });

    c.bench_function("payment_solution", |b| {
        b.iter(|| {
            payment_solution(
                black_box(0.01),
                black_box(36),
                black_box(10_000.0),
                black_box(0.0),
                black_box(false),
            )
            .expect("bench fixture")
        })
    });

    c.bench_function("amortization_solution_series_12", |b| {
        b.iter(|| {
            let sol = amortization_solution(
                black_box(0.08 / 12.0),
                black_box(12),
                black_box(10_000.0),
                black_box(0.0),
                black_box(false),
            )
            .expect("bench fixture");
            black_box(sol.series().len())
        })
    });
}

fn suite_c_stocks(c: &mut Criterion) {
    let prices: Vec<f64> = (0..252)
        .map(|i| 100.0 * (1.0 + 0.0004 * i as f64))
        .collect();

    c.bench_function("simple_returns_252", |b| {
        b.iter(|| simple_returns(black_box(&prices)).expect("bench fixture"))
    });

    c.bench_function("volatility_252", |b| {
        b.iter(|| {
            let r = simple_returns(black_box(&prices)).expect("bench fixture");
            volatility(black_box(&r)).expect("bench fixture")
        })
    });
}

/// Suite D — technical analysis hot path (series, no formula strings).
///
/// Quant pattern under test: `const` / once-built params + pure series compute.
fn suite_d_ta(c: &mut Criterion) {
    let n = 252usize;
    let closes: Vec<f64> = (0..n)
        .map(|i| 100.0 + (i as f64) * 0.05 + ((i % 7) as f64) * 0.1)
        .collect();
    let high: Vec<f64> = closes.iter().map(|x| x + 0.5).collect();
    let low: Vec<f64> = closes.iter().map(|x| x - 0.5).collect();

    let closes_2k: Vec<f64> = (0..2520).map(|i| 100.0 + (i as f64) * 0.02).collect();
    let high_2k: Vec<f64> = closes_2k.iter().map(|x| x + 0.4).collect();
    let low_2k: Vec<f64> = closes_2k.iter().map(|x| x - 0.4).collect();

    // Validated once outside the iter — mirrors production quant usage.
    const FAST_9_3: StochasticParams = StochasticParams::fast(9, 3);
    const FULL_14_3_3: StochasticParams = StochasticParams::full(14, 3, 3);
    let stoch_fast = ValidatedStochastic::new(FAST_9_3).expect("bench params");
    let stoch_full = ValidatedStochastic::new(FULL_14_3_3).expect("bench params");

    c.bench_function("sma_20_252", |b| {
        b.iter(|| sma(black_box(&closes), black_box(20)).expect("bench fixture"))
    });

    c.bench_function("ema_20_252", |b| {
        b.iter(|| ema(black_box(&closes), black_box(20)).expect("bench fixture"))
    });

    c.bench_function("sma_20_2520", |b| {
        b.iter(|| sma(black_box(&closes_2k), black_box(20)).expect("bench fixture"))
    });

    c.bench_function("ema_20_2520", |b| {
        b.iter(|| ema(black_box(&closes_2k), black_box(20)).expect("bench fixture"))
    });

    c.bench_function("sma_last_20_252", |b| {
        b.iter(|| sma_last(black_box(&closes), black_box(20)).expect("bench fixture"))
    });

    c.bench_function("stoch_fast_9_3_252", |b| {
        b.iter(|| {
            stoch_fast
                .compute(black_box(&high), black_box(&low), black_box(&closes))
                .expect("bench fixture")
        })
    });

    c.bench_function("stoch_full_14_3_3_252", |b| {
        b.iter(|| {
            stoch_full
                .compute(black_box(&high), black_box(&low), black_box(&closes))
                .expect("bench fixture")
        })
    });

    c.bench_function("stoch_fast_9_3_2520", |b| {
        b.iter(|| {
            stoch_fast
                .compute(
                    black_box(&high_2k),
                    black_box(&low_2k),
                    black_box(&closes_2k),
                )
                .expect("bench fixture")
        })
    });

    // Anti-pattern comparison: rebuild ValidatedStochastic inside the loop (should be near-noise vs series).
    c.bench_function("stoch_revalidate_each_call_252", |b| {
        b.iter(|| {
            let v = ValidatedStochastic::new(black_box(FAST_9_3)).expect("bench");
            v.compute(black_box(&high), black_box(&low), black_box(&closes))
                .expect("bench fixture")
        })
    });

    const MACD_STD: MacdParams = MacdParams::standard();
    const BB_STD: BollingerParams = BollingerParams::standard();
    let macd_eng = ValidatedMacd::new(MACD_STD).expect("bench");
    let bb_eng = ValidatedBollinger::new(BB_STD).expect("bench");
    let volume: Vec<f64> = (0..n).map(|i| 1_000.0 + i as f64).collect();
    let volume_2k: Vec<f64> = (0..2520).map(|i| 1_000.0 + i as f64).collect();
    let rvol_eng = ValidatedRvol::new(RvolParams::days_20()).expect("bench");
    let vwap_eng = ValidatedVwap::new(VwapParams::cumulative_typical()).expect("bench");
    let kc_eng = ValidatedKeltner::new(KeltnerParams::standard()).expect("bench");

    c.bench_function("macd_12_26_9_252", |b| {
        b.iter(|| macd_eng.compute(black_box(&closes)).expect("bench"))
    });
    c.bench_function("bollinger_20_2_252", |b| {
        b.iter(|| bb_eng.compute(black_box(&closes)).expect("bench"))
    });
    c.bench_function("keltner_std_252", |b| {
        b.iter(|| {
            kc_eng
                .compute(black_box(&high), black_box(&low), black_box(&closes))
                .expect("bench")
        })
    });
    c.bench_function("vwap_cum_252", |b| {
        b.iter(|| {
            vwap_eng
                .compute(
                    black_box(&high),
                    black_box(&low),
                    black_box(&closes),
                    black_box(&volume),
                )
                .expect("bench")
        })
    });
    c.bench_function("rvol_20_252", |b| {
        b.iter(|| rvol_eng.compute(black_box(&volume)).expect("bench"))
    });
    c.bench_function("rvol_20_2520", |b| {
        b.iter(|| rvol_eng.compute(black_box(&volume_2k)).expect("bench"))
    });
}

/// Suite E — European closed-form options (BSM / Black ’76 / GK).
fn suite_e_derivatives(c: &mut Criterion) {
    let bsm = BsmParams::atm_one_year(100.0, 0.05, 0.20);
    let b76 = Black76Params::atm_one_year(100.0, 0.05, 0.20);
    let gk = GkParams::atm_one_year(1.10, 0.05, 0.03, 0.12);
    let eng = ValidatedBsm::new(bsm).expect("bench params");

    c.bench_function("bsm_price_atm_1y", |b| {
        b.iter(|| bsm_price(black_box(bsm), black_box(OptionType::Call)).expect("bench"))
    });
    c.bench_function("bsm_greeks_atm_1y", |b| {
        b.iter(|| bsm_greeks(black_box(bsm), black_box(OptionType::Call)).expect("bench"))
    });
    c.bench_function("bsm_cross_greeks_atm_1y", |b| {
        b.iter(|| bsm_cross_greeks(black_box(bsm), black_box(OptionType::Call)).expect("bench"))
    });
    c.bench_function("bsm_validated_price_atm_1y", |b| {
        b.iter(|| eng.price(black_box(OptionType::Call)).expect("bench"))
    });
    c.bench_function("bsm_implied_vol_atm_1y", |b| {
        let mkt = bsm_price(bsm, OptionType::Call).expect("fixture");
        b.iter(|| {
            bsm_implied_vol(black_box(bsm), black_box(OptionType::Call), black_box(mkt))
                .expect("bench")
        })
    });
    c.bench_function("bsm_solution_atm_1y", |b| {
        b.iter(|| bsm_solution(black_box(bsm), black_box(OptionType::Call)).expect("bench"))
    });
    c.bench_function("black76_price_atm_1y", |b| {
        b.iter(|| black76_price(black_box(b76), black_box(OptionType::Call)).expect("bench"))
    });
    c.bench_function("black76_greeks_atm_1y", |b| {
        b.iter(|| black76_greeks(black_box(b76), black_box(OptionType::Call)).expect("bench"))
    });
    c.bench_function("gk_price_atm_1y", |b| {
        b.iter(|| gk_price(black_box(gk), black_box(OptionType::Call)).expect("bench"))
    });
    c.bench_function("gk_greeks_atm_1y", |b| {
        b.iter(|| gk_greeks(black_box(gk), black_box(OptionType::Call)).expect("bench"))
    });
}

criterion_group!(
    benches,
    suite_a_core_math,
    suite_b_solution_overhead,
    suite_c_stocks,
    suite_d_ta,
    suite_e_derivatives
);
criterion_main!(benches);
