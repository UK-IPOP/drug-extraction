import json
import requests
from rich import print, pretty

pretty.install()

payload = {
    "classId": "N02A",
    "relaSource": "ATC",
}
url = "https://rxnav.nlm.nih.gov/REST/rxclass/classMembers.json"

response = requests.get(url, params=payload)
print(response.status_code)

data = response.json()
print(data)

scraped_data = []
for drug_member in data["drugMemberGroup"]["drugMember"]:
    rx_id = drug_member["minConcept"]["rxcui"]
    name = drug_member["minConcept"]["name"]
    scraped_data.append(
        {
            "rx_id": rx_id,
            "name": name,
        }
    )

with open("data/input/drugs.jsonl", "w") as f:
    for drug in scraped_data:
        json_line = json.dumps(drug) + "\n"
        f.write(json_line)
