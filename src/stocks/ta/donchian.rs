//! # Donchian channels
//!
//! ```text
//! upper[i]  = max(high[i-N+1 ..= i])
//! lower[i]  = min(low [i-N+1 ..= i])
//! middle[i] = (upper + lower) / 2
//! ```
//!
//! Classic breakout envelope (Turtle-style). Warm-up: first `N-1` bars are `None`.
//!
//! ---
//!
//! ## Trading perspective
//!
//! | Event | Habit |
//! |-------|-------|
//! | Close > upper | Upside breakout screen |
//! | Close < lower | Downside breakout screen |
//! | Channel width | Range expansion / compression |
//!
//! ---
//!
//! ## Engineering
//!
//! [`DonchianParams`] → [`donchian`] / [`DonchianState`] → [`donchian_solution`].  
//! Live [`DonchianState::push`] is **amortized O(1)** via monotonic sliding max/min
//! (not an O(period) scan of the window).
//!
//! ## Word problem
//!
//! > Highs 11,12,13,14 and lows 9,10,11,12 with N=3. Upper on last bar?
//!
//! Expect: **14** (max of last three highs 12,13,14).
//!
//! ```
//! use finance_solution::stocks::ta::{donchian, DonchianParams};
//! let h = [11.0, 12.0, 13.0, 14.0];
//! let l = [9.0, 10.0, 11.0, 12.0];
//! let s = donchian(&h, &l, DonchianParams::new(3)).unwrap();
//! assert!((s.upper[3].unwrap() - 14.0).abs() < 1e-12);
//! assert!((s.lower[3].unwrap() - 10.0).abs() < 1e-12); // min(lows 10,11,12)
//! ```

use crate::stocks::ta::common::{opt_cell, require_same_len, validate_series};
use crate::stocks::ta::ring::{SlidingMax, SlidingMin};
use crate::util::error::{require_finite, FinanceError, FinanceResult};
use crate::util::primitives::PeriodLength;
use crate::{columns_with_strings, print_table_locale_opt};

/// Donchian lookback.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DonchianParams {
    pub period: usize,
}

impl DonchianParams {
    pub const fn new(period: usize) -> Self {
        Self { period }
    }

    /// Common 20-bar channel.
    pub const fn period_20() -> Self {
        Self { period: 20 }
    }

