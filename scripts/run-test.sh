#!/usr/bin/env bash

go run main.go extract "data/pokemon.csv" --id-col "#" --target-col "Name"
