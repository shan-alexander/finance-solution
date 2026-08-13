//! Risk metrics: volatility, Sharpe, Sortino, max drawdown, beta, rolling drawdown,
//! Calmar, Ulcer index, correlation, information ratio, and trade-PnL helpers.
//!
//! ## Which ratio when?
//!
//! | Metric | Question it answers | Needs |
//! |--------|---------------------|--------|
//! | [`sharpe_ratio`] | Return per unit total vol | Return series + RF |
//! | [`sortino_ratio`] | Return per unit *downside* vol | Returns + target |
//! | [`calmar_ratio`] | CAGR per unit peak–trough pain | Price path + years |
//! | [`ulcer_index`] | How deep/persistent were drawdowns? | Prices |
//! | [`information_ratio`] | Active return per tracking error | Asset + benchmark returns |
//! | [`correlation`] | Do two series move together? | Paired series |
//! | [`win_rate`] / [`profit_factor`] / [`expectancy`] | Trade list quality | Per-trade PnL |
//!
//! **Calmar** is path-level (prices + time); **Sharpe/Sortino** are return-series. Do not mix
//! without aligning sampling frequency.
//!
//! # Error handling (v0.1+)
//!
//! All public functions return [`FinanceResult`]. Short series, zero sample vol, empty
//! prices, and length mismatches yield structured [`FinanceError`] values.
use crate::stocks::returns::{mean_return, simple_returns};
use crate::util::error::{require_finite, FinanceError, FinanceResult};

/// Sample standard deviation of a return series (population divisor `n - 1`).
///
/// # Examples
/// ```
/// use finance_solution::{volatility, FinanceError};
///
/// assert!(volatility(&[0.01, 0.02, -0.01]).is_ok());
/// match volatility(&[0.01]) {
///     Err(FinanceError::Unsolvable { message }) => assert!(message.contains("two")),
///     other => panic!("expected Unsolvable, got {other:?}"),
/// }
/// ```
pub fn volatility(returns: &[f64]) -> FinanceResult<f64> {
    if returns.len() < 2 {
        return Err(FinanceError::Unsolvable {
            message: "volatility requires at least two returns",
        });
    }
    let mean = mean_return(returns)?;
    let mut sum_sq = 0.0;
    for r in returns {
        require_finite("returns", *r)?;
        let d = r - mean;
        sum_sq += d * d;
    }
    Ok((sum_sq / (returns.len() - 1) as f64).sqrt())
}

/// Annualized volatility: `volatility(returns) * sqrt(periods_per_year)`.
///
/// # Examples
/// ```
/// use finance_solution::{volatility_annualized, FinanceError};
///
/// assert!(volatility_annualized(&[0.01, 0.02], 12.0).is_ok());
/// match volatility_annualized(&[0.01, 0.02], 0.0) {
///     Err(FinanceError::Unsolvable { message }) => assert!(message.contains("periods_per_year")),
///     other => panic!("expected Unsolvable, got {other:?}"),
/// }
/// ```
pub fn volatility_annualized(returns: &[f64], periods_per_year: f64) -> FinanceResult<f64> {
    require_finite("periods_per_year", periods_per_year)?;
    if periods_per_year <= 0.0 {
        return Err(FinanceError::Unsolvable {
            message: "periods_per_year must be positive",
        });
    }
    Ok(volatility(returns)? * periods_per_year.sqrt())
}

/// Sharpe ratio: `(mean - risk_free) / volatility` over the return series.
///
/// # Examples
/// ```
/// use finance_solution::{sharpe_ratio, FinanceError};
///
/// assert!(sharpe_ratio(&[0.02, 0.01, 0.03], 0.0).is_ok());
/// // Constant returns → zero volatility → undefined Sharpe.
/// match sharpe_ratio(&[0.01, 0.01, 0.01], 0.0) {
///     Err(FinanceError::Unsolvable { message }) => assert!(message.contains("volatility")),
///     other => panic!("expected Unsolvable, got {other:?}"),
/// }
/// ```
pub fn sharpe_ratio(returns: &[f64], risk_free_rate: f64) -> FinanceResult<f64> {
    require_finite("risk_free_rate", risk_free_rate)?;
    let vol = volatility(returns)?;
    if vol == 0.0 {
        return Err(FinanceError::Unsolvable {
            message: "sharpe_ratio undefined when volatility is zero",
        });
    }
    let mean = mean_return(returns)?;
    Ok((mean - risk_free_rate) / vol)
}

