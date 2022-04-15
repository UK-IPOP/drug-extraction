use std::str::SplitWhitespace;

use std::error::Error;
use strsim::{damerau_levenshtein, jaro_winkler, levenshtein, osa_distance, sorensen_dice};

/// Will need to be modified/extended to account for drug tags
/// This will be serialized directly into json so this should
/// be our final data structure that we want in the output

pub struct DistanceResult {
    edits: Option<i32>,
    similarity: f64,
    search: SimpleSearcher,
    matched: String,
    algorithm: dyn Algorithm,
}

pub trait Algorithm {
    // distance should take self.search_term() and compare to x
    fn distance(source: &str, target: &str) -> f64;
}
pub struct SimpleSearcher {
    word: String,
    group: Option<String>,
    algorithm: dyn Algorithm,
}

pub trait Searcher {
    fn search_term(&self) -> &str;
    fn compare(&self, x: &str) -> f64;
}

impl Searcher for SimpleSearcher {
    fn search_term(&self) -> &str {
        self.word.as_str()
    }
    fn compare(&self, x: &str) -> f64 {
        self.algorithm.distance(self.search_term(), x)
    }
}

fn search(searcher: &dyn Searcher, text: &str) -> Vec<f64> {
    let r = text
        .trim()
        .split_whitespace()
        .map(|x| {
            let res = searcher.compare(x);
            hi()
            res
        })
        .collect();
    r
}

fn run() {
    let s = SimpleSearcher {
        word: "ih".to_string(),
        group: None,
    };
    let res = search(&s, "nik is cool ih likes drugs");
    println!("{:?}", res);
}

