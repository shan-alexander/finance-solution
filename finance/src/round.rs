//! Utilities for rounding money amounts to the nearest hundredth or ten-thousandth part.

/// Round to four decimal places or the nearest 1/10,000th part.
///
/// This function uses f64::round() which rounds halfway cases away from 0.0.
pub fn round_to_fraction_of_cent(val: f64) -> f64 {
    (val * 10_000.0).round() / 10_000.0
}

/// Round to two decimal places or the nearest 1/100th part.
///
/// This function uses f64::round() which rounds halfway cases away from 0.0.
pub fn round_to_cent(val: f64) -> f64 {
    (val * 100.0).round() / 100.0
}

/// Round to 6 decimal points (ULPS = units of least precision)
///
/// This function uses f64::round() which rounds halfway cases away from 0.0.
pub fn round_to_ulps6(val: f64) -> f64 {
    (val * 1_000_000.0).round() / 1_000_000.0
}

/// This function uses f64::round() which rounds halfway cases away from 0.0.
pub fn round_6(val: f64) -> f64 {
    (val * 1_000_000.0).round() / 1_000_000.0
}

/// Round to 8 decimal points (ULPS = units of least precision)
///
/// This function uses f64::round() which rounds halfway cases away from 0.0.
pub fn round_to_ulps8(val: f64) -> f64 {
    (val * 100_000_000.0).round() / 100_000_000.0
}

pub fn assert_rounded_4(val_1: f64, val_2: f64) {
    assert!(val_1.is_finite());
    assert!(val_2.is_finite());
    assert_eq!(round_to_fraction_of_cent(val_1), round_to_fraction_of_cent(val_2));
}

pub fn assert_rounded_6(val_1: f64, val_2: f64) {
    assert!(val_1.is_finite());
    assert!(val_2.is_finite());
    assert_eq!(round_6(val_1), round_6(val_2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_to_fraction_of_cent() {
        assert_eq!(295_489.9418, round_to_fraction_of_cent(295_489.941849));
        assert_eq!(295_489.9418, round_to_fraction_of_cent(295_489.94175));
        assert_ne!(295_489.9418, round_to_fraction_of_cent(295_489.94185));
    }

    #[test]
    fn test_round_to_cent() {
        assert_eq!(295_489.94, round_to_cent(295_489.941849));
        assert_eq!(295_489.94, round_to_cent(295_489.9449));
        assert_ne!(295_489.94, round_to_cent(295_489.9451234));
    }

    #[test]
    fn test_round_to_ulps6() {
        assert_eq!(295_489.941849, round_to_ulps6(295_489.9418494449));
        assert_eq!(295_489.533367, round_to_ulps6(295_489.5333669999999));
        assert_eq!(295_489.945123, round_to_ulps6(295_489.9451229999));
        assert_ne!(295_489.945123, round_to_ulps6(295_489.9451249999));
    }

    #[test]
    fn test_round_to_ulps8() {
        assert_eq!(295_489.94184944, round_to_ulps8(295_489.9418494449));
        assert_eq!(295_489.53336700, round_to_ulps8(295_489.5333669999999));
        assert_eq!(295_489.94512333, round_to_ulps8(295_489.945123329999));
        assert_ne!(295_489.94512333, round_to_ulps8(295_489.9451249999));
    }
}


