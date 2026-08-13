//! # Rolling least-squares linear regression
//!
//! Fit \(y = \text{intercept} + \text{slope}\cdot x\) over the last **N** samples,
//! where \(x = 0,1,\ldots,N-1\) (oldest → newest in the window).
//!
//! **Versatile:** pass **any** `f64` series — closes, highs, lows, typical price,
//! volume, custom transforms. The crate does not force OHLCV; your engine picks
//! the slice (e.g. highs for resistance slope, closes for trend slope).
//!
//! ---
//!
//! ## Trading perspective
//!
//! | Output | Habit |
//! |--------|-------|
//! | **slope** | Direction & steepness per bar (price units / bar) |
//! | **angle_degrees** | `atan(slope)` in degrees — comparable trend “angle” when scale is fixed |
//! | **r_squared** | How linear the window is (1 = perfect line) |
//! | **intercept** | Fitted value at the oldest bar of the window |
//!
//! ---
//!
//! ## Engineering
//!
//! | Layer | API |
//! |-------|-----|
//! | Params | [`LinRegParams`] |
//! | Batch | [`linear_regression`] (via [`LinRegState`]) |
//! | Live | [`LinRegState::push`] / `push_bars` / `from_history` |
//! | Teaching | [`linear_regression_solution`] |
//!
//! Each push when warm is **O(1)** (running `Σy`, `Σxy`, `Σy²`; fixed `x = 0..N-1`).
//! First full window seeds those sums in O(N).
//!
//! ## Word problem
//!
//! > Closes 1,2,3,4,5 over five bars. Slope of the 5-bar regression?
//!
//! Expect: slope **1.0** (perfect line).
//!
//! ```
//! use finance_solution::stocks::ta::{linear_regression, LinRegParams};
//! let y = [1.0, 2.0, 3.0, 4.0, 5.0];
//! let s = linear_regression(&y, LinRegParams::new(5)).unwrap();
//! assert!((s[4].unwrap().slope - 1.0).abs() < 1e-12);
//! assert!((s[4].unwrap().r_squared - 1.0).abs() < 1e-12);
//! ```

use crate::stocks::ta::common::validate_series;
use crate::stocks::ta::ring::RingF64;
use crate::util::error::{require_finite, FinanceResult};
use crate::util::primitives::PeriodLength;
use crate::{columns_with_strings, print_table_locale_opt};

/// Rolling regression window length.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LinRegParams {
    pub period: usize,
}

impl LinRegParams {
    pub const fn new(period: usize) -> Self {
        Self { period }
    }

    /// Common short trend window.
    pub const fn period_20() -> Self {
        Self { period: 20 }
    }

    pub const fn period_50() -> Self {
        Self { period: 50 }
    }
}

/// One fitted window.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LinRegBar {
    /// \(\Delta y / \Delta x\) with \(x\) in bar-index units (0 = oldest in window).
    pub slope: f64,
    /// \(y\) at \(x = 0\) (oldest bar of the window).
    pub intercept: f64,
    /// Coefficient of determination in \([0, 1]\) (clamped); 1 = perfect fit.
    pub r_squared: f64,
    /// `atan(slope)` radians.
    pub angle_radians: f64,
    /// `atan(slope)` degrees.
    pub angle_degrees: f64,
}

/// Validated pack.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ValidatedLinReg {
    params: LinRegParams,
}

impl ValidatedLinReg {
    pub fn new(params: LinRegParams) -> FinanceResult<Self> {
        PeriodLength::new(params.period)?;
        if params.period < 2 {
            return Err(crate::util::error::FinanceError::Unsolvable {
                message: "linear regression period must be >= 2",
            });
        }
        Ok(Self { params })
    }

    pub fn params(self) -> LinRegParams {
        self.params
    }

    pub fn compute(self, series: &[f64]) -> FinanceResult<Vec<Option<LinRegBar>>> {
        linear_regression_validated(series, self)
    }
}

/// Incremental rolling regression on a caller-chosen series.
///
/// After warm-up each [`push`](Self::push) is **O(1)** (running `Σy`, `Σxy`, `Σy²`;
/// `Σx` / `Σx²` are constant for fixed window length with `x = 0..N-1`).
#[derive(Clone, Debug)]
pub struct LinRegState {
    params: LinRegParams,
    ring: RingF64,
    /// Set once the window is full.
    sums: Option<LinRegSums>,
    last: Option<LinRegBar>,
}

#[derive(Clone, Copy, Debug)]
struct LinRegSums {
    sum_y: f64,
    sum_xy: f64,
    sum_y2: f64,
}

impl LinRegState {
    pub fn new(params: LinRegParams) -> FinanceResult<Self> {
        let _ = ValidatedLinReg::new(params)?;
        Ok(Self {
            params,
            ring: RingF64::with_capacity(params.period),
            sums: None,
            last: None,
        })
    }

