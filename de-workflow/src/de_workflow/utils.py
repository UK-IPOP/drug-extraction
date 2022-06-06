#! /usr/bin/python3

import collections
import json
import subprocess
import pathlib
from typing import Optional
import pandas as pd
import requests
from rich import print

from . import report


def load_search_data() -> dict[str, list[str]]:
    response = requests.get(
        "https://raw.githubusercontent.com/UK-IPOP/drug-extraction/main/de-workflow/data/drug_info.json"
    )
    if response.status_code == 200:
        return json.loads(response.text)
    else:
        raise ConnectionError(f"Failed to fetch search data from {response.url}")


def command(
    file_name: pathlib.Path,
    id_column: str,
    target_columns: list[str],
    search_data: dict[str, list[str]],
    algorithm: str,
):
    for i, target_column in enumerate(target_columns):
        command_list = [
            "extract-drugs",
            "simple-search",
            file_name,
            "--target-column",
            target_column,
            "--id-column",
            id_column,
            "--algorithm",
            algorithm,
            "--threshold",
            "0.9",
            "--format",
            "csv",
            "--search-words",
            "|".join(search_data.keys()),
        ]
        print(f"[cyan]Running on column: '{target_column}'...")
        # runs the command on each column
        subprocess.call(command_list)

        # after it runs we need to move the file so it doesn't get overwritten
        # by the next command
        df = pd.read_csv("extracted_drugs.csv")
        df["source_column"] = i + 1
        df.to_csv(f"extracted_drugs_{i + 1}.csv", index=False)
        pathlib.Path("extracted_drugs.csv").unlink()


# expects the files to have been created
def combine_outputs(tag_lookup: dict[str, list[str]]):
    # can hardcode as dependency
    paths = [
        p
        for p in pathlib.Path(".").iterdir()
        if p.name.startswith("extracted_drugs") and p.name.endswith(".csv")
    ]

    combined = pd.concat([pd.read_csv(p) for p in paths])
    combined["tags"] = combined.search_term.apply(
        lambda x: ";".join(tag_lookup[x.lower()])
        if len(tag_lookup[x.lower()]) > 1
        else tag_lookup[x.lower()][0]
    )
    combined.drop(columns=["edits"], inplace=True)
    combined.to_csv("./dense_results.csv", index=False)


def make_wide():
    records: collections.defaultdict[str, dict[str, int]] = collections.defaultdict(
        dict
    )
    source = pd.read_csv("./dense_results.csv")
    for row in source.itertuples():
        column_name = f"{row.search_term}_{row.source_column}"
        records[row.record_id][column_name] = row.count
        for tag in row.tags.split(";"):
            records[row.record_id][f"{tag}_{row.source_column}"] = row.count

    csv_records: list[dict[str, str | int]] = []
    for record_id, drugs in records.items():
        csv_row: dict[str, str | int] = dict(record_id=record_id)
        for drug in drugs.keys():
            csv_row.update({drug.lower(): 1})
        csv_records.append(csv_row)

    df = pd.DataFrame(csv_records)
    df = df.reindex(columns=sorted(df.columns))  # type: ignore
    df.to_csv("./extracted_drugs_wide.csv", index=False)


def merge_to_source(source_file: pathlib.Path, id_column: str):
    df_wide = pd.read_csv("./extracted_drugs_wide.csv")
    df_source = pd.read_csv(source_file, low_memory=False)
    df_merged = df_source.merge(df_wide, left_on=id_column, right_on="record_id")
    df_merged.drop(columns=["record_id"], inplace=True)
    df_merged.to_csv("./extracted_drugs_merged.csv", index=False)
    df_merged.to_csv("./merged_results.csv", index=False)


def cleanup():
    for p in pathlib.Path(".").iterdir():
        if p.name.startswith("extracted_drugs") and p.name.endswith(".csv"):
            p.unlink()


# this puts the commands in order so that they can rely on dependent files
def run(
    file_name: pathlib.Path,
    id_column: str,
    target_columns: list[str],
    algorithm: str,
):
    search_data = load_search_data()
    command(
        file_name=file_name,
        id_column=id_column,
        target_columns=target_columns,
        search_data=search_data,
        algorithm=algorithm,
    )
    combine_outputs(tag_lookup=search_data)
    make_wide()
    merge_to_source(source_file=file_name, id_column=id_column)
    print("[cyan]Generating report...")
    report.generate_report()
    cleanup()
