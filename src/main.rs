pub mod binary_detector;
mod cli;
mod file_reader;
mod file_walker;
mod processor;
mod test_utils;

use clap::Parser;
use cli::Cli;
use processor::{process_files, process_input_paths};
use std::io::{self, BufRead, IsTerminal};
use std::process;

fn main() {
    let args = Cli::parse();

    if io::stdin().is_terminal() {
        match process_files(&args.root) {
            Ok(content) => {
                println!("{}", content);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
    } else {
        // The file paths were piped in, just read and combine the files
        let stdin = io::stdin();
        let paths: Vec<String> = stdin.lock().lines().collect::<Result<_, _>>().unwrap();

        match process_input_paths(paths) {
            Ok(content) => {
                // Print the concatenated content to stdout
                println!("{}", content);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
    }
}
