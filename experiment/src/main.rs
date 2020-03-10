#![allow(dead_code)]
mod fina; // a copy&paste of the finance-math crate library, for quick reference
use experiment::*;
use env_logger::Env;



fn main() {
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    // try_fv_vec();
    // alternative_1::finance::main();
    // verbose::present_value::main();
    // verbose::present_value_annuity::main();
    // verbose::future_value::main();
    // verbose::future_value_annuity::main();
    // verbose::net_present_value::main();
    // verbose::payment::main();
    // verbose::nper::main();
    // verbose::convert_rates::main();
    // verbose::perpetuity::main();
    // verbose::bilbo_baggins::main();
    verbose::tvm_solver::main();

    
    // let decimal_test = dec!(12.123456789123456789123456789123456789123456789123456789);
    // dbg!(&decimal_test);
    // let decimal_test2 = dec!(12.123456789123456789123456789123456789123456789123456789);
    // assert_eq!(decimal_test,decimal_test2);
    // dbg!(decimal_test * dec!(0.39));
    // let f64_test = 12.123456789123456789123456789123456789123456789123456789;
    // dbg!(&f64_test);
    // dbg!(f64_test * 0.39);
    
}