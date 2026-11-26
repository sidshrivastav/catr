use clap::{Parser, Subcommand};
use std::{
    fs::File,
    io::{self, Write},
    path::PathBuf,
};

#[derive(Parser)]
#[command(name = "catr", version, about = "cat but rust")]
struct Cli {
    /// Files to concatenate. Use '-' for standard input.
    #[arg(value_name = "FILE")]
    files: Vec<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    // Future subcommands go here.
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(command) => match command {},
        None => cat(&cli.files),
    }
}

fn cat(files: &[PathBuf]) -> io::Result<()> {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();

    let default = [PathBuf::from("-")];

    let files = if files.is_empty() {
        &default[..]
    } else {
        files
    };

    for path in files {
        if path.as_os_str() == "-" {
            io::copy(&mut stdin, &mut stdout)?;
        } else {
            io::copy(&mut File::open(path)?, &mut stdout)?;
        }
    }

    stdout.flush()
}