/// Sortino ratio: `(mean - target) / downside_deviation`, using returns below `target` only.
///
/// # Examples
/// ```
/// use finance_solution::{sortino_ratio, FinanceError};
///
/// assert!(sortino_ratio(&[0.02, -0.03, 0.01], 0.0).is_ok());
/// // All returns above target → no downside.
/// match sortino_ratio(&[0.01, 0.02, 0.03], 0.0) {
///     Err(FinanceError::Unsolvable { message }) => assert!(message.contains("below target")),
///     other => panic!("expected Unsolvable, got {other:?}"),
/// }
/// ```
pub fn sortino_ratio(returns: &[f64], target: f64) -> FinanceResult<f64> {
    require_finite("target", target)?;
    if returns.len() < 2 {
        return Err(FinanceError::Unsolvable {
            message: "sortino_ratio requires at least two returns",
        });
    }
    for r in returns {
        require_finite("returns", *r)?;
    }
    let mut sum_sq = 0.0;
    let mut downside_count = 0usize;
    for &r in returns {
        let shortfall = r - target;
        if shortfall < 0.0 {
            sum_sq += shortfall * shortfall;
            downside_count += 1;
        }
    }
    if downside_count == 0 {
        return Err(FinanceError::Unsolvable {
            message: "sortino_ratio undefined when no returns fall below target",
        });
    }
    // Sample-style denominator over the full series length (n - 1).
    let dd = (sum_sq / (returns.len() - 1) as f64).sqrt();
    if dd == 0.0 {
        return Err(FinanceError::Unsolvable {
            message: "sortino_ratio undefined when downside deviation is zero",
        });
    }
    let mean = mean_return(returns)?;
    Ok((mean - target) / dd)
}

/// Maximum peak-to-trough drawdown over a positive price series (most negative fraction).
///
/// # Examples
/// ```
/// use finance_solution::{max_drawdown, FinanceError};
///
/// let dd = max_drawdown(&[100.0, 120.0, 90.0]).unwrap();
/// assert!((dd - 0.25).abs() < 1e-12);
/// match max_drawdown(&[100.0]) {
///     Err(FinanceError::Unsolvable { message }) => assert!(message.contains("two")),
///     other => panic!("expected Unsolvable, got {other:?}"),
/// }
/// ```
pub fn max_drawdown(prices: &[f64]) -> FinanceResult<f64> {
    if prices.len() < 2 {
        return Err(FinanceError::Unsolvable {
            message: "max_drawdown requires at least two prices",
        });
    }
    let series = drawdown_series(prices)?;
    Ok(series.into_iter().fold(0.0_f64, f64::max))
}

/// Running drawdown series (one value per price, starting at 0).
///
/// # Examples
/// ```
/// use finance_solution::{drawdown_series, FinanceError};
///
/// assert!(drawdown_series(&[100.0, 110.0]).is_ok());
/// match drawdown_series(&[]) {
///     Err(FinanceError::Unsolvable { message }) => assert!(message.contains("one")),
///     other => panic!("expected Unsolvable, got {other:?}"),
/// }
/// ```
pub fn drawdown_series(prices: &[f64]) -> FinanceResult<Vec<f64>> {
    if prices.is_empty() {
        return Err(FinanceError::Unsolvable {
            message: "drawdown_series requires at least one price",
        });
    }
    let mut peak = prices[0];
    require_finite("prices", peak)?;
    if peak <= 0.0 {
        return Err(FinanceError::InvalidCashflow {
            message: "drawdown_series requires positive prices",
        });
    }
    let mut out = Vec::with_capacity(prices.len());
    for &p in prices {
        require_finite("prices", p)?;
        if p <= 0.0 {
            return Err(FinanceError::InvalidCashflow {
                message: "drawdown_series requires positive prices",
            });
        }
        if p > peak {
            peak = p;
        }
        out.push((peak - p) / peak);
    }
    Ok(out)
}

