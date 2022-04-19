//tutorial-read-01.rs
use clap::Parser;
use csv::StringRecord;
use std::error::Error;
use std::ffi::OsString;
use std::fs::{self, File};
use std::io::{LineWriter, Write};
use std::process;
use std::str::FromStr;
use std::{env, path::Path};

use extract_drugs_core::utils::{self as drug_core, Output};

#[derive(Parser)]
#[clap(args_override_self = true)]
#[clap(author, version, about, long_about = None)]
struct Tool {
    #[clap(default_value_t = String::from("cli/data/Medical_Examiner_Case_Archive.csv"))]
    file: String,

    #[clap(short, default_value_t = String::from("Case Number"))]
    id_column: String,

    #[clap(short, default_value_t = String::from("Primary Cause"))]
    target_column: String,

    #[clap(short, default_value_t = String::from("cocaine|heroin|fentanyl|alcohol"))]
    search_words: String,

    #[clap(short, default_value_t = String::from("l"))]
    algorithm: String,

    #[clap(short, default_value_t = 0.90)]
    limit: f64,
}

fn run() -> Result<(), Box<dyn Error>> {
    // will move into struct (clap)
    let args = Tool::parse();
    let file_path = args.file;
    let target_col = args.target_column;
    let id_col = args.id_column;
    let search_words = args
        .search_words
        .split('|')
        .map(|x| x.to_uppercase())
        .collect::<Vec<String>>();
    let user_algo = args.algorithm;
    let limit = args.limit;
    // handle config file
    // let config = args.config

    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);

    let algorithm = drug_core::Algorithm::from_str(&user_algo).unwrap();
    let distance = drug_core::initialize_distance(algorithm);

    // clones, could use scoped alternative to return header indices
    let headers = rdr.headers()?.clone();
    let target_col_index = get_header_index(&headers, target_col).unwrap();
    let id_col_index = get_header_index(&headers, id_col).unwrap();
    println!("{:?}", headers);

    let mut out_file = fs::File::create("extracted.jsonl").unwrap();

    for result in rdr.records() {
        let record = result?;
        if record.is_empty() {
            continue;
        }
        let record_id = record.get(id_col_index).unwrap();
        let text = record.get(target_col_index).unwrap();
        if text.is_empty() || record_id.is_empty() {
            continue;
        }
        let res = drug_core::scan(
            algorithm,
            distance,
            text,
            record_id,
            &search_words,
            Some(limit),
        );
        // if !res.is_empty() {
        //     println!("{:?}", res);
        // }
        for output in res {
            let json_string = serde_json::to_string(&output).unwrap();
            out_file.write_all(json_string.as_bytes());
            out_file.write(b"\n");
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

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
    // drug_core::
}
