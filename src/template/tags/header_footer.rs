use crate::template::tags::all_file_paths::replace_all_file_paths_tag;
use crate::template::tags::total_file_size::replace_total_file_size_tag;
use crate::template::tags::tree::replace_tree_tag;
use std::path::PathBuf;

/// Replaces tags in the header or footer with the actual values.
///
/// # Arguments
///
/// * `text` - The header or footer text that may contain tags.
/// * `file_paths` - A slice of `PathBuf` representing the file paths.
/// * `root` - The root path used for tree representation.
///
/// # Returns
///
/// A `String` with all tags replaced.
pub fn process_header_footer(text: &str, file_paths: &[PathBuf], root: &PathBuf) -> String {
    let mut processed_text = text.to_string();

    processed_text = replace_all_file_paths_tag(&processed_text, file_paths.to_vec());
    processed_text = replace_tree_tag(&processed_text, file_paths.to_vec(), root.clone());
    replace_total_file_size_tag(&processed_text, file_paths.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::temp_dir::TempDir;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_process_header_footer() {
        let td = TempDir::new().unwrap();
        let file1_path = td.path().join("file1.txt");
        let file2_path = td.path().join("file2.txt");

        // Create files with known sizes
        let mut file1 = File::create(&file1_path).unwrap();
        file1.write_all(&[0u8; 1024]).unwrap(); // 1 KB

        let mut file2 = File::create(&file2_path).unwrap();
        file2.write_all(&[0u8; 2048]).unwrap(); // 2 KB

        let file_paths = vec![file1_path, file2_path];
        let root = td.path_buf();

        let text = r#"
Files:{{ALL_FILE_PATHS}}
Tree: {{TREE}}
Total Size: {{TOTAL_FILE_SIZE}}"#;

        let result = process_header_footer(&text, &file_paths, &root);

        // File list
        assert!(result.contains("file1.txt"));
        assert!(result.contains("file2.txt"));

        // File size
        assert!(result.contains("Total Size: 3 KB"));

        // Tree
        let tree_text = r#"├── file1.txt
└── file2.txt"#;

        assert!(result.contains(tree_text));
    }
}
