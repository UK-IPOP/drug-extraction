#! /bin/bash

# script to run the benchmarks for all three languages

cd go-lang
make bench

cd ../python-lang
make bench

cd ../rust-lang
make bench
make report
