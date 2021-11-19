from sodapy import Socrata
import json

client = Socrata("datacatalog.cookcountyil.gov", None)
results = client.get("cjeq-bs86", limit=100_000)  # id for case archives dataset

# jsonlines 😃
with open("perf-comp/data/records.jsonl", "w") as f:
    for result in results:
        json.dump(result, f)
        f.write("\n")
