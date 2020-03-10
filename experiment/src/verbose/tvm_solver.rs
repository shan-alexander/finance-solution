#![allow(dead_code)]
#![allow(unused_imports)]

use float_cmp::ApproxEq;
use log::Level;
use log::{info, warn, log_enabled};

pub fn main() {
    try_solver();
    try_bilbo_baggins_condensed();
}

fn try_solver() {

    let pv = TimeValueMoneyInput::PV { period_rate: 0.034_f64, 
                            period_count: 5, 
                            fv: 250_000_f64, 
                          }.solve();
    dbg!(pv);
}

fn try_bilbo_baggins_condensed() {
    // problem inputs
    let retire_in_years_from_now: u8 = 30;
    let retirement_income_per_month: f64 = 4_000.;
    let retirement_income_for_how_many_years: u8 = 20;
    let buy_cabin_in_years_from_now: u8 = 20;
    let cost_of_cabin: f64 = 50_000.;
    let inheretance_left_behind: f64 = 200_000.;
    let currently_save_per_month: f64 = 2_500.;
    let effective_annual_rate_before_retire: f64 = 0.045;
    let effective_annual_rate_after_retire: f64 = 0.028;

    // solve 
    let monthly_epr_before_retire = Rate::Ear2Per { ear: effective_annual_rate_before_retire, periods_per_year: 12}.solve();
    let monthly_epr_after_retire = Rate::Ear2Per { ear: effective_annual_rate_after_retire, periods_per_year: 12}.solve();
    let months_of_savings_before_buying_cabin = buy_cabin_in_years_from_now * 12;

    let fv_savings_before_buying_cabin = TVM::FVann { period_rate: monthly_epr_before_retire, 
                                                      period_count: months_of_savings_before_buying_cabin as u16,
                                                      pmt: currently_save_per_month,
                                                      due: false
                                                    }.solve();
    // dbg!(fv_savings_before_buying_cabin);
    let money_remaining_after_cabin_purchase = fv_savings_before_buying_cabin - cost_of_cabin; 
    let retirement_income_at_beginning_of_retirement = TVM::PVann { period_rate: monthly_epr_after_retire,
                                                                    period_count: (retirement_income_for_how_many_years*12) as u16,
                                                                    pmt: retirement_income_per_month,
                                                                    due: false
                                                                  }.solve();
    // dbg!(retirement_income_at_beginning_of_retirement);
    let inheretance_at_time_of_retirement = TVM::PV { period_rate: effective_annual_rate_after_retire, 
                                                      period_count: retirement_income_for_how_many_years as u16, 
                                                      fv: inheretance_left_behind, 
                                                }.solve();
    // dbg!(retirement_income_at_beginning_of_retirement);
    let how_much_needed_at_moment_of_retirement_to_achieve_goals = &inheretance_at_time_of_retirement + &retirement_income_at_beginning_of_retirement;
    let years_between_cabin_purchase_and_retirement = retire_in_years_from_now - buy_cabin_in_years_from_now;
    let fv_of_money_after_cabin_purchase_at_retirement = TVM::FV { period_rate: effective_annual_rate_before_retire, 
                                                                   period_count: years_between_cabin_purchase_and_retirement as u16,
                                                                   pv: money_remaining_after_cabin_purchase, 
                                                                  }.solve();


    let net_amount_needed_at_retirement = &how_much_needed_at_moment_of_retirement_to_achieve_goals - &fv_of_money_after_cabin_purchase_at_retirement;
    // dbg!(net_amount_needed_at_retirement);
    let months_between_cabin_and_retirement = ((retire_in_years_from_now - buy_cabin_in_years_from_now) * 12) as u16;


    let monthly_savings_needed_after_cabin = TVM::PMT {period_rate: monthly_epr_before_retire, 
                                                        period_count: months_between_cabin_and_retirement, 
                                                        pv: 0., 
                                                        fv: net_amount_needed_at_retirement, 
                                                        due: false,
                                                      }.solve();
    // final answer
    dbg!(monthly_savings_needed_after_cabin);
                                                                    
}

