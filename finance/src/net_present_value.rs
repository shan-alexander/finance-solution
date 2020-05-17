//! **Net Present Value calculations**. Given a series of cashflows, an initial investment (the cashflow at time0), a number of periods such as years, and fixed
//! or varying interest rates, what is the net value of the series of cashflows right now?
//!
//! For most common usages, we recommend the [`net_present_value_schedule_solution`](./fn.net_present_value_schedule_solution.html) function, to provide a better debugging experience and additional features.
//! This function allows you to provide varying cashflows and/or varying rates.
//! 
//! For very simple NPV calculations involving a constant cashflow and constant rate, the [`net_present_value_solution`](./fn.net_present_value_solution.html) function can be used.
//! 
//! ## Examples
//! 
//! **Simple Usage:**
//! ```
//! # use finance::net_present_value_solution;
//! let (rate, periods, initial_investment, cashflow) = (0.034, 3, -1000, 400);
//! let npv = net_present_value_solution(rate, periods, initial_investment, cashflow);
//! dbg!(npv.print_table());
//! ```
//! > outputs to terminal:
//! ```text
//! period   rate   present_value  future_value  investment_value 
//! ------  ------  -------------  ------------  ---------------- 
//! 0       0.0000    -1_000.0000   -1_000.0000       -1_000.0000 
//! 1       0.0340       386.8472      400.0000         -613.1528 
//! 2       0.0340       374.1269      400.0000         -239.0259 
//! 3       0.0340       361.8248      400.0000          122.7989 
//! ```
//! 
//! **More typical usage (varying cashflows):**
//! ```
//! # use finance::net_present_value_schedule_solution;
//! let rates = vec![0.034, 0.034, 0.034];
//! let cashflows = vec![-1000, 300, 400, 500];
//! let npv = net_present_value_schedule_solution(&rates, &cashflows);
//! dbg!(npv.print_table());
//! ```
//! > outputs to terminal:
//! ```text
//! period   rate   present_value  future_value  investment_value 
//! ------  ------  -------------  ------------  ---------------- 
//! 0       0.0000    -1_000.0000   -1_000.0000       -1_000.0000 
//! 1       0.0340       290.1354      300.0000         -709.8646 
//! 2       0.0340       374.1269      400.0000         -335.7377 
//! 3       0.0340       452.2810      500.0000          116.5433 
//! ```


// use crate::tvm_cashflow::*;
// Needed for the Rustdoc comments.
#[allow(unused_imports)]
use crate::present_value_annuity::present_value_annuity;
use crate::*;

use std::ops::Deref;

/// Returns the net present value of a future series of constant cashflows and constant rate, subtracting the initial investment cost. Returns f64.
///
/// Related functions:
/// * To calculate a net present value with a varying rate or varying cashflow or both, use [`net_present_value_schedule`].
///
/// The net present value annuity formula is:
///
/// npv = initial_investment + sum( cashflow / (1 + rate)<sup>period</sup> )
/// 
/// or
/// 
/// npv = initial_investment +  cashflow * ((1. - (1. / (1. + rate)).powf(periods)) / rate)
///
/// # Arguments
/// * `rate` - The rate at which the investment grows or shrinks per period,
/// expressed as a floating point number. For instance 0.05 would mean 5%. Often appears as
/// `r` or `i` in formulas.
/// * `periods` - The number of periods such as quarters or years. Often appears as `n` or `t`.
/// * `cashflow` - The value of the constant cashflow (aka payment).
/// * `initial investment` - The value of the initial investment (should be negative, or 0).
///
/// # Panics
/// The call will fail if `initial_investment` is positive. This value should always be negative, and cashflows be positive, or the reverse, because these monies are going opposite directions.
///
/// # Examples
/// Net Present Value of a series of -$1000 investment which will payback $500 yearly for 10 years.
/// ```
/// use finance::*;
/// let (rate, periods, initial_investment, cashflow) = (0.034, 10, -1000, 500);
///
/// // Find the present value of this scenario.
/// let net_present_value = net_present_value(rate, periods, initial_investment, cashflow);
///
/// // Confirm that the present value is correct to four decimal places (one hundredth of a cent).
/// assert_approx_equal!(3179.3410288, net_present_value);
/// ```
pub fn net_present_value<C, I>(rate: f64, periods: u32, initial_investment: I, cashflow: C) -> f64 
where I: Into<f64> + Copy, C: Into<f64> + Copy
{
    let annuity = cashflow.into();
    let ii = initial_investment.into();
    let pv_cashflow = annuity * ((1. - (1. / (1. + rate)).powf(periods as f64)) / rate);
    let npv = ii + pv_cashflow;
    npv
}