    pub fn from_history(params: LinRegParams, series: &[f64]) -> FinanceResult<Self> {
        let mut s = Self::new(params)?;
        let _ = s.push_bars(series)?;
        Ok(s)
    }

    pub fn params(&self) -> LinRegParams {
        self.params
    }

    pub fn reset(&mut self) {
        self.ring.clear();
        self.sums = None;
        self.last = None;
    }

    pub fn push(&mut self, y: f64) -> FinanceResult<Option<LinRegBar>> {
        require_finite("series", y)?;
        let n = self.params.period;
        let nf = n as f64;

        if let Some(mut s) = self.sums {
            let y_old = self.ring.oldest().unwrap();
            let sum_y_old = s.sum_y;
            // Slide with re-indexed x=0..n-1:
            // sum_xy' = sum_xy - sum_y + y_old + (n-1)*y_new
            s.sum_xy = s.sum_xy - sum_y_old + y_old + (nf - 1.0) * y;
            s.sum_y = sum_y_old - y_old + y;
            s.sum_y2 = s.sum_y2 - y_old * y_old + y * y;
            let _ = self.ring.push(y);
            self.sums = Some(s);
            let bar = fit_ols_from_sums(n, s.sum_y, s.sum_xy, s.sum_y2);
            self.last = Some(bar);
            Ok(Some(bar))
        } else {
            let _ = self.ring.push(y);
            if !self.ring.is_full() {
                self.last = None;
                return Ok(None);
            }
            let mut ordered = Vec::with_capacity(n);
            self.ring.copy_ordered(&mut ordered);
            let mut sum_y = 0.0;
            let mut sum_xy = 0.0;
            let mut sum_y2 = 0.0;
            for (i, &yi) in ordered.iter().enumerate() {
                let x = i as f64;
                sum_y += yi;
                sum_xy += x * yi;
                sum_y2 += yi * yi;
            }
            self.sums = Some(LinRegSums {
                sum_y,
                sum_xy,
                sum_y2,
            });
            let bar = fit_ols_from_sums(n, sum_y, sum_xy, sum_y2);
            self.last = Some(bar);
            Ok(Some(bar))
        }
    }

    pub fn push_bars(&mut self, series: &[f64]) -> FinanceResult<Vec<Option<LinRegBar>>> {
        let mut out = Vec::with_capacity(series.len());
        for &y in series {
            out.push(self.push(y)?);
        }
        Ok(out)
    }

    pub fn last(&self) -> Option<LinRegBar> {
        self.last
    }
}

#[derive(Clone, Debug)]
pub struct LinRegSolution {
    series: Vec<Option<LinRegBar>>,
    y: Vec<f64>,
    params: LinRegParams,
    formula: String,
}

impl LinRegSolution {
    pub fn series(&self) -> &[Option<LinRegBar>] {
        &self.series
    }
    pub fn formula(&self) -> &str {
        &self.formula
    }
    pub fn params(&self) -> LinRegParams {
        self.params
    }

    pub fn print_table(&self) {
        self.print_table_locale_opt(None, None);
    }

    pub fn print_table_locale(&self, locale: &num_format::Locale, precision: usize) {
        self.print_table_locale_opt(Some(locale), Some(precision));
    }

    fn print_table_locale_opt(
        &self,
        locale: Option<&num_format::Locale>,
        precision: Option<usize>,
    ) {
        let columns = columns_with_strings(&[
            ("period", "i", true),
            ("y", "f", true),
            ("slope", "f", true),
            ("angle_deg", "f", true),
            ("r2", "f", true),
        ]);
        let data = self
            .y
            .iter()
            .enumerate()
            .map(|(i, y)| {
                let (slope, ang, r2) = match self.series[i] {
                    Some(b) => (
                        b.slope.to_string(),
                        b.angle_degrees.to_string(),
                        b.r_squared.to_string(),
                    ),
                    None => ("n/a".to_string(), "n/a".to_string(), "n/a".to_string()),
                };
                vec![i.to_string(), y.to_string(), slope, ang, r2]
            })
            .collect();
        print_table_locale_opt(&columns, data, locale, precision);
    }
}

/// Batch rolling OLS. `series` is **your** choice of bar field (close, high, …).
pub fn linear_regression(
    series: &[f64],
    params: LinRegParams,
) -> FinanceResult<Vec<Option<LinRegBar>>> {
    ValidatedLinReg::new(params)?.compute(series)
}

pub fn linear_regression_solution(
    series: &[f64],
    params: LinRegParams,
) -> FinanceResult<LinRegSolution> {
    let s = linear_regression(series, params)?;
    Ok(LinRegSolution {
        series: s,
        y: series.to_vec(),
        params,
        formula: format!(
            "OLS over last {}: y = intercept + slope*x, x=0..N-1; angle=atan(slope)",
            params.period
        ),
    })
}

