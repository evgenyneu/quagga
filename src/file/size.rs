use std::fs;
use std::io;
use std::path::PathBuf;

/// Checks if the total size of the files exceeds the specified maximum size.
///
///
/// # Arguments
///
/// * `file_paths` - A slice of `PathBuf` representing the file paths.
/// * `max_total_size` - The maximum allowed total size in bytes.
///
/// # Returns
///
/// * `Ok(())` - If the total size is within the limit.
/// * `Err(io::Error)` - If the total size exceeds the limit or an error occurs during size calculation.
pub fn check_total_size(file_paths: Vec<PathBuf>, max_total_size: u64) -> io::Result<()> {
    let total_size = calculate_total_size(file_paths)?;

    if total_size > max_total_size {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                r#"Total size of files ({}) exceeds the maximum allowed size ({}).
Use --max-total-size=BYTES option to increase the limit.
"#,
                human_readable_size(total_size),
                human_readable_size(max_total_size)
            ),
        ));
    }
    Ok(())
}

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
        let metadata = fs::metadata(&path).map_err(|e| {
            io::Error::new(
                e.kind(),
                format!("Failed to read metadata for file {}: {}", path.display(), e),
            )
        })?;

        // Check if the path points to a regular file
        if !metadata.is_file() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Path is not a regular file: {}", path.display()),
            ));
        }

        total_size += metadata.len();
    }

    Ok(total_size)
}

/// Converts bytes into a human-readable string with appropriate units.
/// This will display file sizes in B, KB, MB, GB, or TB depending on size.
///
/// # Arguments
/// * `bytes` - The size in bytes to be converted.
///
/// # Returns
/// A `String` representing the size in a human-friendly format.
pub fn human_readable_size(bytes: u64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit = units[0];

    for &next_unit in &units[1..] {
        if size < 1024.0 {
            break;
        }
        size /= 1024.0;
        unit = next_unit;
    }

    if size.fract() == 0.0 {
        format!("{:.0} {}", size, unit) // No decimal places if it's a whole number
    } else {
        format!("{:.2} {}", size, unit) // Two decimal places otherwise
    }
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

    #[test]
    fn test_check_total_size_within_limit() {
        let td = TempDir::new().unwrap();

        let file1_path = td.mkfile_with_contents("file1.txt", "12345"); // 5 bytes
        let file2_path = td.mkfile_with_contents("file2.txt", "1234567890"); // 10 bytes

        let files = vec![file1_path, file2_path];

        // Total size is 15 bytes, set max_total_size to 20 bytes
        let result = check_total_size(files, 20);

        assert!(result.is_ok());
    }

    #[test]
    fn test_check_total_size_exceeds_limit() {
        let td = TempDir::new().unwrap();

        let file1_path = td.mkfile_with_contents("file1.txt", "12345"); // 5 bytes
        let file2_path = td.mkfile_with_contents("file2.txt", "1234567890"); // 10 bytes

        let files = vec![file1_path, file2_path];

        // Total size is 15 bytes, set max_total_size to 10 bytes
        let result = check_total_size(files, 10);

        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();

        assert!(
            err_msg.contains("Total size of files (15 B) exceeds the maximum allowed size (10 B).")
        );
    }

    #[test]
    fn test_check_total_size_equal_to_limit() {
        let td = TempDir::new().unwrap();

        let file1_path = td.mkfile_with_contents("file1.txt", "12345"); // 5 bytes
        let file2_path = td.mkfile_with_contents("file2.txt", "12345"); // 5 bytes

        let files = vec![file1_path, file2_path];

        // Total size is 10 bytes, set max_total_size to 10 bytes
        let result = check_total_size(files, 10);

        assert!(result.is_ok());
    }

    #[test]
    fn test_human_readable_size() {
        assert_eq!(human_readable_size(500), "500 B");
        assert_eq!(human_readable_size(1024), "1 KB");
        assert_eq!(human_readable_size(1048576), "1 MB");
        assert_eq!(human_readable_size(1634546), "1.56 MB");
        assert_eq!(human_readable_size(1073741824), "1 GB");
        assert_eq!(human_readable_size(1373741824342), "1.25 TB");
    }

    #[test]
    fn test_calculate_total_size_with_nonexistent_file_error() {
        let td = TempDir::new().unwrap();

        let file1_path = td.mkfile_with_contents("file1.txt", "12345"); // 5 bytes
        let nonexistent_file_path = td.path().join("nonexistent.txt");

        let files = vec![file1_path, nonexistent_file_path.clone()];

        let result = calculate_total_size(files);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::NotFound);
        assert!(error.to_string().contains(&format!("Failed to read metadata for file {}", nonexistent_file_path.display())));
    }
}
