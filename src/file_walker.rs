use crate::binary_detector::is_valid_text_file;
use ignore::Walk;
use std::error::Error;
use std::path::Path;
use std::path::PathBuf;

/// Walks through the directory tree starting from `root` and collects all file paths.
///
/// # Arguments
///
/// * `root` - The root directory to start walking from.
///
/// # Returns
///
/// * `Ok(Vec<PathBuf>)` containing the paths of valid text files.
/// * `Err(Box<dyn Error>)` if an error occurs during directory traversal or file reading.
pub fn get_all_files(root: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut files = Vec::new();

    for result in Walk::new(root) {
        match result {
            Ok(entry) => {
                if entry.file_type().unwrap().is_file() {
                    let path = entry.path().to_path_buf();

                    match is_valid_text_file(path.clone()) {
                        Ok(true) => files.push(path),
                        Ok(false) => continue,             // Skip binary files
                        Err(e) => return Err(Box::new(e)), // Propagate the error
                    }
                }
            }
            Err(err) => return Err(Box::new(err)),
        }
    }

    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::temp_dir::TempDir;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_get_all_files() {
        let td = TempDir::new().unwrap();

        // Create directories and files
        td.mkdir("subdir");
        td.mkfile("file1.txt");
        td.mkfile("file2.txt");
        td.mkfile(".hidden");
        td.mkfile("subdir/file3.txt");

        // Call the function to get all files
        let result = get_all_files(td.path());
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
        // Create a temporary directory
        let td = TempDir::new().unwrap();

        // Create text files
        td.mkfile_with_contents("file1.txt", "fn main() {}");
        td.mkfile_with_contents("file2.rs", "println!(\"Hello, world!\");");

        // Create a binary file
        let binary_file_path = td.path().join("binary.bin");
        let mut binary_file = File::create(&binary_file_path).unwrap();
        let binary_content = [0x00, 0xFF, 0x00, 0xFF];
        binary_file.write_all(&binary_content).unwrap();

        // Get all files starting from the temporary directory
        let result = get_all_files(td.path());

        // Assert that the result is Ok
        assert!(result.is_ok());

        let files = result.unwrap();

        // Collect file names
        let file_names: Vec<String> = files
            .iter()
            .map(|path| path.file_name().unwrap().to_string_lossy().into_owned())
            .collect();

        // Assert that text files are included
        assert!(file_names.contains(&"file1.txt".to_string()));
        assert!(file_names.contains(&"file2.rs".to_string()));

        // Assert that binary file is not included
        assert!(!file_names.contains(&"binary.bin".to_string()));
    }

    #[test]
    fn test_get_all_files_with_no_files() {
        // Create an empty temporary directory
        let td = TempDir::new().unwrap();

        // Get all files starting from the temporary directory
        let result = get_all_files(td.path());

        // Assert that the result is Ok and the list is empty
        assert!(result.is_ok());
        let files = result.unwrap();
        assert!(files.is_empty());
    }

    #[test]
    fn test_get_all_files_with_nonexistent_directory() {
        // Create a path to a non-existent directory
        let non_existent_path = Path::new("/path/to/nonexistent/directory");

        let result = get_all_files(non_existent_path);

        assert!(result.is_err());
    }
}
