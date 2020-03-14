#![allow(dead_code)]
#![allow(unused_imports)]

use float_cmp::ApproxEq;
use log::Level;
use log::{info, warn, log_enabled};

pub fn main() {
    // There are 5 types of NPV formulas.
    // try_npv();
    // try_npv_series();
    // try_npv_var_cf();
    // try_npv_var_rates();
    try_npv_var_rates_cfs();
}


fn try_npv() {
    // Expect $122,891.34.
    let initial_investment = 100_000.0;
    let discount_rate = 0.10;
    let n = 10;
    let cash_flow = 20_000.0;
    println!("first npv try with initial investment: {}", &initial_investment);
    dbg!(npv(initial_investment, discount_rate, n, cash_flow));

    // dbg!(net_present_value_constant_cash_flow_single(initial_investment, discount_rate, n, cash_flow));

    // Expect $22,891.34.
    let initial_investment = -100_000.0;
    let discount_rate = 0.10;
    let n = 10;
    let cash_flow = 20_000.0;
    println!("second npv try with initial investment: {}", &initial_investment);
    let npv = npv(initial_investment, discount_rate, n, cash_flow);
    dbg!(npv);

}

fn try_npv_series() {
    // Expect $22,891.34.
    let initial_investment = -100_000.0;
    let discount_rate = 0.10;
    let n = 10;
    let cash_flow = 20_000.0;
    dbg!(npv_series(initial_investment, discount_rate, n, cash_flow));
}

fn try_npv_var_cfs() {
    // Expect $80,015.02  https://financeformulas.net/Net_Present_Value.html
    let cashflows = vec![-500_000.,200_000.,300_000.,200_000.];
    let discount_rate = 0.10;
    let npv = npv_var_cfs(discount_rate, &cashflows);
    dbg!(npv);
}

fn try_npv_var_rates() {
    // need to find third-party example
    let initial_investment = -500_000.;
    let constant_cashflow = 200_000.;
    let rates = vec![0.0358,0.01321,0.03455,0.089, 1.44233];
    let npv = npv_var_rates(&rates, initial_investment, constant_cashflow);
    dbg!(npv);
}

fn try_npv_var_rates_cfs() {
    // need to find third-party example
    let cashflows = vec![-500_000.,200_000.,300_000.,200_000.,250_000.,250_000.];
    let rates = vec![0.0358,0.01321,0.03455,0.089, 1.44233];
    let npv = npv_var_rates_cfs(&rates, &cashflows);
    dbg!(npv);
}


#[derive(Debug)]
pub struct NetPresentValue {
    pub periodic_rate: f64,
    pub periods: f64,
    pub cashflows: Vec<f64>,
    pub present_value_cashflows: Vec<f64>,
    pub initial_investment: f64,
    pub sum_of_future_cashflows: f64,
    pub npv_of_future_cashflows: f64,
    pub npv: f64,
}
impl NetPresentValue {
    pub fn new(periodic_rate: f64, periods: f64, cashflows: Vec<f64>, present_value_cashflows: Vec<f64>, initial_investment: f64, sum_of_future_cashflows: f64, npv_of_future_cashflows: f64, npv: f64,) -> Self {
        Self {
            periodic_rate,
            periods,
            cashflows,
            present_value_cashflows,
            initial_investment,
            sum_of_future_cashflows,
            npv_of_future_cashflows,
            npv,
        }
    }
}
/// Return the Net Present Value (NPV) of a constant cash flow (cf_1..cf_n are all equal). 
pub fn npv<T: Into<f64> + Copy, II: Into<f64> + Copy, C: Into<f64> + Copy>(initial_investment: II, periodic_rate: f64, periods: T, cashflow: C) -> NetPresentValue {
    let ii = initial_investment.into();
    let c = cashflow.into();
    let n = periods.into();
    // assertions to ensure valid financial computation
    assert!(ii.is_finite());
    assert!(periodic_rate.is_finite());
    assert!(n >= 1.);
    assert!(c.is_finite());
    // warning to ensure developer did not mistake rate with percentage
    if ii > 0. { 
        warn!("You used a positive initial investment amount (your cf0 = {}). Are you sure?", ii);
    }
    if periodic_rate > 1. || periodic_rate < - 1. { 
        warn!("You provided a periodic rate ({}) greater than 1. Are you sure you expect a {}% return?", periodic_rate, periodic_rate*100.0); 
    }
    let mut cashflows = vec![ii];
    let mut sum_future_cashflows = 0.;
    for _i in 1..n as i32 {
        sum_future_cashflows += c;
        cashflows.push(c);
    }
    
    let mut npv_future_cashflows = 0.;
    let mut present_value_cashflows = vec![];
    for t in 1..=n as i32 {
        let present_value_cashflow = c / (1.0 + periodic_rate).powi(t);
        assert!(present_value_cashflow.is_finite());
        npv_future_cashflows += present_value_cashflow;
        assert!(npv_future_cashflows.is_finite());
        present_value_cashflows.push(present_value_cashflow);
    }
    let npv = ii + npv_future_cashflows;
    NetPresentValue::new(periodic_rate, n, cashflows, present_value_cashflows, ii, sum_future_cashflows, npv_future_cashflows, npv)
}

