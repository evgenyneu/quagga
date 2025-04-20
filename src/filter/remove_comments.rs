use crate::file::file_reader::FileContent;

/// Removes single-line comments from the file content.
///
/// # Arguments
///
/// * `file_content` - The FileContent struct to process.
///
/// # Returns
///
/// The content of the file as a String, with comments removed (stub: returns input unchanged).
pub fn remove_comments_from_file(file_content: FileContent) -> String {
    file_content.content
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_remove_comments_from_file_stub() {
        let file = FileContent {
            path: PathBuf::from("example.rs"),
            content: String::from("// comment\nlet x = 1;"),
        };

        let result = remove_comments_from_file(file);

        assert_eq!(result, "// comment\nlet x = 1;");
    }
}
