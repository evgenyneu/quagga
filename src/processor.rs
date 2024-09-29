use crate::cli::Cli;
use crate::dry_run::concatenate_file_paths;
use std::error::Error;
use std::path::PathBuf;

use crate::file_reader::read_and_concatenate_files;
use crate::file_walker::get_all_files;
use crate::template::template::{read_and_validate_template, TemplateParts};

/// The function called by `main.rs` that processes files based on provided command line options.
///
/// # Arguments
///
/// * `cli` - Command line arguments.
/// * `piped_paths` - An optional `Vec<PathBuf>` representing a list of file paths the user
///                  has piped in via stdin. When present, the program will process the
///                  files in the list instead of walking the root directory.
///
/// # Returns
///
/// A `Result` containing the concatenated contents of the files as a `String` if successful,
/// or an error if any operation fails.
pub fn run(cli: &Cli, piped_paths: Option<Vec<PathBuf>>) -> Result<String, Box<dyn Error>> {
    if cli.dry_run {
        return concatenate_file_paths(cli, piped_paths);
    }

    let template = read_and_validate_template(cli.template.clone())?;

    if let Some(path_list) = piped_paths {
        return read_and_concatenate_files(path_list, template)
            .map_err(|e| Box::new(e) as Box<dyn Error>);
    } else {
        return process_files(cli, template);
    }
}

/// Processes files starting from the given root path:
/// - Retrieves file paths by walking the root directory.
/// - Reads and concatenates their contents.
///
/// # Arguments
///
/// * `cli` - Command line arguments.
///
/// # Returns
///
/// A `Result` containing the concatenated contents of the files as a `String` if successful,
/// or an `io::Error` if an error occurs.
///
/// # Errors
///
/// This function will return an error if:
/// - Retrieving the list of files fails.
/// - Reading any of the files fails.
pub fn process_files(cli: &Cli, template: TemplateParts) -> Result<String, Box<dyn Error>> {
    let mut files = get_all_files(cli)?;
    files.sort();

    read_and_concatenate_files(files, template).map_err(|e| Box::new(e) as Box<dyn Error>)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::temp_dir::TempDir;
    use clap::Parser;

    #[test]
    fn test_run_dry() {
        let td = TempDir::new().unwrap();
        let path1 = td.mkfile("file1.txt");
        let path2 = td.mkfile("file2.txt");

        let mut cli = Cli::parse_from(&["test", "--dry-run"]);
        cli.root = td.path_buf();

        let result = run(&cli, None);

        assert!(result.is_ok());
        let expected = format!("{}\n{}", path1.display(), path2.display());
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_process_files_success() {
        let td = TempDir::new().unwrap();
        let file1_path = td.mkfile_with_contents("file1.txt", "Hello");
        let file2_path = td.mkfile_with_contents("file3.txt", "World!");

        let mut cli = Cli::parse_from(&["test"]);
        cli.root = td.path_buf();

        let template = TemplateParts {
            header: "Header".to_string(),
            item: "File: {{FILEPATH}}\nContent:\n{{CONTENT}}\n---".to_string(),
            footer: "Footer".to_string(),
        };

        let result = process_files(&cli, template);

        assert!(result.is_ok());

        let expected_output = format!(
            "\
Header
File: {}
Content:
Hello
---
File: {}
Content:
World!
---
Footer",
            file1_path.display(),
            file2_path.display()
        );

        assert_eq!(result.unwrap(), expected_output);
    }

    #[test]
    fn test_process_files_with_nonexistent_directory() {
        let mut cli = Cli::parse_from(&["test"]);
        cli.root = PathBuf::from("/path/to/nonexistent/directory");

        let result = process_files(&cli, TemplateParts::default());

        assert!(result.is_err());
    }
}