/// Running maximum drawdown magnitude observed up to each price index.
///
/// # Examples
/// ```
/// use finance_solution::{rolling_max_drawdown, FinanceError};
///
/// let r = rolling_max_drawdown(&[100.0, 90.0]).unwrap();
/// assert!((r[1] - 0.10).abs() < 1e-12);
/// assert!(matches!(
///     rolling_max_drawdown(&[]),
///     Err(FinanceError::Unsolvable { .. })
/// ));
/// ```
pub fn rolling_max_drawdown(prices: &[f64]) -> FinanceResult<Vec<f64>> {
    let dd = drawdown_series(prices)?;
    let mut out = Vec::with_capacity(dd.len());
    let mut running = 0.0_f64;
    for d in dd {
        running = running.max(d);
        out.push(running);
    }
    Ok(out)
}

/// OLS beta of asset returns vs market returns (same length series).
///
/// # Examples
/// ```
/// use finance_solution::{beta, FinanceError};
///
/// let market = [0.01, 0.02, -0.01, 0.03];
/// let asset = [0.02, 0.04, -0.02, 0.06]; // ~2x market
/// let b = beta(&asset, &market).unwrap();
/// assert!((b - 2.0).abs() < 1e-9);
///
/// assert!(matches!(
///     beta(&[0.1], &[0.1]),
///     Err(FinanceError::Unsolvable { .. })
/// ));
/// match beta(&[0.1, 0.2], &[0.1]) {
///     Err(FinanceError::Unsolvable { message }) => assert!(message.contains("equal length")),
///     other => panic!("expected Unsolvable, got {other:?}"),
/// }
/// ```
pub fn beta(asset_returns: &[f64], market_returns: &[f64]) -> FinanceResult<f64> {
    if asset_returns.len() != market_returns.len() {
        return Err(FinanceError::Unsolvable {
            message: "beta requires asset and market return series of equal length",
        });
    }
    if asset_returns.len() < 2 {
        return Err(FinanceError::Unsolvable {
            message: "beta requires at least two paired returns",
        });
    }
    for r in asset_returns.iter().chain(market_returns.iter()) {
        require_finite("returns", *r)?;
    }
    let mean_a = mean_return(asset_returns)?;
    let mean_m = mean_return(market_returns)?;
    let n = asset_returns.len() as f64;
    let mut cov = 0.0;
    let mut var_m = 0.0;
    for i in 0..asset_returns.len() {
        let da = asset_returns[i] - mean_a;
        let dm = market_returns[i] - mean_m;
        cov += da * dm;
        var_m += dm * dm;
    }
    cov /= n - 1.0;
    var_m /= n - 1.0;
    if var_m == 0.0 {
        return Err(FinanceError::Unsolvable {
            message: "beta undefined when market variance is zero",
        });
    }
    Ok(cov / var_m)
}

/// Volatility of simple returns computed from consecutive prices.
///
/// # Examples
/// ```
/// use finance_solution::{price_volatility, FinanceError};
///
/// assert!(price_volatility(&[100.0, 110.0, 105.0]).is_ok());
/// // Only one return → sample volatility undefined.
/// match price_volatility(&[100.0, 110.0]) {
///     Err(FinanceError::Unsolvable { message }) => assert!(message.contains("two")),
///     other => panic!("expected Unsolvable, got {other:?}"),
/// }
/// ```
pub fn price_volatility(prices: &[f64]) -> FinanceResult<f64> {
    let rets = simple_returns(prices)?;
    volatility(&rets)
}

