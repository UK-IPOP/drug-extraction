#! /bin/bash

echo Starting python section...
cd python
poetry run python main.py
poetry run python main.py

echo Starting go section...
cd ../go
./searcher
./searcher

echo Starting rust section...
cd ../rust
./target/release/search
./target/release/search