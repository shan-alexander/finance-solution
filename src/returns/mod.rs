//! Educational doubling-time helpers: Rule of 72 / 69 / 70 and exact formulas.
//!
//! Use [`doubling_solution`] for a full comparison at one rate (formulas + table),
//! or [`doubling_compare_rates`] to see how approximations behave across rates.
//!
//! All public functions return [`crate::FinanceResult`] (Result-only API, v0.1+).
pub mod rule_of_72;

#[doc(inline)]
pub use rule_of_72::*;