/// CAGR from a positive price path over `years` years: `(end/start)^(1/years) − 1`.
///
/// # Examples
/// ```
/// use finance_solution::cagr_from_prices;
/// // Double in 2 years → CAGR ≈ 41.42%
/// let g = cagr_from_prices(&[100.0, 200.0], 2.0).unwrap();
/// assert!((g - (2.0_f64.sqrt() - 1.0)).abs() < 1e-12);
/// ```
pub fn cagr_from_prices(prices: &[f64], years: f64) -> FinanceResult<f64> {
    require_finite("years", years)?;
    if years <= 0.0 {
        return Err(FinanceError::Unsolvable {
            message: "years must be positive",
        });
    }
    if prices.len() < 2 {
        return Err(FinanceError::Unsolvable {
            message: "cagr_from_prices requires at least two prices",
        });
    }
    let start = prices[0];
    let end = *prices.last().unwrap();
    require_finite("prices", start)?;
    require_finite("prices", end)?;
    if start <= 0.0 || end <= 0.0 {
        return Err(FinanceError::Unsolvable {
            message: "cagr_from_prices requires strictly positive prices",
        });
    }
    Ok((end / start).powf(1.0 / years) - 1.0)
}

/// CAGR using `(prices.len()−1) / periods_per_year` as the year fraction.
pub fn cagr_from_prices_periods(prices: &[f64], periods_per_year: f64) -> FinanceResult<f64> {
    require_finite("periods_per_year", periods_per_year)?;
    if periods_per_year <= 0.0 {
        return Err(FinanceError::Unsolvable {
            message: "periods_per_year must be positive",
        });
    }
    if prices.len() < 2 {
        return Err(FinanceError::Unsolvable {
            message: "cagr_from_prices_periods requires at least two prices",
        });
    }
    let years = (prices.len() - 1) as f64 / periods_per_year;
    cagr_from_prices(prices, years)
}

/// Calmar ratio: `CAGR / |max drawdown|` on a positive price series.
///
/// Higher is better (more growth per unit historical peak-to-trough loss).  
/// Undefined when max drawdown is zero (monotone path). Compare funds only with
/// similar `years` / sampling.
///
/// # Examples
/// ```
/// use finance_solution::calmar_ratio;
/// // Mild path with some drawdown
/// let prices = [100.0, 120.0, 90.0, 150.0];
/// let c = calmar_ratio(&prices, 2.0).unwrap();
/// assert!(c.is_finite());
/// ```
pub fn calmar_ratio(prices: &[f64], years: f64) -> FinanceResult<f64> {
    let cagr = cagr_from_prices(prices, years)?;
    let dd = max_drawdown(prices)?.abs();
    if dd == 0.0 {
        return Err(FinanceError::Unsolvable {
            message: "calmar_ratio undefined when max drawdown is zero",
        });
    }
    Ok(cagr / dd)
}

/// Calmar with year fraction from sample length and `periods_per_year`.
pub fn calmar_ratio_periods(prices: &[f64], periods_per_year: f64) -> FinanceResult<f64> {
    let cagr = cagr_from_prices_periods(prices, periods_per_year)?;
    let dd = max_drawdown(prices)?.abs();
    if dd == 0.0 {
        return Err(FinanceError::Unsolvable {
            message: "calmar_ratio undefined when max drawdown is zero",
        });
    }
    Ok(cagr / dd)
}

/// Ulcer index: `sqrt(mean of squared percentage drawdowns)` (Martin).
///
/// Uses [`drawdown_series`] fractions (≤ 0), converted to percent points before squaring.
/// Higher Ulcer ⇒ more painful path; often paired with return for “Ulcer Performance”.
pub fn ulcer_index(prices: &[f64]) -> FinanceResult<f64> {
    let dd = drawdown_series(prices)?;
    if dd.is_empty() {
        return Err(FinanceError::Unsolvable {
            message: "ulcer_index requires prices",
        });
    }
    let mut sum = 0.0;
    for d in &dd {
        // store as percent points (×100) for classic Ulcer scale
        let pct = d * 100.0;
        sum += pct * pct;
    }
    Ok((sum / dd.len() as f64).sqrt())
}

