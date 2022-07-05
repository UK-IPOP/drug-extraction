#! /bin/bash

filenames=(
    "go-lang/logs/bench_results.log"
    "python-lang/logs/bench_results.log"
    "python-lang/logs/bench_hist.svg"
    "r-lang/logs/bench_results.log"
    "fast-python-lang/logs/bench_results.log"
    "fast-python-lang/logs/bench_hist.svg"
    "rust-lang/logs/bench_results.log"
    "data/results.log"
)

for f in ${filenames[@]}; do 
    rm $f
done
