//! This library exists to support the CLI and Web UI applications.
//!
//! It exposes a limited API that could be utilized by other applications.
//!
//! HOWEVER, its development will always be driven by the needs of the CLI and Web UI applications.
//!
//! The main functionality is encompassed in [`Drug`], [`Search`], and [`SearchOutput`].

use csv::WriterBuilder;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::error;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

use strsim::{damerau_levenshtein, jaro_winkler, levenshtein, osa_distance, sorensen_dice};

/// ValueError occurs when an invalid value was provided
#[derive(Debug)]
pub struct ValueError;

impl Display for ValueError {
    /// Formatting for ValueError
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Received an unexpected value")
    }
}

/// Make ValueError Error type
impl error::Error for ValueError {}

/// Type Alias for std::result::Result using ValueError
type Result<T> = std::result::Result<T, ValueError>;

/// Damerau Levenshtein Algorithm
/// https://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance
fn my_damerau(a: &str, b: &str) -> f64 {
    damerau_levenshtein(a, b) as f64
}

/// Levenshtein Algorithm
/// https://en.wikipedia.org/wiki/Levenshtein_distance
fn my_leven(a: &str, b: &str) -> f64 {
    levenshtein(a, b) as f64
}

/// Optimal &str Alignment Algorithm (OSA)
/// https://en.wikipedia.org/wiki/Optimal_string_alignment
fn my_osa(a: &str, b: &str) -> f64 {
    osa_distance(a, b) as f64
}

/// Jaro-Winkler Algorithm
/// https://en.wikipedia.org/wiki/Jaro%E2%80%93Winkler_distance
fn my_jw(a: &str, b: &str) -> f64 {
    jaro_winkler(a, b) as f64
}

/// Sorensen-Dice Algorithm
/// https://en.wikipedia.org/wiki/S%C3%B8rensen%E2%80%93Dice_coefficient
fn my_sd(a: &str, b: &str) -> f64 {
    sorensen_dice(a, b) as f64
}

/// Initialize the distance function based on the selected [`Algorithm`]
pub fn initialize_distance(a: Algorithm) -> fn(&str, &str) -> f64 {
    match a {
        Algorithm::DAMERAU => my_damerau,
        Algorithm::LEVENSHTEIN => my_leven,
        Algorithm::JAROWINKLER => my_jw,
        Algorithm::OSA => my_osa,
        Algorithm::SORENSENDICE => my_sd,
    }
}

/// Algorithm enum
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Algorithm {
    /// Damerau Levenshtein Algorithm
    DAMERAU,
    /// Levenshtein Algorithm
    LEVENSHTEIN,
    /// Jaro Winkler Algorithm
    JAROWINKLER,
    /// Optimal String Alignment Algorithm (OSA)
    OSA,
    /// Sorensen Dice Algorithm
    SORENSENDICE,
}

impl Algorithm {
    /// Utility function to see if the select Algorithm is returning an edit distance or similarity score.
    pub fn is_edits(&self) -> bool {
        match self {
            Algorithm::OSA | Algorithm::DAMERAU | Algorithm::LEVENSHTEIN => true,
            Algorithm::JAROWINKLER | Algorithm::SORENSENDICE => false,
        }
    }

    /// Utility function to get a list of the available algorithms as a string
    /// This is used for the CLI
    pub fn options() -> &'static [&'static str] {
        &[
            "Levenshtein",
            "Damerau",
            "OSA",
            "JaroWinkler",
            "SorensenDice",
        ]
    }
}

