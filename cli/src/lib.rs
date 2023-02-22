use std::{
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::Read,
    path::Path,
    str::FromStr,
};

use color_eyre::{
    eyre::{eyre, Context, ContextCompat},
    Help, Report,
};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressIterator, ProgressStyle};
use itertools::Itertools;
use rayon::prelude::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};
use serde::{Deserialize, Serialize};

// TODO: switch to generic type params not impl
// TODO: use lifetimes where possible to avoid cloning

#[derive(Deserialize, Debug, Clone)]
pub struct SearchTerm {
    /// The search term
    pub word: String,
    /// Optional metadata to include in the output
    pub metadata: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct StandardOutput {
    #[serde(rename = "Similarity Score")]
    pub sim: f64,
    #[serde(rename = "Search Term")]
    pub target: String,
    #[serde(rename = "Matched Term")]
    pub match_: String,
    #[serde(rename = "Metadata")]
    pub metadata: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct IdentifiedOutput {
    #[serde(rename = "Row ID")]
    pub row_id: String,
    #[serde(rename = "Similarity Score")]
    pub sim: f64,
    #[serde(rename = "Search Term")]
    pub target: String,
    #[serde(rename = "Matched Term")]
    pub match_: String,
    #[serde(rename = "Metadata")]
    pub metadata: Option<String>,
    #[serde(rename = "Source Column")]
    pub column: String,
}

fn remove_symbols_except_dash(s: &str) -> String {
    let ss: String = s
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' {
                c
            } else {
                ' '
            }
        })
        .collect();
    ss.trim().to_ascii_uppercase()
}

fn validate_terms_headers(header: &csv::StringRecord) -> usize {
    match header.len() {
        usize::MIN..=0 => {
            println!("The search terms file must have at least 1 column");
            std::process::exit(1);
        }
        1..=2 => header.len(), // this is good, return length so we know how many columns to expect
        3..=usize::MAX => {
            println!(
                "The search terms file must have 1-2 columns, detected {} columns",
                header.len()
            );
            std::process::exit(1);
        }
        _ => {
            println!("Unexpected number of columns detected. Expected 1-2 columns in the search term file.");
            std::process::exit(1);
        }
    }
}

pub fn load_search_terms<P: AsRef<Path>>(filename: &P) -> Result<Vec<SearchTerm>, Report> {
    let cwd = env::current_dir()
        .wrap_err("Could not get current working directory. Please check your permissions.")?;
    let fpath = cwd.join(filename);
    let mut rdr = csv::Reader::from_path(fpath).wrap_err("Could not find search terms file.")?;

    let num_cols = validate_terms_headers(
        rdr.headers()
            .wrap_err("Could not read headers. Check for valid UTF-8")?,
    );
    let terms = match num_cols {
        1 => read_terms_only(&mut rdr),
        2 => read_terms_with_tags(&mut rdr),
        _ => {
            println!("Unexpected number of columns returned from validator. Expected 2 columns in the search term file.");
            println!("Please check the format of the search term file.");
            std::process::exit(1);
        }
    };
    terms
}

fn initialize_dataset_reader<P: AsRef<Path>>(file_path: &P) -> Result<csv::Reader<File>, Report> {
    let cwd = env::current_dir()
        .wrap_err("Could not get current working directory. Please check your permissions.")?;
    let fpath = cwd.join(file_path);
    let file = File::open(fpath).wrap_err("Could not find dataset file.")?;
    let rdr = csv::Reader::from_reader(file);
    Ok(rdr)
}

pub fn read_headers<P: AsRef<Path>>(file_path: &P) -> Result<Vec<String>, Report> {
    let mut rdr = initialize_dataset_reader(file_path)?;
    let header_row = rdr
        .headers()
        .wrap_err("Could not read headers. Check for valid UTF-8")?
        .iter()
        .map(|s| s.to_string())
        .collect_vec();
    Ok(header_row)
}

fn find_column_index(header: &csv::StringRecord, search_col: &str) -> usize {
    header
        .iter()
        .position(|c| c.to_ascii_uppercase() == search_col.to_ascii_uppercase())
        .wrap_err("Could not find column in dataset.")
        .unwrap()
}