    pub const fn period_55() -> Self {
        Self { period: 55 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ValidatedDonchian {
    params: DonchianParams,
}

impl ValidatedDonchian {
    pub fn new(params: DonchianParams) -> FinanceResult<Self> {
        PeriodLength::new(params.period)?;
        Ok(Self { params })
    }

    pub fn params(self) -> DonchianParams {
        self.params
    }

    pub fn compute(self, high: &[f64], low: &[f64]) -> FinanceResult<DonchianSeries> {
        donchian_validated(high, low, self)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DonchianSeries {
    pub upper: Vec<Option<f64>>,
    pub lower: Vec<Option<f64>>,
    pub middle: Vec<Option<f64>>,
    pub params: DonchianParams,
}

impl DonchianSeries {
    pub fn last(&self) -> Option<(f64, f64, f64)> {
        let n = self.upper.len();
        if n == 0 {
            return None;
        }
        Some((self.upper[n - 1]?, self.middle[n - 1]?, self.lower[n - 1]?))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DonchianBarOutput {
    pub upper: f64,
    pub middle: f64,
    pub lower: f64,
}

/// Incremental Donchian.
///
/// After warm-up each [`push`](Self::push) is **amortized O(1)** (sliding max/min deques).
#[derive(Clone, Debug)]
pub struct DonchianState {
    params: DonchianParams,
    high_max: SlidingMax,
    low_min: SlidingMin,
    last: Option<DonchianBarOutput>,
}

impl DonchianState {
    pub fn new(params: DonchianParams) -> FinanceResult<Self> {
        let _ = ValidatedDonchian::new(params)?;
        Ok(Self {
            params,
            high_max: SlidingMax::with_window(params.period),
            low_min: SlidingMin::with_window(params.period),
            last: None,
        })
    }

    pub fn from_history(params: DonchianParams, high: &[f64], low: &[f64]) -> FinanceResult<Self> {
        let mut s = Self::new(params)?;
        let _ = s.push_bars(high, low)?;
        Ok(s)
    }

    pub fn params(&self) -> DonchianParams {
        self.params
    }

    pub fn reset(&mut self) {
        self.high_max.clear();
        self.low_min.clear();
        self.last = None;
    }

    pub fn push(&mut self, high: f64, low: f64) -> FinanceResult<Option<DonchianBarOutput>> {
        require_finite("high", high)?;
        require_finite("low", low)?;
        if high < low {
            return Err(FinanceError::InvalidCashflow {
                message: "high must be >= low for each bar",
            });
        }
        let u = self.high_max.push(high).unwrap();
        let l = self.low_min.push(low).unwrap();
        if !self.high_max.is_full() {
            self.last = None;
            return Ok(None);
        }
        let out = DonchianBarOutput {
            upper: u,
            middle: 0.5 * (u + l),
            lower: l,
        };
        self.last = Some(out);
        Ok(Some(out))
    }

    pub fn push_bars(
        &mut self,
        high: &[f64],
        low: &[f64],
    ) -> FinanceResult<Vec<Option<DonchianBarOutput>>> {
        validate_series("high", high)?;
        validate_series("low", low)?;
        require_same_len(high, low, "high/low")?;
        let mut out = Vec::with_capacity(high.len());
        for i in 0..high.len() {
            out.push(self.push(high[i], low[i])?);
        }
        Ok(out)
    }

    pub fn last(&self) -> Option<DonchianBarOutput> {
        self.last
    }
}

#[derive(Clone, Debug)]
pub struct DonchianSolution {
    series: DonchianSeries,
    close: Option<Vec<f64>>,
    formula: String,
}

impl DonchianSolution {
    pub fn series(&self) -> &DonchianSeries {
        &self.series
    }
    pub fn formula(&self) -> &str {
        &self.formula
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
            ("upper", "f", true),
            ("middle", "f", true),
            ("lower", "f", true),
        ]);
        let n = self.series.upper.len();
        let data = (0..n)
            .map(|i| {
                vec![
                    i.to_string(),
                    opt_cell(self.series.upper[i]),
                    opt_cell(self.series.middle[i]),
                    opt_cell(self.series.lower[i]),
                ]
            })
            .collect();
        let _ = &self.close;
        print_table_locale_opt(&columns, data, locale, precision);
    }
}

pub fn donchian(
    high: &[f64],
    low: &[f64],
    params: DonchianParams,
) -> FinanceResult<DonchianSeries> {
    ValidatedDonchian::new(params)?.compute(high, low)
}

pub fn donchian_solution(
    high: &[f64],
    low: &[f64],
    params: DonchianParams,
) -> FinanceResult<DonchianSolution> {
    let series = donchian(high, low, params)?;
    Ok(DonchianSolution {
        series,
        close: None,
        formula: format!(
            "Donchian({}): upper=max(high,N), lower=min(low,N), mid=(u+l)/2",
            params.period
        ),
    })
}

fn donchian_validated(
    high: &[f64],
    low: &[f64],
    eng: ValidatedDonchian,
) -> FinanceResult<DonchianSeries> {
    validate_series("high", high)?;
    validate_series("low", low)?;
    require_same_len(high, low, "high/low")?;
    for i in 0..high.len() {
        if high[i] < low[i] {
            return Err(FinanceError::InvalidCashflow {
                message: "high must be >= low for each bar",
            });
        }
    }
    let mut st = DonchianState::new(eng.params)?;
    let bars = st.push_bars(high, low)?;
    let n = high.len();
    let mut upper = vec![None; n];
    let mut lower = vec![None; n];
    let mut middle = vec![None; n];
    for (i, b) in bars.into_iter().enumerate() {
        if let Some(o) = b {
            upper[i] = Some(o.upper);
            lower[i] = Some(o.lower);
            middle[i] = Some(o.middle);
        }
    }
    Ok(DonchianSeries {
        upper,
        lower,
        middle,
        params: eng.params,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_window() {
        let h = [11.0, 12.0, 13.0, 14.0];
        let l = [9.0, 10.0, 11.0, 12.0];
        let s = donchian(&h, &l, DonchianParams::new(3)).unwrap();
        assert!(s.upper[1].is_none());
        assert!((s.upper[3].unwrap() - 14.0).abs() < 1e-12);
        assert!((s.lower[3].unwrap() - 10.0).abs() < 1e-12); // min of lows 10,11,12
        assert!((s.middle[3].unwrap() - 12.0).abs() < 1e-12);
    }

    #[test]
    fn state_parity() {
        let n = 40usize;
        let h: Vec<_> = (0..n).map(|i| 100.0 + (i % 7) as f64).collect();
        let l: Vec<_> = (0..n).map(|i| 90.0 + (i % 7) as f64).collect();
        let batch = donchian(&h, &l, DonchianParams::period_20()).unwrap();
        let st = DonchianState::from_history(DonchianParams::period_20(), &h, &l).unwrap();
        let (u, m, lo) = batch.last().unwrap();
        let o = st.last().unwrap();
        assert!((u - o.upper).abs() < 1e-12);
        assert!((m - o.middle).abs() < 1e-12);
        assert!((lo - o.lower).abs() < 1e-12);
    }

    #[test]
    fn period_one_is_raw_hl() {
        let h = [5.0, 7.0];
        let l = [4.0, 6.0];
        let s = donchian(&h, &l, DonchianParams::new(1)).unwrap();
        assert_eq!(s.upper[0], Some(5.0));
        assert_eq!(s.lower[0], Some(4.0));
        assert_eq!(s.middle[0], Some(4.5));
        assert_eq!(s.upper[1], Some(7.0));
    }

    #[test]
    fn high_lt_low_err() {
        assert!(donchian(&[1.0, 0.0], &[0.0, 1.0], DonchianParams::new(2)).is_err());
    }

    #[test]
    fn reset_clears() {
        let h = [10.0; 25];
        let l = [9.0; 25];
        let mut st = DonchianState::from_history(DonchianParams::period_20(), &h, &l).unwrap();
        assert!(st.last().is_some());
        st.reset();
        assert!(st.last().is_none());
    }
}
