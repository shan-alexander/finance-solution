//! **Future Value calculations.** This function can be used to calculate future value.
//!
//! # Formulas
//!
//! The <i>future value</i> calculation is:
//! > img
//!

// Import needed for the function references in the Rustdoc comments.
#[allow(unused_imports)]
use crate::*;

/// Academic function for future value. Returns a solution struct.
pub fn fv<T>(rate: f64, periods: u32, present_value: T) -> tvm::TvmSolution
    where T: Into<f64> + Copy 
{
    let solution = tvm::future_value_solution(rate, periods, present_value.into(), false);
    tvm::TvmSolution::new(
        *solution.calculated_field(),
        TvmCalculationType::Academic,
        solution.continuous_compounding(),
        solution.rate(), 
        solution.periods(), 
        solution.present_value(),
        -solution.future_value(),
        &ConcreteFormula::fv(solution.rate(), solution.periods(), solution.present_value(), -solution.future_value()),
        SYMBOLIC_FORMULAS.fv_academic,
    )
}