pub fn load_dataset_words_only<P: AsRef<Path>>(
    file_path: &P,
    search_cols: &[String],
) -> Result<HashSet<String>, Report> {
    let mut rdr = initialize_dataset_reader(file_path)?;
    let header_row = rdr
        .headers()
        .wrap_err("Could not read headers. Check for valid UTF-8")?;
    let target_col_indices = search_cols
        .iter()
        .map(|col| find_column_index(header_row, col))
        .collect_vec();
    let words = rdr
        .records()
        .par_bridge()
        .progress_with(ProgressBar::new_spinner().with_message("Loading dataset..."))
        .flat_map(|row| {
            row.unwrap_or_else(|e| {
                eprintln!("Error reading row: {}", e);
                std::process::exit(1);
            })
            .iter()
            .enumerate()
            .filter(|(i, _)| target_col_indices.contains(i))
            .flat_map(|(_, cell)| {
                cell.split_whitespace()
                    .map(remove_symbols_except_dash)
                    .unique()
                    .collect::<HashSet<String>>()
            })
            .unique()
            .collect::<HashSet<String>>()
        })
        .collect();
    Ok(words)
}

fn identify_words(
    data: Vec<(String, String, String)>,
) -> HashMap<String, HashMap<String, HashSet<String>>> {
    let mut identified_words: HashMap<String, HashMap<String, HashSet<String>>> = HashMap::new();
    for (word, id, col) in data
        .iter()
        .progress_with(ProgressBar::new_spinner().with_message("Building word index..."))
    {
        identified_words
            .entry(word.to_string())
            .and_modify(|word_info| {
                word_info
                    .entry(col.to_string())
                    .and_modify(|ids| {
                        ids.insert(id.clone());
                    })
                    .or_insert_with(|| {
                        let mut ids = HashSet::new();
                        ids.insert(id.clone());
                        ids
                    });
            })
            .or_insert_with(|| {
                let mut info: HashMap<String, HashSet<String>> = HashMap::new();
                info.entry(col.to_string())
                    .and_modify(|ids| {
                        ids.insert(id.clone());
                    })
                    .or_insert_with(|| {
                        let mut ids = HashSet::new();
                        ids.insert(id.clone());
                        ids
                    });
                info
            });
    }
    identified_words
}

pub fn load_dataset_identified<P: AsRef<Path>>(
    file_path: &P,
    search_cols: &[String],
    id_col: &str,
) -> Result<HashMap<String, HashMap<String, HashSet<String>>>, Report> {
    let mut rdr = initialize_dataset_reader(file_path)?;
    let header_row = rdr
        .headers()
        .wrap_err("Could not read headers. Check for valid UTF-8")?;
    let target_col_indices = search_cols
        .iter()
        .map(|col| find_column_index(header_row, col))
        .collect_vec();
    let id_col_index = find_column_index(header_row, id_col);
    let header_items: Vec<String> = header_row.into_iter().map(|s| s.to_string()).collect();

    let data: Vec<(String, String, String)> = rdr
        .records()
        .par_bridge()
        .progress_with(ProgressBar::new_spinner().with_message("Loading dataset..."))
        .flat_map(|row| {
            let record = row.unwrap_or_else(|e| {
                eprintln!("Error reading row: {}", e);
                std::process::exit(1);
            });
            let id = record
                .get(id_col_index)
                .wrap_err("Unable to parse id column")
                .unwrap()
                .to_string();
            record
                .iter()
                .enumerate()
                .filter(|(i, _)| target_col_indices.contains(i))
                .flat_map(|(j, cell)| {
                    cell.split_whitespace()
                        .map(remove_symbols_except_dash)
                        .unique()
                        .map(|word| (word, id.clone(), header_items[j].to_owned()))
                        .collect::<Vec<(String, String, String)>>()
                })
                .collect_vec()
        })
        .collect();

    let id_words = identify_words(data);
    Ok(id_words)
}

pub fn load_stdin_words() -> Result<HashSet<String>, Report> {
    let std_input = read_stdin_to_string()?;
    let clean_text = remove_symbols_except_dash(&std_input);
    let words: HashSet<String> = clean_text
        .split_whitespace()
        .map(|s| s.to_string())
        .unique()
        .collect();
    Ok(words)
}

