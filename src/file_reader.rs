use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

/// Reads the contents of the given files and concatenates them into a single string.
///
/// # Arguments
///
/// * `files` - A vector of `PathBuf` representing the paths to the files to read.
///
/// # Returns
///
/// A `Result` containing the concatenated contents of the files as a `String` if successful,
/// or an `io::Error` if an error occurs while reading any of the files.
///
/// # Errors
///
/// This function will return an error if any of the files cannot be opened or read.
/// It will stop at the first encountered error.
pub fn read_and_concatenate_files(files: Vec<PathBuf>) -> io::Result<String> {
    let mut contents = String::new();

    for file_path in files {
        let mut file = fs::File::open(&file_path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        contents.push_str(&buffer);
    }

    Ok(contents)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::temp_dir::TempDir;

    #[test]
    fn test_read_and_concatenate_files() {
        let td = TempDir::new().unwrap();

        let file1_path = td.mkfile_with_contents("file1.txt", "Hello");
        let file2_path = td.mkfile_with_contents("file2.txt", " ");
        let file3_path = td.mkfile_with_contents("file3.txt", "World!");
        let files = vec![file1_path, file2_path, file3_path];

        let result = read_and_concatenate_files(files);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello World!");
    }

    #[test]
    fn test_read_and_concatenate_files_with_nonexistent_file() {
        let td = TempDir::new().unwrap();
        let file1_path = td.mkfile_with_contents("file1.txt", "Hello");
        let file2_path = td.path().join("nonexistent.txt");
        let files = vec![file1_path, file2_path];

        let result = read_and_concatenate_files(files);

        assert!(result.is_err());
    }
}
