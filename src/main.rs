use clap::Parser;
use std::{io, path::PathBuf};

#[derive(Parser)]
#[command(name = "catr", version, about = "cat but rust")]
struct Cli {
    /// Files to concatenate. Use '-' for standard input.
    #[arg(value_name = "FILE")]
    files: Vec<PathBuf>,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    catr::concatenate(
        &cli.files,
        &mut io::stdin().lock(),
        &mut io::stdout().lock(),
    )
}
