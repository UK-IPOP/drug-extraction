use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    author,
    about,
    version,
    long_about = "A fuzzy search tool for extracting drug mentions from large datasets."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Interactive configuration wizard
    Interactive,
    /// File-IO
    Search(SearchArgs),
    /// Piped-IO
    Pipe(PipeArgs),
}

#[derive(Args, Debug)]
pub struct SearchArgs {
    /// The file with your search terms
    #[arg(short, long, default_value = "search_terms.csv")]
    pub terms_file: PathBuf,

    /// The dataset file to search
    #[arg(short, long)]
    pub data_file: PathBuf,

    /// The column name(s) in the dataset to search
    #[arg(short, long, num_args = 1, required = true)]
    pub search_cols: Vec<String>,

    /// The column name in the dataset to keep as identifier
    #[arg(short, long)]
    pub id_col: Option<String>,

    /// Minimum similarity for match (0.0 - 1.0)
    #[arg(short, long, default_value_t = 0.95)]
    pub limit: f64,
}

#[derive(Args, Debug)]
pub struct PipeArgs {
    /// The file with your search terms
    #[arg(short, long, default_value = "search_terms.csv")]
    pub terms_file: PathBuf,

    /// Minimum similarity for match (0.0 - 1.0)
    #[arg(short, long, default_value_t = 0.95)]
    pub limit: f64,
}
