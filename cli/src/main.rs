use clap::{Args, Parser, Subcommand};
use color_eyre::{
    eyre::{eyre, Context},
    Result,
};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Select};
use drug_extraction_cli::run_searcher;
use std::path::PathBuf;

fn welcome() {
    println!("Welcome to the UK-IPOP Drug Extraction tool.");
    println!("------------------------------------------");
}

#[derive(Parser, Debug)]
#[command(
    author,
    about,
    version,
    long_about = "A fuzzy search tool for extracting drug mentions from large datasets."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Interactive configuration wizard
    Interactive,
    /// File-IO
    Search(SearchArgs),
}

#[derive(Args, Debug)]
struct SearchArgs {
    /// The file with your search terms
    #[arg(short = 't', long, default_value = "search_terms.csv")]
    terms_file: PathBuf,

    /// The dataset file to search
    #[arg(short = 'd', long)]
    data_file: PathBuf,

    /// The column name(s) in the dataset to search
    #[arg(short = 'c', long, num_args = 1, required = true)]
    search_cols: Vec<String>,

    /// The column name in the dataset to keep as identifier [optional]
    #[arg(short, long)]
    id_col: Option<String>,
}

/// Interactive configuration wizard
fn interactive_wizard() -> Result<SearchArgs> {
    let theme = ColorfulTheme::default();

    let terms_file: PathBuf = Input::<String>::with_theme(&theme)
        .with_prompt("What is the path to the search terms file?")
        .default("search_terms.csv".to_string())
        .interact_text()?
        .into();

    // confirm that the file exists
    if !terms_file.exists() {
        return Err(eyre!("The file {} does not exist.", terms_file.display()));
    }

    let data_file: PathBuf = Input::<String>::with_theme(&theme)
        .with_prompt("What is the path to the data file?")
        .interact_text()?
        .into();

    // confirm that the file exists
    if !data_file.exists() {
        return Err(eyre!("The file {} does not exist.", data_file.display()));
    }

    let headers: Vec<String> = csv::Reader::from_path(&data_file)
        .wrap_err("Could not open file")?
        .headers()
        .wrap_err("Could not read headers for datafile")?
        .clone()
        .iter()
        .map(|h| h.to_ascii_uppercase())
        .collect();

    let search_cols = MultiSelect::with_theme(&theme)
        .with_prompt(">Which column(s) do you want to search? \nMultiselect with <Space> and then <Enter> to continue.")
        .items(&headers)
        .interact()?;

    if search_cols.is_empty() {
        println!("You must select at least one column to search.");
        println!("Use the arrow keys to select the columns you want to search.");
        println!("Press `Space` to select and unselect columns and `Enter` to continue.");
        std::process::exit(1);
    }
    // TODO: this seems backwards going from the index to the name...
    let search_cols = search_cols
        .iter()
        .map(|&x| headers[x].to_string())
        .collect::<Vec<String>>();

    let has_id_col = Confirm::with_theme(&theme)
        .with_prompt("Do you want to use an ID column?")
        .default(false)
        .interact()?;

    let id_col = if has_id_col {
        let id_col_index = Select::with_theme(&theme)
            .with_prompt("Which column do you want to use as the ID column?")
            .items(&headers)
            .interact()?;
        Some(headers[id_col_index].to_owned())
    } else {
        None
    };

    let args = SearchArgs {
        terms_file,
        data_file,
        id_col,
        search_cols,
    };

    Ok(args)
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    welcome();

    match cli.command {
        Commands::Interactive => {
            let args = interactive_wizard()?;
            run_searcher(
                args.data_file,
                args.terms_file,
                args.search_cols,
                args.id_col,
            )?;
        }
        Commands::Search(args) => {
            run_searcher(
                args.data_file,
                args.terms_file,
                args.search_cols,
                args.id_col,
            )?;
        }
    }
    Ok(())
}
