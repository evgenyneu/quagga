use crate::cli::Cli;
use crate::file::size::check_total_size;
use crate::filter::filter::filter_lines_in_files;
use crate::template::concatenate::concatenate_files;
use crate::template::template::Template;
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

/// Represents the content of a file along with its path.
///
/// # Fields
///
/// * `path` - The file path.
/// * `content` - The contents of the file as a `String`.
#[derive(Debug, Clone)]
pub struct FileContent {
    pub path: PathBuf,
    pub content: String,
}

/// Reads and concatenates files using the provided template.
///
/// # Arguments
///
/// * `files` - A vector of `PathBuf` representing the paths to the files to read.
/// * `template` - A `Template` struct containing the template sections.
///
/// # Returns
///
/// A `Result` containing the output prompt text, splitted into parts, if successful,
/// or an `io::Error` if an error occurs while reading any of the files or if the files vector is empty.
pub fn read_and_concatenate_files(
    files: Vec<PathBuf>,
    template: Template,
    cli: &Cli,
) -> io::Result<Vec<String>> {
    if files.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "No files to process",
        ));
    }

    check_total_size(files.clone(), cli.max_total_size)?;
    let file_contents = read_files(files, cli.binary)?;
    let filtered = filter_lines_in_files(&file_contents, cli);
    let concatenated = concatenate_files(template, filtered, cli);
    Ok(concatenated)
}

/// Reads the contents of the given files and returns a vector of `FileContent`.
///
/// # Arguments
///
/// * `paths` - A vector of `PathBuf` representing the file paths.
/// * `force` - A boolean indicating whether to force reading a file when it is not valid UTF-8 text
///             by removing removing invalid UTF-8 sequences.
///
/// # Returns
///
/// A `Result` containing a vector of `FileContent` if successful, or an `io::Error` if an error occurs.
pub fn read_files(paths: Vec<PathBuf>, force: bool) -> io::Result<Vec<FileContent>> {
    let mut file_contents = Vec::new();

    for path in paths {
        let content = read_text_file(path.clone(), force)?;

        file_contents.push(FileContent {
            path: path.clone(),
            content,
        });
    }

    Ok(file_contents)
}

/// Reads and returns the content of the given text file.
/// It tries to read the file as UTF-8 text first. If it fails and `force` is true
/// then it reads the file as binary data and removes invalid UTF-8 sequences.
///
/// # Arguments
///
/// * `path` - A path to a text file.
/// * `force` - A boolean indicating whether to force reading the file when it is not valid UTF-8 text
///             by removing removing invalid UTF-8 sequences.
///
/// # Returns
///
/// A `Result` containing a the content of the text file or error if the file cannot be read.
pub fn read_text_file(path: PathBuf, force: bool) -> io::Result<String> {
    let mut file = fs::File::open(&path).map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("Failed to open file {}: {}", path.display(), e),
        )
    })?;

    let mut content = String::new();

    // Try reading the file as UTF-8 text first
    match file.read_to_string(&mut content) {
        Ok(_) => {
            return Ok(content);
        }
        Err(e) => {
            if force {
                // If the file is not valid UTF-8 text, try reading it as binary data
                return force_read_text_file(path);
            } else {
                return Err(io::Error::new(
                    e.kind(),
                    format!("Failed to read file {}: {}", path.display(), e),
                ));
            }
        }
    }
}

