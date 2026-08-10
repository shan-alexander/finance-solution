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

#[doc(inline)]
pub use path::*;
#[doc(inline)]
pub use returns::*;
#[doc(inline)]
pub use risk::*;
