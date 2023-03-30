mod io;

use std::{
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
    str::FromStr,
};

use cached::proc_macro::cached;
use cached::SizedCache;
use color_eyre::{
    eyre::{eyre, Context, ContextCompat},
    Help, Report,
};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressIterator, ProgressStyle};
use itertools::Itertools;
use rayon::prelude::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};
use serde::{Deserialize, Serialize};

// TODO: switch to generic type params not impl
// TODO: use lifetimes where possible to avoid cloning
// TODO: setup output type without metadata

fn max_ngram_size(terms: &[SearchTerm]) -> usize {
    terms
        .iter()
        .map(|t| t.word.split_whitespace().count())
        .max()
        .unwrap()
}

pub fn remove_symbols_except_dash(s: &str) -> String {
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
    ss.trim().to_ascii_uppercase()
}

fn identify_words(
    data: Vec<(String, String, String)>,
) -> HashMap<String, HashMap<String, HashSet<String>>> {
    let mut identified_words: HashMap<String, HashMap<String, HashSet<String>>> = HashMap::new();
    for (word, id, col) in data
        .iter()
        .progress_with(ProgressBar::new_spinner().with_message("Building word index..."))
    {
        identified_words
            .entry(word.to_string())
            .and_modify(|word_info| {
                word_info
                    .entry(col.to_string())
                    .and_modify(|ids| {
                        ids.insert(id.clone());
                    })
                    .or_insert_with(|| {
                        let mut ids = HashSet::new();
                        ids.insert(id.clone());
                        ids
                    });
            })
            .or_insert_with(|| {
                let mut info: HashMap<String, HashSet<String>> = HashMap::new();
                info.entry(col.to_string())
                    .and_modify(|ids| {
                        ids.insert(id.clone());
                    })
                    .or_insert_with(|| {
                        let mut ids = HashSet::new();
                        ids.insert(id.clone());
                        ids
                    });
                info
            });
    }
    identified_words
}

pub fn initialize_progress_with_style(kind: &str) -> Result<ProgressStyle, Report> {
    let progress_enum = ProgressKind::from_str(kind)?;
    match progress_enum {
        ProgressKind::Spinner => Ok(ProgressStyle::default_spinner()),
        ProgressKind::Bar => Ok(ProgressStyle::with_template(
            "({elapsed_precise}) [{bar:.cyan/blue}] {pos}/{len} ({eta_precise})",
        )
        .wrap_err("Couldn't initialize progress bar. Please file bug report.")?
        .progress_chars("#>-")),
    }
}

pub fn find_matches<'a, W: Iterator<Item = &'a String> + Clone>(
    terms: &'a [SearchTerm],
    words: W,
    threshold: f64,
    pb_style: &ProgressStyle,
) -> Vec<(&'a SearchTerm, &'a String, f64)> {
    terms
        .iter()
        .cartesian_product(words.into_iter())
        .collect_vec()
        .par_iter()
        .progress_with_style(pb_style.to_owned())
        .filter_map(|(term, word)| {
            let sim = strsim::jaro_winkler(&term.word, word);
            if sim >= threshold {
                Some((*term, *word, sim))
            } else {
                None
            }
        })
        .collect()
}

pub fn assemble_standard_output<'a>(
    matches: &'a [(&'a SearchTerm, &'a String, f64)],
    pb_style: &ProgressStyle,
) -> Vec<StandardOutput> {
    matches
        .par_iter()
        .progress_with_style(pb_style.to_owned())
        .map(|(term, word, sim)| StandardOutput {
            target: term.word.to_string(),
            match_: word.to_string(),
            metadata: term.metadata.to_owned(),
            sim: *sim,
        })
        .collect()
}

pub fn assemble_identified_output(
    matches: &[(&SearchTerm, &String, f64)],
    lookup: &HashMap<String, HashMap<String, HashSet<String>>>,
) -> Vec<IdentifiedOutput> {
    // lookup is map of word -> map of col name -> set of ids
    matches
        .iter()
        .flat_map(|(term, word, sim)| {
            let word_entry = lookup
                .get(*word)
                .wrap_err("Unable to lookup word in map")
                .unwrap();
            let word_cols = word_entry.keys().collect_vec();
            word_cols
                .iter()
                .flat_map(|col| {
                    let word_ids = word_entry
                        .get(*col)
                        .wrap_err("Unable to lookup col in map")
                        .unwrap();
                    word_ids.iter().map(|id| IdentifiedOutput {
                        row_id: id.to_string(),
                        target: term.word.to_owned(),
                        match_: word.to_string(),
                        sim: *sim,
                        metadata: term.metadata.to_owned(),
                        column: col.to_string(),
                    })
                })
                .collect_vec()
        })
        .collect()
}

