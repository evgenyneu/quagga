use crate::file::size::{calculate_total_size, human_readable_size};
use std::error::Error;
use std::path::PathBuf;

pub fn get_total_size(files: Vec<PathBuf>) -> Result<String, Box<dyn Error>> {
    let total_size = calculate_total_size(files)?;
    Ok(human_readable_size(total_size))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::temp_dir::TempDir;

    #[test]
    fn test_get_total_size() {
        let td = TempDir::new().unwrap();

        // Create test files with known sizes
        td.mkfile_with_contents("file1.txt", "Hello"); // 5 bytes
        td.mkfile_with_contents("file2.txt", "World!"); // 6 bytes

        let files = vec![td.path().join("file1.txt"), td.path().join("file2.txt")];

        let result = get_total_size(files);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "11 B");
    }
}
