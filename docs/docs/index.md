# Welcome

This is a CLI (command-line-interface) tool designed for 
people wanting to extract drugs from free-text.   

## Quickstart

Typical usage pattern is something like:

```bash
drug-extraction extract <filename>.csv --id-col "id" --target-col "text"
```

More advanced usage:
```bash
drug-extraction pipeline <filename>.csv --id-col "#" --target-col "Name" --format --format-type=csv
```

## Installation

### Tool Installation

Download the command line tool from GitHub [Releases](https://github.com/UK-IPOP/drug-extraction/releases)
for your corresponding operating system and architecture.

### Package Installation

Retrieve the package code from GitHub.

```bash
go get github.com/UK-IPOP/drug-extraction
```

## Getting Started

### Possible Configurations

Commands:
- extract
- clean
- format
- pipeline
- --target-col
- --id-col
- --format
- --format-type
- --strict
- --clean

