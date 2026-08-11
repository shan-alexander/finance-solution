//! Shared utilities: errors, validation helpers, domain newtypes, root-finding.
pub mod error;
pub mod primitives;
pub mod root_find;

#[doc(inline)]
pub use error::{FinanceError, FinanceResult};
#[doc(inline)]
pub use primitives::{Money, PeriodLength, Periods, PositivePrice, Rate};
#[doc(inline)]
pub use root_find::brent_root;
