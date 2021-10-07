#!/usr/bin/env bash

echo "Testing CLI tool..."

go run main.go extract "data/pokemon.csv" --id-col "#" --target-col "Name"
