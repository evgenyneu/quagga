use home::home_dir;
use std::path::PathBuf;

/// Searches for a `.quagga_template` file in the project root directory and the home directory.
/// Returns the path to the template file if found.
///
/// # Arguments
///
/// * `project_root` - PathBuf of the project root directory.
/// * `home_dir_override` - Optional PathBuf to override the default home directory.
///
/// # Returns
///
/// An `Option<PathBuf>` containing the path to the `.quagga_template` file if it exists.
pub fn quagga_template_path(
    project_root: PathBuf,
    home_dir_override: Option<PathBuf>,
) -> Option<PathBuf> {
    // Check project root directory
    let current_template = project_root.join(".quagga_template");
    if current_template.exists() {
        return Some(current_template);
    }

    // Check home directory
    let home_directory = if let Some(dir) = home_dir_override {
        Some(dir)
    } else if let Some(dir) = home_dir() {
        Some(dir)
    } else {
        None
    };

    if let Some(home) = home_directory {
        let home_template = home.join(".quagga_template");
        if home_template.exists() {
            return Some(home_template);
        }
    }

    // Template not found
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::temp_dir::TempDir;

    #[test]
    fn test_quagga_template_in_project_root() {
        let project_td = TempDir::new().unwrap();
        let project_template_path = project_td.mkfile(".quagga_template");

        let result = quagga_template_path(project_td.path_buf(), None);

        assert_eq!(result.unwrap(), project_template_path);
    }

    #[test]
    fn test_quagga_template_in_home_directory() {
        let home_td = TempDir::new().unwrap();
        let home_template_path = home_td.mkfile(".quagga_template");
        let project_td = TempDir::new().unwrap();

        let result = quagga_template_path(project_td.path_buf(), Some(home_td.path_buf()));

        assert_eq!(result.unwrap(), home_template_path);
    }

    #[test]
    fn test_quagga_template_not_found() {
        let project_td = TempDir::new().unwrap();
        let home_td = TempDir::new().unwrap();

        let result = quagga_template_path(project_td.path_buf(), Some(home_td.path_buf()));

        assert!(result.is_none());
    }

    #[test]
    fn test_project_root_precedence_over_home_directory() {
        // Create temporary directories for project root and home directory
        let project_td = TempDir::new().unwrap();
        let home_td = TempDir::new().unwrap();

        // Create .quagga_template files in both directories
        let project_template_path = project_td.mkfile(".quagga_template");
        home_td.mkfile(".quagga_template");

        let result = quagga_template_path(project_td.path_buf(), Some(home_td.path_buf()));

        assert_eq!(result.unwrap(), project_template_path);
    }
}
