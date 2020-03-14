#![allow(dead_code)]
#![allow(unused_imports)]

use float_cmp::ApproxEq;
use log::Level;
use log::{info, warn, log_enabled};

pub fn main() {
    try_present_value_perpetuity();
}

fn try_present_value_perpetuity() {
    dbg!(present_value_perpetuity_simple(500_f64, 0.01, false));
    dbg!(present_value_perpetuity_general(500_f64, 0.01, 4_f64, 12_f64, false));
    dbg!(present_value_perpetuity_general(500_f64, 0.01, 4_f64, 12_f64, true));
}

pub fn present_value_perpetuity_simple<C: Into<f64> + Copy>(payment: C, rate: f64, due: bool) -> f64 {
    let pmt = payment.into();
    if due == true {
        pmt + (pmt / rate)
    } else {
        pmt / rate
    }
}

pub fn present_value_perpetuity_general<T: Into<f64> + Copy, C: Into<f64> + Copy>(payment: C, periodic_rate: f64, compounds_per_year: T, payments_per_year: T, due: bool) -> f64 {
    // (rate of interest per payment period) is p = (1+i)^c ─1
    // where i is the periodic rate of interest and c is the number of interest conversion periods per payment interval.
    // c = # of interest conversion periods per year / # of payment periods per year
    // https://www.georgebrown.ca/uploadedFiles/TLC/_documents/Formula%20Sheet%20for%20Financial%20Mathematics.pdf
    let pmt = payment.into();
    let cpy = compounds_per_year.into();
    let ppy = payments_per_year.into();
    let c = cpy / ppy;
    let p = (1. + periodic_rate).powf(c) - 1.;
    if due == true {
        pmt + (pmt / p)
    } else {
        pmt / p
    }
}