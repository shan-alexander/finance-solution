#![allow(dead_code)]
#![allow(unused_imports)]
// #[macro_use]
// use std::ops::{Add, Mul};
// use crate::num_traits::pow;
// use crate::ordered_float::*;
use float_cmp::ApproxEq;
use log::Level;
use log::{info, warn, log_enabled};
use env_logger::Env;


pub fn main() {
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    // try_present_value();
    // try_future_value();
    // try_present_value_series();
    // try_future_value_series();
    // try_future_and_present_value();
    // try_npv();
    // try_npv_series();
    // try_npv_var_cf();
    // try_npv_var_rates();
    try_npv_var_rates_cfs();

}

fn try_future_and_present_value() {
    let rate_of_return = 1.034f64;
    let present_value_1 = 250_000f64;
    let periods = 5;
    dbg!(future_value_series(rate_of_return, present_value_1, periods));
    let future_value_1 = future_value(rate_of_return, present_value_1, periods);
    dbg!(&future_value_1);
    
    let present_value_2 = present_value(rate_of_return, future_value_1.future_value, periods);
    dbg!(&present_value_2);
    // pv_debug!(&present_value_2);

    dbg!(future_value_series(0.0f64, 250_000f64, 5));
    dbg!(future_value_series(0.034f64, 0f64, 5));
    dbg!(future_value_series(5f64, 250_000_000_000_000_000_000_000f64, 5));
    dbg!(future_value_series(1.0f64/3f64, 250_000f64, 5));
    // dbg!(future_value_vec(f64::NAN, 250_000f64, 5));
    // dbg!(future_value_vec(0.034f32, 250_000f32, 5));
    // dbg!(future_value_vec(NotNaN::unchecked_new(0.034f64), NotNaN::unchecked_new(250_000.0f64), 5));
}

fn try_present_value() {
    // expect 1047.6190
    let rate_of_return = 0.05f64;
    let future_value_1 = 1_100f64;
    let periods = 1;
    let present_value_1 = present_value(rate_of_return, future_value_1, periods);
    dbg!(present_value_1);
    
    // expect 211_513.1216
    let rate_of_return = 0.034f64;
    let future_value_1 = 250_000f64;
    let periods = 5;
    let present_value_2 = present_value(rate_of_return, future_value_1, periods);
    dbg!(present_value_2);

    // expect 7181.0056
    let rate_of_return = 1.034f64;
    let future_value_1 = 250_000f64;
    let periods = 5;
    let present_value_3 = present_value(rate_of_return, future_value_1, periods);
    dbg!(present_value_3);
    // println!("{:?}", present_value_3); 

}

fn try_present_value_series() {
    // expect 1047.6190
    let rate_of_return = 0.05f64;
    let future_value_1 = 1_100f64;
    let periods = 1;
    let present_value_1 = present_value_series(rate_of_return, future_value_1, periods);
    dbg!(present_value_1);
    
    // expect 211_513.1216
    let rate_of_return = 0.034f64;
    let future_value_1 = 250_000f64;
    let periods = 5;
    let present_value_2 = present_value_series(rate_of_return, future_value_1, periods);
    dbg!(present_value_2);

    // expect 7181.0056
    let rate_of_return = 1.034f64;
    let future_value_1 = 250_000f64;
    let periods = 5;
    let present_value_3 = present_value_series(rate_of_return, future_value_1, periods);
    dbg!(present_value_3);
}

fn try_future_value() {
    // expect 1100
    let rate_of_return = 0.05f64;
    let present_value_1 = 1_047.6190f64;
    let periods = 1;
    let future_value_1 = future_value(rate_of_return, present_value_1, periods);
    dbg!(future_value_1);
    
    // expect 250_000
    let rate_of_return = 0.034f64;
    let present_value_2 = 211_513.1216f64;
    let periods = 5;
    let future_value_2 = future_value(rate_of_return, present_value_2, periods);
    dbg!(future_value_2);

    // expect 250_000
    let rate_of_return = 1.034f64;
    let present_value_3 = 7_181.0056f64;
    let periods = 5;
    let present_value_3 = future_value(rate_of_return, present_value_3, periods);
    dbg!(present_value_3);
}

