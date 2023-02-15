//! Utility functions for the CLI.
//!
//! This module contains utility functions for the CLI.
//!
//!
use clap::{Parser, Subcommand};
use csv::StringRecord;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Confirm, Input, Select};
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::process::exit;
use std::str::FromStr;

use drug_extraction_core as drug_core;

/// CLI Application to extract drugs from a CSV file
#[derive(Parser)]
#[clap(args_override_self = true)]
#[clap(version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

/// Subcommands for the CLI.
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Interactive mode. Useful for first time users or one-offs.
    Interactive,

    /// Simple Execute mode. Useful for automation/scripts.
    #[clap(arg_required_else_help = true)]
    SimpleSearch {
        /// File to extract drugs from.
        file: String,

        /// ID column to retain in output for linking later. Optional.
        #[clap(long)]
        id_column: Option<String>,

        /// Target text column to search for drugs.
        #[clap(long)]
        target_column: String,

        /// Search words to search for.
        #[clap(long)]
        search_words: String,

        /// String Similarity Algorithm to use.
        #[clap(long)]
        algorithm: drug_core::Algorithm,

        /// Maximum number of edits to allow.
        /// Should be None for Algorithms that do not use edit distance.
        #[clap(long)]
        max_edits: Option<i32>,

        /// Minimum Similarity threshold to capture.
        #[clap(long)]
        threshold: Option<f64>,

        /// Output format (JSONL, CSV).
        #[clap(long)]
        format: drug_core::OutputFormat,

        /// Disable caching. Not recommended, but will decrease memory usage.
        #[clap(long)]
        no_cache: bool,
    },
    /// Drug-based Execute mode. Useful for automation/scripts.
    #[clap(arg_required_else_help = true)]
    DrugSearch {
        /// File to extract drugs from.
        file: String,

        /// ID column to retain in output for linking later. Optional.
        #[clap(long)]
        id_column: Option<String>,

        /// Target text column to search for drugs.
        #[clap(long)]
        target_column: String,

        /// RxClass ID. Used to get drugs belonging to this class from the RxClass API.
        #[clap(long)]
        rx_class_id: String,

        /// RxClass RelaSource value.
        #[clap(long)]
        rx_class_relasource: String,

        /// String Similarity Algorithm to use. Needs to align with the RxClass ID and API.
        #[clap(long)]
        algorithm: drug_core::Algorithm,

        /// Maximum number of edits to allow.
        /// Should be None for Algorithms that do not use edit distance.
        #[clap(long)]
        max_edits: Option<i32>,

        /// Minimum Similarity threshold to capture.
        #[clap(long)]
        threshold: Option<f64>,

        /// Output format (JSONL, CSV).
        #[clap(long)]
        format: drug_core::OutputFormat,

        /// Disable caching. Not recommended, but will decrease memory usage.
        #[clap(long)]
        no_cache: bool,
    },
}

pub fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
    match cli.command {
        Commands::Interactive => run_interactive()?,
        Commands::SimpleSearch {
            file,
            id_column,
            target_column,
            search_words,
            algorithm,
            max_edits,
            threshold,
            format,
            no_cache,
        } => {
            let ssi = SsInput {
                fpath: file,
                target_column,
                id_column,
                search_words,
                algorithm,
                max_edits,
                threshold,
                format,
                no_cache,
            };
            run_simple_searcher(ssi)?;
        }
        Commands::DrugSearch {
            file,
            id_column,
            target_column,
            rx_class_id,
            rx_class_relasource,
            algorithm,
            max_edits,
            threshold,
            format,
            no_cache,
        } => {
            let dsi = DsInput {
                fpath: file,
                target_column,
                id_column,
                rx_class_id,
                rx_class_relasource,
                algorithm,
                max_edits,
                threshold,
                format,
                no_cache,
            };
            run_drug_searcher(dsi)?;
        }
    }
    Ok(())
}

