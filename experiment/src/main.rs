mod fina;

fn main() {
    let fv_answer = fina::fv(3.4,250000.0,5.0);
    println!("{}", fv_answer);

    let fv_vec_answer = fv_vec(3.4,250000.0,5);
    println!("{:?}", fv_vec_answer);
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