#[derive(Debug)]
pub struct NpvPeriod {
    pub period: f64,
    pub initial_investment: f64, 
    pub periodic_rate: f64, 
    pub cash_flow: f64,
    pub present_value: f64,
}
impl NpvPeriod {
   pub fn new(period: f64, initial_investment: f64, periodic_rate: f64, cash_flow: f64, present_value: f64) -> Self {
    Self {
        period,
        initial_investment, 
        periodic_rate, 
        cash_flow,
        present_value,
    }
   }
}
/// Return the Net Present Value (NPV) of a constant cash flow (cf_1..cf_n are all equal). 
pub fn npv_series<T: Into<f64> + Copy>(initial_investment: f64, periodic_rate: f64, num_periods: T, cash_flow: f64) -> Vec<NpvPeriod> {
    let n = num_periods.into();
    // assertions to ensure valid financial computation
    assert!(initial_investment.is_finite());
    assert!(periodic_rate.is_finite());
    assert!(n >= 1.);
    assert!(cash_flow.is_finite());
    if initial_investment > 0. { 
        warn!("You used a positive initial investment amount (your cf0 = {}). Are you sure? Typically the initial investment is negative (cash outflow).", initial_investment);
    }
    if periodic_rate > 1. || periodic_rate < - 1. { 
        warn!("You provided a periodic rate ({}) greater than 1. Are you sure you expect a {}% return?", periodic_rate, periodic_rate*100.0); 
    }
    // final computation for NPV with constant cashflow (cf1..cf_n is the same)
    let npv_period = NpvPeriod::new(0., initial_investment, periodic_rate, initial_investment, initial_investment);
    
    let mut cashflows = vec![npv_period];
    for t in 1..=n as i32 {
        let pv = cash_flow / (1.0 + periodic_rate).powi(t);
        assert!(pv.is_finite());
        cashflows.push(NpvPeriod::new(t as f64, initial_investment, periodic_rate, cash_flow, pv));
    }
    cashflows
}

