use ignore::Walk;
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
/// A vector containing the paths of all files found.
pub fn get_all_files(root: &Path) -> Result<Vec<PathBuf>, ignore::Error> {
    let mut files = Vec::new();

    for result in Walk::new(root) {
        match result {
            Ok(entry) => {
                if entry.file_type().unwrap().is_file() {
                    files.push(entry.path().to_path_buf());
                }
            }
            Err(err) => return Err(err),
        }
    }

    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::temp_dir::TempDir;

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

        // Use the TempDir helper method to assert the presence of files
        td.assert_contains(&files, "file1.txt");
        td.assert_contains(&files, "file2.txt");
        td.assert_contains(&files, "subdir/file3.txt");

        // Ensure no directories are included
        td.assert_not_contains(&files, "subdir");

        // Ensure hidden files are included
        td.assert_not_contains(&files, ".hidden");
    }
}
