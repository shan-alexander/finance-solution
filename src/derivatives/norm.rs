//! Standard normal PDF and CDF (no external special-function crate).

/// φ(x) = (1/√(2π)) exp(−x²/2)
#[inline]
pub fn norm_pdf(x: f64) -> f64 {
    const INV_SQRT_2PI: f64 = 0.398_942_280_401_432_7;
    INV_SQRT_2PI * (-0.5 * x * x).exp()
}

/// Φ(x) — standard normal CDF (Abramowitz & Stegun 26.2.17 style).
#[inline]
pub fn norm_cdf(x: f64) -> f64 {
    if x.is_nan() {
        return f64::NAN;
    }
    if x > 8.0 {
        return 1.0;
    }
    if x < -8.0 {
        return 0.0;
    }
    // A&S 26.2.17
    const B0: f64 = 0.231_641_9;
    const B1: f64 = 0.319_381_530;
    const B2: f64 = -0.356_563_782;
    const B3: f64 = 1.781_477_937;
    const B4: f64 = -1.821_255_978;
    const B5: f64 = 1.330_274_429;

    let t = 1.0 / (1.0 + B0 * x.abs());
    let poly = ((((B5 * t + B4) * t + B3) * t + B2) * t + B1) * t;
    let pdf = norm_pdf(x.abs());
    let tail = pdf * poly;
    if x >= 0.0 {
        1.0 - tail
    } else {
        tail
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pdf_at_zero() {
        let p = norm_pdf(0.0);
        assert!((p - 0.398_942_280_4).abs() < 1e-9);
    }

    #[test]
    fn cdf_symmetry() {
        assert!((norm_cdf(0.0) - 0.5).abs() < 1e-7);
        assert!((norm_cdf(1.0) + norm_cdf(-1.0) - 1.0).abs() < 1e-6);
        // Φ(1.96) ≈ 0.975
        assert!((norm_cdf(1.96) - 0.975).abs() < 5e-4);
    }
}
