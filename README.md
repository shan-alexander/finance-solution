# finance-solution

**finance-solution** is a Rust library for time-value-of-money (TVM), cashflow, amortization, and related finance formulas — with **detailed solution structs**, period-by-period series, and pretty-printed tables. Scalar math executes in nanoseconds, making it suitable for quant systems. Solution structs execute in microseconds, providing observability with speed.

## finance-solution is a financial library for solving time-value-of-money problems and financial math, including common quant calculations. 💸

People who will find this crate helpful include:
- **Students of Finance** who want to solve their financial problems using something better than a crude handheld calculator. Using this library also reduces the chance of human error, and provides better output and data displays than Excel in less time thereby providing a better surface for internalizing financial math concepts.
- **New developers** who want to learn Rust using Finance as a topic.
- **Experienced Rust developers** who want to learn more about finance and/or incorporate safe financial calculations into their projects.
- **Serious Rust developers** who want to build financial software, and prefer to rely on a rigourously tested library instead of reinventing the wheel and spending hundreds of hours to develop and test their own library of financial calculations.

In the v0.0.0 release, this library was geared only towards the basic financial equations, regarding:
- **Simple Time-Value-of-Money formulas** -- this includes `present_value`, `future_value`, `rate`, and `periods` (known as NPER in Excel).
- **Cashflow Time-Value-of-Money formulas** -- this includes `present_value_annuity`, `future_value_annuity`, `net_present_value`, and `payment` (known as PMT in Excel). 
- **Rate conversions** -- this includes all conversions between `apr`, `ear`, and `epr`, and also includes conversion for continuous compounding (`apr_continuous`, `ear_continuous`).

> The v0.1.0 release revamped to eliminate panicking APIs, enforcing all methods to return a `Result` (as `FinanceResult` or `FinanceError`) and added an amortization module.

> The v0.2.0 release added domain newtypes, full `stocks::ta` technical analysis indicators like Bollinger and Stochastics for both batch + incremental `*State`, and added teaching solutions/tables, and Criterion TA benches.

> **Current Status:** **0.3.0** (edition 2021, MSRV 1.70). Adds the **`derivatives`** module (European Black–Scholes–Merton price, Greeks ΔΓθνρ, implied vol, put–call parity helpers, `BsmState`) plus a real 1m OHLCV example joining underlier TA with a BSM snapshot. Public math remains **Result-only** — no dual panicking / `try_*` APIs. The crate is a **math library** for engines and frameworks (not a market-data feed or order router).

## Modules (0.3 layout)

| Module | Contents |
|--------|----------|
| `tvm` | Present/future value, rate, periods (simple + continuous, fixed + schedule) |
| `cashflow` | Payment (PMT), annuities, NPV, NPER |
| `convert_rate` | APR ↔ EPR ↔ EAR (+ continuous) |
| `amortization` | Solution/series/tables + PPMT, IPMT, CUMPRINC, CUMIPMT |
| `returns` | Doubling rules: solution + comparison tables (72/70/69 vs exact) |
| `stocks` | Price-path solution/series/tables; returns, vol, Sharpe, Sortino, beta, drawdowns |
| `stocks::ta` | Technical analysis: SMA/EMA, Stochastic, MACD, Bollinger, Keltner, VWAP, RVOL (+ `*State`) |
| `derivatives` | European BSM option price, Greeks, IV, parity residual, intrinsic/time value, `BsmState` (Black ’76 / FX planned later) |
| `util` | `FinanceError`, `FinanceResult`, domain newtypes (`Rate`, `PeriodLength`, …) |
| `round` | Rounding + test assert helpers |

## Quick start

```rust
use finance_solution::*;

// Present value of $4,000 in 3 years at 5% (simple compounding)
// Fixture rates/amounts are valid; production code should use `?` or match.
let pv = present_value(0.05, 3, 4_000.0, false).expect("fixture: 5% PV");
assert_rounded_2!(pv, -3_455.35); // sign: opposite of future value

// Prefer the solution API for formulas + period series
let answer = present_value_solution(0.05, 3, 4_000.0, false).expect("fixture");
println!("{}", answer.formula());
answer.series().print_table();

// Handle bad input without panicking
match present_value(-1.5, 3, 4_000.0, false) {
    Ok(v) => println!("pv = {v}"),
    Err(e) => eprintln!("error: {e}"),
}
```

