#[cfg(test)]
mod tests {
    // use super::*;
    use finance::*;
    use std::fmt::Debug;

    #[test]
    fn test_symmetry_one() {
        let periodic_rate = 0.10;
        let periods = 4;
        let present_value = 5_000.00;
        check_symmetry(periodic_rate, periods, present_value);
    }

    fn check_symmetry(periodic_rate_in: f64, periods_in: u32, present_value_in: f64) {
        dbg!("test_symmetry_internal", periodic_rate_in, periods_in, present_value_in);

        // Calculate the future value given the other three inputs so that we have all four values
        // which we can use in various combinations to confirm that all four basic TVM functions
        // return consistent values.
        let future_value_calc = future_value(periodic_rate_in, periods_in, present_value_in);
        dbg!(future_value_calc);

        let periodic_rate_calc = rate(periods_in, present_value_in, future_value_calc);
        dbg!(periodic_rate_calc);
        assert_rounded_6(periodic_rate_calc, periodic_rate_in);

        let periods_calc = periods(periodic_rate_in, present_value_in, future_value_calc).ceil() as u32;
        dbg!(periods_calc);
        if periodic_rate_in == 0.0 {
            // In this case the present value and future value will be the same and periods() will
            // return zero since no periods are required.
            assert_rounded_4(present_value_in, future_value_calc);
            assert_eq!(0, periods_calc);
        } else if periodic_rate_in == -1.0 {
            // The investment will drop to zero by the end of the first period so periods() will
            // return 1.
            assert_rounded_4(0.0, future_value_calc);
            assert_eq!(1, periods_calc);
        } else {
            // This is the normal case and we expect periods() to return the same number of periods
            // we started with.
            assert_eq!(periods_calc, periods_in);
        }

        let present_value_calc = present_value(periodic_rate_in, periods_in, future_value_calc);
        dbg!(present_value_calc);
        assert_rounded_4(present_value_calc, present_value_in);

        // Create a list of rates that are all the same so that we can try the _schedule functions
        // For present value and future value
        let mut periodic_rates_in = vec![];
        for _ in 0..periods_in {
            periodic_rates_in.push(periodic_rate_in);
        }

        let present_value_schedule_calc = present_value_schedule(&periodic_rates_in, future_value_calc);
        dbg!(present_value_schedule_calc);
        assert_rounded_6(present_value_schedule_calc, present_value_in);

        let future_value_schedule_calc = future_value_schedule(&periodic_rates_in, present_value_in);
        dbg!(future_value_schedule_calc);
        assert_rounded_6(future_value_schedule_calc, future_value_calc);

        // Create TvmSolution structs by solving for each of the four possible variables.
        let solutions = [
            rate_solution(periods_in, present_value_in, future_value_calc),
            periods_solution(periodic_rate_in, present_value_in, future_value_calc),
            present_value_solution(periodic_rate_in, periods_in, future_value_calc),
            future_value_solution(periodic_rate_in, periods_in, present_value_in),
        ];
        for solution in solutions.iter() {
            dbg!(solution);
            assert_rounded_6(periodic_rate_in, solution.periodic_rate);
            if solution.calculated_field.is_periods() {
                // There are a few special cases in which the number of periods might be zero or one
                // instead of matching periods_in. So check against the number returned from
                // periods().
                assert_eq!(periods_calc, solution.periods);
            } else {
                assert_eq!(periods_in, solution.periods);
            }
            assert_eq!(periods_in, solution.periods);
            assert_rounded_4(present_value_in, solution.present_value);
            assert_rounded_4(future_value_calc, solution.future_value);
        }

        let schedules = [
            present_value_schedule_solution(&periodic_rates_in, future_value_calc),
            future_value_schedule_solution(&periodic_rates_in, present_value_in),
        ];
        for schedule in schedules.iter() {
            dbg!(schedule);
            assert_eq!(periods_in, schedule.periodic_rates.len() as u32);
            assert_eq!(periods_in, schedule.periods);
            assert_rounded_4(present_value_in, schedule.present_value);
            assert_rounded_4(future_value_calc, schedule.future_value);
        }
        
        // Check each series in isolation.
        for solution in solutions.iter() {
            check_series_internal(solution, periodic_rate_in, periods_in, present_value_in, future_value_calc, periods_calc);
        }
        for schedule in schedules.iter() {
            check_series_internal(schedule, periodic_rate_in, periods_in, present_value_in, future_value_calc, periods_calc);
        }

        // Confirm that all of the series have the same values for all periods regardless of how we
        // did the calculation.
        // For the reference solution take the result of future_value_solution(). It would also work
        // to use the result of rate_solution() and present_value_solution() but not
        // periods_solution() since there are some special cases in which this will create fewer
        // periods than the other functions.
        let reference_solution = solutions.iter().find(|x| x.calculated_field.is_future_value()).unwrap();
        for solution in solutions.iter().filter(|x| !x.calculated_field.is_future_value()) {
            check_series_same_values(reference_solution, solution);
        }
        for schedule in schedules.iter() {
            check_series_same_values(reference_solution, schedule);
        }

    }

