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

criterion_group!(
    benches,
    suite_a_core_math,
    suite_b_solution_overhead,
    suite_c_stocks
);
criterion_main!(benches);
