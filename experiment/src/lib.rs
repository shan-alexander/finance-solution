// extern crate num_traits;
// extern crate ordered_float;
extern crate float_cmp;
extern crate finance;

pub mod alternative_1;
//pub mod format;
pub mod verbose;


#[macro_export]
macro_rules! assert_rounded_4 {
    ( $x1:expr, $x2:expr ) => {
        assert_eq!(($x1 * 10_000.0f64).round() / 10_000.0, ($x2 * 10_000f64).round() / 10_000.0);
    };
}