### Payment + amortization table

```rust
use finance_solution::*;

let rate = 0.08 / 12.0;
let periods = 60;
let principal = 13_000.0;

let solution = payment_solution(rate, periods, principal, 0.0, false).expect("fixture loan");
solution.print_table();

// Excel-style period components (1-based period index)
let interest_m1 = ipmt(rate, 1, periods, principal, 0.0, false).expect("period 1");
let principal_m1 = ppmt(rate, 1, periods, principal, 0.0, false).expect("period 1");
let year1_interest = cumipmt(rate, periods, principal, 0.0, 1, 12, false).expect("range");
```

### Rate conversion

```rust
use finance_solution::*;

let r = apr(0.034, 12).expect("fixture: 3.4% APR monthly");
println!("EPR = {}, EAR = {}", r.epr(), r.ear());
```

### Doubling rules & price paths

```rust
use finance_solution::*;

let d = doubling_solution(0.08).expect("fixture: 8%");
assert_rounded_2!(d.rule_of_72(), 9.0);
d.print_table();

let prices = [100.0, 110.0, 105.0, 120.0];
let path = price_path_solution(&prices, PricePathOptions::new(12.0)).expect("fixture prices");
path.print_summary();
path.series().print_table();
```

### Technical analysis — quant pattern (stochastics)

`finance-solution` is a **math library**, not a trading engine: you own the bar loop and data feed.
For indicators, define each variation **once** as a `const` parameter pack, validate once, then
call `.compute` on every batch of bars (or free functions for one-off scripts).

```rust
use finance_solution::*;

// 1) Strategy knobs — fixed packs (FastStoch(9,3), FullStoch(14,3,3), …)
const FAST_9_3: StochasticParams = StochasticParams::fast(9, 3);
const FULL_14_3_3: StochasticParams = StochasticParams::full(14, 3, 3);
const MACD_STD: MacdParams = MacdParams::standard(); // (12, 26, 9)
const BB_20_2: BollingerParams = BollingerParams::standard();

// 2) Validate once at process/strategy startup (O(1))
//    Fallible constructors are named `new` → FinanceResult (not `try_new`):
//    fallibility lives in the return type, matching Result-only crate policy.
let stoch = ValidatedStochastic::new(FAST_9_3).expect("params");
let macd_eng = ValidatedMacd::new(MACD_STD).expect("params");
let bb = ValidatedBollinger::new(BB_20_2).expect("params");

// 3) Hot path — reuse engines on each symbol / day batch
// let kd = stoch.compute(&high, &low, &close)?;
// let m = macd_eng.compute(&closes)?;
// let bands = bb.compute(&closes)?;

// Teaching / audit path (formulas + print_table with n/a warm-up):
// let sol = stochastics_solution(&high, &low, &close, FAST_9_3)?;
// sol.print_table();
```

Common stochastic packs: `StochasticParams::fast(9, 3)`, `fast(14, 3)`, `full(14, 3, 3)`,
`full(60, 10, 1)`. Same idea for MACD, Bollinger, Keltner, VWAP, RVOL — see `stocks::ta` rustdoc.

### Options — European Black–Scholes–Merton

**Trading:** model mark vs mid, delta hedge ratio, vega/theta carry, IV from mid.  
**Engineering:** sync pure functions; your feed calls `set_spot` / `set_vol_from_price` on per-contract [`BsmState`].

| Input | Unit in this crate |
|-------|--------------------|
| `time_years` | **years** (`21.0/365.25` ≈ 21 calendar days) |
| `rate`, `dividend_yield` | continuous absolute (`0.05` = 5%) |
| `vol` | annualized absolute (`0.20` = 20%) |
| raw **vega** | per **+1.0** in σ → use `vega_per_vol_point()` for per 1% |
| raw **theta** | per **year** → use `theta_per_calendar_day()` for daily |

```rust
use finance_solution::*;

let p = BsmParams::atm_one_year(100.0, 0.05, 0.20);
let call = bsm_price(p, OptionType::Call).expect("fixture");
let g = bsm_greeks(p, OptionType::Call).expect("fixture");
// Desk scales: per 1% vol, per calendar day
let _vega_1pct = g.vega_per_vol_point();
let _theta_day = g.theta_per_calendar_day();
let iv = bsm_implied_vol(p, OptionType::Call, call).expect("fixture");
assert!((iv - 0.20).abs() < 1e-4);

// Live: per-contract state in *your* HashMap (not inside this crate)
let mut opt = BsmState::new(p, OptionType::Call).expect("fixture");
opt.set_spot(101.0).expect("fixture");
let _ = opt.greeks();
// opt.set_vol_from_price(mid)?;  // on option quote
```

