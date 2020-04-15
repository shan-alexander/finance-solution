//! Utilities for rounding money amounts to the nearest hundredth or ten-thousandth part.

/// Round to two decimal places. This function uses f64::round() which rounds halfway cases away
/// from 0.0.
pub fn round_2(val: f64) -> f64 {
    (val * 100.0).round() / 100.0
}

/// Round to four decimal places. This function uses f64::round() which rounds halfway cases away
/// from 0.0.
pub fn round_4(val: f64) -> f64 {
    (val * 10_000.0).round() / 10_000.0
}

/// Round to six decimal places. This function uses f64::round() which rounds halfway cases away
/// from 0.0.
pub fn round_6(val: f64) -> f64 {
    (val * 1_000_000.0).round() / 1_000_000.0
}

/// Round to eight decimal places. This function uses f64::round() which rounds halfway cases away
/// from 0.0.
pub fn round_8(val: f64) -> f64 {
    (val * 100_000_000.0).round() / 100_000_000.0
}

#[inline(always)]
pub fn assert_rounded_2(val_1: f64, val_2: f64) {
    assert!(val_1.is_finite());
    assert!(val_2.is_finite());
    assert_eq!(round_2(val_1), round_2(val_2));
}

#[inline(always)]
pub fn assert_rounded_4(val_1: f64, val_2: f64) {
    assert!(val_1.is_finite());
    assert!(val_2.is_finite());
    assert_eq!(round_4(val_1), round_4(val_2));
}

#[inline(always)]
pub fn assert_rounded_6(val_1: f64, val_2: f64) {
    assert!(val_1.is_finite());
    assert!(val_2.is_finite());
    assert_eq!(round_6(val_1), round_6(val_2));
}

#[inline(always)]
pub fn assert_rounded_8(val_1: f64, val_2: f64) {
    assert!(val_1.is_finite());
    assert!(val_2.is_finite());
    assert_eq!(round_8(val_1), round_8(val_2));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_equals_zero() {
        assert_eq!(0.0f64, 0.0f64);
        assert_eq!(-0.0f64, -0.0f64);
        assert_eq!(0.0f64, -0.0f64);
    }

    #[test]
    fn test_round_2() {
        assert_eq!(295_489.94, round_2(295_489.941849));
        assert_eq!(295_489.94, round_2(295_489.9449));
        assert_ne!(295_489.94, round_2(295_489.9451234));
    }

    #[test]
    fn test_round_4() {
        assert_eq!(295_489.9418, round_4(295_489.941849));
        assert_eq!(295_489.9418, round_4(295_489.94175));
        assert_ne!(295_489.9418, round_4(295_489.94185));
    }

    #[test]
    fn test_round_6() {
        assert_eq!(295_489.941849, round_6(295_489.9418494449));
        assert_eq!(295_489.533367, round_6(295_489.5333669999999));
        assert_eq!(295_489.945123, round_6(295_489.9451229999));
        assert_ne!(295_489.945123, round_6(295_489.9451249999));
    }

    #[test]
    fn test_round_8() {
        assert_eq!(295_489.94184944, round_8(295_489.9418494449));
        assert_eq!(295_489.53336700, round_8(295_489.5333669999999));
        assert_eq!(295_489.94512333, round_8(295_489.945123329999));
        assert_ne!(295_489.94512333, round_8(295_489.9451249999));
    }

    #[test]
    fn test_assert_rounded_2_nominal() {
        assert_rounded_2!(53_243.7448, 53_243.7401);
    }

    #[test]
    #[should_panic]
    fn test_assert_rounded_2_panic() {
        assert_rounded_2!(53_243.7458, 53_243.7401);
    }

    /*
    #[test]
    fn test_assert_approx_equal() {
        assert_approx_equal!(53_243.7448, 53_243.7401);
    }
    */

    #[test]
    #[should_panic]
    fn test_assert_approx_equal_panic() {
        assert_rounded_2!(53_243.8123, 53_243.7401);
    }

}


