# Notes

We utilize string similarity algorithms as defined and implemented by [Danny Guo](https://github.com/dguo)
in the [str-sim](https://github.com/dguo/strsim-rs) package.

For more information on string similarity algorithms, please consult [this](https://en.wikipedia.org/wiki/String_metric) Wikipedia page for a **comparison/list** of algorithms
and [this](https://en.wikipedia.org/wiki/Edit_distance) page for an explanation of string metrics more generally.

We've chosen to utilize a mono-repo format for this project. This is our first implemnation of the mono-repo structure when using Rust. If it causes more problems than
benefits, we will switch to independent repositories for each project.

This repository contains multiple projects:

- A command line tool (available via cargo install)
- A core Rust library for parsing strings and comparing them to common drugs
  - This is also configurable for custom search options and integration with the popular [RxNorm](https://www.nlm.nih.gov/research/umls/rxnorm/index.html) library from the National Library of Medicine ([NLMS](https://www.nlm.nih.gov))
  - This also contains web assembly bindings for the public functions
- A website utilizing the core Web Assembly bindings

We use [wasm bindgen](https://github.com/rustwasm/wasm-bindgen) and [wasm-pack](https://github.com/rustwasm/wasm-pack) to generate web assembly from the core lib.

use [this](https://doc.rust-lang.org/std/io/struct.LineWriter.html) to write lines of json

we assume user's know if their metric return edit distance, similarity scores as well as the difference between true
mathematical "metrics" (which obey triangle inequality) and not.

The default search is Levenshtein which is a mathematical metric that returns a # of edits before being translated to simialrity score