fn try_future_value_series() {
    // expect 1100
    let rate_of_return = 0.05f64;
    let present_value_1 = 1_047.6190f64;
    let periods = 1;
    let future_value_1 = future_value_series(rate_of_return, present_value_1, periods);
    dbg!(future_value_1);
    
    // expect 250_000
    let rate_of_return = 0.034f64;
    let present_value_2 = 211_513.1216f64;
    let periods = 5;
    let future_value_2 = future_value_series(rate_of_return, present_value_2, periods);
    dbg!(future_value_2);

    // expect 250_000
    let rate_of_return = 1.034f64;
    let present_value_3 = 7_181.0056f64;
    let periods = 5;
    let present_value_3 = future_value_series(rate_of_return, present_value_3, periods);
    dbg!(present_value_3);
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
pub struct PresentValue {
    rate: f64,
    periods: u16,
    present_value: f64,
    future_value: f64,
    period_values: Vec<PresentValuePeriod>,
}
impl PresentValue {
    pub fn new(rate: f64, periods: u16, present_value: f64, future_value: f64, period_values: Vec<PresentValuePeriod>) -> Self {
        Self {
            rate,
            periods,
            present_value,
            future_value,
            period_values,
        }
    }
}
/// Return the Present Value of a future amount.
pub fn present_value(rate_of_return: f64, future_value: f64, periods: u16) -> PresentValue {
    // assertions to ensure valid financial computation
    assert!(rate_of_return.is_finite());
    assert!(future_value.is_finite());
    assert!(future_value >= 0.);
    if rate_of_return > 1. { 
        warn!("You used a rate of return ({}) greater than 1, therefore implying a return of {}%. Are you sure?", rate_of_return, rate_of_return*100.);
    }
    // final computation for returning a present value
    let present_value = future_value / (1. + rate_of_return).powi(periods as i32);
    
    let mut period_values = vec![PresentValuePeriod::new(periods, rate_of_return, future_value, future_value, present_value)];
    for period in 1..=periods {
        let period_value = future_value / (1. + rate_of_return).powi(period as i32);
        period_values.insert(0, PresentValuePeriod::new(periods-period, rate_of_return, future_value, period_value, present_value));
    }

    PresentValue::new(rate_of_return, periods, present_value, future_value, period_values)
}

#[derive(Debug)]
pub struct PresentValuePeriod {
    period: u16,
    rate: f64,
    future_value: f64,
    period_value: f64,
    present_value: f64,
}
impl PresentValuePeriod {
    pub fn new(period: u16, rate: f64, future_value: f64, period_value: f64, present_value: f64) -> Self {
        Self {
            period,
            rate,
            future_value,
            period_value,
            present_value,

        }
    }
}
/// Return the Present Value of a future amount.
pub fn present_value_series(rate_of_return: f64, future_value: f64, periods: u16) -> Vec<PresentValuePeriod> {
    // assertions to ensure valid financial computation
    assert!(rate_of_return.is_finite());
    assert!(future_value.is_finite());
    assert!(future_value >= 0.);
    // final computation for returning a series of present values
    let interest_mult = 1. + rate_of_return;
    let present_value = future_value / (interest_mult).powi(periods as i32);
    let mut present_value_periods = vec![PresentValuePeriod::new(periods, rate_of_return, future_value, future_value, present_value)];
    for period in 1..=periods {
        let period_value = future_value / (interest_mult).powi(period as i32);
        present_value_periods.insert(0, PresentValuePeriod::new(periods-period, rate_of_return, future_value, period_value, present_value));
    }
    present_value_periods
}

#[derive(Debug)]
pub struct FutureValue {
    rate: f64,
    periods: u16,
    present_value: f64,
    future_value: f64,
}
impl FutureValue {
    pub fn new(rate: f64, periods: u16, present_value: f64, future_value: f64) -> Self {
        Self {
            rate,
            periods,
            present_value,
            future_value,
        }
    }
}
/// Returns a Future Value of a present amount.
pub fn future_value(interest_rate: f64, present_value: f64, periods: u16) -> FutureValue {
    // assertions to ensure valid financial computation
    assert!(interest_rate.is_finite());
    assert!(interest_rate >= 0.);
    assert!(present_value.is_finite());
    assert!(present_value >= 0.);
    // warning to ensure developer did not mistake rate with percentage
    if interest_rate > 1. { 
        warn!("You provided a rate ({}) greater than 1. Are you sure you expect a {}% return?", interest_rate, interest_rate*100.0); 
    }
    // final computation for future value
    let future_value = present_value * (1. + interest_rate).powi(periods as i32);
    FutureValue::new(interest_rate, periods, present_value, future_value)
}

#[derive(Debug)]
pub struct FutureValuePeriod {
    period: u16,
    rate: f64,
    present_value: f64,
    period_value: f64,
    future_value: f64,
}
impl FutureValuePeriod {
    pub fn new(period: u16, rate: f64, present_value: f64, period_value: f64, future_value: f64) -> Self {
        Self {
            period,
            rate,
            present_value,
            period_value,
            future_value,
        }
    }
}
/// Return a vector of future values for each period, starting with Period0 (present value) to Period_n (future value).
pub fn future_value_series(interest_rate: f64, present_value: f64, periods: u16) -> Vec<FutureValuePeriod> {
    // assertions to ensure valid financial computation
    assert!(interest_rate.is_finite());
    assert!(interest_rate >= 0.);
    assert!(present_value.is_finite());
    assert!(present_value >= 0.);
    // final computation for returning a series of future values
    let interest_mult = 1. + interest_rate;
    let future_value = present_value * interest_mult.powi(periods as i32);
    let mut v = vec![FutureValuePeriod::new(0, interest_rate, present_value, present_value, future_value)];
    for period in 1..=periods {
        let period_value = present_value * interest_mult.powi(period as i32);
        v.push(FutureValuePeriod::new(period, interest_rate, present_value, period_value, future_value));
    }
    v
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

#[derive(Debug)]
pub struct NetPresentValue {
    rate: f64,
    periods: u16,
    cashflows: Vec<f64>,
    present_value_cashflows: Vec<f64>,
    initial_investment: f64,
    sum_of_future_cashflows: f64,
    npv_of_future_cashflows: f64,
    npv: f64,
}
impl NetPresentValue {
    pub fn new(rate: f64, periods: u16, cashflows: Vec<f64>, present_value_cashflows: Vec<f64>, initial_investment: f64, sum_of_future_cashflows: f64, npv_of_future_cashflows: f64, npv: f64,) -> Self {
        Self {
            rate,
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
pub fn npv(initial_investment: f64, discount_rate: f64, periods: u16, cashflow: f64) -> NetPresentValue {
    // assertions to ensure valid financial computation
    assert!(initial_investment.is_finite());
    assert!(discount_rate.is_finite());
    assert!(periods > 0);
    assert!(cashflow.is_finite());
    // warning to ensure developer did not mistake rate with percentage
    if initial_investment > 0. { 
        warn!("You used a positive initial investment amount (your cf0 = {}). Are you sure?", initial_investment);
    }
    let mut cashflows = vec![initial_investment];
    let mut sum_future_cashflows = 0.;
    for _i in 1..periods {
        sum_future_cashflows += cashflow;
        cashflows.push(cashflow);
    }
    
    let mut npv_future_cashflows = 0.;
    let mut present_value_cashflows = vec![];
    for t in 1..=periods {
        let present_value_cashflow = cashflow / (1.0 + discount_rate).powi(t as i32);
        assert!(present_value_cashflow.is_finite());
        npv_future_cashflows += present_value_cashflow;
        assert!(npv_future_cashflows.is_finite());
        present_value_cashflows.push(present_value_cashflow);
    }
    let npv = initial_investment + npv_future_cashflows;
    NetPresentValue::new(discount_rate, periods, cashflows, present_value_cashflows, initial_investment, sum_future_cashflows, npv_future_cashflows, npv)
}

#[derive(Debug)]
pub struct NpvPeriod {
    period: u16,
    initial_investment: f64, 
    rate: f64, 
    cash_flow: f64,
    present_value: f64,
}
impl NpvPeriod {
   pub fn new(period: u16, initial_investment: f64, rate: f64, cash_flow: f64, present_value: f64) -> Self {
    Self {
        period,
        initial_investment, 
        rate, 
        cash_flow,
        present_value,
    }
   }
}
/// Return the Net Present Value (NPV) of a constant cash flow (cf_1..cf_n are all equal). 
pub fn npv_series(initial_investment: f64, discount_rate: f64, n: u16, cash_flow: f64) -> Vec<NpvPeriod> {
    // assertions to ensure valid financial computation
    assert!(initial_investment.is_finite());
    assert!(discount_rate.is_finite());
    assert!(n > 0);
    assert!(cash_flow.is_finite());
    // warning to ensure developer did not mistake rate with percentage
    if initial_investment > 0. { 
        warn!("You used a positive initial investment amount (your cf0 = {}). Are you sure? The fn npv() will sum the NPV of cash flows.", initial_investment);
        info!(target: "npv", "This function expects your initial investment (cf0) to be a positive number when it is a cash outflow. The formula will subtract cf0 from the sum(npv of cashflows). cf0 should not be negative unless you actually receive cash inflow in cf0, which is a rare financial situation.");
    }
    
    // final computation for NPV with constant cashflow (cf1..cf_n is the same)
    let npv_period = NpvPeriod::new(0, initial_investment, discount_rate, initial_investment, initial_investment);
    
    let mut cashflows = vec![npv_period];
    for t in 1..=n {
        let pv = cash_flow / (1.0 + discount_rate).powi(t as i32);
        assert!(pv.is_finite());
        cashflows.push(NpvPeriod::new(t, initial_investment, discount_rate, cash_flow, pv));
    }
    cashflows
}

#[derive(Debug)]
pub struct NpvVarRate {
    rate: Vec<f64>,
    periods: u16,
    cashflows: Vec<f64>,
    present_value_cashflows: Vec<f64>,
    initial_investment: f64,
    sum_future_cashflows: f64,
    npv_future_cashflows: f64,
    npv: f64,
}
impl NpvVarRate {
    pub fn new(rate: Vec<f64>, periods: u16, cashflows: Vec<f64>, present_value_cashflows: Vec<f64>, initial_investment: f64, sum_future_cashflows: f64, npv_future_cashflows: f64, npv: f64,) -> Self {
        Self {
            rate,
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
        if r > &1. { 
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
    rate: f64,
    periods: u16,
    cashflows: Vec<f64>,
    present_value_cashflows: Vec<f64>,
    initial_investment: f64,
    sum_future_cashflows: f64,
    npv_future_cashflows: f64,
    npv: f64,
}
impl NpvVarCashflow {
    pub fn new(rate: f64, periods: u16, cashflows: Vec<f64>, present_value_cashflows: Vec<f64>, initial_investment: f64, sum_future_cashflows: f64, npv_future_cashflows: f64, npv: f64,) -> Self {
        Self {
            rate,
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
pub fn npv_var_cfs(rate: f64, cashflows: &[f64]) -> NpvVarCashflow {
    let initial_investment = cashflows[0];
    let n = cashflows.len()-1;
    // assertions to ensure valid financial computation
    assert!(rate.is_finite());
    assert!(n > 0);
    for cf in cashflows {
        assert!(cf.is_finite());
    }
    // warning to ensure developer did not mistake rate with cf0
    if initial_investment > 0. { 
        warn!("You used a positive initial investment amount (your cf0 = {}). Are you sure?", initial_investment);
    }
    
    // final computation for NPV with variable cashflow (cf1..cf_n can vary)
    let mut npv_future_cashflows = 0.0f64;
    let mut present_value_cashflows = vec![initial_investment];
    for t in 1..=n {
        let present_value_cashflow = cashflows[t] / (1.0 + rate).powi(t as i32);
        assert!(present_value_cashflow.is_finite());
        present_value_cashflows.push(present_value_cashflow);
        npv_future_cashflows += present_value_cashflow;
        assert!(npv_future_cashflows.is_finite());
    }
    
    let future_cashflows: Vec<f64> = cashflows[1..].to_vec();
    let all_cashflows = cashflows.to_vec();

    NpvVarCashflow::new(rate, n as u16, all_cashflows, present_value_cashflows, cashflows[0], future_cashflows.iter().sum(), npv_future_cashflows, npv_future_cashflows + initial_investment)
}

#[derive(Debug)]
pub struct NpvVarRatesCfs {
    rate: Vec<f64>,
    periods: u16,
    cashflows: Vec<f64>,
    present_value_cashflows: Vec<f64>,
    initial_investment: f64,
    sum_future_cashflows: f64,
    npv_future_cashflows: f64,
    npv: f64,
}
impl NpvVarRatesCfs {
    pub fn new(rate: Vec<f64>, periods: u16, cashflows: Vec<f64>, present_value_cashflows: Vec<f64>, initial_investment: f64, sum_future_cashflows: f64, npv_future_cashflows: f64, npv: f64,) -> Self {
        Self {
            rate,
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
pub fn npv_var_rates_cfs(rates: &[f64], cashflows: &[f64]) -> NpvVarRatesCfs {
    // the first cashflows[0] should be the initial investment at period0, but the rates should be for periods 1..n  
    let num_periods = rates.len();
    let num_cfs = cashflows.len();
    assert_eq!(num_periods, num_cfs-1);
    assert!(num_periods > 0);
    assert!(num_cfs > 0);
    for cf in cashflows {
        assert!(cf.is_finite());
    }
    let mut large_rate_check = false;
    for r in rates {
        assert!(r.is_finite());
        if r > &1. { 
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
        let present_value_cashflow = cashflows[t] / (1.0 + rates[t-1]).powi(t as i32);
        assert!(present_value_cashflow.is_finite());
        present_value_cashflows.push(present_value_cashflow);
        npv_future_cashflows += present_value_cashflow;
        assert!(npv_future_cashflows.is_finite());
    }
    
    let future_cashflows: Vec<f64> = cashflows[1..].to_vec();
    let all_cashflows = cashflows.to_vec();
    let all_rates = rates.to_vec();

    NpvVarRatesCfs::new(all_rates, num_periods as u16, all_cashflows, present_value_cashflows, cashflows[0], future_cashflows.iter().sum(), npv_future_cashflows, npv_future_cashflows + initial_investment)
}



pub fn round_to_fraction_of_cent(val: f64) -> f64 {
    (val * 10_000.0).round() / 10_000.0
}

pub fn round_to_cent(val: f64) -> f64 {
    (val * 100.0).round() / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_future_value_1() {
        let rate_of_return = 0.034;
        let present_value_1 = 250_000.0;
        let periods = 5;
        let expected_value = 295489.941778856;
        let actual_value = future_value(rate_of_return, present_value_1, periods);
        assert_eq!(round_to_cent(expected_value), round_to_cent(actual_value));
        assert!( float_cmp::approx_eq!(f64, expected_value, actual_value, ulps = 4) );
    }

    #[test]
    fn test_future_value_2() {
        let rate_of_return = 0.08;
        let present_value_1 = 13_000.0;
        let periods = 6;
        let expected_value = 20_629.37;
        let actual_value = future_value(rate_of_return, present_value_1, periods);
        assert_eq!(round_to_cent(expected_value), round_to_cent(actual_value));
        // assert!(exp_value.approx_eq(act_value, (0.0, 2)));

    }

    #[test]
    fn test_present_value_1() {
        let rate_of_return = 0.08;
        let future_value = 20_629.37;
        let periods = 6;
        let expected_value = 13_000.0;
        let actual_value = present_value(rate_of_return, future_value, periods);
        assert_eq!(round_to_cent(expected_value), round_to_cent(actual_value));
    }

    #[test]
    fn test_present_value_2() {
        let rate_of_return = 0.08;
        let future_value = 20_629.37;
        let periods = 6;
        let expected_value = 13_000.0;
        let actual_value = present_value(rate_of_return, future_value, periods);
        assert_eq!(round_to_cent(expected_value), round_to_cent(actual_value));
    }

}

