//tutorial-read-01.rs
use clap::{Parser, Subcommand};
use csv::StringRecord;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Confirm, Input, Select};
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::error::Error;
use std::ffi::OsString;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::io::{LineWriter, Write};
use std::process::{self, exit};
use std::str::FromStr;
use std::{env, path::Path};
use walkdir::WalkDir;

use extract_drugs_core as drug_core;
use extract_drugs_core::SearchInput;

#[derive(Parser)]
#[clap(args_override_self = true)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Interact,

    #[clap(arg_required_else_help = true)]
    Run {
        file: String,

        #[clap(long)]
        id_column: Option<String>,

        #[clap(long)]
        target_column: String,

        #[clap(long)]
        search_words: String,

        #[clap(long)]
        algorithm: String,

        #[clap(long)]
        max_edits: Option<i32>,

        #[clap(long)]
        threshold: Option<f64>,

        #[clap(long)]
        format: drug_core::OutputFormat,
    },
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    match args.command {
        Commands::Interact => {
            println!("Welcome to the UK-IPOP Drug Extraction tool.");
            println!("------------------------------------------");
            let file_path: String = Input::new()
                .with_prompt("Please enter the path to your data file:")
                .interact_text()
                .unwrap();
            let mut columns = String::new();
            BufReader::new(File::open(&file_path).unwrap())
                .read_line(&mut columns)
                .unwrap();
            let column_choices = columns.split(',').collect::<Vec<&str>>();
            let id_col_select = Select::with_theme(&ColorfulTheme::default())
                .items(&column_choices)
                .with_prompt("Please select your ID column or press `Esc` or `q` to move on with NO ID specified:")
                .default(0)
                .interact_opt()
                .unwrap();
            let has_id = id_col_select.is_some();
            let user_id_col = column_choices[id_col_select.unwrap()].to_string();
            let target_col_select = Select::with_theme(&ColorfulTheme::default())
                .items(&column_choices)
                .with_prompt("Please select your target text column (required):")
                .default(0)
                .interact()
                .unwrap();
            let target_col = column_choices[target_col_select].to_string();
            let algorithm_options = drug_core::Algorithm::options();
            let user_algo = Select::with_theme(&ColorfulTheme::default())
                .items(&algorithm_options)
                .with_prompt("Please select your algorithm:")
                .default(0)
                .interact()
                .unwrap();
            let algorithm = drug_core::Algorithm::from_str(&algorithm_options[user_algo]).unwrap();
            let max_edits: i32 = Input::new()
                .with_prompt("Please enter the maximum number of edits you would like to allow or press `Esc` or `q` to move on:")
                .interact_text()
                .unwrap();
            let thresh: f64 = Input::new()
                .with_prompt(
                    "Please enter the minimum similarity threshold you would like to filter to:",
                )
                .interact_text()
                .unwrap();
            let user_search_words: String = Input::new()
                .with_prompt("Please enter your search words separated by a `|` symbol:")
                .interact_text()
                .unwrap();
            let search_words = user_search_words
                .split('|')
                .map(|x| x.to_uppercase())
                .collect::<Vec<String>>();
            let format_options = vec!["JSONL", "CSV"];
            let user_format_choice = Select::with_theme(&ColorfulTheme::default())
                .items(&format_options)
                .with_prompt("Please enter the output format you would prefer:")
                .default(0)
                .interact()
                .unwrap();
            if !Confirm::new().with_prompt("Start?").interact().unwrap() {
                exit(1)
            }
            let user_format =
                drug_core::OutputFormat::from_str(format_options[user_format_choice]).unwrap();
            let distance = drug_core::initialize_distance(algorithm);

            let file = File::open(&file_path)?;
            let mut rdr = csv::Reader::from_reader(file);
            // clones, could use scoped alternative to return header indices
            let headers = rdr.headers()?.clone();
            let target_col_index = get_header_index(&headers, target_col).unwrap();
            let id_col_index = if has_id {
                Some(get_header_index(&headers, user_id_col).unwrap())
            } else {
                None
            };
            println!("{:?}", headers);

            let mut out_file = match user_format {
                drug_core::OutputFormat::JSONL => {
                    fs::File::create("extracted_drugs.jsonl").unwrap()
                }
                drug_core::OutputFormat::CSV => {
                    let headers =
                        "record_id,search_term,matched_term,algorithm,edits,similarity".to_string();
                    let mut f = fs::File::create("extracted_drugs.csv").unwrap();
                    f.write_all(headers.as_bytes());
                    f.write(b"\n");
                    f
                }
            };

            let line_count = BufReader::new(File::open(&file_path).unwrap())
                .lines()
                .count();
            let bar = initialize_progress(line_count as u64);
            for result in rdr.records() {
                let record = result?;
                if record.is_empty() {
                    continue;
                }
                let record_id = if has_id {
                    Some(record.get(id_col_index.unwrap()).unwrap().to_string())
                } else {
                    None
                };

                let text = record.get(target_col_index).unwrap();
                if text.is_empty() {
                    continue;
                }
                let searcher = drug_core::SimpleInput::new(
                    algorithm,
                    distance,
                    Some(max_edits),
                    Some(thresh),
                    search_words.as_slice(),
                );
                let res = searcher.scan(text, record_id);
                let output_list = drug_core::format(res, user_format);
                let mut output = output_list.iter().peekable();
                while let Some(out) = output.next() {
                    // check for last item
                    if output.peek().is_some() {
                        out_file.write_all(out.as_bytes()).unwrap();
                        out_file.write(b"\n");
                    }
                }
                bar.inc(1);
            }
            bar.finish();
        }
        Commands::Run {
            file,
            id_column,
            target_column,
            search_words,
            algorithm,
            max_edits,
            threshold,
            format,
        } => {
            let file_path = file;
            let target_col = target_column;
            let user_id_col = id_column;
            let has_id = user_id_col.is_some();
            let search_words = search_words
                .split('|')
                .map(|x| x.to_uppercase())
                .collect::<Vec<String>>();
            let user_algo = algorithm;
            let max_edits = max_edits;
            let thresh = threshold;
            let user_format = format;
            let file = File::open(&file_path)?;
            let mut rdr = csv::Reader::from_reader(file);

            let algorithm = drug_core::Algorithm::from_str(&user_algo).unwrap();
            let distance = drug_core::initialize_distance(algorithm);

            // clones, could use scoped alternative to return header indices
            let headers = rdr.headers()?.clone();
            let target_col_index = get_header_index(&headers, target_col).unwrap();
            let id_col_index = if has_id {
                Some(get_header_index(&headers, user_id_col.unwrap()).unwrap())
            } else {
                None
            };
            println!("{:?}", headers);

            let mut out_file = match user_format {
                drug_core::OutputFormat::JSONL => {
                    fs::File::create("extracted_drugs.jsonl").unwrap()
                }
                drug_core::OutputFormat::CSV => {
                    let headers =
                        "record_id,search_term,matched_term,algorithm,edits,similarity".to_string();
                    let mut f = fs::File::create("extracted_drugs.csv").unwrap();
                    f.write_all(headers.as_bytes());
                    f.write(b"\n");
                    f
                }
            };

            let line_count = BufReader::new(File::open(&file_path).unwrap())
                .lines()
                .count();
            let bar = initialize_progress(line_count as u64);
            for result in rdr.records() {
                let record = result?;
                if record.is_empty() {
                    continue;
                }
                let record_id = if has_id {
                    Some(record.get(id_col_index.unwrap()).unwrap().to_string())
                } else {
                    None
                };

                let text = record.get(target_col_index).unwrap();
                if text.is_empty() {
                    continue;
                }
                let searcher = drug_core::SimpleInput::new(
                    algorithm,
                    distance,
                    max_edits,
                    thresh,
                    search_words.as_slice(),
                );
                let res = searcher.scan(text, record_id);
                let output_list = drug_core::format(res, user_format);
                let mut output = output_list.iter().peekable();
                while let Some(out) = output.next() {
                    // check for last item
                    if output.peek().is_some() {
                        out_file.write_all(out.as_bytes()).unwrap();
                        out_file.write(b"\n");
                    }
                }
                bar.inc(1);
            }
            bar.finish();
        }
    }

    Ok(())
}

fn get_header_index(headers: &StringRecord, search: String) -> Option<usize> {
    let s = search.to_uppercase();
    for (i, h) in headers.iter().enumerate() {
        if h.to_ascii_uppercase() == s {
            return Some(i);
        }
    }
    None
}

fn initialize_progress(items: u64) -> ProgressBar {
    let pb = ProgressBar::new(items);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:.cyan/blue}] {pos}/{len} ({eta})")
            .progress_chars("#>-"),
    );
    pb
}

fn get_total_files(target_folder: &str) -> usize {
    WalkDir::new(target_folder).into_iter().count()
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
