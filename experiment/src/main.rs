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
    // lib::main();
    // verbose::bilbo_baggins::main();
    // verbose::convert_rate::main();
    // verbose::convert_rates::main();
    // verbose::debug::main();
    // verbose::examples_1::main();
    // verbose::find_rate::main();
    // verbose::future_value::main();
    // verbose::future_value_annuity::main();
    // verbose::net_present_value::main();
    // verbose::nper::main();
    // verbose::payment::main();
    // verbose::payment_solution::main();
    // verbose::payment_tvm::main();
    // verbose::periods::main();
    // verbose::perpetuity::main();
    verbose::present_value::main();
    // verbose::present_value_annuity::main();
    // verbose::rate::main();
    // verbose::tvm_simple::main();
    // verbose::tvm_solver::main();
    // verbose::weird_enum_solver::main();

    // dbg!(finance::future_value_annuity_solution(0.034,10, 500));


    // dbg!(finance::convert_rate::apr(0.034, 12));
    // dbg!(finance::convert_rate::apr_continuous(0.034));
    // dbg!(finance::convert_rate::epr(0.034, 12));
    // dbg!(finance::convert_rate::ear(0.034, 12));
    // dbg!(finance::convert_rate::apr(0.034, 12));
    // dbg!(finance::convert_rate::apr_continuous(0.10));
    // dbg!(finance::convert_rate::ear_continuous(0.10517091807564749));

    // let b = epr!(0.034, 12);
    // dbg!(b);
    // let c = ear!(0.034, 12);
    // dbg!(c);

}