impl FromStr for Algorithm {
    type Err = ValueError;
    /// Parses an Algorithm type from a string reference.
    /// Only uses the first character of the string.
    fn from_str(s: &str) -> Result<Algorithm> {
        match s.to_uppercase().chars().next().unwrap_or('L') {
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
    /// Converts an Algorithm type to a string representation.
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

/// A struct to hold the results of a [`SimpleSearch::scan()`].
///
/// Simple Search focuses on comparing strings which could be anything provided by the user.
///
/// Simple Search uses [`SimpleSearch`] and [`SimpleResult`] to hold the input and output data.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SimpleResult {
    /// The ID of the record being searched.
    /// Can be empty if the record flag was not used in the CLI.
    pub record_id: Option<String>,
    /// The algorithm used to calculate the score.
    pub algorithm: Algorithm,
    /// The number of edits between the matched words.
    ///
    /// Can be empty if the [`Algorithm::is_edits()`] function returns false (thus the algorithm does not have an edit distance).
    pub edits: Option<i32>,
    /// The similarity score between the matched words.
    ///
    /// This will be computed regardless of the [`Algorithm::is_edits()`] function status since edit distance can be converted to similarity.
    pub similarity: f64,
    /// The search term.
    pub search_term: String,
    /// The matched word.
    pub matched_term: String,
}

/// The desired output format.
#[derive(Clone, Copy, Debug)]
pub enum OutputFormat {
    JSONL,
    CSV,
}

impl FromStr for OutputFormat {
    type Err = ValueError;
    /// Parses an OutputFormat type from a string reference.
    fn from_str(s: &str) -> Result<OutputFormat> {
        match s.to_uppercase().as_str() {
            "JSONL" => Ok(OutputFormat::JSONL),
            "CSV" => Ok(OutputFormat::CSV),
            _ => Err(ValueError),
        }
    }
}

/// Format the data in the desired output format.
/// This is used for the CLI and the web API.
/// The output format is determined by the [`OutputFormat`] enum.
/// This uses serde_json::to_string_pretty for JSONL and csv::WriterBuilder for CSV.
/// The output is returned as a Vector of Strings.
/// For CSV, the first row is the column headers and the vector items will need to be string-joined with a comma.
///
/// Examples:
/// TODO: Add examples
pub fn format(
    data: Vec<SearchOutput>,
    format: OutputFormat,
) -> std::result::Result<Vec<String>, Box<dyn Error>> {
    match format {
        OutputFormat::JSONL => {
            Ok(data
                .iter()
                .map(|x| match x {
                    SearchOutput::DrugResult(y) => serde_json::to_string(y)
                        .expect("could not deserialize drug result to string"),
                    SearchOutput::SimpleResult(y) => serde_json::to_string(y)
                        .expect("could not deserialize simple result to string"),
                })
                .collect::<Vec<String>>())
        }
        OutputFormat::CSV => {
            let mut wtr = WriterBuilder::new().has_headers(false).from_writer(vec![]);
            for row in data {
                wtr.serialize(row)?;
            }
            let csv_data = String::from_utf8(wtr.into_inner()?)?;
            Ok(csv_data
                .split('\n')
                .map(|x| x.to_string())
                .filter(|x| !x.is_empty())
                .collect::<Vec<String>>())
        }
    }
}

/// A struct to hold the input into a Simple Search.
///
/// Simple Search focuses on comparing strings which could be anything provided by the user.
///
/// Simple Search uses [`SimpleSearch`] and [`SimpleResult`] to hold the input and output data.
pub struct SimpleSearch {
    /// The [`Algorithm`] to use.
    pub algorithm: Algorithm,
    /// The distance function to use, based on the [`Algorithm`] selected.
    pub distance: fn(&str, &str) -> f64,
    /// The **maximum** number of edits allowed.
    ///
    /// This *can* be None if the user does not want to limit the results based on the number of edits.
    ///
    /// This *should* be None if the [`Algorithm`] is not an edit distance.
    pub max_edits: Option<i32>,
    /// The *minimum* similarity score required.
    ///
    /// This **can** be None if the user does not want to limit the results based on the similarity score.
    pub similarity_threshold: Option<f64>,
    /// The target search words in the format of a vector of strings.
    pub targets: Vec<String>,
}

impl SimpleSearch {
    /// Create a new SimpleSearch struct.
    pub fn new(
        algorithm: Algorithm,
        distance: fn(&str, &str) -> f64,
        max_edits: Option<i32>,
        similarity_threshold: Option<f64>,
        targets: &[String],
    ) -> SimpleSearch {
        SimpleSearch {
            algorithm,
            distance,
            max_edits,
            similarity_threshold,
            targets: targets.to_vec(),
        }
    }
}

/// A struct to hold the results of a [`Search::scan()`].
///
/// This is used for the CLI and the web API.
///
/// The enum will correspond to the type of search run (Simple/Drug).
///
/// This will show up in the JSONL and CSV output to assist the user in understanding the results.
///
/// TODO: Add examples
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchOutput {
    SimpleResult(SimpleResult),
    DrugResult(DrugResult),
}

/// Search trait.
pub trait Search {
    /// Scan the data for matches.
    fn scan(&self, text: &str, record: Option<String>) -> Vec<SearchOutput>;
}

impl Search for SimpleSearch {
    /// Scanning function to find matches.
    ///
    /// Searches the input text for the target words. This also does some pre-processing to remove
    /// punctuation and other non-alphanumeric characters as well as upper-casing the input text.
    ///
    /// The search will be limited by the number of edits and/or similarity threshold (if) provided in the [`SimpleSearch`] struct.
    ///
    /// The results will be returned as a vector of [`SimpleResult`] structs.
    ///
    /// # Examples
    /// TODO: Add examples
    /// ```rust
    /// let search = SimpleSearch::new(Algorithm::Levenshtein, levenshtein, None, None, &["hello", "world"]);
    /// let results = search.scan("hello world", None);
    /// ```
    ///
    fn scan(&self, text: &str, record: Option<String>) -> Vec<SearchOutput> {
        let clean = text
            .replace(&['(', ')', ',', '\"', '.', ';', ':', ']', '['][..], "")
            .to_uppercase();
        let words = clean.split_whitespace();
        let mut results: Vec<SimpleResult> = Vec::new();
        for word in words {
            for target in &self.targets {
                let d = (self.distance)(target, word);
                let res = SimpleResult {
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
        if let Some(me) = self.max_edits {
            // filter by edits
            results
                .into_iter()
                .filter(|x| x.edits.expect("result did not have edits") <= me)
                .map(SearchOutput::SimpleResult)
                .collect::<Vec<SearchOutput>>()
        } else if let Some(thresh) = self.similarity_threshold {
            // filter by similarity
            results
                .into_iter()
                .filter(|x| x.similarity >= thresh)
                .map(SearchOutput::SimpleResult)
                .collect::<Vec<SearchOutput>>()
        } else {
            // return all
            results
                .into_iter()
                .map(SearchOutput::SimpleResult)
                .collect()
        }
    }
}

/// A struct to hold data regarding a specific Drug.
///
/// This is used for the CLI and the web API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Drug {
    /// The name of the drug.
    pub name: String,
    /// The drug's RxNorm ID (RX_CUI).
    pub rx_id: String,
    /// The drug's RxClass ID.
    pub class_id: String,
}

/// A struct to hold search information for a Drug Search.
///
/// Drug Search focuses on comparing drug names to target text.
///
/// Drug Search uses [`DrugSearch`] and [`DrugResult`] to hold the input and output data.
pub struct DrugSearch {
    /// The [`Algorithm`] to use.
    pub algorithm: Algorithm,
    /// The distance function to use, based on the [`Algorithm`] selected.
    pub distance: fn(&str, &str) -> f64,
    /// The **maximum** number of edits allowed.
    ///
    /// This *can* be None if the user does not want to limit the results based on the number of edits.
    ///
    /// This *should* be None if the [`Algorithm`] is not an edit distance.
    pub max_edits: Option<i32>,
    /// The *minimum* similarity score required.
    pub similarity_threshold: Option<f64>,
    /// The target search words in the format of a vector of [`Drug`]s.
    pub targets: Vec<Drug>,
}

impl DrugSearch {
    /// Create a new DrugSearch struct.
    pub fn new(
        algorithm: Algorithm,
        distance: fn(&str, &str) -> f64,
        max_edits: Option<i32>,
        similarity_threshold: Option<f64>,
        targets: &[Drug],
    ) -> DrugSearch {
        DrugSearch {
            algorithm,
            distance,
            max_edits,
            similarity_threshold,
            targets: targets.to_vec(),
        }
    }
}

/// A struct to hold the results of a [`Search::scan()`].
///
/// Drug Search focuses on comparing drug names to target text.
///
/// Drug Search uses [`DrugSearch`] and [`DrugResult`] to hold the input and output data.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DrugResult {
    pub record_id: Option<String>,
    pub algorithm: Algorithm,
    pub edits: Option<i32>,
    pub similarity: f64,
    pub matched_term: String,
    pub drug: Drug,
}

impl Search for DrugSearch {
    /// Scanning function to find matches.
    ///
    /// Searches the input text for the target drug names. This also does some pre-processing to remove
    /// punctuation and other non-alphanumeric characters as well as upper-casing the input text.
    ///
    /// The search will be limited by the number of edits and/or similarity threshold (if) provided in the [`DrugSearch`] struct.
    ///
    /// The results will be returned as a vector of [`DrugResult`] structs.
    ///
    /// # Examples
    /// TODO: Add examples
    /// ```rust
    /// let search = DrugSearch::new(Algorithm::Levenshtein, levenshtein, None, None, &["hello", "world"]);
    /// let results = search.scan("hello world", None);
    /// ```
    ///
    fn scan(&self, text: &str, record: Option<String>) -> Vec<SearchOutput> {
        let clean = text
            .replace(&['(', ')', ',', '\"', '.', ';', ':'][..], "")
            .to_uppercase();
        let words = clean.split_whitespace();
        let mut results: Vec<DrugResult> = Vec::new();
        for word in words {
            for target in &self.targets {
                for t in target.name.split('/') {
                    let d = (self.distance)(t.to_uppercase().as_str(), word);
                    let res = DrugResult {
                        record_id: record.clone(),
                        matched_term: word.to_string(),
                        algorithm: self.algorithm,
                        edits: if self.algorithm.is_edits() {
                            Some(d as i32)
                        } else {
                            None
                        },
                        similarity: if self.algorithm.is_edits() {
                            1.0 - (d / (t.chars().count().max(word.chars().count()) as f64))
                        } else {
                            d
                        },
                        drug: target.to_owned(),
                    };
                    results.push(res);
                }
            }
        }
        if let Some(me) = self.max_edits {
            // filter by edits
            results
                .into_iter()
                .filter(|x| x.edits.expect("result did not have edits") <= me)
                .map(SearchOutput::DrugResult)
                .collect::<Vec<SearchOutput>>()
        } else if let Some(thresh) = self.similarity_threshold {
            // filter by similarity
            results
                .into_iter()
                .filter(|x| x.similarity >= thresh)
                .map(SearchOutput::DrugResult)
                .collect::<Vec<SearchOutput>>()
        } else {
            // return all
            results.into_iter().map(SearchOutput::DrugResult).collect()
        }
    }
}

/// A utility function to initialize the correct Searcher (Drug or Simple) based on user provided data.
///
/// Returns a Box<dyn Search> that will need to be unboxed.
pub fn initialize_searcher(
    algorithm: Algorithm,
    distance: fn(&str, &str) -> f64,
    max_edits: Option<i32>,
    similarity_threshold: Option<f64>,
    search_words: Option<&[String]>,
    drug_list: Option<Vec<Drug>>,
) -> Box<dyn Search> {
    if let Some(drugs) = drug_list {
        Box::new(DrugSearch::new(
            algorithm,
            distance,
            max_edits,
            similarity_threshold,
            drugs.as_ref(),
        ))
    } else {
        Box::new(SimpleSearch::new(
            algorithm,
            distance,
            max_edits,
            similarity_threshold,
            search_words.unwrap_or_default(),
        ))
    }
}

/// A function to get some nice stats about the drugs in the list.
pub fn analyze(
    data: Vec<SearchOutput>,
    total_targets: i32,
    total_records: i32,
    is_drug: bool,
    has_id: bool,
) -> Result<Vec<String>> {
    let mut results: Vec<String> = Vec::new();
    if data.is_empty() {
        results.push("Unable to analyze, no matches found.".to_string());
        return Ok(results);
    }
    if is_drug {
        if has_id {
            let mut found_targets: Vec<String> = Vec::new();
            let mut found_ids: Vec<String> = Vec::new();
            for r in data {
                if let SearchOutput::DrugResult(drug) = r {
                    found_targets.push(drug.drug.name.clone());
                    found_ids.push(
                        drug.record_id
                            .as_ref()
                            .expect("could not reference record id")
                            .clone(),
                    );
                }
            }
            let unique_records = found_ids.clone().into_iter().collect::<HashSet<_>>();
            results.push(format!(
                "Found drugs in {} of {} records (~{:.2}%).",
                unique_records.len(),
                total_records,
                100.0 * unique_records.len() as f64 / total_targets as f64
            ));
            let counts = found_ids.into_iter().counts();
            let key_with_max_value = counts
                .iter()
                .max_by_key(|entry| entry.1)
                .expect("could not find max");
            results.push(format!(
                "Most common record: {} (detected {} drugs)",
                key_with_max_value.0, key_with_max_value.1
            ));
            let unique_targets = found_targets
                .clone()
                .into_iter()
                .unique()
                .collect::<HashSet<_>>();
            results.push(format!(
                "Found {} of {} drugs (~{:.2}%).",
                unique_targets.len(),
                total_targets,
                100.0 * unique_targets.len() as f64 / total_targets as f64
            ));
            let counts = found_targets.into_iter().counts();
            let key_with_max_value = counts
                .iter()
                .max_by_key(|entry| entry.1)
                .expect("could not find max");
            results.push(format!(
                "The most common drug is {} with {} detections.",
                key_with_max_value.0, key_with_max_value.1
            ));
        } else {
            let mut found_targets: Vec<String> = Vec::new();
            results.push("No record ID flag provided.".to_string());
            for r in data {
                if let SearchOutput::DrugResult(drug) = r {
                    found_targets.push(drug.drug.name.clone());
                }
            }
            let unique_targets = found_targets.into_iter().unique().collect::<HashSet<_>>();
            results.push(format!(
                "Found {} of {} drugs (~{:.2}%).",
                unique_targets.len(),
                total_targets,
                100.0 * unique_targets.len() as f64 / total_targets as f64
            ));
            let counts = unique_targets.into_iter().counts();
            let key_with_max_value = counts
                .iter()
                .max_by_key(|entry| entry.1)
                .expect("could not find max");
            results.push(format!(
                "The most common drug is {} with {} detections.",
                key_with_max_value.0, key_with_max_value.1
            ));
        }
    } else if has_id {
        let mut found_targets: Vec<String> = Vec::new();
        let mut found_ids: Vec<String> = Vec::new();
        for r in data {
            if let SearchOutput::SimpleResult(simple) = r {
                found_targets.push(simple.search_term.clone());
                found_ids.push(
                    simple
                        .record_id
                        .as_ref()
                        .expect("could not reference record id")
                        .clone(),
                );
            }
        }
        let unique_records = found_ids.clone().into_iter().collect::<HashSet<_>>();
        results.push(format!(
            "Found targets in {} of {} records (~{:.2}%).",
            unique_records.len(),
            total_records,
            100.0 * unique_records.len() as f64 / total_records as f64,
        ));
        let counts = found_ids.clone().into_iter().counts();
        let key_with_max_value = counts
            .iter()
            .max_by_key(|(_, v)| *v)
            .expect("could not find max");
        results.push(format!(
            "Most common record: {} (detected {} targets)",
            key_with_max_value.0, key_with_max_value.1
        ));
        let unique_targets = found_targets
            .clone()
            .into_iter()
            .unique()
            .collect::<HashSet<_>>();
        results.push(format!(
            "Found {} of {} targets (~{:.2}%).",
            unique_targets.len(),
            total_targets,
            100.0 * unique_targets.len() as f64 / total_targets as f64
        ));
        let counts = found_targets.into_iter().counts();
        let key_with_max_value = counts
            .iter()
            .max_by_key(|(_, v)| *v)
            .expect("could not find max");
        results.push(format!(
            "The most common target is {} with {} detections.",
            key_with_max_value.0, key_with_max_value.1
        ));
    } else {
        let mut found_targets: Vec<String> = Vec::new();
        results.push("No record ID flag provided.".to_string());
        for r in data {
            if let SearchOutput::SimpleResult(simple) = r {
                found_targets.push(simple.search_term.clone());
            }
        }
        let unique_targets = found_targets
            .clone()
            .into_iter()
            .unique()
            .collect::<HashSet<_>>();
        results.push(format!(
            "Found {} of {} targets (~{:.2}%).",
            unique_targets.len(),
            total_targets,
            100.0 * unique_targets.len() as f64 / total_targets as f64
        ));
        let counts = found_targets.into_iter().counts();
        let key_with_max_value = counts
            .iter()
            .max_by_key(|(_, v)| *v)
            .expect("could not find max");
        results.push(format!(
            "The most common target is {} with {} detections.",
            key_with_max_value.0, key_with_max_value.1
        ));
    }
    Ok(results)
}
