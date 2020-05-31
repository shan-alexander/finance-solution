//! **Present Value calculations.** This function can be used to calculate present value.
//!
//! # Formulas
//!
//! The <i>present value</i> calculation is:
//! > img
//!

// Import needed for the function references in the Rustdoc comments.
#[allow(unused_imports)]
use crate::*;

/// Academic function for present value. Returns a solution struct.
pub fn pv<T>(rate: f64, periods: u32, future_value: T) -> tvm::TvmSolution
    where T: Into<f64> + Copy 
{
    let solution = tvm::present_value_solution(rate, periods, future_value.into(), false);
    tvm::TvmSolution::new(
        *solution.calculated_field(),
        TvmCalculationType::Academic,
        solution.continuous_compounding(),
        solution.rate(), 
        solution.periods(), 
        -solution.present_value(),
        solution.future_value(),
        &ConcreteFormula::pv(solution.rate(), solution.periods(), -solution.present_value(), solution.future_value()),
        SYMBOLIC_FORMULAS.pv_academic,
    )
}