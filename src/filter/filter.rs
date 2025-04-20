use crate::cli::Cli;
use crate::file::file_reader::FileContent;

/// Filters lines in the provided files according to CLI options.
///
/// # Arguments
///
/// * `file_contents` - A reference to a vector of FileContent structs.
/// * `cli` - Reference to the Cli options.
///
/// # Returns
///
/// A vector of FileContent structs, possibly filtered according to CLI flags.
pub fn filter_lines_in_files(file_contents: &Vec<FileContent>, _cli: &Cli) -> Vec<FileContent> {
    // Stub: currently returns the input unchanged.
    file_contents.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::Cli;
    use crate::file::file_reader::FileContent;
    use clap::Parser;
    use std::path::PathBuf;

    #[test]
    fn test_filter_lines_in_files_noop() {
        let input = vec![FileContent {
            path: PathBuf::from("foo.rs"),
            content: String::from("fn main() {}"),
        }];

        let cli = Cli::parse_from(&["test"]);

        let output = filter_lines_in_files(&input, &cli);

        assert_eq!(output.len(), 1);
    }
}
