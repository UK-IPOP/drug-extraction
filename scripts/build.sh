#!/usr/bin/env bash

echo "Building for MacOS x ARM"
env GOOS=darwin GOARCH=arm64 go build -o bin/drug-extraction-MacOS main.go

echo "Building for Linux x AMD64"
env GOOS=linux GOARCH=amd64 go build -o bin/drug-extraction-Linux main.go