////////////////////////////////////////////////////////////
///

#[cfg(test)]
mod test {

    use std::path::PathBuf;

    use super::*;
    use csv::StringRecord;
    use indicatif::ProgressIterator;
    use itertools::Itertools;
    use serde_json::to_string;
    use strsim::jaro_winkler;

    #[test]
    fn test_words() {
        // so i think this is what we would want to do...
        // but i think to avoid re-looping we should bucket the
        // target words by length, and then only loop over
        // the buckets
        let targets = vec!["he ho", "hell"];
        let words: Vec<&str> = "hi hello hey howdy hola he hell hi hello hi hello hey aloha"
            .split_whitespace()
            .collect();
        let ngrams = words.windows(2).unique().map(|x| x.join(" ")).collect_vec();
        dbg!(ngrams);
    }

    #[test]
    fn test_jaro() {
        // so basically, if you miss the first letter, you have to
        // get the next 13 letters right to get a 0.95.
        // this means we can do a simply "if" check for first letter matching if
        // the words are shorter than 14 characters
        // not quite apparently this is 0.9583
        let j = jaro_winkler("ALCOHOL", "IALCOHOL");
        dbg!(j);
    }

    #[test]
    fn test_single_window() {
        let data = vec![1, 2];
        let windows = data.windows(3).collect_vec();
        dbg!(windows);
    }

    #[test]
    fn test_cols() {
        use cached::proc_macro::cached;
        use cached::SizedCache;
        use itertools::Itertools;

        #[cached(
            type = "SizedCache<String, f64>",
            create = "{ SizedCache::with_size(1_000) }",
            convert = r#"{ format!("{}{}", a, b) }"#
        )]
        fn jw(a: &str, b: &str) -> f64 {
            strsim::jaro_winkler(a, b)
        }
        // its seeming like we are leaning to building a "source data" type
        // similar to previous input types
        // this doesn't hold our options, but the actual data we pulled from the
        // data file
        let path = PathBuf::from_str("./data/records.csv").unwrap();
        let mut f = super::initialize_dataset_reader(&path).unwrap();
        let spath = PathBuf::from_str("./data/search_terms2.csv").unwrap();
        let search_terms = load_search_terms(&spath).unwrap();
        let cols = vec![7, 8, 9, 10, 11];
        let id_col = Some(1);
        let mut grid_data: Vec<(String, usize, SearchTerm, String, f64)> = Vec::new();
        for (i, row) in f.records().enumerate() {
            let result = row.unwrap();
            let id_val = if let Some(id_) = id_col {
                result.get(id_).unwrap().to_string()
            } else {
                i.to_string()
            };
            for term in &search_terms {
                let gram_length = term.word.split_whitespace().count();
                for c in &cols {
                    let cell = result.get(*c).unwrap();
                    let clean = super::remove_symbols_except_dash(cell);
                    let words: Vec<String> = clean
                        .split_whitespace()
                        .map(|x| x.to_string())
                        .collect_vec();
                    let grams = if gram_length == 1 {
                        let r = words.iter().map(|x| x.to_owned()).unique().collect_vec();
                        r
                    } else {
                        let windows = words.windows(gram_length).collect_vec();
                        let r = windows.iter().map(|w| w.join(" ")).unique().collect_vec();
                        r
                    };
                    for gram in grams {
                        let sim = jw(&term.word, &gram);
                        if sim > 0.95 {
                            grid_data.push((id_val.clone(), *c, term.clone(), gram, sim))
                        }
                    }
                }
            }
        }
        dbg!(&grid_data[200..220]);
    }
    #[test]
    fn test_map_order() {
        let x = vec![1, 2, 3];
        let y = x.iter().map(|a| a * 2).collect_vec();
        dbg!(y);
    }

    #[test]
    fn test_combosss() {
        let mut rdr = initialize_dataset_reader(&Path::new("./data/records.csv")).unwrap();
        let terms_path = PathBuf::from("./data/search_terms.csv");
        let search_terms = load_search_terms(&terms_path).unwrap();
        let crosses = rdr
            .records()
            // don't have to enumerate since the position is available via
            // stringrecord.position.unwrap.line()
            // .enumerate()
            .map(|r| r.unwrap())
            .cartesian_product(&search_terms)
            .collect_vec();
        println!("Number of crosses: {}", crosses.len());
        println!("Size of crosses: {:#?}", std::mem::size_of_val(&*crosses));

        println!("{:#?}", &crosses[..2])
    }

    struct Input {
        num: usize,
        name: String,
    }

    impl From<StringRecord> for Input {
        fn from(value: StringRecord) -> Self {
            Input {
                num: 12,
                name: "nick".to_string(),
            }
        }
    }
}
