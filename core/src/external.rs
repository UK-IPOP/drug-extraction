use std::fs::File;
use std::io::BufReader;
use crate::utils;

pub fn get_drugs_from_file() -> Vec<utils::Drug> {
    // sample taken from: https://rxnav.nlm.nih.gov/REST/rxclass/classMembers.json?classId=D000701&relaSource=MESH
    let file = File::open("data/sample_opioids.json").unwrap();
    let reader = BufReader::new(file);
    let data: utils::DrugResponse = serde_json::from_reader(reader).unwrap();
    let drug_info: Vec<utils::DrugInfo> = data.drug_member_group
        .iter()
        .map(|d| d.min_concept.clone())
        .collect();
    let drug_list = utils::parse_drugs(drug_info);
    drug_list
}

// This functionality needs a lot of work
// TODO: support parsing into json
// TODO: support specifying an ID column and a text column
// TODO: after ^^ modify
pub fn read_text_records_from_file() -> Vec<String> {
    // returns only the text of the records
    let file = File::open("data/records.csv").unwrap();
    let mut rdr = csv::Reader::from_reader(file);
    let mut records = Vec::<String>::new();
    for result in rdr.records() {
        let record = result.unwrap();
        let text = record[8].to_string().to_uppercase().trim().to_string();
        records.push(remove_symbols(text));
    }
    records
}

fn remove_symbols(x: String) -> String {
    x.replace(&['(', ')', ',', '\"', '.', ';', ':', '!','*'][..], "")
}