/// Reads the file as binary data and then convets it to UTF-8 by removing invalid UTF-8 sequences.
///
/// # Arguments
///
/// * `path` - A path to a file.
///
/// # Returns
///
/// A `Result` containing a the content of the text file or error if the file cannot be read.
pub fn force_read_text_file(path: PathBuf) -> io::Result<String> {
    let mut file = fs::File::open(&path)?;
    let mut bytes = Vec::new();

    file.read_to_end(&mut bytes).map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("Failed to read as binary {}: {}", path.display(), e),
        )
    })?;

    // Replaces invalid UTF-8 sequences with the Unicode replacement character \u{FFFD}.
    let content = String::from_utf8_lossy(&bytes);

    // Removes the replacement character to make the string a valid UTF-8 text
    let cleaned_content = content.replace("\u{FFFD}", "");
    return Ok(cleaned_content);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::template::template::{PromptTemplate, Template};
    use crate::test_utils::temp_dir::TempDir;
    use clap::Parser;

    #[test]
    fn test_read_and_concatenate_files() {
        let td = TempDir::new().unwrap();

        let file1_path = td.mkfile_with_contents("file1.txt", "Hello");
        let file2_path = td.mkfile_with_contents("file32.txt", "World!");
        let files = vec![file1_path.clone(), file2_path.clone()];

        let template = Template {
            prompt: PromptTemplate {
                header: "Header".to_string(),
                file: "File: <file-path>\nContent:\n<file-content>\n---".to_string(),
                footer: "Footer".to_string(),
            },
            part: Default::default(),
        };

        let cli = Cli::parse_from(&["test"]);

        let result = read_and_concatenate_files(files, template, &cli);

        assert!(result.is_ok());
        let content = result.unwrap();
        assert_eq!(content.len(), 1);

        let expected = format!(
            "\
Header
File: {}
Content:
Hello
---
File: {}
Content:
World!
---
Footer",
            file1_path.display(),
            file2_path.display()
        );

        assert_eq!(content[0], expected);
    }

    #[test]
    fn test_read_and_concatenate_files_with_nonexistent_file() {
        let td = TempDir::new().unwrap();
        let file1_path = td.mkfile_with_contents("file1.txt", "Hello");
        let file2_path = td.path().join("nonexistent.txt");
        let files = vec![file1_path, file2_path];
        let cli = Cli::parse_from(&["test"]);

        let result = read_and_concatenate_files(files, Template::default(), &cli);

        assert!(result.is_err());
    }

    #[test]
    fn test_read_and_concatenate_files_total_size_exceeds_limit() {
        let td = TempDir::new().unwrap();

        let file_content = "1234567890a"; // 11 bytes
        let file1_path = td.mkfile_with_contents("file1.txt", file_content);
        let files = vec![file1_path.clone()];

        let template = Template {
            prompt: PromptTemplate {
                header: "Header".to_string(),
                file: "<file-content>".to_string(),
                footer: "Footer".to_string(),
            },
            part: Default::default(),
        };

        let mut cli = Cli::parse_from(&["test"]);
        cli.max_total_size = 10; // Set max_total_size to 10 bytes

        let result = read_and_concatenate_files(files, template, &cli);

        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("exceeds the maximum"));
    }

    #[test]
    fn test_read_and_concatenate_files_no_files_error() {
        let template = Template::default();
        let cli = Cli::parse_from(&["test"]);

        let result = read_and_concatenate_files(vec![], template, &cli);

        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert_eq!(err_msg, "No files to process");
    }

    #[test]
    fn test_read_files_with_invalid_utf8_force_false() {
        let td = TempDir::new().unwrap();
        let bytes = [0xC0, 0xC1]; // Invalid UTF-8 sequences
        let path = td.mkfile_with_bytes("invalid_utf_8.txt", &bytes);
        let files = vec![path.clone()];

        let result = read_files(files, false);

        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();

        assert_eq!(
            err_msg,
            format!(
                "Failed to read file {}: stream did not contain valid UTF-8",
                path.display()
            )
        );
    }

    #[test]
    fn test_read_files_with_invalid_utf8_force_true() {
        let td = TempDir::new().unwrap();
        // Mix of valid UTF-8 and invalid bytes
        let bytes = b"Valid text \xFF\xFE Invalid bytes \xC0\xC1 End.";
        let path = td.mkfile_with_bytes("invalid_utf8.txt", bytes);
        let files = vec![path.clone()];

        let result = read_files(files, true);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].content, "Valid text  Invalid bytes  End.");
        assert_eq!(result[0].path, path);
    }

    #[test]
    fn test_force_read_text_file_valid_utf8() {
        let td = TempDir::new().unwrap();
        let path = td.mkfile_with_contents("valid_utf8.txt", "This is a valid UTF-8 string.");

        let result = force_read_text_file(path.clone());

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "This is a valid UTF-8 string.");
    }

    #[test]
    fn test_force_read_text_file_invalid_utf8() {
        let td = TempDir::new().unwrap();
        // Mix of valid UTF-8 and invalid bytes
        let bytes = b"Valid text \xFF\xFE Invalid bytes \xC0\xC1 End.";
        let path = td.mkfile_with_bytes("invalid_utf8.txt", bytes);

        let result = force_read_text_file(path.clone());

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Valid text  Invalid bytes  End.");
    }

    #[test]
    fn test_force_read_text_file_empty_file() {
        let td = TempDir::new().unwrap();
        let path = td.mkfile_with_contents("empty.txt", "");

        let result = force_read_text_file(path.clone());

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");
    }

    #[test]
    fn test_read_text_file_valid_utf8_force_false() {
        let td = TempDir::new().unwrap();
        let path = td.mkfile_with_contents("valid_utf8.txt", "This is a valid UTF-8 string.");

        let result = read_text_file(path.clone(), false);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "This is a valid UTF-8 string.");
    }

    #[test]
    fn test_read_text_file_valid_utf8_force_true() {
        let td = TempDir::new().unwrap();
        let path = td.mkfile_with_contents("another_valid_utf8.txt", "Another valid UTF-8 string.");

        let result = read_text_file(path.clone(), true);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Another valid UTF-8 string.");
    }

    #[test]
    fn test_read_text_file_invalid_utf8_force_false() {
        let td = TempDir::new().unwrap();
        let bytes = [0xC0, 0xC1, 0xFF];
        let path = td.mkfile_with_bytes("invalid_utf8_force_false.txt", &bytes);

        let result = read_text_file(path.clone(), false);

        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();

        let expected = format!(
            "Failed to read file {}: stream did not contain valid UTF-8",
            path.display()
        );

        assert_eq!(err_msg, expected);
    }

    #[test]
    fn test_read_text_file_invalid_utf8_force_true() {
        let td = TempDir::new().unwrap();
        let bytes = b"Valid text \xFF\xFE Invalid bytes \xC0\xC1 End.";
        let path = td.mkfile_with_bytes("invalid_utf8_force_true.txt", bytes);

        let result = read_text_file(path.clone(), true);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Valid text  Invalid bytes  End.");
    }

    #[test]
    fn test_read_text_file_empty_file() {
        let td = TempDir::new().unwrap();
        let path = td.mkfile_with_contents("empty.txt", "");

        let result = read_text_file(path.clone(), false);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");
    }

    #[test]
    fn test_read_text_file_non_existent() {
        let non_existent_path = PathBuf::from("/path/to/non/existent/file.txt");

        let result = read_text_file(non_existent_path.clone(), false);

        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("Failed to open file /path/to/non/existent/file.txt"));
    }
}
