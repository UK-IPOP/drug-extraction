# Drug Extraction Tool

## Welcome

This is a CLI (command-line-interface) tool designed for
people wanting to extract drugs from free-text. It also supports a web-driven
UI for less technical users, although the web capabilities are more limited.

## [Web Docs](https://uk-ipop.github.io/drug-extraction/)

## [API Docs / GoDoc](https://pkg.go.dev/github.com/UK-IPOP/drug-extraction@v0.2.0)

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

## Road Map

I am currently working on improving the provided [drug_info.yaml](pkg/models/drug_info.yaml)
file by optimizing the search terms based on performance analysis. Once this is done I have
a larger road map of features you can see [here](https://uk-ipop.github.io/drug-extraction/upcoming/).

## Contributing

See [Contributing](./CONTRIBUTING.md)

## License

[MIT License](LICENSE)
