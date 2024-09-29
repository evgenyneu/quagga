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
/// * `paths` - An optional `Vec<PathBuf>` representing a list of file paths.
///             When present, the program will simply concatenate the paths and return them,
///             without traversing the root directory.
///
/// # Returns
///
/// `Ok(String)` containing the concatenated file paths, or an error if something goes wrong.
pub fn concatenate_file_paths(
    cli: &Cli,
    paths: Option<Vec<PathBuf>>,
) -> Result<String, Box<dyn Error>> {
    let files = if let Some(paths) = paths {
        paths
    } else {
        get_all_files(cli)?
    };

    Ok(format_file_paths(files))
}

fn format_file_paths(file_paths: Vec<PathBuf>) -> String {
    let mut sorted_paths = file_paths.clone();
    sorted_paths.sort();

    let file_paths: Vec<String> = sorted_paths
        .iter()
        .map(|file| file.display().to_string())
        .collect();

    file_paths.join("\n")
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

    #[test]
    fn test_format_file_paths() {
        let path1 = PathBuf::from("file1.txt");
        let path2 = PathBuf::from("file2.txt");
        let path3 = PathBuf::from("file3.txt");

        let files = vec![path3.clone(), path1.clone(), path2.clone()];
        let output = format_file_paths(files);

        let expected = format!(
            "{}\n{}\n{}",
            path1.display(),
            path2.display(),
            path3.display()
        );

        assert_eq!(output, expected);
    }
}