/// This function is used to validate the input parameters.
fn validate_options(max_edits: Option<i32>, threshold: Option<f64>) {
    if threshold.is_none() && max_edits.is_none() {
        println!("You have not specified a threshold or max edits. This will return ALL items in ALL records.");
        println!("Please specify a threshold or max edits.");
        println!("Exiting...");
        exit(0);
    }
    if threshold.is_some() && max_edits.is_some() {
        println!("You have specified both a threshold and max edits. Max edits takes precedence.");
        let confirmed = Confirm::new()
            .with_prompt("Are you sure you want to continue?")
            .interact();
        if confirmed.is_err() && !confirmed.unwrap() {
            exit(0);
        } else {
            println!("Continuing...");
        }
    }
    if max_edits.is_some() {
        if let Some(me) = max_edits {
            if !(0..=5).contains(&me) {
                println!("Max edits must be between 0 and 5");
                exit(0);
            }
        }
    }
    if threshold.is_some() {
        if let Some(th) = threshold {
            if !(0.0..=1.0).contains(&th) {
                println!("Threshold must be between 0.0 and 1.0");
                exit(0);
            }
        }
    }
}

fn run_interactive() -> Result<(), Box<dyn Error>> {
    println!("Welcome to the UK-IPOP Drug Extraction tool.");
    println!("------------------------------------------");
    println!("This tool will extract drugs from a text file.");

    let search_types = vec!["Simple Search", "Drug Search"];
    let search_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select the type of search you want to perform")
        .items(&search_types)
        .interact()?;
    let chosen_search = search_types[search_type];
    match chosen_search {
        "Simple Search" => interactive_simple_search()?,
        "Drug Search" => interactive_drug_search()?,
        _ => panic!("Unexpected search type"),
    }
    Ok(())
}

fn interactive_simple_search() -> Result<(), Box<dyn Error>> {
    let file_path: String = Input::new()
        .with_prompt("Please enter the path to your data file")
        .interact_text()?;
    let mut columns = String::new();
    BufReader::new(File::open(&file_path)?).read_line(&mut columns)?;
    let column_choices = columns.split(',').collect::<Vec<&str>>();
    let id_col_select = Select::with_theme(&ColorfulTheme::default())
        .items(&column_choices)
        .with_prompt(
            "Please select your ID column or press `Esc` or `q` to move on with NO ID specified",
        )
        .default(0)
        .interact_opt()?;
    let user_id_col = id_col_select.map(|id_sel| column_choices[id_sel].to_string());
    let target_col_select = Select::with_theme(&ColorfulTheme::default())
        .items(&column_choices)
        .with_prompt("Please select your target text column (required)")
        .default(0)
        .interact()?;
    let target_col = column_choices[target_col_select].to_string();
    let algorithm_options = drug_core::Algorithm::options();
    let user_algo = Select::with_theme(&ColorfulTheme::default())
        .items(algorithm_options)
        .with_prompt("Please select your algorithm")
        .default(0)
        .interact()?;
    let algorithm = drug_core::Algorithm::from_str(algorithm_options[user_algo])?;
    let max_edits = if algorithm.is_edits() {
        let max_edits_select = Select::with_theme(&ColorfulTheme::default())
            .items(&["0", "1", "2", "3", "4", "5"])
            .with_prompt("Please select the maximum number of edits allowed")
            .default(0)
            .interact()?;
        Some(max_edits_select as i32)
    } else {
        None
    };
    let threshold = if !algorithm.is_edits() {
        let threshold_select: f64 = Input::new()
            .with_prompt("Please enter the threshold (0.0 - 1.0)")
            .interact()?;
        Some(threshold_select)
    } else {
        None
    };
    let format_options = vec!["JSONL", "CSV"];
    let user_format_choice = Select::with_theme(&ColorfulTheme::default())
        .items(&format_options)
        .with_prompt("Please enter the output format you would prefer")
        .default(0)
        .interact()?;
    let user_search_words: String = Input::new()
        .with_prompt("Please enter your search words separated by a `|` symbol")
        .interact()?;

    let user_format = drug_core::OutputFormat::from_str(format_options[user_format_choice])?;
    let cache_choice = Confirm::new()
        .with_prompt("Would you like to use caching?")
        .interact()?;

    let ssi = SsInput {
        fpath: file_path,
        target_column: target_col,
        id_column: user_id_col,
        search_words: user_search_words,
        algorithm,
        max_edits,
        threshold,
        format: user_format,
        no_cache: !cache_choice,
    };
    run_simple_searcher(ssi)?;
    Ok(())
}

