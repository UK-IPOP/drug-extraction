# Drug Extraction Core

This is the core library used by the [CLI](https://github.com/UK-IPOP/drug-extraction/tree/main/cli) and [Web](https://github.com/UK-IPOP/drug-extraction/tree/main/web) application interfaces.

As noted in the main [ToolBox](https://github.com/UK-IPOP/drug-extraction) documentation, this library's development will be driven heavily by the needs of the CLI application.

## Description

Drugs, as defined by the `Drug` type include a `name`, `rx_cui`, `rx_class_id`, and `rx_class_relasource`. Technically these could be manually compiled, but we recommend fetching from the [RxClass API](https://lhncbc.nlm.nih.gov/RxNav/APIs/RxClassAPIs.html).

For an example, see [fetch_drugs()](https://github.com/UK-IPOP/drug-extraction/blob/67adf274f9493cfefd85c6e00f9ed2329797c113/cli/src/utils.rs#L643) from the CLI application.

## Requires

- [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) package manager (rust toolchain)
- [just](https://github.com/casey/just) (optional dev-dependency if you clone this repo)

## Installation

Cargo is available as a part of the Rust toolchain and is readily available via curl + sh combo (see [here](https://doc.rust-lang.org/cargo/getting-started/installation.html)).

To install the drug-extraction-core library, simply:

```toml
drug-extraction-core = 0.1.0
```

inside your `Cargo.toml`.

## Usage

A simple usage example:

```rust
let search = DrugSearch::new(
    Algorithm::Levenshtein,
    levenshtein,
    None,
    None,
    &["hello", "world"]
);
let results = search.scan("hello world", None);
println!("{:?}", results);
```

We support both CSV and JSONL output.

Full API documentation can be found on [docs.rs](https://docs.rs/drug-extraction-core/latest/drug-extraction-core/).

## Example Workflow

For analytical purposes, I would suggest combining all of these jsonlines files into one larger file and then you can process it with a tool like [jq](https://stedolan.github.io/jq/) or Python - [Pandas](https://pandas.pydata.org) depending on your use case. 🙂

## Support

If you encounter any issues or need support please either contact [@nanthony007](<[github.com/](https://github.com/nanthony007)>) or [open an issue](https://github.com/UK-IPOP/drug-extraction/issues/new).

## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

See [CONTRIBUTING.md](CONTRIBUTING.md) for more details. 😃

## MIT License

[LICENSE](LICENSE)
