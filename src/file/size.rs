use std::fs;
use std::io;
use std::path::PathBuf;

/// Calculates the total size of the files given by their paths.
///
/// # Arguments
///
/// * `file_paths` - A vector of `PathBuf` representing the file paths.
///
/// # Returns
///
/// * `Ok(u64)` - The total size of the files in bytes.
/// * `Err(io::Error)` - An error occurred while accessing a file's metadata.
pub fn calculate_total_size(file_paths: Vec<PathBuf>) -> io::Result<u64> {
    let mut total_size = 0u64;

    for path in file_paths {
        let metadata = fs::metadata(&path)?;

        // Check if the path points to a regular file
        if !metadata.is_file() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Path is not a regular file",
            ));
        }

        total_size += metadata.len();
    }

    Ok(total_size)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::temp_dir::TempDir;

    #[test]
    fn test_calculate_total_size_with_valid_files() {
        let td = TempDir::new().unwrap();

        // Create test files with known sizes
        let file1_path = td.mkfile_with_contents("file1.txt", "12345"); // 5 bytes
        let file2_path = td.mkfile_with_contents("file2.txt", "1234567890"); // 10 bytes

        let files = vec![file1_path, file2_path];

        let result = calculate_total_size(files).unwrap();
        assert_eq!(result, 15);
    }

    #[test]
    fn test_calculate_total_size_with_empty_files() {
        let td = TempDir::new().unwrap();

        let file1_path = td.mkfile_with_contents("file1.txt", ""); // 0 bytes
        let file2_path = td.mkfile_with_contents("file2.txt", ""); // 0 bytes

        let files = vec![file1_path, file2_path];

        let result = calculate_total_size(files).unwrap();
        assert_eq!(result, 0);
    }

    #[test]
    fn test_calculate_total_size_with_nonexistent_file() {
        let td = TempDir::new().unwrap();

        let file1_path = td.mkfile_with_contents("file1.txt", "12345"); // 5 bytes
        let file2_path = td.path().join("nonexistent.txt"); // Does not exist

        let files = vec![file1_path, file2_path];

        let result = calculate_total_size(files);

        assert!(result.is_err());
    }

    #[test]
    fn test_calculate_total_size_with_directory_path() {
        let td = TempDir::new().unwrap();

        let dir_path = td.path().join("subdir");
        std::fs::create_dir(&dir_path).unwrap();

        let files = vec![dir_path];

        let result = calculate_total_size(files);

        assert!(result.is_err());
    }
}
