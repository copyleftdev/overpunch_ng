#![allow(clippy::disallowed_methods)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use overpunch_ng::{convert_from_signed_format, convert_to_signed_format, extract, format};
use rust_decimal::Decimal;
use std::str::FromStr;

fn bench_extract(c: &mut Criterion) {
    c.bench_function("extract (ebcdic)", |b| {
        b.iter(|| {
            black_box(extract(black_box("1234567G"), black_box(3))).unwrap();
        })
    });
}

fn bench_format(c: &mut Criterion) {
    let val = Decimal::from_str("1234.567").unwrap();
    c.bench_function("format (ebcdic)", |b| {
        b.iter(|| {
            black_box(format(black_box(val), black_box(3))).unwrap();
        })
    });
}

fn bench_convert_from(c: &mut Criterion) {
    c.bench_function("convert_from_signed_format", |b| {
        b.iter(|| {
            black_box(convert_from_signed_format(
                black_box("1234567G"),
                black_box("s9(7)v999"),
            ))
            .unwrap();
        })
    });
}

fn bench_convert_to(c: &mut Criterion) {
    let val = Decimal::from_str("1234.567").unwrap();
    c.bench_function("convert_to_signed_format", |b| {
        b.iter(|| {
            black_box(convert_to_signed_format(
                black_box(val),
                black_box("s9(7)v999"),
            ))
            .unwrap();
        })
    });
}

criterion_group!(
    benches,
    bench_extract,
    bench_format,
    bench_convert_from,
    bench_convert_to,
);
criterion_main!(benches);
