#! /bin/bash

# use to run tests in all languages

cd go-lang
make test

cd ../rust-lang
make test

cd ../python-lang
make test
