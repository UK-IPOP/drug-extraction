# CHANGELOG

The primary point of this manually written CHANGELOG is to provide explanation for
significant architectural or UI changes between releases both major and minor.
Details about code changes, commits, etc. can be found on the [Releases](https://github.com/UK-IPOP/drug-extraction/releases) page.

## 2023-05-26 -- Version 1.0.0

Highlights:

- We no longer support `de-workflow`, but it is still available on PyPI as a previous release.
  - Deprecated due to limited usage and easily replaceable by scripts
- We no longer support `drug-extraction-core`
  - This functionality has moved to the `drug-extraction-cli` library (lib.rs)
  - The purpose of the "`core`" crate was to have something that could be built into WASM and extended, but using maturin for a PyO3 binary removes this need and we have no plans for web assembly support at this time.
- Website now serves as an interactive hub, contains no functionality of prior site.
