pub mod future_value;
#[doc(inline)]
pub use future_value::*;

pub mod present_value;
#[doc(inline)]
pub use present_value::*;

pub mod rate;
#[doc(inline)]
pub use rate::*;

pub mod periods;
#[doc(inline)]
pub use periods::*;

pub mod round;
#[doc(inline)]
pub use round::*;

pub mod tvm_simple;
#[doc(inline)]
pub use tvm_simple::*;

#[macro_export]
macro_rules! assert_approx_equal {
    ( $x1:expr, $x2:expr ) => {
        if ($x1 * 10_000.0f64).round() / 10_000.0 != ($x2 * 10_000.0f64).round() / 10_000.0 {
            let max_length = 6;
            let mut str_1 = format!("{}", $x1);
            let mut str_2 = format!("{}", $x2);
            if str_1 == "-0.".to_string() {
                str_1 = "0.0".to_string();
            }
            if str_2 == "-0.".to_string() {
                str_2 = "0.0".to_string();
            }
            let mut length = std::cmp::min(str_1.len(), str_2.len());
            length = std::cmp::min(length, max_length);
            assert_eq!(str_1[..length], str_2[..length]);
        }
    };
}

#[macro_export]
macro_rules! assert_rounded_2 {
    ( $x1:expr, $x2:expr ) => {
        assert_eq!(($x1 * 100.0f64).round() / 100.0, ($x2 * 100.0f64).round() / 100.0);
    };
}

#[macro_export]
macro_rules! assert_rounded_4 {
    ( $x1:expr, $x2:expr ) => {
        assert_eq!(($x1 * 10_000.0f64).round() / 10_000.0, ($x2 * 10_000.0f64).round() / 10_000.0);
    };
}

#[macro_export]
macro_rules! assert_rounded_6 {
    ( $x1:expr, $x2:expr ) => {
        assert_eq!(($x1 * 1_000_000.0f64).round() / 1_000_000.0, ($x2 * 1_000_000f64).round() / 1_000_000.0);
    };
}

#[macro_export]
macro_rules! assert_rounded_8 {
    ( $x1:expr, $x2:expr ) => {
        assert_eq!(($x1 * 100_000_000.0f64).round() / 100_000_000.0, ($x2 * 100_000_000.0f64).round() / 100_000_000.0);
    };
}



