use serde::Serialize;
use serde_json::Value;
use std::cmp::max;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::{stdin, stdout};
use std::string::String;
use std::time::Instant;
use strsim::{jaro_winkler, levenshtein};

pub fn load_data() -> BufReader<File> {
    let file = File::open("../data/records.jsonl").expect("could not open input file");
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
            combined_primary.push_str(value.as_str().unwrap_or_default())
        }
    }
    row["primary_combined"] = Value::String(combined_primary);
    row
}

#[derive(Serialize, Debug)]
struct ResultData {
    casenumber: String,
    results: Vec<HashMap<String, Value>>,
}

const ENDLINE_BYTE: &[u8] = "\n".as_bytes();

pub fn levenshtein_runner(reader: BufReader<File>) {
    let mut out_file =
        File::create("../data/rust-levenshtein.jsonl").expect("could not create output file.");
    for line in reader.lines() {
        let line = line.expect("no valid line when reading file");
        let json_value: Value = serde_json::from_str(&line).expect("could not convert to json");
        let row = combine_cols(json_value);
        for col in ["primary_combined", "secondarycause"].iter().cloned() {
            let case_id = row
                .get("casenumber")
                .expect("row did not have case number")
                .as_str()
                .expect("could not convert case_id Value to str");
            let possible_text = row.get(col);
            let text = match possible_text {
                Some(t) => t.as_str().expect("could not convert text Value to str"),
                _ => {
                    let v = "";
                    v
                }
            };
            let search_results = search_record_levenshtein(text.to_string(), col);
            if search_results.len() == 0 {
                continue;
            }
            let data = ResultData {
                casenumber: case_id.to_string(),
                results: search_results,
            };
            let json_data = serde_json::to_string(&data).expect("could not create json data");
            out_file
                .write(&[json_data.as_bytes(), ENDLINE_BYTE].concat())
                .expect("could not write jsonline");
        }
    }
}

fn search_record_levenshtein(text: String, level: &str) -> Vec<HashMap<String, Value>> {
    let mut data: Vec<HashMap<String, Value>> = Vec::new();
    let clean_text = text
        .to_lowercase()
        .replace(|c: char| !c.is_alphanumeric(), "");
    for word in clean_text.split_whitespace() {
        let mut word_data: HashMap<String, Value> = HashMap::new();
        let start_time = Instant::now();
        let d = levenshtein(word, "heroin");
        let elapsed_time = start_time.elapsed().as_secs_f64();
        let distance: f64 = 1.0 - (d as f64 / max(word.len(), "heroin".len()) as f64);
        word_data.insert(String::from("word"), Value::from(word));
        word_data.insert(String::from("distance"), Value::from(distance));
        word_data.insert(String::from("level"), Value::from(level));
        word_data.insert(String::from("metric"), Value::from("NormalizedLevenshtein"));
        word_data.insert(String::from("time"), Value::from(elapsed_time));
        data.push(word_data);
    }
    data
}

pub fn jarowinkler_runner(reader: BufReader<File>) {
    let mut out_file =
        File::create("../data/rust-jarowinkler.jsonl").expect("could not create output file.");
    for line in reader.lines() {
        let line = line.expect("no valid line when reading file");
        let json_value: Value = serde_json::from_str(&line).expect("could not convert to json");
        let row = combine_cols(json_value);
        for col in ["primary_combined", "secondarycause"].iter().cloned() {
            let case_id = row
                .get("casenumber")
                .expect("row did not have case number")
                .as_str()
                .expect("could not convert case_id Value to str");
            let possible_text = row.get(col);
            let text = match possible_text {
                Some(t) => t.as_str().expect("could not convert text Value to str"),
                _ => {
                    let v = "";
                    v
                }
            };
            let search_results = search_record_jarowinkler(text.to_string(), col);
            if search_results.len() == 0 {
                continue;
            }
            let data = ResultData {
                casenumber: case_id.to_string(),
                results: search_results,
            };
            let json_data = serde_json::to_string(&data).expect("could not create json data");
            out_file
                .write(&[json_data.as_bytes(), ENDLINE_BYTE].concat())
                .expect("could not write jsonline");
        }
    }
}

fn search_record_jarowinkler(text: String, level: &str) -> Vec<HashMap<String, Value>> {
    let mut data: Vec<HashMap<String, Value>> = Vec::new();
    let clean_text = text
        .to_lowercase()
        .replace(|c: char| !c.is_alphanumeric(), "");
    for word in clean_text.split_whitespace() {
        let mut word_data: HashMap<String, Value> = HashMap::new();
        let start_time = Instant::now();
        let d = jaro_winkler(word, "heroin");
        let elapsed_time = start_time.elapsed().as_secs_f64();
        let distance: f64 = 1.0 - (d as f64 / max(word.len(), "heroin".len()) as f64);
        word_data.insert(String::from("word"), Value::from(word));
        word_data.insert(String::from("distance"), Value::from(distance));
        word_data.insert(String::from("level"), Value::from(level));
        word_data.insert(String::from("metric"), Value::from("JaroWinkler"));
        word_data.insert(String::from("time"), Value::from(elapsed_time));
        data.push(word_data);
    }
    data
}
