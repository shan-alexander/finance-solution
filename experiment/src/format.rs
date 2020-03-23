#![allow(dead_code)]

use num_format::{ToFormattedString, Locale};
use std::fmt::Debug;

pub fn main() {
    // try_integer();
    // try_format_integer();
    // try_format_integer_locale();
    try_format_money();
    try_format_money_locale();
    // try_format_rate();
    // try_format_rate_locale();
}

fn try_integer() {
    for locale in [Locale::en, Locale::vi].iter() {
        println!("\nlocale = {}", locale.name());
        dbg!(0.to_formatted_string(locale));
        dbg!((-1).to_formatted_string(locale));
        dbg!(1.to_formatted_string(locale));
        dbg!((-10).to_formatted_string(locale));
        dbg!(10.to_formatted_string(locale));
        dbg!(123456789.to_formatted_string(locale));
        dbg!((-123456789).to_formatted_string(locale));
    }
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

fn try_format_integer_locale() {
    for locale in [Locale::en, Locale::vi].iter() {
        println!("\nlocale = {}", locale.name());
        dbg!(format_integer_locale(123456789, locale));
        dbg!(format_integer_locale(-123456789, locale));
        dbg!(format_integer_locale(123u8, locale));
        dbg!(format_integer_locale(-123i8, locale));
        dbg!(format_integer_locale(12345u16, locale));
        dbg!(format_integer_locale(12345i16, locale));
        dbg!(format_integer_locale(-12345i16, locale));
        dbg!(format_integer_locale(1234567890u64, locale));
        dbg!(format_integer_locale(123456789i64, locale));
        dbg!(format_integer_locale(12345678901234567i128, locale));
        dbg!(format_integer_locale(u8::min_value(), locale));
        dbg!(format_integer_locale(u8::max_value(), locale));
        dbg!(format_integer_locale(i8::min_value(), locale));
        dbg!(format_integer_locale(i8::max_value(), locale));
        dbg!(format_integer_locale(u16::min_value(), locale));
        dbg!(format_integer_locale(u16::max_value(), locale));
        dbg!(format_integer_locale(i16::min_value(), locale));
        dbg!(format_integer_locale(i16::max_value(), locale));
        dbg!(format_integer_locale(u32::min_value(), locale));
        dbg!(format_integer_locale(u32::max_value(), locale));
        dbg!(format_integer_locale(i32::min_value(), locale));
        dbg!(format_integer_locale(i32::max_value(), locale));
        dbg!(format_integer_locale(u64::min_value(), locale));
        dbg!(format_integer_locale(u64::max_value(), locale));
        dbg!(format_integer_locale(i64::min_value(), locale));
        dbg!(format_integer_locale(i64::max_value(), locale));
        dbg!(format_integer_locale(i128::min_value(), locale));
        dbg!(format_integer_locale(i128::max_value(), locale));
    }
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
    dbg!(f32::NAN);
    dbg!(f32::INFINITY);
    dbg!(f32::NEG_INFINITY);
    dbg!(f32::MIN);
    dbg!(f32::MIN_POSITIVE);
    dbg!(f32::MAX);
    dbg!(format_money(f32::NAN));
    dbg!(format_money(f32::INFINITY));
    dbg!(format_money(f32::NEG_INFINITY));
    dbg!(format_money(f32::MIN));
    dbg!(format_money(f32::MIN_POSITIVE));
    dbg!(format_money(f32::MAX));
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

fn try_format_money_locale() {
    for locale in [Locale::en, Locale::vi].iter() {
        println!("\nlocale = {}", locale.name());
        dbg!(format_money_locale(0, locale));
        dbg!(format_money_locale(1, locale));
        dbg!(format_money_locale(-1, locale));
        dbg!(format_money_locale(0.00023, locale));
        dbg!(format_money_locale(-0.00023, locale));
        dbg!(format_money_locale(0.005678, locale));
        dbg!(format_money_locale(-0.005678, locale));
        dbg!(format_money_locale(12345.987654321, locale));
        dbg!(format_money_locale(-12345.987654321, locale));
        dbg!(f32::NAN);
        dbg!(f32::INFINITY);
        dbg!(f32::NEG_INFINITY);
        dbg!(f32::MIN);
        dbg!(f32::MIN_POSITIVE);
        dbg!(f32::MAX);
        dbg!(format_money_locale(f32::NAN, locale));
        dbg!(format_money_locale(f32::INFINITY, locale));
        dbg!(format_money_locale(f32::NEG_INFINITY, locale));
        dbg!(format_money_locale(f32::MIN, locale));
        dbg!(format_money_locale(f32::MIN_POSITIVE, locale));
        dbg!(format_money_locale(f32::MAX, locale));
        dbg!(f64::NAN);
        dbg!(f64::INFINITY);
        dbg!(f64::NEG_INFINITY);
        dbg!(f64::MIN);
        dbg!(f64::MIN_POSITIVE);
        dbg!(f64::MAX);
        dbg!(format_money_locale(f64::NAN, locale));
        dbg!(format_money_locale(f64::INFINITY, locale));
        dbg!(format_money_locale(f64::NEG_INFINITY, locale));
        dbg!(format_money_locale(f64::MIN, locale));
        dbg!(format_money_locale(f64::MIN_POSITIVE, locale));
        dbg!(format_money_locale(f64::MAX, locale));
    }
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

fn try_format_rate_locale() {
    for locale in [Locale::en, Locale::vi].iter() {
        println!("\nlocale = {}", locale.name());
        dbg!(format_rate_locale(0, locale));
        dbg!(format_rate_locale(1, locale));
        dbg!(format_rate_locale(-1, locale));
        dbg!(format_rate_locale(0.00023, locale));
        dbg!(format_rate_locale(-0.00023, locale));
        dbg!(format_rate_locale(0.005678, locale));
        dbg!(format_rate_locale(-0.005678, locale));
        dbg!(format_rate_locale(12345.987654321, locale));
        dbg!(format_rate_locale(-12345.987654321, locale));
        dbg!(format_rate_locale(f64::NAN, locale));
        dbg!(format_rate_locale(f64::INFINITY, locale));
        dbg!(format_rate_locale(f64::NEG_INFINITY, locale));
        dbg!(format_rate_locale(f64::MIN, locale));
        dbg!(format_rate_locale(f64::MIN_POSITIVE, locale));
        dbg!(format_rate_locale(f64::MAX, locale));
    }
}

pub fn format_integer<T>(val: T) -> String
    where T: Into<i128> + Debug
{
    format!("{:?}", val)
}

pub fn format_integer_locale<T>(val: T, locale: &Locale) -> String
    where T: Into<i128> + Debug
{
    val.into().to_formatted_string(locale)
}

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

pub fn format_money_locale<T>(val: T, locale: &Locale) -> String
    where T: Into<f64> + Debug
{
    let val: f64 = val.into();
    if val.is_finite() {
        format_f64_locale(val, locale)
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

pub fn format_rate_locale<T>(val: T, locale: &Locale) -> String
    where T: Into<f64> + Debug
{
    let val: f64 = val.into();
    if val.is_finite() {
        let val_100 = val * 100.0;
        if val_100.is_finite() {
            format!("{}%", format_f64_locale(val_100, locale))
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
        format!("{:.*}", precision, val)
    } else {
        format!("{:?}", val)
    }
}

pub fn format_f64_locale(val: f64, locale: &Locale) -> String {
    if val.is_finite() {
        let precision = 4;
        let left = format_integer_locale(val.trunc().abs() as i128, locale);
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
        assert_eq!(format_integer(123456789), "123456789");
        assert_eq!(format_integer(-123456789), "-123456789");
        assert_eq!(format_integer(123u8), "123");
        assert_eq!(format_integer(-123i8), "-123");
        assert_eq!(format_integer(12345u16), "12345");
        assert_eq!(format_integer(12345i16), "12345");
        assert_eq!(format_integer(-12345i16), "-12345");
        assert_eq!(format_integer(1234567890u64), "1234567890");
        assert_eq!(format_integer(123456789i64), "123456789");
        assert_eq!(format_integer(12345678901234567i128), "12345678901234567");
        assert_eq!(format_integer(u8::min_value()), "0");
        assert_eq!(format_integer(u8::max_value()), "255");
        assert_eq!(format_integer(i8::min_value()), "-128");
        assert_eq!(format_integer(i8::max_value()), "127");
        assert_eq!(format_integer(u16::min_value()), "0");
        assert_eq!(format_integer(u16::max_value()), "65535");
        assert_eq!(format_integer(i16::min_value()), "-32768");
        assert_eq!(format_integer(i16::max_value()), "32767");
        assert_eq!(format_integer(u32::min_value()), "0");
        assert_eq!(format_integer(u32::max_value()), "4294967295");
        assert_eq!(format_integer(i32::min_value()), "-2147483648");
        assert_eq!(format_integer(i32::max_value()), "2147483647");
        assert_eq!(format_integer(u64::min_value()), "0");
        assert_eq!(format_integer(u64::max_value()), "18446744073709551615");
        assert_eq!(format_integer(i64::min_value()), "-9223372036854775808");
        assert_eq!(format_integer(i64::max_value()), "9223372036854775807");
        assert_eq!(format_integer(i128::min_value()), "-170141183460469231731687303715884105728");
        assert_eq!(format_integer(i128::max_value()), "170141183460469231731687303715884105727");
    }

    #[test]
    fn test_format_integer_locale() {
        let locale = Locale::en;
        assert_eq!(format_integer_locale(123456789, &locale),"123,456,789");
        assert_eq!(format_integer_locale(-123456789, &locale),"-123,456,789");
        assert_eq!(format_integer_locale(123u8, &locale),"123");
        assert_eq!(format_integer_locale(-123i8, &locale),"-123");
        assert_eq!(format_integer_locale(12345u16, &locale),"12,345");
        assert_eq!(format_integer_locale(12345i16, &locale),"12,345");
        assert_eq!(format_integer_locale(-12345i16, &locale),"-12,345");
        assert_eq!(format_integer_locale(1234567890u64, &locale),"1,234,567,890");
        assert_eq!(format_integer_locale(123456789i64, &locale),"123,456,789");
        assert_eq!(format_integer_locale(12345678901234567i128, &locale),"12,345,678,901,234,567");
        assert_eq!(format_integer_locale(u8::min_value(), &locale),"0");
        assert_eq!(format_integer_locale(u8::max_value(), &locale),"255");
        assert_eq!(format_integer_locale(i8::min_value(), &locale),"-128");
        assert_eq!(format_integer_locale(i8::max_value(), &locale),"127");
        assert_eq!(format_integer_locale(u16::min_value(), &locale),"0");
        assert_eq!(format_integer_locale(u16::max_value(), &locale),"65,535");
        assert_eq!(format_integer_locale(i16::min_value(), &locale),"-32,768");
        assert_eq!(format_integer_locale(i16::max_value(), &locale),"32,767");
        assert_eq!(format_integer_locale(u32::min_value(), &locale),"0");
        assert_eq!(format_integer_locale(u32::max_value(), &locale),"4,294,967,295");
        assert_eq!(format_integer_locale(i32::min_value(), &locale),"-2,147,483,648");
        assert_eq!(format_integer_locale(i32::max_value(), &locale),"2,147,483,647");
        assert_eq!(format_integer_locale(u64::min_value(), &locale),"0");
        assert_eq!(format_integer_locale(u64::max_value(), &locale),"18,446,744,073,709,551,615");
        assert_eq!(format_integer_locale(i64::min_value(), &locale),"-9,223,372,036,854,775,808");
        assert_eq!(format_integer_locale(i64::max_value(), &locale),"9,223,372,036,854,775,807");
        assert_eq!(format_integer_locale(i128::min_value(), &locale),"-170,141,183,460,469,231,731,687,303,715,884,105,728");
        assert_eq!(format_integer_locale(i128::max_value(), &locale),"170,141,183,460,469,231,731,687,303,715,884,105,727");
        
        let locale = Locale::vi;
        assert_eq!(format_integer_locale(123456789, &locale), "123.456.789");
        assert_eq!(format_integer_locale(-123456789, &locale), "-123.456.789");
        assert_eq!(format_integer_locale(123u8, &locale), "123");
        assert_eq!(format_integer_locale(-123i8, &locale), "-123");
        assert_eq!(format_integer_locale(12345u16, &locale), "12.345");
        assert_eq!(format_integer_locale(12345i16, &locale), "12.345");
        assert_eq!(format_integer_locale(-12345i16, &locale), "-12.345");
        assert_eq!(format_integer_locale(1234567890u64, &locale), "1.234.567.890");
        assert_eq!(format_integer_locale(123456789i64, &locale), "123.456.789");
        assert_eq!(format_integer_locale(12345678901234567i128, &locale), "12.345.678.901.234.567");
        assert_eq!(format_integer_locale(u8::min_value(), &locale), "0");
        assert_eq!(format_integer_locale(u8::max_value(), &locale), "255");
        assert_eq!(format_integer_locale(i8::min_value(), &locale), "-128");
        assert_eq!(format_integer_locale(i8::max_value(), &locale), "127");
        assert_eq!(format_integer_locale(u16::min_value(), &locale), "0");
        assert_eq!(format_integer_locale(u16::max_value(), &locale), "65.535");
        assert_eq!(format_integer_locale(i16::min_value(), &locale), "-32.768");
        assert_eq!(format_integer_locale(i16::max_value(), &locale), "32.767");
        assert_eq!(format_integer_locale(u32::min_value(), &locale), "0");
        assert_eq!(format_integer_locale(u32::max_value(), &locale), "4.294.967.295");
        assert_eq!(format_integer_locale(i32::min_value(), &locale), "-2.147.483.648");
        assert_eq!(format_integer_locale(i32::max_value(), &locale), "2.147.483.647");
        assert_eq!(format_integer_locale(u64::min_value(), &locale), "0");
        assert_eq!(format_integer_locale(u64::max_value(), &locale), "18.446.744.073.709.551.615");
        assert_eq!(format_integer_locale(i64::min_value(), &locale), "-9.223.372.036.854.775.808");
        assert_eq!(format_integer_locale(i64::max_value(), &locale), "9.223.372.036.854.775.807");
        assert_eq!(format_integer_locale(i128::min_value(), &locale), "-170.141.183.460.469.231.731.687.303.715.884.105.728");
        assert_eq!(format_integer_locale(i128::max_value(), &locale), "170.141.183.460.469.231.731.687.303.715.884.105.727");
    }

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
    fn test_format_money_locale() {
        let locale = Locale::en;
        assert_eq!(format_money_locale(0, &locale), "0.0000");
        assert_eq!(format_money_locale(1, &locale), "1.0000");
        assert_eq!(format_money_locale(-1, &locale), "-1.0000");
        assert_eq!(format_money_locale(0.00023, &locale), "0.0002");
        assert_eq!(format_money_locale(-0.00023, &locale), "-0.0002");
        assert_eq!(format_money_locale(0.005678, &locale), "0.0057");
        assert_eq!(format_money_locale(-0.005678, &locale), "-0.0057");
        assert_eq!(format_money_locale(12345.987654321, &locale), "12,345.9877");
        assert_eq!(format_money_locale(-12345.987654321, &locale), "-12,345.9877");
        assert_eq!(format_money_locale(f64::NAN, &locale), "NaN");
        assert_eq!(format_money_locale(f64::INFINITY, &locale), "inf");
        assert_eq!(format_money_locale(f64::NEG_INFINITY, &locale), "-inf");

        let locale = Locale::vi;
        assert_eq!(format_money_locale(0, &locale), "0,0000");
        assert_eq!(format_money_locale(1, &locale), "1,0000");
        assert_eq!(format_money_locale(-1, &locale), "-1,0000");
        assert_eq!(format_money_locale(0.00023, &locale), "0,0002");
        assert_eq!(format_money_locale(-0.00023, &locale), "-0,0002");
        assert_eq!(format_money_locale(0.005678, &locale), "0,0057");
        assert_eq!(format_money_locale(-0.005678, &locale), "-0,0057");
        assert_eq!(format_money_locale(12345.987654321, &locale), "12.345,9877");
        assert_eq!(format_money_locale(-12345.987654321, &locale), "-12.345,9877");
        assert_eq!(format_money_locale(f64::NAN, &locale), "NaN");
        assert_eq!(format_money_locale(f64::INFINITY, &locale), "inf");
        assert_eq!(format_money_locale(f64::NEG_INFINITY, &locale), "-inf");
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

    #[test]
    fn test_format_rate_locale() {
        let locale = Locale::en;
        assert_eq!(format_rate_locale(0, &locale), "0.0000%");
        assert_eq!(format_rate_locale(1, &locale), "100.0000%");
        assert_eq!(format_rate_locale(-1, &locale), "-100.0000%");
        assert_eq!(format_rate_locale(0.00023, &locale), "0.0230%");
        assert_eq!(format_rate_locale(-0.00023, &locale), "-0.0230%");
        assert_eq!(format_rate_locale(0.005678, &locale), "0.5678%");
        assert_eq!(format_rate_locale(-0.005678, &locale), "-0.5678%");
        assert_eq!(format_rate_locale(12345.987654321, &locale), "1,234,598.7654%");
        assert_eq!(format_rate_locale(-12345.987654321, &locale), "-1,234,598.7654%");
        assert_eq!(format_rate_locale(f64::NAN, &locale), "NaN");
        assert_eq!(format_rate_locale(f64::INFINITY, &locale), "inf");
        assert_eq!(format_rate_locale(f64::NEG_INFINITY, &locale), "-inf");
        assert!(format_rate_locale(f64::MIN, &locale).len() > 10);
        assert!(format_rate_locale(f64::MAX, &locale).len() > 10);

        let locale = Locale::vi;
        assert_eq!(format_rate_locale(0, &locale), "0,0000%");
        assert_eq!(format_rate_locale(1, &locale), "100,0000%");
        assert_eq!(format_rate_locale(-1, &locale), "-100,0000%");
        assert_eq!(format_rate_locale(0.00023, &locale), "0,0230%");
        assert_eq!(format_rate_locale(-0.00023, &locale), "-0,0230%");
        assert_eq!(format_rate_locale(0.005678, &locale), "0,5678%");
        assert_eq!(format_rate_locale(-0.005678, &locale), "-0,5678%");
        assert_eq!(format_rate_locale(12345.987654321, &locale), "1.234.598,7654%");
        assert_eq!(format_rate_locale(-12345.987654321, &locale), "-1.234.598,7654%");
        assert_eq!(format_rate_locale(f64::NAN, &locale), "NaN");
        assert_eq!(format_rate_locale(f64::INFINITY, &locale), "inf");
        assert_eq!(format_rate_locale(f64::NEG_INFINITY, &locale), "-inf");
        assert!(format_rate_locale(f64::MIN, &locale).len() > 10);
        assert!(format_rate_locale(f64::MAX, &locale).len() > 10);
   }
}
