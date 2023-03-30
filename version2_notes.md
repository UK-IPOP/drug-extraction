# Version 2 Goals and Priorities

The goals of this version are:

- To make working with CSV I/O simpler
- To make it possible to search pure text
  - This also opens up scanning PDFs and ANY general text sent to std.out
- metadata now is great because it can be ANYthing and even null for certain values
  - this increases possibilities greatly but means we will not support metadata analysis in the de-workflow
- Simplify algorithm complexity
  - Just use one algorithm
    - This simplifies output, reduces complications for config
- Use maturin to build main CLI as python package
  - Then installable for data scientists via pipx
- TODO: Make de-workflow package operate soley on output of primary CLI
  - This separates concerns and dependencies
  - don't include dataframe due to its insane size
- TODO: Make website an interactive playground for algorithm exploration
  - I.e. use selected algorithm and show users similarities of input words so they can experiment with a good threshold
- Update opendata-pipeline to use new version of CLI (using `standard` subcommand most likely)
- Mention in docs that people can use our search terms by downloading from github

TODO: Improve python metadata in pyproject.toml


## Notices

- Pipe command needs some sort of indicator when various steps are done... it just loads then spits out std-out...
  - IN FACT, this just completely crashed
- We should **validate** the search_terms file format as soon as it is provided in `interactive` mode
- Word indexing is very slow and does not use full CPU...
  - flamegraph to check this?
- use criterion to benchmark str-sim algorithms just for reference on the repo
- n-gram support!
- metadata is not required, but parsing search_terms file breaks when rows have different schema (i.e. diff cols)
  - do we need to parse/validate row by row?
