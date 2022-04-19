//tutorial-read-01.rs
use clap::Parser;
use csv::StringRecord;
use indicatif::{ProgressBar, ProgressStyle};
use std::error::Error;
use std::ffi::OsString;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::io::{LineWriter, Write};
use std::process;
use std::str::FromStr;
use std::{env, path::Path};
use walkdir::WalkDir;

use extract_drugs_core::utils::{self as drug_core, Input, Output};

#[derive(Parser)]
#[clap(args_override_self = true)]
#[clap(author, version, about, long_about = None)]
struct Tool {
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
}

fn run() -> Result<(), Box<dyn Error>> {
    // will move into struct (clap)
    let args = Tool::parse();
    let file_path = args.file;
    let target_col = args.target_column;
    let user_id_col = args.id_column;
    let has_id = user_id_col.is_some();
    let search_words = args
        .search_words
        .split('|')
        .map(|x| x.to_uppercase())
        .collect::<Vec<String>>();
    let user_algo = args.algorithm;
    let max_edits = args.max_edits;
    let thresh = args.threshold;
    // handle config file
    // let config = args.config

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

    let mut out_file = fs::File::create("extracted.jsonl").unwrap();

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
        let searcher = drug_core::SearchInput::new(
            algorithm,
            distance,
            max_edits,
            thresh,
            search_words.as_slice(),
        );
        let res = searcher.scan(text, record_id);
        // if !res.is_empty() {
        //     println!("{:?}", res);
        // }
        for output in res {
            let json_string = serde_json::to_string(&output).unwrap();
            out_file.write_all(json_string.as_bytes());
            out_file.write(b"\n");
        }
        bar.inc(1);
    }
    bar.finish();
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
    // drug_core::
}