/// Returns the net present value of a future series of constant cashflows and constant rate, subtracting the initial investment cost. Returns a solution struct with additional features..
///
/// Related functions:
/// * To calculate a net present value with a varying rate or varying cashflow or both, use [`net_present_value_schedule`].
///
/// The net present value annuity formula is:
///
/// npv = initial_investment + sum( cashflow / (1 + rate)<sup>period</sup> )
/// 
/// or
/// 
/// npv = initial_investment +  cashflow * ((1. - (1. / (1. + rate)).powf(periods)) / rate)
///
/// # Arguments
/// * `rate` - The rate at which the investment grows or shrinks per period,
/// expressed as a floating point number. For instance 0.05 would mean 5%. Often appears as
/// `r` or `i` in formulas.
/// * `periods` - The number of periods such as quarters or years. Often appears as `n` or `t`.
/// * `cashflow` - The value of the constant cashflow (aka payment).
/// * `initial investment` - The value of the initial investment (should be negative, or 0).
pub fn net_present_value_solution<C, I>(rate: f64, periods: u32, initial_investment: I, cashflow: C) -> NpvSolution 
where I: Into<f64> + Copy, C: Into<f64> + Copy
{
    let annuity = cashflow.into();
    let ii = initial_investment.into();
    let rates = repeating_vec![rate, periods];
    let mut cashflows = repeating_vec![annuity, periods];
    cashflows.insert(0, ii);
    net_present_value_schedule_solution(&rates, &cashflows)

}

/// Returns the net present value of a schedule of rates and cashflows (can be varying), subtracting the initial investment cost. Returns f64.
///
/// # Examples
/// Net Present Value of a series of -$1000 investment which will payback $500 yearly for 10 years.
/// ```
/// use finance::*;
/// let (rates, cashflows) = (vec![0.034, 0.089, 0.055], vec![-1000, 200, 300, 500]);
///
/// // Find the present value of this scenario.
/// let net_present_value = net_present_value_schedule(&rates, &cashflows);
///
/// // Confirm that the present value is correct to four decimal places (one hundredth of a cent).
/// assert_approx_equal!(-127.8016238, net_present_value);
/// 
/// // present_value(0.034, 1, 200): $193.42
/// // present_value(0.089, 2, 300): $252.97
/// // present_value(0.055, 3, 500): $425.81
/// // initial investment:          -$1000
/// // sum of the above:            -$127.80 (net present value)
/// 
/// ```
pub fn net_present_value_schedule<C>(rates: &[f64], cashflows: &[C]) -> f64 
where C: Into<f64> + Copy
{
    let (periods, r, c, initial_investment) = check_schedule(rates, cashflows);
    // let mut cflows = vec![];
    // for i in 0..cashflows.len() {
    //     cflows.push(cashflows[i].into());
    // }
    // let cashflows = &cflows;
    // assert!(cashflows[0] <= 0.0, "The initial investment (cashflows[0]) should be negative or zero");
    // assert!(cashflows.len() >= 2, "Must provide at least 2 values in cashflows, the initial investment at the 0 position and the following cashflows, or a single cashflow representing a repeating constant cashflow.");
    // assert!(rates.len() >= 1, "Must provide at least 1 rate.");
    // let rate_length = rates.len();
    // let cashflow_length = cashflows.len();
    // let initial_investment = cashflows[0];
    // let mut cashflow_vec = vec![initial_investment];
    // let mut rate_vec = vec![];
    // let periods: u32;
    // let r: &[f64];
    // let c: &[f64];

    // if rate_length == 1 && cashflow_length == 2 {
    //     r = &rates;
    //     c = &cashflows;
    //     periods = 1_u32;
    // } else if rate_length > 1 && cashflow_length > 2 {
    //     r = &rates;
    //     c = &cashflows;
    //     periods = rate_length as u32;
    // } else if rate_length > 1 && cashflow_length == 2 {
    //     r = &rates;
    //     periods = rate_length as u32;
    //     for _i in 0..periods {
    //         cashflow_vec.push(cashflows[1])
    //     }
    //     c = &cashflow_vec;
    // } else if rate_length == 1 && cashflow_length > 2 {
    //     c = &cashflows;
    //     periods = cashflow_length as u32 - 1;
    //     for _i in 0..periods {
    //         rate_vec.push(rates[0])
    //     }
    //     r = &rate_vec;
    // } else {
    //     // revise this panic message
    //     panic!("At least rates or cashflows for net_present_value_schedule must provide the full series of inputs. Only one input can be a shorthand expression of a repeating input. If both are repeating constant inputs, use the net_present_value function.");
    // }

    let mut pv_accumulator = 0_f64;
    for i in 0..periods { 
        let present_value = present_value(r[i as usize], (i+1) as u32, c[i as usize + 1]);
        pv_accumulator = pv_accumulator + present_value;
    }
    let npv = initial_investment + pv_accumulator;
    npv
}

