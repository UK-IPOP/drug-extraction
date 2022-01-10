import json
import requests
from rich import print, pretty

pretty.install()

request_data = [
    {
        "classId": "D000700",
        "relaSource": "MESH",
        "category_name": "analgesics",
    },
    {
        "classId": "D000777",
        "relaSource": "MESH",
        "category_name": "stimulants",
    },
]

url = "https://rxnav.nlm.nih.gov/REST/rxclass/classMembers.json"


def remove_old_data():
    """Remove old data from output file."""
    with open("data/input/drugs.jsonl", "w") as f:
        f.truncate()


remove_old_data()

for payload in request_data:
    response = requests.get(url, params=payload)
    print(f"{payload['category_name']} --- {response.status_code}")

    data = response.json()

    scraped_data = []
    for drug_member in data["drugMemberGroup"]["drugMember"]:
        rx_id = drug_member["minConcept"]["rxcui"]
        name = drug_member["minConcept"]["name"]
        scraped_data.append(
            {
                "rx_id": rx_id,
                "name": name,
                "category_name": payload["category_name"],
            }
        )

    with open("data/input/drugs.jsonl", "a") as f:
        for drug in scraped_data:
            json_line = json.dumps(drug) + "\n"
            f.write(json_line)
