#[cfg(test)]
mod tests {
    // use super::*;
    use finance::*;

    #[test]
    fn test_symmetry_one() {
        let rate = 0.10;
        let periods = 4;
        let present_value = 5_000.00;
        check_symmetry(rate, periods, present_value);
    }

    #[test]
    fn test_symmetry_multiple() {
        let rates = vec![-1.0, -0.5, -0.05, -0.005, 0.0, 0.005, 0.05, 0.5, 1.0, 10.0, 100.0];
        // let rates = vec![-0.5, -0.05, -0.005, 0.0, 0.005, 0.05, 0.5, 1.0, 10.0, 100.0];
        // let periods: Vec<u32> = vec![0, 1, 2, 5, 10, 36, 100, 1_000];
        let periods: Vec<u32> = vec![0, 1, 2, 5, 10, 36];
        let present_values: Vec<f64> = vec![-1_000_000.0, -1_234.98, -1.0, 0.0, 5.55555, 99_999.99];
        for rate_one in rates.iter() {
            for periods_one in periods.iter() {
                for present_value_one in present_values.iter() {
                    if !(*periods_one > 50 && *rate_one > 0.01) {
                        check_symmetry(*rate_one, *periods_one, *present_value_one);
                    }
                }
            }
        }
    }

    fn check_symmetry(rate_in: f64, periods_in: u32, present_value_in: f64) {
        //bg!("test_symmetry_internal", rate_in, periods_in, present_value_in);

        // Calculate the future value given the other three inputs so that we have all four values
        // which we can use in various combinations to confirm that all four basic TVM functions
        // return consistent values.
        let future_value_calc = future_value(rate_in, periods_in, present_value_in);
        //bg!(future_value_calc);
        //bg!(future_value_calc.is_normal());

        let rate_calc = rate(periods_in, present_value_in, future_value_calc);
        //bg!(rate_calc);
        if periods_in == 0 || present_value_in == 0.0 {
            // With zero periods or zero for the present value, presumably the future value is the
            // same as the present value and any periodic rate would be fine so we arbitrarily
            // return zero.
            assert_approx_equal_symmetry_test!(present_value_in, future_value_calc);
            assert_approx_equal_symmetry_test!(0.0, rate_calc);
        } else {
            //bg!(rate_calc, rate_in);
            assert_approx_equal_symmetry_test!(rate_calc, rate_in);
        }

        let fractional_periods_calc = periods(rate_in, present_value_in, future_value_calc);
        //bg!(fractional_periods_calc);
        let periods_calc = round_4(fractional_periods_calc).ceil() as u32;
        //bg!(periods_calc);
        if rate_in == 0.0 || present_value_in == 0.0 || periods_in == 0 {
            // If the rate is zero or the present value is zero then the present value and future
            // value will be the same and periods() will return zero since no periods are required.
            assert_approx_equal_symmetry_test!(present_value_in, future_value_calc);
            assert_eq!(0, periods_calc);
        } else if rate_in == -1.0 {
            // The investment will drop to zero by the end of the first period so periods() will
            // return 1.
            assert_approx_equal_symmetry_test!(0.0, future_value_calc);
            assert_eq!(1, periods_calc);
        } else {
            // This is the normal case and we expect periods() to return the same number of periods
            // we started with.
            assert_eq!(periods_calc, periods_in);
        }

        if future_value_calc.is_normal() {
            let present_value_calc = present_value(rate_in, periods_in,future_value_calc);
            //bg!(present_value_calc);
            assert_approx_equal_symmetry_test!(present_value_calc, present_value_in);
        };

        // Create a list of rates that are all the same so that we can try the _schedule functions
        // For present value and future value
        let mut rates_in = vec![];
        for _ in 0..periods_in {
            rates_in.push(rate_in);
        }

        if future_value_calc.is_normal() {
            let present_value_schedule_calc = present_value_schedule(&rates_in, future_value_calc);
            //bg!(present_value_schedule_calc);
            assert_approx_equal_symmetry_test!(present_value_schedule_calc, present_value_in);
        }

        let future_value_schedule_calc = future_value_schedule(&rates_in, present_value_in);
        //bg!(future_value_schedule_calc);
        assert_approx_equal_symmetry_test!(future_value_schedule_calc, future_value_calc);

        // Create TvmSolution structs by solving for each of the four possible variables.
        let mut solutions = vec![
            rate_solution(periods_in, present_value_in, future_value_calc),
            periods_solution(rate_in, present_value_in, future_value_calc),
            future_value_solution(rate_in, periods_in, present_value_in),
        ];
        if future_value_calc.is_normal() {
            solutions.push(present_value_solution(rate_in, periods_in, future_value_calc));
        }
        for solution in solutions.iter() {
            //bg!(solution);
            if solution.calculated_field().is_rate() {
                // There are a few special cases in which the calculated rate is arbitrarily set to
                // zero since any value would work. We've already checked rate_calc against those
                // special cases, so use that here for the comparison.
                assert_approx_equal_symmetry_test!(rate_calc, solution.rate());
            } else {
                assert_approx_equal_symmetry_test!(rate_in, solution.rate());
            }
            if solution.calculated_field().is_periods() {
                // There are a few special cases in which the number of periods might be zero or one
                // instead of matching periods_in. So check against the number returned from
                // periods().
                assert_eq!(periods_calc, solution.periods());
            } else {
                assert_eq!(periods_in, solution.periods());
            }
            assert_approx_equal_symmetry_test!(present_value_in, solution.present_value());
            assert_approx_equal_symmetry_test!(future_value_calc, solution.future_value());
        }

        let mut schedules = vec![
            future_value_schedule_solution(&rates_in, present_value_in),
        ];
        if future_value_calc.is_normal() {
            schedules.push(present_value_schedule_solution(&rates_in, future_value_calc));
        }
        for schedule in schedules.iter() {
            //bg!(schedule);
            assert_eq!(periods_in, schedule.rates().len() as u32);
            assert_eq!(periods_in, schedule.periods());
            assert_approx_equal_symmetry_test!(present_value_in, schedule.present_value());
            assert_approx_equal_symmetry_test!(future_value_calc, schedule.future_value());
        }
        
        // Check each series in isolation.
        for solution in solutions.iter() {
            let label = format!("Solution for {:?}", solution.calculated_field());
            //bg!(&label);
            check_series_internal(label, solution.calculated_field().clone(), &solution.series(), rate_in, periods_in, present_value_in, future_value_calc, rate_calc, periods_calc);
        }
        for schedule in schedules.iter() {
            let label = format!("Schedule for {:?}", schedule.calculated_field());
            //bg!(&label);
            check_series_internal(label, schedule.calculated_field().clone(), &schedule.series(), rate_in, periods_in, present_value_in, future_value_calc, rate_calc, periods_calc);
        }

        // Confirm that all of the series have the same values for all periods regardless of how we
        // did the calculation.
        // For the reference solution take the result of future_value_solution(). It would also work
        // to use the result of rate_solution() and present_value_solution() but not
        // periods_solution() since there are some special cases in which this will create fewer
        // periods than the other functions.
        let reference_solution = solutions.iter().find(|x| x.calculated_field().is_future_value()).unwrap();
        for solution in solutions.iter().filter(|x| !x.calculated_field().is_future_value()) {
            let label = format!("Solution for {:?}", solution.calculated_field());
            check_series_same_values(reference_solution, label, solution.calculated_field().clone(), &solution.series());
        }
        for schedule in schedules.iter() {
            let label = format!("Schedule for {:?}", schedule.calculated_field());
            check_series_same_values(reference_solution, label, schedule.calculated_field().clone(), &schedule.series());
        }

    }

