mod models;

use csv::StringRecord;
use csv::WriterBuilder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error;
use std::fmt;
use std::fmt::Display;
use std::hash::Hash;
use std::iter::{Filter, FlatMap};
use std::str::{FromStr, SplitWhitespace};

use strsim::{damerau_levenshtein, jaro_winkler, levenshtein, osa_distance, sorensen_dice};

/// Will need to be modified/extended to account for drug tags
/// This will be serialized directly into json so this should
/// be our final data structure that we want in the output

/// ValueError occurs when an invalid value was provided
#[derive(Debug)]
pub struct ValueError;

impl Display for ValueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Received an unexpected value")
    }
}

impl error::Error for ValueError {}

type Result<T> = std::result::Result<T, ValueError>;

// TODO: these functions could probably be better implemented using a closure or something
// since they currently take 2 function calls to execute
fn my_damerau(a: &str, b: &str) -> f64 {
    damerau_levenshtein(a, b) as f64
}

fn my_leven(a: &str, b: &str) -> f64 {
    levenshtein(a, b) as f64
}

fn my_osa(a: &str, b: &str) -> f64 {
    osa_distance(a, b) as f64
}

pub fn initialize_distance(a: Algorithm) -> fn(&str, &str) -> f64 {
    match a {
        Algorithm::DAMERAU => my_damerau,
        Algorithm::LEVENSHTEIN => my_leven,
        Algorithm::JAROWINKLER => jaro_winkler,
        Algorithm::OSA => my_osa,
        Algorithm::SORENSENDICE => sorensen_dice,
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Algorithm {
    DAMERAU,
    LEVENSHTEIN,
    JAROWINKLER,
    OSA,
    SORENSENDICE,
}

impl Algorithm {
    pub fn is_edits(&self) -> bool {
        match self {
            Algorithm::OSA | Algorithm::DAMERAU | Algorithm::LEVENSHTEIN => true,
            Algorithm::JAROWINKLER | Algorithm::SORENSENDICE => false,
        }
    }

    pub fn options() -> Vec<String> {
        vec![
            "Levenshtein".to_string(),
            "Damerau".to_string(),
            "OSA".to_string(),
            "JaroWinkler".to_string(),
            "SorensenDice".to_string(),
        ]
    }
}

impl FromStr for Algorithm {
    type Err = ValueError;
    /// Parses an Algorithm type from a string reference.
    fn from_str(s: &str) -> Result<Algorithm> {
        match s.to_uppercase().chars().next().unwrap() {
            'L' => Ok(Algorithm::LEVENSHTEIN),
            'D' => Ok(Algorithm::DAMERAU),
            'O' => Ok(Algorithm::OSA),
            'J' => Ok(Algorithm::JAROWINKLER),
            'S' => Ok(Algorithm::SORENSENDICE),
            _ => Err(ValueError),
        }
    }
}

impl ToString for Algorithm {
    fn to_string(&self) -> String {
        match self {
            Algorithm::DAMERAU => String::from("Damerau"),
            Algorithm::LEVENSHTEIN => String::from("Levenshtein"),
            Algorithm::OSA => String::from("OSA"),
            Algorithm::JAROWINKLER => String::from("JaroWinkler"),
            Algorithm::SORENSENDICE => String::from("SorensenDice"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SimpleOutput {
    pub record_id: Option<String>,
    pub algorithm: Algorithm,
    pub edits: Option<i32>,
    pub similarity: f64,
    pub search_term: String,
    pub matched_term: String,
}

#[derive(Clone, Copy, Debug)]
pub enum OutputFormat {
    JSONL,
    CSV,
}

impl FromStr for OutputFormat {
    type Err = ValueError;
    /// Parses an Algorithm type from a string reference.
    fn from_str(s: &str) -> Result<OutputFormat> {
        match s.to_uppercase().as_str() {
            "JSONL" => Ok(OutputFormat::JSONL),
            "CSV" => Ok(OutputFormat::CSV),
            _ => Err(ValueError),
        }
    }
}

pub fn format(data: Vec<SearchOutput>, format: OutputFormat) -> Vec<String> {
    match format {
        OutputFormat::JSONL => data
            .iter()
            .map(|x| serde_json::to_string(x).unwrap())
            .collect::<Vec<String>>(),
        OutputFormat::CSV => {
            let mut wtr = WriterBuilder::new().has_headers(false).from_writer(vec![]);
            for row in data {
                wtr.serialize(row).unwrap();
            }
            let csv_data = String::from_utf8(wtr.into_inner().unwrap()).unwrap();
            csv_data
                .split('\n')
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
        }
    }
}

// can duplicate for Drugs later just switching type of targets
// need to validate (in `::new()`) that everything is valid/aligns
// i.e. max edits not threshold only applies for `has_edits()` algos
pub struct SimpleInput {
    pub algorithm: Algorithm,
    pub distance: fn(&str, &str) -> f64,
    pub max_edits: Option<i32>,
    pub similarity_threshold: Option<f64>,
    pub targets: Vec<String>,
}

impl SimpleInput {
    pub fn new(
        algorithm: Algorithm,
        distance: fn(&str, &str) -> f64,
        max_edits: Option<i32>,
        similarity_threshold: Option<f64>,
        targets: &[String],
    ) -> SimpleInput {
        SimpleInput {
            algorithm,
            distance,
            max_edits,
            similarity_threshold,
            targets: targets.to_vec(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchOutput {
    SimpleOutput(SimpleOutput),
    DrugOutput(DrugOutput),
}
pub trait SearchInput {
    fn scan(&self, text: &str, record: Option<String>) -> Vec<SearchOutput>;
}

impl SearchInput for SimpleInput {
    fn scan(&self, text: &str, record: Option<String>) -> Vec<SearchOutput> {
        let clean = text
            .replace(&['(', ')', ',', '\"', '.', ';', ':'][..], "")
            .to_uppercase();
        let words = clean.split_whitespace();
        let mut results: Vec<SimpleOutput> = Vec::new();
        for word in words {
            for target in &self.targets {
                let d = (self.distance)(target, word);
                let res = SimpleOutput {
                    record_id: record.clone(),
                    search_term: target.to_string(),
                    matched_term: word.to_string(),
                    algorithm: self.algorithm,
                    edits: if self.algorithm.is_edits() {
                        Some(d as i32)
                    } else {
                        None
                    },
                    similarity: if self.algorithm.is_edits() {
                        1.0 - (d / (target.chars().count().max(word.chars().count()) as f64))
                    } else {
                        d
                    },
                };
                results.push(res);
            }
        }
        if self.max_edits.is_some() {
            // filter by edits
            let edits = self.max_edits.unwrap();
            results
                .into_iter()
                .filter(|x| x.edits.unwrap() <= edits)
                .map(SearchOutput::SimpleOutput)
                .collect::<Vec<SearchOutput>>()
        } else if self.similarity_threshold.is_some() {
            // filter by similarity
            let thresh = self.similarity_threshold.unwrap();
            results
                .into_iter()
                .filter(|x| x.similarity >= thresh)
                .map(SearchOutput::SimpleOutput)
                .collect::<Vec<SearchOutput>>()
        } else {
            // return all
            results
                .into_iter()
                .map(SearchOutput::SimpleOutput)
                .collect()
        }
    }
}

// /////////////////////////////////////////////////////
//

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Drug {
    pub name: String,
    pub rx_id: String,
    pub group_name: String,
    pub class_id: String,
}

pub struct DrugInput {
    pub algorithm: Algorithm,
    pub distance: fn(&str, &str) -> f64,
    pub max_edits: Option<i32>,
    pub similarity_threshold: Option<f64>,
    pub targets: Vec<Drug>,
}

impl DrugInput {
    pub fn new(
        algorithm: Algorithm,
        distance: fn(&str, &str) -> f64,
        max_edits: Option<i32>,
        similarity_threshold: Option<f64>,
        targets: &[Drug],
    ) -> DrugInput {
        DrugInput {
            algorithm,
            distance,
            max_edits,
            similarity_threshold,
            targets: targets.to_vec(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DrugOutput {
    pub record_id: Option<String>,
    pub algorithm: Algorithm,
    pub edits: Option<i32>,
    pub similarity: f64,
    pub matched_term: String,
    pub drug: Drug,
}

impl SearchInput for DrugInput {
    fn scan(&self, text: &str, record: Option<String>) -> Vec<SearchOutput> {
        let clean = text
            .replace(&['(', ')', ',', '\"', '.', ';', ':'][..], "")
            .to_uppercase();
        let words = clean.split_whitespace();
        let mut results: Vec<DrugOutput> = Vec::new();
        for word in words {
            for target in &self.targets {
                let d = (self.distance)(target.name.to_uppercase().as_str(), word);
                let res = DrugOutput {
                    record_id: record.clone(),
                    matched_term: word.to_string(),
                    algorithm: self.algorithm,
                    edits: if self.algorithm.is_edits() {
                        Some(d as i32)
                    } else {
                        None
                    },
                    similarity: if self.algorithm.is_edits() {
                        1.0 - (d / (target.name.chars().count().max(word.chars().count()) as f64))
                    } else {
                        d
                    },
                    drug: target.to_owned(),
                };
                results.push(res);
            }
        }
        for r in &results {
            if r.edits.unwrap() < 4 {
                println!("{:?}", r);
            }
        }
        if self.max_edits.is_some() {
            // filter by edits
            let edits = self.max_edits.unwrap();
            results
                .into_iter()
                .filter(|x| x.edits.unwrap() <= edits)
                .map(SearchOutput::DrugOutput)
                .collect::<Vec<SearchOutput>>()
        } else if self.similarity_threshold.is_some() {
            // filter by similarity
            let thresh = self.similarity_threshold.unwrap();
            results
                .into_iter()
                .filter(|x| x.similarity >= thresh)
                .map(SearchOutput::DrugOutput)
                .collect::<Vec<SearchOutput>>()
        } else {
            // return all
            results.into_iter().map(SearchOutput::DrugOutput).collect()
        }
    }
}

pub fn fetch_drugs(class_id: &str, rela_source: &str) -> Vec<Drug> {
    let url = format!(
        "https://rxnav.nlm.nih.gov/REST/rxclass/classMembers.json?classId={}&relaSource={}",
        class_id, rela_source
    );
    let res = reqwest::blocking::get(url).unwrap().json::<Root>().unwrap();
    res.drug_member_group
        .drug_member
        .iter()
        .map(|item| Drug {
            name: item.min_concept.name.to_string(),
            rx_id: item.min_concept.rxcui.to_string(),
            group_name: "Opioid".to_string(),
            class_id: class_id.to_string(),
        })
        .collect::<Vec<Drug>>()
}

pub fn initialize_searcher(
    algorithm: Algorithm,
    distance: fn(&str, &str) -> f64,
    max_edits: Option<i32>,
    similarity_threshold: Option<f64>,
    search_words: Option<&[String]>,
    drug_list: Option<Vec<Drug>>,
) -> Box<dyn SearchInput> {
    if let Some(drugs) = drug_list {
        Box::new(DrugInput::new(
            algorithm,
            distance,
            max_edits,
            similarity_threshold,
            drugs.as_ref(),
        ))
    } else {
        Box::new(SimpleInput::new(
            algorithm,
            distance,
            max_edits,
            similarity_threshold,
            search_words.unwrap(),
        ))
    }
}

////// nonesense  for parsing json ////////
///
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub drug_member_group: DrugMemberGroup,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrugMemberGroup {
    pub drug_member: Vec<DrugMember>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrugMember {
    pub min_concept: MinConcept,
    pub node_attr: Vec<NodeAttr>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinConcept {
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