Join underlier TA with the option chain in **your** engine: `StochState` on bars + `BsmState` on strikes, same process, sync math, optional `rayon` over the chain on a spot tick.

### Examples

```bash
cargo run --example common_word_problems
cargo run --example amortization_table_demo
cargo run --example early_payoff_what_if
cargo run --example doubling_rules
cargo run --example price_path_analysis
cargo run --example ta_indicators
cargo run --example bsm_option
# Real 1m OHLCV (GitHub tree only; excluded from crates.io package size)
cargo run --example real_bars_ta_bsm --release
# REAL_BARS_SYMBOL=SPY cargo run --example real_bars_ta_bsm --release
```

## Design notes

1. **Result-only public math** — domain failures are `FinanceResult` / `FinanceError`. There is **no** dual panicking + `try_*` pair; the ordinary name *is* the fallible function.
2. **Solution + series** — `*_solution` types carry inputs, symbolic/numeric formulas, and `.series()` for period detail.
3. **Excel-compatible signs** — loans/payments follow spreadsheet conventions (positive principal → negative payment, etc.).
4. **Enums for flags** — prefer [`Compounding`] and [`PaymentTiming`]; `bool` still converts via `From` for ergonomics. TA uses enums where modes matter (e.g. `VwapMode`, `VwapPriceSource`, `StdevKind`).
5. **Tables** — `.print_table()` / `.print_table_locale()` for teaching and debugging (copy/paste friendly).
6. **Hot path vs solution path** — scalar / series APIs are the production path (`FinanceResult`, no formula strings). Solution APIs are the teaching/observability path. Same formulas; different packaging. See [`benches/RESULTS.md`](benches/RESULTS.md).
7. **TA params** — one core function per indicator + `Copy` `*Params` + optional `Validated*`; presets via `const` constructors (`fast`, `standard`, …), not a zoo of nearly identical free functions.
8. **Derivatives units** — BSM vega/theta raw model units differ from desk screens; always use the helper methods above when mapping to “per vol point” / “per day” P&L.
9. **Math library, not an engine** — no websockets, symbol books, calendars, or order routing. You own `HashMap`s, session resets, and concurrency (`rayon` / `tokio` outside this crate).

### Live bars: incremental state (not a market-data engine)

This crate does **not** subscribe to feeds or manage multi-symbol books. It **does** provide
pure **`*State` machines** so *your* quant engine can update indicators on each payload
(tick / 5s / 1m) without recomputing the full history:

| Batch (research / backtest) | Incremental (live) |
|-----------------------------|--------------------|
| `stoch.compute(&h,&l,&c)` | `StochState::push(h,l,c)` |
| `macd_eng.compute(&closes)` | `MacdState::push(close)` |
| `vwap(...)` cumulative | `VwapState::push(...)` + **`reset()` when *you* open a session** |
| `bsm_price` / `bsm_greeks` | `BsmState::set_spot` / `set_vol_from_price` |

**Quant engine sketch** (per-symbol underlier + optional option chain):

