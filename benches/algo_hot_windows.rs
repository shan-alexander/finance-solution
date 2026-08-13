//! Suite F — **before vs after** algorithm improvements for TA hot windows.
//!
//! Criterion: `cargo bench --bench algo_hot_windows`
//!
//! | Indicator | Before (reference in this harness) | After (crate `*State`) |
//! |-----------|------------------------------------|-------------------------|
//! | WMA       | O(window) reweight each full bar   | O(1) slide `W' = W − Σ + n·p` |
//! | Bollinger | O(window) variance scan            | O(1) via `Σ` / `Σx²` |
//! | LinReg    | O(window) OLS recompute            | O(1) sufficient stats |
//! | HMA       | composed of naive WMAs             | composed of O(1) WMAs |
//! | Stoch     | O(k_period) max/min scan           | amortized O(1) sliding max/min |
//! | Donchian  | O(period) max/min scan             | amortized O(1) sliding max/min |
//!
//! Parity is asserted once at startup (same outputs within tight abs/rel tol).
//! Timed work is a full streaming pass: `push` every bar of a synthetic series.

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use finance_solution::stocks::ta::{
    BollingerBarOutput, BollingerParams, BollingerState, DonchianBarOutput, DonchianParams,
    DonchianState, HmaState, LinRegBar, LinRegParams, LinRegState, StdevKind, StochState,
    StochasticParams, WmaState,
};
use std::collections::VecDeque;
use std::hint::black_box;
use std::time::Duration;

// ---------------------------------------------------------------------------
// Synthetic fixture (deterministic, no NaNs)
// ---------------------------------------------------------------------------

fn synthetic_closes(n: usize) -> Vec<f64> {
    (0..n)
        .map(|i| {
            let t = i as f64;
            100.0 + 0.03 * t + (t * 0.17).sin() * 2.5 + ((i % 11) as f64) * 0.05
        })
        .collect()
}

/// Synthetic H/L/C with valid high ≥ low.
fn synthetic_hlc(n: usize) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let close = synthetic_closes(n);
    let high: Vec<f64> = close
        .iter()
        .enumerate()
        .map(|(i, &c)| c + 0.4 + ((i % 5) as f64) * 0.05)
        .collect();
    let low: Vec<f64> = close
        .iter()
        .enumerate()
        .map(|(i, &c)| c - 0.4 - ((i % 3) as f64) * 0.04)
        .collect();
    (high, low, close)
}

// ---------------------------------------------------------------------------
// Shared ring (bench-only) — same storage shape for fair before/after
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
struct BenchRing {
    buf: VecDeque<f64>,
    cap: usize,
    sum: f64,
}

impl BenchRing {
    fn new(cap: usize) -> Self {
        Self {
            buf: VecDeque::with_capacity(cap),
            cap,
            sum: 0.0,
        }
    }

    fn is_full(&self) -> bool {
        self.buf.len() == self.cap
    }

    fn sum(&self) -> f64 {
        self.sum
    }

    fn push(&mut self, v: f64) -> Option<f64> {
        if self.buf.len() < self.cap {
            self.buf.push_back(v);
            self.sum += v;
            None
        } else {
            let old = self.buf.pop_front().unwrap();
            self.sum += v - old;
            self.buf.push_back(v);
            Some(old)
        }
    }

    fn iter(&self) -> impl Iterator<Item = f64> + '_ {
        self.buf.iter().copied()
    }
}

// ---------------------------------------------------------------------------
// BEFORE: naive O(window) streaming states (historical algorithms)
// ---------------------------------------------------------------------------

/// Pre-optimization WMA: full reweight of ordered window each ready bar.
struct WmaBefore {
    ring: BenchRing,
    weight_sum: f64,
}

impl WmaBefore {
    fn new(period: usize) -> Self {
        Self {
            ring: BenchRing::new(period),
            weight_sum: (period * (period + 1)) as f64 / 2.0,
        }
    }

    fn push(&mut self, close: f64) -> Option<f64> {
        let _ = self.ring.push(close);
        if !self.ring.is_full() {
            return None;
        }
        let mut num = 0.0;
        for (i, p) in self.ring.iter().enumerate() {
            num += (i + 1) as f64 * p;
        }
        Some(num / self.weight_sum)
    }
}

