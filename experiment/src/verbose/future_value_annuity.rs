#![allow(dead_code)]
#![allow(unused_imports)]

use float_cmp::ApproxEq;
use log::Level;
use log::{info, warn, log_enabled};

pub fn main() { 
    // try_future_value_annuity();
    // try_future_value_annuity_due();
    try_future_value_annuity_general();
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
    pub num_periods: u16,
    pub future_value_annuity: f64,
}
impl FutureValueAnnuitySolution {
    pub fn new(annuity_payment_amount: f64, periodic_rate: f64, num_periods: u16, future_value_annuity: f64) -> Self {
        Self {
            annuity_payment_amount,
            periodic_rate,
            num_periods,
            future_value_annuity,
        }
    }
}
pub fn future_value_annuity(annuity_payment_amount: f64, periodic_rate: f64, num_periods: u16) -> FutureValueAnnuitySolution {
    // FV_ann = Constant_Cashflow * [ ( (1+periodic_rate)^n -1 )/ periodic_rate ]
    let fv_ann = annuity_payment_amount * ((1. + periodic_rate).powi(num_periods as i32) - 1.) / periodic_rate;

    FutureValueAnnuitySolution::new(annuity_payment_amount, periodic_rate, num_periods, fv_ann)
}

pub fn future_value_annuity_due(annuity_payment_amount: f64, periodic_rate: f64, num_periods: u16) -> FutureValueAnnuitySolution {
    //  FV_ann_due = PMT * (((1 + r)^n -1)/i)  * (1 + r)
    let fv_ann_due = (annuity_payment_amount * ((1. + periodic_rate).powi(num_periods as i32) - 1.) / periodic_rate) * (1. + periodic_rate);

    FutureValueAnnuitySolution::new(annuity_payment_amount, periodic_rate, num_periods, fv_ann_due)
}

#[derive(Debug)]
pub struct FutureValueAnnuityGeneralSolution {
    pub annuity_payment_amount: f64,
    pub periodic_rate: f64,
    pub num_periods: u16,
    pub compound_intervals_per_payment_period: u16,
    pub future_value_annuity: f64,
}
impl FutureValueAnnuityGeneralSolution {
    pub fn new(annuity_payment_amount: f64, periodic_rate: f64, num_periods: u16, compound_intervals_per_payment_period: u16, future_value_annuity: f64) -> Self {
        Self {
            annuity_payment_amount,
            periodic_rate,
            num_periods,
            compound_intervals_per_payment_period,
            future_value_annuity,
        }
    }
}
pub fn future_value_annuity_general(annuity_payment_amount: f64, periodic_rate: f64, num_payment_periods: u16, num_compounding_periods_per_payment_period: u16) -> FutureValueAnnuityGeneralSolution {
    // p = (1+i)^c ─1   where i is the periodic rate of interest and c is the number of interest conversion periods per payment interval.
    // p is the "equivalent rate of interest per payment period"
    let p: f64 = (1. + periodic_rate).powi(num_compounding_periods_per_payment_period as i32) - 1.;
    // FV_ann_general = Constant_Cashflow * [ ( (1+p)^n -1 )/ p ]
    let fv_ann_gen = annuity_payment_amount * ((1. + p).powi(num_payment_periods as i32) - 1.) / p;

    FutureValueAnnuityGeneralSolution::new(annuity_payment_amount, periodic_rate, num_payment_periods, num_compounding_periods_per_payment_period, fv_ann_gen)
}

pub fn future_value_annuity_general_due(annuity_payment_amount: f64, periodic_rate: f64, num_payment_periods: u16, num_compounding_periods_per_payment_period: u16) -> FutureValueAnnuityGeneralSolution {
    // p = (1+i)^c ─1   where i is the periodic rate of interest and c is the number of interest conversion periods per payment interval.
    // p is the "equivalent rate of interest per payment period"
    let p: f64 = (1. + periodic_rate).powi(num_compounding_periods_per_payment_period as i32) - 1.;
    // FV_ann_general_due = Constant_Cashflow * [ ( (1+p)^n -1 )/ p ] * (1 + i)
    let fv_ann_gen_due = (annuity_payment_amount * ((1. + p).powi(num_payment_periods as i32) - 1.) / p) * (1. + periodic_rate);

    FutureValueAnnuityGeneralSolution::new(annuity_payment_amount, periodic_rate, num_payment_periods, num_compounding_periods_per_payment_period, fv_ann_gen_due)
}



