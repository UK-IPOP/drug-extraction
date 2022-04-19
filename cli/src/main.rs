//tutorial-read-01.rs
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;
use std::str::FromStr;

use extract_drugs_core::utils as drug_core;

fn run() -> Result<(), Box<dyn Error>> {
    // will move into struct (clap)
    let args = env::args();
    let file_path = get_x_arg(1).unwrap();
    let user_search_words = get_x_arg(2).unwrap().to_ascii_uppercase();
    let search_words = user_search_words
        .to_str()
        .unwrap()
        .split('|')
        .collect::<Vec<&str>>();
    let limit = get_x_arg(3)
        .unwrap()
        .to_str()
        .unwrap()
        .parse::<f64>()
        .unwrap();
    // let file_path = "cli/data/Medical_Examiner_Case_Archive.csv";
    // let search_word = "COCAINE";
    // let limit = Some(0.95);
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);

    let algorithm = drug_core::Algorithm::from_str("l").unwrap();
    let distance = drug_core::initialize_distance(algorithm);

    // clones, could use scoped alternative to return header indices
    let headers = rdr.headers()?.clone();
    let target_index = 7;
    println!("{:?}", headers);

    for (i, result) in rdr.records().enumerate() {
        let record = result?;
        let cod = record.get(target_index).unwrap();
        if cod.is_empty() {
            continue;
        }
        let res = drug_core::scan(
            algorithm,
            distance,
            cod,
            i.to_string().as_str(),
            &search_words,
            Some(limit),
        );
        if !res.is_empty() {
            println!("{:?}", res);
        }
    }
    Ok(())
}

/// Returns the first positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
fn get_x_arg(x: usize) -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(x) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
    // drug_core::
}
