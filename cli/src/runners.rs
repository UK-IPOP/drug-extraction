// !

use crate::options::{Cli, Commands, PipeOptions, StandardOptions};
use clap::Parser;
use color_eyre::Result;

pub fn run_standard_program(args: &StandardOptions) -> Result<()> {
    // its seeming like we are leaning to building a "source data" type
    // similar to previous input types
    // this doesn't hold our options, but the actual data we pulled from the
    // data file
    let data_path = PathBuf::from(&args.data_file);
    let mut rdr = lib::initialize_dataset_reader(&data_path).unwrap();
    let header = &rdr.headers().unwrap().clone();
    let terms_path = PathBuf::from(&args.terms_file);
    let search_terms = lib::load_search_terms(&terms_path).unwrap();
    let target_col_indices = &args
        .search_cols
        .iter()
        .map(|c| lib::find_column_index(header, c))
        .collect_vec();
    let mut grid_data: Vec<IdentifiedOutput> = Vec::new();

    let spinner = ProgressBar::new_spinner();
    rdr.records()
        .enumerate()
        .into_iter()
        .progress_with(spinner)
        .for_each(|(i, row)| {
            if i % 1_000 == 0 {
                println!("{}", i);
            }
            let result = row.unwrap();
            let id_val = if let Some(id_) = &args.id_col {
                let id_col_index = lib::find_column_index(header, id_);
                result.get(id_col_index).unwrap().to_string()
            } else {
                i.to_string()
            };
            for term in &search_terms {
                let gram_length = term.word.split_whitespace().count();
                // J is column index so we can re-index into target cols form args
                for (j, c) in target_col_indices.iter().enumerate() {
                    let cell = result.get(*c).unwrap();
                    let clean = lib::remove_symbols_except_dash(cell);
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
                            let ido = IdentifiedOutput {
                                row_id: id_val.clone(),
                                column: args.search_cols[j].to_owned(),
                                target: term.word.to_owned(),
                                metadata: term.metadata.to_owned(),
                                match_: gram.to_string(),
                                sim,
                            };
                            grid_data.push(ido);
                        }
                    }
                }
            }
        });

    write_output(&grid_data, &args.output_type);

    Ok(())
}

fn run_pipe_program(args: &PipeOptions) -> Result<()> {
    if atty::is(atty::Stream::Stdin) {
        println!("No data found on standard input. Please pipe data to this program.");
        println!("For example: `cat datafile.txt | extract-drugs pipe");
        println!("Alternatively, you can use the `standard` subcommand to read from a file.");
        std::process::exit(1);
    } else {
    }
    Ok(())
}

fn interactive_wizard() -> Result<(), Report> {
    greet(false);

    let theme = ColorfulTheme::default();

    let terms_file: PathBuf = Input::<String>::with_theme(&theme)
        .with_prompt("What is the path to the search terms file?")
        .default("search_terms.csv".to_string())
        .interact_text()?
        .into();

    let data_file: PathBuf = Input::<String>::with_theme(&theme)
        .with_prompt("What is the path to the data file?")
        .interact_text()?
        .into();

    let headers = lib::read_headers(&data_file)?;

    let search_cols = MultiSelect::with_theme(&theme)
        .with_prompt("Which column(s) do you want to search? (multi-select with Space)")
        .items(&headers)
        .interact()?;

    if search_cols.is_empty() {
        println!("You must select at least one column to search.");
        println!("Use the arrow keys to select the columns you want to search.");
        println!("Press `Space` to select and unselect columns and `Enter` to continue.");
        std::process::exit(1);
    }
    let search_cols = search_cols
        .iter()
        .map(|&x| headers[x].to_string())
        .collect::<Vec<String>>();

    let has_id_col = Confirm::with_theme(&theme)
        .with_prompt("Do you want to use an ID column?")
        .default(false)
        .interact()?;

    let id_col = if has_id_col {
        let id_col_index = FuzzySelect::with_theme(&theme)
            .with_prompt("Which column do you want to use as the ID column?")
            .items(&headers)
            .interact()?;
        Some(&headers[id_col_index])
    } else {
        None
    };

    let threshold = Input::<f64>::with_theme(&theme)
        .with_prompt("What is the threshold for matches?")
        .default(0.95)
        .interact()?;

    let output_type = Select::with_theme(&theme)
        .with_prompt("What type of output do you want?")
        .items(&["CSV", "JSONL"])
        .default(0)
        .interact()?;
    let output_type = OutputFileType::from_str(["csv", "jsonl"][output_type]).unwrap();

    let args = CLIStandardOptions {
        terms_file,
        data_file,
        id_col: id_col.cloned(),
        search_cols,
        threshold,
        output_type,
    };

    run_standard_program(&args)?;

    Ok(())
}
