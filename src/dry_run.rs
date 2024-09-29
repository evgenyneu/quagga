use crate::cli::Cli;
use crate::file_walker::get_all_files;
use std::error::Error;
use std::path::PathBuf;

/// Concatenates file paths for the --dry-run functionality.
/// Returns a single string of file paths separated by newlines.
///
/// # Arguments
///
/// * `cli` - Command line arguments.
/// * `piped_paths` - An optional `Vec<PathBuf>` representing a list of file paths the user
///                  has piped in via stdin. When present, the program will simply
///                 concatenate the paths and return them.
///
/// # Returns
///
/// `Ok(String)` containing the concatenated file paths, or an error if something goes wrong.
pub fn concatenate_file_paths(
    cli: &Cli,
    piped_paths: Option<Vec<PathBuf>>,
) -> Result<String, Box<dyn Error>> {
    let mut files = if let Some(paths) = piped_paths {
        paths
    } else {
        get_all_files(cli)?
    };

    files.sort();

    let file_paths: Vec<String> = files
        .iter()
        .map(|file| file.display().to_string())
        .collect();

    Ok(file_paths.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::temp_dir::TempDir;
    use clap::Parser;

    #[test]
    fn test_concatenate_file_paths_run_with_multiple_files() {
        let td = TempDir::new().unwrap();
        let path1 = td.mkfile("file1.txt");
        let path3 = td.mkfile("file3.txt");
        let path2 = td.mkfile("file2.txt");

        let mut cli = Cli::parse_from(&["quagga"]);
        cli.root = td.path_buf();

        let output = concatenate_file_paths(&cli, None).unwrap();

        let expected = format!(
            "{}\n{}\n{}",
            path1.display(),
            path2.display(),
            path3.display()
        );

        assert_eq!(output, expected);
    }

    #[test]
    fn test_concatenate_file_paths_run_with_piped_files_and_not_from_dir() {
        // Create files in root dir that should be ignored
        let td = TempDir::new().unwrap();
        td.mkfile("file1.txt");
        td.mkfile("file2.txt");

        let mut cli = Cli::parse_from(&["quagga"]);
        cli.root = td.path_buf();

        let piped_files = vec![PathBuf::from("piped1.txt"), PathBuf::from("piped2.txt")];

        let output = concatenate_file_paths(&cli, Some(piped_files)).unwrap();

        assert_eq!(output, "piped1.txt\npiped2.txt");
    }

    #[test]
    fn test_concatenate_file_paths_run_with_no_files() {
        let td = TempDir::new().unwrap();

        let mut cli = Cli::parse_from(&["quagga"]);
        cli.root = td.path_buf();

        let output = concatenate_file_paths(&cli, None).unwrap();
        assert_eq!(output, "");
    }
}
