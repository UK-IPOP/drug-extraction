#!/usr/bin/env bash

echo "Building for MacOS x ARM"
env GOOS=darwin GOARCH=arm64 go build -o bin/drug_extraction_macos_arm main.go

echo "Building for MacOS x AMD64"
env GOOS=darwin GOARCH=amd64 go build -o bin/drug_extraction_macos_amd main.go

echo "Building for Linux x AMD64"
env GOOS=linux GOARCH=amd64 go build -o bin/drug_extraction_linux_amd main.go

#echo "Building for Windows x AMD64"
#env GOOS=linux GOARCH=amd64 go build -o bin/Windows/drug-extraction.exe main.go