fn check_schedule<C>(rates:&[f64], cashflows: &[C]) -> (u32, Vec<f64>, Vec<f64>, f64) 
where C: Into<f64> + Copy
{
    let mut cflows = vec![];
    for i in 0..cashflows.len() {
        cflows.push(cashflows[i].into());
    }
    let cashflows = &cflows;
    assert!(cashflows[0] <= 0.0, "The initial investment (cashflows[0]) should be negative or zero");
    assert!(cashflows.len() >= 2, "Must provide at least 2 values in cashflows, the initial investment at the 0 position and the following cashflows, or a single cashflow representing a repeating constant cashflow.");
    assert!(rates.len() >= 1, "Must provide at least 1 rate.");
    let rate_length = rates.len();
    let cashflow_length = cashflows.len();
    let initial_investment = cashflows[0];
    let mut cashflow_vec = vec![initial_investment];
    let mut rate_vec = vec![];
    let periods: u32;
    let r: &[f64];
    let c: &[f64];

    if rate_length == 1 && cashflow_length == 2 {
        r = &rates;
        c = &cashflows;
        periods = 1_u32;
    } else if rate_length > 1 && cashflow_length > 2 {
        r = &rates;
        c = &cashflows;
        periods = rate_length as u32;
    } else if rate_length > 1 && cashflow_length == 2 {
        r = &rates;
        periods = rate_length as u32;
        for _i in 0..periods {
            cashflow_vec.push(cashflows[1])
        }
        c = &cashflow_vec;
    } else if rate_length == 1 && cashflow_length > 2 {
        c = &cashflows;
        periods = cashflow_length as u32 - 1;
        for _i in 0..periods {
            rate_vec.push(rates[0])
        }
        r = &rate_vec;
    } else {
        // revise this panic message
        panic!("At least rates or cashflows for net_present_value_schedule must provide the full series of inputs. Only one input can be a shorthand expression of a repeating input. If both are repeating constant inputs, use the net_present_value function.");
    }
    (periods, r.to_vec(), c.to_vec(), initial_investment)
}


