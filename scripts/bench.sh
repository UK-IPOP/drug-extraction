#! /bin/bash

echo "Building docker images..."
docker build -t go-benchmarks -f go-lang/Dockerfile ./go-lang

docker build -t python-benchmarks -f python-lang/Dockerfile ./python-lang

docker build -t r-benchmarks -f r-lang/Dockerfile ./r-lang

docker build -t fast-python-benchmarks -f fast-python-lang/Dockerfile ./fast-python-lang

docker build -t rust-benchmarks -f rust-lang/Dockerfile ./rust-lang

echo "Docker images built, running benchmarks..."

echo "Running go benchmarks..."
docker run --rm --name go-benchmarks go-benchmarks

echo "Running python benchmarks..."
docker run --name python-benchmarks python-benchmarks
docker cp python-benchmarks:/python-app/logs/bench_hist.svg ./data/python_hist.svg
docker rm python-benchmarks

echo "Running fast-python benchmarks..."
docker run --rm --name fast-python-benchmarks fast-python-benchmarks
docker cp fast-python-benchmarks:/fast-python-app/logs/bench_hist.svg ./data/fast_python_hist.svg
docker rm fast-python-benchmarks

echo "Running r benchmarks..."
docker run --rm --name r-benchmarks r-benchmarks

echo "Running rust benchmarks..."
docker run --name rust-benchmarks rust-benchmarks 

echo "Benchmarks complete!"
docker cp rust-benchmarks:/rust-app/target/criterion/ ./data/rust-report
docker rm rust-benchmarks

# open images and reports
open data/python_hist.svg
open data/fast_python_hist.svg
open data/rust-report/reports/Algorithms/index.html
