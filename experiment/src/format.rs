#![allow(dead_code)]

use num_format::{SystemLocale, ToFormattedString};

pub fn main() {
    try_locale();
    // try_integer();
    // try_format_integer();
    // try_format_money();
    // try_format_rate();
}

fn try_locale() {
    // From https://crates.io/crates/num-format

    let locale = SystemLocale::default().unwrap();
    println!("My system's default locale is...");
    println!("{:#?}", &locale);

    let available = SystemLocale::available_names().unwrap();
    println!("My available locale names are...");
    println!("{:#?}", available);
}

fn try_integer() {
    let locale = SystemLocale::default().unwrap();
    // let locale = Locale::luo;
    // let locale = Locale::vi;
    dbg!(0.to_formatted_string(&locale));
    dbg!((-1).to_formatted_string(&locale));
    dbg!(1.to_formatted_string(&locale));
    dbg!((-10).to_formatted_string(&locale));
    dbg!(10.to_formatted_string(&locale));
    dbg!(123456789.to_formatted_string(&locale));
    dbg!((-123456789).to_formatted_string(&locale));
}

fn try_format_integer() {
    dbg!(format_integer(123456789));
    dbg!(format_integer(-123456789));
    dbg!(format_integer(123u8));
    dbg!(format_integer(-123i8));
    dbg!(format_integer(12345u16));
    dbg!(format_integer(12345i16));
    dbg!(format_integer(-12345i16));
    dbg!(format_integer(1234567890u64));
    dbg!(format_integer(123456789i64));
    dbg!(format_integer(12345678901234567i128));
    dbg!(format_integer(u8::min_value()));
    dbg!(format_integer(u8::max_value()));
    dbg!(format_integer(i8::min_value()));
    dbg!(format_integer(i8::max_value()));
    dbg!(format_integer(u16::min_value()));
    dbg!(format_integer(u16::max_value()));
    dbg!(format_integer(i16::min_value()));
    dbg!(format_integer(i16::max_value()));
    dbg!(format_integer(u32::min_value()));
    dbg!(format_integer(u32::max_value()));
    dbg!(format_integer(i32::min_value()));
    dbg!(format_integer(i32::max_value()));
    dbg!(format_integer(u64::min_value()));
    dbg!(format_integer(u64::max_value()));
    dbg!(format_integer(i64::min_value()));
    dbg!(format_integer(i64::max_value()));
    dbg!(format_integer(i128::min_value()));
    dbg!(format_integer(i128::max_value()));
}

fn try_format_money() {
    dbg!(format_money(0));
    dbg!(format_money(1));
    dbg!(format_money(-1));
    dbg!(format_money(0.00023));
    dbg!(format_money(-0.00023));
    dbg!(format_money(0.005678));
    dbg!(format_money(- 0.005678));
    dbg!(format_money(12345.987654321));
    dbg!(format_money(-12345.987654321));
    dbg!(f64::NAN);
    dbg!(f64::INFINITY);
    dbg!(f64::NEG_INFINITY);
    dbg!(f64::MIN);
    dbg!(f64::MIN_POSITIVE);
    dbg!(f64::MAX);
    dbg!(format_money(f64::NAN));
    dbg!(format_money(f64::INFINITY));
    dbg!(format_money(f64::NEG_INFINITY));
    dbg!(format_money(f64::MIN));
    dbg!(format_money(f64::MIN_POSITIVE));
    dbg!(format_money(f64::MAX));
}

fn try_format_rate() {
    dbg!(format_rate(0));
    dbg!(format_rate(1));
    dbg!(format_rate(-1));
    dbg!(format_rate(0.00023));
    dbg!(format_rate(-0.00023));
    dbg!(format_rate(0.005678));
    dbg!(format_rate(- 0.005678));
    dbg!(format_rate(12345.987654321));
    dbg!(format_rate(-12345.987654321));
    dbg!(format_rate(f64::NAN));
    dbg!(format_rate(f64::INFINITY));
    dbg!(format_rate(f64::NEG_INFINITY));
    dbg!(format_rate(f64::MIN));
    dbg!(format_rate(f64::MIN_POSITIVE));
    dbg!(format_rate(f64::MAX));
}

pub fn format_integer<T>(val: T) -> String
    where T: Into<i128>
{
    val.into().to_formatted_string(&SystemLocale::default().unwrap())
}

pub fn format_money<T>(val: T) -> String
    where T: Into<f64>
{
    let val: f64 = val.into();
    if val.is_finite() {
        format_f64(val)
    } else {
        format!("{:?}", val)
    }
}

pub fn format_rate<T>(val: T) -> String
    where T: Into<f64>
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

