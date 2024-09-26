/// Checks if the template is valid by ensuring it contains exactly one `{{HEADER}}` and one `{{FOOTER}}` tag.
///
/// # Arguments
///
/// * `template_content` - A string slice containing the template text.
///
/// # Returns
///
/// * `true` if the template is valid.
/// * `false` otherwise.
pub fn is_valid_template(template_content: &str) -> bool {
    let header_count = template_content.matches("{{HEADER}}").count();
    let footer_count = template_content.matches("{{FOOTER}}").count();

    header_count == 1 && footer_count == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_template() {
        let template = "Global header\n{{HEADER}}\nItem\n{{FOOTER}}\nGlobal footer";
        assert!(is_valid_template(template));
    }

    #[test]
    fn test_missing_header_and_footer() {
        let template = "some text";
        assert!(!is_valid_template(template));
    }

    #[test]
    fn test_missing_header() {
        let template = "Global header\nItem\n{{FOOTER}}\nGlobal footer";
        assert!(!is_valid_template(template));
    }

    #[test]
    fn test_missing_footer() {
        let template = "Global header\n{{HEADER}}\nItem\nGlobal footer";
        assert!(!is_valid_template(template));
    }

    #[test]
    fn test_multiple_headers() {
        let template = "{{HEADER}}\nItem\n{{HEADER}}\n{{FOOTER}}";
        assert!(!is_valid_template(template));
    }

    #[test]
    fn test_multiple_footers() {
        let template = "{{HEADER}}\nItem\n{{FOOTER}}\n{{FOOTER}}";
        assert!(!is_valid_template(template));
    }
}
