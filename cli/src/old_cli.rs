//!
//!

fn greet(std_err: bool) {
    match std_err {
        true => {
            eprintln!();
            eprintln!("Welcome to the UK IPOP Fuzzy Drug Searcher!");
            eprintln!("===========================================");
            eprintln!();
            eprintln!("This program will search a datafile for matches to a list of terms. For more information, please consult the User Guide: https://github.com/UK-IPOP/drug-extraction or the `--help` menu.");
        }
        false => {
            println!();
            println!("Welcome to the UK IPOP Fuzzy Drug Searcher!");
            println!("===========================================");
            println!();
            println!("This program will search a datafile for matches to a list of terms. For more information, please consult the User Guide: https://github.com/UK-IPOP/drug-extraction or the `--help` menu.");
        }
    }
}

fn main() -> Result<(), Report> {
    color_eyre::install()?;

    let cli = Cli::parse();

    match cli.command {
        Commands::Standard(args) => run_standard_program(&args)?,
        Commands::Pipe(args) => run_pipe_program(&args)?,
        Commands::Interactive => interactive_wizard()?,
    }

    Ok(())
}
