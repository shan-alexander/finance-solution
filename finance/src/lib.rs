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
macro_rules! assert_rounded_2 {
    ( $x1:expr, $x2:expr ) => {
        assert_eq!(round_2($x1), round_2($x2));
    };
}

#[macro_export]
macro_rules! assert_rounded_4 {
    ( $x1:expr, $x2:expr ) => {
        assert_eq!(round_4($x1), round_4($x2));
    };
}

#[macro_export]
macro_rules! assert_rounded_6 {
    ( $x1:expr, $x2:expr ) => {
        assert_eq!(round_6($x1), round_6($x2));
    };
}

#[macro_export]
macro_rules! assert_rounded_8 {
    ( $x1:expr, $x2:expr ) => {
        assert_eq!(round_8($x1), round_8($x2));
    };
}



