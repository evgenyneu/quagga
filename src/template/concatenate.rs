use crate::file_reader::FileContent;
use crate::template::template::TemplateParts;

/// Concatenates the contents of multiple files using the provided template.
///
/// # Arguments
///
/// * `template` - A `TemplateParts` struct containing the header, item, and footer.
/// * `files` - A vector of `FileContent` structs.
///
/// # Returns
///
/// A `String` containing the concatenated contents.
pub fn concatenate_files(_template: TemplateParts, files: Vec<FileContent>) -> String {
    let mut contents = String::new();

    for file in files {
        let formatted_path = format!("\n\n-------\n{}\n-------\n\n", file.path.display());
        contents.push_str(&formatted_path);
        contents.push_str(&file.content);
    }

    contents
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::template::template::TemplateParts;
    use std::path::PathBuf;

    #[test]
    fn test_concatenate_files() {
        let template = TemplateParts::default();
        let file1 = FileContent {
            path: PathBuf::from("file1.txt"),
            content: "Hello".to_string(),
        };
        let file2 = FileContent {
            path: PathBuf::from("file2.txt"),
            content: "World!".to_string(),
        };
        let files = vec![file1, file2];

        let result = concatenate_files(template, files);

        let expected_output = format!(
            "\n\n-------\n{}\n-------\n\n{}\n\n-------\n{}\n-------\n\n{}",
            "file1.txt", "Hello", "file2.txt", "World!"
        );

        assert_eq!(result, expected_output);
    }
}
