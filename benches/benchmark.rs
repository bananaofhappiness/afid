use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs::File;
use afid::bench;

fn criterion_benchmark(c: &mut Criterion){
    let file1 = File::open("test_texts/latin1.txt").unwrap();
    let file2 = File::open("test_texts/latin2.txt").unwrap();
    c.bench_function("128k", |b| b.iter(|| bench::compare_files_128k(black_box(&file1), black_box(&file2))));
    c.bench_function("64k", |b| b.iter(|| bench::compare_files_64k(black_box(&file1), black_box(&file2))));
    c.bench_function("32k", |b| b.iter(|| bench::compare_files_32k(black_box(&file1), black_box(&file2))));
    c.bench_function("16k", |b| b.iter(|| bench::compare_files_16k(black_box(&file1), black_box(&file2))));
    c.bench_function("8k", |b| b.iter(|| bench::compare_files_8k(black_box(&file1), black_box(&file2))));
    c.bench_function("4k", |b| b.iter(|| bench::compare_files_4k(black_box(&file1), black_box(&file2))));
    c.bench_function("2k", |b| b.iter(|| bench::compare_files_2k(black_box(&file1), black_box(&file2))));
    c.bench_function("1k", |b| b.iter(|| bench::compare_files_1k(black_box(&file1), black_box(&file2))));
    c.bench_function("512", |b| b.iter(|| bench::compare_files_512(black_box(&file1), black_box(&file2))));
    c.bench_function("256", |b| b.iter(|| bench::compare_files_256(black_box(&file1), black_box(&file2))));
    c.bench_function("128", |b| b.iter(|| bench::compare_files_128(black_box(&file1), black_box(&file2))));
    c.bench_function("64", |b| b.iter(|| bench::compare_files_64(black_box(&file1), black_box(&file2))));
    c.bench_function("32", |b| b.iter(|| bench::compare_files_32(black_box(&file1), black_box(&file2))));
    c.bench_function("16", |b| b.iter(|| bench::compare_files_16(black_box(&file1), black_box(&file2))));
    c.bench_function("8", |b| b.iter(|| bench::compare_files_8(black_box(&file1), black_box(&file2))));
    c.bench_function("4", |b| b.iter(|| bench::compare_files_4(black_box(&file1), black_box(&file2))));
    c.bench_function("2", |b| b.iter(|| bench::compare_files_2(black_box(&file1), black_box(&file2))));
    c.bench_function("1", |b| b.iter(|| bench::compare_files_1(black_box(&file1), black_box(&file2))));
    
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);