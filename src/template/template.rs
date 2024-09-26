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
/// 2. Removes comments from the template.
/// 3. Validates the template to ensure it contains required tags.
/// 4. Parses the template into its components: header, item, and footer.
///
/// # Arguments
///
/// * `template_path` - An `Option<PathBuf>` specifying the path to the template file.
///
/// # Returns
///
/// * `Ok(TemplateParts)` containing the parsed template components.
/// * `Err<Box<dyn Error>>` if an error occurs during reading, validation, or parsing.
pub fn read_and_validate_template(
    template_path: Option<PathBuf>,
) -> Result<TemplateParts, Box<dyn Error>> {
    let template_content = read_template(template_path)?;
    let cleaned_template = remove_comments(&template_content);
    validator::validate(&cleaned_template)?;
    let parsed_template = parse_template(&cleaned_template)?;
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

/// Removes comments from the entire template string.
/// Comments start with `{{#}}` and extend to the end of the line.
///
/// - Lines that start with optional whitespace followed by `{{#}}` are completely removed, including the line break.
/// - Inline comments starting with `{{#}}` are removed from the point where `{{#}}` appears.
///
/// # Arguments
///
/// * `template` - A string slice representing the template.
///
/// # Returns
///
/// A `String` with all comments removed.
pub fn remove_comments(template: &str) -> String {
    template
        .lines()
        .filter_map(|line| {
            let trimmed_line = line.trim_start();
            if trimmed_line.starts_with("{{#}}") {
                None
            } else {
                let uncommented_line = if let Some(index) = line.find("{{#}}") {
                    &line[..index]
                } else {
                    line
                };
                Some(uncommented_line)
            }
        })
        .collect::<Vec<&str>>()
        .join("\n")
}

/// Parses the template string into header, item, and footer segments.
///
/// # Arguments
///
/// * `template_content` - A string slice containing the template text without comments.
///
/// # Returns
///
/// * `Ok(TemplateParts)` containing `header`, `item`, and `footer` strings.
/// * `Err<io::Error>` if an error occurs during parsing.
pub fn parse_template(template_content: &str) -> io::Result<TemplateParts> {
    let mut lines = template_content.lines();

    let mut header_lines = Vec::new();
    let mut item_lines = Vec::new();
    let mut footer_lines = Vec::new();

    let mut in_item = false;

    while let Some(line) = lines.next() {
        let trimmed_line = line.trim_start();

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
    fn test_remove_comments() {
        let template = "\
Line before comment{{#}} This is a comment
Line without comment
{{#}} Full line comment
Another line {{#}} Another comment";

        let expected = "\
Line before comment
Line without comment
Another line ";

        let result = remove_comments(template);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_remove_comments_no_comments() {
        let template = "Line one\nLine two\nLine three";

        let result = remove_comments(template);
        assert_eq!(result, "Line one\nLine two\nLine three");
    }

    #[test]
    fn test_remove_comments_only_comments() {
        let template = "{{#}} Comment one\n{{#}} Comment two";

        let result = remove_comments(template);
        assert_eq!(result, "");
    }

    #[test]
    fn test_remove_comments_skip_comment_lines_with_whitespace() {
        let template = "\
{{#}} Comment line
  {{#}} Indented comment line
Code line
Code line{{#}} Inline comment
";
        let expected = "\
Code line
Code line";

        let result = remove_comments(template);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_template_with_indented_tags() {
        let template = r#"
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

    #[test]
    fn test_read_and_validate_template_with_default_template() {
        let result = read_and_validate_template(None);
        assert!(result.is_ok());
        let template_parts = result.unwrap();
        assert!(template_parts.header.is_empty());
        assert!(!template_parts.item.is_empty());
        assert!(template_parts.footer.is_empty());
    }

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

    #[test]
    fn test_read_and_validate_template_with_comments() {
        let template = "\
Header line
{{#}} This is a comment
{{HEADER}}
Item line{{#}} Inline comment
{{CONTENT}}
{{FOOTER}}
Footer line
";
        let td = TempDir::new().unwrap();
        let template_path = td.mkfile_with_contents("template.txt", template);

        let result = read_and_validate_template(Some(template_path)).unwrap();

        assert_eq!(result.header.trim(), "Header line");
        assert_eq!(result.item.trim(), "Item line\n{{CONTENT}}");
        assert_eq!(result.footer.trim(), "Footer line");
    }
}
