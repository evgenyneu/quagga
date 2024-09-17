mod cli;
mod file_walker;
mod processor;
mod test_utils;

use clap::Parser;
use cli::Cli;
use processor::process_files;

fn main() {
    let args = Cli::parse();

    match process_files(&args.root) {
        Ok(content) => {
            println!("{}", content);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}
