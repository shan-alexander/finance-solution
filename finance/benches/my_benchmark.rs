use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {

    c.bench_function("present_value f64", |b| b.iter(|| 
        finance::present_value(black_box(0.034), black_box(5), black_box(250_000), false)));
    
    c.bench_function("present_value solution", |b| b.iter(|| 
        finance::present_value_solution(black_box(0.034), black_box(5), black_box(250_000), false)));

    c.bench_function("pv annuity solution", |b| b.iter(|| 
        finance::present_value_annuity_solution(
            black_box(0.034), 
            black_box(5), 
            black_box(1_000), 
            black_box(false))));
    
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);