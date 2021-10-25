---
hide:
  - navigation
---

# Welcome

This is a CLI (command-line-interface) tool designed for
people wanting to extract drugs from free-text.

## Quick Start

Simplest usage:

```bash
drug-extraction
```

More advanced usage:

```bash
drug-extraction pipeline pokemon.csv --id-col "#" --target-col "Name" --format --format-type=csv
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
