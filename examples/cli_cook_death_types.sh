#! /usr/bin/env bash


# extract non-drug terms (suicide, homicide, etc.) from Cook County data
# using fuzzy-matching with no metadata
# across multiple columns

extract-drugs search \
    -t data/death_terms.csv \
    -d data/cook_records.csv  \
    -c "Primary Cause" \
    -c "Secondary Cause" \
    -c "Manner of Death"