    fn check_series_internal(_label: String, calculated_field: TvmVariable, series: &[TvmPeriod], rate_in: f64, periods_in: u32, present_value_in: f64, future_value_calc: f64, rate_calc: f64, periods_calc: u32) {
        //bg!(label);
        //bg!(&series);
        if calculated_field.is_periods() {
            // There are a few special cases in which the number of periods might be zero or one
            // instead of matching periods_in. So check against the number returned from
            // periods().
            assert_eq!(periods_calc + 1, series.len() as u32);
        } else {
            assert_eq!(periods_in + 1, series.len() as u32);
        }
        let mut prev_value: Option<f64> = None;
        for (period, entry) in series.iter().enumerate() {
            assert_eq!(period as u32, entry.period());
            if period == 0 {
                assert_approx_equal_symmetry_test!(0.0, entry.rate());
                // The first entry should always contain the starting value.
                assert_approx_equal_symmetry_test!(present_value_in, entry.value());
            } else {
                // We're past period 0.
                let effective_rate = if calculated_field.is_rate() {
                    // There are a few special cases in which the calculated rate is arbitrarily set
                    // to zero since any value would work. We've already checked rate_calc against
                    // those special cases, so use that here for the comparison.
                    assert_approx_equal_symmetry_test!(rate_calc, entry.rate());
                    rate_calc
                } else {
                    assert_approx_equal_symmetry_test!(rate_in, entry.rate());
                    rate_in
                };
                // Compare this period's value to the one before.
                if effective_rate == 0.0 || prev_value.unwrap() == 0.0 {
                    // The rate is zero or the previous value was zero so each period's value should
                    // be the same as the one before.
                    assert_approx_equal_symmetry_test!(entry.value(), prev_value.unwrap());
                } else if present_value_in.signum() == effective_rate.signum() {
                    // Either the starting value and the rate are both positive or they're both
                    // negative. In either case each period's value should be greater than the one
                    // before.
                    assert!(entry.value() > prev_value.unwrap());
                } else {
                    // Either the starting value is positive and the rate is negative or vice versa.
                    // In either case each period's value should be smaller than the one before.
                    assert!(entry.value() < prev_value.unwrap());
                }
            }
            if period == series.len() - 1 {
                // This is the last period's entry. It should contain the future value.
                //bg!(future_value_calc, entry.value());
                assert_approx_equal_symmetry_test!(future_value_calc, entry.value());
            }
            prev_value = Some(entry.value());
        }
    }