/// Pre-optimization Bollinger: SMA + full-window stdev each ready bar.
struct BollingerBefore {
    period: usize,
    num_std: f64,
    stdev: StdevKind,
    ring: BenchRing,
}

impl BollingerBefore {
    fn new(params: BollingerParams) -> Self {
        Self {
            period: params.period,
            num_std: params.num_std,
            stdev: params.stdev,
            ring: BenchRing::new(params.period),
        }
    }

    fn push(&mut self, close: f64) -> Option<BollingerBarOutput> {
        let _ = self.ring.push(close);
        if !self.ring.is_full() {
            return None;
        }
        let n = self.period as f64;
        let mid = self.ring.sum() / n;
        let mut ss = 0.0;
        for x in self.ring.iter() {
            let d = x - mid;
            ss += d * d;
        }
        let sd = match self.stdev {
            StdevKind::Population => (ss / n).sqrt(),
            StdevKind::Sample => {
                if n > 1.0 {
                    (ss / (n - 1.0)).sqrt()
                } else {
                    0.0
                }
            }
        };
        let band = self.num_std * sd;
        let upper = mid + band;
        let lower = mid - band;
        let width = upper - lower;
        let pct_b = if width > 0.0 {
            Some((close - lower) / width)
        } else {
            None
        };
        Some(BollingerBarOutput {
            middle: mid,
            upper,
            lower,
            pct_b,
        })
    }
}

/// Pre-optimization LinReg: recompute OLS on ordered window each ready bar.
struct LinRegBefore {
    period: usize,
    ring: BenchRing,
}

impl LinRegBefore {
    fn new(period: usize) -> Self {
        Self {
            period,
            ring: BenchRing::new(period),
        }
    }

    fn push(&mut self, y: f64) -> Option<LinRegBar> {
        let _ = self.ring.push(y);
        if !self.ring.is_full() {
            return None;
        }
        let n = self.period;
        let nf = n as f64;
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let mut sum_xy = 0.0;
        let mut sum_x2 = 0.0;
        let mut sum_y2 = 0.0;
        for (i, yi) in self.ring.iter().enumerate() {
            let x = i as f64;
            sum_x += x;
            sum_y += yi;
            sum_xy += x * yi;
            sum_x2 += x * x;
            sum_y2 += yi * yi;
        }
        let denom = nf * sum_x2 - sum_x * sum_x;
        let (slope, intercept) = if denom.abs() < 1e-18 {
            (0.0, sum_y / nf)
        } else {
            let slope = (nf * sum_xy - sum_x * sum_y) / denom;
            let intercept = (sum_y - slope * sum_x) / nf;
            (slope, intercept)
        };
        let mean_y = sum_y / nf;
        let ss_tot = (sum_y2 - nf * mean_y * mean_y).max(0.0);
        let mut ss_res = 0.0;
        for (i, yi) in self.ring.iter().enumerate() {
            let pred = intercept + slope * (i as f64);
            let e = yi - pred;
            ss_res += e * e;
        }
        let r_squared = if ss_tot <= 0.0 {
            1.0
        } else {
            (1.0 - ss_res / ss_tot).clamp(0.0, 1.0)
        };
        let angle_radians = slope.atan();
        Some(LinRegBar {
            slope,
            intercept,
            r_squared,
            angle_radians,
            angle_degrees: angle_radians.to_degrees(),
        })
    }
}

/// Pre-optimization HMA: Hull built from naive WMAs (same structure as production).
struct HmaBefore {
    half: WmaBefore,
    full: WmaBefore,
    sqrt_wma: WmaBefore,
}

impl HmaBefore {
    fn new(period: usize) -> Self {
        let half_len = period / 2;
        let sqrt_len = (period as f64).sqrt().round() as usize;
        let sqrt_len = sqrt_len.max(1);
        Self {
            half: WmaBefore::new(half_len.max(1)),
            full: WmaBefore::new(period),
            sqrt_wma: WmaBefore::new(sqrt_len),
        }
    }

