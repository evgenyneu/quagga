mod cli;
mod file_walker;
mod test_utils;

use clap::Parser;
use cli::Cli;
use file_walker::get_all_files;

fn main() {
    let args = Cli::parse();

    // Call the function to get all files starting from the root directory
    let files = get_all_files(&args.root);

    // For now, just print the file paths
    for file in files {
        println!("{:?}", file);
    }
}
