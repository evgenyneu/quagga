use std::error::Error;
use std::fs;
use std::path::PathBuf;

/// Copies the embedded default template to `.quagga_template` in the specified `root` directory.
/// Returns a success message if copied, or an error if the file already exists.
///
/// # Arguments
///
/// * `root` - The root directory where the `.quagga_template` file will be created.
///
/// # Returns
///
/// * `Ok(String)` with the success message.
/// * `Err` if an error occurred or the file already exists.
pub fn copy_template(root: &PathBuf) -> Result<String, Box<dyn Error>> {
    let template_content = include_str!("../../templates/default.txt");
    let destination = root.join(".quagga_template");

    if destination.exists() {
        return Err(format!("Template file '{}' already exists.", destination.display()).into());
    }

    fs::write(&destination, template_content)?;
    Ok(format!(
        "Template was copied to '{}'.",
        destination.display()
    ))
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::temp_dir::TempDir;

    #[test]
    fn test_copy_template_success() {
        let td = TempDir::new().unwrap();

        let result = copy_template(&td.path_buf());

        assert!(result.is_ok());
        let msg = result.unwrap();

        let expected_msg = format!(
            "Template was copied to '{}'.",
            td.path().join(".quagga_template").display()
        );

        assert_eq!(msg, expected_msg);

        let template_path = td.path().join(".quagga_template");
        assert!(template_path.exists());

        let content = fs::read_to_string(template_path).unwrap();
        assert!(content.contains("{{HEADER}}"));
    }

    #[test]
    fn test_copy_template_file_exists() {
        let td = TempDir::new().unwrap();
        let existing_file = td.mkfile(".quagga_template");

        let result = copy_template(&td.path_buf());
        assert!(result.is_err());

        let err_msg = result.unwrap_err().to_string();

        let expected_msg = format!(
            "Template file '{}' already exists.",
            existing_file.display()
        );

        assert_eq!(err_msg, expected_msg);
    }
}