    fn check_series_internal<T>(solution: &T, periodic_rate_in: f64, periods_in: u32, present_value_in: f64, future_value_calc: f64, periods_calc: u32)
        where T: TvmCalcSeries + Debug
    {
        dbg!(solution);
        let series = solution.series();
        dbg!(&series);
        if solution.calculated_field().is_periods() {
            // There are a few special cases in which the number of periods might be zero or one
            // instead of matching periods_in. So check against the number returned from
            // periods().
            assert_eq!(periods_calc + 1, series.len() as u32);
        } else {
            assert_eq!(periods_in + 1, series.len() as u32);
        }
        let mut prev_value = None;
        for (period, entry) in series.iter().enumerate() {
            assert_eq!(period as u32, entry.period);
            if period == 0 {
                assert_rounded_6(0.0, entry.rate);
                // The first entry should always contain the starting value.
                assert_rounded_4(present_value_in, entry.value);
            } else {
                // We're past period 0.
                assert_rounded_6(periodic_rate_in, entry.rate);
                // Compare this period's value to the one before.
                if periodic_rate_in == 0.0 {
                    // The rate is zero so each period's value should be the same as the one
                    // before.
                    assert_rounded_4(entry.value, prev_value.unwrap());
                } else if present_value_in.signum() == periodic_rate_in.signum() {
                    // Either the starting value and the rate are both positive or they're both
                    // negative. In either case each period's value should be greater than the one
                    // before.
                    assert!(entry.value > prev_value.unwrap());
                } else {
                    // Either the starting value is positive and the rate is negative or vice versa.
                    // In either case each period's value should be smaller than the one before.
                    assert!(entry.value < prev_value.unwrap());
                }
            }
            if period == series.len() - 1 {
                // This is the last period's entry. It should contain the future value.
                assert_rounded_4(future_value_calc, entry.value);
            }
            prev_value = Some(entry.value);
        }
    }

    fn check_series_same_values<T, U>(reference_solution: &T, solution: &U)
        where
            T: TvmCalcSeries + Debug,
            U: TvmCalcSeries + Debug
    {
        dbg!(reference_solution);
        let reference_series = reference_solution.series();
        dbg!(&reference_series);

        dbg!(solution);
        let series = solution.series();
        dbg!(&series);

        if solution.calculated_field().is_periods() && reference_series.len() != series.len() {

            // There are a few special cases in which the number of periods might be zero or one
            // instead of matching periods_in.

            // There will always be at least a period 0.
            let reference_entry = &reference_series[0];
            let entry = &series[0];
            assert_eq!(reference_entry.period, entry.period);
            assert_rounded_6(reference_entry.rate, entry.rate);
            assert_rounded_4(reference_entry.value, entry.value);

            // Check the last period.
            let reference_entry = &reference_series.last().unwrap();
            let entry = &series.last().unwrap();
            assert_eq!(reference_entry.period, entry.period);
            assert_rounded_6(reference_entry.rate, entry.rate);
            assert_rounded_4(reference_entry.value, entry.value);

        } else {

            // This is the usual case where we expect the two series to be identical except for
            // the formulas.

            assert_eq!(reference_series.len(), series.len());

            for (period, reference_entry) in reference_series.iter().enumerate() {
                let entry = &series[period];
                assert_eq!(reference_entry.period, entry.period);
                assert_rounded_6(reference_entry.rate, entry.rate);
                assert_rounded_4(reference_entry.value, entry.value);
            }
        }
    }

    /*
    #[test]
    fn test_symmetry_multiple() {
        let rates = vec![-1.0, -0.5, -0.05, -0.005, 0.0, 0.005, 0.05, 0.5, 1.0, 10.0, 100.0];
        let periods: Vec<u32> = vec![0, 1, 2, 5, 10, 36, 100, 1_000];
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

