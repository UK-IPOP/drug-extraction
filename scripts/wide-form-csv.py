#! /usr/bin/python3

import collections
import subprocess
import csv

# this section is a `simple-search`` example
# command
command_list = [
    "./target/release/extract-drugs",
    "simple-search",
    "./cli/data/records.csv",
    "--algorithm",
    "l",
    "--max-edits",
    "2",
    "--target-column",
    "Primary Cause",
    "--id-column",
    "Case Number",
    "--search-words",
    "heroin|cocaine|alcohol|fentanyl",
    "--format",
    "csv",
]
# run the command
subprocess.call(command_list)

# this flags if drugs were detected, not their counts
# in other words a drug could occur multiple times in the same record and
# this flag will only be 1
records: collections.defaultdict[str, dict[str, int]] = collections.defaultdict(dict)
with open("./extracted_drugs.csv", "r") as f:
    reader = csv.DictReader(f)
    for row in reader:
        # use `search_term` as the key
        records[row["record_id"]][row["search_term"]] = 1

csv_records: list[dict[str, str | int]] = []

for record_id, drugs in records.items():
    csv_row: dict[str, str | int] = dict(Record_Id=record_id)
    for drug in drugs.keys():
        csv_row.update({drug.title(): 1})
    csv_records.append(csv_row)

# write to file
header = set([v for record in csv_records for v in record.keys()])
with open("./extracted_drugs_wide.csv", "w") as f:
    writer = csv.DictWriter(f, fieldnames=header)
    writer.writeheader()
    writer.writerows(csv_records)


# could finish with something like this
# df1 = pd.read_csv("./extracted_drugs.csv")
# df2 = pd.read_csv("./extracted_drugs_wide.csv")
# df = pd.merge(df1, df2, left_on="Case Number", right_on="Record_Id")
# df.head()
# df.to_csv("./extracted_drugs_combined.csv", index=False)