    fn push(&mut self, close: f64) -> Option<f64> {
        let wh = self.half.push(close);
        let wf = self.full.push(close);
        match (wh, wf) {
            (Some(h), Some(f)) => {
                let raw = 2.0 * h - f;
                self.sqrt_wma.push(raw)
            }
            _ => None,
        }
    }
}

/// Pre-optimization Donchian: O(window) max/min scan each ready bar.
struct DonchianBefore {
    period: usize,
    high: BenchRing,
    low: BenchRing,
}

impl DonchianBefore {
    fn new(period: usize) -> Self {
        Self {
            period,
            high: BenchRing::new(period),
            low: BenchRing::new(period),
        }
    }

    fn push(&mut self, high: f64, low: f64) -> Option<DonchianBarOutput> {
        let _ = self.high.push(high);
        let _ = self.low.push(low);
        if !self.high.is_full() {
            return None;
        }
        let u = self.high.iter().fold(f64::NEG_INFINITY, f64::max);
        let l = self.low.iter().fold(f64::INFINITY, f64::min);
        Some(DonchianBarOutput {
            upper: u,
            middle: 0.5 * (u + l),
            lower: l,
        })
    }

    #[allow(dead_code)]
    fn period(&self) -> usize {
        self.period
    }
}

/// Pre-optimization Fast Stoch: O(k_period) HH/LL scan; %D via ring mean.
struct StochBefore {
    k_period: usize,
    d_period: usize,
    high: BenchRing,
    low: BenchRing,
    smooth_k: BenchRing,
    prev_raw_k: Option<f64>,
    last_k: Option<f64>,
    last_d: Option<f64>,
}

impl StochBefore {
    fn new(k_period: usize, d_period: usize) -> Self {
        Self {
            k_period,
            d_period,
            high: BenchRing::new(k_period),
            low: BenchRing::new(k_period),
            smooth_k: BenchRing::new(d_period),
            prev_raw_k: None,
            last_k: None,
            last_d: None,
        }
    }

    /// Returns `Some((%K, %D))` when both defined (fast stoch: k_smooth=1).
    fn push(&mut self, high: f64, low: f64, close: f64) -> Option<(f64, f64)> {
        let _ = self.high.push(high);
        let _ = self.low.push(low);
        if !self.high.is_full() {
            return None;
        }
        let hh = self.high.iter().fold(f64::NEG_INFINITY, f64::max);
        let ll = self.low.iter().fold(f64::INFINITY, f64::min);
        let range = hh - ll;
        let raw = if range == 0.0 {
            self.prev_raw_k.unwrap_or(50.0)
        } else {
            100.0 * (close - ll) / range
        };
        self.prev_raw_k = Some(raw);
        self.last_k = Some(raw);
        let _ = self.smooth_k.push(raw);
        if !self.smooth_k.is_full() {
            return None;
        }
        let d = self.smooth_k.sum() / self.d_period as f64;
        self.last_d = Some(d);
        Some((raw, d))
    }

    #[allow(dead_code)]
    fn k_period(&self) -> usize {
        self.k_period
    }
}

// ---------------------------------------------------------------------------
// Parity (document correct answer)
// ---------------------------------------------------------------------------

fn approx_opt(a: Option<f64>, b: Option<f64>, tol: f64) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(x), Some(y)) => (x - y).abs() <= tol || (x - y).abs() <= tol * x.abs().max(1.0),
        _ => false,
    }
}

fn max_abs_diff(a: f64, b: f64) -> f64 {
    (a - b).abs()
}

