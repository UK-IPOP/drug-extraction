from drug_extraction import models

# de = models.DrugExtractor()
# extracted_data = de.extract("man i love alcohol and cocaine so much")
# print(extracted_data)


d = models.DrugExtractor()
extracted_drugs = d.extract("man i love alcohol and cocaine so much")
for drug, count in extracted_drugs.items():
    print(d.drug_info(drug), "Count: ", count)