    fn check_series_same_values(reference_solution: &TvmSolution, _label: String, calculated_field: TvmVariable, series: &[TvmPeriod]) {
        //bg!(reference_solution);
        let reference_series = reference_solution.series();
        //bg!(&reference_series);

        //bg!(label);
        //bg!(&series);

        if calculated_field.is_periods() && reference_series.len() != series.len() {

            // There are a few special cases in which the number of periods might be zero or one
            // instead of matching periods_in.

            // There will always be at least a period 0.
            let reference_entry = &reference_series[0];
            let entry = &series[0];
            //bg!(&reference_entry, &entry);
            assert_eq!(reference_entry.period(), entry.period());
            assert_approx_equal_symmetry_test!(reference_entry.rate(), entry.rate());
            assert_approx_equal_symmetry_test!(reference_entry.value(), entry.value());

            // Check the last period.
            let reference_entry = &reference_series.last().unwrap();
            let entry = &series.last().unwrap();
            //bg!(&reference_entry, &entry);
            if reference_series.len() > 1 && series.len() > 1 {
                assert_approx_equal_symmetry_test!(reference_entry.rate(), entry.rate());
            }
            assert_approx_equal_symmetry_test!(reference_entry.value(), entry.value());

        } else {

            // This is the usual case where we expect the two series to be identical except for
            // the formulas.

            assert_eq!(reference_series.len(), series.len());

            for (period, reference_entry) in reference_series.iter().enumerate() {
                let entry = &series[period];
                //bg!(&reference_entry, &entry);
                assert_eq!(reference_entry.period(), entry.period());
                if calculated_field.is_rate() {
                    // There are a few special cases where the calculated rate will be zero since
                    // any answer would work.
                    if entry.rate() != 0.0 {
                        assert_approx_equal_symmetry_test!(reference_entry.rate(), entry.rate());
                    }
                } else {
                    assert_approx_equal_symmetry_test!(reference_entry.rate(), entry.rate());
                }
                //bg!(reference_entry.value, round_2(reference_entry.value), entry.value, round_2(entry.value));
                // assert_approx_equal_symmetry_test!(reference_entry.value, entry.value);
                // assert_eq!(reference_entry.value.round(), entry.value.round());
            }
        }
    }

