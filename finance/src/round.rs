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

pub fn assert_f64(val_1: f64, val_2: f64) {
    assert!(val_1.is_finite());
    assert!(val_2.is_finite());
    assert_eq!(round_to_fraction_of_cent(val_1), round_to_fraction_of_cent(val_2));
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
}


