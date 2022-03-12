use criterion::{criterion_group, criterion_main, Criterion};

const S1: &str = "alcohol";
const S2: &str = "acloholism";

pub fn bench_str_sim(c: &mut Criterion) {
    let mut group = c.benchmark_group("Algorithms");
    group.significance_level(0.01).sample_size(100000);
    group.bench_function("Levenshtein", |b| b.iter(|| strsim::levenshtein(S1, S2)));
    group.bench_function("Jaro-Winkler", |b| b.iter(|| strsim::jaro_winkler(S1, S2)));
    group.bench_function("Damerau-Levenshtein", |b| {
        b.iter(|| strsim::damerau_levenshtein(S1, S2))
    });
    group.bench_function("Sorensen-Dice", |b| {
        b.iter(|| strsim::sorensen_dice(S1, S2))
    });
    group.bench_function("Optimal String Alignment", |b| {
        b.iter(|| strsim::osa_distance(S1, S2))
    });
    group.finish();
}

criterion_group!(benches, bench_str_sim);
criterion_main!(benches);
