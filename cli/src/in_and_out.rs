// !
use color_eyre::{
    eyre::{eyre, Context},
    Help, Result,
};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressIterator};
use rayon::prelude::{ParallelBridge, ParallelIterator};
use serde::Serialize;
use std::{
    env,
    fs::File,
    io::{self, BufReader, Read},
    path::Path,
};

use crate::types::{DataSetInfo, SearchTerm};

//* Search Terms File Section *//

fn validate_terms_headers(reader: &mut csv::Reader<File>) -> Result<()> {
    match reader
        .headers()
        .wrap_err("Could not read headers")
        .suggestion("Check for valid UTF8")?
        .len()
    {
        usize::MIN..=0 => Err(eyre!("Search terms file must have at least 1 column")
            .suggestion("Check your search_terms file structure")),
        1..=2 => Ok(()),
        3..=usize::MAX => Err(eyre!("Search terms file must have 1-2 columns.")
            .suggestion("Check your search_terms file structure")),
        _ => Err(eyre!("Invalid search terms header").suggestion("Please try again")),
    }
}

pub fn load_search_terms<P: AsRef<Path>>(file_path: P) -> Result<Vec<SearchTerm>> {
    let fpath = env::current_dir()
        .wrap_err("Could not get current working directory.")?
        .join(file_path);
    let mut rdr = csv::Reader::from_path(fpath)
        .wrap_err("Could not find search terms file.")
        .suggestion("Check your provided filepath")?;

    validate_terms_headers(&mut rdr)?;

    let records: Result<Vec<SearchTerm>> = rdr
        .deserialize()
        .progress_with(ProgressBar::new_spinner().with_message("Loading terms..."))
        .map(|row| {
            let mut term: SearchTerm = row?;
            term.term = term.term.to_ascii_uppercase();
            Ok(term)
        })
        .collect();
    // let all_have_metadata = records.unwrap().into_iter().all(|s| s.metadata.is_some());
    // Ok((records.unwrap(), all_have_metadata))
    Ok(records.unwrap())
}

//* Dataset File Section *//

pub fn initialize_dataset<P: AsRef<Path>>(
    file_path: &P,
    search_cols: &[String],
) -> Result<DataSetInfo> {
    let fpath = env::current_dir()
        .wrap_err("Could not get current working directory. Please check your permissions.")?
        .join(file_path);
    let mut reader = csv::Reader::from_path(fpath)?;
    let header = pull_header(&mut reader)?;
    let search_column_indices = find_column_indices(&header, search_cols)?;
    Ok(DataSetInfo {
        reader,
        header,
        search_column_indices,
    })
}

pub fn pull_header(reader: &mut csv::Reader<File>) -> Result<Vec<String>> {
    Ok(reader
        .headers()
        .wrap_err("Could not read headers")
        .suggestion("Check for valid UTF8")?
        .iter()
        .map(|s| s.to_ascii_uppercase())
        .collect())
}

pub fn find_column_indices(header: &[String], search_cols: &[String]) -> Result<Vec<usize>> {
    search_cols
        .iter()
        .map(|c| {
            let sc = c.to_ascii_uppercase();
            let pos = header.iter().position(|hc| *hc == sc);
            match pos {
                Some(i) => Ok(i),
                None => Err(eyre!("Unable to find column {}", c)
                    .suggestion("Please check structure of your files")),
            }
        })
        .collect()
}

//* Standard Input / Output Section *//

pub fn read_stdin() -> BufReader<io::Stdin> {
    BufReader::new(io::stdin())
}

//* CSV File Output *//

pub fn write_csv<O: Serialize>(output: &[O]) -> Result<()> {
    let mut wtr = csv::Writer::from_path("output.csv")
        .wrap_err("Unable to create CSV output file.")
        .suggestion("Check permissions")?;
    let _ = output
        .iter()
        .progress_with(ProgressBar::new_spinner().with_message("Writing CSV..."))
        .map(|row| {
            wtr.serialize(row).wrap_err("Unable to serialize output")?;
            Ok(())
        })
        .collect::<Result<()>>();
    wtr.flush().wrap_err("Unable to flush output")?;
    Ok(())
}
