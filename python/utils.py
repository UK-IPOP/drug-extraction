from __future__ import annotations

import json
import time
from typing import Generator, Any, TextIO

from strsimpy.normalized_levenshtein import NormalizedLevenshtein
from strsimpy.jaro_winkler import JaroWinkler
from rich import pretty, print

pretty.install()


def load_data() -> TextIO:
    f = open("../data/records.jsonl", "r")
    return f


def get_user_input() -> str:
    print("Which metric would you like to run?")
    choice = input("JaroWinkler or Levenshtein? (J/L): ")
    if choice.upper() not in {"J", "L"}:
        raise ValueError("expected J or L")
    return choice


def join_cols(record: dict[str, Any]) -> str:
    """Joins various column values."""
    cause1 = record.get("primarycause", "")
    cause2 = record.get("primarycause_linea", "")
    cause3 = record.get("primarycause_lineb", "")
    cause4 = record.get("primarycause_linec", "")
    return f"{cause1} {cause2} {cause3} {cause4}".strip()


def load_drugs() -> list[dict[str, str]]:
    """This loads the drugs from file.

    It does not need to stream the file because in production it
    will be loading from the API not from a file.
    """
    data = []
    with open("../data/drugs.jsonl", "r") as file:
        for line in file:
            json_line = json.loads(line)
            data.append(json_line)
    return data


def search_record(
    text: str,
    level: str,
    searcher: NormalizedLevenshtein | JaroWinkler,
    drug_list: list[dict[str, str]],
) -> Generator[dict[str, str | float], None, None]:
    """Searches a single text record"""
    y = text.translate(str.maketrans("", "", "(),;:@#$%^&*_+={}[]|<>/")).upper()
    for drug_info in drug_list:
        drug_names = [d.strip().upper() for d in drug_info["name"].split("/")]
        id_ = drug_info["rx_id"]
        for name in drug_names:
            for word in y.split():  # default splits on space
                start_time = time.time()
                d = searcher.distance(s0=word, s1=name)
                time_elapsed = time.time() - start_time
                distance = (
                    d
                    if type(searcher) == JaroWinkler
                    else 1 - (d / max(len(word), len(name)))
                )  # normalizes
                yield {
                    "word": word,
                    "similarity": distance,
                    "level": level,
                    "metric": "JaroWinkler"
                    if type(searcher) == JaroWinkler
                    else "NormalizedLevenshtein",
                    "time": time_elapsed,
                    "drug_name": name,
                    "drug_id": id_,
                }


def runner(search_metric: str, input_file: TextIO):
    if search_metric.upper() == "J":
        metric = JaroWinkler()
        fpath_ending = "python-jarowinkler"
    elif search_metric.upper() == "L":
        metric = NormalizedLevenshtein()
        fpath_ending = "python-levenshtein"
    else:
        print("Invalid search metric.")
        return

    drugs = load_drugs()
    with open(f"../data/{fpath_ending}.jsonl", "w") as out_file:
        for line in input_file:
            data = json.loads(line)
            data["primary_combined"] = join_cols(data)
            # run the searcher for each col
            for col in ("primary_combined", "secondarycause"):
                if row_text := data.get(col):
                    search_results = search_record(
                        text=row_text, level=col, searcher=metric, drug_list=drugs
                    )
                    if search_results:
                        for result in search_results:
                            json_data = (
                                json.dumps({"casenumber": data["casenumber"], **result})
                                + "\n"
                            )
                            out_file.write(json_data)