/// Returns the net present value of a schedule of rates and cashflows (can be varying), subtracting the initial investment cost. 
/// Returns a custom solution struct with detailed information and additional functionality.
/// 
/// # Example
/// ```
/// let rates = vec![0.034, 0.034, 0.034];
/// let cashflows = vec![-1000, 300, 400, 500];
/// let npv = finance::net_present_value_schedule_solution(&rates, &cashflows);
/// dbg!(npv.print_table());
/// ```
/// > outputs to terminal:
/// ```text
/// period   rate   present_value  future_value  investment_value 
/// ------  ------  -------------  ------------  ---------------- 
/// 0       0.0000    -1_000.0000   -1_000.0000       -1_000.0000 
/// 1       0.0340       290.1354      300.0000         -709.8646 
/// 2       0.0340       374.1269      400.0000         -335.7377 
/// 3       0.0340       452.2810      500.0000          116.5433 
/// ```
pub fn net_present_value_schedule_solution<C>(rates: &[f64], cashflows: &[C]) -> NpvSolution 
where C: Into<f64> + Copy
{
    let (periods, rates, cashflows, initial_investment) = check_schedule(rates, cashflows);

    let mut sum_accumulator = 0_f64;
    let mut pv_accumulator = 0_f64;
    for i in 0..periods { 
        let present_value = present_value(rates[i as usize], (i+1) as u32, cashflows[i as usize + 1]);
        pv_accumulator = pv_accumulator + present_value;
        sum_accumulator = sum_accumulator + cashflows[i as usize + 1];
    }
    let sum_of_cashflows = sum_accumulator;
    let sum_of_discounted_cashflows = pv_accumulator;
    let net_present_value = initial_investment + pv_accumulator;

    NpvSolution::new(rates, periods, initial_investment, cashflows, sum_of_cashflows, sum_of_discounted_cashflows, net_present_value)
}

/// The custom solution information of a NPV scenario. 
/// The struct values are immutable by the user of the library.
#[derive(Debug)]
pub struct NpvSolution {
    rates: Vec<f64>,
    periods: u32,
    cashflows: Vec<f64>,
    initial_investment: f64,
    sum_of_cashflows: f64,
    sum_of_discounted_cashflows: f64,
    net_present_value: f64,
}
impl NpvSolution {
    /// Create a new instance of the struct
    pub fn new(
        rates: Vec<f64>, 
        periods: u32, 
        initial_investment: f64, 
        cashflows: Vec<f64>, 
        sum_of_cashflows: f64, 
        sum_of_discounted_cashflows:f64,
        net_present_value:f64) -> Self {
            Self {
                rates, 
                periods, 
                initial_investment, 
                cashflows, 
                sum_of_cashflows, 
                sum_of_discounted_cashflows,
                net_present_value,
            }
    }

    pub fn series(&self) -> NpvSeries {
         net_present_value_schedule_series(self)
    }

    /// Call `rate_avg` on a NpvSolution to get the simple average rate of a schedule;
    pub fn rate_avg(&self) -> f64 {
        let mut rate_accumulator = 0_f64;
        for r in &self.rates {
            rate_accumulator = rate_accumulator + r;
        }
        rate_accumulator / self.periods as f64
    }

    /// Returns the rate schedule
    pub fn rates(&self) -> &[f64] {
        &self.rates
    }
    /// Returns the number of periods as u32.
    pub fn periods(&self) -> u32 {
        self.periods
    }
    /// Returns the initial investment as f64.
    pub fn initial_investment(&self) -> f64 {
        self.initial_investment
    }
    /// Returns a Vec<f64> of the cashflows.
    pub fn cashflows(&self) -> &[f64] {
        &self.cashflows
    }
    /// Returns the sum of the cashflows at their future value.
    pub fn sum_of_cashflows(&self) -> f64 {
        self.sum_of_cashflows
    }
    /// Returns the sum of the cashflows at their present value.
    pub fn sum_of_discounted_cashflows(&self) -> f64 {
        self.sum_of_discounted_cashflows
    }
    /// Returns the net present value as f64.
    pub fn net_present_value(&self) -> f64 {
        self.net_present_value
    }
    /// Alias for net_present_value()
    pub fn npv(&self) -> f64 {
        self.net_present_value
    }

    /// Pretty-print a table of the calculations at each period for visual analysis. 
    pub fn print_table(&self) {
        self.series().print_table();
    }

    /// Pretty-print a table of the calculations at each period for visual analysis, and provide a Locale for monetary formatting and preferred decimal precision.
    pub fn print_table_locale(&self, locale: &num_format::Locale, precision: usize) {
        self.series().print_table_locale(locale, precision);
    }

    /// Return the max discounted cashflow (present value of the cashflow)
    pub fn max_discounted_cashflow(&self) -> f64 {
        self.series().max_discounted_cashflow() 
    }
    /// Return the min discounted cashflow (present value of the cashflow)
    pub fn min_discounted_cashflow(&self) -> f64 {
        self.series().min_discounted_cashflow() 
    }

}

