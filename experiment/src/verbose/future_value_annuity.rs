#![allow(dead_code)]
#![allow(unused_imports)]

use float_cmp::ApproxEq;
use log::Level;
use log::{info, warn, log_enabled};
use std::fmt::Debug;
use std::fmt;
use crate::format;

pub fn main() { 
    try_future_value_annuity();
    // try_future_value_annuity_due();
    // try_future_value_annuity_general();
    // try_future_value_annuity_general_due();
}

fn try_future_value_annuity() {
    // future value annuity (simple)
    // expect 6590.40  -> returns 6590.3974
    let pmt = 500_f64;
    let periods = 10;
    let discount_rate = 0.06_f64;
    let fv_ann_simple_solution = future_value_annuity(pmt, discount_rate, periods);
    dbg!(fv_ann_simple_solution);
}

fn try_future_value_annuity_general() {
    // haven't found 3rd party example yet
    let pmt = 500_f64;
    let periods = 10; // yearly payment periods
    let compounding_periods_per_payment_period = 4; // quarterly compounds
    let periodic_rate = 0.06_f64; // 6% per year
    let fv_ann_general_solution = future_value_annuity_general(pmt, periodic_rate, periods, compounding_periods_per_payment_period);
    dbg!(fv_ann_general_solution);
}

fn try_future_value_annuity_due() {
    // future value annuity DUE (simple)
    // expect 5468.41  -> returns 5468.409884
    let pmt = 1000_f64;
    let periods = 5;
    let discount_rate = 0.03_f64;
    let fv_ann_due_simple_solution = future_value_annuity_due(pmt, discount_rate, periods);
    dbg!(fv_ann_due_simple_solution);

    // annuity DUE example
    // For example, the treasurer of ABC Imports expects to invest $50,000 of the firm's funds in a long-term investment vehicle at the beginning of each year for the next five years. He expects that the company will earn 6% interest that will compound annually. What is the value that these payments should have at the end of the five-year period?
    // expect $298,765.90 (i think they rounded each calculation, not just one rounding at the end)
    // https://www.accountingtools.com/articles/what-is-the-formula-for-the-future-value-of-an-annuity-due.html
    let annuity_payment = 50_000_f64;
    let rate = 0.06_f64;
    let periods = 5_u16; // 5 years
    let future_value_annuity_due_solution = future_value_annuity_due(annuity_payment, rate, periods);
    dbg!(future_value_annuity_due_solution);

    // modification to example above
    // what if the interest on the investment compounded monthly instead of annually, and the amount invested were $4,000 at the end of each month? 
    // expect $280,475.50 (i think they rounded each calculation, not just one rounding at the end)
    // https://www.accountingtools.com/articles/what-is-the-formula-for-the-future-value-of-an-annuity-due.html
    let annuity_payment = 4_000_f64;
    let rate = 0.005_f64; // the 6% rate from above become 0.5% when converted to monthly periodic rate
    let periods = 60_u16; // 60 months instead of 5 years
    let future_value_annuity_due_solution = future_value_annuity_due(annuity_payment, rate, periods);
    dbg!(future_value_annuity_due_solution);
}

fn try_future_value_annuity_general_due() {
    // haven't found 3rd party example yet
    let pmt = 500_f64;
    let periods = 10; // yearly payment periods
    let compounding_periods_per_payment_period = 4; // quarterly compounds
    let periodic_rate = 0.06_f64; // 6% per year
    let fv_ann_general_due_solution = future_value_annuity_general_due(pmt, periodic_rate, periods, compounding_periods_per_payment_period);
    dbg!(fv_ann_general_due_solution);
}

#[derive(Debug)]
pub struct FutureValueAnnuitySolution {
    pub annuity_payment_amount: f64,
    pub periodic_rate: f64,
    pub num_periods: f64,
    pub future_value_annuity: f64,
    pub due_at_beginning: bool,
}

impl FutureValueAnnuitySolution {
    pub fn new(annuity_payment_amount: f64, periodic_rate: f64, num_periods: f64, future_value_annuity: f64, due_at_beginning: bool) -> Self {
        Self {
            annuity_payment_amount,
            periodic_rate,
            num_periods,
            future_value_annuity,
            due_at_beginning,
        }
    }
}

