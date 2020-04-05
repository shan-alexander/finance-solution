use criterion::{black_box, criterion_group, criterion_main, Criterion};

use experiment::verbose::weird_enum_solver::{TVM};

fn criterion_benchmark(c: &mut Criterion) {

    c.bench_function("present value PVS", |b| b.iter(|| 
        finance::present_value(black_box(0.034), black_box(250_000), black_box(5))));
    
    c.bench_function("pv_1 f64", |b| b.iter(|| 
        finance::present_value(black_box(0.034), black_box(5), black_box(250_000))));
    
    c.bench_function("enum pv f64", |b| b.iter(|| 
        TVM::PV { period_rate: black_box(0.034), period_count: black_box(5), fv: black_box(250_000f64) } ));

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

// Notes on present_value_solution():
// Bench: 1.4776 us  when including PeriodValues
// Bench: 26.650 ns  when removing the PeriodValues calculation

