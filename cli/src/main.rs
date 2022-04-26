//! Primary application for drug extraction
//!
//!
use clap::Parser;
use std::error::Error;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = utils::Cli::parse();
    utils::run(cli)?;
    Ok(())
}
