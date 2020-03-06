// use std::ops::{Add, Mul};
// use crate::num_traits::pow;
// use crate::ordered_float::*;

// present value, future value, net present value.

pub fn main() {
    let rate_of_return = 0.034f64;
    let present_value_1 = 250_000f64;
    let periods = 5;
    dbg!(future_value_vec(rate_of_return, present_value_1, periods));
    let future_value_1 = future_value(rate_of_return, present_value_1, periods);
    dbg!(&future_value_1);
    let present_value_2 = present_value(rate_of_return, future_value_1, periods);
    dbg!(&present_value_2);
    dbg!(future_value_vec(0.0f64, 250_000f64, 5));
    dbg!(future_value_vec(0.034f64, 0f64, 5));
    dbg!(future_value_vec(5f64, 250_000_000_000_000_000_000_000f64, 5));
    dbg!(future_value_vec(1.0f64/3f64, 250_000f64, 5));
    // dbg!(future_value_vec(f64::NAN, 250_000f64, 5));
    // dbg!(future_value_vec(0.034f32, 250_000f32, 5));
    // dbg!(future_value_vec(NotNaN::unchecked_new(0.034f64), NotNaN::unchecked_new(250_000.0f64), 5));
}

pub fn future_value(interest_rate: f64, present_value: f64, periods: u16) -> f64 {
    assert!(interest_rate.is_finite());
    assert!(interest_rate >= 0.0);
    assert!(present_value.is_finite());
    assert!(present_value >= 0.0);
    present_value * (1.0 + interest_rate).powi(periods as i32)
}

pub fn future_value_vec(interest_rate: f64, present_value: f64, periods: u16) -> Vec<f64> {
    assert!(interest_rate.is_finite());
    assert!(interest_rate >= 0.0);
    assert!(present_value.is_finite());
    assert!(present_value >= 0.0);
    let mut v = vec![];
    let interest_mult = 1.0 + interest_rate;
    for period in 1..=periods {
        v.push(present_value * interest_mult.powi(period as i32));
    }
    v
}

pub fn present_value(rate_of_return: f64, future_value: f64, periods: u16) -> f64 {
    assert!(rate_of_return.is_finite());
    assert!(future_value.is_finite());
    assert!(future_value >= 0.0);
    future_value / (1.0 + rate_of_return).powi(periods as i32)
}

/*
pub fn future_value_vec<T>(interest_rate: T, present_value: T, periods: u16) -> Vec<T>
    where T: Mul<T, Output=T> + Add<T, Output=T> + pow::Pow<T, Output=T> + From<u16> + Copy
{
    let mut v: Vec<T> = vec![];
    let interest_mult: T = T::from(1) + interest_rate;
    for period in 1..=periods {
        let period_float: T = T::from(period);
        v.push(present_value * interest_mult.pow(period_float));
    }
    v
}
*/

/*
pub fn future_value_vec<T>(interest_rate: T, present_value: T, periods: u16) -> Vec<T>
    where T: Into<f64> + From<f64> + Copy
{
    let mut v: Vec<T> = vec![];
    let interest_mult: f64 = 1.0 + f64::from(interest_rate);
    let present_value_f64 = present_value as f64;
    for period in 1..=periods {
        let period_float: f64 = period as f64;
        v.push(T::from(present_value_f64 * interest_mult.powf(period_float)));
    }
    v
}
*/
