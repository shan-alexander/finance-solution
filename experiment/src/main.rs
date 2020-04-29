#![allow(dead_code)]
#![allow(unused_imports)]
// mod fina; // a copy&paste of the finance-math crate library, for quick reference
use experiment::*;
use env_logger::Env;

fn main() {
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    // try_fv_vec();
    // alternative_1::finance::main();
    // format::main();
    // verbose::present_value::main();
    // verbose::present_value_annuity::main();
    // verbose::future_value::main();
    // verbose::future_value_annuity::main();
    // verbose::rate::main();
    // verbose::periods::main();
    // verbose::payment_solution::main();
    // verbose::net_present_value::main();
    verbose::payment::main();
    // verbose::nper::main();
    // verbose::convert_rates::main();
    // verbose::convert_rate::main();
    // verbose::perpetuity::main();
    // verbose::bilbo_baggins::main();
    // verbose::weird_enum_solver::main();
    // verbose::tvm_solver::main();
    // verbose::find_rate::main();
    // verbose::examples_1::main();
    // verbose::payment_tvm::main();
    // lib::main();

    /*
    let a = apr!(0.034, 12);
    dbg!(a);
    let b = epr!(0.034, 12);
    dbg!(b);
    let c = ear!(0.034, 12);
    dbg!(c);
    */
}