```rust
use finance_solution::*;
use std::collections::HashMap;

const FAST: StochasticParams = StochasticParams::fast(9, 3);
const MACD: MacdParams = MacdParams::standard();
const VWAP: VwapParams = VwapParams::cumulative_typical();

struct SymbolPipeline {
    stoch: StochState,
    macd: MacdState,
    vwap: VwapState,
    ema20: EmaState,
    /// Your key type (strike, expiry, right, …) — not defined by this crate
    options: HashMap<String, BsmState>,
}

impl SymbolPipeline {
    fn new() -> FinanceResult<Self> {
        // *State::new is fallible (bad periods → Err), same idea as File::open
        Ok(Self {
            stoch: StochState::new(FAST)?,
            macd: MacdState::new(MACD)?,
            vwap: VwapState::new(VWAP)?,
            ema20: EmaState::new(20)?,
            options: HashMap::new(),
        })
    }

    /// Once per day/session (or on reconnect): seed from your store, then only push live bars.
    fn seed_history(
        &mut self,
        high: &[f64],
        low: &[f64],
        close: &[f64],
        volume: &[f64],
    ) -> FinanceResult<()> {
        self.stoch = StochState::from_history(FAST, high, low, close)?;
        self.macd = MacdState::from_history(MACD, close)?;
        self.vwap = VwapState::from_history(VWAP, high, low, close, volume)?;
        self.ema20 = EmaState::from_history(20, close)?;
        Ok(())
    }

    fn on_bar(&mut self, high: f64, low: f64, close: f64, volume: f64) -> FinanceResult<()> {
        let _kd = self.stoch.push(high, low, close)?;       // Option<(k,d)>
        let _m = self.macd.push(close)?;                    // Option<(macd,signal,hist)>
        let _vw = self.vwap.push(high, low, close, volume)?; // Option<f64>
        let _e = self.ema20.push(close)?;
        // Spot moved — re-mark the chain (or throttle / rayon in your runtime)
        for opt in self.options.values_mut() {
            opt.set_spot(close)?;
        }
        Ok(())
    }

    /// Multi-bar payload (e.g. several 1m bars in one message).
    fn on_bars(
        &mut self,
        high: &[f64],
        low: &[f64],
        close: &[f64],
        volume: &[f64],
    ) -> FinanceResult<()> {
        let _ = self.stoch.push_bars(high, low, close)?;
        let _ = self.macd.push_bars(close)?;
        let _ = self.vwap.push_bars(high, low, close, volume)?;
        let _ = self.ema20.push_bars(close)?;
        if let Some(&last) = close.last() {
            for opt in self.options.values_mut() {
                opt.set_spot(last)?;
            }
        }
        Ok(())
    }

    fn on_option_quote(&mut self, key: &str, mid: f64) -> FinanceResult<()> {
        if let Some(opt) = self.options.get_mut(key) {
            let _iv = opt.set_vol_from_price(mid)?;
        }
        Ok(())
    }

    /// Your calendar decides this — the library never auto-resets at “market open”.
    fn on_session_open(&mut self) {
        self.vwap.reset(); // common: new day VWAP
        // stoch/macd/ema often CONTINUE across days; reset only if your strategy wants it
    }
}
```

**Maximum control:** continue multi-day series (never reset), or reset VWAP/RVOL at open, or
rebuild state from a sliced history. All of that is **your** policy; we only expose the levers.

State is parity-tested against batch series (see `stocks::ta::state` tests). Also:
`SmaState`, `BollingerState`, `KeltnerState`, `RvolState`.

### Parameter order (when possible)

`rate, periods, present_value, future_value, …`

Money arguments often accept any `Into<f64>` (`i32`, `f32`, …). Rates stay `f64`; period counts are `u32` (NPER returns `f64` for fractional periods).

## Error handling

**Why.** Libraries used from web handlers, batch jobs, or CLIs must not abort when a user types `-150%` interest or period `0`.

**Canonical pattern** (structured match + `?`):

```rust
use finance_solution::{payment, FinanceError, FinanceResult};

fn loan_payment(principal: f64) -> FinanceResult<f64> {
    payment(0.09 / 12.0, 60, principal, 0.0, false)
}

match loan_payment(20_000.0) {
    Ok(pmt) => println!("payment = {pmt}"),
    Err(FinanceError::InvalidRate { rate }) => {
        eprintln!("interest rate {rate} is not usable");
    }
    Err(FinanceError::NonFinite { field, value }) => {
        eprintln!("{field} must be finite; got {value}");
    }
    Err(e) => eprintln!("finance error: {e}"),
}
```

**Affordability example** (compose with `?`):

```rust
use finance_solution::{payment, present_value, FinanceResult};

fn affordability(
    future_goal: f64,
    discount_rate: f64,
    loan_apr: f64,
    years: u32,
    loan_months: u32,
) -> FinanceResult<f64> {
    // Present value of the goal (Excel-style sign: positive FV → negative PV).
    let pv = present_value(discount_rate, years, future_goal, false)?;
    // Positive principal → negative payment (borrower cash out).
    let principal = pv.abs();
    payment(loan_apr / 12.0, loan_months, principal, 0.0, false)
}

// Call from a fallible main or handler:
// let pmt = affordability(50_000.0, 0.05, 0.06, 10, 60)?;
// pmt ≈ -593.43
```

