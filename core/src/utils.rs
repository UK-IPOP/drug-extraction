use std::cmp::min;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::BufReader;
use std::ops::Deref;
use serde::Deserialize;
use serde::Serialize;
use strsim::damerau_levenshtein;
use itertools::Itertools;
use drug_extraction::saiyan;


// TODO: remove pub where unneeded
// TODO: move most code to core lib
// TODO: move some code to models module

const BASE_URL: &str = "https://rxnav.nlm.nih.gov/REST/rxclass";

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrugResponse {
    pub drug_member_group: Vec<DrugMember>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrugMember {
    pub min_concept: DrugInfo,
    pub node_attr: Vec<NodeAttr>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrugInfo {
    pub rxcui: String,
    pub name: String,
    pub tty: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeAttr {
    pub attr_name: String,
    pub attr_value: String,
}

#[derive(Default, Debug)]
pub struct CustomSearchResult {
    pub text: String,
    pub search_term: String,
    pub exact_match: Option<bool>,
    pub edit_distance: Option<f64>,
    pub similarity: Option<f64>,
}

pub trait SearchResult {}

impl SearchResult for CustomSearchResult;

pub trait Searcher {
    fn new(a: &str, b: &str, exact: bool, edits: f64) -> Self;
    fn search(&self) -> Box<dyn SearchResult>;
}

pub struct CustomSearcher {
    pub search_text: String,
    pub compare_text: String,
    pub exact_match: bool,
}

impl Searcher for CustomSearcher {
    fn new(a: &str, b: &str, exact: bool, edits: f64) -> CustomSearcher {
        CustomSearcher{
            search_text: a.to_string(),
            compare_text: b.to_string(),
            exact_match: exact,
        }
    }
    fn search(&self) -> Vec<CustomSearchResult> {
        let mut record_results = Vec::<CustomSearchResult>::new();


    }
}

pub struct StandardSearchResult {
    search_text: String,
    matched_text: String,
    exact_match: bool,
    edit_distance: Option<f64>,
    similarity: Option<f64>,
}


pub struct StandardSearcher {
    exact: bool,
    edits: f64,
    threshold: f64,
}

impl StandardSearcher {
    pub fn search(&self, a: &str, b: &str) -> StandardSearchResult {
        match self.exact {
            // this arm runs simple exact matching
            true => run_exact_search(a, b),
            // this arm is responsible for running leven algorithm
            false => run_leven_search(a, b)
        }
    }
}



pub struct DrugSearchResult {
    search_text: String,
    matched_text: String,
    exact_match: bool,
    edit_distance: Option<f64>,
    similarity: Option<f64>,
}


pub struct DrugSearcher {
    exact: bool,
    edits: f64,
    threshold: f64,
}

impl DrugSearcherSearcher {

    pub fn search(&self, a: &str, b: &str) -> DrugSearchResult {
        match self.exact {
            // this arm runs simple exact matching
            true => run_exact_search(a, b),
            // this arm is responsible for running leven algorithm
            false => run_leven_search(a, b)
        }
    }
}


fn run_exact_search(a: &str, b: &str) -> StandardSearchResult {
    if a.upper() == b.upper() {
        StandardSearchResult{
            search_text: a.to_string(),
            matched_text: b.to_string(),
            exact_match: true,
            edit_distance: None,
            similarity: None
        }
    } else {
        StandardSearchResult{
            search_text: a.to_string(),
            matched_text: b.to_string(),
            exact_match: false,
            edit_distance: None,
            similarity: None
        }
    }
}

fn run_leven_search(a: &str, b: &str) -> StandardSearchResult {
    let d = damerau_levenshtein(a, b) as f64;
    let sim = 1.0 - (d) / (a.chars().count().max(b.chars().count()) as f64);
    StandardSearchResult{
        search_text: a.to_string(),
        matched_text: b.to_string(),
        exact_match: if d == 0 {true} else {false},
        edit_distance: Some(d),
        similarity: Some(sim)
    }
}

// can search the whole record
// may yield many responses for one drug
pub fn custom_search(text: &String, term: &String, exact: bool, edits: Option<f64>) -> Vec<CustomSearchResult> {
    let target = term.to_owned();

    let ngrams = target.split(" ").count();
    for phrase in text.split_whitespace().combinations(ngrams) {
        // rejoins ngrams together
        let word = phrase.join(" ");
        if exact {
            // no else because we don't care when there is no match
            if word == target {
                let search_result = CustomSearchResult {
                    text: word,
                    search_term: target.to_string(),
                    exact_match: Some(true),
                    edit_distance: None,
                    similarity: None
                };
                record_results.push(search_result);
            }
        } else  { // not exact only thus run leven algorithm
            let d = damerau_levenshtein(word.as_str(), target.as_str()) as f64;
            match edits {
                Some(e) if d <= e => {
                    let sim = 1.0 - (d) / (word.chars().count().max(target.chars().count()) as f64);
                    let search_result = CustomSearchResult {
                        text: word,
                        search_term: target.to_string(),
                        exact_match: None,
                        edit_distance: Some(d),
                        similarity: Some(sim)
                    };
                    record_results.push(search_result);
                }
                _ => continue
            }
        }
    }
    record_results
}





#[derive(Deserialize, Debug, Clone)]
pub struct Drug {
    #[serde(rename(deserialize = "rxcui"))]
    pub rx_id: String,
    pub name: String,
}

impl Drug {
    pub fn new(rx_id: String, name: String) -> Drug {
        Drug { rx_id, name }
    }

    // if ngram search is > 1
    pub fn aliases(&self) -> Vec<String> {
        if self.name.contains("/") {
            // deal with split naming
            self.name
                .trim()
                .split("/")
                .collect::<Vec<&str>>()
                .iter()
                .map(|a| a.to_string().to_uppercase().trim().to_string())
                .collect()
        } else {
            vec![self.name.to_uppercase().trim().to_string()]
        }
    }

    pub fn ngrams(&self) -> usize {
        self.aliases()
            .iter()
            .map(|a| a.split(" ").count())
            .max()
            .unwrap() // error because should always have a value
        // can propagate error later
    }

}


// make private later
pub fn parse_drugs(drugs: Vec<DrugInfo>) -> Vec<Drug> {
    let mut d = Vec::new();
    for drug in drugs {
        d.push(Drug::new(drug.rxcui, drug.name));
    }
    d
}



