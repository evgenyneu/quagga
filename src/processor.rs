use std::error::Error;
use std::path::Path;
use std::path::PathBuf;

use crate::file_reader::read_and_concatenate_files;
use crate::file_walker::get_all_files;

/// Processes files based on provided root directory or list of file paths.
///
/// # Arguments
///
/// * `root` - A `PathBuf` representing the root directory to search for files.
/// * `paths` - An optional `Vec<PathBuf>` representing a list of file paths to process.
///
/// # Returns
///
/// A `Result` containing the concatenated contents of the files as a `String` if successful,
/// or an error if any operation fails.
pub fn run(root: PathBuf, paths: Option<Vec<PathBuf>>) -> Result<String, Box<dyn Error>> {
    if let Some(path_list) = paths {
        return read_and_concatenate_files(path_list).map_err(|e| Box::new(e) as Box<dyn Error>);
    } else {
        return process_files(&root);
    }
}

/// Processes files starting from the given root path:
/// - Retrieves all files.
/// - Reads and concatenates their contents.
///
/// # Arguments
///
/// * `root` - The root directory to start processing.
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
pub fn process_files(root: &Path) -> Result<String, Box<dyn Error>> {
    // Get all files starting from the root directory
    let mut files = get_all_files(root)?;

    // Sort the files alphabetically by their path
    files.sort();

    // Read and concatenate the contents of the files
    return read_and_concatenate_files(files).map_err(|e| Box::new(e) as Box<dyn Error>);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::temp_dir::TempDir;

    #[test]
    fn test_process_files_success() {
        // Create a temporary directory for the test
        let td = TempDir::new().unwrap();

        // Create test files with contents
        let file1_path = td.mkfile_with_contents("file1.txt", "Hello");
        let file2_path = td.mkfile_with_contents("file2.txt", " ");
        let file3_path = td.mkfile_with_contents("file3.txt", "World!");

        // Call the function under test
        let result = process_files(td.path());

        // Assert the result
        assert!(result.is_ok());

        let expected_output = format!(
          "\n\n-------\n{}\n-------\n\nHello\n\n-------\n{}\n-------\n\n \n\n-------\n{}\n-------\n\nWorld!",
          file1_path.display(),
          file2_path.display(),
          file3_path.display()
        );

        assert_eq!(result.unwrap(), expected_output);
    }

    #[test]
    fn test_process_files_with_nonexistent_directory() {
        // Create a path to a non-existent directory
        let non_existent_path = Path::new("/path/to/nonexistent/directory");

        // Call the function under test
        let result = process_files(non_existent_path);

        // Assert that an error is returned
        assert!(result.is_err());
    }
}
