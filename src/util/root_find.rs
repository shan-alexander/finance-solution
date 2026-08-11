//! Scalar root-finding helpers used by implied vol and other solvers.
//!
//! | Method | Use in this crate |
//! |--------|-------------------|
//! | Newton–Raphson | Primary IV step when vega is reliable |
//! | **Brent** | Robust bracketed fallback after Newton |
//!
//! Brent combines bisection reliability with secant / inverse-quadratic speed on
//! smooth problems. We previously used plain bisection as the fallback; Brent is
//! the standard upgrade for the same bracketed contract.

use crate::util::error::{FinanceError, FinanceResult};

/// Find a root of continuous `f` on bracket `[lo, hi]` (Brent 1973).
///
/// Requires opposite signs at the endpoints (or a zero on an endpoint).
pub fn brent_root(
    lo: f64,
    hi: f64,
    mut f: impl FnMut(f64) -> f64,
    tol: f64,
    max_iter: usize,
) -> FinanceResult<f64> {
    let mut a = lo;
    let mut b = hi;
    let mut fa = f(a);
    let mut fb = f(b);
    if !fa.is_finite() || !fb.is_finite() {
        return Err(FinanceError::Unsolvable {
            message: "Brent: non-finite function value at bracket",
        });
    }
    if fa == 0.0 {
        return Ok(a);
    }
    if fb == 0.0 {
        return Ok(b);
    }
    if fa * fb > 0.0 {
        return Err(FinanceError::Unsolvable {
            message: "Brent: bracket does not change sign",
        });
    }

    let mut c = a;
    let mut fc = fa;
    let mut d = b - a;
    let mut e = d;

    for _ in 0..max_iter {
        if fb == 0.0 {
            return Ok(b);
        }
        if fa * fb > 0.0 {
            a = c;
            fa = fc;
            d = b - a;
            e = d;
        }
        if fa.abs() < fb.abs() {
            // swap a <-> b
            c = b;
            b = a;
            a = c;
            fc = fb;
            fb = fa;
            fa = fc;
        }

        let tol1 = 2.0 * f64::EPSILON * b.abs() + 0.5 * tol;
        let xm = 0.5 * (a - b);
        if xm.abs() <= tol1 {
            return Ok(b);
        }

        if e.abs() >= tol1 && fa.abs() > fb.abs() {
            let s = fb / fa;
            let (mut p, mut q) = if (a - c).abs() <= f64::EPSILON {
                // linear interpolation (secant)
                (2.0 * xm * s, 1.0 - s)
            } else {
                // inverse quadratic
                let q0 = fa / fc;
                let r = fb / fc;
                let p = s * (2.0 * xm * q0 * (q0 - r) - (b - a) * (r - 1.0));
                let q = (q0 - 1.0) * (r - 1.0) * (s - 1.0);
                (p, q)
            };
            if p > 0.0 {
                q = -q;
            } else {
                p = -p;
            }
            let min1 = 3.0 * xm * q.abs() - (tol1 * q).abs();
            let min2 = (e * q).abs();
            if 2.0 * p < min1.min(min2) {
                e = d;
                d = p / q;
            } else {
                d = xm;
                e = d;
            }
        } else {
            d = xm;
            e = d;
        }

        c = b;
        fc = fb;
        if d.abs() > tol1 {
            b += d;
        } else {
            b += xm.signum() * tol1;
            if xm == 0.0 {
                b += if a > b { tol1 } else { -tol1 };
            }
        }
        fb = f(b);
        if !fb.is_finite() {
            return Err(FinanceError::Unsolvable {
                message: "Brent: non-finite function value",
            });
        }
    }

    Err(FinanceError::Unsolvable {
        message: "Brent: max iterations exceeded",
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sqrt_two() {
        let r = brent_root(1.0, 2.0, |x| x * x - 2.0, 1e-14, 100).unwrap();
        assert!((r - 2.0_f64.sqrt()).abs() < 1e-10);
    }

    #[test]
    fn no_sign_change_err() {
        assert!(brent_root(1.0, 2.0, |x| x + 1.0, 1e-8, 50).is_err());
    }

    #[test]
    fn endpoint_root() {
        let r = brent_root(0.0, 2.0, |x| x - 2.0, 1e-12, 50).unwrap();
        assert!((r - 2.0).abs() < 1e-12);
    }

    #[test]
    fn cubic_root() {
        // x^3 - x - 2 = 0, root ≈ 1.521
        let r = brent_root(1.0, 2.0, |x| x * x * x - x - 2.0, 1e-12, 100).unwrap();
        assert!((r * r * r - r - 2.0).abs() < 1e-10);
    }

    #[test]
    fn sine_root() {
        let r = brent_root(3.0, 3.2, |x| x.sin(), 1e-14, 100).unwrap();
        assert!((r - std::f64::consts::PI).abs() < 1e-10);
    }
}
