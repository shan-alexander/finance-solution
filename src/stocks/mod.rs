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
//! | **TA** | [`ta`] — SMA/EMA/WMA/HMA/Stoch/MACD/BB/KC/Donchian/VWAP/RVOL/RSI/ATR/LinReg + `*State` | Batch series + incremental push |
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
    atr, atr_solution, bollinger, bollinger_solution, donchian, donchian_solution, ema, ema_last,
    hma, hma_last, keltner, keltner_solution, linear_regression, linear_regression_solution, macd,
    macd_solution, rsi, rsi_solution, rvol, rvol_solution, sma, sma_last, stochastics,
    stochastics_solution, vwap, vwap_solution, wma, wma_last, AtrParams, AtrSeries, AtrSolution,
    AtrState, BollingerBarOutput, BollingerParams, BollingerSeries, BollingerSolution,
    BollingerState, DonchianBarOutput, DonchianParams, DonchianSeries, DonchianSolution,
    DonchianState, EmaState, HmaState, KeltnerBarOutput, KeltnerParams, KeltnerSeries,
    KeltnerSolution, KeltnerState, LinRegBar, LinRegParams, LinRegSolution, LinRegState,
    MacdParams, MacdSeries, MacdSolution, MacdState, RsiParams, RsiSeries, RsiSolution, RsiState,
    RvolParams, RvolSeries, RvolSolution, RvolState, SmaState, StdevKind, StochBarOutput,
    StochState, StochasticParams, StochasticSeries, StochasticSolution, ValidatedAtr,
    ValidatedBollinger, ValidatedDonchian, ValidatedKeltner, ValidatedLinReg, ValidatedMacd,
    ValidatedRsi, ValidatedRvol, ValidatedStochastic, ValidatedVwap, VwapMode, VwapParams,
    VwapPriceSource, VwapSeries, VwapSolution, VwapState, WmaState,
};
