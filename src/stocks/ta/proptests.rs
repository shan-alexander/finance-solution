//! Property tests: random paths, batch series ≡ incremental state.
//!
//! These complement fixed Excel-style fixtures: they catch branch/window bugs that no
//! hand-picked golden number hits. Run with the rest of `cargo test`.

#[cfg(test)]
mod tests {
    use crate::stocks::ta::*;
    use proptest::prelude::*;

    /// Finite closes in a trading-like magnitude band (avoids NaN/Inf by construction).
    fn closes_strategy() -> impl Strategy<Value = Vec<f64>> {
        prop::collection::vec(-500.0f64..500.0, 1..80)
    }

    fn hlc_strategy() -> impl Strategy<Value = (Vec<f64>, Vec<f64>, Vec<f64>)> {
        closes_strategy().prop_map(|close| {
            let high: Vec<_> = close.iter().map(|c| c + 0.5).collect();
            let low: Vec<_> = close.iter().map(|c| c - 0.5).collect();
            (high, low, close)
        })
    }

    fn approx_opt(a: Option<f64>, b: Option<f64>) -> Result<(), TestCaseError> {
        match (a, b) {
            (None, None) => Ok(()),
            (Some(x), Some(y)) => {
                prop_assert!((x - y).abs() < 1e-8, "{} vs {}", x, y);
                Ok(())
            }
            _ => {
                prop_assert!(false, "Option mismatch {:?} vs {:?}", a, b);
                Ok(())
            }
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(32))]

        #[test]
        fn sma_batch_matches_state(closes in closes_strategy(), period in 1usize..25) {
            let batch = sma(&closes, period).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let mut st = SmaState::new(period).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let streamed = st.push_bars(&closes).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            prop_assert_eq!(batch.len(), streamed.len());
            for i in 0..batch.len() {
                approx_opt(batch[i], streamed[i])?;
            }
        }

        #[test]
        fn ema_batch_matches_state(closes in closes_strategy(), period in 1usize..25) {
            let batch = ema(&closes, period).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let mut st = EmaState::new(period).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let streamed = st.push_bars(&closes).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            prop_assert_eq!(batch.len(), streamed.len());
            for i in 0..batch.len() {
                approx_opt(batch[i], streamed[i])?;
            }
        }

        #[test]
        fn stoch_batch_matches_state(
            (high, low, close) in hlc_strategy(),
            k in 2usize..15,
            ks in 1usize..5,
            d in 1usize..5,
        ) {
            let p = StochasticParams::full(k, ks, d);
            let batch = stochastics(&high, &low, &close, p).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let mut st = StochState::new(p).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let streamed = st.push_bars(&high, &low, &close).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            prop_assert_eq!(batch.k.len(), streamed.len());
            for i in 0..streamed.len() {
                approx_opt(batch.k[i], streamed[i].k)?;
                approx_opt(batch.d[i], streamed[i].d)?;
            }
        }

        #[test]
        fn rvol_batch_matches_state(
            volume in prop::collection::vec(0.0f64..10_000.0, 1..60),
            lookback in 1usize..20,
        ) {
            let p = RvolParams::new(lookback);
            let batch = rvol(&volume, p).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let mut st = RvolState::new(p).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let streamed = st.push_bars(&volume).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            prop_assert_eq!(batch.rvol.len(), streamed.len());
            for i in 0..streamed.len() {
                approx_opt(batch.rvol[i], streamed[i])?;
            }
        }

        #[test]
        fn vwap_cum_batch_matches_state(
            (high, low, close) in hlc_strategy(),
            vol_scale in prop::collection::vec(0.1f64..5_000.0, 1..80),
        ) {
            // Align volume length to the OHLC path (independent strategies can differ).
            let n = close.len();
            prop_assume!(!vol_scale.is_empty());
            let volume: Vec<f64> = (0..n).map(|i| vol_scale[i % vol_scale.len()]).collect();
            let p = VwapParams::cumulative_typical();
            let batch = vwap(&high, &low, &close, &volume, p).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let mut st = VwapState::new(p).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let streamed = st.push_bars(&high, &low, &close, &volume).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            prop_assert_eq!(batch.vwap.len(), streamed.len());
            for i in 0..streamed.len() {
                approx_opt(batch.vwap[i], streamed[i])?;
            }
        }

        #[test]
        fn bollinger_last_matches(
            closes in prop::collection::vec(-200.0f64..200.0, 25..80),
            period in 2usize..20,
            num_std in 0.5f64..3.0,
        ) {
            let p = BollingerParams::new(period, num_std);
            let batch = bollinger(&closes, p).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let mut st = BollingerState::new(p).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let _ = st.push_bars(&closes).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let bl = batch.middle.iter().rev().find_map(|x| *x);
            let sl = st.last().map(|o| o.middle);
            approx_opt(bl, sl)?;
        }

        #[test]
        fn macd_last_matches_state(
            closes in prop::collection::vec(-100.0f64..100.0, 40..90),
        ) {
            let p = MacdParams::standard();
            let batch = macd(&closes, p).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let mut st = MacdState::new(p).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let _ = st.push_bars(&closes).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            match (batch.last(), st.last()) {
                (Some((a, b, c)), Some((d, e, f))) => {
                    prop_assert!((a - d).abs() < 1e-7);
                    prop_assert!((b - e).abs() < 1e-7);
                    prop_assert!((c - f).abs() < 1e-7);
                }
                (None, None) => {}
                other => prop_assert!(false, "macd last mismatch {other:?}"),
            }
        }

        #[test]
        fn keltner_last_matches_state(
            (high, low, close) in hlc_strategy(),
        ) {
            prop_assume!(close.len() >= 30);
            let p = KeltnerParams::standard();
            let batch = keltner(&high, &low, &close, p).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let mut st = KeltnerState::new(p).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let _ = st.push_bars(&high, &low, &close).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let bm = batch.middle.iter().rev().find_map(|x| *x);
            let sm = st.last().map(|o| o.middle);
            approx_opt(bm, sm)?;
        }
    }
}
