pub mod binary_detector;
mod cli;
mod file_reader;
mod file_walker;
mod processor;
mod test_utils;
use std::path::PathBuf;

use clap::Parser;
use cli::Cli;
use processor::run;
use std::io::{self, BufRead, IsTerminal};
use std::process;

/// Main entry point for the application.
fn main() {
    let args = Cli::parse();
    let piped_paths = piped_paths();

    match run(args.root, piped_paths.clone()) {
        Ok(content) => {
            println!("{}", content);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

/// Reads file paths from stdin if they are piped in.
///
/// # Returns
///
/// A `Vec<PathBuf>` containing the paths read from stdin, or `None` if no paths are piped in.
fn piped_paths() -> Option<Vec<PathBuf>> {
    if io::stdin().is_terminal() {
        return None;
    }

    let stdin = io::stdin();

    return Some(
        stdin
            .lock()
            .lines()
            .filter_map(|line| line.ok())
            .map(PathBuf::from)
            .collect(),
    );
}
