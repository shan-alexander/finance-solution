//! Suite G — CRR tree **allocation strategy** before vs proposed after.
//!
//! Criterion: `cargo bench --bench crr_tree_buffers`
//!
//! Production [`crr_price`] today allocates a new `Vec` for the terminal layer and
//! **again each backward step** (`next = vec![0.0; step+1]; v = next`). Math is O(N²);
//! the proposal reuses two pre-sized buffers (ping-pong) and, for multi-price desks,
//! a long-lived workspace across many trees.
//!
//! This harness keeps both algorithms **local** (does not change the crate API yet).
//! Parity is asserted at startup. Use results to decide whether to land the change.
//!
//! | Variant | What it models |
//! |---------|----------------|
//! | `alloc_per_layer` | Current production style (new Vec each step) |
//! | `pingpong_prealloc` | One tree: two buffers sized N+1, swap layers |
//! | `workspace_multi` | Many prices: reuse workspace vs alloc each call |

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use finance_solution::derivatives::{crr_price, CrrParams, ExerciseStyle, OptionType};
use std::hint::black_box;
use std::time::Duration;

// ---------------------------------------------------------------------------
// Shared CRR math pieces (mirror production)
// ---------------------------------------------------------------------------

#[inline]
fn payoff(s: f64, k: f64, call: bool) -> f64 {
    if call {
        (s - k).max(0.0)
    } else {
        (k - s).max(0.0)
    }
}

#[derive(Clone, Copy)]
struct TreeIn {
    spot: f64,
    strike: f64,
    time_years: f64,
    rate: f64,
    dividend_yield: f64,
    vol: f64,
    steps: usize,
    american: bool,
    call: bool,
}

impl TreeIn {
    fn from_crr(p: CrrParams, option_type: OptionType) -> Self {
        Self {
            spot: p.spot,
            strike: p.strike,
            time_years: p.time_years,
            rate: p.rate,
            dividend_yield: p.dividend_yield,
            vol: p.vol,
            steps: p.steps,
            american: p.style.is_american(),
            call: matches!(option_type, OptionType::Call),
        }
    }
}

// ---------------------------------------------------------------------------
// BEFORE: alloc a new layer every step (production-like)
// ---------------------------------------------------------------------------

fn price_alloc_per_layer(p: TreeIn) -> f64 {
    let n = p.steps;
    if p.time_years == 0.0 {
        return payoff(p.spot, p.strike, p.call);
    }
    if p.vol == 0.0 {
        let f = p.spot * ((p.rate - p.dividend_yield) * p.time_years).exp();
        let disc = (-p.rate * p.time_years).exp();
        return disc * payoff(f, p.strike, p.call);
    }
    let dt = p.time_years / n as f64;
    let u = (p.vol * dt.sqrt()).exp();
    let d = 1.0 / u;
    let a = ((p.rate - p.dividend_yield) * dt).exp();
    let denom = u - d;
    let p_star = if denom <= 0.0 {
        0.5
    } else {
        ((a - d) / denom).clamp(0.0, 1.0)
    };
    let disc = (-p.rate * dt).exp();
    let q_star = 1.0 - p_star;

    let mut v: Vec<f64> = (0..=n)
        .map(|j| {
            let s = p.spot * u.powi(j as i32) * d.powi((n - j) as i32);
            payoff(s, p.strike, p.call)
        })
        .collect();

    for step in (0..n).rev() {
        let mut next = vec![0.0; step + 1];
        for j in 0..=step {
            let s = p.spot * u.powi(j as i32) * d.powi((step - j) as i32);
            let cont = disc * (p_star * v[j + 1] + q_star * v[j]);
            let ex = payoff(s, p.strike, p.call);
            next[j] = if p.american { cont.max(ex) } else { cont };
        }
        v = next;
    }
    v[0]
}

// ---------------------------------------------------------------------------
// AFTER: ping-pong two preallocated buffers (single tree)
// ---------------------------------------------------------------------------

struct CrrWorkspace {
    a: Vec<f64>,
    b: Vec<f64>,
}

impl CrrWorkspace {
    fn with_capacity(n: usize) -> Self {
        Self {
            a: vec![0.0; n + 1],
            b: vec![0.0; n + 1],
        }
    }

    fn ensure(&mut self, n: usize) {
        let need = n + 1;
        if self.a.len() < need {
            self.a.resize(need, 0.0);
            self.b.resize(need, 0.0);
        }
    }
}

fn price_pingpong(p: TreeIn, ws: &mut CrrWorkspace) -> f64 {
    let n = p.steps;
    if p.time_years == 0.0 {
        return payoff(p.spot, p.strike, p.call);
    }
    if p.vol == 0.0 {
        let f = p.spot * ((p.rate - p.dividend_yield) * p.time_years).exp();
        let disc = (-p.rate * p.time_years).exp();
        return disc * payoff(f, p.strike, p.call);
    }
    ws.ensure(n);
    let dt = p.time_years / n as f64;
    let u = (p.vol * dt.sqrt()).exp();
    let d = 1.0 / u;
    let a = ((p.rate - p.dividend_yield) * dt).exp();
    let denom = u - d;
    let p_star = if denom <= 0.0 {
        0.5
    } else {
        ((a - d) / denom).clamp(0.0, 1.0)
    };
    let disc = (-p.rate * dt).exp();
    let q_star = 1.0 - p_star;

    // Terminal into a
    for j in 0..=n {
        let s = p.spot * u.powi(j as i32) * d.powi((n - j) as i32);
        ws.a[j] = payoff(s, p.strike, p.call);
    }

    // src = a, dst = b, then swap roles each step
    let mut use_a_as_src = true;
    for step in (0..n).rev() {
        let (src, dst) = if use_a_as_src {
            (&ws.a, &mut ws.b)
        } else {
            (&ws.b, &mut ws.a)
        };
        for j in 0..=step {
            let s = p.spot * u.powi(j as i32) * d.powi((step - j) as i32);
            let cont = disc * (p_star * src[j + 1] + q_star * src[j]);
            let ex = payoff(s, p.strike, p.call);
            dst[j] = if p.american { cont.max(ex) } else { cont };
        }
        use_a_as_src = !use_a_as_src;
    }
    if use_a_as_src {
        ws.a[0]
    } else {
        ws.b[0]
    }
}

