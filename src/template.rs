use std::fs;
use std::io;
use std::path::PathBuf;

/// Reads the template from a given path or falls back to the default embedded template.
///
/// # Arguments
///
/// * `template_path` - An `Option<PathBuf>` specifying the path to the template file.
///
/// # Returns
///
/// * `Ok<String>` containing the template content.
/// * `Err<io::Error>` if an I/O error occurs while reading the template.
pub fn read_template(template_path: Option<PathBuf>) -> io::Result<String> {
    match template_path {
        Some(path) => fs::read_to_string(&path),
        None => Ok(DEFAULT_TEMPLATE.to_string()),
    }
}

/// The default template embedded into the executable.
const DEFAULT_TEMPLATE: &str = include_str!("../templates/default.txt");

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::temp_dir::TempDir;

    #[test]
    fn test_read_template_with_none() {
        let result = read_template(None).unwrap();
        assert_eq!(result, DEFAULT_TEMPLATE);
    }

    #[test]
    fn test_read_template_with_valid_path() {
        let td = TempDir::new().unwrap();
        let template_content = "Custom Template Content";
        let template_path = td.mkfile_with_contents("template.txt", template_content);

        let result = read_template(Some(template_path)).unwrap();
        assert_eq!(result, template_content);
    }

    #[test]
    fn test_read_template_with_invalid_path() {
        let td = TempDir::new().unwrap();
        let invalid_path = td.path().join("nonexistent_template.txt");

        let result = read_template(Some(invalid_path));

        assert!(result.is_err());
    }
}