fn format_f64(val: f64) -> String {
    if val.is_finite() {
        let precision = 4;
        let locale = SystemLocale::default().unwrap();
        let left = format_integer(val.trunc().abs() as i128);
        let right = &format!("{:.*}", precision, val.fract().abs())[2..];
        let minus_sign = if val.is_sign_negative() { locale.minus_sign() } else { "" };
        format!("{}{}{}{}", minus_sign, left, locale.decimal(), right)
    } else {
        format!("{:?}", val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_integer() {
        let locale = SystemLocale::default();
        if let Ok(loc) = locale {
            if loc.minus_sign() == "-" && loc.decimal() == "." {
                assert_eq!(format_integer(123456789), "123,456,789");
                assert_eq!(format_integer(-123456789), "-123,456,789");
                assert_eq!(format_integer(123u8), "123");
                assert_eq!(format_integer(-123i8), "-123");
                assert_eq!(format_integer(12345u16), "12,345");
                assert_eq!(format_integer(12345i16), "12,345");
                assert_eq!(format_integer(-12345i16), "-12,345");
                assert_eq!(format_integer(1234567890u64), "1,234,567,890");
                assert_eq!(format_integer(123456789i64), "123,456,789");
                assert_eq!(format_integer(12345678901234567i128), "12,345,678,901,234,567");
                assert_eq!(format_integer(u8::min_value()), "0");
                assert_eq!(format_integer(u8::max_value()), "255");
                assert_eq!(format_integer(i8::min_value()), "-128");
                assert_eq!(format_integer(i8::max_value()), "127");
                assert_eq!(format_integer(u16::min_value()), "0");
                assert_eq!(format_integer(u16::max_value()), "65,535");
                assert_eq!(format_integer(i16::min_value()), "-32,768");
                assert_eq!(format_integer(i16::max_value()), "32,767");
                assert_eq!(format_integer(u32::min_value()), "0");
                assert_eq!(format_integer(u32::max_value()), "4,294,967,295");
                assert_eq!(format_integer(i32::min_value()), "-2,147,483,648");
                assert_eq!(format_integer(i32::max_value()), "2,147,483,647");
                assert_eq!(format_integer(u64::min_value()), "0");
                assert_eq!(format_integer(u64::max_value()), "18,446,744,073,709,551,615");
                assert_eq!(format_integer(i64::min_value()), "-9,223,372,036,854,775,808");
                assert_eq!(format_integer(i64::max_value()), "9,223,372,036,854,775,807");
                assert_eq!(format_integer(i128::min_value()), "-170,141,183,460,469,231,731,687,303,715,884,105,728");
                assert_eq!(format_integer(i128::max_value()), "170,141,183,460,469,231,731,687,303,715,884,105,727");
            }
        }
    }

    #[test]
    fn test_format_money() {
        let locale = SystemLocale::default();
        if let Ok(loc) = locale {
            if loc.minus_sign() == "-" && loc.decimal() == "." {
                assert_eq!(format_money(0), "0.0000");
                assert_eq!(format_money(1), "1.0000");
                assert_eq!(format_money(-1), "-1.0000");
                assert_eq!(format_money(0.00023), "0.0002");
                assert_eq!(format_money(-0.00023), "-0.0002");
                assert_eq!(format_money(0.005678), "0.0057");
                assert_eq!(format_money(-0.005678), "-0.0057");
                assert_eq!(format_money(12345.987654321), "12,345.9877");
                assert_eq!(format_money(-12345.987654321), "-12,345.9877");
                assert_eq!(format_money(f64::NAN), "NaN");
                assert_eq!(format_money(f64::INFINITY), "inf");
                assert_eq!(format_money(f64::NEG_INFINITY), "-inf");
                assert_eq!(format_money(f64::MIN), "-170,141,183,460,469,231,731,687,303,715,884,105,727.0000");
                assert_eq!(format_money(f64::MIN_POSITIVE), "0.0000");
                assert_eq!(format_money(f64::MAX), "170,141,183,460,469,231,731,687,303,715,884,105,727.0000");
            }
        }
    }

    #[test]
    fn test_format_rate() {
        let locale = SystemLocale::default();
        if let Ok(loc) = locale {
            if loc.minus_sign() == "-" && loc.decimal() == "." {
                assert_eq!(format_rate(0), "0.0000%");
                assert_eq!(format_rate(1), "100.0000%");
                assert_eq!(format_rate(-1), "-100.0000%");
                assert_eq!(format_rate(0.00023), "0.0230%");
                assert_eq!(format_rate(-0.00023), "-0.0230%");
                assert_eq!(format_rate(0.005678), "0.5678%");
                assert_eq!(format_rate(-0.005678), "-0.5678%");
                assert_eq!(format_rate(12345.987654321), "1,234,598.7654%");
                assert_eq!(format_rate(-12345.987654321), "-1,234,598.7654%");
                assert_eq!(format_rate(f64::NAN), "NaN");
                assert_eq!(format_rate(f64::INFINITY), "inf");
                assert_eq!(format_rate(f64::NEG_INFINITY), "-inf");
                assert!(format_rate(f64::MIN).len() > 10);
                assert!(format_rate(f64::MAX).len() > 10);
            }
        }
    }

}

