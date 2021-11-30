from __future__ import annotations

from collections import defaultdict
import json
from typing import Generator
from strsimpy.jaro_winkler import JaroWinkler

from rich.progress import track
import time


def join_cols(record) -> str:
    """Joins various column values."""
    return f"{record.get('primarycause', '')}\
        {record.get('primarycause_linea', '')}\
            {record.get('primarycause_lineb', '')}\
                {record.get('primarycause_linec', '')}"


def search_record(
    text: str, level: str, searcher: JaroWinkler
) -> Generator[dict[str, str | float], None, None]:
    y = text.translate(str.maketrans("", "", "!@#$%^&*()'\";:,.?{}[]"))
    for word in y.split():
        s_time = time.time()
        yield {
            "word": word,
            "distance": searcher.similarity(s0=word, s1="heroin"),
            "level": level,
            "metric": "Jaro-Winkler"
            if type(searcher) == JaroWinkler
            else "Levenshtein",
            "time": time.time() - s_time,
        }


with open("../../data/records.jsonl", "r") as f:
    json_list = list(f)


with open("../../data/third-party/python-jaro.jsonl", "w") as f:
    for json_str in track(json_list, description="Comparing strings: "):
        result = json.loads(json_str)
        result["primary_combined"] = join_cols(result)
        # process result
        jaro_winkler = JaroWinkler()
        for level in ("primary_combined", "secondarycause"):
            # doesn't have data in that field
            if not result.get(level):
                continue

            results = list(
                search_record(text=result[level], level=level, searcher=jaro_winkler)
            )
            if results:
                f.write(
                    json.dumps(
                        {
                            "casenumber": result["casenumber"],
                            "results": results,
                        }
                    )
                    + "\n"
                )
