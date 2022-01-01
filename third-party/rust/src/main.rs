mod utils;

extern crate serde_json;

use std::io::prelude::*;
use std::io::{stdin, stdout, BufReader};

fn main() {
    let file_lines = utils::get_file_lines();
    let file_data = utils::load_data();
    let user_input = utils::get_user_input();
    match user_input.as_str() {
        "J" => utils::jarowinkler_runner(file_data, file_lines),
        "L" => utils::levenshtein_runner(file_data,  file_lines),
        _ => println!("Not acceptable value provided, please use `J` or `L`."),
    }
}