use criterion::{black_box, criterion_group, criterion_main, Criterion};
// use rust_decimal::Decimal; // Import the Decimal type
// use rust_decimal_macros::*; // Procedural macros need importing directly

use experiment::verbose::present_value;

fn criterion_benchmark(c: &mut Criterion) {

    c.bench_function("present value PVS", |b| b.iter(|| 
        present_value::present_value(black_box(0.034f64), black_box(250_000f64), black_box(5))));
    
    c.bench_function("pv_1 f64", |b| b.iter(|| 
        present_value::pv_1(black_box(0.034f64), black_box(250_000f64), black_box(5))));
    
    c.bench_function("pv_2 f64", |b| b.iter(|| 
        present_value::pv_2(black_box(0.034f64), black_box(250_000f64), black_box(5))));
        
    
        // c.bench_function("present value decimal", |b| b.iter(|| 
        //     present_value::present_value(black_box(dec!(0.034)), black_box(dec!(250_000)), black_box(5))));
}


criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);