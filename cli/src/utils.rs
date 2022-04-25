use clap::{Parser, Subcommand};
use csv::StringRecord;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Confirm, Input, Select};
use drug_core::initialize_searcher;
use indicatif::{ProgressBar, ProgressStyle};
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::process::exit;
use std::str::FromStr;

use extract_drugs_core as drug_core;

/// Main entry point for the CLI.
#[derive(Parser)]
#[clap(args_override_self = true)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

/// Subcommands for the CLI.
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Interactive mode. Useful for first time users or one-offs.
    Interactive,

    /// Execute mode. Useful for automation/scripts.
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
    },
    /// Execute mode. Useful for automation/scripts.
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
            };
            run_drug_searcher(dsi)?;
        }
    }
    Ok(())
}

// TODO: Could return a Result<()>
/// This function is used to validate the input parameters.
fn validate_options(max_edits: Option<i32>, threshold: Option<f64>) {
    if threshold.is_none() && max_edits.is_none() {
        let confirmed = Confirm::new().with_prompt("You have not specified a threshold or max edits. This will return ALL items in ALL records. Are you sure you want to continue?").interact().unwrap();
        if !confirmed {
            exit(1);
        } else {
            println!("Continuing...");
        }
    }
    if threshold.is_some() && max_edits.is_some() {
        println!("You have specified both a threshold and max edits. Max edits takes precedence.");
    }
    if max_edits.is_some() {
        if let Some(me) = max_edits {
            if !(0..=10).contains(&me) {
                println!("Max edits must be between 0 and 10");
                exit(1);
            }
        }
    }
    if threshold.is_some() {
        if let Some(th) = threshold {
            if !(0.0..=1.0).contains(&th) {
                println!("Threshold must be between 0.0 and 1.0");
                exit(1);
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
        .interact_text()
        .unwrap();
    let mut columns = String::new();
    BufReader::new(File::open(&file_path).unwrap())
        .read_line(&mut columns)
        .unwrap();
    let column_choices = columns.split(',').collect::<Vec<&str>>();
    let id_col_select = Select::with_theme(&ColorfulTheme::default())
        .items(&column_choices)
        .with_prompt(
            "Please select your ID column or press `Esc` or `q` to move on with NO ID specified",
        )
        .default(0)
        .interact_opt()
        .unwrap();
    let user_id_col = id_col_select.map(|id_sel| column_choices[id_sel].to_string());
    let target_col_select = Select::with_theme(&ColorfulTheme::default())
        .items(&column_choices)
        .with_prompt("Please select your target text column (required)")
        .default(0)
        .interact()
        .unwrap();
    let target_col = column_choices[target_col_select].to_string();
    let algorithm_options = drug_core::Algorithm::options();
    let user_algo = Select::with_theme(&ColorfulTheme::default())
        .items(&algorithm_options)
        .with_prompt("Please select your algorithm")
        .default(0)
        .interact()
        .unwrap();
    let algorithm = drug_core::Algorithm::from_str(&algorithm_options[user_algo]).unwrap();
    let max_edits = Input::new()
        .with_prompt("Please enter the maximum number of edits you would like to allow or press enter to move on")
        .default(3)
        .interact()
        .unwrap();
    let thresh: f64 = Input::new()
        .with_prompt("Please enter the minimum similarity threshold you would like to filter to")
        .default(0.9)
        .interact()
        .unwrap();
    let format_options = vec!["JSONL", "CSV"];
    let user_format_choice = Select::with_theme(&ColorfulTheme::default())
        .items(&format_options)
        .with_prompt("Please enter the output format you would prefer")
        .default(0)
        .interact()
        .unwrap();
    let user_search_words: String = Input::new()
        .with_prompt("Please enter your search words separated by a `|` symbol")
        .interact()
        .unwrap();

    let user_format =
        drug_core::OutputFormat::from_str(format_options[user_format_choice]).unwrap();

    let ssi = SsInput {
        fpath: file_path,
        target_column: target_col,
        id_column: user_id_col,
        search_words: user_search_words,
        algorithm,
        max_edits: Some(max_edits),
        threshold: Some(thresh),
        format: user_format,
    };
    run_simple_searcher(ssi)?;
    Ok(())
}

fn interactive_drug_search() -> Result<(), Box<dyn Error>> {
    let file_path: String = Input::new()
        .with_prompt("Please enter the path to your data file")
        .interact_text()
        .unwrap();
    let mut columns = String::new();
    BufReader::new(File::open(&file_path).unwrap())
        .read_line(&mut columns)
        .unwrap();
    let column_choices = columns.split(',').collect::<Vec<&str>>();
    let id_col_select = Select::with_theme(&ColorfulTheme::default())
        .items(&column_choices)
        .with_prompt(
            "Please select your ID column or press `Esc` or `q` to move on with NO ID specified",
        )
        .default(0)
        .interact_opt()
        .unwrap();
    let user_id_col = id_col_select.map(|id_sel| column_choices[id_sel].to_string());
    let target_col_select = Select::with_theme(&ColorfulTheme::default())
        .items(&column_choices)
        .with_prompt("Please select your target text column (required)")
        .default(0)
        .interact()
        .unwrap();
    let target_col = column_choices[target_col_select].to_string();
    let algorithm_options = drug_core::Algorithm::options();
    let user_algo = Select::with_theme(&ColorfulTheme::default())
        .items(&algorithm_options)
        .with_prompt("Please select your algorithm")
        .default(0)
        .interact()
        .unwrap();
    let algorithm = drug_core::Algorithm::from_str(&algorithm_options[user_algo]).unwrap();
    let max_edits = Input::new()
        .with_prompt("Please enter the maximum number of edits you would like to allow or press enter to move on")
        .default(3)
        .interact()
        .unwrap();
    let thresh: f64 = Input::new()
        .with_prompt("Please enter the minimum similarity threshold you would like to filter to")
        .default(0.9)
        .interact_text()
        .unwrap();
    let user_rx_id = Input::new()
        .with_prompt("Please enter your target RxNorm ID")
        .interact_text()
        .unwrap();
    let user_rx_relasource = Input::new()
        .with_prompt("Please enter your RxNorm relationship source")
        .interact_text()
        .unwrap();
    let format_options = vec!["JSONL", "CSV"];
    let user_format_choice = Select::with_theme(&ColorfulTheme::default())
        .items(&format_options)
        .with_prompt("Please enter the output format you would prefer")
        .default(0)
        .interact()
        .unwrap();

    let user_format =
        drug_core::OutputFormat::from_str(format_options[user_format_choice]).unwrap();

    let dsi = DsInput {
        fpath: file_path,
        target_column: target_col,
        id_column: user_id_col,
        rx_class_id: user_rx_id,
        rx_class_relasource: user_rx_relasource,
        algorithm,
        max_edits: Some(max_edits),
        threshold: Some(thresh),
        format: user_format,
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
}

fn run_simple_searcher(ssi: SsInput) -> Result<(), Box<dyn Error>> {
    validate_options(ssi.max_edits, ssi.threshold);
    if !ssi.search_words.contains('|') {
        println!("Please enter your search words separated by a `|` symbol");
        exit(1);
    }
    let search_words = ssi
        .search_words
        .split('|')
        .map(|x| x.to_uppercase())
        .collect::<Vec<String>>();
    let file = File::open(&ssi.fpath)?;
    let mut rdr = csv::Reader::from_reader(file);
    let headers = rdr.headers()?.clone();

    let distance = drug_core::initialize_distance(ssi.algorithm);
    let target_col_index = get_header_index(&headers, ssi.target_column).unwrap();
    let has_id = ssi.id_column.is_some();
    let id_col_index = if has_id {
        Some(get_header_index(&headers, ssi.id_column.unwrap()).unwrap())
    } else {
        None
    };
    let mut out_file = initialize_output_file(ssi.format, false);

    let line_count = BufReader::new(File::open(&ssi.fpath).unwrap())
        .lines()
        .count();
    let searcher = initialize_searcher(
        ssi.algorithm,
        distance,
        ssi.max_edits,
        ssi.threshold,
        Some(&search_words),
        None,
    );
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
        let res = searcher.scan(text, record_id);
        let output_list = drug_core::format(res, ssi.format);
        write_output(output_list, &mut out_file);
        bar.inc(1);
    }
    bar.finish();

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
}
fn run_drug_searcher(dsi: DsInput) -> Result<(), Box<dyn Error>> {
    validate_options(dsi.max_edits, dsi.threshold);

    let drugs = drug_core::fetch_drugs(&dsi.rx_class_id, &dsi.rx_class_relasource);
    let file = File::open(&dsi.fpath)?;
    let mut rdr = csv::Reader::from_reader(file);
    let headers = rdr.headers()?.clone();

    let distance = drug_core::initialize_distance(dsi.algorithm);
    let target_col_index = get_header_index(&headers, dsi.target_column).unwrap();
    let has_id = dsi.id_column.is_some();
    let id_col_index = if has_id {
        Some(get_header_index(&headers, dsi.id_column.unwrap()).unwrap())
    } else {
        None
    };
    let mut out_file = initialize_output_file(dsi.format, true);

    let line_count = BufReader::new(File::open(&dsi.fpath).unwrap())
        .lines()
        .count();
    let searcher = initialize_searcher(
        dsi.algorithm,
        distance,
        dsi.max_edits,
        dsi.threshold,
        None,
        Some(drugs),
    );
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
        let res = searcher.scan(text, record_id);
        let output_list = drug_core::format(res, dsi.format);
        write_output(output_list, &mut out_file);
        bar.inc(1);
    }
    bar.finish();

    Ok(())
}

fn write_output(data: Vec<String>, output_file: &mut File) {
    let mut output = data.iter().peekable();
    while let Some(out) = output.next() {
        // check for last item
        if output.peek().is_some() {
            output_file.write_all(out.as_bytes()).unwrap();
            output_file.write_all(b"\n").unwrap();
        }
    }
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

fn initialize_output_file(format: drug_core::OutputFormat, is_drugs: bool) -> fs::File {
    let out_file = match format {
        drug_core::OutputFormat::JSONL => fs::File::create("extracted_drugs.jsonl").unwrap(),
        drug_core::OutputFormat::CSV => {
            let my_headers = if is_drugs {
                "record_id,algorithm,edits,similarity,matched_term,drug_name,rx_id,group_name,class_id".to_string()
            } else {
                "record_id,algorithm,edits,similarity,matched_term,search_term".to_string()
            };
            let mut f = fs::File::create("extracted_drugs.csv").unwrap();
            f.write_all(my_headers.as_bytes()).unwrap();
            f.write_all(b"\n").unwrap();
            f
        }
    };
    out_file
}
