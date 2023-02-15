//! Primary application for drug extraction
//!
//!
//!
//!
//!

// TODO: remove 'expect' statements
// TODO: replace most `?` with handled errors

// TODO: remove itertools dependency and just use
// nested for loops

// eventually i want to:
//  use rayon parallel iterators
// color_eyre for better error reporting
//

// TODO: pick a style for these imports and follow it
// TODO: use doctests for examples and testing

use anyhow::Result;

use clap::{Parser, Subcommand};
use itertools::Itertools;
// use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::{self, Read},
    path::PathBuf,
};
use strsim::jaro_winkler;

#[derive(Parser, Debug)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Standard(StandardCli),
    Pipe(PipeCli),
    Interactive,
}

#[derive(Parser, Debug)]
struct InteractiveCli {}

#[derive(Parser, Debug)]
struct StandardCli {
    /// The search terms file.
    ///
    /// This file should contain a column named "term"
    /// and a column named "metadata"
    ///
    /// The metadata column can be empty or contain any
    /// additional information you want to associate with
    /// the search term.
    ///
    /// The term column should contain the search term.
    ///
    /// The term column must come before the metadata column.
    /// Thus: term, metadata
    /// Example: "cocaine", "drug"
    #[arg(long, default_value = "search_terms.csv")]
    pub terms_file: PathBuf,

    /// The dataset file.
    /// This file is the one we will be searching.
    /// It has no requirements other than being correctly
    /// formatted csv.
    #[arg(long)]
    pub data_file: PathBuf,

    /// The column in the dataset file that contains the text
    /// we will be searching.
    /// This column must be a string column.
    #[arg(long)]
    pub search_col: String,

    /// The column in the dataset file that contains the identifier
    /// for each row to re-join later to the original.
    #[arg(long)]
    pub id_col: Option<String>,

    /// threshold
    /// This is the threshold for the similarity score
    /// between the search term and the text in the dataset.
    /// The default is 0.95 which is quite strong.
    /// If you want to be more lenient you can lower this.
    ///
    /// The similarity score is a number between 0 and 1.
    /// 0 means no similarity and 1 means perfect similarity.
    ///
    /// This score will be returned in the output file so you can
    /// filter on it later.
    #[arg(long, default_value_t = 0.95)]
    pub threshold: f64,
}

#[derive(Parser, Debug)]
struct PipeCli {
    #[arg(short = 'f', long, default_value = "search_terms.csv")]
    terms_file: PathBuf,

    #[arg(short, long, default_value_t = 0.95)]
    pub threshold: f64,
}

#[derive(Deserialize, Debug)]
struct SearchTerm {
    word: String,
    metadata: Option<String>,
}

#[derive(Debug, Serialize)]
struct StandardOutput {
    #[serde(rename = "Similarity Score")]
    sim: f64,
    #[serde(rename = "Search Term")]
    target: String,
    #[serde(rename = "Matched Term")]
    match_: String,
    #[serde(rename = "Metadata")]
    metadata: Option<String>,
}

#[derive(Debug, Serialize)]
struct IdentifiedOutput {
    #[serde(rename = "Row ID")]
    row_id: String,
    #[serde(rename = "Similarity Score")]
    sim: f64,
    #[serde(rename = "Search Term")]
    target: String,
    #[serde(rename = "Matched Term")]
    match_: String,
    #[serde(rename = "Metadata")]
    metadata: Option<String>,
}

