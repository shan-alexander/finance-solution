use criterion::{black_box, criterion_group, criterion_main, Criterion};

use experiment::verbose::present_value;
use experiment::verbose::weird_enum_solver::{TVM};

fn criterion_benchmark(c: &mut Criterion) {

    c.bench_function("present value PVS", |b| b.iter(|| 
        present_value::present_value(black_box(0.034f64), black_box(250_000f64), black_box(5))));
    
    c.bench_function("pv_1 f64", |b| b.iter(|| 
        present_value::pv(black_box(0.034f64), black_box(5), black_box(250_000f64))));
    
    c.bench_function("enum pv f64", |b| b.iter(|| 
        TVM::PV { period_rate: black_box(0.034f64), period_count: black_box(5), fv: black_box(250_000f64) } ));
    
 
}


criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);