fn interactive_drug_search() -> Result<(), Box<dyn Error>> {
    let file_path: String = Input::new()
        .with_prompt("Please enter the path to your data file")
        .interact_text()?;
    let mut columns = String::new();
    BufReader::new(File::open(&file_path)?).read_line(&mut columns)?;
    let column_choices = columns.split(',').collect::<Vec<&str>>();
    let id_col_select = Select::with_theme(&ColorfulTheme::default())
        .items(&column_choices)
        .with_prompt(
            "Please select your ID column or press `Esc` or `q` to move on with NO ID specified",
        )
        .default(0)
        .interact_opt()?;
    let user_id_col = id_col_select.map(|id_sel| column_choices[id_sel].to_string());
    let target_col_select = Select::with_theme(&ColorfulTheme::default())
        .items(&column_choices)
        .with_prompt("Please select your target text column (required)")
        .default(0)
        .interact()?;
    let target_col = column_choices[target_col_select].to_string();
    let algorithm_options = drug_core::Algorithm::options();
    let user_algo = Select::with_theme(&ColorfulTheme::default())
        .items(algorithm_options)
        .with_prompt("Please select your algorithm")
        .default(0)
        .interact()?;
    let algorithm = drug_core::Algorithm::from_str(algorithm_options[user_algo])?;
    let max_edits = if algorithm.is_edits() {
        let max_edits_select = Select::with_theme(&ColorfulTheme::default())
            .items(&["0", "1", "2", "3", "4", "5"])
            .with_prompt("Please select the maximum number of edits allowed")
            .default(0)
            .interact()?;
        Some(max_edits_select as i32)
    } else {
        None
    };
    let threshold = if !algorithm.is_edits() {
        let threshold_select: f64 = Input::new()
            .with_prompt("Please enter the threshold (0.0 - 1.0)")
            .interact()?;
        Some(threshold_select)
    } else {
        None
    };
    let user_rx_id = Input::new()
        .with_prompt("Please enter your target RxNorm ID")
        .interact_text()?;
    let user_rx_relasource = Input::new()
        .with_prompt("Please enter your RxNorm relationship source")
        .interact_text()?;
    let format_options = vec!["JSONL", "CSV"];
    let user_format_choice = Select::with_theme(&ColorfulTheme::default())
        .items(&format_options)
        .with_prompt("Please enter the output format you would prefer")
        .default(0)
        .interact()?;

    let user_format = drug_core::OutputFormat::from_str(format_options[user_format_choice])?;
    let cache_choice = Confirm::new()
        .with_prompt("Would you like to use caching?")
        .interact()?;

    let dsi = DsInput {
        fpath: file_path,
        target_column: target_col,
        id_column: user_id_col,
        rx_class_id: user_rx_id,
        rx_class_relasource: user_rx_relasource,
        algorithm,
        max_edits,
        threshold,
        format: user_format,
        no_cache: !cache_choice,
    };
    run_drug_searcher(dsi)?;
    Ok(())
}

struct SsInput {
    fpath: String,
    target_column: String,
    id_column: Option<String>,
    search_words: String,
    algorithm: drug_core::Algorithm,
    max_edits: Option<i32>,
    threshold: Option<f64>,
    format: drug_core::OutputFormat,
    no_cache: bool,
}

