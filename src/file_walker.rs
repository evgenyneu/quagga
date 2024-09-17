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
pub fn get_all_files(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();

    for result in Walk::new(root) {
        match result {
            Ok(entry) => {
                if entry.file_type().unwrap().is_file() {
                    files.push(entry.path().to_path_buf());
                }
            }
            Err(err) => eprintln!("Error reading entry: {}", err),
        }
    }

    files
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::temp_dir::TempDir;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    // Helper to create a temporary directory for tests
    fn tmpdir() -> TempDir {
        TempDir::new().unwrap()
    }

    // Helper to write files with content
    fn wfile<P: AsRef<Path>>(path: P, contents: &str) {
        let mut file = File::create(path).unwrap();
        file.write_all(contents.as_bytes()).unwrap();
    }

    // Helper to create directories
    fn mkdirp<P: AsRef<Path>>(path: P) {
        std::fs::create_dir_all(path).unwrap();
    }

    #[test]
    fn test_get_all_files() {
        let td = tmpdir();

        // Create directories and files
        mkdirp(td.path().join("subdir"));
        wfile(td.path().join("file1.txt"), "content");
        wfile(td.path().join("file2.txt"), "content");
        wfile(td.path().join(".hidden"), "content");
        wfile(td.path().join("subdir/file3.txt"), "content");

        // Call the function to get all files
        let files = get_all_files(td.path());

        assert_eq!(files.len(), 3);

        // Use the TempDir helper method to assert the presence of files
        td.assert_contains(&files, &Path::new("file1.txt"));
        td.assert_contains(&files, &Path::new("file2.txt"));
        td.assert_contains(&files, &Path::new("subdir/file3.txt"));

        // Ensure no directories are included
        td.assert_not_contains(&files, &Path::new("subdir"));

        // Ensure hidden files are included
        td.assert_not_contains(&files, &Path::new(".hidden"));
    }
}
