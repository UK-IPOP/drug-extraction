#! /bin/bash

docker build -t go-benchmarks -f go-lang/Dockerfile . 

docker build -t python-benchmarks -f python-lang/Dockerfile .

docker build -t rust-benchmarks -f rust-lang/Dockerfile .

docker run go-benchmarks

docker run python-benchmarks

docker run rust-benchmarks

docker cp rust-benchmarks:/rust-app/target/criterion/ ./rust-report

open rust-report/reports/Algorithms/index.html