/// Pearson correlation of two equal-length series.
///
/// Uses the algebraically equivalent form
/// `Σ(dx·dy) / sqrt(Σdx² · Σdy²)` (same as sample correlation; `n−1` cancels).
/// Errors if either series has zero variance.
///
/// # Examples
/// ```
/// use finance_solution::correlation;
/// let x = [1.0, 2.0, 3.0, 4.0];
/// let y = [2.0, 4.0, 6.0, 8.0];
/// assert!((correlation(&x, &y).unwrap() - 1.0).abs() < 1e-12);
/// ```
pub fn correlation(x: &[f64], y: &[f64]) -> FinanceResult<f64> {
    if x.len() != y.len() {
        return Err(FinanceError::Unsolvable {
            message: "correlation requires equal-length series",
        });
    }
    if x.len() < 2 {
        return Err(FinanceError::Unsolvable {
            message: "correlation requires at least two pairs",
        });
    }
    for (a, b) in x.iter().zip(y.iter()) {
        require_finite("x", *a)?;
        require_finite("y", *b)?;
    }
    let n = x.len() as f64;
    let mean_x = x.iter().sum::<f64>() / n;
    let mean_y = y.iter().sum::<f64>() / n;
    let mut cov = 0.0;
    let mut vx = 0.0;
    let mut vy = 0.0;
    for i in 0..x.len() {
        let dx = x[i] - mean_x;
        let dy = y[i] - mean_y;
        cov += dx * dy;
        vx += dx * dx;
        vy += dy * dy;
    }
    let denom = (vx * vy).sqrt();
    if denom == 0.0 {
        return Err(FinanceError::Unsolvable {
            message: "correlation undefined when a series has zero variance",
        });
    }
    Ok(cov / denom)
}

/// Information ratio: `mean(active) / stdev(active)` where `active[i] = asset[i] − benchmark[i]`.
///
/// Active returns and tracking error use the same sampling frequency as the inputs
/// (not annualized). For desk IR, pass already-period-matched excess returns.
pub fn information_ratio(asset_returns: &[f64], benchmark_returns: &[f64]) -> FinanceResult<f64> {
    if asset_returns.len() != benchmark_returns.len() {
        return Err(FinanceError::Unsolvable {
            message: "information_ratio requires equal-length series",
        });
    }
    if asset_returns.len() < 2 {
        return Err(FinanceError::Unsolvable {
            message: "information_ratio requires at least two returns",
        });
    }
    let mut active = Vec::with_capacity(asset_returns.len());
    for i in 0..asset_returns.len() {
        require_finite("asset_returns", asset_returns[i])?;
        require_finite("benchmark_returns", benchmark_returns[i])?;
        active.push(asset_returns[i] - benchmark_returns[i]);
    }
    let vol = volatility(&active)?;
    if vol == 0.0 {
        return Err(FinanceError::Unsolvable {
            message: "information_ratio undefined when tracking error is zero",
        });
    }
    Ok(mean_return(&active)? / vol)
}

/// Win rate over a trade P&amp;L series: `count(pnl > 0) / n` (zeros count as non-wins).
pub fn win_rate(trade_pnls: &[f64]) -> FinanceResult<f64> {
    if trade_pnls.is_empty() {
        return Err(FinanceError::Unsolvable {
            message: "win_rate requires at least one trade",
        });
    }
    let mut wins = 0usize;
    for &p in trade_pnls {
        require_finite("trade_pnls", p)?;
        if p > 0.0 {
            wins += 1;
        }
    }
    Ok(wins as f64 / trade_pnls.len() as f64)
}

/// Profit factor: `sum(positive pnl) / |sum(negative pnl)|`.
///
/// Errors if there are no losses (denominator zero) or no trades.
pub fn profit_factor(trade_pnls: &[f64]) -> FinanceResult<f64> {
    if trade_pnls.is_empty() {
        return Err(FinanceError::Unsolvable {
            message: "profit_factor requires at least one trade",
        });
    }
    let mut gp = 0.0;
    let mut gl = 0.0;
    for &p in trade_pnls {
        require_finite("trade_pnls", p)?;
        if p > 0.0 {
            gp += p;
        } else if p < 0.0 {
            gl += -p;
        }
    }
    if gl == 0.0 {
        return Err(FinanceError::Unsolvable {
            message: "profit_factor undefined when there are no losing trades",
        });
    }
    Ok(gp / gl)
}

