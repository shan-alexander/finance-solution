//! **Net Present Value calculations**. Given a series of cashflows, an initial investment (the cashflow at time0), a number of periods such as years, and fixed
//! or varying interest rates, what is the net value of the series of cashflows right now?
//!

// use crate::tvm_cashflow::*;
// Needed for the Rustdoc comments.
#[allow(unused_imports)]
use crate::present_value_annuity::present_value_annuity;
use crate::*;

/// Returns the **net present value** of a future series of constant cashflows and constant rate, subtracting the initial investment cost. Returns f64.
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
    let pv_cashflow = present_value_annuity(rate, periods, annuity, false);
    let npv = ii + pv_cashflow;
    npv
}

/// Returns the **net present value of a schedule** of rates and cashflows (can be varying), subtracting the initial investment cost. Returns f64.
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
/// //present_value(0.034, 1, 200): $193.42
/// //present_value(0.089, 2, 300): $252.97
/// //present_value(0.055, 3, 500): $425.81
/// //initial investment:          -$1000
/// //sum of the above:            -$127.80 (net present value)
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

pub fn net_present_value_schedule_solution<C>(rates: &[f64], cashflows: &[C]) -> NetPresentValueSolution 
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

    NetPresentValueSolution::new(rates, periods, initial_investment, cashflows, sum_of_cashflows, sum_of_discounted_cashflows, net_present_value)
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
}

#[derive(Debug)]
pub struct NetPresentValueSolution {
    rates: Vec<f64>,
    periods: u32,
    cashflows: Vec<f64>,
    initial_investment: f64,
    sum_of_cashflows: f64,
    sum_of_discounted_cashflows: f64,
    net_present_value: f64,
}
impl NetPresentValueSolution {
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

    /// Call `rate_avg` on a NetPresentValueSolution to get the simple average rate of a schedule;
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
    /// Shortcut function for net_present_value()
    pub fn npv(&self) -> f64 {
        self.net_present_value
    }

}