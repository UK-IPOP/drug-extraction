#! /bin/bash

docker build -t go-benchmarks -f go-lang/Dockerfile ./go-lang

docker build -t python-benchmarks -f python-lang/Dockerfile ./python-lang

docker build -t rust-benchmarks -f rust-lang/Dockerfile ./rust-lang

docker run go-benchmarks

docker run python-benchmarks

docker run rust-benchmarks

docker cp rust-benchmarks:/rust-app/target/criterion/ ./rust-report

open rust-report/reports/Algorithms/index.html