#[derive(Debug)]
pub struct NpvSeries(Vec<NpvPeriod>);
impl NpvSeries {
    pub(crate) fn new(series: Vec<NpvPeriod>) -> Self {
        Self {
            0: series,
        }
    }
    pub fn filter<P>(&self, predicate: P) -> Self
        where P: Fn(&&NpvPeriod) -> bool
    {
        Self {
            0: self.iter().filter(|x| predicate(x)).map(|x| x.clone()).collect()
        }
    }

    pub fn print_table(&self) {
        self.print_table_locale_opt(None, None);
    }

    pub fn print_table_locale(&self, locale: &num_format::Locale, precision: usize) {
        self.print_table_locale_opt(Some(locale), Some(precision));
    }

    fn print_table_locale_opt(&self, locale: Option<&num_format::Locale>, precision: Option<usize>) {
        let columns = vec![("period", "i", true), ("rate", "f", true), ("present_value", "f", true), ("future_value", "f", true), ("investment_value", "f", true)];
        let data = self.iter()
            .map(|entry| vec![entry.period.to_string(), entry.rate.to_string(), entry.present_value.to_string(), entry.future_value.to_string(), entry.investment_value.to_string()])
            .collect::<Vec<_>>();
        print_table_locale_opt(&columns, data, locale, precision);
    }

    pub fn print_ab_comparison(&self, other: &NpvSeries) {
        self.print_ab_comparison_locale_opt(other, None, None);
    }

    pub fn print_ab_comparison_locale(&self, other: &NpvSeries, locale: &num_format::Locale, precision: usize) {
        self.print_ab_comparison_locale_opt(other, Some(locale), Some(precision));
    }

    fn print_ab_comparison_locale_opt (&self, other: &NpvSeries, locale: Option<&num_format::Locale>, precision: Option<usize>) {
        let columns = vec![("period", "i", true),
                           ("rate_a", "f", true), ("rate_b", "f", true),
                           ("present_value_a", "f", true), ("present_value_b", "f", true),
                           ("future_value_a", "f", true), ("future_value_b", "f", true),
                           ("investment_value_a", "f", true), ("investment_value_b", "f", true)];
        let mut data = vec![];
        let rows = max(self.len(), other.len());
        for row_index in 0..rows {
            data.push(vec![
                row_index.to_string(),
                self.get(row_index).map_or("".to_string(), |x| x.rate.to_string()),
                other.get(row_index).map_or("".to_string(), |x| x.rate.to_string()),
                self.get(row_index).map_or("".to_string(), |x| x.present_value.to_string()),
                other.get(row_index).map_or("".to_string(), |x| x.present_value.to_string()),
                self.get(row_index).map_or("".to_string(), |x| x.future_value.to_string()),
                other.get(row_index).map_or("".to_string(), |x| x.future_value.to_string()),
                self.get(row_index).map_or("".to_string(), |x| x.investment_value.to_string()),
                other.get(row_index).map_or("".to_string(), |x| x.investment_value.to_string()),
            ]);
        }
        print_table_locale_opt(&columns, data, locale, precision);
    }

    /// Return the max discounted cashflow (present value of the cashflow)
    pub fn max_discounted_cashflow(&self) -> f64 {
        assert!(self.len() > 1); 
        self.iter().skip(1).fold(std::f64::MIN, |acc, x| acc.max(x.present_value()))
    }

    /// Return the min discounted cashflow (present value of the cashflow)
    pub fn min_discounted_cashflow(&self) -> f64 {
        assert!(self.len() > 1); 
        self.iter().skip(1).fold(std::f64::MAX, |acc, x| acc.min(x.present_value()))
    }
}
impl Deref for NpvSeries {
    type Target = Vec<NpvPeriod>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Debug)]
