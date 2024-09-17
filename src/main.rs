mod cli;
mod file_reader;
mod file_walker;
mod processor;
mod test_utils;

use clap::Parser;
use cli::Cli;
use processor::process_files;
use std::process;

fn main() {
    let args = Cli::parse();

    match process_files(&args.root) {
        Ok(content) => {
            // Print the concatenated content to stdout
            println!("{}", content);
        }
        Err(e) => {
            // Print the error to stderr and exit with a non-zero code
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}
