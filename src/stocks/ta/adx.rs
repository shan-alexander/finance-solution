//! # Average Directional Index (ADX) / +DI / −DI / DX
//!
//! Wilder directional movement system (period \(N\), classic **14**):
//!
//! ```text
//! +DM = up-move if up > down and up > 0 else 0
//! −DM = down-move if down > up and down > 0 else 0
//! TR  = true range
//! Smooth TR, +DM, −DM with Wilder (seed = SMA of first N, then Wilder)
//! +DI = 100 * smooth(+DM) / smooth(TR)
//! −DI = 100 * smooth(−DM) / smooth(TR)
//! DX  = 100 * |+DI − −DI| / (+DI + −DI)
//! ADX = Wilder smooth of DX (first ADX = SMA of first N DX values)
//! ```
//!
//! Warm-up: first bar has no prior close (TR = H−L; DM from first change needs bar 1).  
//! First DI/DX at index `N−1` after N TR/DM samples; first ADX after N DX values
//! (index roughly `2N−2` on a continuous path).
//!
//! ---
//!
//! ## Trading perspective
//!
//! | Signal | Habit (classic, not a rule) |
//! |--------|-----------------------------|
//! | ADX rising / \> ~25 | Trend strength — trend systems “allowed” |
//! | ADX low / falling | Range — oscillators / mean-reversion more natural |
//! | +DI \> −DI | Bullish directional bias |
//! | −DI \> +DI | Bearish directional bias |
//! | DI cross | Direction change screen (filter with ADX level) |
//!
//! **ADX is not direction** — only strength. Always read **+DI / −DI** (or price structure)
//! for side.
//!
//! ## vs ATR / Supertrend / MACD
//!
//! | | ADX/DI | ATR | Supertrend | MACD |
//! |--|--------|-----|------------|------|
//! | Measures | Trend *strength* + DI direction | Volatility size | Trail stop / side | Momentum of closes |
//! | Good at | Regime filter | Stops / size | In/out of trend | Timing / hist flips |
//!
//! ## Pairs well with
//!
//! - **Supertrend / SAR / Donchian** — take breakouts only if ADX confirms trend.
//! - **RSI / WillR / CCI** — fade extremes only if ADX is weak (range regime).
//! - **Moving averages** — DI side + MA slope agreement.
//!
//! ---
//!
//! ## Engineering
//!
//! [`AdxParams`] → [`adx`] / [`AdxState`] → [`adx_solution`].  
//! Batch uses [`AdxState`] end-to-end. After seeds, each push is **O(1)**.  
//! Smoothing uses the same **average-seed then Wilder** style as this crate’s ATR
//! (SMA of first N samples, then \((prev·(N−1)+x)/N\)).
//!
//! ## Word problem
//!
//! > Can ADX be defined on a series shorter than \(2N−1\) bars?
//!
//! Usually **no** for a full ADX value (need N DM/TR seeds then N DX for ADX seed).
//!
//! ```
//! use finance_solution::stocks::ta::{adx, AdxParams};
//! let n = 40usize;
//! let h: Vec<_> = (0..n).map(|i| 101.0 + i as f64 * 0.2).collect();
//! let l: Vec<_> = (0..n).map(|i| 99.0 + i as f64 * 0.2).collect();
//! let c: Vec<_> = (0..n).map(|i| 100.0 + i as f64 * 0.2).collect();
//! let s = adx(&h, &l, &c, AdxParams::period_14()).unwrap();
//! assert!(s.adx.iter().any(|x| x.is_some()));
//! assert!(s.plus_di.iter().any(|x| x.is_some()));
//! ```

use crate::stocks::ta::common::{opt_cell, require_hlc, true_range};
use crate::util::error::{require_finite, FinanceError, FinanceResult};
use crate::util::primitives::PeriodLength;
use crate::{columns_with_strings, print_table_locale_opt};

/// ADX / DI Wilder period.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AdxParams {
    pub period: usize,
}

impl AdxParams {
    pub const fn new(period: usize) -> Self {
        Self { period }
    }