#[derive(Debug)]
pub struct NpvVarRate {
    pub periodic_rates: Vec<f64>,
    pub periods: u16,
    pub cashflows: Vec<f64>,
    pub present_value_cashflows: Vec<f64>,
    pub initial_investment: f64,
    pub sum_future_cashflows: f64,
    pub npv_future_cashflows: f64,
    pub npv: f64,
}
impl NpvVarRate {
    pub fn new(periodic_rates: Vec<f64>, periods: u16, cashflows: Vec<f64>, present_value_cashflows: Vec<f64>, initial_investment: f64, sum_future_cashflows: f64, npv_future_cashflows: f64, npv: f64,) -> Self {
        Self {
            periodic_rates,
            periods,
            cashflows,
            present_value_cashflows,
            initial_investment,
            sum_future_cashflows,
            npv_future_cashflows,
            npv,
        }
    }
}
/// Return the Net Present Value (NPV) of a variable rate with constant cashflow. 
/// The rates slice should not include a rate for periods 1..n (not period 0)
pub fn npv_var_rates(rates: &[f64], initial_investment: f64, constant_cashflow: f64) -> NpvVarRate {
    let n = rates.len();
    // assertions to ensure valid financial computation
    assert!(initial_investment.is_finite());
    assert!(constant_cashflow.is_finite());
    assert!(n > 0);
    let mut large_rate_check = false;
    for r in rates {
        assert!(r.is_finite());
        if r > &1. || r < &1.{ 
            large_rate_check = true;
        }
    }
    if large_rate_check == true {
        warn!("You used a rate higher than 1.0, indicating a percentage over 100%. Are you sure?");
    }
    
    // final computation for NPV with variable rate
    let mut npv_future_cashflows = 0.0f64;
    let mut present_value_cashflows = vec![initial_investment];
    let mut all_cashflows = vec![initial_investment];
    let mut future_cashflows: Vec<f64> = vec![];
    for t in 1..=n {
        all_cashflows.push(constant_cashflow);
        let present_value_cashflow = constant_cashflow / (1.0 + rates[t-1]).powi(t as i32);
        assert!(present_value_cashflow.is_finite());
        present_value_cashflows.push(present_value_cashflow);
        npv_future_cashflows += present_value_cashflow;
        assert!(npv_future_cashflows.is_finite());
        future_cashflows.push(constant_cashflow);
    }
    let all_rates = rates.to_vec();

    NpvVarRate::new(all_rates, (n + 1) as u16, all_cashflows, present_value_cashflows, initial_investment, future_cashflows.iter().sum(), npv_future_cashflows, npv_future_cashflows + initial_investment)
}



#[derive(Debug)]
pub struct NpvVarCashflow {
    pub periodic_rate: f64,
    pub periods: u16,
    pub cashflows: Vec<f64>,
    pub present_value_cashflows: Vec<f64>,
    pub initial_investment: f64,
    pub sum_future_cashflows: f64,
    pub npv_future_cashflows: f64,
    pub npv: f64,
}
impl NpvVarCashflow {
    pub fn new(periodic_rate: f64, periods: u16, cashflows: Vec<f64>, present_value_cashflows: Vec<f64>, initial_investment: f64, sum_future_cashflows: f64, npv_future_cashflows: f64, npv: f64,) -> Self {
        Self {
            periodic_rate,
            periods,
            cashflows,
            present_value_cashflows,
            initial_investment,
            sum_future_cashflows,
            npv_future_cashflows,
            npv,
        }
    }
}
/// Return the Net Present Value (NPV) of a variable cash flow (cf_1..cf_n can vary). 
pub fn npv_var_cfs(periodic_rate: f64, cashflows: &[f64]) -> NpvVarCashflow {
    let initial_investment = cashflows[0];
    let n = cashflows.len()-1;
    // assertions to ensure valid financial computation
    assert!(periodic_rate.is_finite());
    assert!(n > 0);
    for cf in cashflows {
        assert!(cf.is_finite());
    }
    // warning to ensure developer did not mistake rate with cf0
    if initial_investment > 0. { 
        warn!("You used a positive initial investment amount (your cf0 = {}). Are you sure?", initial_investment);
    }
    if periodic_rate > 1. || periodic_rate < - 1. { 
        warn!("You provided a periodic rate ({}) greater than 1. Are you sure you expect a {}% return?", periodic_rate, periodic_rate*100.0); 
    }
    
    // final computation for NPV with variable cashflow (cf1..cf_n can vary)
    let mut npv_future_cashflows = 0.0f64;
    let mut present_value_cashflows = vec![initial_investment];
    for t in 1..=n {
        let present_value_cashflow = cashflows[t] / (1.0 + periodic_rate).powi(t as i32);
        assert!(present_value_cashflow.is_finite());
        present_value_cashflows.push(present_value_cashflow);
        npv_future_cashflows += present_value_cashflow;
        assert!(npv_future_cashflows.is_finite());
    }
    
    let future_cashflows: Vec<f64> = cashflows[1..].to_vec();
    let all_cashflows = cashflows.to_vec();

    NpvVarCashflow::new(periodic_rate, n as u16, all_cashflows, present_value_cashflows, cashflows[0], future_cashflows.iter().sum(), npv_future_cashflows, npv_future_cashflows + initial_investment)
}

