//! Ordered **price-path** analytics (equities or any positive price series).
//!
//! # Layers
//!
//! | Layer | API | Purpose |
//! |-------|-----|---------|
//! | Scalars | [`simple_return`], [`volatility`], [`beta`], … | One-off metrics |
//! | Solution | [`price_path_solution`] | Summary stats + formulas for a full path |
//! | Series | [`PricePathSolution::series`] | Period detail (return, wealth, drawdown) |
//! | Tables | [`PricePathSeries::print_table`] | Terminal / copy-paste output |
//! | **TA** | [`ta`] — SMA/EMA/Stoch/MACD/BB/KC/VWAP/RVOL + `*State` | Batch series + incremental push |
//!
//! # Error handling (v0.1+)
//!
//! Public scalars and [`price_path_solution`] return [`crate::FinanceResult`].
//! Empty series, non-positive prices, length mismatches, and zero volatility cases are
//! structured [`crate::FinanceError`] values — not panics.
//! Prefer composing with `?` when prices come from users or external data.
pub mod path;
pub mod returns;
pub mod risk;
pub mod ta;

#[doc(inline)]
pub use path::*;
#[doc(inline)]
pub use returns::*;
#[doc(inline)]
pub use risk::*;
// TA: re-export common entry points; full surface under `stocks::ta`.
#[doc(inline)]
pub use ta::{
    bollinger, bollinger_solution, ema, ema_last, keltner, keltner_solution, macd, macd_solution,
    rvol, rvol_solution, sma, sma_last, stochastics, stochastics_solution, vwap, vwap_solution,
    BollingerBarOutput, BollingerParams, BollingerSeries, BollingerSolution, BollingerState,
    EmaState, KeltnerBarOutput, KeltnerParams, KeltnerSeries, KeltnerSolution, KeltnerState,
    MacdParams, MacdSeries, MacdSolution, MacdState, RvolParams, RvolSeries, RvolSolution,
    RvolState, SmaState, StdevKind, StochBarOutput, StochState, StochasticParams, StochasticSeries,
    StochasticSolution, ValidatedBollinger, ValidatedKeltner, ValidatedMacd, ValidatedRvol,
    ValidatedStochastic, ValidatedVwap, VwapMode, VwapParams, VwapPriceSource, VwapSeries,
    VwapSolution, VwapState,
};
