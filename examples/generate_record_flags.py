#! /usr/bin/env python3

# **IMPORTANT**
# This script assumes the output of the search command that you run
# (via subprocess command or another way) fits into memory. It will
# need to be modified to work with a large dataset.
# -------------
# This script is somewhat created for our use case but should be general
# enough to be adopted. It is required because I couldn't find an alternate
# way (for example, using joins) to get the same output (binary flags
# for terms and meta-categories at the row level). If you have an idea how to
# improve the script or how to get the same results using simpler methods
# (i.e. a join) please
# [Submit an Issue](https://github.com/UK-IPOP/drug-extraction/issues) 🙂.
# We use that script frequently so any help is welcomed.
#
# This script also showcases how to use python subprocess module to run
# the main CLI, although that code is commented out to not overwrite the
# existing `data/output.csv` file.


import csv
from collections import defaultdict
from dataclasses import dataclass
from pathlib import Path
import subprocess


# this matches the output of the search command
@dataclass
class SearchOutput:
    row_id: str
    search_term: str
    matched_term: str
    edits: int
    similarity_score: float
    search_field: str
    metadata: str | None


# terminal command
command_list = [
    "extract-drugs",
    "search",
    "-t=data/search_terms.csv",
    "-d=data/cook_records.csv",
    "-c=Primary Cause",
    "-c=Secondary Cause",
    "-c=Primary Cause Line A",
    "-c=Primary Cause Line B",
    "-c=Primary Cause Line C",
]

# run the command
# * Uncomment the following code block to run the subprocess command *
# print(f"Running command: {' '.join(command_list)}")
# return_code = subprocess.call(command_list)
# if return_code != 0:
#     raise RuntimeError("Search command failed")


records: dict[str, defaultdict[str, int]] = {}

# to switch this from binary flags (1/0) to counts, change all
# " = 1" to " += 1" :)
print("Reading output...")
# read output
with open(Path().cwd() / "data" / "output.csv", "r") as f:
    reader = csv.DictReader(f)
    for row in reader:
        record = SearchOutput(**row)  # type: ignore
        # binary flag for each search term
        if record.row_id not in records:
            records[record.row_id] = defaultdict(int)
        records[record.row_id][record.search_term] = 1
        # binary flag for search field
        # need to rename so doesn't overwrite on joining to source data
        records[record.row_id][f"{record.search_field.replace(' ', '_')}_matched"] = 1
        # metadata binary flags, assumes metadata is pipe delimited
        # uses "group" to avoid potential column name conflicts
        if record.metadata:
            for meta in record.metadata.split("|"):
                records[record.row_id][f"{meta.upper()}_meta"] = 1


print("Writing wide form...")
# write wide form
with open(Path().cwd() / "data" / "wide_form_output.csv", "w") as f:
    fields: list[str] = []
    fields.append("row_id")
    additional = set(k for r in records.values() for k in r.keys())
    fields.extend(additional)
    writer = csv.DictWriter(f, fieldnames=fields)
    writer.writeheader()
    for row_id, record in records.items():
        record["row_id"] = row_id  # type: ignore
        writer.writerow(record)

print("Sample records:")
subprocess.call(["tail", "-n", "2", "data/wide_form_output.csv"])

print("Saved wide form.")