/// Fail loud if before/after disagree on a long path (parity gate for the suite).
fn assert_parity(closes: &[f64]) {
    let period = 20usize;
    let tol = 1e-9;

    // --- WMA ---
    let mut before = WmaBefore::new(period);
    let mut after = WmaState::new(period).expect("wma");
    let mut wma_checked = 0usize;
    for &c in closes {
        let b = before.push(c);
        let a = after.push(c).expect("wma push");
        assert!(
            approx_opt(b, a, tol),
            "WMA parity fail: before={b:?} after={a:?}"
        );
        if b.is_some() {
            wma_checked += 1;
        }
    }
    assert!(wma_checked > 0, "WMA never warmed");

    // --- Bollinger ---
    let params = BollingerParams::standard();
    let mut bb_b = BollingerBefore::new(params);
    let mut bb_a = BollingerState::new(params).expect("bb");
    let mut bb_checked = 0usize;
    let mut bb_max = 0.0_f64;
    for &c in closes {
        let b = bb_b.push(c);
        let a = bb_a.push(c).expect("bb push");
        match (b, a) {
            (None, None) => {}
            (Some(x), Some(y)) => {
                bb_max = bb_max
                    .max(max_abs_diff(x.middle, y.middle))
                    .max(max_abs_diff(x.upper, y.upper))
                    .max(max_abs_diff(x.lower, y.lower));
                assert!(approx_opt(x.pct_b, y.pct_b, 1e-8), "BB %B parity fail");
                assert!(
                    max_abs_diff(x.middle, y.middle) < 1e-9
                        && max_abs_diff(x.upper, y.upper) < 1e-8
                        && max_abs_diff(x.lower, y.lower) < 1e-8,
                    "BB band parity fail mid={} up={} lo={}",
                    max_abs_diff(x.middle, y.middle),
                    max_abs_diff(x.upper, y.upper),
                    max_abs_diff(x.lower, y.lower)
                );
                bb_checked += 1;
            }
            other => panic!("BB warm-up parity: {other:?}"),
        }
    }
    assert!(bb_checked > 0, "BB never warmed");

    // --- LinReg ---
    let lr_p = LinRegParams::new(period);
    let mut lr_b = LinRegBefore::new(period);
    let mut lr_a = LinRegState::new(lr_p).expect("lr");
    let mut lr_checked = 0usize;
    for &c in closes {
        let b = lr_b.push(c);
        let a = lr_a.push(c).expect("lr push");
        match (b, a) {
            (None, None) => {}
            (Some(x), Some(y)) => {
                assert!(
                    max_abs_diff(x.slope, y.slope) < 1e-9
                        && max_abs_diff(x.intercept, y.intercept) < 1e-8
                        && max_abs_diff(x.r_squared, y.r_squared) < 1e-8,
                    "LinReg parity: slope {} vs {}, r2 {} vs {}",
                    x.slope,
                    y.slope,
                    x.r_squared,
                    y.r_squared
                );
                lr_checked += 1;
            }
            other => panic!("LinReg warm-up parity: {other:?}"),
        }
    }
    assert!(lr_checked > 0, "LinReg never warmed");

    // --- HMA (derived from WMA) ---
    let h_period = 16usize;
    let mut h_b = HmaBefore::new(h_period);
    let mut h_a = HmaState::new(h_period).expect("hma");
    let mut h_checked = 0usize;
    for &c in closes {
        let b = h_b.push(c);
        let a = h_a.push(c).expect("hma push");
        assert!(
            approx_opt(b, a, 1e-8),
            "HMA parity fail: before={b:?} after={a:?}"
        );
        if b.is_some() {
            h_checked += 1;
        }
    }
    assert!(h_checked > 0, "HMA never warmed");

    // --- Donchian ---
    let (high, low, _) = synthetic_hlc(closes.len());
    let mut dc_b = DonchianBefore::new(period);
    let mut dc_a = DonchianState::new(DonchianParams::new(period)).expect("donchian");
    let mut dc_checked = 0usize;
    for i in 0..high.len() {
        let b = dc_b.push(high[i], low[i]);
        let a = dc_a.push(high[i], low[i]).expect("dc push");
        match (b, a) {
            (None, None) => {}
            (Some(x), Some(y)) => {
                assert!(
                    max_abs_diff(x.upper, y.upper) < 1e-12
                        && max_abs_diff(x.lower, y.lower) < 1e-12
                        && max_abs_diff(x.middle, y.middle) < 1e-12,
                    "Donchian parity fail"
                );
                dc_checked += 1;
            }
            other => panic!("Donchian warm-up: {other:?}"),
        }
    }
    assert!(dc_checked > 0, "Donchian never warmed");

    // --- Stoch (fast) ---
    let (h2, l2, c2) = synthetic_hlc(closes.len());
    let k_period = period;
    let d_period = 3usize;
    let st_params = StochasticParams::fast(k_period, d_period);
    let mut st_b = StochBefore::new(k_period, d_period);
    let mut st_a = StochState::new(st_params).expect("stoch");
    let mut st_checked = 0usize;
    for i in 0..c2.len() {
        let b = st_b.push(h2[i], l2[i], c2[i]);
        let a = st_a.push(h2[i], l2[i], c2[i]).expect("stoch push");
        match (b, a) {
            (None, None) => {}
            (Some((bk, bd)), Some((ak, ad))) => {
                assert!(
                    max_abs_diff(bk, ak) < 1e-9 && max_abs_diff(bd, ad) < 1e-9,
                    "Stoch parity: before=({bk},{bd}) after=({ak},{ad})"
                );
                st_checked += 1;
            }
            other => panic!("Stoch warm-up parity: {other:?}"),
        }
    }
    assert!(st_checked > 0, "Stoch never warmed");

    eprintln!(
        "algo_hot_windows parity OK (n={}, period={}): wma={} bb={} lr={} hma={} donchian={} stoch={} bb_err≈{:.2e}",
        closes.len(),
        period,
        wma_checked,
        bb_checked,
        lr_checked,
        h_checked,
        dc_checked,
        st_checked,
        bb_max
    );
}

