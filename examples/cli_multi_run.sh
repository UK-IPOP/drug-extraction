#! /usr/bin/env bash


# extract with no id column using short options
# with simple search terms

# do this on two datasets and concat them using python csv readers
# you could combine using something like `sed`, but we want to add a 
# new column telling us the source of the data
# you could also easily do this in pandas

# stop on error 
set -e

# first dataset
extract-drugs search \
    -t data/simple_search_terms.csv \
    -d data/cook_records.csv  \
    -c "Primary Cause" \
    -c "Secondary Cause"

# copy first dataset to new location
mv output.csv cook_output.csv

# second dataset
extract-drugs search \
    -t data/simple_search_terms.csv \
    -d data/san_diego_records.csv  \
    -c "Cause of Death"

# copy second dataset to new location
mv output.csv san_diego_output.csv

# combine the two datasets using python stdlib
python3 -c """
import csv
from pathlib import Path

# make a writer
with open(Path().cwd() / 'combined_output.csv', 'w') as outfile:
    fieldnames = [
        'row_id',
        'search_term',
        'matched_term',
        'edits',
        'similarity_score',
        'search_field',
        'metadata',
        'data_source',
    ]
    writer = csv.DictWriter(outfile, fieldnames=fieldnames)
    writer.writeheader()

    # read first dataset and write to new file
    with open(Path().cwd() / 'cook_output.csv', 'r') as infile1:
        reader = csv.DictReader(infile1)
        for row in reader:
            row['data_source'] = 'cook_county'
            writer.writerow(row)

    # read second dataset and write to new file
    with open(Path().cwd() / 'san_diego_output.csv', 'r') as infile1:
        reader = csv.DictReader(infile1)
        for row in reader:
            row['data_source'] = 'san_diego_county'
            writer.writerow(row)
"""

# remove intermediate files
rm cook_output.csv
rm san_diego_output.csv

# print first few lines
head combined_output.csv