![logo](../images/logo.png)

# Drug Extraction CLI

- [Drug Extraction CLI](#drug-extraction-cli)
  - [Demo](#demo)
  - [Description](#description)
  - [Requires](#requires)
  - [Installation](#installation)
    - [Python Developers / Data Scientists](#python-developers--data-scientists)
    - [Rust Developers](#rust-developers)
  - [Usage](#usage)
    - [Interactive](#interactive)
    - [Search](#search)
  - [Output Data Dictionary](#output-data-dictionary)
  - [Examples](#examples)
  - [Support](#support)
  - [Contributing](#contributing)
  - [MIT License](#mit-license)

## Demo

![demo-gif](../images/demo.gif)

## Description

This application takes a CSV file and parses text records from another CSV file to detect and extract search term mentions using string similarity algorithms to account for common misspellings. It is named for the drug searching it does most commonly for us at IPOP but is flexible enough to accept any type search terms.

If you are wondering about specific use cases, check out the [Examples](../examples/) folder!

## Requires

- [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) package manager (rust toolchain)
- [just](https://github.com/casey/just) (optional dev-dependency if you clone this repo)

## Installation

To install the drug-extraction-cli application, simply:

### Python Developers / Data Scientists

Please use [pipx](https://pypa.github.io/pipx/) since it is designed *specifically* for this use case of installing Python CLI apps into isolated virtual environments.

```bash
pipx install extract-drugs
```

### Rust Developers

```bash
cargo install drug-extraction-cli
```

> **IMPORTANT!** Both of these will install an executable called `extract-drugs`.
>
> No matter how you install the package from either packaging index, the binary program will be named `extract-drugs` for more intuitive commands.
>
> INFO: The naming discrepancy is due to to how `maturin` handles package names and wanting to both keep the same CLI command/name and maintain the Rust namespace. Apologies, but you'll be fine 🙂.

## Usage

This application has two commands: `interactive` and `search`. Both of these commands have the same underlying functionality, the latter allows you to pass command-line arguments and is better suited to automated processing or advanced users while the former allows interactive declaration of the same configuration options and is better for new or first time users.

API documentation for the library can be found on [docs.rs](https://docs.rs/crate/drug-extraction-cli/latest).

### Interactive

This will present you with a series of prompts to help you select correct options. Highly recommended for new users or one-off runs.

Usage:

```bash
extract-drugs interactive
```

This command is demoed in the GIF above.

### Search

`search` functions the same as `interactive` but allows you to declaratively provide the configuration options.

## Output Data Dictionary

This tool will output an `output.csv` file with the following format:

|   Column Name    |                                     Description                                      |   Data Type    |                  Limits/Ranges                   |
| :--------------: | :----------------------------------------------------------------------------------: | :------------: | :----------------------------------------------: |
|      row_id      |   Identifier from `--id-col` if provided, else line number of row in `--data-file`   |     String     |                       None                       |
|   search_term    | The search term, cleaned and normalized. This is the actual term that was compared.  |     String     |                       None                       |
|   matched_term   | The matched term, cleaned and normalized. This is the actual term that was compared. |     String     |                       None                       |
|      edits       |                               The `osa` edit distance                                |    Integer     |     0-2 (top limit due to exclusion filter)      |
| similarity_score |                         The `jaro_winkler` similarity score                          |     Float      | 0.95-1.0  (bottom limit due to exclusion filter) |
|   search_field   |             The field that this match was found in, from `--search-cols`             |     String     |                       None                       |
|     metadata     |           The attached metadata to `search_term` in the search_terms file            | String or None |                       None                       |

## Examples

For a whole showcase of example runs of this tool check out the shell scripts inside the [examples](../examples/) folder.

For a showcase of potential analytical value that can be derived from running this tool, checkout the Jupyter Notebooks in the same folder!

## Support

If you encounter any issues or need support please either contact [@nanthony007](<[github.com/](https://github.com/nanthony007)>) or [open an issue](https://github.com/UK-IPOP/drug-extraction/issues/new).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## MIT License

[LICENSE](../LICENSE)
