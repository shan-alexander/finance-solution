//! Integration tests for payment and amortization helpers.

use finance_solution::*;

#[test]
fn payment_and_amortization_agree() {
    let rate = 0.09 / 12.0;
    let periods = 36;
    let pv = 15_000.0;
    let pmt = payment(rate, periods, pv, 0.0, false).unwrap();

    let mut sum_principal = 0.0;
    let mut sum_interest = 0.0;
    for period in 1..=periods {
        let princ = ppmt(rate, period, periods, pv, 0.0, false).unwrap();
        let int = ipmt(rate, period, periods, pv, 0.0, false).unwrap();
        assert_approx_equal!(princ + int, pmt);
        sum_principal += princ;
        sum_interest += int;
    }

    assert_approx_equal!(sum_principal, -pv);
    assert_approx_equal!(
        sum_interest,
        payment_solution(rate, periods, pv, 0.0, false)
            .unwrap()
            .sum_of_interest()
    );
    assert_approx_equal!(
        cumprinc(rate, periods, pv, 0.0, 1, periods, false).unwrap(),
        sum_principal
    );
    assert_approx_equal!(
        cumipmt(rate, periods, pv, 0.0, 1, periods, false).unwrap(),
        sum_interest
    );
}

#[test]
fn amortization_solution_series_and_errors() {
    let rate = 0.09 / 12.0;
    let periods = 36;
    let pv = 15_000.0;

    let sol = amortization_solution(rate, periods, pv, 0.0, false).unwrap();
    let series = sol.series();
    assert_eq!(series.len(), periods as usize);
    assert!(!sol.formula().is_empty());
    assert!(!series[0].formula().is_empty());
    assert_approx_equal!(sol.ipmt(1).unwrap(), series[0].interest());
    assert_approx_equal!(sol.ppmt(1).unwrap(), series[0].principal());

    match amortization_solution(-1.5, periods, pv, 0.0, false) {
        Err(FinanceError::InvalidRate { rate }) => assert_eq!(rate, -1.5),
        other => panic!("expected InvalidRate, got {other:?}"),
    }
}

#[test]
fn payment_rejects_bad_rate() {
    assert!(payment(-1.0, 10, 1000.0, 0.0, false).is_err());
    assert!(payment(f64::NAN, 10, 1000.0, 0.0, false).is_err());

    match payment(-1.5, 10, 1000.0, 0.0, false) {
        Err(FinanceError::InvalidRate { rate }) => assert_eq!(rate, -1.5),
        other => panic!("expected InvalidRate, got {other:?}"),
    }
}
