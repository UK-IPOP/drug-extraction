# use this to parse times out of the logfiles.

from pathlib import Path
import datetime
from rich import pretty, print

pretty.install()

for p in Path("data/results").glob("*.log"):
    with open(p, "r") as f:
        last_line = f.readlines()[-1]
        parts = last_line.split()
        data = dict(
            datetime=datetime.datetime.strptime(
                parts[0] + " " + parts[1], "%Y/%m/%d %H:%M:%S"
            ),
            record_count=int(parts[2]),
            total_time=float(parts[5]),
            algorithm=parts[8],
            average_time=float(parts[-1]),
        )
        print(data)
