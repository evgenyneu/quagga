use crate::info::show_paths::format_file_paths;
use std::path::PathBuf;

/// Replaces the `<all-file-paths>` tag in the given text with the actual file paths.
///
/// # Arguments
///
/// * `text` - The input string which may contain the `<all-file-paths>` tag.
/// * `file_paths` - A list of file paths to be included in the output.
///
/// # Returns
///
/// A new string where the `<all-file-paths>` tag is replaced with the formatted file paths.
pub fn replace_all_file_paths_tag(text: &str, file_paths: Vec<PathBuf>) -> String {
    if text.contains("<all-file-paths>") {
        let formatted_paths = format_file_paths(file_paths);
        text.replace("<all-file-paths>", &formatted_paths)
    } else {
        text.to_string() // Return unchanged text if the tag is not present
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_replace_all_file_paths_tag() {
        let template = "\
        Header\n\
        <all-file-paths>\n\
        Footer";

        let file_paths = vec![PathBuf::from("file1.txt"), PathBuf::from("file2.txt")];
        let result = replace_all_file_paths_tag(template, file_paths);

        assert!(result.contains("file1.txt\nfile2.txt"));
        assert!(result.contains("Header"));
        assert!(result.contains("Footer"));
    }

    #[test]
    fn test_replace_all_file_paths_tag_missing() {
        let template = "\
      Header\n\
      Footer";

        let file_paths = vec![PathBuf::from("file1.txt"), PathBuf::from("file2.txt")];
        let result = replace_all_file_paths_tag(template, file_paths);

        assert_eq!(result, template);
    }
}
