//!
//!

// TODO: use doctests for examples and testing
// TODO: document --> check rust book for pub/private functions

// TODO: document things REALLY well, we won't be re-publishing on crates.io

use color_eyre::{eyre::WrapErr, Report, Result};

use clap::{Parser, Subcommand};
use dialoguer::{theme::ColorfulTheme, Confirm, FuzzySelect, Input, MultiSelect, Select};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressIterator};
use itertools::Itertools;
use lib::IdentifiedOutput;
use rayon::prelude::*;
use serde::Serialize;
use std::{fs::File, path::PathBuf, str::FromStr};
use strsim::jaro_winkler;

mod io;
mod options;
mod runners;
mod search;
mod types;

#[derive(Parser, Debug)]
#[command(
    author,
    about,
    version,
    long_about = "A fuzzy search tool for extracting data from large datasets."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Standard(CLIStandardOptions),
    Pipe(PipeOptions),
    #[command(about = "Interactive Wizard")]
    Interactive,
}

#[derive(Parser, Debug)]
#[command(about = "File based IO")]
pub struct CLIStandardOptions {
    /// The file with your search terms
    #[arg(short = 'f', long, default_value = "search_terms.csv")]
    pub terms_file: PathBuf,

    /// The dataset file to search
    #[arg(short, long)]
    pub data_file: PathBuf,

    /// The column name(s) in the dataset to search
    #[arg(short, long, num_args = 1)]
    pub search_cols: Vec<String>,

    /// The column name in the dataset to keep as identifier
    #[arg(short, long)]
    pub id_col: Option<String>,

    /// Minimum similarity for match (0.0 - 1.0)
    #[arg(short, long, default_value_t = 0.95)]
    pub threshold: f64,

    /// Output file type, Options: csv, jsonl
    #[arg(short, long, default_value = "csv")]
    pub output_type: OutputFileType,
}

#[derive(Parser, Debug)]
#[command(about = "Pipe based IO")]
struct PipeOptions {
    /// File with your search terms
    #[arg(short = 'f', long, default_value = "search_terms.csv")]
    terms_file: PathBuf,

    /// Minimum similarity for match (0.0 - 1.0)
    #[arg(short, long, default_value_t = 0.95)]
    threshold: f64,
}

#[derive(Parser, Debug)]
#[command(about = "Interactive Wizard")]
struct InteractiveOptions {}

fn greet(std_err: bool) {
    match std_err {
        true => {
            eprintln!();
            eprintln!("Welcome to the UK IPOP Fuzzy Drug Searcher!");
            eprintln!("===========================================");
            eprintln!();
            eprintln!("This program will search a datafile for matches to a list of terms. For more information, please consult the User Guide: https://github.com/UK-IPOP/drug-extraction or the `--help` menu.");
        }
        false => {
            println!();
            println!("Welcome to the UK IPOP Fuzzy Drug Searcher!");
            println!("===========================================");
            println!();
            println!("This program will search a datafile for matches to a list of terms. For more information, please consult the User Guide: https://github.com/UK-IPOP/drug-extraction or the `--help` menu.");
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum OutputFileType {
    #[default]
    Csv,
    Jsonl,
}

impl FromStr for OutputFileType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "csv" => Ok(OutputFileType::Csv),
            "jsonl" => Ok(OutputFileType::Jsonl),
            _ => Err(format!("{} is not a valid output file type", s)),
        }
    }
}

fn write_csv<O: Serialize>(output: &[O]) -> Result<(), Report> {
    let mut wtr =
        csv::Writer::from_path("output.csv").wrap_err("Unable to create CSV output file.")?;
    for row in output
        .iter()
        .progress_with(ProgressBar::new_spinner().with_message("Writing CSV..."))
    {
        wtr.serialize(row).wrap_err("Unable to serialize output")?;
    }
    wtr.flush().wrap_err("Unable to flush output")?;
    Ok(())
}

fn write_jsonl<O: Serialize>(output: &[O]) -> Result<(), Report> {
    use std::io::{BufWriter, Write};

    let mut wtr = BufWriter::new(File::create("output.jsonl")?);

    for row in output
        .iter()
        .progress_with(ProgressBar::new_spinner().with_message("Writing JSONL..."))
    {
        let json = serde_json::to_string(row).wrap_err("Unable to serialize output")?;
        wtr.write_all(json.as_bytes())
            .wrap_err("Unable to write serialized json bytes to file")?;
        wtr.write_all(b"\n")
            .wrap_err("Unable to write newline to file")?;
    }
    Ok(())
}

fn write_output<O: Serialize>(output: &[O], file_type: &OutputFileType) -> Result<(), Report> {
    match file_type {
        OutputFileType::Csv => write_csv(output),
        OutputFileType::Jsonl => write_jsonl(output),
    }
}

