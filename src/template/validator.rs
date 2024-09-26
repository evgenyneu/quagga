use std::error::Error;

/// Validates the template by checking for required tags and returns detailed error messages.
///
/// # Arguments
///
/// * `template_content` - A string slice containing the template text.
///
/// # Returns
///
/// * `Ok(())` if the template is valid.
/// * `Err(Box<dyn Error>)` with a detailed error message if the template is invalid.
pub fn validate(template_content: &str) -> Result<(), Box<dyn Error>> {
    let header_count = template_content.matches("{{HEADER}}").count();
    let footer_count = template_content.matches("{{FOOTER}}").count();

    if header_count == 0 {
        return Err("Template is invalid: Missing {{HEADER}} tag.".into());
    } else if header_count > 1 {
        return Err("Template is invalid: Multiple {{HEADER}} tags found.".into());
    }

    if footer_count == 0 {
        return Err("Template is invalid: Missing {{FOOTER}} tag.".into());
    } else if footer_count > 1 {
        return Err("Template is invalid: Multiple {{FOOTER}} tags found.".into());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_template() {
        let template = "Global header\n{{HEADER}}\nItem\n{{FOOTER}}\nGlobal footer";
        assert!(validate(template).is_ok());
    }

    #[test]
    fn test_missing_header_and_footer() {
        let template = "some text";
        let result = validate(template);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Template is invalid: Missing {{HEADER}} tag."
        );
    }

    #[test]
    fn test_missing_header() {
        let template = "Global header\nItem\n{{FOOTER}}\nGlobal footer";
        let result = validate(template);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Template is invalid: Missing {{HEADER}} tag."
        );
    }

    #[test]
    fn test_missing_footer() {
        let template = "Global header\n{{HEADER}}\nItem\nGlobal footer";
        let result = validate(template);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Template is invalid: Missing {{FOOTER}} tag."
        );
    }

    #[test]
    fn test_multiple_headers() {
        let template = "{{HEADER}}\nItem\n{{HEADER}}\n{{FOOTER}}";
        let result = validate(template);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Template is invalid: Multiple {{HEADER}} tags found."
        );
    }

    #[test]
    fn test_multiple_footers() {
        let template = "{{HEADER}}\nItem\n{{FOOTER}}\n{{FOOTER}}";
        let result = validate(template);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Template is invalid: Multiple {{FOOTER}} tags found."
        );
    }
}
