use color_eyre::{
    eyre::{eyre, Context, ContextCompat},
    Result,
};
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};

use std::{collections::HashSet, fs::File, path::Path, time::Duration};

use serde::{Deserialize, Serialize};

use itertools::Itertools;

/// Create a spinner with default style, takes a message
fn initialize_spinner_style(msg: String) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.with_style(
        ProgressStyle::with_template("{spinner:.blue} {msg}")
            .unwrap()
            .tick_strings(&[
                "▹▹▹▹▹",
                "▸▹▹▹▹",
                "▹▸▹▹▹",
                "▹▹▸▹▹",
                "▹▹▹▸▹",
                "▹▹▹▹▸",
                "▪▪▪▪▪",
            ]),
    )
    .with_message(msg)
}

/// Initialize a progress bar with default style, takes a message and length
fn initialize_progress_bar(msg: String, len: u64) -> ProgressBar {
    let pb = ProgressBar::new(len);
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.with_style(
        ProgressStyle::default_bar()
            .template("{spinner:.blue} {msg} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} ({eta})").unwrap()
            .progress_chars("##-"),
    )
    .with_message(msg)
}

/// Struct to hold search term and metadata
#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct SearchTerm {
    /// The search term
    pub term: String,
    /// Optional metadata to be included in output
    pub metadata: Option<String>,
}

/// Struct to hold search output
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct SearchOutput<'a> {
    /// The row id of the matched record, either from specified column or line number
    row_id: &'a str,
    /// The search term that was matched
    search_term: &'a str,
    /// The matched term from the record
    matched_term: &'a str,
    /// The number of edits required to match the search term
    edits: usize,
    /// The similarity score between the search term and the matched term
    similarity_score: f64,
    /// The field that was searched
    search_field: &'a str,
    /// The metadata associated with the search term
    metadata: &'a Option<String>,
}

/// Function to read in search terms from a csv file
/// Performs cleaning of terms, ignoring metadata column
pub fn read_terms_from_file<P: AsRef<Path>>(p: P) -> Result<Vec<SearchTerm>> {
    let mut rdr = csv::Reader::from_path(p).wrap_err("Unable to read search terms file")?;
    let mut records: Vec<SearchTerm> = Vec::new();
    for (i, row) in rdr
        .deserialize()
        .enumerate()
        .progress_with(initialize_spinner_style(
            "Loading Search Terms...".to_string(),
        ))
    {
        let mut record: SearchTerm =
            row.wrap_err(format!("Could not load search term from line: {}", i))?;
        record.term = clean_text(&record.term);
        records.push(record);
    }
    records.sort_by_key(|x| x.term.split_ascii_whitespace().count());
    Ok(records)
}

/// Function to remove non-alphanumeric characters from a string
/// keeps hyphens due to their usage in abbreviations/medical terms.
/// Also uppercase for standardization.
/// Example:
/// ```
/// use drug_extraction_cli::clean_text;
///
/// let s = "This is a test-string with 1234 and some punctuation!@#$%^&*()";
/// let cleaned = clean_text(s);
/// assert_eq!(cleaned, "THIS IS A TEST STRING WITH 1234 AND SOME PUNCTUATION");
/// ```
pub fn clean_text(s: &str) -> String {
    // TODO: remove hyphenation and fix doc-test
    s.replace(|c: char| !c.is_ascii_alphanumeric(), " ")
    // s.replace(|c: char| !c.is_ascii_alphanumeric() && c != '-', " ")
        .trim()
        .to_ascii_uppercase()
}

