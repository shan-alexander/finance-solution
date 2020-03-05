mod fina;

fn main() {
    let starting_cash: f64 = 250000.0;
    let rate: f64 = 3.4;
    let years: u32 = 5;
    println!("Starting cash: {}", &starting_cash);
    let fv_answer = fina::fv(rate, starting_cash, years as f64);
    println!("Future value in {} years with {}% rate of return: {}", &years, &rate, &fv_answer);

    let fv_vec_answer = fv_vec(rate, starting_cash, years);
    println!("{:?}", &fv_vec_answer);

    let pv_answer = pv(rate, fv_answer, years);
    println!("Present Value of the future value answer: {}", pv_answer);

}

// shan's idea to make a secondary function of the FutureValue function which
// returns a Vec of each time period, and thus the final value in the Vec is 
// the final answer.
fn fv_vec(rate:f64,cf0:f64,num_of_period:u32) -> Vec<f64> {
    let mut period_vals: Vec<f64> = vec![];
    let mut current_val = cf0;
    for _i in 0..num_of_period {
        let period_val = (
            current_val * 
            (1_f64 + rate/100_f64) * 
            100_f64
            ).round() / 100_f64;
        current_val = period_val;
        period_vals.push(period_val);
    }
    period_vals
}

pub fn pv(rate:f64,cf1:f64, n:u32) -> f64{
    (cf1 / (1_f64 + rate/(100_f64)).powf(n.into()) ).round()
    }
