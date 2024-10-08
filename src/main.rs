mod cli;
mod file;
mod info;
mod output;
mod processor;
mod template;
mod test_utils;
mod walk;
use clap::Parser;
use cli::Cli;
use processor::generate_prompt_and_output;
use std::io::{self, BufRead, IsTerminal};
use std::path::PathBuf;
use std::process;

/// Main entry point for the application.
fn main() {
    let args = Cli::parse();
    let piped_paths = piped_paths();

    match generate_prompt_and_output(&args, piped_paths.clone()) {
        Ok(_) => {}
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
        // Input is coming from the terminal, not from a pipe
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
