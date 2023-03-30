use std::io::{BufWriter, Write};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use itertools::Itertools;
use rayon::prelude::{IntoParallelIterator, ParallelBridge, ParallelIterator};

pub fn bench_loops(c: &mut Criterion) {
    c.bench_function("standard loop", |b| {
        b.iter(|| {
            let n = black_box(10_000_000);
            let _: Vec<_> = (0..n).into_iter().map(|x| x * x).collect();
        })
    });
    c.bench_function("parallel loop", |b| {
        b.iter(|| {
            let n = black_box(10_000_000);
            let _: Vec<_> = (0..n).into_par_iter().map(|x| x * x).collect();
        })
    });
    c.bench_function("standard nested loop", |b| {
        b.iter(|| {
            let n = black_box(10_000_000);
            let _: Vec<_> = (0..n)
                .into_iter()
                .map(|x| (0..n).into_iter().map(move |y| x * y))
                .collect();
        })
    });
    c.bench_function("parallel (single) nested loop", |b| {
        b.iter(|| {
            let n = black_box(10_000_000);
            let _: Vec<_> = (0..n)
                .into_par_iter()
                .map(|x| (0..n).into_iter().map(move |y| x * y))
                .collect();
        })
    });
    c.bench_function("parallel (double) nested loop", |b| {
        b.iter(|| {
            let n = black_box(10_000_000);
            let _: Vec<_> = (0..n)
                .into_par_iter()
                .map(|x| (0..n).into_par_iter().map(move |y| x * y))
                .collect();
        })
    });
    c.bench_function("parallel (triple) nested loop", |b| {
        b.iter(|| {
            let n = black_box(10_000_000);
            let _: Vec<_> = (0..n)
                .into_par_iter()
                .map(|x| (0..n).map(move |y| (0..n).map(move |z| x * y * z)))
                .collect();
        })
    });
    c.bench_function("parallel (triple) nested loop", |b| {
        b.iter(|| {
            let n = black_box(10_000_000);
            let _: Vec<_> = (0..n)
                .into_iter()
                .map(|x| {
                    (0..n)
                        .into_iter()
                        .map(move |y| (0..n).into_iter().map(move |z| x * y * z))
                })
                .collect();
        })
    });
}

pub fn bench_io(c: &mut Criterion) {
    let mut group = c.benchmark_group("io");
    group.bench_function("read buffered", |b| {
        b.iter(|| {
            let f = File::open("./data/records.csv").unwrap();
            let rdr = BufReader::new(f);
            let _: Vec<Vec<String>> = rdr
                .lines()
                .map(|l| {
                    l.unwrap()
                        .split(',')
                        .map(|x| x.to_owned())
                        .collect::<Vec<String>>()
                })
                .collect();
        });
    });
    group.bench_function("read parallel buffered", |b| {
        b.iter(|| {
            let f = File::open("./data/records.csv").unwrap();
            let rdr = BufReader::new(f);
            let _: Vec<Vec<String>> = rdr
                .lines()
                .par_bridge()
                .map(|l| {
                    l.unwrap()
                        .split(',')
                        .map(|x| x.to_owned())
                        .collect::<Vec<String>>()
                })
                .collect();
        });
    });
    group.bench_function("read parallel csv", |b| {
        b.iter(|| {
            let mut rdr = csv::Reader::from_path("./data/records.csv").unwrap();
            let _: Vec<csv::StringRecord> =
                rdr.records().par_bridge().map(|l| l.unwrap()).collect();
        });
    });
    group.bench_function("read csv", |b| {
        b.iter(|| {
            let mut rdr = csv::Reader::from_path("./data/records.csv").unwrap();
            let _: Vec<csv::StringRecord> = rdr.records().map(|l| l.unwrap()).collect();
        });
    });
    // group.bench_function("write print", |b| {
    //     b.iter(|| {
    //         println!("hi");
    //     });
    // });
    // group.bench_function("write_buffer", |b| {
    //     b.iter(|| {
    //         let mut wtr = BufWriter::new(std::io::stdout().lock());
    //         writeln!(wtr, "hi").unwrap();
    //     });
    // });
    group.finish();
}

pub fn bench_combos(c: &mut Criterion) {
    let mut group = c.benchmark_group("combos");

    group.bench_function("standard looping", |b| {
        b.iter(|| {
            let _: Vec<(i64, i64)> = (0..1_000)
                .flat_map(|x| (0..1_000).map(move |y| (x, y)).collect::<Vec<(i64, i64)>>())
                .collect();
        })
    });
    group.bench_function("parallel (1) looping", |b| {
        b.iter(|| {
            let _: Vec<(i64, i64)> = (0..1_000)
                .into_par_iter()
                .flat_map(|x| (0..1_000).map(move |y| (x, y)).collect::<Vec<(i64, i64)>>())
                .collect();
        })
    });
    group.bench_function("parallel (2) looping", |b| {
        b.iter(|| {
            let _: Vec<(i64, i64)> = (0..1_000)
                .into_par_iter()
                .flat_map(|x| {
                    (0..1_000)
                        .into_par_iter()
                        .map(move |y| (x, y))
                        .collect::<Vec<(i64, i64)>>()
                })
                .collect();
        })
    });
    group.bench_function("itertools looping", |b| {
        b.iter(|| {
            let _: Vec<(i64, i64)> = (0..1_000)
                .cartesian_product((0..1_000).into_iter())
                .collect();
        })
    });
}

// criterion_group!(benches, bench_loops);
criterion_group!(benches, bench_io);
// criterion_group!(benches, bench_combos);
criterion_main!(benches);
