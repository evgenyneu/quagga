use crate::file::size::{calculate_total_size, human_readable_size};
use std::path::PathBuf;

/// Replaces the `<total-file-size>` tag in the given text with the total size of the files.
///
/// # Arguments
///
/// * `text` - The input string that may contain the `<total-file-size>` tag.
/// * `file_paths` - A list of file paths whose sizes will be summed.
///
/// # Returns
///
/// A new string where the `<total-file-size>` tag is replaced with the total file size in a human-readable format.
pub fn replace_total_file_size_tag(text: &str, file_paths: Vec<PathBuf>) -> String {
    if text.contains("<total-file-size>") {
        match calculate_total_size(file_paths) {
            Ok(total_size) => {
                let readable_size = human_readable_size(total_size);
                text.replace("<total-file-size>", &readable_size)
            }
            Err(_) => text.to_string(), // If there's an error, return the original text
        }
    } else {
        text.to_string() // Return the original text if the tag is not present
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::temp_dir::TempDir;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_replace_total_file_size_tag_present() {
        let td = TempDir::new().unwrap();
        let file1_path = td.path().join("file1.txt");
        let file2_path = td.path().join("file2.txt");

        // Create files with known sizes
        let mut file1 = File::create(&file1_path).unwrap();
        file1.write_all(&[0u8; 1024]).unwrap(); // 1 KB

        let mut file2 = File::create(&file2_path).unwrap();
        file2.write_all(&[0u8; 2048]).unwrap(); // 2 KB

        let template = "Total size: <total-file-size>";
        let file_paths = vec![file1_path, file2_path];

        let result = replace_total_file_size_tag(template, file_paths);
        assert_eq!(result, "Total size: 3 KB");
    }

    #[test]
    fn test_replace_total_file_size_tag_not_present() {
        let template = "No size info here.";
        let file_paths = vec![];
        let result = replace_total_file_size_tag(template, file_paths);
        assert_eq!(result, "No size info here.");
    }

    #[test]
    fn test_replace_total_file_size_tag_with_error() {
        let template = "Total size: <total-file-size>";
        let invalid_path = PathBuf::from("/invalid/path.txt");
        let file_paths = vec![invalid_path];

        let result = replace_total_file_size_tag(template, file_paths);
        assert_eq!(result, template);
    }
}