#[derive(Debug)]
pub enum ConvertRate {
    Apr2Per { apr: f64, periods_per_year: u16 },
    Apr2Ear { apr: f64, periods_per_year: u16 },
    Per2Ear { periodic_rate: f64, periods_per_year: u16 },
    Ear2Per { ear: f64, periods_per_year: u16 },
}
type Rate = ConvertRate;
impl ConvertRate {
    pub fn solve(self) -> f64 {
        match self {
            Self::Apr2Per { apr, periods_per_year, } => apr_to_per(apr, periods_per_year),
            Self::Apr2Ear { apr, periods_per_year, } => apr_to_ear(apr, periods_per_year),
            Self::Per2Ear { periodic_rate, periods_per_year, } => per_to_ear(periodic_rate, periods_per_year),
            Self::Ear2Per { ear, periods_per_year, } => ear_to_per(ear, periods_per_year),
        }
    }
}

#[derive(Debug)]
pub enum TimeValueMoneyInput {
    PV { period_rate: f64, period_count: u16, fv: f64 },
    FV { period_rate: f64, period_count: u16, pv: f64},
    PMT { period_rate: f64, period_count: u16, pv: f64, fv: f64, due: bool },
    PVann { period_rate: f64, period_count: u16, pmt: f64, due: bool },
    FVann { period_rate: f64, period_count: u16, pmt: f64, due: bool },
    // FVsched,
    // NPER,
    // PMT,
    // PERP,
    // RATE,
}
type TVM = TimeValueMoneyInput;
impl TimeValueMoneyInput {
    pub fn solve(self) -> f64 {
        match self {
            Self::PV { period_rate, period_count, fv, } => pv(period_rate, period_count, fv),
            Self::FV { period_rate, period_count, pv, } => fv(period_rate, period_count, pv),
            Self::PMT { period_rate, period_count, pv, fv, due, } => pmt(period_rate, period_count, pv, fv, due),
            Self::PVann { period_rate, period_count, pmt, due, } => pv_ann(period_rate, period_count, pmt, due),
            Self::FVann { period_rate, period_count, pmt, due, } => fv_ann(period_rate, period_count, pmt, due),
        }
    }
}

// speed formulas -> f64

pub fn pv(r: f64, n: u16, fv: f64) -> f64 {
    // PV = 𝐅𝐕 / (𝟏 + 𝐢)^n
    // Bench:  17.781 ns
    fv / (1. + r).powi(n as i32)
}
pub fn fv(r: f64, n: u16, pv: f64) -> f64 {
    // 𝐅𝐕 = PV * (𝟏 + 𝐢)^n
    // Bench:  
    pv * (1. + r).powi(n as i32)
}
pub fn pmt(r: f64, n: u16, pv: f64, fv: f64, due: bool) -> f64 {
    if r <= 0.0 {
        (pv + fv) / n as f64
    } else {
        (pv * (1. + r).powi(n as i32) + fv) / (((1. + r).powi(n as i32) - 1.) / r) / (1. + (r * due as i32 as f64))
    }
}
pub fn pv_ann(r: f64, n: u16, pmt: f64, due: bool) -> f64 {
    pmt * ((1. - (1. / (1. + r)).powi(n as i32)) / r) * (1. + (r * due as i32 as f64))
}
pub fn fv_ann(r: f64, n: u16, pmt: f64, due: bool) -> f64 {
    pmt * (((1. + r).powi(n as i32) - 1.) / r) * (1. + (r * due as i32 as f64))
}
pub fn apr_to_per(r: f64, n: u16) -> f64 {
    r / n as f64
}
pub fn apr_to_ear(r: f64, n: u16) -> f64 {
    (1_f64 + (r/n as f64)).powi(n as i32) - 1_f64
}
pub fn per_to_ear(r: f64, n: u16) -> f64 {
    (1_f64 + r).powi(n as i32) - 1_f64
}
pub fn ear_to_per(r: f64, n: u16) -> f64 {
    (1_f64 + r).powf(1_f64/n as f64) - 1_f64
}
// need per to apr
// need ear to apr