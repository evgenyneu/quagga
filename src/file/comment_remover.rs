use crate::file::file_content::FileContent;
use warrah::comment_remover::remove_all_comments::remove_all_comments;
use warrah::process::file_path::get_marker_by_file_path;

pub fn remove_comments(file_contents: Vec<FileContent>) -> Vec<FileContent> {
    file_contents
        .into_iter()
        .map(remove_comments_from_file)
        .collect()
}

/// Removes comments from a single file if markers are found for its extension.
fn remove_comments_from_file(file_content: FileContent) -> FileContent {
    let markers = match get_marker_by_file_path(&file_content.path) {
        Some(markers) => markers,
        None => return file_content,
    };

    let content = remove_all_comments(&file_content.content, markers, true);

    FileContent {
        path: file_content.path,
        content,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_remove_comments() {
        let files = vec![
            FileContent {
                path: PathBuf::from("file1.rs"),
                content: String::from("let x = 1; // comment"),
            },
            FileContent {
                path: PathBuf::from("file2.txt"),
                content: String::from("Unchanged content"),
            },
        ];

        let result = remove_comments(files);

        assert_eq!(result[0].content, "let x = 1;");
        assert_eq!(result[1].content, "Unchanged content");
    }

    #[test]
    fn test_remove_comments_from_file_with_markers() {
        let file = FileContent {
            path: PathBuf::from("example.rs"),
            content: String::from(
                r#"let x = 1; // single line comment
    /* multi-line
       nice
       comment */
    let y = 2; // another single line
    let z = 3; /* inline multi-line */ let w = 4;"#,
            ),
        };

        let result = remove_comments_from_file(file);

        assert_eq!(
            result.content,
            "let x = 1;\n\n    let y = 2;\n    let z = 3; let w = 4;"
        );
    }

    #[test]
    fn test_remove_comments_from_file_no_markers() {
        let file = FileContent {
            path: PathBuf::from("example.txt"),
            content: String::from("Unchanged content"),
        };

        let result = remove_comments_from_file(file);

        assert_eq!(result.content, "Unchanged content");
    }
}
