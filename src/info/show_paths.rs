use std::path::PathBuf;

/// Converts a list file paths to string. The paths are sorted.
///
/// # Arguments
///
/// * `sorted_paths` - A list of file paths.
///
/// # Returns
///
/// A string containing the file paths separated by newlines.
pub fn format_file_paths(file_paths: Vec<PathBuf>) -> String {
    let mut sorted_paths = file_paths.clone();
    sorted_paths.sort();

    let file_paths: Vec<String> = sorted_paths
        .iter()
        .map(|file| file.display().to_string())
        .collect();

    file_paths.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_file_paths() {
        let path1 = PathBuf::from("file1.txt");
        let path2 = PathBuf::from("file2.txt");
        let path3 = PathBuf::from("file3.txt");

        let files = vec![path3.clone(), path1.clone(), path2.clone()];
        let output = format_file_paths(files);

        let expected = format!(
            "{}\n{}\n{}",
            path1.display(),
            path2.display(),
            path3.display()
        );

        assert_eq!(output, expected);
    }

    #[test]
    fn test_format_file_paths_no_paths() {
        let files: Vec<PathBuf> = vec![];
        let output = format_file_paths(files);
        assert_eq!(output, "");
    }
}
