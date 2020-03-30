pub fn round_to_fraction_of_cent(val: f64) -> f64 {
    (val * 10_000.0).round() / 10_000.0
}

pub fn round_to_cent(val: f64) -> f64 {
    (val * 100.0).round() / 100.0
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