// ---------------------------------------------------------------------------
// Streaming drivers (what Criterion times)
// ---------------------------------------------------------------------------

fn run_wma_before(closes: &[f64], period: usize) -> Option<f64> {
    let mut s = WmaBefore::new(period);
    let mut last = None;
    for &c in closes {
        last = s.push(c);
    }
    last
}

fn run_wma_after(closes: &[f64], period: usize) -> Option<f64> {
    let mut s = WmaState::new(period).expect("wma");
    let mut last = None;
    for &c in closes {
        last = s.push(c).expect("push");
    }
    last
}

fn run_bb_before(closes: &[f64], params: BollingerParams) -> Option<f64> {
    let mut s = BollingerBefore::new(params);
    let mut last = None;
    for &c in closes {
        if let Some(o) = s.push(c) {
            last = Some(o.middle);
        }
    }
    last
}

fn run_bb_after(closes: &[f64], params: BollingerParams) -> Option<f64> {
    let mut s = BollingerState::new(params).expect("bb");
    let mut last = None;
    for &c in closes {
        if let Some(o) = s.push(c).expect("push") {
            last = Some(o.middle);
        }
    }
    last
}

fn run_lr_before(closes: &[f64], period: usize) -> Option<f64> {
    let mut s = LinRegBefore::new(period);
    let mut last = None;
    for &c in closes {
        if let Some(o) = s.push(c) {
            last = Some(o.slope);
        }
    }
    last
}

fn run_lr_after(closes: &[f64], period: usize) -> Option<f64> {
    let mut s = LinRegState::new(LinRegParams::new(period)).expect("lr");
    let mut last = None;
    for &c in closes {
        if let Some(o) = s.push(c).expect("push") {
            last = Some(o.slope);
        }
    }
    last
}

fn run_hma_before(closes: &[f64], period: usize) -> Option<f64> {
    let mut s = HmaBefore::new(period);
    let mut last = None;
    for &c in closes {
        last = s.push(c);
    }
    last
}

fn run_hma_after(closes: &[f64], period: usize) -> Option<f64> {
    let mut s = HmaState::new(period).expect("hma");
    let mut last = None;
    for &c in closes {
        last = s.push(c).expect("push");
    }
    last
}

fn run_donchian_before(high: &[f64], low: &[f64], period: usize) -> Option<f64> {
    let mut s = DonchianBefore::new(period);
    let mut last = None;
    for i in 0..high.len() {
        if let Some(o) = s.push(high[i], low[i]) {
            last = Some(o.upper);
        }
    }
    last
}

