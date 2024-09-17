use std::io;
use std::path::Path;

use crate::file_reader::read_and_concatenate_files;
use crate::file_walker::get_all_files;

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
pub fn process_files(root: &Path) -> io::Result<String> {
    // Get all files starting from the root directory
    let files = get_all_files(root)?;

    // Read and concatenate the contents of the files
    let contents = read_and_concatenate_files(files)?;

    Ok(contents)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::temp_dir::TempDir;
    use std::io;

    #[test]
    fn test_process_files_success() {
        // Create a temporary directory for the test
        let td = TempDir::new().unwrap();

        // Create test files with contents
        td.mkfile_with_contents("file1.txt", "Hello");
        td.mkfile_with_contents("file2.txt", " ");
        td.mkfile_with_contents("file3.txt", "World!");

        // Call the function under test
        let result = process_files(td.path());

        // Assert the result
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello World!");
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