/// Struct to hold information about the dataset
#[derive(Debug)]
pub struct DataSet {
    /// csv reader for the dataset
    pub reader: csv::Reader<File>,
    /// rows in the dataset from first scan
    pub rows: usize,
    /// indices of the columns to search in the dataset
    pub clean_search_columns: Vec<ColumnInfo>,
    /// index of the column to use as an id
    pub clean_id_column: Option<ColumnInfo>,
    /// csv writer for the output file
    pub writer: csv::Writer<File>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ColumnInfo {
    pub name: String,
    pub index: usize,
}

/// Function to get the column index for a given column name
/// Returns an error if the column name is not found
/// Typically ran using the header from the csv reader and is called
/// inside the [collect_column_info] function to do this for each target column.
/// Example:
/// ```
/// use drug_extraction_cli::get_column_info;
///
/// let header = vec!["ID", "NAME", "DESCRIPTION"];
/// let column = "NAME";
/// let column_info = get_column_info(&header, &column);
/// assert!(column_info.is_ok());
/// let column_info = column_info.unwrap();
/// assert_eq!(column_info.name, "NAME");
/// assert_eq!(column_info.index, 1);
/// ```
pub fn get_column_info<S: AsRef<str> + PartialEq>(header: &[S], column: &S) -> Result<ColumnInfo> {
    let pos = header.iter().position(|h| h == column);
    match pos {
        Some(i) => Ok(ColumnInfo {
            name: column.as_ref().to_string(),
            index: i,
        }),
        None => Err(eyre!("Unable to find column {}", column.as_ref())),
    }
}

/// Function to collect column info for each column to search
/// Typically ran using the header from the csv reader and is called
/// inside the [initialize_dataset] function to do this for each target column.
/// Example:
/// ```
/// use drug_extraction_cli::collect_column_info;
///
/// let header = vec!["ID", "NAME", "DESCRIPTION"];
/// let columns = vec!["NAME", "DESCRIPTION"];
/// let column_info = collect_column_info(&header, &columns);
/// assert!(column_info.is_ok());
/// let column_info = column_info.unwrap();
/// assert_eq!(column_info.len(), 2);
/// assert_eq!(column_info[0].name, "NAME");
/// assert_eq!(column_info[0].index, 1);
/// assert_eq!(column_info[1].name, "DESCRIPTION");
/// assert_eq!(column_info[1].index, 2);
/// ```
pub fn collect_column_info<S: AsRef<str> + PartialEq>(
    header: &[S],
    column_names: &[S],
) -> Result<Vec<ColumnInfo>> {
    column_names
        .iter()
        .map(|column| get_column_info(header, column))
        .collect()
}

/// Function to initialize the dataset
pub fn initialize_dataset<P: AsRef<Path>>(
    data_file: P,
    search_columns: &[String],
    id_column: Option<String>,
) -> Result<DataSet> {
    let mut rdr = csv::Reader::from_path(&data_file).wrap_err("Unable to initialize csv reader")?;
    let header = rdr
        .headers()
        .wrap_err("Unable to parse csv headers")?
        .iter()
        .map(clean_text)
        .collect_vec();
    // clean search cols and id col
    let clean_search_cols = search_columns.iter().map(|c| clean_text(c)).collect_vec();
    let clean_id_col = id_column.map(|c| clean_text(&c));
    let column_info = collect_column_info(&header, &clean_search_cols)
        .wrap_err("Unable to collect column indices")?;
    let ds = match clean_id_col {
        Some(c) => DataSet {
            reader: csv::Reader::from_path(&data_file)
                .wrap_err("Unable to initialize csv reader")?,
            rows: rdr.records().count(),
            clean_search_columns: column_info,
            clean_id_column: Some(get_column_info(&header, &c)?),
            writer: csv::Writer::from_path("output.csv")?,
        },
        None => DataSet {
            reader: csv::Reader::from_path(&data_file)
                .wrap_err("Unable to initialize csv reader")?,
            rows: rdr.records().count(),
            clean_search_columns: column_info,
            clean_id_column: None,
            writer: csv::Writer::from_path("output.csv")?,
        },
    };
    Ok(ds)
}

/// Primary search function
pub fn search(mut dataset: DataSet, search_terms: Vec<SearchTerm>) -> Result<()> {
    let mut total_records_with_matches = 0;
    let mut total_records = 0;
    let mut matched_terms: HashSet<&str> = HashSet::new();

    let spinner =
        initialize_progress_bar("Searching for matches...".to_string(), dataset.rows as u64);
    for (i, row) in dataset
        .reader
        .records()
        .enumerate()
        .progress_with(spinner.clone())
    {
        let record = row.wrap_err(format!("Unable to read record from line {}", i))?;

        let id = match &dataset.clean_id_column {
            Some(c) => record
                .get(c.index)
                .wrap_err(format!(
                    "Unable to read id column {} from line {}",
                    c.name, i
                ))?
                .to_string(),
            None => i.to_string(),
        };

        let mut found_match = false;
        for column in &dataset.clean_search_columns {
            let text = record.get(column.index).wrap_err(format!(
                "Unable to read column {} from line {}",
                column.name, i
            ))?;
            let cleaned_text = clean_text(text);
            let grams = cleaned_text.split_ascii_whitespace().collect_vec();
            for (term_len, term_list) in &search_terms
                .iter()
                .group_by(|st| st.term.split_ascii_whitespace().count())
            {
                let combos = if term_len == 1 {
                    term_list.cartesian_product(
                        grams
                            .iter()
                            .unique()
                            .map(|word| word.to_string())
                            .collect_vec(),
                    )
                } else {
                    term_list.cartesian_product(
                        grams
                            .windows(term_len)
                            .unique()
                            .map(|words| words.join(" "))
                            .collect_vec(),
                    )
                };
                for (search_term, comparison_term) in combos {
                    let edits = strsim::osa_distance(&search_term.term, &comparison_term);
                    match edits {
                        0 => {
                            dataset
                                .writer
                                .serialize(SearchOutput {
                                    row_id: &id,
                                    search_term: &search_term.term,
                                    matched_term: &comparison_term,
                                    edits,
                                    similarity_score: 1.0,
                                    search_field: &column.name,
                                    metadata: &search_term.metadata,
                                })
                                .wrap_err("Enable to serialize output")?;
                            found_match = true;
                            matched_terms.insert(&search_term.term);
                        }
                        1 => {
                            let sim = strsim::jaro_winkler(&search_term.term, &comparison_term);
                            if sim >= 0.95 {
                                dataset
                                    .writer
                                    .serialize(SearchOutput {
                                        row_id: &id,
                                        search_term: &search_term.term,
                                        matched_term: &comparison_term,
                                        edits,
                                        similarity_score: sim,
                                        search_field: &column.name,
                                        metadata: &search_term.metadata,
                                    })
                                    .wrap_err("Enable to serialize output")?;
                                found_match = true;
                                matched_terms.insert(&search_term.term);
                            }
                        }
                        2 => {
                            let sim = strsim::jaro_winkler(&search_term.term, &comparison_term);
                            if sim >= 0.97 {
                                dataset
                                    .writer
                                    .serialize(SearchOutput {
                                        row_id: &id,
                                        search_term: &search_term.term,
                                        matched_term: &comparison_term,
                                        edits,
                                        similarity_score: sim,
                                        search_field: &column.name,
                                        metadata: &search_term.metadata,
                                    })
                                    .wrap_err("Enable to serialize output")?;
                                found_match = true;
                                matched_terms.insert(&search_term.term);
                            }
                        }
                        _ => continue,
                    }
                }
            }
        }
        if found_match {
            total_records_with_matches += 1;
        }
        total_records += 1;
    }
    dataset.writer.flush().wrap_err("Unable to flush writer")?;
    spinner.finish_with_message("Done!");

    println!(
        "Found matches in {:} of {:} records ({:.2}%)",
        total_records_with_matches,
        total_records,
        (total_records_with_matches as f64 / total_records as f64) * 100.0
    );
    println!(
        "Found {:} of {:} search terms ({:.2}%)",
        matched_terms.len(),
        search_terms.len(),
        (matched_terms.len() as f64 / search_terms.len() as f64) * 100.0
    );

    Ok(())
}

pub fn run_searcher<P: AsRef<Path>>(
    data_file: P,
    search_terms_file: P,
    search_columns: Vec<String>,
    id_column: Option<String>,
) -> Result<()> {
    let search_terms = read_terms_from_file(search_terms_file)?;
    let dataset = initialize_dataset(data_file, &search_columns, id_column)?;
    search(dataset, search_terms)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_text_no_changes() {
        let s = "This is a test string.";
        assert_eq!(clean_text(s), "this is a test string".to_ascii_uppercase());
    }

    #[test]
    fn test_clean_text_numeric() {
        let s = "This is a test string with 1234 numbers.";
        assert_eq!(
            clean_text(s),
            "this is a test string with 1234 numbers".to_ascii_uppercase()
        );
    }

    #[test]
    fn test_clean_text_symbols() {
        let s = "!@#$%^&*()_+-";
        assert_eq!(clean_text(s), "");
    }

    #[test]
    fn test_clean_empty() {
        let s = "";
        assert_eq!(clean_text(s), "");
    }

    #[test]
    fn test_clean_end_whitespace() {
        let s = "!! This is a test string.   ";
        assert_eq!(clean_text(s), "this is a test string".to_ascii_uppercase());
    }

    #[test]
    fn test_clean_end_whitespace2() {
        let s = "!! This is a test to test- - hyphenated string.   ";
        assert_eq!(clean_text(s), "this is a test to test    hyphenated string".to_ascii_uppercase());
    }

    #[test]
    fn test_whitespace_split() {
        let s = "!! This is a test to test- - hyphenated string.   ";
        assert_eq!(clean_text(s), "this is a test to test    hyphenated string".to_ascii_uppercase());
        let c = clean_text(s);
        let v = c.split_ascii_whitespace().collect_vec();
        assert_eq!(v, vec!["THIS", "IS", "A", "TEST", "TO", "TEST", "HYPHENATED", "STRING"]);
    }

    #[test]
    fn test_get_column_info() {
        let header = vec!["a", "b", "c"];
        let col = "a";
        assert_eq!(get_column_info(&header, &col).unwrap().index, 0);
    }

    #[test]
    fn test_get_column_info_errors() {
        let header = vec!["a", "b", "c"];
        let col = "d";
        assert!(get_column_info(&header, &col).is_err());
    }

    #[test]
    fn test_collect_column_info() {
        let header = vec!["a", "b", "c"];
        let cols = vec!["a", "b"];
        let info = collect_column_info(&header, &cols);
        assert!(info.is_ok());
        let info = info.unwrap();
        assert_eq!(info.len(), 2);
        assert_eq!(
            info,
            vec![
                ColumnInfo {
                    name: "a".to_string(),
                    index: 0
                },
                ColumnInfo {
                    name: "b".to_string(),
                    index: 1
                }
            ]
        );
    }

    #[test]
    fn test_collect_column_info_sample() -> Result<()> {
        let header = csv::Reader::from_path("../data/search_terms.csv")?
            .headers()?
            .into_iter()
            .map(clean_text)
            .collect_vec();
        let cols = vec!["term", "metadata"]
            .iter()
            .map(|c| clean_text(c))
            .collect_vec();
        let info = collect_column_info(&header, &cols)?;
        assert_eq!(info.len(), 2);
        Ok(())
    }

    #[test]
    fn test_enumerated_reader() {
        let mut reader = csv::Reader::from_path("../data/search_terms.csv").unwrap();
        let (i, _) = reader.records().enumerate().next().unwrap();
        assert!(i == 0);
    }
}
