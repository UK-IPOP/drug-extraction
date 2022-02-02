// this is a package for extracting drugs
// using RxNorm classes as input
// and searching using string similarity measures
// to capture misspellings

// Mental notes:
// use only stdlib if possible
// handle expected errors gracefully

use reqwest::Error;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct DrugClass {
    class_id: String,
    class_name: String,
    class_type: String,
}

#[derive(Deserialize, Debug)]
struct DrugClassList {
    #[serde(rename(deserialize = "rxclassMinConcept"))]
    class_list: Vec<DrugClass>,
}

#[derive(Deserialize, Debug)]
struct DrugClasses {
    #[serde(rename(deserialize = "rxclassMinConceptList"))]
    concepts: DrugClassList,
}

// we will need:
const BASE_URL: &str = "https://rxnav.nlm.nih.gov/REST/rxclass";

pub async fn get_drug_classes() -> Result<(), Error> {
    let request_url = format!("{}/allClasses.json", BASE_URL);
    let classes = reqwest::get(request_url)
        .await?
        .json::<DrugClasses>()
        .await?;

    println!("{:?}", classes.concepts.class_list[0]);
    Ok(())
}
// an http function to get the RxNorm classes

// an http function to get the drugs from an RxNorm class

// a parsing function to combine the drugs into a set of names (unique) linked to IDs

// the damerau-levenshtein distance function
// this should return the distance between two strings
// AND the similarity score between two strings (i.e. 4, 0.60)

// a function to run the damerau-levenshtein distance on a set of drug names

// a function to write the results to a file

// a struct that represents a drug name + id(s) pairing
// a struct that represents drug classes
// a struct that represents results of string search

// utility functions
// most functions should be public for either WASM use or CLI use

// ignore this is just a template for ensuring WASM works
pub fn saiyan() -> String {
    return "Goku".to_string();
}
