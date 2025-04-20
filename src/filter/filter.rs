use crate::cli::Cli;
use crate::file::file_reader::FileContent;
use crate::filter::remove_comments::remove_comments_from_file;

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
pub fn filter_lines_in_files(file_contents: &Vec<FileContent>, cli: &Cli) -> Vec<FileContent> {
    file_contents
        .iter()
        .cloned()
        .map(|file| filter_lines_in_single_file(file, cli))
        .collect()
}

/// Filters lines in a single file according to CLI options.
///
/// # Arguments
///
/// * `file_content` - The FileContent struct to filter.
/// * `cli` - Reference to the Cli options.
///
/// # Returns
///
/// The FileContent struct, possibly filtered according to CLI flags.
pub fn filter_lines_in_single_file(file_content: FileContent, cli: &Cli) -> FileContent {
    if cli.remove_comments {
        let filtered_content = remove_comments_from_file(file_content.clone());

        FileContent {
            path: file_content.path,
            content: filtered_content,
        }
    } else {
        file_content
    }
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

    #[test]
    fn test_filter_lines_in_single_file_noop() {
        let input = FileContent {
            path: PathBuf::from("bar.py"),
            content: String::from("print('hello world')"),
        };

        let cli = Cli::parse_from(&["test"]);

        let _output = filter_lines_in_single_file(input.clone(), &cli);
    }
}
