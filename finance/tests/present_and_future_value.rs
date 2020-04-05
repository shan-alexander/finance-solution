#[cfg(test)]
mod tests {
    // use crate::*;

    #[test]
    fn present_and_future_value_show_running() {

    }
    /*
    #[test]
    fn test_present_future_fixed_rate() {
        // let rates = vec![-1.0, -0.5, -0.05, -0.005, 0.0, 0.005, 0.05, 0.5, 1.0, 10.0, 100.0];
        let rates = vec![-0.99999, -0.5, -0.05, -0.005, 0.0, 0.005, 0.05, 0.5, 1.0, 10.0, 100.0];
        // let periods: Vec<u32> = vec![0, 1, 2, 5, 10, 36, 100, 1_000];
        let periods: Vec<u32> = vec![1, 2, 5, 10, 36, 100, 1_000];
        let present_values: Vec<f64> = vec![-1_000_000.1234, -1_234.987654321, -1.0, 0.0, 5.555555555, 9_999_999_999_999.99];

        for rate in rates.iter() {
            for period in periods.iter() {
                for present_value in present_values.iter() {
                    let rate = *rate;
                    let period = *period;
                    let present_value = *present_value;
                    let result_future_value = future_value(rate, period, present_value);
                    assert!(result_future_value.is_finite());
                    if present_value == 0.0 {
                        assert_eq!(0.0, result_future_value);
                    } else if rate == 0.0 || period == 0 {
                        assert_f64(present_value, result_future_value);
                    } else if rate.signum() == present_value.signum() {
                        // The rate and present value are either both positive or both negative.
                        assert!(result_future_value > present_value);
                    } else {
                        assert!(result_future_value < present_value);
                    }

                    let result_future_value_solution = future_value_solution(rate, period, present_value);
                    assert_f64(present_value, result_future_value_solution.present_value);
                    assert_eq!(result_future_value, result_future_value_solution.future_value)
                }
            }
        }

    }
    */
}

