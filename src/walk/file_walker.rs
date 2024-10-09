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
    let walker_builder = configure_walk_builder(cli)?;
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

/// Setup the `WalkBuilder` with the necessary configurations.
fn configure_walk_builder(cli: &Cli) -> Result<WalkBuilder, Box<dyn Error>> {
    let overrides = build_overrides(cli)?;
    let mut walker_builder = WalkBuilder::new(&cli.root);
    walker_builder.overrides(overrides);
    walker_builder.git_ignore(!cli.no_gitignore);
    walker_builder.max_depth(cli.max_depth);
    walker_builder.max_filesize(Some(cli.max_filesize));
    walker_builder.require_git(false); // Apply git-related gitignore rules even if .git directory is missing
    walker_builder.hidden(!cli.hidden);
    walker_builder.follow_links(cli.follow_links);

    if !cli.no_quagga_ignore {
        add_quagga_ignore_files(&mut walker_builder, cli.root.clone(), None)?;
    }

    Ok(walker_builder)
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

    if !cli.binary && !is_valid_text_file(path.clone())? {
        return Ok(false);
    }

    // If `--contain` option is used, check if file contains the specified texts
    if !cli.contain.is_empty() && !file_contains_text(path, &cli.contain, cli.binary)? {
        return Ok(false);
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::temp_dir::TempDir;
    use clap::Parser;
    use std::os::unix::fs as unix_fs;

    #[test]
    fn test_get_all_files() {
        let td = TempDir::new().unwrap();
        td.mkdir("subdir");
        td.mkfile("file1.txt");
        td.mkfile("file2.txt");
        td.mkfile(".hidden"); // Should be ignored
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
    }

    #[test]
    fn test_get_all_files_filters_binary_files() {
        let td = TempDir::new().unwrap();
        td.mkfile_with_contents("file1.txt", "fn main() {}");
        td.mkfile_with_contents("file2.rs", "println!(\"Hello, world!\");");
        td.mkfile_with_bytes("binary.bin", &[0x00, 0xFF, 0x00, 0xFF]);

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

    #[test]
    fn test_get_all_files_respect_gitingore() {
        let td = TempDir::new().unwrap();
        td.mkfile_with_contents("file1.txt", "Hello");
        td.mkfile_with_contents("file2.md", "World!");
        td.mkfile_with_contents(".gitignore", "*.md");

        let mut cli = Cli::parse_from(&["test"]);
        cli.root = td.path_buf();

        let result = get_all_files(&cli);

        assert!(result.is_ok());
        let files = result.unwrap();
        assert_eq!(files.len(), 1);
        td.assert_contains(&files, "file1.txt");
    }

    #[test]
    fn test_get_all_files_no_gitignore() {
        let td = TempDir::new().unwrap();
        td.mkfile_with_contents("file1.txt", "Hello");
        td.mkfile_with_contents("file2.md", "World!");
        td.mkfile_with_contents(".gitignore", "*.md");

        let mut cli = Cli::parse_from(&["test", "--no-gitignore"]);
        cli.root = td.path_buf();

        let result = get_all_files(&cli);

        assert!(result.is_ok());
        let files = result.unwrap();
        assert_eq!(files.len(), 2);
        td.assert_contains(&files, "file1.txt");
        td.assert_contains(&files, "file2.md");
    }

    #[test]
    fn test_get_all_files_max_depth() {
        let td = TempDir::new().unwrap();
        td.mkfile("file.txt");
        td.mkdir("dir1");
        td.mkfile("dir1/file1.txt");
        td.mkdir("dir1/dir2");
        td.mkfile("dir1/dir2/file2.txt");

        let mut cli = Cli::parse_from(&["test", "--max-depth", "2"]);
        cli.root = td.path_buf();

        let result = get_all_files(&cli);

        assert!(result.is_ok());
        let files = result.unwrap();
        assert_eq!(files.len(), 2);
        td.assert_contains(&files, "file.txt");
        td.assert_contains(&files, "dir1/file1.txt");
    }

    #[test]
    fn test_get_all_files_max_filesize() {
        let td = TempDir::new().unwrap();
        td.mkfile_with_contents("file_four_bytes.txt", "1234");
        td.mkfile_with_contents("file_five_bytes.txt", "12345");

        // Set the maximum file size to 4 bytes
        let mut cli = Cli::parse_from(&["test", "--max-filesize", "4"]);
        cli.root = td.path_buf();

        let result = get_all_files(&cli);

        assert!(result.is_ok());
        let files = result.unwrap();
        assert_eq!(files.len(), 1);
        td.assert_contains(&files, "file_four_bytes.txt");
    }

    #[test]
    fn test_should_include_path_ignore_binary_files() {
        let td = TempDir::new().unwrap();
        let text_file = td.mkfile_with_contents("file.txt", "Hello");
        let binary_file_path: PathBuf =
            td.mkfile_with_bytes("binary.bin", &[0x00, 0xFF, 0x00, 0xFF]);

        let mut cli = Cli::parse_from(&["test"]);
        cli.root = td.path_buf();

        let result_text = should_include_path(&text_file, &cli).unwrap();
        let result_binary = should_include_path(&binary_file_path, &cli).unwrap();

        assert!(result_text);
        assert!(!result_binary);
    }

    #[test]
    fn test_should_include_path_accept_binary_with_cli_override() {
        let td: TempDir = TempDir::new().unwrap();

        let binary_file_path: PathBuf =
            td.mkfile_with_bytes("binary.bin", &[0x00, 0xFF, 0x00, 0xFF]);

        let mut cli = Cli::parse_from(&["test", "--binary"]);
        cli.root = td.path_buf();

        let result_binary = should_include_path(&binary_file_path, &cli).unwrap();

        assert!(result_binary);
    }

    #[test]
    fn test_get_all_files_accept_hidden_with_cli_override() {
        let td = TempDir::new().unwrap();
        td.mkfile("file.txt");
        td.mkfile(".hidden");

        let mut cli = Cli::parse_from(&["test", "--hidden"]);
        cli.root = td.path_buf();

        let result = get_all_files(&cli);

        assert!(result.is_ok());
        let files = result.unwrap();
        assert_eq!(files.len(), 2);
        td.assert_contains(&files, "file.txt");
        td.assert_contains(&files, ".hidden");
    }

    #[test]
    #[cfg(unix)]
    fn test_get_all_files_with_follow_links() {
        let td = TempDir::new().unwrap();

        // Create a real directory with a file
        let td2 = TempDir::new().unwrap();
        td2.mkdir("real_dir");
        let original_dir = td2.path().join("real_dir");
        td2.mkfile_with_contents("real_dir/file3.txt", "Real File");

        // Create a symlink to the real_dir
        let symlink_path = td.path().join("symlink_dir");
        unix_fs::symlink(&original_dir, &symlink_path).unwrap();

        let mut cli = Cli::parse_from(&["quagga"]);
        cli.root = td.path_buf();
        cli.follow_links = true;

        let result = get_all_files(&cli).unwrap();

        assert_eq!(result.len(), 1);
        td.assert_contains(&result, "symlink_dir/file3.txt"); // symlinked file should be included
    }

    #[test]
    #[cfg(unix)]
    fn test_get_all_files_should_not_follow_symlink_by_default() {
        let td = TempDir::new().unwrap();

        // Create a real directory with a file
        let td2 = TempDir::new().unwrap();
        td2.mkdir("real_dir");
        let original_dir = td2.path().join("real_dir");
        td2.mkfile_with_contents("real_dir/file3.txt", "Real File");

        // Create a symlink to the real_dir
        let symlink_path = td.path().join("symlink_dir");
        unix_fs::symlink(&original_dir, &symlink_path).unwrap();

        let mut cli = Cli::parse_from(&["quagga"]);
        cli.root = td.path_buf();

        let result = get_all_files(&cli).unwrap();

        assert_eq!(result.len(), 0);
    }
}
