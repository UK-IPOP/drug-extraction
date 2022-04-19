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

#[derive(Clone, Copy, Debug)]
pub enum Algorithm {
    DAMERAU,
    LEVENSHTEIN,
    JAROWINKLER,
    OSA,
    SORENSENDICE,
}

impl Algorithm {
    fn is_edits(&self) -> bool {
        match self {
            Algorithm::OSA | Algorithm::DAMERAU | Algorithm::LEVENSHTEIN => true,
            Algorithm::JAROWINKLER | Algorithm::SORENSENDICE => false,
        }
    }
}

impl FromStr for Algorithm {
    type Err = ValueError;
    /// Parses an Algorithm type from a string reference.
    fn from_str(s: &str) -> Result<Algorithm> {
        match s.to_uppercase().as_str() {
            "L" => Ok(Algorithm::LEVENSHTEIN),
            "D" => Ok(Algorithm::DAMERAU),
            "O" => Ok(Algorithm::OSA),
            "J" => Ok(Algorithm::JAROWINKLER),
            "S" => Ok(Algorithm::SORENSENDICE),
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

#[derive(Debug)]
pub struct Output {
    pub record_id: String,
    pub search_term: String,
    pub matched_term: String,
    pub algorithm: Algorithm,
    pub edits: Option<i32>,
    pub similarity: f64,
}

pub fn scan(
    a: Algorithm,
    distance: fn(&str, &str) -> f64,
    text: &str,
    record: &str,
    target: &str,
    limit: Option<f64>,
) -> Vec<Output> {
    let clean = text
        .replace(&['(', ')', ',', '\"', '.', ';', ':'][..], "")
        .to_uppercase();
    let words = clean.split_whitespace();
    let mut results: Vec<Output> = Vec::new();
    for word in words {
        let d = distance(target, word);
        let res = Output {
            record_id: record.to_string(),
            search_term: target.to_string(),
            matched_term: word.to_string(),
            algorithm: a,
            edits: if a.is_edits() { Some(d as i32) } else { None },
            similarity: if !a.is_edits() {
                d
            } else {
                1.0 - (d / (target.chars().count().max(word.chars().count()) as f64))
            },
        };
        results.push(res);
    }
    match limit {
        Some(l) => {
            let mut filt_results: Vec<Output> = Vec::new();
            for r in results {
                if r.similarity >= l {
                    filt_results.push(r);
                }
            }
            filt_results
        }
        None => results,
    }
}