/*
impl Debug for FutureValueAnnuitySolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ {}, {}, {}, {}, {} }}",
               &format!("annuity_payment_amount: {}", format::format_money(self.annuity_payment_amount)),
               &format!("periodic_rate: {}", format::format_rate(self.periodic_rate)),
               &format!("num_periods: {}", format::format_period(self.num_periods)),
               &format!("future_value_annuity: {}", format::format_money(self.future_value_annuity)),
               &format!("due_at_beginning: {:?}", self.due_at_beginning),
        )
    }
}
*/

pub fn future_value_annuity<T: Into<f64> + Copy, C: Into<f64> + Copy>(annuity_payment_amount: C, periodic_rate: f64, num_periods: T) -> FutureValueAnnuitySolution {
    let c = annuity_payment_amount.into();
    let n = num_periods.into();
    // FV_ann = Constant_Cashflow * [ ( (1+periodic_rate)^n -1 )/ periodic_rate ]
    let fv_ann = c * ((1. + periodic_rate).powf(n) - 1.) / periodic_rate;
    FutureValueAnnuitySolution::new(c, periodic_rate, n, fv_ann, false)
}

pub fn future_value_annuity_due<T: Into<f64> + Copy, C: Into<f64> + Copy>(annuity_payment_amount: C, periodic_rate: f64, num_periods: T) -> FutureValueAnnuitySolution {
    let c = annuity_payment_amount.into();
    let n = num_periods.into();
    //  FV_ann_due = PMT * (((1 + r)^n -1)/i)  * (1 + r)
    let fv_ann_due = (c * ((1. + periodic_rate).powf(n) - 1.) / periodic_rate) * (1. + periodic_rate);

    FutureValueAnnuitySolution::new(c, periodic_rate, n, fv_ann_due, true)
}

#[derive(Debug)]
pub struct FutureValueAnnuityGeneralSolution {
    pub annuity_payment_amount: f64,
    pub periodic_rate: f64,
    pub num_periods: f64,
    pub compound_intervals_per_payment_period: f64,
    pub future_value_annuity: f64,
}
impl FutureValueAnnuityGeneralSolution {
    pub fn new(annuity_payment_amount: f64, periodic_rate: f64, num_periods: f64, compound_intervals_per_payment_period: f64, future_value_annuity: f64) -> Self {
        Self {
            annuity_payment_amount,
            periodic_rate,
            num_periods,
            compound_intervals_per_payment_period,
            future_value_annuity,
        }
    }
}
pub fn future_value_annuity_general<T: Into<f64> + Copy, TT:Into<f64> + Copy, C: Into<f64> + Copy>(annuity_payment_amount: C, periodic_rate: f64, num_payment_periods: T, num_compounding_periods_per_payment_period: TT) -> FutureValueAnnuityGeneralSolution {
    let pmt = annuity_payment_amount.into();
    let n = num_payment_periods.into();
    let c = num_compounding_periods_per_payment_period.into();
    // p = (1+i)^c ─1   where i is the periodic rate of interest and c is the number of interest conversion periods per payment interval.
    // p is the "equivalent rate of interest per payment period"
    let p: f64 = (1. + periodic_rate).powf(c) - 1.;
    // FV_ann_general = Constant_Cashflow * [ ( (1+p)^n -1 )/ p ]
    let fv_ann_gen = pmt * ((1. + p).powf(n) - 1.) / p;

    FutureValueAnnuityGeneralSolution::new(pmt, periodic_rate, n, c, fv_ann_gen)
}

pub fn future_value_annuity_general_due<T: Into<f64> + Copy, TT:Into<f64> + Copy, C:Into<f64> + Copy>(annuity_payment_amount: C, periodic_rate: f64, num_payment_periods: T, num_compounding_periods_per_payment_period: TT) -> FutureValueAnnuityGeneralSolution {
    let pmt = annuity_payment_amount.into();
    let n = num_payment_periods.into();
    let c = num_compounding_periods_per_payment_period.into();
    // p = (1+i)^c ─1   where i is the periodic rate of interest and c is the number of interest conversion periods per payment interval.
    // p is the "equivalent rate of interest per payment period"
    let p: f64 = (1. + periodic_rate).powf(c) - 1.;
    // FV_ann_general_due = Constant_Cashflow * [ ( (1+p)^n -1 )/ p ] * (1 + i)
    let fv_ann_gen_due = (pmt * ((1. + p).powf(n) - 1.) / p) * (1. + periodic_rate);

    FutureValueAnnuityGeneralSolution::new(pmt, periodic_rate, n, c, fv_ann_gen_due)
}



