use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

/// Checks if the file at the given path contains any of the specified texts.
///
/// # Arguments
///
/// * `file_path` - The path to the file to check.
/// * `texts` - A vector of strings to search for.
///
/// # Returns
///
/// * `Ok(true)` if the file contains any of the texts.
/// * `Ok(false)` if the file does not contain any of the texts.
/// * `Err` if an error occurs while reading the file.
pub fn file_contains_text(file_path: &PathBuf, texts: &[String]) -> io::Result<bool> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        for text in texts {
            if line.contains(text) {
                return Ok(true);
            }
        }
    }

    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::temp_dir::TempDir;

    #[test]
    fn test_file_contains_text_found() {
        let td = TempDir::new().unwrap();
        let file_path = td.mkfile_with_contents("test.txt", "This is a test file.");

        let texts = vec!["test".to_string()];
        let result = file_contains_text(&file_path, &texts).unwrap();

        assert!(result);
    }

    #[test]
    fn test_file_contains_text_not_found() {
        let td = TempDir::new().unwrap();
        let file_path = td.mkfile_with_contents("test.txt", "This is a sample file.");

        let texts = vec!["notfound".to_string()];
        let result = file_contains_text(&file_path, &texts).unwrap();

        assert!(!result);
    }

    #[test]
    fn test_file_contains_text_multiple_texts() {
        let td = TempDir::new().unwrap();
        let file_path = td.mkfile_with_contents("test.txt", "This is a test file.");

        let texts = vec!["notfound".to_string(), "test".to_string()];
        let result = file_contains_text(&file_path, &texts).unwrap();

        assert!(result);
    }
}
