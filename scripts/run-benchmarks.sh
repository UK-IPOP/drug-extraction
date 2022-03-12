#! /bin/bash

cd go-lang
make bench-save

cd ../python-lang
make bench-save

cd ../rust-lang
make bench
make report