fn run_simple_searcher(ssi: SsInput) -> Result<(), Box<dyn Error>> {
    validate_options(ssi.max_edits, ssi.threshold);

    println!("Welcome to the UK-IPOP Drug Extraction tool.");
    println!("------------------------------------------");
    let mut state = if !ssi.no_cache {
        println!("Establishing cache...");
        let map: HashMap<(String, String), f64> = HashMap::new();
        Some(map)
    } else {
        println!("No cache will be used");
        None
    };
    println!("Extracting targets from {}", ssi.fpath);

    let search_words = ssi
        .search_words
        .split('|')
        .map(|x| x.to_uppercase())
        .collect::<Vec<String>>();
    let file = File::open(&ssi.fpath)?;
    let mut rdr = csv::Reader::from_reader(file);
    let headers = rdr.headers()?.clone();

    let distance = drug_core::initialize_distance(ssi.algorithm);
    let target_col_index =
        get_header_index(&headers, ssi.target_column).expect("could not find target column");
    let has_id = ssi.id_column.is_some();
    let id_col_index = if has_id {
        Some(
            get_header_index(&headers, ssi.id_column.expect("no id column found"))
                .expect("header index inaccessible"),
        )
    } else {
        None
    };
    let mut out_file = initialize_output_file(ssi.format, false)?;

    let line_count = csv::Reader::from_reader(File::open(&ssi.fpath)?)
        .records()
        .count();

    let searcher = drug_core::initialize_searcher(
        ssi.algorithm,
        distance,
        ssi.max_edits,
        ssi.threshold,
        Some(&search_words),
        None,
    );
    let bar = initialize_progress(line_count as u64);
    let mut results: Vec<drug_core::SearchOutput> = Vec::new();
    for result in rdr.records() {
        let record = result?;
        if record.is_empty() {
            bar.inc(1);
            continue;
        }
        let record_id = if has_id {
            Some(
                record
                    .get(id_col_index.expect("no id column found"))
                    .expect("couldn't get record id")
                    .to_string(),
            )
        } else {
            None
        };

        let text = record
            .get(target_col_index)
            .expect("couldn't get record text");
        if text.is_empty() {
            bar.inc(1);
            continue;
        }
        let mut res = searcher.scan(text, record_id, &mut state);
        let output_list = drug_core::format(res.clone(), ssi.format)?;
        write_output(output_list, &mut out_file)?;
        results.append(&mut res);

        bar.inc(1);
    }
    bar.finish_with_message("Done.");
    // analyze
    let analysis = drug_core::analyze(
        results,
        search_words.len() as i32,
        line_count as i32,
        false,
        has_id,
    )?;
    for a in analysis {
        println!("{}", a);
    }
    println!("Done!");
    Ok(())
}

struct DsInput {
    fpath: String,
    target_column: String,
    id_column: Option<String>,
    rx_class_id: String,
    rx_class_relasource: String,
    algorithm: drug_core::Algorithm,
    max_edits: Option<i32>,
    threshold: Option<f64>,
    format: drug_core::OutputFormat,
    no_cache: bool,
}
fn run_drug_searcher(dsi: DsInput) -> Result<(), Box<dyn Error>> {
    validate_options(dsi.max_edits, dsi.threshold);

    println!("Welcome to the UK-IPOP Drug Extraction tool.");
    println!("------------------------------------------");
    let mut state = if !dsi.no_cache {
        println!("Establishing cache...");
        let map: HashMap<(String, String), f64> = HashMap::new();
        Some(map)
    } else {
        println!("No cache will be used");
        None
    };

    let drugs = fetch_drugs(&dsi.rx_class_id, &dsi.rx_class_relasource)?;
    let file = File::open(&dsi.fpath)?;
    let mut rdr = csv::Reader::from_reader(file);
    let headers = rdr.headers()?.clone();

    let distance = drug_core::initialize_distance(dsi.algorithm);
    let target_col_index =
        get_header_index(&headers, dsi.target_column).expect("could not find target column");
    let has_id = dsi.id_column.is_some();
    let id_col_index = if has_id {
        Some(
            get_header_index(&headers, dsi.id_column.expect("id column not found"))
                .expect("header index inaccessible"),
        )
    } else {
        None
    };
    let mut out_file = initialize_output_file(dsi.format, true)?;

    let line_count = csv::Reader::from_reader(File::open(&dsi.fpath)?)
        .records()
        .count();

    let searcher = drug_core::initialize_searcher(
        dsi.algorithm,
        distance,
        dsi.max_edits,
        dsi.threshold,
        None,
        Some(drugs.clone()),
    );
    let bar = initialize_progress(line_count as u64);

    let mut results: Vec<drug_core::SearchOutput> = Vec::new();
    for result in rdr.records() {
        let record = result?;
        if record.is_empty() {
            bar.inc(1);
            continue;
        }
        let record_id = if has_id {
            Some(
                record
                    .get(id_col_index.expect("id col inaccessible"))
                    .expect("record did not contain id")
                    .to_string(),
            )
        } else {
            None
        };

        let text = record
            .get(target_col_index)
            .expect("couldn't get record text");
        if text.is_empty() {
            bar.inc(1);
            continue;
        }
        let mut res = searcher.scan(text, record_id, &mut state);
        let output_list = drug_core::format(res.clone(), dsi.format)?;
        write_output(output_list, &mut out_file)?;
        results.append(&mut res);

        bar.inc(1);
    }
    bar.finish_with_message("Done.");
    // analyze
    let analysis =
        drug_core::analyze(results, drugs.len() as i32, line_count as i32, true, has_id)?;
    for a in analysis {
        println!("{}", a);
    }
    println!("Done!");
    Ok(())
}

