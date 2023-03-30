use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    ascii::AsciiExt,
    io::{BufRead, BufReader, Lines, Stdin},
    iter::Enumerate,
    path::PathBuf,
    str::FromStr,
    time::Duration,
};

use cached::{proc_macro::cached, SizedCache};
use color_eyre::{eyre::Context, Help, Result};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressIterator, ProgressStyle};
use itertools::Itertools;
use rayon::prelude::{
    IntoParallelIterator, IntoParallelRefIterator, ParallelBridge, ParallelIterator,
};

use crate::{
    in_and_out::{find_column_indices, load_search_terms, pull_header},
    options::{PipeArgs, SearchArgs},
    types::{Output, ProgressKind, SearchTerm},
};

#[cached(
    type = "SizedCache<String, f64>",
    create = "{ SizedCache::with_size(1_000) }",
    convert = r#"{ format!("{}{}", a, b) }"#
)]
pub fn jw(a: &str, b: &str) -> f64 {
    strsim::jaro_winkler(a, b)
}

fn remove_symbols_except_dash(s: String) -> String {
    let ss: String = s
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' {
                c
            } else {
                ' '
            }
        })
        .collect();
    ss.trim().to_owned()
}

fn initialize_spinner_style(msg: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.with_style(
        ProgressStyle::with_template("{spinner:.blue} {msg}")
            .unwrap()
            .tick_strings(&[
                "▹▹▹▹▹",
                "▸▹▹▹▹",
                "▹▸▹▹▹",
                "▹▹▸▹▹",
                "▹▹▹▸▹",
                "▹▹▹▹▸",
                "▪▪▪▪▪",
            ]),
    )
    .with_message(msg.to_owned())
}

#[derive(Debug)]
struct SearchLines {
    words: Vec<String>,
    identifier: String,
    field: String,
}
struct SearchInput {
    lines: Vec<SearchLines>,
    search_terms: Vec<SearchTerm>,
    limit: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct SearchOutput {
    row_id: String,
    search_term: String,
    matched_term: String,
    similarity: f64,
    metadata: Option<String>,
    search_field: String,
}

fn search(input: SearchInput) -> Vec<SearchOutput> {
    let len = &input.lines.len();
    let outputs: Vec<SearchOutput> = input
        .lines
        .par_iter()
        .progress_with(ProgressBar::new(*len as u64))
        .flat_map(|line| {
            input.search_terms.par_iter().flat_map(|st| {
                line.words
                    .windows(st.ngrams())
                    .map(|window| window.join(" "))
                    .unique()
                    .filter_map(|w| {
                        let sim = jw(&w, &st.term);
                        if sim >= input.limit {
                            let output = SearchOutput {
                                row_id: line.identifier.clone(),
                                search_term: st.term.clone(),
                                matched_term: w,
                                similarity: sim,
                                metadata: st.metadata.to_owned(),
                                search_field: line.field.to_owned(),
                            };
                            Some(output)
                        } else {
                            None
                        }
                    })
                    .collect_vec()
            })
        })
        .collect();
    outputs
}

fn load_stdin_lines() -> Vec<SearchLines> {
    let stdin_lock = std::io::stdin();
    BufReader::new(stdin_lock)
        .lines()
        .enumerate()
        .progress_with(initialize_spinner_style("Scanning stdin lines..."))
        .filter_map(|(line_num, line)| {
            if let Ok(line) = line {
                Some(SearchLines {
                    words: remove_symbols_except_dash(line)
                        .to_ascii_uppercase()
                        .split_ascii_whitespace()
                        .map(|s| s.to_string())
                        .collect_vec(),
                    identifier: line_num.to_string(),
                    field: "stdin".to_string(),
                })
            } else {
                None
            }
        })
        .collect_vec()
}

fn write_outputs(outputs: Vec<SearchOutput>) -> Result<()> {
    let mut wtr = csv::Writer::from_path("output.csv")
        .wrap_err("Couldn't create csv output file")
        .suggestion("Check permissions")?;

    for output in outputs {
        wtr.serialize(output).unwrap();
    }
    Ok(())
}

pub fn pipe_searcher(args: PipeArgs) -> Result<()> {
    let lines = load_stdin_lines();
    dbg!(&lines[0..5]);

    let search_terms = load_search_terms(args.terms_file)?;
    dbg!(&search_terms);

    let search_input = SearchInput {
        lines,
        search_terms,
        limit: args.limit,
    };
    let outputs = search(search_input);
    dbg!(&outputs);

    write_outputs(outputs)?;
    Ok(())
}

fn load_datafile_lines(args: &SearchArgs) -> Result<Vec<SearchLines>> {
    let mut rdr = csv::Reader::from_path(&args.data_file)?;
    let headers = pull_header(&mut rdr)?;

    let col_indices = find_column_indices(&headers, &args.search_cols)?;

    let lines = rdr
        .records()
        .enumerate()
        .progress_with(initialize_spinner_style("Loading datafile lines..."))
        .flat_map(|(i, record)| {
            let row = record
                .wrap_err("Unable to read row")
                .suggestion("Please check your csv file formatting and structure")
                .unwrap();
            let identifier = if let Some(id_col_name) = &args.id_col {
                let idc = id_col_name.to_ascii_uppercase();
                match &headers.iter().position(|h| *h == idc) {
                    Some(h_index) => row.get(*h_index).unwrap().to_owned(),
                    None => i.to_string(),
                }
            } else {
                i.to_string()
            };
            col_indices
                .iter()
                .map(|ci| {
                    let cell = row.get(*ci).unwrap();
                    let words = remove_symbols_except_dash(cell.to_owned())
                        .to_ascii_uppercase()
                        .split_ascii_whitespace()
                        .map(|s| s.to_string())
                        .collect_vec();
                    SearchLines {
                        words,
                        field: headers[*ci].clone(),
                        identifier: identifier.clone(),
                    }
                })
                .collect_vec()
        })
        .collect_vec();
    Ok(lines)
}

pub fn datafile_searcher(args: SearchArgs) -> Result<()> {
    let lines = load_datafile_lines(&args)?;
    dbg!(&lines[0..5]);

    let search_terms = load_search_terms(args.terms_file)?;
    dbg!(&search_terms);

    let search_input = SearchInput {
        lines,
        search_terms,
        limit: args.limit,
    };
    let outputs = search(search_input);
    dbg!(&outputs);

    write_outputs(outputs)?;
    Ok(())
}
