use crate::cli::Cli;
use crate::walk::binary_detector::is_valid_text_file;
use crate::walk::contain::file_contains_text;
use crate::walk::quagga_ignore::add_quagga_ignore_files;
use crate::walk::walk_overrides::build_overrides;
use ignore::WalkBuilder;
use std::error::Error;
use std::path::PathBuf;

/// Walks through the directory tree starting from `root` and collects all paths
/// to text files for the output prompt.
///
/// # Arguments
///
/// * `cli` - Command line arguments.
///
/// # Returns
///
/// * `Ok(Vec<PathBuf>)` containing the paths to text files for the output prompt.
/// * `Err<Box<dyn Error>>` if an error occurs during directory traversal or file reading.
pub fn get_all_files(cli: &Cli) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let overrides = build_overrides(cli)?;
    let mut walker_builder = WalkBuilder::new(&cli.root);
    walker_builder.overrides(overrides);
    // walker_builder.git_ignore(!cli.no_gitignore);

    if !cli.no_quagga_ignore {
        add_quagga_ignore_files(&mut walker_builder, cli.root.clone(), None)?;
    }

    let walker = walker_builder.build();
    let mut files = Vec::new();

    for entry in walker {
        let entry = entry?;
        let path = entry.path().to_path_buf();

        if should_include_path(&path, cli)? {
            files.push(path);
        }
    }

    Ok(files)
}

/// Determines whether a path should be included in the output prompt.
///
/// # Arguments
///
/// * `path` - The path to evaluate.
/// * `cli` - Command line arguments.
///
/// # Returns
///
/// * `Ok(true)` if the file should be included.
/// * `Ok(false)` if the file should be skipped.
/// * `Err<Box<dyn Error>>` if an error occurs during evaluation.
fn should_include_path(path: &PathBuf, cli: &Cli) -> Result<bool, Box<dyn Error>> {
    if !path.is_file() {
        return Ok(false);
    }

    if !is_valid_text_file(path.clone())? {
        return Ok(false);
    }

    // If `--contain` option is used, check if file contains the specified texts
    if !cli.contain.is_empty() && !file_contains_text(path, &cli.contain)? {
        return Ok(false);
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::temp_dir::TempDir;
    use clap::Parser;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_get_all_files() {
        let td = TempDir::new().unwrap();
        td.mkdir("subdir");
        td.mkfile("file1.txt");
        td.mkfile("file2.txt");
        td.mkfile(".hidden");
        td.mkfile("subdir/file3.txt");

        let mut cli = Cli::parse_from(&["test"]);
        cli.root = td.path_buf();

        let result = get_all_files(&cli);
        assert!(result.is_ok());

        let files = result.unwrap();
        assert_eq!(files.len(), 3);

        td.assert_contains(&files, "file1.txt");
        td.assert_contains(&files, "file2.txt");
        td.assert_contains(&files, "subdir/file3.txt");

        // Ensure no directories are included
        td.assert_not_contains(&files, "subdir");

        // Ensure hidden files are included
        td.assert_not_contains(&files, ".hidden");
    }

    #[test]
    fn test_get_all_files_filters_binary_files() {
        let td = TempDir::new().unwrap();
        td.mkfile_with_contents("file1.txt", "fn main() {}");
        td.mkfile_with_contents("file2.rs", "println!(\"Hello, world!\");");

        // Create a binary file
        let binary_file_path = td.path().join("binary.bin");
        let mut binary_file = File::create(&binary_file_path).unwrap();
        let binary_content = [0x00, 0xFF, 0x00, 0xFF];
        binary_file.write_all(&binary_content).unwrap();

        let mut cli = Cli::parse_from(&["test"]);
        cli.root = td.path_buf();

        let result = get_all_files(&cli);

        assert!(result.is_ok());

        let files = result.unwrap();

        let file_names: Vec<String> = files
            .iter()
            .map(|path| path.file_name().unwrap().to_string_lossy().into_owned())
            .collect();

        assert!(file_names.contains(&"file1.txt".to_string()));
        assert!(file_names.contains(&"file2.rs".to_string()));

        // Assert that binary file is not included
        assert!(!file_names.contains(&"binary.bin".to_string()));
    }

    #[test]
    fn test_get_all_files_with_no_files() {
        let td = TempDir::new().unwrap();
        let mut cli = Cli::parse_from(&["test"]);
        cli.root = td.path_buf();

        let result = get_all_files(&cli);

        assert!(result.is_ok());
        let files = result.unwrap();
        assert!(files.is_empty());
    }

    #[test]
    fn test_get_all_files_with_nonexistent_directory() {
        let mut cli = Cli::parse_from(&["test"]);
        cli.root = PathBuf::from("/path/to/nonexistent/directory");
        let result = get_all_files(&cli);

        assert!(result.is_err());
    }

    #[test]
    fn test_get_all_files_with_file_read_error() {
        let td = TempDir::new().unwrap();
        let file_path = td.mkfile_with_contents("file1.txt", "fn main() {}");

        // Set the file permissions to simulate a read error (e.g., remove read permissions)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&file_path).unwrap().permissions();
            perms.set_mode(0o000); // No permissions
            std::fs::set_permissions(&file_path, perms).unwrap();
        }

        let mut cli = Cli::parse_from(&["test"]);
        cli.root = td.path_buf();

        let result = get_all_files(&cli);

        assert!(result.is_err());
    }

    #[test]
    fn test_get_all_files_respects_quagga_ignore() {
        let td = TempDir::new().unwrap();
        td.mkfile_with_contents("file1.txt", "Hello");
        td.mkfile_with_contents("file2.md", "World!");
        td.mkfile_with_contents(".quagga_ignore", "*.md");

        let mut cli = Cli::parse_from(&["test"]);
        cli.root = td.path_buf();

        let result = get_all_files(&cli);
        assert!(result.is_ok());

        let files = result.unwrap();
        assert_eq!(files.len(), 1);

        td.assert_contains(&files, "file1.txt");
        td.assert_not_contains(&files, "file2.md"); // Ignored in .quagga_ignore
    }

    #[test]
    fn test_get_all_files_ignores_quagga_ignore_when_flag_is_set() {
        let td = TempDir::new().unwrap();
        td.mkfile_with_contents("file1.txt", "Hello");
        td.mkfile_with_contents("file2.md", "World!");
        td.mkfile_with_contents(".quagga_ignore", "*.md");

        let mut cli = Cli::parse_from(&["test", "--no-quagga-ignore"]);
        cli.root = td.path_buf();

        let result = get_all_files(&cli);
        assert!(result.is_ok());

        let files = result.unwrap();
        assert_eq!(files.len(), 2);

        // Both files should be included because --no-quagga-ignore is passed
        td.assert_contains(&files, "file1.txt");
        td.assert_contains(&files, "file2.md");
    }

    #[test]
    fn test_get_all_files_with_contain_option() {
        let td = TempDir::new().unwrap();
        let file1 = td.mkfile_with_contents("file1.txt", "This is a test file.");
        td.mkfile_with_contents("file2.txt", "Another sample.");

        let mut cli = Cli::parse_from(&["test", "--contain", "test"]);
        cli.root = td.path_buf();

        let result = get_all_files(&cli).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], file1);
    }
}
