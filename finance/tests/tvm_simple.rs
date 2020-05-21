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
                        if !(*periods_one == 0 && *present_value_one != 0.0) {
                            check_symmetry(*rate_one, *periods_one, *present_value_one);
                        }
                    }
                }
            }
        }
    }

    fn check_symmetry(rate_in: f64, periods_in: u32, present_value_in: f64) {
        //bg!("check_symmetry", rate_in, periods_in, present_value_in);

        // Calculate the future value given the other three inputs so that we have all four values
        // which we can use in various combinations to confirm that all four basic TVM functions
        // return consistent values.
        let future_value_calc = future_value(rate_in, periods_in, present_value_in, false);
        //bg!(future_value_calc);
        //bg!(future_value_calc.is_normal());

        let rate_calc = rate(periods_in, present_value_in, future_value_calc, false);
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

        let fractional_periods_calc = periods(rate_in, present_value_in, future_value_calc, false);
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
            let present_value_calc = present_value(rate_in, periods_in, future_value_calc, false);
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
            rate_solution(periods_in, present_value_in, future_value_calc, false),
            periods_solution(rate_in, present_value_in, future_value_calc, false),
            future_value_solution(rate_in, periods_in, present_value_in, false),
        ];

        if future_value_calc.is_normal() {
            solutions.push(present_value_solution(rate_in, periods_in, future_value_calc, false));
        }
        for solution in solutions.iter() {
            //bg!(solution);
            if solution.calculated_field().is_rate() {
                // There are a few special cases in which the calculated rate is arbitrarily set to
                // zero since any value would work. We've already checked rate_calc against those
                // special cases, so use that here for the comparison.
                if !is_approx_equal_symmetry_test!(rate_calc, solution.rate()) {
                    dbg!(rate_calc, solution.rate(), &solution);
                }
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

        let mut schedules = vec![future_value_schedule_solution(&rates_in, present_value_in)];
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
            check_series_internal(label, solution.calculated_field(), &solution.series(), rate_in, periods_in, present_value_in, future_value_calc, rate_calc, periods_calc);
        }
        for solution in schedules.iter() {
            let label = format!("Schedule for {:?}", solution.calculated_field());
            //bg!(&label);
            check_series_internal(label, solution.calculated_field(),  &solution.series(), rate_in, periods_in, present_value_in, future_value_calc, rate_calc, periods_calc);
        }

        // Confirm that all of the series have the same values for all periods regardless of how we
        // did the calculation. For the reference solution take the result of
        // future_value_solution(). It would also work to use the result of rate_solution() and
        // present_value_solution() but not periods_solution() since there are some special cases in
        // which this will create fewer periods than the other functions.
        let reference_solution = solutions.iter().find(|solution| solution.calculated_field().is_future_value()).unwrap();
        let reference_series = reference_solution.series();
        for solution in solutions.iter().filter(|solution| !solution.calculated_field().is_future_value()) {
            let label = format!("Solution for {:?}", solution.calculated_field());
            check_series_same_values(reference_solution, &reference_series,label, solution.calculated_field(), &solution.series());
        }
        for schedule in schedules.iter() {
            let label = format!("Schedule for {:?}", schedule.calculated_field());
            check_series_same_values(reference_solution, &reference_series, label, schedule.calculated_field(), &schedule.series());
        }
    }

    fn check_series_internal(_label: String, calculated_field: &TvmVariable, series: &TvmSeries, rate_in: f64, periods_in: u32, present_value_in: f64, future_value_calc: f64, rate_calc: f64, periods_calc: u32) {
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

    fn check_series_same_values(_reference_solution: &TvmSolution, reference_series: &TvmSeries, _label: String, calculated_field: &TvmVariable, series: &[TvmPeriod]) {
        //bg!(reference_solution);
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
                dbg!(reference_entry.value(), round_4(reference_entry.value()), entry.value(), round_4(entry.value()));
                assert_approx_equal_symmetry_test!(reference_entry.value(), entry.value());
                // assert_eq!(reference_entry.value.round(), entry.value.round());
            }
        }
    }

    #[test]
    fn test_continuous_symmetry_one() {
        let rate = 0.10;
        let periods = 4;
        let present_value = 5_000.00;
        check_continuous_symmetry(rate, periods, present_value);
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

    fn check_continuous_symmetry(rate_in: f64, periods_in: u32, present_value_in: f64) {
        println!();
        dbg!("check_continuous_symmetry", rate_in, periods_in, present_value_in);

        /*
        let fv_calc = present_value_in * std::f64::consts::E.powf(rate_in * periods_in as f64);
        dbg!(fv_calc);
        let pv_calc = fv_calc / std::f64::consts::E.powf(rate_in * periods_in as f64);
        dbg!(pv_calc);
        */

        // Calculate the future value given the other three inputs so that we have all four values
        // which we can use in various combinations to confirm that all four continuous TVM
        // functions return consistent values.
        let future_value_calc = future_value(rate_in, periods_in, present_value_in, true);
        dbg!(future_value_calc);

        let rate_calc = rate::rate(periods_in, present_value_in, future_value_calc, true);
        dbg!(rate_calc);
        if periods_in == 0 || present_value_in == 0.0 {
            // With zero periods or zero for the present value, presumably the future value is the
            // same as the present value and any rate would be fine so we arbitrarily
            // return zero.
            assert_approx_equal_symmetry_test!(present_value_in, future_value_calc);
            assert_approx_equal_symmetry_test!(0.0, rate_calc);
        } else {
            dbg!(rate_calc, rate_in);
            assert_approx_equal_symmetry_test!(rate_calc, rate_in);
        }

        let fractional_periods_calc = periods(rate_in, present_value_in, future_value_calc, true);
        dbg!(fractional_periods_calc);
        let periods_calc = round_4(fractional_periods_calc).ceil() as u32;
        dbg!(periods_calc);
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
            let present_value_calc = present_value(rate_in, periods_in, future_value_calc, true);
            dbg!(present_value_calc);
            assert_approx_equal_symmetry_test!(present_value_calc, present_value_in);
        };

        // Create TvmSolution structs by solving for each of the four possible variables.
        let mut solutions = vec![
            rate_solution(periods_in, present_value_in, future_value_calc, true),
            periods_solution(rate_in, present_value_in, future_value_calc, true),
            future_value_solution(rate_in, periods_in, present_value_in, true),
        ];

        if future_value_calc.is_normal() {
            solutions.push(present_value_solution(rate_in, periods_in, future_value_calc, true));
        }
        for solution in solutions.iter() {
            dbg!(solution);
            // let series = solution.series();
            // dbg!(&series);
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

        // Check each series in isolation.
        /*
        for solution in solutions.iter() {
            let label = format!("Solution for {:?}", solution.calculated_field());
            //bg!(&label);
            check_series_internal(label, solution.calculated_field().clone(), &solution.series(), rate_in, periods_in, present_value_in, future_value_calc, rate_calc, periods_calc);
        }
        */

        // Confirm that all of the series have the same values for all periods regardless of how we
        // did the calculation. For the reference solution take the result of
        // future_value_solution(). It would also work to use the result of rate_solution() and
        // present_value_solution() but not periods_solution() since there are some special cases in
        // which this will create fewer periods than the other functions.
        let reference_solution = solutions.iter().find(|solution| solution.calculated_field().is_future_value()).unwrap();
        let reference_series = reference_solution.series();
        for solution in solutions.iter().filter(|solution| !solution.calculated_field().is_future_value()) {
            let label = format!("Solution for {:?}", solution.calculated_field());
            check_series_same_values(reference_solution, &reference_series, label, solution.calculated_field(), &solution.series());
        }
    }

    #[test]
    fn test_simple_to_continuous_symmetry_one() {
        let rate = 0.10;
        let periods = 4;
        let present_value = 5_000.00;
        check_simple_to_continuous_symmetry(rate, periods, present_value);
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

    fn check_simple_to_continuous_symmetry(rate_in: f64, periods_in: u32, present_value_in: f64) {
        println!();
        dbg!("check_simple_to_continuous_symmetry", rate_in, periods_in, present_value_in);

        // Calculate the future value given the other three inputs so that we have all four values
        // which we can use in various combinations to confirm that all four continuous TVM
        // functions return consistent values.
        let future_value_calc = future_value(rate_in, periods_in, present_value_in, true);
        dbg!(future_value_calc);

        // Create TvmSolution structs with continuous compounding by solving for each of the four possible variables.
        let continuous_solutions = vec![
            rate_solution(periods_in, present_value_in, future_value_calc, true),
            periods_solution(rate_in, present_value_in, future_value_calc, true),
            present_value_solution(rate_in, periods_in, future_value_calc, true),
            future_value_solution(rate_in, periods_in, present_value_in, true),
        ];

        // For each solution with continuous compounding create a corresponding solution with
        // simple compounding.
        /*
        let simple_solutions = continuous_solutions.iter()
            .map(|continuous_solution| continuous_solution.with_simple_compounding())
            .collect::<Vec<_>>();
        */
        let simple_solutions = [
            continuous_solutions[0].rate_solution(false, None),
            continuous_solutions[1].periods_solution(false),
            continuous_solutions[2].present_value_solution(false, None),
            continuous_solutions[3].future_value_solution(false, None),
        ];

        // Compare the continuous solutions to the corresponding simple solutions.
        for (index, continuous_solution) in continuous_solutions.iter().enumerate() {
            let simple_solution = &simple_solutions[index];
            println!("\nContinuous compounding vs. simple compounding adjusting {} while keeping the other three values constant.\n", continuous_solution.calculated_field().to_string().to_lowercase());
            dbg!(&continuous_solution, &simple_solution);
            assert_eq!(continuous_solution.calculated_field(), simple_solution.calculated_field());
            assert!(continuous_solution.continuous_compounding());
            assert!(!simple_solution.continuous_compounding());
            if continuous_solution.calculated_field().is_rate() {
                // We expect the rate to be lower with continuous compounding when the other three
                // inputs are held constant.
                assert!(continuous_solution.rate().abs() < simple_solution.rate().abs());
            } else {
                // The rate was an input rather than being calculated, so it should be the same.
                assert_eq!(continuous_solution.rate(), simple_solution.rate());
            }
            if continuous_solution.calculated_field().is_periods() {
                // We expect the fractional periods to be the same or lower with continuous
                // compounding when the other three inputs are held constant.
                assert!(continuous_solution.fractional_periods() <= simple_solution.fractional_periods());
                // Depending on rounding the number of periods may be the same or less for
                // continuous compounding.
                assert!(continuous_solution.periods() <= simple_solution.periods());
            } else {
                // The number of periods was an input rather than being calculated, so it should be
                // the same.
                assert_eq!(continuous_solution.periods(), simple_solution.periods());
            }
            if continuous_solution.calculated_field().is_present_value() {
                // We expect the present value to be lower with continuous compounding when the
                // other three inputs are held constant. This is because it takes less of an initial
                // investment to reach the same final value.
                assert!(continuous_solution.present_value().abs() < simple_solution.present_value().abs());
            } else {
                // The present value was an input rather than being calculated, so it should be the
                // same.
                assert_eq!(continuous_solution.present_value(), simple_solution.present_value());
            }
            if continuous_solution.calculated_field().is_future_value() {
                // We expect the future value to be higher with continuous compounding when the
                // other three inputs are held constant.
                assert!(continuous_solution.future_value().abs() > simple_solution.future_value().abs());
            } else {
                // The future value was an input rather than being calculated, so it should be the
                // same.
                assert_eq!(continuous_solution.future_value(), simple_solution.future_value());
            }
            assert_ne!(continuous_solution.formula(), simple_solution.formula());
            assert_ne!(continuous_solution.symbolic_formula(), simple_solution.symbolic_formula());
        }

        // For each solution with simple compounding create a corresponding solution with
        // continuous compounding. This should get us back to the equivalents of our original list
        // of solutions with continuous compounding.
        /*
        let continuous_solutions_round_trip = simple_solutions.iter()
            .map(|simple_solution| simple_solution.with_continuous_compounding())
            .collect::<Vec<_>>();
        */
        let continuous_solutions_round_trip = [
            continuous_solutions[0].rate_solution(true, None),
            continuous_solutions[1].periods_solution(true),
            continuous_solutions[2].present_value_solution(true, None),
            continuous_solutions[3].future_value_solution(true, None),
        ];

        // Compare the recently created continuous solutions to the original continuous solutions.
        for (index, solution) in continuous_solutions.iter().enumerate() {
            let solution_round_trip = &continuous_solutions_round_trip[index];
            println!("\nOriginal continuous compounding vs. derived continuous compounding where the calculated field is {}.\n", solution.calculated_field().to_string().to_lowercase());
            dbg!(&solution, &solution_round_trip);
            assert_eq!(solution, solution_round_trip);
        }
        /*
        for (calculated_field, continuous_solution) in continuous_solutions.iter() {
            dbg!(&continuous_solution);
            dbg!(&continuous_solution.series());

        }
        */

        // Check each series in isolation.
        /*
        for solution in solutions.iter() {
            let label = format!("Solution for {:?}", solution.calculated_field());
            //bg!(&label);
            check_series_internal(label, solution.calculated_field().clone(), &solution.series(), rate_in, periods_in, present_value_in, future_value_calc, rate_calc, periods_calc);
        }
        */

        /*
        // Confirm that all of the series have the same values for all periods regardless of how we
        // did the calculation. For the reference solution take the result of
        // future_value_solution(). It would also work to use the result of rate_solution() and
        // present_value_solution() but not periods_solution() since there are some special cases in
        // which this will create fewer periods than the other functions.
        let reference_solution = solutions.iter().find(|x| x.calculated_field().is_future_value()).unwrap();
        for solution in solutions.iter().filter(|x| !x.calculated_field().is_future_value()) {
            let label = format!("Solution for {:?}", solution.calculated_field());
            check_series_same_values(reference_solution, label, solution.calculated_field().clone(), &solution.series());
        }
        */
    }

    fn setup_for_compounding_periods() -> (TvmSolution, Vec<u32>) {
        let rate = 0.10;
        let periods = 4;
        let present_value = 5_000.00;
        let compounding_periods = vec![1, 2, 4, 6, 12, 24, 52, 365];
        (future_value_solution(rate, periods, present_value, false), compounding_periods)
    }

    #[test]
    fn test_with_compounding_periods_vary_future_value() {
        println!("\ntest_with_compounding_periods_vary_future_value()\n");

        let (solution, compounding_periods) = setup_for_compounding_periods();
        dbg!(&compounding_periods);

        for one_compounding_period in compounding_periods.iter() {
            println!("\nSimple compounding original vs. compounding periods = {} while varying future value.\n", one_compounding_period);
            dbg!(&solution, solution.future_value_solution(false, Some(*one_compounding_period)));
        }
    }

    #[test]
    fn test_with_compounding_periods_vary_present_value() {
        println!("\ntest_with_compounding_periods_vary_present_value()\n");

        let (solution, compounding_periods) = setup_for_compounding_periods();
        dbg!(&compounding_periods);

        for one_compounding_period in compounding_periods.iter() {
            println!("\nSimple compounding original vs. compounding periods = {} while varying present value.\n", one_compounding_period);
            dbg!(&solution, solution.present_value_solution(false, Some(*one_compounding_period)));
        }
    }
}