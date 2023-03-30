use std::{fmt::Display, fs::File, str::FromStr};

use color_eyre::{eyre::eyre, Report};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
pub struct SearchTerm {
    pub term: String,
    pub metadata: Option<String>,
}

impl SearchTerm {
    pub fn ngrams(&self) -> usize {
        self.term.split_ascii_whitespace().count()
    }
}

#[derive(Debug)]
pub struct DataSetInfo {
    pub reader: csv::Reader<File>,
    pub header: Vec<String>,
    pub search_column_indices: Vec<usize>,
}

#[derive(Debug)]
pub enum ProgressKind {
    Spinner,
    Bar,
}

impl FromStr for ProgressKind {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "spinner" => Ok(ProgressKind::Spinner),
            "bar" => Ok(ProgressKind::Bar),
            _ => Err(eyre!("{} is not a valid progress kind", s)),
        }
    }
}

//* Outputs Section *//

#[derive(Debug, Serialize)]
pub struct Output<'a> {
    #[serde(rename = "Row ID")]
    pub row_id: &'a str,
    #[serde(rename = "Search Term")]
    pub target_term: &'a str,
    #[serde(rename = "Match")]
    pub matched: &'a str,
    #[serde(rename = "Similarity")]
    pub sim: f64,
    #[serde(rename = "Source Column")]
    pub column: Option<&'a str>,
    #[serde(rename = "Metadata")]
    pub metadata: Option<&'a str>,
}

impl Display for Output<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{},{}",
            self.row_id, self.target_term, self.matched, self.sim,
        )
    }
}
