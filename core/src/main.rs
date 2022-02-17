extern crate core;

use serde::Deserialize;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;

use itertools::Itertools;
use strsim::{damerau_levenshtein, levenshtein};
mod utils;
mod external;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // this entire function mimics how a user would consume/utilize the library
    // most of this is pseudocode for CLI and web apps

    let records = external::read_text_records_from_file();
    // let drugs = external::get_drugs_from_file();

    let drugs = vec!["cocaine".to_string().to_uppercase(), "tramadol".to_string().to_uppercase()];

    for record in &records {
        for drug_name in &drugs {
            // TODO::CORE perform search on drug and record
            // should have exact match param
            // should take either drug OR String as search param
            // ie search_record(text: record, search_term: drug, exact:False)
            // AND / OR
            // ie search_record(text: record, search_term: "narcan", exact:False)
            // should work
            let results = utils::custom_search(record, drug_name, false, Some(2.0));
            match results.len() {
                x if x > 0 => {
                    println!("{:?}", results[0]);
                },
                _ => continue
            }
        }
    }

    // let classID = "D000701";
    // let relaSource = "MESH";
    // let drugs = get_drug_class_members(&classID, &relaSource).await?;
    // println!("{:?}", drugs[0]);
    // println!("{:?}", drugs[0].aliases());
    // println!("{:?}", drugs[0].ngrams());
    //
    // for drug in &drugs {
    //     if drug.name.contains("/") {
    //         println!("{:?}", drug);
    //         println!("{:?}", drug.aliases());
    //         println!("{:?}", drug.ngrams());
    //         break;
    //     }
    // }
    //
    // let records = read_text_records_from_file();
    // println!("{:?}", &records[0..10]);

    Ok(())
}
