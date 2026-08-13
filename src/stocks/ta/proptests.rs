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
        fn vwap_rolling_batch_matches_state(
            (high, low, close) in hlc_strategy(),
            vol_scale in prop::collection::vec(0.1f64..5_000.0, 1..80),
            period in 2usize..15,
        ) {
            let n = close.len();
            prop_assume!(!vol_scale.is_empty());
            let volume: Vec<f64> = (0..n).map(|i| vol_scale[i % vol_scale.len()]).collect();
            let p = VwapParams::rolling_typical(period);
            let batch = vwap(&high, &low, &close, &volume, p).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let mut st = VwapState::new(p).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let streamed = st.push_bars(&high, &low, &close, &volume).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            prop_assert_eq!(batch.vwap.len(), streamed.len());
            for i in 0..streamed.len() {
                approx_opt(batch.vwap[i], streamed[i])?;
            }
        }

        #[test]
        fn vwap_close_source_batch_matches_state(
            (high, low, close) in hlc_strategy(),
            vol_scale in prop::collection::vec(0.1f64..5_000.0, 1..60),
        ) {
            let n = close.len();
            prop_assume!(!vol_scale.is_empty());
            let volume: Vec<f64> = (0..n).map(|i| vol_scale[i % vol_scale.len()]).collect();
            let p = VwapParams {
                mode: VwapMode::Cumulative,
                price_source: VwapPriceSource::Close,
            };
            let batch = vwap(&high, &low, &close, &volume, p).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let mut st = VwapState::new(p).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let streamed = st.push_bars(&high, &low, &close, &volume).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            for i in 0..n {
                approx_opt(batch.vwap[i], streamed[i])?;
            }
        }

        #[test]
        fn bollinger_full_series_matches_state(
            closes in prop::collection::vec(-200.0f64..200.0, 5..80),
            period in 2usize..20,
            num_std in 0.5f64..3.0,
        ) {
            let p = BollingerParams::new(period, num_std);
            let batch = bollinger(&closes, p).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let mut st = BollingerState::new(p).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let streamed = st.push_bars(&closes).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            prop_assert_eq!(batch.middle.len(), streamed.len());
            for i in 0..streamed.len() {
                match (streamed[i], batch.middle[i], batch.upper[i], batch.lower[i]) {
                    (None, None, None, None) => {}
                    (Some(bar), Some(m), Some(u), Some(l)) => {
                        prop_assert!((bar.middle - m).abs() < 1e-8);
                        prop_assert!((bar.upper - u).abs() < 1e-8);
                        prop_assert!((bar.lower - l).abs() < 1e-8);
                    }
                    other => prop_assert!(false, "bollinger bar mismatch at {i}: {other:?}"),
                }
            }
        }

        #[test]
        fn bollinger_population_full_series(
            closes in prop::collection::vec(-100.0f64..100.0, 10..60),
            period in 2usize..15,
        ) {
            let p = BollingerParams::with_stdev(period, 2.0, StdevKind::Population);
            let batch = bollinger(&closes, p).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let mut st = BollingerState::new(p).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let streamed = st.push_bars(&closes).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            for i in 0..streamed.len() {
                match (streamed[i].map(|b| b.middle), batch.middle[i]) {
                    (None, None) => {}
                    (Some(a), Some(b)) => prop_assert!((a - b).abs() < 1e-8),
                    other => prop_assert!(false, "{other:?}"),
                }
            }
        }

        #[test]
        fn macd_full_series_matches_state(
            closes in prop::collection::vec(-100.0f64..100.0, 40..90),
        ) {
            let p = MacdParams::standard();
            let batch = macd(&closes, p).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let mut st = MacdState::new(p).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let streamed = st.push_bars(&closes).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            prop_assert_eq!(batch.macd.len(), streamed.len());
            for i in 0..streamed.len() {
                match (streamed[i], batch.macd[i], batch.signal[i], batch.histogram[i]) {
                    (None, None, None, None) => {}
                    // State returns None until signal ready; batch may have macd before signal.
                    (None, Some(_), None, None) => {}
                    (Some((m, s, h)), Some(bm), Some(bs), Some(bh)) => {
                        prop_assert!((m - bm).abs() < 1e-7);
                        prop_assert!((s - bs).abs() < 1e-7);
                        prop_assert!((h - bh).abs() < 1e-7);
                    }
                    other => prop_assert!(false, "macd mismatch at {i}: {other:?}"),
                }
            }
        }

        #[test]
        fn keltner_full_series_matches_state(
            (high, low, close) in hlc_strategy(),
        ) {
            prop_assume!(close.len() >= 30);
            let p = KeltnerParams::standard();
            let batch = keltner(&high, &low, &close, p).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let mut st = KeltnerState::new(p).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let streamed = st.push_bars(&high, &low, &close).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            for i in 0..streamed.len() {
                match (streamed[i], batch.middle[i], batch.upper[i], batch.atr[i]) {
                    (None, None, None, None) => {}
                    // ATR may warm before EMA or vice versa → batch None on channel, state None.
                    (None, _, _, _) => {}
                    (Some(bar), Some(m), Some(u), Some(a)) => {
                        prop_assert!((bar.middle - m).abs() < 1e-8);
                        prop_assert!((bar.upper - u).abs() < 1e-8);
                        prop_assert!((bar.atr - a).abs() < 1e-8);
                    }
                    other => prop_assert!(false, "keltner mismatch at {i}: {other:?}"),
                }
            }
        }

        #[test]
        fn rsi_batch_matches_state(closes in closes_strategy(), period in 2usize..20) {
            let p = RsiParams::new(period);
            let batch = rsi(&closes, p).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let mut st = RsiState::new(p).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let streamed = st
                .push_bars(&closes)
                .map_err(|e| TestCaseError::fail(format!("{e}")))?;
            prop_assert_eq!(batch.rsi.len(), streamed.len());
            for i in 0..streamed.len() {
                approx_opt(batch.rsi[i], streamed[i])?;
            }
        }

        #[test]
        fn atr_batch_matches_state(
            (high, low, close) in hlc_strategy(),
            period in 2usize..20,
        ) {
            let p = AtrParams::new(period);
            let batch = atr(&high, &low, &close, p)
                .map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let mut st = AtrState::new(p).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let streamed = st
                .push_bars(&high, &low, &close)
                .map_err(|e| TestCaseError::fail(format!("{e}")))?;
            prop_assert_eq!(batch.atr.len(), streamed.len());
            for i in 0..streamed.len() {
                approx_opt(batch.atr[i], streamed[i])?;
            }
        }

        #[test]
        fn sma_last_matches_series_tail(
            closes in prop::collection::vec(-200.0f64..200.0, 5..50),
            period in 1usize..20,
        ) {
            let series = sma(&closes, period).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let last = sma_last(&closes, period).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let tail = series.iter().rev().find_map(|x| *x);
            approx_opt(last, tail)?;
        }

        #[test]
        fn ema_last_matches_series_tail(
            closes in prop::collection::vec(-200.0f64..200.0, 5..50),
            period in 1usize..20,
        ) {
            let series = ema(&closes, period).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let last = ema_last(&closes, period).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let tail = series.iter().rev().find_map(|x| *x);
            approx_opt(last, tail)?;
        }

        #[test]
        fn wma_batch_matches_state(closes in closes_strategy(), period in 1usize..20) {
            let batch = wma(&closes, period).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let mut st = WmaState::new(period).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let streamed = st.push_bars(&closes).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            for i in 0..batch.len() {
                approx_opt(batch[i], streamed[i])?;
            }
        }

        #[test]
        fn hma_batch_matches_state(
            closes in prop::collection::vec(-200.0f64..200.0, 10..60),
            period in 2usize..25,
        ) {
            let batch = hma(&closes, period).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let mut st = HmaState::new(period).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let streamed = st.push_bars(&closes).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            for i in 0..batch.len() {
                approx_opt(batch[i], streamed[i])?;
            }
        }

        #[test]
        fn linreg_batch_matches_state(
            series in prop::collection::vec(-200.0f64..200.0, 5..50),
            period in 2usize..20,
        ) {
            let p = LinRegParams::new(period);
            let batch = linear_regression(&series, p).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let mut st = LinRegState::new(p).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let streamed = st.push_bars(&series).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            for i in 0..batch.len() {
                match (batch[i], streamed[i]) {
                    (None, None) => {}
                    (Some(a), Some(b)) => {
                        prop_assert!((a.slope - b.slope).abs() < 1e-9);
                        prop_assert!((a.r_squared - b.r_squared).abs() < 1e-9);
                    }
                    other => prop_assert!(false, "{other:?}"),
                }
            }
        }

        #[test]
        fn donchian_batch_matches_state(
            (high, low, _close) in hlc_strategy(),
            period in 2usize..20,
        ) {
            let p = DonchianParams::new(period);
            let batch = donchian(&high, &low, p).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let mut st = DonchianState::new(p).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let streamed = st.push_bars(&high, &low).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            for i in 0..batch.upper.len() {
                match (streamed[i], batch.upper[i], batch.lower[i]) {
                    (None, None, None) => {}
                    (Some(bar), Some(u), Some(l)) => {
                        prop_assert!((bar.upper - u).abs() < 1e-12);
                        prop_assert!((bar.lower - l).abs() < 1e-12);
                    }
                    other => prop_assert!(false, "{other:?}"),
                }
            }
        }

        #[test]
        fn willr_batch_matches_state(
            (high, low, close) in hlc_strategy(),
            period in 2usize..20,
        ) {
            let p = WillrParams::new(period);
            let batch = willr(&high, &low, &close, p).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let mut st = WillrState::new(p).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let streamed = st.push_bars(&high, &low, &close).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            for i in 0..batch.willr.len() {
                approx_opt(batch.willr[i], streamed[i])?;
            }
        }

        #[test]
        fn obv_batch_matches_state(
            (closes, volume) in closes_strategy().prop_flat_map(|closes| {
                let n = closes.len();
                prop::collection::vec(0.0f64..1e6, n..=n)
                    .prop_map(move |volume| (closes.clone(), volume))
            }),
        ) {
            let p = ObvParams::default_pack();
            let batch = obv(&closes, &volume, p).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let mut st = ObvState::new(p).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let streamed = st.push_bars(&closes, &volume).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            for i in 0..batch.obv.len() {
                approx_opt(batch.obv[i], streamed[i])?;
            }
        }

        #[test]
        fn cci_batch_matches_state(
            (high, low, close) in hlc_strategy(),
            period in 2usize..20,
        ) {
            let p = CciParams::new(period);
            let batch = cci(&high, &low, &close, p).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let mut st = CciState::new(p).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let streamed = st.push_bars(&high, &low, &close).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            for i in 0..batch.cci.len() {
                approx_opt(batch.cci[i], streamed[i])?;
            }
        }

        #[test]
        fn adx_batch_matches_state((high, low, close) in hlc_strategy()) {
            prop_assume!(close.len() >= 40);
            let p = AdxParams::period_14();
            let batch = adx(&high, &low, &close, p).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let mut st = AdxState::new(p).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            let streamed = st.push_bars(&high, &low, &close).map_err(|e| TestCaseError::fail(format!("{e}")))?;
            for i in 0..streamed.len() {
                approx_opt(batch.plus_di[i], streamed[i].plus_di)?;
                approx_opt(batch.minus_di[i], streamed[i].minus_di)?;
                approx_opt(batch.dx[i], streamed[i].dx)?;
                approx_opt(batch.adx[i], streamed[i].adx)?;
            }
        }
    }
}
