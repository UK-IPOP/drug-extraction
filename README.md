# Analysis

This branch exists for performance analysis comparing Python, Golang, and Rust performance on string similarity metrics using Chicago Medical Examiners Case Archives as a sample dataset.

Each folder (`python`, `go`, `rust`) contains the same program rewritten in each language.

The program will prompt the user for input as to which algorithm to use (JaroWinkler or Levenshtein) and
then will stream the sample dataset. For each record it will read the (created) `primary_combined` and `secondarycause` fields
and perform some string preprocessing. Then, for each word (as determined by ascii-space) in the record it will perform string similarity comparisons for _each_ drug name saved in the `data/input/drugs.jsonl` file. Those drugs were extracted from the RxClass API using the `scripts/fetch-drug-info.py` script.

Results of each program run are logged to `data/results` folder with a log file for each language.

> The time recorded is for _each_ algorithm/comparison run. (i.e. each drug compared to each word in each record)

Results are calculated as followed:

- Summed time for all algorithm runs
- Total number of algorithm runs
- Average time for each algorithm run
