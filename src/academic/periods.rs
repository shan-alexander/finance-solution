//! **Period calculations.** This function can be used to calculate how many periods 
//! are needed to convert a present value into a specified future value given a rate.
//!
//! # Formulas
//!
//! The <i>periods</i> calculation is:
//! > img
//!

// Import needed for the function references in the Rustdoc comments.
// #[allow(unused_imports)]

use crate::*;
use crate::core::tvm;

/// Academic function for periods. Returns a solution struct.
pub fn periods<P, F>(rate: f64, present_value: P, future_value: F) -> tvm::TvmSolution
    where P: Into<f64> + Copy, F: Into<f64> + Copy
{
    let solution = tvm::periods_solution(rate, present_value.into(), -future_value.into(), false);
    tvm::TvmSolution::new(
        *solution.calculated_field(),
        CalculationType::Academic,
        solution.continuous_compounding(),
        solution.rate(),
        solution.periods(),
        solution.present_value(),
        -solution.future_value(),
        &ConcreteFormula::fv(solution.rate(), solution.periods(), solution.present_value(), -solution.future_value()),
        SYMBOLIC_FORMULAS.fv_academic,
    )
}