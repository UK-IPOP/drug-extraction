from __future__ import annotations

import json
import time
from typing import Generator, Any

from strsimpy.normalized_levenshtein import NormalizedLevenshtein
from strsimpy.jaro_winkler import JaroWinkler
from rich.progress import track
from rich import pretty, print

pretty.install()


def load_data() -> list[str]:
    with open("../../data/records.jsonl", "r") as f:
        json_list = list(f)
        return json_list


def get_user_input() -> str:
    print("Which metric would you like to run?")
    choice = input("JaroWinkler or Levenshtein? (J/L): ")
    if choice.upper() not in {"J", "L"}:
        raise ValueError("expected J or L")
    return choice


def join_cols(record: dict[str, Any]) -> str:
    """Joins various column values."""
    return f"{record.get('primarycause', '')}\
        {record.get('primarycause_linea', '')}\
            {record.get('primarycause_lineb', '')}\
                {record.get('primarycause_linec', '')}"


def search_record(
    text: str, level: str, searcher: NormalizedLevenshtein | JaroWinkler
) -> Generator[dict[str, str | float], None, None]:
    """Searches a single text record"""
    y = text.translate(str.maketrans("", "", "!@#$%^&*()'\";:,.?{}[]")).lower()
    for word in y.split():  # default splits on space
        start_time = time.time()
        d = searcher.distance(s0=word, s1="heroin")
        time_elapsed = time.time() - start_time
        yield {
            "word": word,
            "distance": d
            if type(searcher) == JaroWinkler
            else d / max(len(word), len("heroin")),  # normalizes
            "col": level,
            "metric": "JaroWinkler"
            if type(searcher) == JaroWinkler
            else "NormalizedLevenshtein",
            "time": time_elapsed,
        }


def runner(search_metric: str, data: list[str]):
    if search_metric.upper() == "J":
        metric = JaroWinkler()
        fpath_ending = "python-jarowinkler"
    else:
        metric = NormalizedLevenshtein()
        fpath_ending = "python-levenshtein"

    with open(f"../../data/third-party/{fpath_ending}.jsonl", "w") as out_file:
        for json_str in track(data, description="Comparing strings..."):
            data = json.loads(json_str)
            data["primary_combined"] = join_cols(data)
            # run the searcher for each col
            for col in ("primary_combined", "secondarycause"):
                if row_text := data.get(col):
                    search_results = search_record(
                        text=row_text, level=col, searcher=metric
                    )
                    if search_results:
                        json_data = (
                            json.dumps(
                                {
                                    "casenumber": data["casenumber"],
                                    "results": list(search_results),
                                }
                            )
                            + "\n"
                        )
                        out_file.write(json_data)