fn linear_regression_validated(
    series: &[f64],
    eng: ValidatedLinReg,
) -> FinanceResult<Vec<Option<LinRegBar>>> {
    validate_series("series", series)?;
    let mut st = LinRegState::new(eng.params)?;
    st.push_bars(series)
}

/// OLS from sufficient stats with `x = 0..n-1` (fixed for a given `n`).
fn fit_ols_from_sums(n: usize, sum_y: f64, sum_xy: f64, sum_y2: f64) -> LinRegBar {
    let nf = n as f64;
    // sum_x = 0+1+...+(n-1) = (n-1)n/2
    // sum_xx = (n-1)n(2n-1)/6
    let sum_x = (nf - 1.0) * nf / 2.0;
    let sum_xx = (nf - 1.0) * nf * (2.0 * nf - 1.0) / 6.0;
    let denom = nf * sum_xx - sum_x * sum_x;
    let slope = if denom.abs() < 1e-18 {
        0.0
    } else {
        (nf * sum_xy - sum_x * sum_y) / denom
    };
    let intercept = (sum_y - slope * sum_x) / nf;

    let mean_y = sum_y / nf;
    let ss_tot = sum_y2 - nf * mean_y * mean_y;
    // ss_res = Σ(y - a - b x)² expanded with sufficient stats
    let ss_res = sum_y2
        + nf * intercept * intercept
        + slope * slope * sum_xx
        + 2.0 * intercept * slope * sum_x
        - 2.0 * intercept * sum_y
        - 2.0 * slope * sum_xy;
    let r_squared = if ss_tot <= 1e-18 {
        1.0
    } else {
        (1.0 - ss_res / ss_tot).clamp(0.0, 1.0)
    };

    let angle_radians = slope.atan();
    LinRegBar {
        slope,
        intercept,
        r_squared,
        angle_radians,
        angle_degrees: angle_radians.to_degrees(),
    }
}

/// OLS on a full window `y[0..n)` with `x = 0..n-1` (tests / cold path).
#[cfg(test)]
fn fit_ols(y: &[f64]) -> LinRegBar {
    let mut sum_y = 0.0;
    let mut sum_xy = 0.0;
    let mut sum_y2 = 0.0;
    for (i, &yi) in y.iter().enumerate() {
        sum_y += yi;
        sum_xy += (i as f64) * yi;
        sum_y2 += yi * yi;
    }
    fit_ols_from_sums(y.len(), sum_y, sum_xy, sum_y2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perfect_line_slope_one() {
        let y = [1.0, 2.0, 3.0, 4.0, 5.0];
        let s = linear_regression(&y, LinRegParams::new(5)).unwrap();
        let b = s[4].unwrap();
        assert!((b.slope - 1.0).abs() < 1e-12);
        assert!((b.r_squared - 1.0).abs() < 1e-12);
        assert!((b.intercept - 1.0).abs() < 1e-12);
    }

    #[test]
    fn state_parity() {
        let y: Vec<f64> = (0..40).map(|i| 100.0 + i as f64 * 0.25).collect();
        let batch = linear_regression(&y, LinRegParams::period_20()).unwrap();
        let st = LinRegState::from_history(LinRegParams::period_20(), &y).unwrap();
        let b = batch.last().unwrap().unwrap();
        let s = st.last().unwrap();
        assert!((b.slope - s.slope).abs() < 1e-12);
    }

    #[test]
    fn period_one_err() {
        assert!(ValidatedLinReg::new(LinRegParams::new(1)).is_err());
    }

    #[test]
    fn flat_series_zero_slope() {
        let y = vec![42.0; 20];
        let b = linear_regression(&y, LinRegParams::new(10))
            .unwrap()
            .last()
            .unwrap()
            .unwrap();
        assert!(b.slope.abs() < 1e-12);
        assert!((b.r_squared - 1.0).abs() < 1e-12);
    }

    #[test]
    fn angle_matches_atan_slope() {
        let y = [0.0, 1.0, 2.0, 3.0, 4.0];
        let b = linear_regression(&y, LinRegParams::new(5))
            .unwrap()
            .last()
            .unwrap()
            .unwrap();
        assert!((b.angle_radians - b.slope.atan()).abs() < 1e-15);
        assert!((b.angle_degrees - b.angle_radians.to_degrees()).abs() < 1e-12);
    }

    #[test]
    fn works_on_highs_not_only_closes() {
        // Versatility: slope of highs series
        let highs = [10.0, 11.0, 12.5, 12.0, 13.0, 14.0, 15.0];
        let s = linear_regression(&highs, LinRegParams::new(5)).unwrap();
        assert!(s[6].unwrap().slope > 0.0);
    }

    #[test]
    fn empty_err() {
        assert!(linear_regression(&[], LinRegParams::new(5)).is_err());
    }
}