fn find_column_index(header: &csv::StringRecord, col_name: &str) -> usize {
    let upper_col = col_name.to_uppercase();
    match header.iter().position(|c| c.to_uppercase() == upper_col) {
        Some(i) => i,
        None => {
            println!("Could not find column named {}", col_name);
            println!("Available columns: {:?}", header);
            println!("Please check the spelling of the column name and try again.");
            std::process::exit(1);
        }
    }
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
    ss.to_ascii_uppercase()
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

fn read_terms_only(reader: &mut csv::Reader<File>) -> Vec<SearchTerm> {
    reader
        .records()
        .into_iter()
        .map(|result| match result {
            Ok(row) => SearchTerm {
                word: row
                    .get(0)
                    .expect("No data found in column 1")
                    .to_uppercase(),
                metadata: None,
            },
            Err(e) => {
                println!("Error reading row: {}", e);
                std::process::exit(1);
            }
        })
        .collect()
}

fn read_terms_with_tags(reader: &mut csv::Reader<File>) -> Vec<SearchTerm> {
    reader
        .records()
        .into_iter()
        .map(|result| match result {
            Ok(row) => SearchTerm {
                word: row
                    .get(0)
                    .expect("No data found in column 1")
                    .to_uppercase(),
                metadata: Some(
                    row.get(1)
                        .expect("No data found in column 2")
                        .to_uppercase(),
                ),
            },
            Err(e) => {
                println!("Error reading row: {}", e);
                std::process::exit(1);
            }
        })
        .collect()
}

fn load_search_terms(filename: &PathBuf) -> Result<Vec<SearchTerm>> {
    let fpath = env::current_dir()?.join(filename);
    let mut rdr = csv::Reader::from_path(fpath)?;

    let num_cols = validate_terms_headers(rdr.headers()?);
    let terms = match num_cols {
        1 => read_terms_only(&mut rdr),
        2 => read_terms_with_tags(&mut rdr),
        _ => {
            println!("Unexpected number of columns returned from validator. Expected 2 columns in the search term file.");
            println!("Please check the format of the search term file.");
            std::process::exit(1);
        }
    };
    Ok(terms)
}

fn load_dataset_words_only(cli: &StandardCli) -> Result<HashSet<String>> {
    let fpath = env::current_dir()?.join(&cli.data_file);
    let file = File::open(fpath)?;
    let mut rdr = csv::Reader::from_reader(file);

    let target_col_index = find_column_index(rdr.headers()?, &cli.search_col);

    let words: HashSet<String> = rdr
        .records()
        .into_iter()
        .flat_map(|result| match result {
            Ok(row) => {
                let text = row
                    .get(target_col_index)
                    .expect("No data found in column 1");
                // clean text, uppercase
                let clean_text = remove_symbols_except_dash(text);
                clean_text
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect::<HashSet<String>>()
            }
            Err(e) => {
                println!("Error reading row: {}", e);
                std::process::exit(1);
            }
        })
        .unique()
        .collect();

    Ok(words)
}

fn load_dataset_words_identified(cli: &StandardCli) -> Result<HashMap<String, HashSet<String>>> {
    let fpath = env::current_dir()?.join(&cli.data_file);
    let file = File::open(fpath)?;
    let mut rdr = csv::Reader::from_reader(file);

    // safe to unwrap because calling function validates `cli.id_col` is Some
    // some weird clone/unwrap here?
    let id_col_index = find_column_index(rdr.headers()?, &cli.id_col.clone().unwrap());
    let target_col_index = find_column_index(rdr.headers()?, &cli.search_col);

    // now here we need to build a map of words to IDs
    let mut words_map: HashMap<String, HashSet<String>> = HashMap::new();
    rdr.records().into_iter().for_each(|result| match result {
        Ok(row) => {
            let id = row.get(id_col_index).expect("No data found in ID column");
            let text = row
                .get(target_col_index)
                .expect("No data found in column 1");
            // clean text, uppercase
            let clean_text = remove_symbols_except_dash(text);
            clean_text.split_whitespace().into_iter().for_each(|w| {
                let word = w.to_string();
                let map_entry = words_map.entry(word).or_insert(HashSet::new());
                map_entry.insert(id.to_string());
            })
        }
        Err(e) => {
            println!("Error reading row: {}", e);
            std::process::exit(1);
        }
    });

    Ok(words_map)
}

fn read_stdin_to_string() -> String {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    buffer
}

fn run_pipe_program(args: &PipeCli) -> Result<()> {
    if atty::is(atty::Stream::Stdin) {
        println!("No data found on standard input. Please pipe data to this program.");
        println!("For example: `cat datafile.txt | extract-drugs pipe");
        println!("Alternatively, you can use the `standard` subcommand to read from a file.");
        std::process::exit(1);
    } else {
        // we have data on standard input
        // but we load the search terms before reading standard input
        // this helps with debugging in configuration problems in
        // search terms file and/or CLI arguments before reading all of standard input
        let search_terms = load_search_terms(&args.terms_file)?;

        let std_input = read_stdin_to_string();
        let clean_text = remove_symbols_except_dash(std_input.as_str());
        let words = clean_text.split_whitespace().collect::<HashSet<&str>>();

        // around here we will want to create our progress bar

        // print the header
        println!("Search Term,Matched Term,Similarity");
        search_terms
            .iter()
            // comparable to nested for loop
            .cartesian_product(words.iter())
            .for_each(|(term, word)| {
                let sim = jaro_winkler(&term.word, word);
                if sim > args.threshold {
                    println!("{},{},{}", term.word, word, sim);
                }
            });
    }
    Ok(())
}

fn run_standard_program(args: &StandardCli) -> Result<()> {
    let search_terms = load_search_terms(&args.terms_file)?;

    match args.id_col {
        Some(_) => {
            println!("ID column provided.");
            println!("Loading dataset words and IDs.");
            let identified_words = load_dataset_words_identified(args)?;

            let mut wtr =
                csv::Writer::from_path(env::current_dir()?.join("identified_output.csv"))?;

            // pb starts here
            search_terms
                .iter()
                // comparable to nested for loop
                .cartesian_product(identified_words.keys())
                .for_each(|(term, word)| {
                    let sim = jaro_winkler(&term.word, word);
                    if sim > args.threshold {
                        // identified output
                        // make multiple times for each ID
                        identified_words
                            .get(word)
                            .unwrap() // ok because comes from `keys()`
                            .iter()
                            .for_each(|id| {
                                let output = IdentifiedOutput {
                                    row_id: id.to_string(),
                                    target: term.word.to_owned(),
                                    match_: word.to_string(),
                                    sim,
                                    metadata: term.metadata.to_owned(),
                                };
                                wtr.serialize(output).unwrap();
                            });
                    }
                });
            Ok(())
        }
        None => {
            println!("No ID column provided.");
            println!("Loading dataset words only.");
            let words = load_dataset_words_only(args)?;

            let mut wtr = csv::Writer::from_path(env::current_dir()?.join("standard_ output.csv"))?;

            // pb starts here
            search_terms
                .iter()
                // comparable to nested for loop
                .cartesian_product(&words)
                .for_each(|(term, word)| {
                    let sim = jaro_winkler(&term.word, word);
                    if sim > args.threshold {
                        // standard output
                        let output = StandardOutput {
                            target: term.word.to_owned(),
                            match_: word.to_string(),
                            sim,
                            metadata: term.metadata.to_owned(),
                        };
                        wtr.serialize(output).unwrap();
                    }
                });

            Ok(())
        }
    }
}

fn run_interactive_program() -> Result<()> {
    todo!("interactive mode")
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    dbg!(&cli);

    match cli.command {
        Commands::Interactive => run_interactive_program()?,
        Commands::Pipe(args) => run_pipe_program(&args)?,
        Commands::Standard(args) => run_standard_program(&args)?,
    }

    Ok(())
}

// tests module
#[cfg(test)]
mod tests {
    use itertools::Itertools;

    #[test]
    fn test_double_iter() {
        let a = vec![1, 2, 3, 4, 5];
        let b = vec![1, 2, 3, 4, 5, 7, 8];
        // for aa in &a {
        //     for bb in &b {
        //         println!("{}, {}", aa, bb);
        //     }
        // }
        let c = a.iter().cartesian_product(b.iter());
        for (aa, bb) in c {
            println!("{}, {}", aa, bb);
        }
    }
}