#[derive(Debug)]
pub struct NpvVarRatesCfs {
    pub periodic_rates: Vec<f64>,
    pub periods: u16,
    pub cashflows: Vec<f64>,
    pub present_value_cashflows: Vec<f64>,
    pub initial_investment: f64,
    pub sum_future_cashflows: f64,
    pub npv_future_cashflows: f64,
    pub npv: f64,
}
impl NpvVarRatesCfs {
    pub fn new(periodic_rates: Vec<f64>, periods: u16, cashflows: Vec<f64>, present_value_cashflows: Vec<f64>, initial_investment: f64, sum_future_cashflows: f64, npv_future_cashflows: f64, npv: f64,) -> Self {
        Self {
            periodic_rates,
            periods,
            cashflows,
            present_value_cashflows,
            initial_investment,
            sum_future_cashflows,
            npv_future_cashflows,
            npv,
        }
    }
}
/// Return the Net Present Value (NPV) of a variable cash flow (cf_1..cf_n can vary). 
pub fn npv_var_rates_cfs(periodic_rates: &[f64], cashflows: &[f64]) -> NpvVarRatesCfs {
    // the first cashflows[0] should be the initial investment at period0, but the rates should be for periods 1..n  
    let num_periods = periodic_rates.len();
    let num_cfs = cashflows.len();
    assert_eq!(num_periods, num_cfs-1);
    assert!(num_periods > 0);
    assert!(num_cfs > 0);
    for cf in cashflows {
        assert!(cf.is_finite());
    }
    let mut large_rate_check = false;
    for r in periodic_rates {
        assert!(r.is_finite());
        if r > &1. || r < &1. { 
            large_rate_check = true;
        }
    }
    if large_rate_check == true {
        warn!("You used a rate higher than 1.0, indicating a percentage over 100%. Are you sure?");
    }

    // warning to ensure developer did not mistake rate with cf0
    let initial_investment = cashflows[0];
    if initial_investment > 0. { 
        warn!("You used a positive initial investment amount (your cf0 = {}). Are you sure?", initial_investment);
    }
    
    // final computation for NPV with variable cashflow (cf1..cf_n can vary)
    let mut npv_future_cashflows = 0.0f64;
    let mut present_value_cashflows = vec![initial_investment];
    for t in 1..num_cfs {
        let present_value_cashflow = cashflows[t] / (1.0 + periodic_rates[t-1]).powi(t as i32);
        assert!(present_value_cashflow.is_finite());
        present_value_cashflows.push(present_value_cashflow);
        npv_future_cashflows += present_value_cashflow;
        assert!(npv_future_cashflows.is_finite());
    }
    
    let future_cashflows: Vec<f64> = cashflows[1..].to_vec();
    let all_cashflows = cashflows.to_vec();
    let all_rates = periodic_rates.to_vec();

    NpvVarRatesCfs::new(all_rates, num_periods as u16, all_cashflows, present_value_cashflows, cashflows[0], future_cashflows.iter().sum(), npv_future_cashflows, npv_future_cashflows + initial_investment)
}

// #[cfg(test)]
// mod tests {
//     use super::*;
// }