/// Expectancy: mean trade P&amp;L (including zeros).
pub fn expectancy(trade_pnls: &[f64]) -> FinanceResult<f64> {
    if trade_pnls.is_empty() {
        return Err(FinanceError::Unsolvable {
            message: "expectancy requires at least one trade",
        });
    }
    for &p in trade_pnls {
        require_finite("trade_pnls", p)?;
    }
    Ok(trade_pnls.iter().sum::<f64>() / trade_pnls.len() as f64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_volatility_constant_zero() {
        let returns = [0.01, 0.01, 0.01, 0.01];
        assert_approx_equal!(volatility(&returns).unwrap(), 0.0);
    }

    #[test]
    fn test_max_drawdown() {
        let prices = [100.0, 120.0, 90.0, 95.0];
        assert_approx_equal!(max_drawdown(&prices).unwrap(), 0.25);
    }

    #[test]
    fn test_sharpe() {
        let returns = [0.02, 0.01, 0.03, -0.01, 0.02];
        let s = sharpe_ratio(&returns, 0.0).unwrap();
        assert!(s.is_finite() && s > 0.0);
    }

    #[test]
    fn test_sortino_has_downside() {
        let returns = [0.02, -0.03, 0.01, -0.01, 0.02];
        let s = sortino_ratio(&returns, 0.0).unwrap();
        assert!(s.is_finite());
    }

    #[test]
    fn test_sortino_no_downside_errs() {
        assert!(sortino_ratio(&[0.01, 0.02, 0.03], 0.0).is_err());
    }

    #[test]
    fn test_beta_double() {
        let market = [0.01, 0.02, -0.01, 0.03];
        let asset: Vec<f64> = market.iter().map(|r| 2.0 * r).collect();
        assert!((beta(&asset, &market).unwrap() - 2.0).abs() < 1e-9);
    }

    #[test]
    fn test_rolling_max_drawdown() {
        let prices = [100.0, 120.0, 90.0, 95.0, 130.0];
        let r = rolling_max_drawdown(&prices).unwrap();
        assert_eq!(r.len(), 5);
        assert_approx_equal!(r[2], 0.25);
        assert_approx_equal!(r[4], 0.25);
    }

    #[test]
    fn test_drawdown_series() {
        let prices = [100.0, 120.0, 90.0];
        let d = drawdown_series(&prices).unwrap();
        assert_approx_equal!(d[0], 0.0);
        assert_approx_equal!(d[1], 0.0);
        assert_approx_equal!(d[2], 0.25);
    }

    #[test]
    fn test_cagr_and_calmar() {
        let prices = [100.0, 200.0];
        let g = cagr_from_prices(&prices, 2.0).unwrap();
        assert!((g - (2.0_f64.sqrt() - 1.0)).abs() < 1e-12);
        // With drawdown path
        let p = [100.0, 150.0, 80.0, 200.0];
        let c = calmar_ratio(&p, 3.0).unwrap();
        assert!(c.is_finite() && c > 0.0);
        assert!(calmar_ratio(&[100.0, 110.0, 120.0], 1.0).is_err()); // no DD
    }

    #[test]
    fn test_ulcer_and_correlation() {
        let prices = [100.0, 120.0, 90.0, 100.0];
        let u = ulcer_index(&prices).unwrap();
        assert!(u > 0.0);
        let x = [1.0, 2.0, 3.0, 4.0];
        let y = [2.0, 4.0, 6.0, 8.0];
        assert!((correlation(&x, &y).unwrap() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_information_ratio_and_trades() {
        let a = [0.02, 0.01, 0.03, -0.01];
        let b = [0.01, 0.01, 0.01, 0.01];
        let ir = information_ratio(&a, &b).unwrap();
        assert!(ir.is_finite());
        let pnls = [10.0, -5.0, 20.0, -2.0, 0.0];
        assert!((win_rate(&pnls).unwrap() - 2.0 / 5.0).abs() < 1e-12);
        assert!((profit_factor(&pnls).unwrap() - 30.0 / 7.0).abs() < 1e-12);
        assert!((expectancy(&pnls).unwrap() - 23.0 / 5.0).abs() < 1e-12);
    }
}
