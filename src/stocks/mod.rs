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
    adx, adx_solution, atr, atr_solution, bollinger, bollinger_solution, cci, cci_solution, dema,
    dema_last, donchian, donchian_solution, ema, ema_last, hma, hma_last, kama, kama_last, keltner,
    keltner_solution, linear_regression, linear_regression_solution, macd, macd_solution, mfi,
    mfi_solution, mom, mom_solution, natr, obv, obv_solution, rma, rma_last, roc, rocp, rsi,
    rsi_solution, rvol, rvol_solution, sar, sar_solution, sma, sma_last, stochastics,
    stochastics_solution, supertrend, supertrend_solution, tema, tema_last, true_range_series,
    vwap, vwap_solution, willr, willr_solution, wma, wma_last, AdxBarOutput, AdxParams, AdxSeries,
    AdxSolution, AdxState, AtrParams, AtrSeries, AtrSolution, AtrState, BollingerBarOutput,
    BollingerParams, BollingerSeries, BollingerSolution, BollingerState, CciParams, CciSeries,
    CciSolution, CciState, DemaState, DonchianBarOutput, DonchianParams, DonchianSeries,
    DonchianSolution, DonchianState, EmaState, HmaState, KamaParams, KamaState, KeltnerBarOutput,
    KeltnerParams, KeltnerSeries, KeltnerSolution, KeltnerState, LinRegBar, LinRegParams,
    LinRegSolution, LinRegState, MacdParams, MacdSeries, MacdSolution, MacdState, MfiParams,
    MfiSeries, MfiSolution, MfiState, MomBarOutput, MomParams, MomSeries, MomSolution, MomState,
    NatrSeries, NatrState, ObvParams, ObvSeries, ObvSolution, ObvState, RmaState, RocSeries,
    RocpSeries, RsiParams, RsiSeries, RsiSolution, RsiState, RvolParams, RvolSeries, RvolSolution,
    RvolState, SarBarOutput, SarParams, SarSeries, SarSolution, SarState, SmaState, StdevKind,
    StochBarOutput, StochState, StochasticParams, StochasticSeries, StochasticSolution,
    SupertrendBar, SupertrendParams, SupertrendSeries, SupertrendSolution, SupertrendState,
    TemaState, ValidatedAdx, ValidatedAtr, ValidatedBollinger, ValidatedCci, ValidatedDonchian,
    ValidatedKama, ValidatedKeltner, ValidatedLinReg, ValidatedMacd, ValidatedMfi, ValidatedMom,
    ValidatedObv, ValidatedRsi, ValidatedRvol, ValidatedSar, ValidatedStochastic,
    ValidatedSupertrend, ValidatedVwap, ValidatedWillr, VwapMode, VwapParams, VwapPriceSource,
    VwapSeries, VwapSolution, VwapState, WillrParams, WillrSeries, WillrSolution, WillrState,
    WmaState,
};
