use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use strsim;

fn bench_edit_algorithms(c: &mut Criterion) {
    const A: &'static str = "alcohol";
    const B: &'static str = "alcoholic";

    let mut group = c.benchmark_group("edit algorithms");

    group.bench_with_input(
        BenchmarkId::new("levenshtein", "levenshtein"),
        &(A, B),
        |b, (x, y)| b.iter(|| strsim::levenshtein(x, y)),
    );
    group.bench_with_input(
        BenchmarkId::new("damerau", "damerau"),
        &(A, B),
        |b, (x, y)| b.iter(|| strsim::damerau_levenshtein(x, y)),
    );
    group.bench_with_input(BenchmarkId::new("osa", "osa"), &(A, B), |b, (x, y)| {
        b.iter(|| strsim::osa_distance(x, y))
    });
    group.finish();
}

fn bench_similarity_algorithms(c: &mut Criterion) {
    const A: &'static str = "alcohol";
    const B: &'static str = "alcoholic";

    let mut group = c.benchmark_group("similarity algorithms");

    group.bench_with_input(BenchmarkId::new("jaro", "jaro"), &(A, B), |b, (x, y)| {
        b.iter(|| strsim::jaro(x, y))
    });
    group.bench_with_input(
        BenchmarkId::new("jaro-winkler", "jaro-winkler"),
        &(A, B),
        |b, (x, y)| b.iter(|| strsim::jaro_winkler(x, y)),
    );
    group.bench_with_input(
        BenchmarkId::new("sorensen-dice", "sorensen-dice"),
        &(A, B),
        |b, (x, y)| b.iter(|| strsim::sorensen_dice(x, y)),
    );
    group.finish();
}

criterion_group!(benches, bench_edit_algorithms, bench_similarity_algorithms);
criterion_main!(benches);
