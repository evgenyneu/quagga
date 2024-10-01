use crate::tree::file_paths_to_tree;
use std::path::PathBuf;

/// Replaces the `{{TREE}}` tag with the ASCII tree representation of the file paths.
///
/// # Arguments
///
/// * `text` - The input string which may contain the `{{TREE}}` tag.
/// * `file_paths` - A list of file paths to be included in the output.
///
/// # Returns
///
/// A new string where the `{{TREE}}` tag is replaced with the formatted file paths.
pub fn replace_tree_tag(text: &str, file_paths: Vec<PathBuf>, root: PathBuf) -> String {
    if text.contains("{{TREE}}") {
        let tree = file_paths_to_tree(file_paths, Some(root));
        text.replace("{{TREE}}", &tree)
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
        {{TREE}}\n\
        Footer";

        let file_paths = vec![
            PathBuf::from("./dir/file1.txt"),
            PathBuf::from("./file2.txt"),
        ];
        let root: PathBuf = PathBuf::from(".");
        let result = replace_tree_tag(template, file_paths, root);

        let expected = r#"Header
.
├── dir
│   └── file1.txt
└── file2.txt
Footer"#;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_replace_all_file_paths_tag_missing() {
        let template = "\
      Header\n\
      Footer";

        let file_paths = vec![PathBuf::from("./file1.txt"), PathBuf::from("./file2.txt")];
        let root: PathBuf = PathBuf::from(".");
        let result = replace_tree_tag(template, file_paths, root);

        assert_eq!(result, template);
    }
}
