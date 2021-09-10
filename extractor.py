import csv
from ruamel.yaml import YAML

yaml = YAML()

with open("./drug_dictionary copy.csv", "r", encoding="utf-8-sig") as f:
    csvreader = csv.DictReader(f)
    headers = csvreader.fieldnames
    data = {}
    for row in csvreader:
        row_dict = {}
        if headers:
            for header in headers:
                row_dict[header] = row[header]
            data[row["name"]] = row_dict

data2 = {}
for k, v in data.items():
    rowdata = {}
    rowdata["name"] = v["name"]
    rowdata["search_terms"] = list(v["search_terms"].split(";"))
    tags = [
        k2.removesuffix("_related")
        for k2, v2 in v.items()
        if v2 == "TRUE" and k2 != "drug_related"
    ]
    rowdata["tags"] = list(tags)
    data2[k] = rowdata

print(data2)
with open("drug_classes.yaml", "w") as f:
    yaml.dump(data2, f)
