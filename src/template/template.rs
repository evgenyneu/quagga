use std::error::Error;
use std::fs;
use std::io;
use std::path::PathBuf;

use super::validator;

/// The default template embedded into the executable.
const DEFAULT_TEMPLATE: &str = include_str!("../../templates/default.txt");

/// Represents the parsed parts of a template.
#[derive(Debug)]
pub struct TemplateParts {
    pub header: String,
    pub item: String,
    pub footer: String,
}

/// Reads, validates, and parses a template from a given path or the default template.
///
/// This function performs the following steps:
/// 1. Reads the template content from the provided path or uses the default template.
/// 2. Validates the template to ensure it contains required tags.
/// 3. Parses the template into its components: header, item, and footer.
///
/// # Arguments
///
/// * `template_path` - An `Option<PathBuf>` specifying the path to the template file.
///
/// # Returns
///
/// * `Ok(TemplateParts)` containing the parsed template components.
/// * `Err<Box<dyn Error>>` if an error occurs during reading, validation, or parsing.
///
/// # Errors
///
/// This function will return an error if:
/// - Reading the template file fails.
/// - Validation of the template fails.
pub fn read_and_validate_template(
    template_path: Option<PathBuf>,
) -> Result<TemplateParts, Box<dyn Error>> {
    let template_content = read_template(template_path)?;
    validator::validate(&template_content)?;
    let parsed_template = parse_template(&template_content)?;
    Ok(parsed_template)
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

/// Parses the template string into header, item, and footer segments.
/// It ignores comment lines starting with `{{#}}`, allowing leading spaces or tabs.
///
/// # Arguments
///
/// * `template_content` - A string slice containing the template text.
///
/// # Returns
///
/// * `Ok(TemplateParts)` - Struct containing `header`, `item`, and `footer` strings.
/// * `Err(io::Error)` - If an error occurs during parsing.
pub fn parse_template(template_content: &str) -> io::Result<TemplateParts> {
    let mut lines = template_content.lines();

    let mut header_lines = Vec::new();
    let mut item_lines = Vec::new();
    let mut footer_lines = Vec::new();

    let mut in_item = false;

    while let Some(line) = lines.next() {
        let line = line.trim_end(); // Remove trailing newline characters

        // Trim leading whitespace for tag matching
        let trimmed_line = line.trim_start();

        // Skip comment lines
        if trimmed_line.starts_with("{{#}}") {
            continue;
        }

        match trimmed_line {
            "{{HEADER}}" => {
                in_item = true;
                continue;
            }
            "{{FOOTER}}" => {
                in_item = false;
                continue;
            }
            _ => {}
        }

        if in_item {
            item_lines.push(line);
        } else if item_lines.is_empty() {
            header_lines.push(line);
        } else {
            footer_lines.push(line);
        }
    }

    Ok(TemplateParts {
        header: header_lines.join("\n"),
        item: item_lines.join("\n"),
        footer: footer_lines.join("\n"),
    })
}

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

    #[test]
    fn test_parse_template_with_indented_tags() {
        let template = r#"
{{#}} Comment line
Header text1
Header text2
{{HEADER}}
Item header
{{CONTENT}}
Item footer
{{FOOTER}}
Footer text1
Footer text1
"#;

        let result = parse_template(template).unwrap();

        assert_eq!(result.header.trim(), "Header text1\nHeader text2");
        assert_eq!(result.item.trim(), "Item header\n{{CONTENT}}\nItem footer");
        assert_eq!(result.footer.trim(), "Footer text1\nFooter text1");
    }

    // #[test]
    // fn test_read_and_validate_template_with_default_template() {
    //     let result = read_and_validate_template(None);
    //     assert!(result.is_ok());
    //     let template_parts = result.unwrap();
    //     assert!(!template_parts.header.is_empty());
    //     assert!(!template_parts.item.is_empty());
    //     assert!(!template_parts.footer.is_empty());
    // }

    #[test]
    fn test_read_and_validate_template_with_valid_custom_template() {
        let td = TempDir::new().unwrap();
        let template_content = r#"
Global Header
{{HEADER}}
Item Section
{{CONTENT}}
{{FOOTER}}
Global Footer
"#;
        let template_path = td.mkfile_with_contents("template.txt", template_content);
        let result = read_and_validate_template(Some(template_path));
        assert!(result.is_ok());
        let template_parts = result.unwrap();
        assert_eq!(template_parts.header.trim(), "Global Header");
        assert_eq!(template_parts.item.trim(), "Item Section\n{{CONTENT}}");
        assert_eq!(template_parts.footer.trim(), "Global Footer");
    }

    #[test]
    fn test_read_and_validate_template_with_invalid_template() {
        let td = TempDir::new().unwrap();
        let invalid_template_content = "This template is missing required tags";
        let template_path =
            td.mkfile_with_contents("invalid_template.txt", invalid_template_content);
        let result = read_and_validate_template(Some(template_path));
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Template is invalid: Missing {{HEADER}} tag."
        );
    }
}