pub fn run_standard_program(args: &CLIStandardOptions) -> Result<(), Report> {
    #[cached(
        type = "SizedCache<String, f64>",
        create = "{ SizedCache::with_size(1_000) }",
        convert = r#"{ format!("{}{}", a, b) }"#
    )]
    fn jw(a: &str, b: &str) -> f64 {
        strsim::jaro_winkler(a, b)
    }
    // its seeming like we are leaning to building a "source data" type
    // similar to previous input types
    // this doesn't hold our options, but the actual data we pulled from the
    // data file
    let data_path = PathBuf::from(&args.data_file);
    let mut rdr = lib::initialize_dataset_reader(&data_path).unwrap();
    let header = &rdr.headers().unwrap().clone();
    let terms_path = PathBuf::from(&args.terms_file);
    let search_terms = lib::load_search_terms(&terms_path).unwrap();
    let target_col_indices = &args
        .search_cols
        .iter()
        .map(|c| lib::find_column_index(header, c))
        .collect_vec();
    let mut grid_data: Vec<IdentifiedOutput> = Vec::new();

    let spinner = ProgressBar::new_spinner();
    rdr.records()
        .enumerate()
        .into_iter()
        .progress_with(spinner)
        .for_each(|(i, row)| {
            if i % 1_000 == 0 {
                println!("{}", i);
            }
            let result = row.unwrap();
            let id_val = if let Some(id_) = &args.id_col {
                let id_col_index = lib::find_column_index(header, id_);
                result.get(id_col_index).unwrap().to_string()
            } else {
                i.to_string()
            };
            for term in &search_terms {
                let gram_length = term.word.split_whitespace().count();
                // J is column index so we can re-index into target cols form args
                for (j, c) in target_col_indices.iter().enumerate() {
                    let cell = result.get(*c).unwrap();
                    let clean = lib::remove_symbols_except_dash(cell);
                    let words: Vec<String> = clean
                        .split_whitespace()
                        .map(|x| x.to_string())
                        .collect_vec();
                    let grams = if gram_length == 1 {
                        let r = words.iter().map(|x| x.to_owned()).unique().collect_vec();
                        r
                    } else {
                        let windows = words.windows(gram_length).collect_vec();
                        let r = windows.iter().map(|w| w.join(" ")).unique().collect_vec();
                        r
                    };
                    for gram in grams {
                        let sim = jw(&term.word, &gram);
                        if sim > 0.95 {
                            let ido = IdentifiedOutput {
                                row_id: id_val.clone(),
                                column: args.search_cols[j].to_owned(),
                                target: term.word.to_owned(),
                                metadata: term.metadata.to_owned(),
                                match_: gram.to_string(),
                                sim,
                            };
                            grid_data.push(ido);
                        }
                    }
                }
            }
        });

    write_output(&grid_data, &args.output_type);

    Ok(())
}

pub mod greeting {
    pub fn hi() {
        print!("hi")
    }
}

pub mod farewell {
    pub fn bye() {
        print!("bye")
    }
}

pub mod talk {
    use crate::greeting::hi;

    pub fn wow() {
        hi();
    }
}

/* fn cli_run_standard_program(args: &StandardOptions) -> Result<(), Report> { */
/*     let search_terms = lib::load_search_terms(&args.terms_file)?; */
/*     let pb_style = if let Ok(style) = lib::initialize_progress_with_style("bar") { */
/*         style */
/*     } else { */
/*         // this shouldn't happen but if it does, we'll just use the default spinner */
/*         ProgressBar::new_spinner().style() */
/*     }; */
/*     match &args.id_col { */
/*         Some(id_col) => { */
/*             let identified_words = */
/*                 lib::load_dataset_identified(&args.data_file, &args.search_cols, id_col)?; */
/*             let matches = lib::find_matches( */
/*                 &search_terms, */
/*                 identified_words.keys(), */
/*                 args.threshold, */
/*                 &pb_style, */
/*             ); */
/*             let outputs = lib::assemble_identified_output(&matches, &identified_words); */
/*             write_output(&outputs, &args.output_type)?; */
/*             Ok(()) */
/*         } */
/*         None => { */
/*             if args.search_cols.len() > 1 { */
/*                 println!( */
/*                     "Warning: You have specified more than one search column, but no ID column." */
/*                 ); */
/*                 println!("This means that the output will not be able to be linked back to the original dataset."); */
/*                 let resume = Confirm::with_theme(&ColorfulTheme::default()) */
/*                     .with_prompt("Continue?") */
/*                     .default(false) */
/*                     .interact() */
/*                     .wrap_err("Unable to confirm")?; */
/*                 if !resume { */
/*                     eprintln!("Stopping..."); */
/*                     std::process::exit(1); */
/*                 } */
/*             }; */
/*             let words = lib::load_dataset_words_only(&args.data_file, args.search_cols.as_slice())?; */
/*             let matches = lib::find_matches(&search_terms, words.iter(), args.threshold, &pb_style); */
/*             let outputs = lib::assemble_standard_output(&matches, &pb_style); */
/*             write_output(&outputs, &args.output_type)?; */
/*             Ok(()) */
/*         } */
/*     } */
/* } */

