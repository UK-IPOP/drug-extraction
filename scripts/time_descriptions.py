from pathlib import Path
import pyspark.pandas as ps
from rich import pretty, print

pretty.install()

# load data, set language field using file name
print("[yellow]***[blue]Running time analysis...[yellow]***")
paths = [str(p) for p in Path("data/output").glob("*.jsonl")]
df = ps.DataFrame()
for path in paths:
    temp = ps.read_json(path)
    temp["language"] = path.split("/")[-1].split("-")[0]
    df = ps.concat([df, temp], ignore_index=True)

print("[blue]Average time per string comparison:")
averages = df.groupby(["language", "metric", "level"])["time"].mean().round(5)
print(averages)

print("[blue]Total time for each language:")
totals = df.groupby(["language", "metric", "level"])["time"].sum()
print(totals)
print("[yellow]***[blue]----------[yellow]***")