    #[test]
    fn test_continuous_symmetry_one() {
        let apr = 0.10;
        let years = 4;
        let present_value = 5_000.00;
        check_continuous_symmetry(apr, years, present_value);
    }

    /*
    #[test]
    fn test_symmetry_multiple() {
        let rates = vec![-1.0, -0.5, -0.05, -0.005, 0.0, 0.005, 0.05, 0.5, 1.0, 10.0, 100.0];
        // let rates = vec![-0.5, -0.05, -0.005, 0.0, 0.005, 0.05, 0.5, 1.0, 10.0, 100.0];
        // let periods: Vec<u32> = vec![0, 1, 2, 5, 10, 36, 100, 1_000];
        let periods: Vec<u32> = vec![0, 1, 2, 5, 10, 36];
        let present_values: Vec<f64> = vec![-1_000_000.0, -1_234.98, -1.0, 0.0, 5.55555, 99_999.99];
        for rate_one in rates.iter() {
            for periods_one in periods.iter() {
                for present_value_one in present_values.iter() {
                    if !(*periods_one > 50 && *rate_one > 0.01) {
                        check_symmetry(*rate_one, *periods_one, *present_value_one);
                    }
                }
            }
        }
    }
    */

    fn check_continuous_symmetry(apr_in: f64, years_in: u32, present_value_in: f64) {
        println!();
        dbg!("test_continuous_symmetry_internal", apr_in, years_in, present_value_in);

        /*
        let fv_calc = present_value_in * std::f64::consts::E.powf(apr_in * years_in as f64);
        dbg!(fv_calc);
        let pv_calc = fv_calc / std::f64::consts::E.powf(apr_in * years_in as f64);
        dbg!(pv_calc);
        */

        // Calculate the future value given the other three inputs so that we have all four values
        // which we can use in various combinations to confirm that all four continuous TVM
        // functions return consistent values.
        let future_value_calc = future_value_continuous(apr_in, years_in, present_value_in);
        dbg!(future_value_calc);

        let apr_calc = rate::apr_continuous(years_in, present_value_in, future_value_calc);
        dbg!(apr_calc);
        if years_in == 0 || present_value_in == 0.0 {
            // With zero years or zero for the present value, presumably the future value is the
            // same as the present value and any apr would be fine so we arbitrarily
            // return zero.
            assert_approx_equal_symmetry_test!(present_value_in, future_value_calc);
            assert_approx_equal_symmetry_test!(0.0, apr_calc);
        } else {
            dbg!(apr_calc, apr_in);
            assert_approx_equal_symmetry_test!(apr_calc, apr_in);
        }

        let fractional_years_calc = years_continuous(apr_in, present_value_in, future_value_calc);
        dbg!(fractional_years_calc);
        let years_calc = round_4(fractional_years_calc).ceil() as u32;
        dbg!(years_calc);
        if apr_in == 0.0 || present_value_in == 0.0 || years_in == 0 {
            // If the apr is zero or the present value is zero then the present value and future
            // value will be the same and years() will return zero since no years are required.
            assert_approx_equal_symmetry_test!(present_value_in, future_value_calc);
            assert_eq!(0, years_calc);
        } else if apr_in == -1.0 {
            // The investment will drop to zero by the end of the first period so years() will
            // return 1.
            assert_approx_equal_symmetry_test!(0.0, future_value_calc);
            assert_eq!(1, years_calc);
        } else {
            // This is the normal case and we expect years() to return the same number of years
            // we started with.
            assert_eq!(years_calc, years_in);
        }

        if future_value_calc.is_normal() {
            let present_value_calc = present_value_continuous(apr_in, years_in,future_value_calc);
            dbg!(present_value_calc);
            assert_approx_equal_symmetry_test!(present_value_calc, present_value_in);
        };

        // Create TvmSolution structs by solving for each of the four possible variables.
        let mut solutions = vec![
            apr_continuous_solution(years_in, present_value_in, future_value_calc),
            years_continuous_solution(apr_in, present_value_in, future_value_calc),
            future_value_continuous_solution(apr_in, years_in, present_value_in),
        ];
        if future_value_calc.is_normal() {
            solutions.push(present_value_continuous_solution(apr_in, years_in, future_value_calc));
        }
        for solution in solutions.iter() {
            dbg!(&solution);
            dbg!(&solution.series());
            if solution.calculated_field().is_rate() {
                // There are a few special cases in which the calculated apr is arbitrarily set to
                // zero since any value would work. We've already checked apr_calc against those
                // special cases, so use that here for the comparison.
                assert_approx_equal_symmetry_test!(apr_calc, solution.rate());
            } else {
                assert_approx_equal_symmetry_test!(apr_in, solution.rate());
            }
            if solution.calculated_field().is_periods() {
                // There are a few special cases in which the number of years might be zero or one
                // instead of matching years_in. So check against the number returned from
                // years().
                assert_eq!(years_calc, solution.periods());
            } else {
                assert_eq!(years_in, solution.periods());
            }
            assert_approx_equal_symmetry_test!(present_value_in, solution.present_value());
            assert_approx_equal_symmetry_test!(future_value_calc, solution.future_value());
        }

        /*
        let mut schedules = vec![
            future_value_schedule_solution(&aprs_in, present_value_in),
        ];
        if future_value_calc.is_normal() {
            schedules.push(present_value_schedule_solution(&aprs_in, future_value_calc));
        }
        for schedule in schedules.iter() {
            //bg!(schedule);
            assert_eq!(years_in, schedule.aprs().len() as u32);
            assert_eq!(years_in, schedule.years());
            assert_approx_equal_symmetry_test!(present_value_in, schedule.present_value());
            assert_approx_equal_symmetry_test!(future_value_calc, schedule.future_value());
        }

        // Check each series in isolation.
        for solution in solutions.iter() {
            let label = format!("Solution for {:?}", solution.calculated_field());
            //bg!(&label);
            check_series_internal(label, solution.calculated_field().clone(), &solution.series(), apr_in, years_in, present_value_in, future_value_calc, apr_calc, years_calc);
        }
        for schedule in schedules.iter() {
            let label = format!("Schedule for {:?}", schedule.calculated_field());
            //bg!(&label);
            check_series_internal(label, schedule.calculated_field().clone(), &schedule.series(), apr_in, years_in, present_value_in, future_value_calc, apr_calc, years_calc);
        }

        // Confirm that all of the series have the same values for all years regardless of how we
        // did the calculation.
        // For the reference solution take the result of future_value_solution(). It would also work
        // to use the result of apr_solution() and present_value_solution() but not
        // years_solution() since there are some special cases in which this will create fewer
        // years than the other functions.
        let reference_solution = solutions.iter().find(|x| x.calculated_field().is_future_value()).unwrap();
        for solution in solutions.iter().filter(|x| !x.calculated_field().is_future_value()) {
            let label = format!("Solution for {:?}", solution.calculated_field());
            check_series_same_values(reference_solution, label, solution.calculated_field().clone(), &solution.series());
        }
        for schedule in schedules.iter() {
            let label = format!("Schedule for {:?}", schedule.calculated_field());
            check_series_same_values(reference_solution, label, schedule.calculated_field().clone(), &schedule.series());
        }
        */

    }

}

