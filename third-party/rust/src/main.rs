extern crate serde_json;
extern crate strsim;
use indicatif::ProgressBar;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{stdin, stdout, BufReader};
use std::time::Instant;
use strsim::{jaro_winkler, normalized_damerau_levenshtein};

fn main() {
    let mut input = String::new();
    println!("Which algorithm do you want to use? (J/L)");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut input)
        .expect("invalid string entered");
    match input.trim() {
        "J" => run_jaro_winkler(),
        "L" => run_levenshtein(),
        _ => println!("Not acceptable value provided, please use `J` or `L`."),
    }
}

#[derive(Serialize)]
struct RowData {
    casenumber: String,
    results: Vec<HashMap<String, String>>,
}

const ENDLINE_BYTE: &[u8] = "\n".as_bytes();

fn make_combined_cause(row: Value) -> Value {
    let cause1 = row["primarycause"].as_str();
    let cause2 = row["primarycause_linea"].as_str();
    let cause3 = row["primarycause_lineb"].as_str();
    let cause4 = row["primarycause_linec"].as_str();
    let mut combined = String::new();
    combined.push_str(cause1.unwrap_or_default());
    combined.push_str(cause2.unwrap_or_default());
    combined.push_str(cause3.unwrap_or_default());
    combined.push_str(cause4.unwrap_or_default());
    let mut new_row = row;
    new_row["primary_combined"] = Value::String(combined);
    return new_row;
}

fn run_levenshtein() {
    let file = File::open("../../data/records.jsonl").expect("couldn't open file");
    let reader = BufReader::new(file);
    let mut out_file = File::create("../../data/third-party/rust-leven.jsonl")
        .expect("couldn't create output file.");
    let pb = ProgressBar::new_spinner();
    for line in reader.lines() {
        let my_line = line.expect("couldn't read line");
        let value: Value = serde_json::from_str(&my_line).expect("unable to parse json");
        let value = make_combined_cause(value);
        for level in ["primary_combined", "secondarycause"] {
            let case_id = value["casenumber"].to_string();
            let words = value[level].to_string();
            let text = words.trim();
            if text != "\"\"" {
                if text != "null" {
                    let leven_result =
                        search_record_levenshtein(text.to_string(), level.to_string());
                    let row_data = RowData {
                        casenumber: case_id,
                        results: leven_result,
                    };
                    let json_data = serde_json::to_string(&row_data).expect("could not serialize");
                    out_file
                        .write(&[json_data.as_bytes(), ENDLINE_BYTE].concat())
                        .expect("unable to write line to file");
                }
            }
        }
        pb.inc(1);
    }
    pb.finish_with_message("Done.");
}

fn run_jaro_winkler() {
    let file = File::open("../../data/records.jsonl").expect("couldn't open file");
    let reader = BufReader::new(file);
    let mut out_file = File::create("../../data/third-party/rust-jaro.jsonl")
        .expect("couldn't create output file.");
    let pb = ProgressBar::new_spinner();
    for line in reader.lines() {
        let my_line = line.expect("couldn't read line");
        let value: Value = serde_json::from_str(&my_line).expect("unable to parse json");
        let value = make_combined_cause(value);
        for level in ["primary_combined", "secondarycause"] {
            let case_id = value["casenumber"].to_string();
            let words = value[level].to_string();
            let text = words.trim();
            if text != "\"\"" {
                if text != "null" {
                    let jaro_result =
                        search_record_jaro_winkler(text.to_string(), level.to_string());
                    let row_data = RowData {
                        casenumber: case_id,
                        results: jaro_result,
                    };
                    let json_data = serde_json::to_string(&row_data).expect("could not serialize");
                    out_file
                        .write(&[json_data.as_bytes(), ENDLINE_BYTE].concat())
                        .expect("unable to write line to file");
                }
            }
        }
        pb.inc(1);
    }
    pb.finish_with_message("Done.");
}

fn search_record_levenshtein(text: String, level: String) -> Vec<HashMap<String, String>> {
    let mut data: Vec<HashMap<String, String>> = Vec::new();
    for word in text.split(" ") {
        let mut word_data: HashMap<String, String> = HashMap::new();
        let now = Instant::now();
        let ratio = normalized_damerau_levenshtein(word, "heroin");
        word_data.insert(String::from("word"), word.to_string());
        word_data.insert(String::from("distance"), ratio.to_string());
        word_data.insert(String::from("level"), level.to_string());
        word_data.insert(
            String::from("metric"),
            String::from("Normalized Levenshtein"),
        );
        word_data.insert(
            String::from("time"),
            now.elapsed().as_secs_f64().to_string(),
        );
        data.push(word_data)
    }
    return data;
}

fn search_record_jaro_winkler(text: String, level: String) -> Vec<HashMap<String, String>> {
    let mut data: Vec<HashMap<String, String>> = Vec::new();
    for word in text.split(" ") {
        let mut word_data: HashMap<String, String> = HashMap::new();
        let now = Instant::now();
        let ratio = jaro_winkler(word, "heroin");
        word_data.insert(String::from("word"), word.to_string());
        word_data.insert(String::from("distance"), ratio.to_string());
        word_data.insert(String::from("level"), level.to_string());
        word_data.insert(String::from("metric"), String::from("Jaro-Winkler"));
        word_data.insert(
            String::from("time"),
            now.elapsed().as_secs_f64().to_string(),
        );
        data.push(word_data)
    }
    return data;
}