pub struct NpvPeriod {
    period: u32,
    rate: f64,
    present_value: f64,
    future_value: f64,
    investment_value: f64,
    formula: String,
    formula_symbolic: String,
}
impl NpvPeriod {
    pub fn new(
        period: u32,
        rate: f64,
        present_value: f64,
        future_value: f64,
        investment_value: f64,
        formula: String,
        formula_symbolic: String,
    ) -> Self {
        Self {
            period,
            rate,
            present_value,
            future_value,
            investment_value,
            formula,
            formula_symbolic,
        }
    }
    /// Returns the period number. The first real period is 1 but there's also a period 0 which
    /// which shows the starting conditions.
    pub fn period(&self) -> u32 {
        self.period
    }

    /// Returns the periodic rate for the current period. If the containing struct is a
    /// [`TvmSolution`] every period will have the same rate. If it's a [`TvmSchedule`] each period
    /// may have a different rate.
    pub fn rate(&self) -> f64 {
        self.rate
    }

    /// Returns the present value of the cashflow.
    pub fn present_value(&self) -> f64 {
        self.present_value
    }

    /// Returns the future value of the cashflow.
    pub fn future_value(&self) -> f64 {
        self.future_value
    }
    
    /// Returns the investment value of the Npv scenario at the time of the current period.
    pub fn investment_value(&self) -> f64 {
        self.investment_value
    }

    /// Returns a text version of the formula used to calculate the value for the current period.
    /// The formula includes the actual values rather than variable names. For the formula with
    /// variables such as pv for present value call `formula_symbolic`.
    pub fn formula(&self) -> &str {
        &self.formula
    }

    /// Returns a text version of the formula used to calculate the value for the current period.
    /// The formula includes variables such as r for the rate. For the formula with actual values
    /// rather than variables call `formula`.
    pub fn formula_symbolic(&self) -> &str {
        &self.formula_symbolic
    }
}

pub(crate) fn net_present_value_schedule_series(schedule: &NpvSolution) -> NpvSeries {
    let mut series = vec![];
  
    let periods = schedule.periods();
    let mut investment_value = 0_f64;

    for period in 0..=periods {         
        let rate = if period == 0 {
            0.0
        } else {
            schedule.rates()[(period-1) as usize]
        };
        let future_value = schedule.cashflows[period as usize];
        let present_value = schedule.cashflows[period as usize] / (1. + rate).powf(period as f64);
        assert!(present_value.is_finite());
        investment_value += present_value;
        let formula = format!("{:.4} = {:.4} / (1 + {:.6})^{}", present_value, future_value, rate, period);
        let formula_symbolic = "present_value = fv / (1 + rate)^periods".to_string();
        series.push(NpvPeriod::new(period, rate, present_value, future_value, investment_value, formula, formula_symbolic))
    }
    NpvSeries::new(series)
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::*;

    #[test]
    fn test_net_present_value_1() {
        let rate = 0.034;
        let periods = 10;
        let ii = -1000;
        let cf = 500;
        let npv = net_present_value(rate, periods, ii, cf);
        assert_approx_equal!(3179.3410288, npv);
    }

    #[test]
    fn test_net_present_value_2() {
        let rate = 0.034;
        let periods = 400;
        let ii = -1000;
        let cf = 500;
        let npv = net_present_value(rate, periods, ii, cf);
        assert_eq!(13_705.85948, (100_000. * npv).round() / 100_000.);
    }

    #[test]
    fn test_net_present_value_3() {
        let rates = vec![0.034,0.089,0.055];
        let cashflows = vec![-1000,200,300,500];
        let npv = net_present_value_schedule(&rates, &cashflows);
        assert_eq!(-127.80162, (100_000. * npv).round() / 100_000.);
    }

    #[test]
    fn test_net_present_value_4() {
        let rates = vec![0.034,0.089,0.055];
        let cashflows = vec![-1000,200,300,500];
        let npv = net_present_value_schedule_solution(&rates, &cashflows);
        assert_eq!(-127.80162, (100_000. * npv.npv()).round() / 100_000.);
    }

    #[test]
    fn test_net_present_value_5() {
        // wildcard use case: positive and negatives
        let rates = vec![0.034,-0.0989,0.055,-0.02];
        let cashflows = vec![-1000,1000,500,-250,-250];
        let npv = net_present_value_schedule_solution(&rates, &cashflows);
        assert_eq!(98.950922304, (10_000_000_000. * npv.npv()).round() / 10_000_000_000.);
    }
}