fn write_output(
    data: Vec<String>,
    output_file: &mut File,
) -> std::result::Result<(), Box<dyn Error>> {
    let output = data.iter().peekable();
    for out in output {
        writeln!(output_file, "{}", out)?;
    }
    Ok(())
}

fn get_header_index(headers: &StringRecord, search: String) -> Option<usize> {
    let s = search.to_uppercase();
    for (i, h) in headers.iter().enumerate() {
        // remove BOM
        if h.to_ascii_uppercase().trim().trim_start_matches('\u{feff}')
            == s.trim().trim_start_matches('\u{feff}')
        {
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

fn initialize_output_file(
    format: drug_core::OutputFormat,
    is_drugs: bool,
) -> std::result::Result<fs::File, Box<dyn Error>> {
    let out_file = match format {
        drug_core::OutputFormat::JSONL => fs::File::create("extracted_drugs.jsonl")?,
        drug_core::OutputFormat::CSV => {
            let my_headers = if is_drugs {
                "record_id,algorithm,edits,similarity,matched_term,drug_name,rx_id,class_id"
                    .to_string()
            } else {
                "record_id,algorithm,edits,similarity,search_term,matched_term".to_string()
            };
            let mut f = fs::File::create("extracted_drugs.csv")?;
            f.write_all(my_headers.as_bytes())?;
            f.write_all(b"\n")?;
            f
        }
    };
    Ok(out_file)
}

/// A function to fetch a list of [`Drug`]s from RxNorm using the RxClass Rest API.
///
/// This function will return a vector of [`Drug`]s.
///
/// # Examples:
/// ```rust
/// # use extract_drugs_core::*;
/// let drugs = fetch_drugs("N02A", "ATC");
/// ```
///
pub fn fetch_drugs(
    class_id: &str,
    rela_source: &str,
) -> std::result::Result<Vec<drug_core::Drug>, Box<dyn Error>> {
    let url = format!(
        "https://rxnav.nlm.nih.gov/REST/rxclass/classMembers.json?classId={}&relaSource={}",
        class_id, rela_source
    );
    let res = reqwest::blocking::get(url)?;
    let data = res.json::<Root>()?;
    let list = data
        .drug_member_group
        .drug_member
        .iter()
        .map(|item| drug_core::Drug {
            name: item.min_concept.name.to_string(),
            rx_id: item.min_concept.rxcui.to_string(),
            class_id: class_id.to_string(),
        })
        .collect::<Vec<drug_core::Drug>>();
    Ok(list)
}

//////////////////////////////////////////////////////
/// A bunch of non-exported types for parsing the RxClass API.
/// This series of types was generated using [this tool](https://transform.tools/json-to-rust-serde) by simply pasting the RxClass API JSON output into the tool.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Root {
    pub drug_member_group: DrugMemberGroup,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DrugMemberGroup {
    pub drug_member: Vec<DrugMember>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DrugMember {
    pub min_concept: MinConcept,
    pub node_attr: Vec<NodeAttr>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MinConcept {
    pub rxcui: String,
    pub name: String,
    pub tty: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NodeAttr {
    pub attr_name: String,
    pub attr_value: String,
}