    pub const fn period_14() -> Self {
        Self { period: 14 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ValidatedAdx {
    params: AdxParams,
}

impl ValidatedAdx {
    pub fn new(params: AdxParams) -> FinanceResult<Self> {
        PeriodLength::new(params.period)?;
        Ok(Self { params })
    }

    pub fn params(self) -> AdxParams {
        self.params
    }

    pub fn compute(self, high: &[f64], low: &[f64], close: &[f64]) -> FinanceResult<AdxSeries> {
        adx_validated(high, low, close, self)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AdxSeries {
    pub plus_di: Vec<Option<f64>>,
    pub minus_di: Vec<Option<f64>>,
    pub dx: Vec<Option<f64>>,
    pub adx: Vec<Option<f64>>,
    pub params: AdxParams,
}

impl AdxSeries {
    pub fn last_adx(&self) -> Option<f64> {
        self.adx.iter().rev().find_map(|x| *x)
    }
}

/// One-bar ADX pack (any field may still be warming up).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AdxBarOutput {
    pub plus_di: Option<f64>,
    pub minus_di: Option<f64>,
    pub dx: Option<f64>,
    pub adx: Option<f64>,
}

/// Incremental ADX / DI / DX.
#[derive(Clone, Debug)]
pub struct AdxState {
    params: AdxParams,
    prev_high: Option<f64>,
    prev_low: Option<f64>,
    prev_close: Option<f64>,
    /// Seed buffers until length == period.
    seed_tr: Vec<f64>,
    seed_pdm: Vec<f64>,
    seed_mdm: Vec<f64>,
    atr: Option<f64>,
    pdm: Option<f64>,
    mdm: Option<f64>,
    seed_dx: Vec<f64>,
    adx: Option<f64>,
    last: Option<AdxBarOutput>,
}

impl AdxState {
    pub fn new(params: AdxParams) -> FinanceResult<Self> {
        PeriodLength::new(params.period)?;
        Ok(Self {
            params,
            prev_high: None,
            prev_low: None,
            prev_close: None,
            seed_tr: Vec::with_capacity(params.period),
            seed_pdm: Vec::with_capacity(params.period),
            seed_mdm: Vec::with_capacity(params.period),
            atr: None,
            pdm: None,
            mdm: None,
            seed_dx: Vec::with_capacity(params.period),
            adx: None,
            last: None,
        })
    }

    pub fn from_history(
        params: AdxParams,
        high: &[f64],
        low: &[f64],
        close: &[f64],
    ) -> FinanceResult<Self> {
        let mut s = Self::new(params)?;
        let _ = s.push_bars(high, low, close)?;
        Ok(s)
    }

    pub fn params(&self) -> AdxParams {
        self.params
    }

    pub fn reset(&mut self) {
        self.prev_high = None;
        self.prev_low = None;
        self.prev_close = None;
        self.seed_tr.clear();
        self.seed_pdm.clear();
        self.seed_mdm.clear();
        self.atr = None;
        self.pdm = None;
        self.mdm = None;
        self.seed_dx.clear();
        self.adx = None;
        self.last = None;
    }

    pub fn push(&mut self, high: f64, low: f64, close: f64) -> FinanceResult<AdxBarOutput> {
        require_finite("high", high)?;
        require_finite("low", low)?;
        require_finite("close", close)?;
        if high < low {
            return Err(FinanceError::InvalidCashflow {
                message: "high must be >= low for each bar",
            });
        }
        let period = self.params.period;
        let pf = period as f64;

        let (plus_dm, minus_dm) = match (self.prev_high, self.prev_low) {
            (Some(ph), Some(pl)) => {
                let up = high - ph;
                let down = pl - low;
                let pdm = if up > down && up > 0.0 { up } else { 0.0 };
                let mdm = if down > up && down > 0.0 { down } else { 0.0 };
                (pdm, mdm)
            }
            _ => (0.0, 0.0),
        };
        let tr = true_range(high, low, self.prev_close);

        let mut plus_di = None;
        let mut minus_di = None;
        let mut dx = None;
        let mut adx_out = None;

        let smoothed = if self.atr.is_none() {
            self.seed_tr.push(tr);
            self.seed_pdm.push(plus_dm);
            self.seed_mdm.push(minus_dm);
            if self.seed_tr.len() == period {
                let atr = self.seed_tr.iter().sum::<f64>() / pf;
                let pdm = self.seed_pdm.iter().sum::<f64>() / pf;
                let mdm = self.seed_mdm.iter().sum::<f64>() / pf;
                self.atr = Some(atr);
                self.pdm = Some(pdm);
                self.mdm = Some(mdm);
                Some((atr, pdm, mdm))
            } else {
                None
            }
        } else {
            let atr = (self.atr.unwrap() * (pf - 1.0) + tr) / pf;
            let pdm = (self.pdm.unwrap() * (pf - 1.0) + plus_dm) / pf;
            let mdm = (self.mdm.unwrap() * (pf - 1.0) + minus_dm) / pf;
            self.atr = Some(atr);
            self.pdm = Some(pdm);
            self.mdm = Some(mdm);
            Some((atr, pdm, mdm))
        };

        if let Some((atr, pdm, mdm)) = smoothed {
            if atr > 0.0 {
                let pdi = 100.0 * pdm / atr;
                let mdi = 100.0 * mdm / atr;
                plus_di = Some(pdi);
                minus_di = Some(mdi);
                let den = pdi + mdi;
                if den > 0.0 {
                    let d = 100.0 * (pdi - mdi).abs() / den;
                    dx = Some(d);
                    if self.adx.is_none() {
                        self.seed_dx.push(d);
                        if self.seed_dx.len() == period {
                            let a = self.seed_dx.iter().sum::<f64>() / pf;
                            self.adx = Some(a);
                            adx_out = Some(a);
                        }
                    } else {
                        let a = (self.adx.unwrap() * (pf - 1.0) + d) / pf;
                        self.adx = Some(a);
                        adx_out = Some(a);
                    }
                }
            }
        }

        self.prev_high = Some(high);
        self.prev_low = Some(low);
        self.prev_close = Some(close);

        let out = AdxBarOutput {
            plus_di,
            minus_di,
            dx,
            adx: adx_out,
        };
        self.last = Some(out);
        Ok(out)
    }

    pub fn push_bars(
        &mut self,
        high: &[f64],
        low: &[f64],
        close: &[f64],
    ) -> FinanceResult<Vec<AdxBarOutput>> {
        require_hlc(high, low, close)?;
        let mut out = Vec::with_capacity(close.len());
        for i in 0..close.len() {
            out.push(self.push(high[i], low[i], close[i])?);
        }
        Ok(out)
    }

    pub fn last(&self) -> Option<AdxBarOutput> {
        self.last
    }
}

pub fn adx(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    params: AdxParams,
) -> FinanceResult<AdxSeries> {
    ValidatedAdx::new(params)?.compute(high, low, close)
}

fn adx_validated(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    eng: ValidatedAdx,
) -> FinanceResult<AdxSeries> {
    let mut st = AdxState::new(eng.params)?;
    let bars = st.push_bars(high, low, close)?;
    let n = bars.len();
    let mut plus_di = vec![None; n];
    let mut minus_di = vec![None; n];
    let mut dx = vec![None; n];
    let mut adx = vec![None; n];
    for (i, b) in bars.into_iter().enumerate() {
        plus_di[i] = b.plus_di;
        minus_di[i] = b.minus_di;
        dx[i] = b.dx;
        adx[i] = b.adx;
    }
    Ok(AdxSeries {
        plus_di,
        minus_di,
        dx,
        adx,
        params: eng.params,
    })
}

#[derive(Clone, Debug)]
pub struct AdxSolution {
    series: AdxSeries,
    close: Vec<f64>,
    formula: String,
}

impl AdxSolution {
    pub fn series(&self) -> &AdxSeries {
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
            ("close", "f", true),
            ("plus_di", "f", true),
            ("minus_di", "f", true),
            ("dx", "f", true),
            ("adx", "f", true),
        ]);
        let data = self
            .close
            .iter()
            .enumerate()
            .map(|(i, c)| {
                vec![
                    i.to_string(),
                    c.to_string(),
                    opt_cell(self.series.plus_di[i]),
                    opt_cell(self.series.minus_di[i]),
                    opt_cell(self.series.dx[i]),
                    opt_cell(self.series.adx[i]),
                ]
            })
            .collect();
        print_table_locale_opt(&columns, data, locale, precision);
    }
}

/// # Examples
/// ```
/// use finance_solution::stocks::ta::{adx_solution, AdxParams};
/// let n = 50usize;
/// let h: Vec<_> = (0..n).map(|i| 101.0 + i as f64 * 0.1).collect();
/// let l: Vec<_> = (0..n).map(|i| 99.0 + i as f64 * 0.1).collect();
/// let c: Vec<_> = (0..n).map(|i| 100.0 + i as f64 * 0.1).collect();
/// let sol = adx_solution(&h, &l, &c, AdxParams::period_14()).unwrap();
/// assert!(sol.formula().contains("Wilder"));
/// ```
pub fn adx_solution(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    params: AdxParams,
) -> FinanceResult<AdxSolution> {
    let series = adx(high, low, close, params)?;
    Ok(AdxSolution {
        series,
        close: close.to_vec(),
        formula: format!(
            "Wilder ADX/DI period={}: +DI/-DI from DM & TR; DX; ADX=Wilder(DX)",
            params.period
        ),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rising_path(n: usize) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        let h: Vec<_> = (0..n).map(|i| 101.0 + i as f64 * 0.3).collect();
        let l: Vec<_> = (0..n).map(|i| 99.0 + i as f64 * 0.3).collect();
        let c: Vec<_> = (0..n).map(|i| 100.0 + i as f64 * 0.3).collect();
        (h, l, c)
    }

    #[test]
    fn produces_adx_on_long_path() {
        let (h, l, c) = rising_path(50);
        let s = adx(&h, &l, &c, AdxParams::period_14()).unwrap();
        assert!(s.adx.iter().filter(|x| x.is_some()).count() > 5);
        assert!(s.plus_di.iter().any(|x| x.is_some()));
        // Strong uptrend: +DI should dominate when defined
        if let (Some(p), Some(m)) = (s.plus_di[49], s.minus_di[49]) {
            assert!(p > m, "+DI={p} −DI={m}");
        }
        let a = s.last_adx().unwrap();
        assert!(a >= 0.0 && a <= 100.0, "adx={a}");
    }

    #[test]
    fn di_before_adx() {
        let (h, l, c) = rising_path(30);
        let s = adx(&h, &l, &c, AdxParams::period_14()).unwrap();
        let first_di = s.plus_di.iter().position(|x| x.is_some()).unwrap();
        let first_adx = s.adx.iter().position(|x| x.is_some()).unwrap();
        assert!(first_di < first_adx);
    }

    #[test]
    fn state_parity() {
        let (h, l, c) = rising_path(45);
        let p = AdxParams::period_14();
        let batch = adx(&h, &l, &c, p).unwrap();
        let mut st = AdxState::new(p).unwrap();
        for i in 0..c.len() {
            let o = st.push(h[i], l[i], c[i]).unwrap();
            match (o.plus_di, batch.plus_di[i]) {
                (None, None) => {}
                (Some(a), Some(b)) => assert!((a - b).abs() < 1e-9, "pdi {i}"),
                other => panic!("pdi {i}: {other:?}"),
            }
            match (o.adx, batch.adx[i]) {
                (None, None) => {}
                (Some(a), Some(b)) => assert!((a - b).abs() < 1e-9, "adx {i}"),
                other => panic!("adx {i}: {other:?}"),
            }
        }
    }

    #[test]
    fn high_lt_low_err() {
        assert!(adx(&[1.0], &[2.0], &[1.5], AdxParams::period_14()).is_err());
    }
}
