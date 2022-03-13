#! /bin/bash

# use to run tests in all languages

cd go-lang
make test-save

cd ../python-lang
make test-save

cd ../rust-lang
make test-save
