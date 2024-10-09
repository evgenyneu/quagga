use crate::file::file_reader::read_text_file;
use std::io;
use std::path::PathBuf;

/// Checks if the file at the given path contains any of the specified texts.
///
/// # Arguments
///
/// * `file_path` - The path to the file to check.
/// * `texts` - A vector of strings to search for.
/// * `force` - A boolean indicating whether to force reading a file when it is not valid UTF-8 text
///             by removing removing invalid UTF-8 sequences.
///
/// # Returns
///
/// * `Ok(true)` if the file contains any of the texts.
/// * `Ok(false)` if the file does not contain any of the texts.
/// * `Err` if an error occurs while reading the file.
pub fn file_contains_text(file_path: &PathBuf, texts: &[String], force: bool) -> io::Result<bool> {
    let content = read_text_file(file_path.clone(), force)?;

    for text in texts {
        if content.contains(text) {
            return Ok(true);
        }
    }

    return Ok(false);
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
        let result = file_contains_text(&file_path, &texts, false).unwrap();

        assert!(result);
    }

    #[test]
    fn test_file_contains_text_invalid_utf8_force_false() {
        let td = TempDir::new().unwrap();
        let bytes = [0xC0, 0xC1, 0xFF];
        let path = td.mkfile_with_bytes("test.txt", &bytes);

        let texts = vec!["test".to_string()];
        let result = file_contains_text(&path, &texts, false);

        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();

        let expected = format!(
            "Failed to read file {}: stream did not contain valid UTF-8",
            path.display()
        );

        assert_eq!(err_msg, expected);
    }

    #[test]
    fn test_file_contains_text_invalid_utf8_force_true_text_found() {
        let td = TempDir::new().unwrap();
        let bytes = b"Valid text \xFF\xFE Invalid bytes \xC0\xC1 End.";
        let path = td.mkfile_with_bytes("file.txt", bytes);

        let texts = vec!["bytes".to_string()];
        let result = file_contains_text(&path, &texts, true);

        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_file_contains_text_invalid_utf8_force_true_text_not_found() {
        let td = TempDir::new().unwrap();
        let bytes = b"Valid text \xFF\xFE Invalid bytes \xC0\xC1 End.";
        let path = td.mkfile_with_bytes("file.txt", bytes);

        let texts = vec!["does not exist".to_string()];
        let result = file_contains_text(&path, &texts, true);

        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_file_contains_text_not_found() {
        let td = TempDir::new().unwrap();
        let file_path = td.mkfile_with_contents("test.txt", "This is a sample file.");

        let texts = vec!["notfound".to_string()];
        let result = file_contains_text(&file_path, &texts, false).unwrap();

        assert!(!result);
    }

    #[test]
    fn test_file_contains_text_multiple_texts() {
        let td = TempDir::new().unwrap();
        let file_path = td.mkfile_with_contents("test.txt", "This is a test file.");

        let texts = vec!["notfound".to_string(), "test".to_string()];
        let result = file_contains_text(&file_path, &texts, false).unwrap();

        assert!(result);
    }
}
