use experiment::verbose::*;
mod common;

// use log::Level;
// use log::{info, warn, log_enabled};

#[test]
fn test_convert_apr_to_periodic_fn() {
    
    // https://stackoverflow.com/questions/26469715/how-do-i-write-a-rust-unit-test-that-ensures-that-a-panic-has-occurred
    //<-- Any panics here inside the test, will cause test failure (good)
    
    // this should panic due to infinite rate provided
    let result = std::panic::catch_unwind(|| convert_rates::convert_apr_to_periodic_f64(1_f64/0_f64, 3));
    assert!(result.is_err());  //probe further for specific error type here, if desired

    // this should pass
    let r = convert_rates::convert_apr_to_periodic_f64(common::get_rate(), 4);
    assert!(r.is_finite());
}



