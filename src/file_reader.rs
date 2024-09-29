use crate::template::concatenate::concatenate_files;
use crate::template::template::TemplateParts;
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

/// Represents the content of a file along with its path.
///
/// # Fields
///
/// * `path` - The file path.
/// * `content` - The contents of the file as a `String`.
#[derive(Debug)]
pub struct FileContent {
    pub path: PathBuf,
    pub content: String,
}

/// Reads and concatenates files using the provided template parts.
///
/// # Arguments
///
/// * `files` - A vector of `PathBuf` representing the paths to the files to read.
/// * `template` - A `TemplateParts` struct containing the header, item, and footer.
///
/// # Returns
///
/// A `Result` containing the concatenated contents as a `String` if successful,
/// or an `io::Error` if an error occurs while reading any of the files.
pub fn read_and_concatenate_files(
    files: Vec<PathBuf>,
    template: TemplateParts,
) -> io::Result<String> {
    let file_contents = read_files(files)?;
    let concatenated = concatenate_files(template, file_contents);
    Ok(concatenated)
}

/// Reads the contents of the given files and returns a vector of `FileContent`.
///
/// # Arguments
///
/// * `paths` - A vector of `PathBuf` representing the file paths.
///
/// # Returns
///
/// A `Result` containing a vector of `FileContent` if successful, or an `io::Error` if an error occurs.
pub fn read_files(paths: Vec<PathBuf>) -> io::Result<Vec<FileContent>> {
    let mut file_contents = Vec::new();

    for path in paths {
        let mut file = fs::File::open(&path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        file_contents.push(FileContent {
            path: path.clone(),
            content,
        });
    }

    Ok(file_contents)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::template::template::TemplateParts;
    use crate::test_utils::temp_dir::TempDir;

    #[test]
    fn test_read_and_concatenate_files() {
        let td = TempDir::new().unwrap();

        let file1_path = td.mkfile_with_contents("file1.txt", "Hello");
        let file2_path = td.mkfile_with_contents("file32.txt", "World!");
        let files = vec![file1_path.clone(), file2_path.clone()];

        let template = TemplateParts {
            header: "Header".to_string(),
            item: "File: {{FILE_PATH}}\nContent:\n{{CONTENT}}\n---".to_string(),
            footer: "Footer".to_string(),
        };

        let result = read_and_concatenate_files(files, template);

        assert!(result.is_ok());

        let expected_output = format!(
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

        assert_eq!(result.unwrap(), expected_output);
    }

    #[test]
    fn test_read_and_concatenate_files_with_nonexistent_file() {
        let td = TempDir::new().unwrap();
        let file1_path = td.mkfile_with_contents("file1.txt", "Hello");
        let file2_path = td.path().join("nonexistent.txt");
        let files = vec![file1_path, file2_path];

        let result = read_and_concatenate_files(files, TemplateParts::default());

        assert!(result.is_err());
    }
}
