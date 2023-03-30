use std::io::{BufRead, BufReader};

use clap::Parser; // this trait required for Cli::parse()
use color_eyre::{eyre::eyre, Help, Result};
use rayon::prelude::{ParallelBridge, ParallelIterator};

mod in_and_out;
mod options;
mod searchers;
mod types;

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = options::Cli::parse();
    // this is a great pattern since it validates CLI arguments
    // FIRST before actually reading std-in

    // pay attention since this is confusing and
    // reverse of what it seems
    if atty::isnt(atty::Stream::Stdin) {
        // STD
        println!("std");
        dbg!(&cli);
        match cli.command {
            options::Commands::Pipe(args) => {
                dbg!(&args);
                searchers::pipe_searcher(args)?;
            }
            _ => {
                return Err(
                    eyre!("Piped-input only acceptable when using the `pipe` subcommand")
                        .suggestion("Try running `fuzzy-drug-extractor pipe` with your piped data"),
                )
            }
        };
    } else {
        // this is a great pattern because
        // no-STD
        println!("no std");

        dbg!(&cli);
        match cli.command {
            options::Commands::Interactive => println!("interactive search"),
            options::Commands::Search(args) => {
                dbg!(&args);
                searchers::datafile_searcher(args)?;
            }
            _ => {
                return Err(eyre!(
                    "Non-piped-input only acceptable when using the `search` subcommand"
                )
                .suggestion("Try running `fuzzy-drug-extractor search` with your datafile"))
            }
        }
    }
    Ok(())
}
