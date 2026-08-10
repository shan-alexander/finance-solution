//! Shared utilities: errors, validation helpers, domain newtypes.
pub mod error;
pub mod primitives;

#[doc(inline)]
pub use error::{FinanceError, FinanceResult};
#[doc(inline)]
pub use primitives::{Money, PeriodLength, Periods, PositivePrice, Rate};
