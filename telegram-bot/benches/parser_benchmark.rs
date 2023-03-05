use criterion::{black_box, criterion_group, criterion_main, Criterion};
use telegram_chatbot::parser::{
    is_spending_reset_request, is_spending_total_request, parse_budget_request,
    parse_metro_request, parse_spending_request,
};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse metro req", |b| {
        b.iter(|| parse_metro_request(black_box("West Cortex".to_string())));
    });

    c.bench_function("parse spending request", |b| {
        b.iter(|| parse_spending_request(black_box("Spent 10.37 Grocery".to_string())));
    });

    c.bench_function("parse budget request", |b| {
        b.iter(|| parse_budget_request(black_box("Budget 5000".to_string())));
    });

    c.bench_function("spending reset request", |b| {
        b.iter(|| is_spending_reset_request(black_box("Spent reset".to_string())));
    });

    c.bench_function("spending total request", |b| {
        b.iter(|| is_spending_total_request(black_box("Spent total".to_string())));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
