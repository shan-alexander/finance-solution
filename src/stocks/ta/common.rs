//! Shared TA helpers (validation, warm-up cells, rolling stats).

use crate::util::error::{require_finite, FinanceError, FinanceResult};

pub(crate) fn opt_cell(v: Option<f64>) -> String {
    match v {
        Some(x) => x.to_string(),
        None => "n/a".to_string(),
    }
}

pub(crate) fn validate_series(name: &'static str, xs: &[f64]) -> FinanceResult<()> {
    if xs.is_empty() {
        return Err(FinanceError::EmptyInput { what: name });
    }
    for &x in xs {
        require_finite(name, x)?;
    }
    Ok(())
}

pub(crate) fn validate_positive_volume(volume: &[f64]) -> FinanceResult<()> {
    validate_series("volume", volume)?;
    for &v in volume {
        if v < 0.0 {
            return Err(FinanceError::InvalidCashflow {
                message: "volume must be non-negative",
            });
        }
    }
    Ok(())
}

pub(crate) fn require_same_len(a: &[f64], b: &[f64], context: &'static str) -> FinanceResult<()> {
    if a.len() != b.len() {
        return Err(FinanceError::LengthMismatch {
            left: a.len(),
            right: b.len(),
            context,
        });
    }
    Ok(())
}

pub(crate) fn require_hlc(high: &[f64], low: &[f64], close: &[f64]) -> FinanceResult<()> {
    validate_series("high", high)?;
    validate_series("low", low)?;
    validate_series("close", close)?;
    require_same_len(high, low, "high/low")?;
    require_same_len(high, close, "high/close")?;
    for i in 0..high.len() {
        if high[i] < low[i] {
            return Err(FinanceError::InvalidCashflow {
                message: "high must be >= low for each bar",
            });
        }
    }
    Ok(())
}

/// Which denominator to use for window standard deviation (Bollinger, etc.).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum StdevKind {
    /// Unbiased sample stdev: divide by `n - 1`. **Default for Bollinger** (common in finance).
    #[default]
    Sample,
    /// Population stdev: divide by `n`.
    Population,
}

/// Standard deviation of a full window. `None` if the kind cannot be computed
/// (`Sample` needs `n ≥ 2`; `Population` needs `n ≥ 1`).
pub(crate) fn window_stdev(window: &[f64], kind: StdevKind) -> Option<f64> {
    let n = window.len();
    match kind {
        StdevKind::Sample if n < 2 => return None,
        StdevKind::Population if n < 1 => return None,
        _ => {}
    }
    let mean = window.iter().sum::<f64>() / n as f64;
    let denom = match kind {
        StdevKind::Sample => n as f64 - 1.0,
        StdevKind::Population => n as f64,
    };
    let var = window.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / denom;
    Some(var.sqrt())
}

/// Sample standard deviation (`n - 1`). `None` if `n < 2`.
pub(crate) fn sample_stdev(window: &[f64]) -> Option<f64> {
    window_stdev(window, StdevKind::Sample)
}

/// True range for bar `i` (needs previous close when `i > 0`).
pub(crate) fn true_range(high: f64, low: f64, prev_close: Option<f64>) -> f64 {
    let hl = high - low;
    match prev_close {
        None => hl,
        Some(pc) => hl.max((high - pc).abs()).max((low - pc).abs()),
    }
}
