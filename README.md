# Drug Extraction Performance Comparison

## Performance Analysis

This branch exists for performance analysis comparing Python, Golang, and Rust performance on string similarity metrics.

Each directory (`python-lang`, `go-lang`, `rust-lang`) contains various benchmarks and tests for algorithms that overlapped in the three packages selected.

### Built With

- [pytest](https://github.com/pytest-dev/pytest)
- [pytest-benchmark](https://github.com/ionelmc/pytest-benchmark/)
- [go-edlib](https://github.com/hbollon/go-edlib)
- [criterion](https://github.com/bheisler/criterion.rs)
- [cargo-criterion](https://github.com/bheisler/cargo-criterion)

## Getting Started

To get a local copy up and running follow these simple example steps.

### Prerequisites

This is an example of how to list things you need to use the software and how to install them.

<!-- Add versions and links here -->

- Rust
- Go
- Python
- Poetry
- Make
- Git

### Installation

Download project using git:

`git clone -b perf-comp https://github.com/UK-IPOP/drug-extraction`

which gives you the performance comparison branch 😃

then simply `cd drug-extraction` to get into the active directory.

## Usage

### Benchmarks

In order to successfully run the benchmarks

To actually run the benchmarks first change the permissions on the script: `chmod u+x scripts/run-benchmarks.sh` then simply `./scripts/run-benchmarks.sh`

The results from each log file (`<LANGUAGE-FOLDER>/logs/bench_results.log`) are appended each benchmarking run and are manually examined and entered into a [spreadsheet](data/bench_results.csv) for easy analysis and cross-language comparison.

> The time recorded is _averaged_ (~100,000 iterations) for each algorithm/comparison run.

### Tests

To get the tests running follow the same procedures as setting up benchmarks. If you have already run benchmarks then you are all set.

To actually run the tests first change the permissions on the script: `chmod u+x scripts/run-tests.sh` then simply `./scripts/run-tests.sh`

The tests for each language are written to identify cases where different languages/packages may have different implementations of an algorithm. The goal of the tests is not code coverage, but to identify cases where different languages may return different edit distances due to the nature of their implementations.

The results of the tests are manually compiled from their log-files (`<LANGUAGE-FOLDER>/logs/test_results.log`) into another spreadsheet for easy analysis and cross-language comparison.

## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

## Contact

Nick Anthony - [@Nanthony95](https://twitter.com/Nanthony95) - nanthony@gmail.com

## Acknowledgments

- Chris Delcher - Twitter: [@chris_delcher](https://twitter.com/chris_delcher) - University [Profile](https://pharmacy.uky.edu/people/chris-delcher)
- Daniel Harris - University [Profile](https://pharmacy.uky.edu/people/daniel-harris)
- Michelle Duong - Twitter: [@mduong26](https://twitter.com/mduong26) - GitHub: [@mduong26](https://github.com/mduong26)