| Quantity | Value | Meaning |
|----------|-------|---------|
| `pv` | **≈ −30,695.66** | Today’s value of $50k in 10 years at 5% |
| Loan principal | **≈ +30,695.66** | `pv.abs()` |
| Monthly payment | **≈ −593.43** | Negative = cash you pay |
| 60 × payment | **≈ −35,606** | Total cash paid over the loan |

**Early payoff / mortgage what-ifs** use the same Result APIs — see `cargo run --example early_payoff_what_if` and the amortization tables below.

**Benefits:**

1. **No surprise panics** on bad external input  
2. **Matchable variants** for UI / metrics (`FinanceError::code()`)  
3. **`std::error::Error` + `Display`** for logging and `?` into app error types  
4. **One name per formula** — fallibility lives in the return type  

## Amortization: solution / series / tables

```text
amortization_solution  →  .series()  →  .print_table()
       ↓                      ↓
   formulas + payment    period rows (principal, interest, balance, formulas)
```

```rust
use finance_solution::*;

let rate = 0.08 / 12.0;
let periods = 12;
let principal = 10_000.0;

let solution = amortization_solution(rate, periods, principal, 0.0, false)
    .expect("fixture: 12-month demo loan");
println!("{}", solution.formula());

let series = solution.series();
assert_eq!(series.len(), periods as usize);
series.print_table(true, true);

assert_approx_equal!(solution.ipmt(1).unwrap(), series[0].interest());
assert_approx_equal!(solution.ppmt(1).unwrap(), series[0].principal());
let _first_half = solution.cumipmt(1, 6).unwrap();
```

Sample `print_table(true, true)` (12-month $10k loan at 8% APR monthly):

```text
period    payment  principal  interest     balance  principal_to_date  interest_to_date  payments_to_date  principal_remaining  interest_remaining  payments_remaining
------  ---------  ---------  --------  ----------  -----------------  ----------------  ----------------  -------------------  ------------------  ------------------
     1  -869.8843  -803.2176  -66.6667  9_196.7824          -803.2176          -66.6667         -869.8843          -9_196.7824           -371.9448         -9_568.7272
     2  -869.8843  -808.5724  -61.3119  8_388.2100        -1_611.7900         -127.9785       -1_739.7686          -8_388.2100           -310.6329         -8_698.8429
   ...
    12  -869.8843  -864.1235   -5.7608     -0.0000       -10_000.0000         -438.6115      -10_438.6115               0.0000             -0.0000              0.0000
```

| Need | API |
|------|-----|
| One Excel cell (`IPMT` month 3) | `ipmt(...)?` / `solution.ipmt(3)?` |
| Full schedule | `amortization_solution(...)?.series()` |
| Terminal table | `.print_table()` / `.print_table_locale()` |
| Safe construction | `amortization_solution` → `FinanceResult` |

## Returns & stocks

```rust
use finance_solution::*;

let s = doubling_solution(0.08).expect("fixture");
s.print_table();
doubling_compare_rates(&[0.01, 0.05, 0.08, 0.12])
    .expect("fixture rates")
    .print_table();

let prices = [100.0, 102.0, 101.0, 108.0, 105.0, 112.0, 110.0, 118.0, 115.0, 125.0];
let path = price_path_solution(
    &prices,
    PricePathOptions::new(12.0).with_years(prices.len() as f64 / 12.0),
)
.expect("fixture prices");
path.print_summary();
path.series().print_table();
```

## Testing & CI

```bash
cargo test
cargo test --doc
cargo clippy --all-targets -- -D warnings
cargo bench   # optional Criterion suites (local / on demand)
```

Symmetry tests, Excel golden values, integration tests, and **TA proptests** (batch ↔ stream parity) cover TVM, cashflow, amortization, returns, stocks, and TA. Derivatives unit tests cover BSM price/Greeks/IV/parity and `BsmState`. CI runs build, tests, doctests, clippy, and fmt on **stable** and **MSRV 1.70**, and asserts there are no public `try_*` dual-API symbols.

## Package notes

- **crates.io** ships the library + small examples; large OHLCV under `examples/data/` is **excluded** (see `Cargo.toml` `exclude`).
- Clone from **GitHub** if you want `cargo run --example real_bars_ta_bsm --release`.

## License

MIT — see [LICENSE](LICENSE).
