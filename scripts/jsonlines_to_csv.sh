#! /usr/bin/env bash

cat pract/output.jsonl \
    | jq '. | {record_id, word_found}' \
    | jq --slurp '.' \
    | jq -r '(map(keys) | add | unique) as $cols
        | map(. as $row | $cols | map($row[.])) as $rows
        | $cols, $rows[] | @csv' > pract/output_cli.csv