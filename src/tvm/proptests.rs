//! Property tests for TVM identity / round-trip relations.
//!
//! Complements Excel fixtures and hand symmetry tests: random valid rates,
//! periods, and present values must satisfy classical identities within
//! floating tolerance.

#[cfg(test)]
mod tests {
    use crate::tvm::{future_value, periods, present_value, rate};
    use proptest::prelude::*;

    // Rates in (-0.5, 1.0) excluding pathologically tiny; periods 1..60; |PV| in [1, 1e6]
    fn arb_rate() -> impl Strategy<Value = f64> {
        prop_oneof![(-0.5f64..-1e-6), (1e-6f64..1.0),]
    }

    fn arb_pv() -> impl Strategy<Value = f64> {
        prop_oneof![(-1e6f64..-1.0), (1.0f64..1e6)]
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(64))]

        /// PV(FV(PV)) ≈ PV for discrete compounding.
        #[test]
        fn present_future_roundtrip(
            rate in arb_rate(),
            periods in 1u32..40u32,
            pv in arb_pv(),
        ) {
            let fv = future_value(rate, periods, pv, false).expect("fv");
            let pv2 = present_value(rate, periods, fv, false).expect("pv");
            let scale = pv.abs().max(1.0);
            prop_assert!((pv2 - pv).abs() / scale < 1e-8, "pv={pv} pv2={pv2} fv={fv}");
        }

        /// rate recovered from (n, PV, FV) matches input (when well-conditioned).
        #[test]
        fn rate_identity_from_fv(
            rate_in in 0.001f64..0.5,
            periods in 2u32..30u32,
            pv in -1e5f64..-10.0,
        ) {
            let fv = future_value(rate_in, periods, pv, false).expect("fv");
            // rate(n, pv, fv) convention: payment stream style — use same signs as library
            let r = rate(periods, pv, fv, false).expect("rate");
            prop_assert!((r - rate_in).abs() < 1e-6, "rate_in={rate_in} r={r}");
        }

        /// periods recovered from (r, PV, FV) matches input.
        #[test]
        fn periods_identity_from_fv(
            rate_in in 0.001f64..0.3,
            periods_in in 2u32..36u32,
            pv in -5e4f64..-10.0,
        ) {
            let fv = future_value(rate_in, periods_in, pv, false).expect("fv");
            let n = periods(rate_in, pv, fv, false).expect("nper");
            prop_assert!((n - periods_in as f64).abs() < 1e-4, "n={n} want={periods_in}");
        }

        /// Continuous: FV and PV invert.
        #[test]
        fn continuous_pv_fv_roundtrip(
            rate in 0.001f64..0.5,
            periods in 1u32..40u32,
            pv in arb_pv(),
        ) {
            let fv = future_value(rate, periods, pv, true).expect("fv");
            let pv2 = present_value(rate, periods, fv, true).expect("pv");
            let scale = pv.abs().max(1.0);
            prop_assert!((pv2 - pv).abs() / scale < 1e-8);
        }
    }
}