fn run_donchian_after(high: &[f64], low: &[f64], period: usize) -> Option<f64> {
    let mut s = DonchianState::new(DonchianParams::new(period)).expect("dc");
    let mut last = None;
    for i in 0..high.len() {
        if let Some(o) = s.push(high[i], low[i]).expect("push") {
            last = Some(o.upper);
        }
    }
    last
}

fn run_stoch_before(high: &[f64], low: &[f64], close: &[f64], k: usize, d: usize) -> Option<f64> {
    let mut s = StochBefore::new(k, d);
    let mut last = None;
    for i in 0..close.len() {
        if let Some((kv, _)) = s.push(high[i], low[i], close[i]) {
            last = Some(kv);
        }
    }
    last
}

fn run_stoch_after(high: &[f64], low: &[f64], close: &[f64], k: usize, d: usize) -> Option<f64> {
    let mut s = StochState::new(StochasticParams::fast(k, d)).expect("stoch");
    let mut last = None;
    for i in 0..close.len() {
        if let Some((kv, _)) = s.push(high[i], low[i], close[i]).expect("push") {
            last = Some(kv);
        }
    }
    last
}

// ---------------------------------------------------------------------------
// Criterion groups
// ---------------------------------------------------------------------------

/// (n_bars, window period) — larger windows make O(window) vs O(1) obvious.
const CASES: &[(usize, usize)] = &[(10_000, 20), (10_000, 100), (50_000, 20), (50_000, 200)];

