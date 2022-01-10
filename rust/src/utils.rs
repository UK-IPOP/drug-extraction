use indicatif;
use log::LevelFilter;
use serde_json::Value;
use simple_logging;
use std::cmp::max;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::{stdin, stdout};
use std::string::String;
use std::time::Instant;
use strsim::{jaro_winkler, levenshtein};

const RECORD_COUNT: u64 = 59_630;

pub fn load_data() -> BufReader<File> {
    let file = File::open("../data/input/records.jsonl").expect("could not open input file");
    let reader = BufReader::new(file);
    reader
}

pub fn get_user_input() -> String {
    let mut input = String::new();
    println!("Which metric would you like to run?");
    print!("JaroWinkler or Levenshtein? (J/L): ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut input)
        .expect("Invalid string. Expected one of `J` or `L`.");
    let clean = input.trim().to_string();
    return clean;
}

fn combine_cols(mut row: Value) -> Value {
    let cols = [
        "primarycause",
        "primarycause_linea",
        "primarycause_lineb",
        "primarycause_linec",
    ];
    let mut combined_primary = String::new();
    for col in cols.iter().cloned() {
        if let Some(value) = row.get(col) {
            combined_primary.push_str(value.as_str().unwrap_or_default());
            combined_primary.push_str(" ");
        }
    }
    row["primary_combined"] = Value::String(combined_primary.trim().to_string());
    row
}

pub fn load_drugs() -> Vec<Value> {
    let file = File::open("../data/input/drugs.jsonl").expect("could not open drug file");
    let reader = BufReader::new(file);
    let mut data: Vec<Value> = Vec::new();
    for line in reader.lines() {
        let line = line.expect("no valid line when reading file");
        let drug_json: Value = serde_json::from_str(&line).expect("could not convert drug to json");
        data.push(drug_json);
    }
    data
}

pub fn levenshtein_runner(reader: BufReader<File>) {
    let drugs = load_drugs();
    let bar = indicatif::ProgressBar::new(RECORD_COUNT);
    simple_logging::log_to_file("../data/results/rust.log", LevelFilter::Info)
        .expect("could not initialize logger");

    let mut result_count = 0;
    let mut total_time = 0.00;
    let metric_name = "Levenshtein";
    for line in reader.lines() {
        let line = line.expect("no valid line when reading file");
        let json_value: Value = serde_json::from_str(&line).expect("could not convert to json");
        let row = combine_cols(json_value);
        for col in ["primary_combined", "secondarycause"].iter().cloned() {
            let possible_text = row.get(col);
            let text = match possible_text {
                Some(t) => t.as_str().expect("could not convert text Value to str"),
                _ => {
                    let v = "";
                    v
                }
            };
            let search_results = search_record_levenshtein(text.to_string(), col, &drugs);
            if search_results.len() == 0 {
                continue;
            }
            for result in search_results {
                result_count += 1;
                total_time += result["time"].as_f64().unwrap();
            }
        }
        bar.inc(1);
    }
    bar.finish();
    let average = total_time / result_count as f64;
    println!(
        "{} results took {} seconds for {} with an average time of {}",
        result_count, total_time, metric_name, average
    );
    log::info!(
        "{} results took {} seconds for {} with an average time of {}",
        result_count,
        total_time,
        metric_name,
        average
    );
}

fn search_record_levenshtein(
    text: String,
    level: &str,
    drug_list: &Vec<Value>,
) -> Vec<HashMap<String, Value>> {
    let mut data: Vec<HashMap<String, Value>> = Vec::new();
    let clean_text = text.to_ascii_uppercase().replace(
        &[
            '(', ')', ',', ';', ':', '@', '#', '$', '%', '^', '&', '*', '_', '+', '=', '{', '}',
            '[', ']', '|', '<', '>', '/',
        ][..],
        "",
    );
    for drug in drug_list {
        let drug_id = drug.get("rx_id").unwrap().as_str().unwrap();
        let drug_words = drug.get("name").unwrap().to_string().to_ascii_uppercase();
        let drug_names = drug_words.split('/');
        for name in drug_names {
            for word in clean_text.split_whitespace() {
                let mut word_data: HashMap<String, Value> = HashMap::new();
                let start_time = Instant::now();
                let d = levenshtein(word, name);
                let elapsed_time = start_time.elapsed().as_secs_f64();
                let distance: f64 = 1.0 - (d as f64 / max(word.len(), name.len()) as f64);
                word_data.insert(String::from("word"), Value::from(word));
                word_data.insert(String::from("similarity"), Value::from(distance));
                word_data.insert(String::from("level"), Value::from(level));
                word_data.insert(String::from("metric"), Value::from("NormalizedLevenshtein"));
                word_data.insert(String::from("time"), Value::from(elapsed_time));
                word_data.insert(
                    String::from("drug_name"),
                    Value::from(name.trim_matches('"')),
                );
                word_data.insert(String::from("drug_id"), Value::from(drug_id));
                data.push(word_data);
            }
        }
    }
    data
}

pub fn jarowinkler_runner(reader: BufReader<File>) {
    let drugs = load_drugs();
    let bar = indicatif::ProgressBar::new(RECORD_COUNT);
    simple_logging::log_to_file("../data/results/rust.log", LevelFilter::Info)
        .expect("could not initialize logger");

    let mut result_count = 0;
    let mut total_time = 0.00;
    let metric_name = "JaroWinkler";
    for line in reader.lines() {
        let line = line.expect("no valid line when reading file");
        let json_value: Value = serde_json::from_str(&line).expect("could not convert to json");
        let row = combine_cols(json_value);
        for col in ["primary_combined", "secondarycause"].iter().cloned() {
            let possible_text = row.get(col);
            let text = match possible_text {
                Some(t) => t.as_str().expect("could not convert text Value to str"),
                _ => {
                    let v = "";
                    v
                }
            };
            let search_results = search_record_jarowinkler(text.to_string(), col, &drugs);
            if search_results.len() == 0 {
                continue;
            }
            for result in search_results {
                result_count += 1;
                total_time += result["time"].as_f64().unwrap();
            }
        }
        bar.inc(1);
    }
    bar.finish();
    let average = total_time / result_count as f64;
    println!(
        "{} results took {} seconds for {} with an average time of {}",
        result_count, total_time, metric_name, average
    );
    log::info!(
        "{} results took {} seconds for {} with an average time of {}",
        result_count,
        total_time,
        metric_name,
        average
    );
}

fn search_record_jarowinkler(
    text: String,
    level: &str,
    drug_list: &Vec<Value>,
) -> Vec<HashMap<String, Value>> {
    let mut data: Vec<HashMap<String, Value>> = Vec::new();
    let clean_text = text.to_ascii_uppercase().replace(
        &[
            '(', ')', ',', ';', ':', '@', '#', '$', '%', '^', '&', '*', '_', '+', '=', '{', '}',
            '[', ']', '|', '<', '>', '/',
        ][..],
        "",
    );
    for drug in drug_list {
        let drug_id = drug.get("rx_id").unwrap().as_str().unwrap();
        let drug_words = drug.get("name").unwrap().to_string().to_ascii_uppercase();
        let drug_names = drug_words.split('/');
        for name in drug_names {
            for word in clean_text.split_whitespace() {
                let mut word_data: HashMap<String, Value> = HashMap::new();
                let start_time = Instant::now();
                let d = jaro_winkler(word, name);
                let elapsed_time = start_time.elapsed().as_secs_f64();
                let distance: f64 = 1.0 - (d as f64 / max(word.len(), name.len()) as f64);
                word_data.insert(String::from("word"), Value::from(word));
                word_data.insert(String::from("similarity"), Value::from(distance));
                word_data.insert(String::from("level"), Value::from(level));
                word_data.insert(String::from("metric"), Value::from("JaroWinkler"));
                word_data.insert(String::from("time"), Value::from(elapsed_time));
                word_data.insert(
                    String::from("drug_name"),
                    Value::from(name.trim_matches('"')),
                );
                word_data.insert(String::from("drug_id"), Value::from(drug_id));
                data.push(word_data);
            }
        }
    }
    data
}