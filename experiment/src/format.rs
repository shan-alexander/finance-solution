#![allow(dead_code)]

use std::fmt::Debug;

pub fn format_money<T>(val: T) -> String
    where T: Into<f64> + Debug
{
    let val: f64 = val.into();
    if val.is_finite() {
        format_f64(val)
    } else {
        format!("{:?}", val)
    }
}

pub fn format_rate<T>(val: T) -> String
    where T: Into<f64> + Debug
{
    let val: f64 = val.into();
    if val.is_finite() {
        let val_100 = val * 100.0;
        if val_100.is_finite() {
            format!("{}%", format_f64(val_100))
        } else {
            format!("{:?}", val)
        }
    } else {
        format!("{:?}", val)
    }
}

pub fn format_period<T>(val: T) -> String
    where T: Into<f64> + Debug
{
    format_f64(val.into())
}

fn format_f64(val: f64) -> String {
    if val.is_finite() {
        let precision = 4;
        format!("{:.*}", precision, val)
    } else {
        format!("{:?}", val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_money() {
        assert_eq!(format_money(0), "0.0000");
        assert_eq!(format_money(1), "1.0000");
        assert_eq!(format_money(-1), "-1.0000");
        assert_eq!(format_money(0.00023), "0.0002");
        assert_eq!(format_money(-0.00023), "-0.0002");
        assert_eq!(format_money(0.005678), "0.0057");
        assert_eq!(format_money(-0.005678), "-0.0057");
        assert_eq!(format_money(12345.987654321), "12345.9877");
        assert_eq!(format_money(-12345.987654321), "-12345.9877");
        assert_eq!(format_money(f64::NAN), "NaN");
        assert_eq!(format_money(f64::INFINITY), "inf");
        assert_eq!(format_money(f64::NEG_INFINITY), "-inf");
    }

    #[test]
    fn test_format_rate() {
        assert_eq!(format_rate(0), "0.0000%");
        assert_eq!(format_rate(1), "100.0000%");
        assert_eq!(format_rate(-1), "-100.0000%");
        assert_eq!(format_rate(0.00023), "0.0230%");
        assert_eq!(format_rate(-0.00023), "-0.0230%");
        assert_eq!(format_rate(0.005678), "0.5678%");
        assert_eq!(format_rate(-0.005678), "-0.5678%");
        assert_eq!(format_rate(12345.987654321), "1234598.7654%");
        assert_eq!(format_rate(-12345.987654321), "-1234598.7654%");
        assert_eq!(format_rate(f64::NAN), "NaN");
        assert_eq!(format_rate(f64::INFINITY), "inf");
        assert_eq!(format_rate(f64::NEG_INFINITY), "-inf");
        assert!(format_rate(f64::MIN).len() > 10);
        assert!(format_rate(f64::MAX).len() > 10);
    }

}
