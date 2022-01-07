import pandas as pd
from pathlib import Path
import json
from rich import pretty, print

pretty.install()

languages = ["rust", "go", "python"]

# load data, set language field using file name
for lang in languages:
    print(f"***Running time analysis for {lang}***")
    data = []
    for p in Path("data").iterdir():
        if p.name.__contains__(lang) and p.name.__contains__("jsonl"):
            with open(p, "r") as file:
                for _ in range(10_000):
                    line = next(file)
                    line_data = json.loads(line)
                    line_data["language"] = lang
                    data.append(line_data)

    print("Sample data:")
    print(data[0])

    df = pd.DataFrame(data=data)
    df.distance = df.distance.astype(float)
    df.time = df.time.astype(float)
    print(f"Table has {df.shape[0]} rows and {df.shape[1]} columns.")

    print(f"{lang.capitalize()} took a total of {df.time.sum().round(5)} seconds.")

    groups = df.groupby(["metric", "level"])["time"].mean().sort_values(ascending=True)
    print(groups)
    print("----------")