fn suite_f_algo_hot_windows(c: &mut Criterion) {
    // Parity gate: same fixture style, shorter path for assert cost.
    assert_parity(&synthetic_closes(2_500));

    let bb_params = BollingerParams::standard();

    // ----- WMA -----
    {
        let mut g = c.benchmark_group("algo_wma");
        g.measurement_time(Duration::from_secs(3));
        g.warm_up_time(Duration::from_secs(1));
        for &(n, period) in CASES {
            let closes = synthetic_closes(n);
            g.throughput(Throughput::Elements(n as u64));
            let id = format!("n{n}_p{period}");
            g.bench_with_input(
                BenchmarkId::new("before_Owindow", &id),
                &closes,
                |b, closes| {
                    b.iter(|| black_box(run_wma_before(black_box(closes), black_box(period))))
                },
            );
            g.bench_with_input(BenchmarkId::new("after_O1", &id), &closes, |b, closes| {
                b.iter(|| black_box(run_wma_after(black_box(closes), black_box(period))))
            });
        }
        g.finish();
    }

    // ----- Bollinger -----
    {
        let mut g = c.benchmark_group("algo_bollinger");
        g.measurement_time(Duration::from_secs(3));
        g.warm_up_time(Duration::from_secs(1));
        // BB period fixed at params.period (20); vary n. Extra case with custom large period.
        let bb_cases: &[(usize, BollingerParams)] = &[
            (
                10_000,
                BollingerParams {
                    period: 20,
                    num_std: 2.0,
                    stdev: StdevKind::Sample,
                },
            ),
            (
                10_000,
                BollingerParams {
                    period: 100,
                    num_std: 2.0,
                    stdev: StdevKind::Sample,
                },
            ),
            (
                50_000,
                BollingerParams {
                    period: 20,
                    num_std: 2.0,
                    stdev: StdevKind::Sample,
                },
            ),
            (
                50_000,
                BollingerParams {
                    period: 200,
                    num_std: 2.0,
                    stdev: StdevKind::Sample,
                },
            ),
        ];
        for &(n, params) in bb_cases {
            let closes = synthetic_closes(n);
            g.throughput(Throughput::Elements(n as u64));
            let id = format!("n{n}_p{}", params.period);
            g.bench_with_input(
                BenchmarkId::new("before_Owindow", &id),
                &closes,
                |b, closes| {
                    b.iter(|| black_box(run_bb_before(black_box(closes), black_box(params))))
                },
            );
            g.bench_with_input(BenchmarkId::new("after_O1", &id), &closes, |b, closes| {
                b.iter(|| black_box(run_bb_after(black_box(closes), black_box(params))))
            });
        }
        let _ = bb_params; // kept for doc clarity
        g.finish();
    }

    // ----- LinReg -----
    {
        let mut g = c.benchmark_group("algo_linreg");
        g.measurement_time(Duration::from_secs(3));
        g.warm_up_time(Duration::from_secs(1));
        for &(n, period) in CASES {
            let closes = synthetic_closes(n);
            g.throughput(Throughput::Elements(n as u64));
            let id = format!("n{n}_p{period}");
            g.bench_with_input(
                BenchmarkId::new("before_Owindow", &id),
                &closes,
                |b, closes| {
                    b.iter(|| black_box(run_lr_before(black_box(closes), black_box(period))))
                },
            );
            g.bench_with_input(BenchmarkId::new("after_O1", &id), &closes, |b, closes| {
                b.iter(|| black_box(run_lr_after(black_box(closes), black_box(period))))
            });
        }
        g.finish();
    }

    // ----- HMA (benefits from WMA O(1)) -----
    {
        let mut g = c.benchmark_group("algo_hma");
        g.measurement_time(Duration::from_secs(3));
        g.warm_up_time(Duration::from_secs(1));
        let hma_cases: &[(usize, usize)] =
            &[(10_000, 16), (10_000, 64), (50_000, 16), (50_000, 100)];
        for &(n, period) in hma_cases {
            let closes = synthetic_closes(n);
            g.throughput(Throughput::Elements(n as u64));
            let id = format!("n{n}_p{period}");
            g.bench_with_input(
                BenchmarkId::new("before_Owindow", &id),
                &closes,
                |b, closes| {
                    b.iter(|| black_box(run_hma_before(black_box(closes), black_box(period))))
                },
            );
            g.bench_with_input(BenchmarkId::new("after_O1", &id), &closes, |b, closes| {
                b.iter(|| black_box(run_hma_after(black_box(closes), black_box(period))))
            });
        }
        g.finish();
    }

    // ----- Donchian (sliding max/min) -----
    {
        let mut g = c.benchmark_group("algo_donchian");
        g.measurement_time(Duration::from_secs(3));
        g.warm_up_time(Duration::from_secs(1));
        for &(n, period) in CASES {
            let (high, low, _) = synthetic_hlc(n);
            g.throughput(Throughput::Elements(n as u64));
            let id = format!("n{n}_p{period}");
            g.bench_function(BenchmarkId::new("before_Owindow", &id), |b| {
                b.iter(|| {
                    black_box(run_donchian_before(
                        black_box(&high),
                        black_box(&low),
                        black_box(period),
                    ))
                })
            });
            g.bench_function(BenchmarkId::new("after_O1", &id), |b| {
                b.iter(|| {
                    black_box(run_donchian_after(
                        black_box(&high),
                        black_box(&low),
                        black_box(period),
                    ))
                })
            });
        }
        g.finish();
    }

    // ----- Stochastic fast (sliding HH/LL) -----
    {
        let mut g = c.benchmark_group("algo_stoch");
        g.measurement_time(Duration::from_secs(3));
        g.warm_up_time(Duration::from_secs(1));
        let stoch_cases: &[(usize, usize, usize)] = &[
            (10_000, 14, 3),
            (10_000, 100, 3),
            (50_000, 14, 3),
            (50_000, 200, 3),
        ];
        for &(n, k_period, d_period) in stoch_cases {
            let (high, low, close) = synthetic_hlc(n);
            g.throughput(Throughput::Elements(n as u64));
            let id = format!("n{n}_k{k_period}_d{d_period}");
            g.bench_function(BenchmarkId::new("before_Owindow", &id), |b| {
                b.iter(|| {
                    black_box(run_stoch_before(
                        black_box(&high),
                        black_box(&low),
                        black_box(&close),
                        black_box(k_period),
                        black_box(d_period),
                    ))
                })
            });
            g.bench_function(BenchmarkId::new("after_O1", &id), |b| {
                b.iter(|| {
                    black_box(run_stoch_after(
                        black_box(&high),
                        black_box(&low),
                        black_box(&close),
                        black_box(k_period),
                        black_box(d_period),
                    ))
                })
            });
        }
        g.finish();
    }
}

criterion_group!(benches, suite_f_algo_hot_windows);
criterion_main!(benches);
