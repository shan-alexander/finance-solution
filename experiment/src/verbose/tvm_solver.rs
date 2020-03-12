#![allow(dead_code)]
#![allow(unused_imports)]

use float_cmp::ApproxEq;
use log::Level;
use log::{info, warn, log_enabled};

pub fn main() {
    let t2 = TimeValueMoneyInput::new()
                .set_period_rate(0.034)
                .set_period_count(36.0)
                .set_fv(2000.0)
                .solve(SolveType::PresentValue)
                .build();
     dbg!(&t2);
     let t3 = TimeValueMoneyInput::new()
            .set_period_rate(0.034)
            .set_period_count(36.0)
            .set_fv(2000.0)
            .solve(SolveType::PresentValue)
            // using the calculated present value, continue solving...
            .set_period_rate(0.058)
            .set_period_count(12_f64)
            .set_pmt(400.0)
            .solve(SolveType::FutureValueAnnuity)
            .build();
    dbg!(&t3);
}

#[derive(Debug)]
pub enum SolveType {
    PresentValue,
    FutureValue,
    PresentValueAnnuity,
    FutureValueAnnuity,
    // NetPresentValue,
    // Nper,
    // Rate,
    Payment,
}

#[derive(Debug, Copy, Clone)]
pub enum PmtType {
    Normal,
    Due,
}
impl Default for PmtType {
    fn default() -> Self {
        PmtType::Normal
    }
}
impl From<PmtType> for i32 {
    fn from(f: PmtType) -> i32 {
        match f {
            PmtType::Normal => 0,
            PmtType::Due => 1,
        }
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct TimeValueMoneyInput {
    period_rate: f64, 
    period_count: f64, 
    pmt: f64,
    pv: f64,
    fv: f64,
    due: PmtType
}
impl TimeValueMoneyInput {
    fn new() -> TimeValueMoneyInput {
        TimeValueMoneyInput {
            ..Self::default()
        }
    }
    fn set_pv(&mut self, pv: f64) -> &mut Self {
        self.pv = pv;
        self
    }
    fn set_fv(&mut self, fv: f64) -> &mut Self {
        self.fv = fv;
        self
    }
    fn set_period_count(&mut self, n: f64) -> &mut Self {
        self.period_count = n;
        self
    }
    fn set_period_rate(&mut self, r: f64) -> &mut Self {
        self.period_rate = r;
        self
    }
    fn set_pmt(&mut self, pmt: f64) -> &mut Self {
        self.pmt = pmt;
        self
    }
    fn solve(&mut self, t: SolveType) -> &mut Self {
        match t {
            SolveType::PresentValue => self.pv = pv(self),
            SolveType::FutureValue => self.fv = fv(self),
            SolveType::FutureValueAnnuity => self.fv = fv_ann(self),
            SolveType::PresentValueAnnuity => self.pv = pv_ann(self),
            // SolveType::Nper => self.period_count = nper(self),
            // SolveType::Rate => self.period_rate = rate(self),
            SolveType::Payment => self.pmt = pmt(self),
        }
        self
    }
    fn build(&self) -> TimeValueMoneyInput {
        *self
    }
}


pub fn fv(tvmi: &mut TimeValueMoneyInput) -> f64 {
    // 𝐅𝐕 = PV * (𝟏 + 𝐢)^n
    tvmi.pv * (1. + tvmi.period_rate).powf(tvmi.period_count)
}
pub fn pv(tvmi: &mut TimeValueMoneyInput) -> f64 {
    // PV = 𝐅𝐕 / (𝟏 + 𝐢)^n
    tvmi.fv / (1. + tvmi.period_rate).powf(tvmi.period_count)
}
pub fn pmt(tvmi: &mut TimeValueMoneyInput) -> f64 {
    if tvmi.period_rate <= 0.0 {
        (tvmi.pv + tvmi.fv) / tvmi.period_count as f64
    } else {
        (tvmi.pv * (1. + tvmi.period_rate).powi(tvmi.period_count as i32) + tvmi.fv) / (((1. + tvmi.period_rate).powi(tvmi.period_count as i32) - 1.) / tvmi.period_rate) / (1. + (tvmi.period_rate * i32::from(tvmi.due) as f64))
    }
}
pub fn pv_ann(tvmi: &mut TimeValueMoneyInput) -> f64 {
    tvmi.pmt * ((1. - (1. / (1. + tvmi.period_rate)).powi(tvmi.period_count as i32)) / tvmi.period_rate) * (1. + (tvmi.period_rate * i32::from(tvmi.due) as f64))
}
pub fn fv_ann(tvmi: &mut TimeValueMoneyInput) -> f64 {
    tvmi.pmt * (((1. + tvmi.period_rate).powi(tvmi.period_count as i32) - 1.) / tvmi.period_rate) * (1. + (tvmi.period_rate * i32::from(tvmi.due) as f64))
}