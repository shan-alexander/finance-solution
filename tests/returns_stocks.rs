//! Integration tests for returns + stocks polish.

use finance_solution::*;

#[test]
fn doubling_solution_symmetry_and_table_fields() {
    let s = doubling_solution(0.08).unwrap();
    assert_rounded_2!(s.rule_of_72(), 9.0);
    let grown = (1.0 + s.rate()).powf(s.exact());
    assert!((grown - 2.0).abs() < 1e-9);

    let multi = doubling_compare_rates(&[0.02, 0.08, 0.15]).unwrap();
    assert_eq!(multi.rows().len(), 3);
}

#[test]
fn price_path_matches_scalars() {
    let prices = [100.0, 120.0, 90.0, 100.0];
    let path = price_path_solution(&prices, PricePathOptions::new(12.0).with_years(1.0)).unwrap();

    assert_approx_equal!(path.total_return(), total_return(100.0, 100.0).unwrap());
    assert_approx_equal!(path.max_drawdown(), max_drawdown(&prices).unwrap());
    assert_approx_equal!(
        path.volatility().unwrap(),
        price_volatility(&prices).unwrap()
    );

    let series = path.series();
    assert_eq!(series.len(), 3);
    let mut wealth = 1.0;
    for row in series.iter() {
        wealth *= 1.0 + row.simple_return();
        assert_approx_equal!(wealth, row.wealth_index());
    }
}

#[test]
fn beta_sortino_rolling() {
    let market = [0.01, -0.02, 0.03, 0.00, 0.02];
    let asset: Vec<f64> = market.iter().map(|r| 1.25 * r).collect();
    assert!((beta(&asset, &market).unwrap() - 1.25).abs() < 1e-9);

    assert!(sortino_ratio(&market, 0.0).unwrap().is_finite());

    let prices = [100.0, 110.0, 95.0, 100.0, 120.0];
    let roll = rolling_max_drawdown(&prices).unwrap();
    assert_eq!(roll.len(), prices.len());
    assert!(roll.windows(2).all(|w| w[1] + 1e-12 >= w[0])); // nondecreasing
}

#[test]
fn print_summary_two_prices_and_no_downside_no_panic() {
    // Two prices → only one return → vol/Sharpe n/a; must not panic on "n/a" formatting.
    let short = price_path_solution(&[100.0, 110.0], PricePathOptions::default()).unwrap();
    assert!(short.volatility().is_none());
    assert!(short.sharpe_ratio().is_none());
    short.print_summary();

    // All positive returns vs target 0 → Sortino None; summary still prints.
    let rising = [100.0, 101.0, 102.0, 103.0, 104.0];
    let path = price_path_solution(&rising, PricePathOptions::default()).unwrap();
    assert!(path.sortino_ratio().is_none());
    path.print_summary();
    path.series().print_table();
}
