#! /bin/bash

echo Starting python section...
cd python
poetry run python main.py
poetry run python main.py

echo Starting go section...
cd ../go
go build .
./searcher
./searcher

echo Starting rust section...
cd ../rust
cargo build --release
./target/release/search
./target/release/search