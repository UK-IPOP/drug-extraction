use reqwest::Error;
use serde::Deserialize;
use std::collections::HashMap;
use std::collections::HashSet;

mod utils;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DrugClass {
    class_id: String,
    class_name: String,
    class_type: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Drug {
    #[serde(rename(deserialize = "rxcui"))]
    rx_id: String,
    name: String,
}

// we will need:
const BASE_URL: &str = "https://rxnav.nlm.nih.gov/REST/rxclass";

pub async fn get_drug_classes() -> Result<Vec<utils::RxclassMinConcept>, Error> {
    let request_url = format!("{}/allClasses.json", BASE_URL);
    let response_data = reqwest::get(request_url)
        .await?
        .json::<utils::ClassListResponse>()
        .await?;

    println!("{:?}", response_data);
    let data = response_data.rxclass_min_concept_list.rxclass_min_concept;
    let mut d = HashSet::new();
    for da in data.iter() {
        d.insert(da.class_id.clone());
    }
    println!("{:?}", d);
    Ok(data)
}

pub async fn get_class_members(class_id: &String, class_type: &String) -> Result<Vec<Drug>, Error> {
    // TODO: have to implement more for VA and RXNORM classes
    // they require ttys param
    // TODO: must parse class_type to one of accepted values

    let request_url = format!(
        "{}/classMembers.json?classId={}&relaSource={}",
        BASE_URL, class_id, class_type
    );
    println!("{}", request_url);
    let response_data = reqwest::get(request_url)
        .await?
        .json::<utils::ClassMembersResponse>()
        .await?;

    println!("{:?}", response_data);
    let drug_members = response_data.drug_member_group.drug_member;
    let mut drugs_list: Vec<Drug> = Vec::new();
    for drug in drug_members {
        drugs_list.push(Drug {
            rx_id: drug.min_concept.rxcui,
            name: drug.min_concept.name,
        });
    }
    Ok(drugs_list)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let drug_classes = get_drug_classes().await?;
    println!("{:?}", drug_classes);
    let members = get_class_members(&drug_classes[0].class_id, &drug_classes[0].class_type).await?;
    println!("{:?}", members);
    Ok(())
}
