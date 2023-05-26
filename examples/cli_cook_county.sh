#! /usr/bin/env bash


# extract with an id column using short options
# with pipe-delimited metadata attached to terms
# and multiple search columns

# data from: https://datacatalog.cookcountyil.gov/Public-Safety/Medical-Examiner-Case-Archive/cjeq-bs86

extract-drugs search \
    -t data/search_terms.csv \
    -d data/cook_records.csv  \
    -c "Primary Cause" \
    -c "Primary Cause Line A" \
    -c "Primary Cause Line B" \
    -c "Primary Cause Line C" \
    -c "Secondary Cause" \
    -i "Case Number"
