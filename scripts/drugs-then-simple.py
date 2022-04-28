#! /usr/bin/python3

import collections
import subprocess
import csv
import shutil
import os

# this section is a `drug-search`` example
# command
command_list1 = [
    "extract-drugs",
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
    "--rx-class-id",
    "N02A",
    "--rx-class-relasource",
    "ATC",
    "--format",
    "csv",
]

# run the command1
subprocess.call(command_list1)

shutil.copy("./extracted_drugs.csv", "./extracted_drugs_1.csv")

# command
command_list2 = [
    "extract-drugs",
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
    "covid-19|coronavirus",
    "--format",
    "csv",
]
# run the command1
subprocess.call(command_list2)

shutil.copy("./extracted_drugs.csv", "./extracted_drugs_2.csv")


csv_records: list[dict[str, str]] = []
with open("./extracted_drugs_1.csv", "r") as f:
    reader = csv.DictReader(f)
    for row in reader:
        csv_records.append(row)

with open("./extracted_drugs_2.csv", "r") as f:
    reader = csv.DictReader(f)
    for row in reader:
        csv_records.append(row)


# write to file
with open("./extracted_drugs_combined.csv", "w") as f:
    writer = csv.DictWriter(
        f,
        fieldnames=[
            "record_id",
            "algorithm",
            "edits",
            "similarity",
            "search_term",
            "matched_term",
            "column_name",
        ],
    )
    writer.writeheader()
    writer.writerows(csv_records)

os.remove("./extracted_drugs_1.csv")
os.remove("./extracted_drugs_2.csv")