// ---------------------------------------------------------------------------
// Parity + fixtures
// ---------------------------------------------------------------------------

fn assert_parity() {
    let cases = [
        CrrParams::new(
            100.0,
            100.0,
            1.0,
            0.05,
            0.0,
            0.20,
            50,
            ExerciseStyle::American,
        ),
        CrrParams::new(
            100.0,
            100.0,
            1.0,
            0.05,
            0.0,
            0.20,
            100,
            ExerciseStyle::European,
        ),
        CrrParams::new(
            90.0,
            100.0,
            0.5,
            0.03,
            0.01,
            0.35,
            80,
            ExerciseStyle::American,
        ),
    ];
    let mut ws = CrrWorkspace::with_capacity(100);
    for p in cases {
        for ot in [OptionType::Call, OptionType::Put] {
            let tin = TreeIn::from_crr(p, ot);
            let a = price_alloc_per_layer(tin);
            let b = price_pingpong(tin, &mut ws);
            let crate_px = crr_price(p, ot).expect("crr");
            assert!(
                (a - b).abs() < 1e-10,
                "alloc vs pingpong: {a} vs {b} style={:?} ot={ot:?}",
                p.style
            );
            assert!(
                (a - crate_px).abs() < 1e-9,
                "harness vs crate: {a} vs {crate_px}"
            );
        }
    }
    eprintln!("crr_tree_buffers parity OK (alloc ≡ pingpong ≡ crr_price)");
}

fn fixture_american_put(steps: usize) -> TreeIn {
    TreeIn::from_crr(
        CrrParams::new(
            100.0,
            100.0,
            1.0,
            0.05,
            0.0,
            0.25,
            steps,
            ExerciseStyle::American,
        ),
        OptionType::Put,
    )
}

/// Many slightly different spots (desk-style grid).
fn spot_grid(n: usize) -> Vec<f64> {
    (0..n).map(|i| 80.0 + i as f64 * 0.4).collect()
}

// ---------------------------------------------------------------------------
// Criterion
// ---------------------------------------------------------------------------

fn suite_g_crr_buffers(c: &mut Criterion) {
    assert_parity();

    let step_sizes = [50usize, 100, 200, 500];

    // Single tree: alloc-per-layer vs ping-pong
    {
        let mut g = c.benchmark_group("crr_single_tree");
        g.measurement_time(Duration::from_secs(3));
        g.warm_up_time(Duration::from_secs(1));
        for &n in &step_sizes {
            let tin = fixture_american_put(n);
            g.throughput(Throughput::Elements((n as u64) * (n as u64))); // ~nodes
            g.bench_with_input(BenchmarkId::new("alloc_per_layer", n), &tin, |b, tin| {
                b.iter(|| black_box(price_alloc_per_layer(black_box(*tin))))
            });
            g.bench_with_input(BenchmarkId::new("pingpong_prealloc", n), &tin, |b, tin| {
                let mut ws = CrrWorkspace::with_capacity(n);
                b.iter(|| black_box(price_pingpong(black_box(*tin), black_box(&mut ws))))
            });
            // Production crate API (sanity reference)
            g.bench_with_input(BenchmarkId::new("crate_crr_price", n), &n, |b, &n| {
                let p = CrrParams::new(
                    100.0,
                    100.0,
                    1.0,
                    0.05,
                    0.0,
                    0.25,
                    n,
                    ExerciseStyle::American,
                );
                b.iter(|| {
                    black_box(crr_price(black_box(p), black_box(OptionType::Put)).expect("px"))
                })
            });
        }
        g.finish();
    }

    // Multi-price: fresh alloc each call vs one workspace reused (IV / surface)
    {
        let mut g = c.benchmark_group("crr_multi_price");
        g.measurement_time(Duration::from_secs(4));
        g.warm_up_time(Duration::from_secs(1));
        let grid_sizes = [20usize, 100];
        let steps = 100usize;
        for &gsize in &grid_sizes {
            let spots = spot_grid(gsize);
            g.throughput(Throughput::Elements(gsize as u64));
            let id = format!("spots{gsize}_N{steps}");

            g.bench_function(BenchmarkId::new("alloc_each_call", &id), |b| {
                b.iter(|| {
                    let mut last = 0.0;
                    for &s in black_box(&spots) {
                        let mut tin = fixture_american_put(steps);
                        tin.spot = s;
                        last = price_alloc_per_layer(tin);
                    }
                    black_box(last)
                })
            });

            g.bench_function(BenchmarkId::new("workspace_reused", &id), |b| {
                let mut ws = CrrWorkspace::with_capacity(steps);
                b.iter(|| {
                    let mut last = 0.0;
                    for &s in black_box(&spots) {
                        let mut tin = fixture_american_put(steps);
                        tin.spot = s;
                        last = price_pingpong(tin, &mut ws);
                    }
                    black_box(last)
                })
            });
        }
        g.finish();
    }
}

criterion_group!(benches, suite_g_crr_buffers);
criterion_main!(benches);