fn run_pipe_program(args: &PipeOptions) -> Result<(), Report> {
    // greet on std err to not interfere with piping
    greet(true);

    if atty::is(atty::Stream::Stdin) {
        println!("No data found on standard input. Please pipe data to this program.");
        println!("For example: `cat datafile.txt | extract-drugs pipe");
        println!("Alternatively, you can use the `standard` subcommand to read from a file.");
        std::process::exit(1);
    } else {
        // we have data on standard input
        // but we load the search terms before reading standard input
        // this helps with debugging configuration problems in
        // search terms file and/or CLI arguments before reading all of standard input
        // which could be very large
        let search_terms = lib::load_search_terms(&args.terms_file)?;
        let words = lib::load_stdin_words()?;
        // this needs to be checked and if an error, print and loop without
        // progress style
        let pb_style = lib::initialize_progress_with_style("spinner");
        match pb_style.is_err() {
            false => {
                // progress stuff
                // print the header
                println!("Search Term,Matched Term,Similarity");
                search_terms
                    .iter()
                    .cartesian_product(words.iter())
                    .collect_vec()
                    .par_iter()
                    .progress_with_style(pb_style.unwrap()) // safe unwrap
                    .for_each(|(term, word)| {
                        let sim = jaro_winkler(&term.word, word);
                        if sim > args.threshold {
                            println!("{},{},{}", term.word, word, sim);
                        }
                    });
            }
            true => {
                // no progress looping
                eprintln!("Unable to initialize progress bar. Continuing without progress bar.");
                // print the header
                println!("Search Term,Matched Term,Similarity");
                search_terms
                    .iter()
                    .cartesian_product(words.iter())
                    .collect_vec()
                    .par_iter()
                    .for_each(|(term, word)| {
                        let sim = jaro_winkler(&term.word, word);
                        if sim > args.threshold {
                            println!("{},{},{}", term.word, word, sim);
                        }
                    });
            }
        }
    }
    Ok(())
}

fn interactive_wizard() -> Result<(), Report> {
    greet(false);

    let theme = ColorfulTheme::default();

    let terms_file: PathBuf = Input::<String>::with_theme(&theme)
        .with_prompt("What is the path to the search terms file?")
        .default("search_terms.csv".to_string())
        .interact_text()?
        .into();

    let data_file: PathBuf = Input::<String>::with_theme(&theme)
        .with_prompt("What is the path to the data file?")
        .interact_text()?
        .into();

    let headers = lib::read_headers(&data_file)?;

    let search_cols = MultiSelect::with_theme(&theme)
        .with_prompt("Which column(s) do you want to search? (multi-select with Space)")
        .items(&headers)
        .interact()?;

    if search_cols.is_empty() {
        println!("You must select at least one column to search.");
        println!("Use the arrow keys to select the columns you want to search.");
        println!("Press `Space` to select and unselect columns and `Enter` to continue.");
        std::process::exit(1);
    }
    let search_cols = search_cols
        .iter()
        .map(|&x| headers[x].to_string())
        .collect::<Vec<String>>();

    let has_id_col = Confirm::with_theme(&theme)
        .with_prompt("Do you want to use an ID column?")
        .default(false)
        .interact()?;

    let id_col = if has_id_col {
        let id_col_index = FuzzySelect::with_theme(&theme)
            .with_prompt("Which column do you want to use as the ID column?")
            .items(&headers)
            .interact()?;
        Some(&headers[id_col_index])
    } else {
        None
    };

    let threshold = Input::<f64>::with_theme(&theme)
        .with_prompt("What is the threshold for matches?")
        .default(0.95)
        .interact()?;

    let output_type = Select::with_theme(&theme)
        .with_prompt("What type of output do you want?")
        .items(&["CSV", "JSONL"])
        .default(0)
        .interact()?;
    let output_type = OutputFileType::from_str(["csv", "jsonl"][output_type]).unwrap();

    let args = CLIStandardOptions {
        terms_file,
        data_file,
        id_col: id_col.cloned(),
        search_cols,
        threshold,
        output_type,
    };

    run_standard_program(&args)?;

    Ok(())
}

fn main() -> Result<(), Report> {
    color_eyre::install()?;

    let cli = Cli::parse();

    match cli.command {
        Commands::Standard(args) => run_standard_program(&args)?,
        Commands::Pipe(args) => run_pipe_program(&args)?,
        Commands::Interactive => interactive_wizard()?,
    }

    Ok(())
}

// tests module
#[cfg(test)]
mod tests {
    use std::{fmt::Display, path::Path};

    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, 1);
    }

    #[test]
    fn test2() {
        let x = "jsonl";
        assert_eq!(OutputFileType::from_str(x).unwrap(), OutputFileType::Jsonl);
    }

    fn read_file<P: AsRef<Path> + Display>(x: P) {
        println!("read {}", x);
        let _ = csv::Reader::from_path(&x);
    }
    #[test]
    fn test_into_path() {
        let x = "./data/records.csv";
        read_file(x);
    }
}
