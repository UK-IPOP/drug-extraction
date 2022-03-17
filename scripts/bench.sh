#! /bin/bash

docker build -t go-benchmarks -f go-lang/Dockerfile ./go-lang

docker build -t python-benchmarks -f python-lang/Dockerfile ./python-lang

docker build -t rust-benchmarks -f rust-lang/Dockerfile ./rust-lang

docker run --rm --name go-benchmarks go-benchmarks

docker run --rm --name python-benchmarks python-benchmarks

docker run --name rust-benchmarks rust-benchmarks 

docker cp rust-benchmarks:/rust-app/target/criterion/ ./rust-report

docker rm rust-benchmarks

open rust-report/reports/Algorithms/index.html