fn read_stdin_to_string() -> Result<String, Report> {
    let mut buffer = String::new();
    std::io::stdin()
        .read_to_string(&mut buffer)
        .wrap_err("Could not read standard input.")
        .suggestion("Please try again.")?;
    Ok(buffer)
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

pub fn initialize_progress_with_style(kind: &str) -> Result<ProgressStyle, Report> {
    let progress_enum = ProgressKind::from_str(kind)?;
    match progress_enum {
        ProgressKind::Spinner => Ok(ProgressStyle::default_spinner()),
        ProgressKind::Bar => Ok(ProgressStyle::with_template(
            "({elapsed_precise}) [{bar:.cyan/blue}] {pos}/{len} ({eta_precise})",
        )
        .wrap_err("Couldn't initialize progress bar. Please file bug report.")?
        .progress_chars("#>-")),
    }
}

fn read_terms_only(reader: &mut csv::Reader<File>) -> Result<Vec<SearchTerm>, Report> {
    reader
        .records()
        .par_bridge()
        .progress_with(ProgressBar::new_spinner().with_message("Loading terms..."))
        .map(|result| {
            let row = result.wrap_err("Error reading row")?;
            let word = row
                .get(0)
                .wrap_err("No data found in column 0")?
                .to_uppercase();
            Ok(SearchTerm {
                word,
                metadata: None,
            })
        })
        .collect()
}

fn read_terms_with_tags(reader: &mut csv::Reader<File>) -> Result<Vec<SearchTerm>, Report> {
    reader
        .records()
        .par_bridge()
        .progress_with(ProgressBar::new_spinner().with_message("Loading terms with metadata..."))
        .map(|result| {
            let row = result.wrap_err("Error reading row")?;
            let word = row
                .get(0)
                .wrap_err("No data found in column 0")?
                .to_uppercase();
            let metadata = row
                .get(1)
                .wrap_err("No data found in column 1")?
                .to_uppercase();
            Ok(SearchTerm {
                word,
                metadata: Some(metadata),
            })
        })
        .collect()
}

pub fn find_matches<'a, W: Iterator<Item = &'a String> + Clone>(
    terms: &'a [SearchTerm],
    words: W,
    threshold: f64,
    pb_style: &ProgressStyle,
) -> Vec<(&'a SearchTerm, &'a String, f64)> {
    terms
        .iter()
        .cartesian_product(words.into_iter())
        .collect_vec()
        .par_iter()
        .progress_with_style(pb_style.to_owned())
        .filter_map(|(term, word)| {
            let sim = strsim::jaro_winkler(&term.word, word);
            if sim >= threshold {
                Some((*term, *word, sim))
            } else {
                None
            }
        })
        .collect()
}

pub fn assemble_standard_output<'a>(
    matches: &'a [(&'a SearchTerm, &'a String, f64)],
    pb_style: &ProgressStyle,
) -> Vec<StandardOutput> {
    matches
        .par_iter()
        .progress_with_style(pb_style.to_owned())
        .map(|(term, word, sim)| StandardOutput {
            target: term.word.to_string(),
            match_: word.to_string(),
            metadata: term.metadata.to_owned(),
            sim: *sim,
        })
        .collect()
}

pub fn assemble_identified_output(
    matches: &[(&SearchTerm, &String, f64)],
    lookup: &HashMap<String, HashMap<String, HashSet<String>>>,
) -> Vec<IdentifiedOutput> {
    // lookup is map of word -> map of col name -> set of ids
    matches
        .iter()
        .flat_map(|(term, word, sim)| {
            let word_entry = lookup
                .get(*word)
                .wrap_err("Unable to lookup word in map")
                .unwrap();
            let word_cols = word_entry.keys().collect_vec();
            word_cols
                .iter()
                .flat_map(|col| {
                    let word_ids = word_entry
                        .get(*col)
                        .wrap_err("Unable to lookup col in map")
                        .unwrap();
                    word_ids.iter().map(|id| IdentifiedOutput {
                        row_id: id.to_string(),
                        target: term.word.to_owned(),
                        match_: word.to_string(),
                        sim: *sim,
                        metadata: term.metadata.to_owned(),
                        column: col.to_string(),
                    })
                })
                .collect_vec()
        })
        .collect()
}
