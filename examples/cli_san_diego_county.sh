#! /usr/bin/env bash


# extract with id column using long options
# with no metadata attached to terms and terms containing 
# many n-grams with one search column

# data from: https://data.sandiegocounty.gov/Safety/Medical-Examiner-Cases/jkvb-n4p7

extract-drugs search \
    --terms-file data/search_terms.csv \
    --data-file data/san_diego_records.csv  \
    --search-cols "Cause of Death" \
    --id-col "Row Number"
