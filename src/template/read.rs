use super::parse::parse_template;
use super::quagga_template::quagga_template_path;
use super::template::Template;
use crate::cli::Cli;
use std::error::Error;
use std::fs;
use std::io;
use std::path::PathBuf;

/// The default template embedded into the executable.
const DEFAULT_TEMPLATE: &str = include_str!("../../templates/default.md");

/// Reads and parses a template from a given path or the default template.
///
/// This function performs the following steps:
/// 1. Reads the template content from the provided path or uses the default template.
/// 2. Parses the template into its components: prompt, header, footer etc.
///
/// # Arguments
///
/// * `template_path` - An `Option<PathBuf>` specifying the path to the template file.
///                     If `None`, the default embedded template is used.
///
/// # Returns
///
/// * `Ok(Template)` containing the parsed template components.
/// * `Err<Box<dyn Error>>` if an error occurs during reading, validation, or parsing.
pub fn read_and_parse_template(template_path: Option<PathBuf>) -> Result<Template, Box<dyn Error>> {
    let template_content = read_template(template_path)?;
    let template_content = template_content.replace("\r\n", "\n"); // Normalize line endings
    let template = parse_template(&template_content)?;
    Ok(template)
}

/// Retrieves the path to the curstom template:
/// - If a custom template path is provided via the CLI, it is used.
/// - Use `.quagga_template` file from the current or home directory,
///   unless the `--no-quagga-template` command line option is used.
///
/// # Arguments
///
/// * `cli` - Command line arguments.
///
/// # Returns
///
/// * An `Option<PathBuf>` containing the path to the custom template file if used.
pub fn path_to_custom_template(cli: &Cli) -> Option<PathBuf> {
    if let Some(path) = cli.template.clone() {
        Some(path) // Use the provided template from --template option
    } else if cli.no_quagga_template {
        None
    } else {
        // Use the .quagga_template file from the current or home directory
        quagga_template_path(cli.root.clone(), None)
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::temp_dir::TempDir;
    use clap::Parser;

    #[test]
    fn test_read_template_with_none() {
        let result = read_template(None).unwrap();
        assert_eq!(result, DEFAULT_TEMPLATE);
    }

    #[test]
    fn test_read_template_with_valid_path() {
        let td = TempDir::new().unwrap();
        let template_content = "Custom Template Content";
        let template_path = td.mkfile_with_contents("template.md", template_content);

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

    #[test]
    fn test_read_and_parse_template_with_default_template() {
        let result = read_and_parse_template(None);
        assert!(result.is_ok());
        let template = result.unwrap();
        assert!(!template.prompt.header.is_empty());
        assert!(!template.prompt.file.is_empty());
        assert!(!template.prompt.footer.is_empty());
        assert!(!template.part.header.is_empty());
        assert!(!template.part.pending.is_empty());
        assert!(!template.part.footer.is_empty());
    }

    #[test]
    fn test_read_and_parse_template_with_valid_custom_template() {
        let td = TempDir::new().unwrap();

        let template_content = r#"
<template>
  <prompt>
    <header>Header</header>
    <file>File</file>
    <footer>Footer</footer>
  </prompt>

  <part>
    <header>Part start</header>
    <footer>Part end</footer>
    <pending>If part pending</pending>
  </part>
</template>
"#;

        let template_path = td.mkfile_with_contents("template.md", template_content);

        let result = read_and_parse_template(Some(template_path));

        assert!(result.is_ok());
        let template_parts = result.unwrap();
        assert_eq!(template_parts.prompt.header.trim(), "Header");
        assert_eq!(template_parts.prompt.file.trim(), "File");
        assert_eq!(template_parts.prompt.footer.trim(), "Footer");
    }

    #[test]
    fn test_read_and_parse_template_with_invalid_template() {
        let td = TempDir::new().unwrap();
        let invalid_template_content = "This template is missing required tags";

        let template_path =
            td.mkfile_with_contents("invalid_template.md", invalid_template_content);

        let result = read_and_parse_template(Some(template_path));

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Opening tag <template> not found in the provided text."
        );
    }

    #[test]
    fn test_path_to_custom_template_template_provided_via_cli() {
        let td = TempDir::new().unwrap();
        let custom_template_path = td.mkfile("custom_template.txt");

        let cli = Cli::parse_from(&[
            "quagga",
            "--template",
            custom_template_path.to_str().unwrap(),
        ]);

        let result = path_to_custom_template(&cli);

        assert_eq!(result.unwrap(), custom_template_path);
    }

    #[test]
    fn test_path_to_custom_template_no_quagga_template_option_set() {
        let td = TempDir::new().unwrap();
        td.mkfile(".quagga_template");

        let mut cli = Cli::parse_from(&["quagga", "--no-quagga-template"]);
        cli.root = td.path_buf();

        let result = path_to_custom_template(&cli);

        assert!(result.is_none());
    }

    #[test]
    fn test_path_to_custom_template_quagga_template_in_project_directory() {
        let project_dir = TempDir::new().unwrap();
        let project_template_path = project_dir.mkfile(".quagga_template");

        let mut cli = Cli::parse_from(&["quagga"]);
        cli.root = project_dir.path_buf();

        let result = path_to_custom_template(&cli);

        assert_eq!(result.unwrap(), project_template_path);
    }

    #[test]
    fn test_path_to_custom_template_no_template_found() {
        let project_dir = TempDir::new().unwrap();

        let mut cli = Cli::parse_from(&["quagga"]);
        cli.root = project_dir.path_buf();

        let result = path_to_custom_template(&cli);

        assert!(result.is_none());
    }
}
