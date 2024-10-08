use crate::template::template::{PartTemplate, PromptTemplate, Template};

/// Parses the entire template string into a `Template` struct.
///
/// # Arguments
///
/// * `text` - The complete template string.
///
/// # Returns
///
/// * `Ok(Template)` containing the parsed template sections.
/// * `Err(String)` with an error message if parsing fails.
pub fn parse_template(text: &str) -> Result<Template, String> {
    let template_content = text_inside_tag(text, "template")?;
    let prompt_content = text_inside_tag(&template_content, "prompt")?;
    let part_content = text_inside_tag(&template_content, "part")?;
    let prompt = parse_prompt_section(&prompt_content)?;
    let part = parse_part_section(&part_content)?;
    let template = Template { prompt, part };

    Ok(template)
}

fn parse_part_section(part_content: &str) -> Result<PartTemplate, String> {
    let header = text_inside_tag(part_content, "header")?;
    let footer = text_inside_tag(part_content, "footer")?;
    let pending = text_inside_tag(part_content, "pending")?;

    Ok(PartTemplate {
        header,
        footer,
        pending,
    })
}

fn parse_prompt_section(prompt_content: &str) -> Result<PromptTemplate, String> {
    let header = text_inside_tag(prompt_content, "header")?;
    let file = text_inside_tag(prompt_content, "file")?;
    let footer = text_inside_tag(prompt_content, "footer")?;

    Ok(PromptTemplate {
        header,
        file,
        footer,
    })
}

/// Extracts the content enclosed between the first opening and last closing tag from the given text:
/// For example, for text: "before <tag>content</tag> after" it will extract "content".
///
/// # Arguments
///
/// * `text` - The complete template string to parse.
/// * `tag` - The specific tag name (e.g., "template" or "prompt").
///
/// # Returns
///
/// * `Ok(String)` containing the extracted section.
/// * `Err(String)` with an error message if parsing fails.
pub fn text_inside_tag(text: &str, tag: &str) -> Result<String, String> {
    let opening_tag = format!("<{}>", tag);
    let closing_tag = format!("</{}>", tag);

    let start = text
        .find(&opening_tag)
        .ok_or_else(|| format!("Opening tag <{}> not found in the provided text.", tag))?;

    let end = text
        .rfind(&closing_tag)
        .ok_or_else(|| format!("Closing tag </{}> not found in the provided text.", tag))?;

    if end < start {
        return Err(format!(
            "Closing tag </{}> found before opening tag <{}>.",
            tag, tag
        ));
    }

    let content = &text[start + opening_tag.len()..end];
    let trimmed_content = trim_indentation(content);
    Ok(trimmed_content)
}

/// Removes the common leading whitespace from each line in the given content.
fn trim_indentation(content: &str) -> String {
    let lines: Vec<&str> = content.split("\n").collect();

    // Determine the minimum indentation (number of leading spaces) among non-empty lines
    let min_indent = lines
        .iter()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.len() - line.trim_start().len())
        .min()
        .unwrap_or(0);

    lines
        .iter()
        .map(|line| {
            if line.len() >= min_indent {
                &line[min_indent..]
            } else {
                if line.trim().is_empty() {
                    ""
                } else {
                    line
                }
            }
        })
        .collect::<Vec<&str>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_template_success() {
        let text = r#"
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

        let result = parse_template(text);

        assert!(result.is_ok());
        let template = result.unwrap();

        // Prompt
        assert_eq!(template.prompt.header, "Header");
        assert_eq!(template.prompt.file, "File");
        assert_eq!(template.prompt.footer, "Footer");

        // Parts
        assert_eq!(template.part.header, "Part start");
        assert_eq!(template.part.footer, "Part end");
        assert_eq!(template.part.pending, "If part pending");
    }

    #[test]
    fn test_parse_template_missing_tag() {
        let text = r#"
<template>
  <prompt>
    <header>Header</header>
    <file>File</file>
    <footer>Footer</footer>

  <part>
    <header>Part start</header>
    <footer>Part end</footer>
    <pending>If part pending</pending>
  </part>
</template>
"#;

        let result = parse_template(text);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Closing tag </prompt> not found in the provided text."
        );
    }

    #[test]
    fn test_text_inside_tag_success() {
        let text = "before
<template>
Content inside template.
</template> after";

        let result = text_inside_tag(text, "template").unwrap();

        let expected = "
Content inside template.
";

        assert_eq!(result, expected);
    }

    #[test]
    fn test_text_inside_tag_remove_indentation_spaces() {
        let text = "
        <template>
          Content inside template.
        </template>";

        let result = text_inside_tag(text, "template").unwrap();

        let expected = "
Content inside template.
";

        assert_eq!(result, expected);
    }

    #[test]
    fn test_text_inside_tag_mixed_indentation() {
        let text = r#"
<template>
    <prompt>
        Line one
          Line two with extra spaces
        Line three
    </prompt>
</template>
"#;

        let result = text_inside_tag(text, "prompt").unwrap();

        let expected = "
Line one
  Line two with extra spaces
Line three
";

        assert_eq!(result, expected);
    }

    #[test]
    fn test_text_inside_tag_missing_opening() {
        let text = "Content without opening tag.\n</template>";
        let result = text_inside_tag(text, "template");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Opening tag <template> not found in the provided text."
        );
    }

    #[test]
    fn test_text_inside_tag_missing_closing() {
        let text = "<template>\nContent without closing tag.";
        let result = text_inside_tag(text, "template");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Closing tag </template> not found in the provided text."
        );
    }

    #[test]
    fn test_text_inside_tag_closing_before_opening() {
        let text = "</template>\nContent before opening tag.\n<template>";
        let result = text_inside_tag(text, "template");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Closing tag </template> found before opening tag <template>."
        );
    }

    #[test]
    fn test_text_inside_tag_multiple_tags() {
        let text = "<template>First content.<template>Some other text.</template> Second content.</template>";
        let result = text_inside_tag(text, "template").unwrap();
        assert_eq!(
            result,
            "First content.<template>Some other text.</template> Second content."
        );
    }

    #[test]
    fn test_text_inside_tag_empty_content() {
        let text = "<template></template>";
        let result = text_inside_tag(text, "template").unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_trim_indentation_uniform() {
        let content = "
          Line one
          Line two
          Line three
        ";

        let result = trim_indentation(content);

        let expected = "
Line one
Line two
Line three
";

        assert_eq!(result, expected);
    }

    #[test]
    fn test_trim_indentation_varying() {
        let content = "
            Line one
              Line two with extra spaces
            Line three";

        let expected = "
Line one
  Line two with extra spaces
Line three";

        let result = trim_indentation(content);
        assert_eq!(result, expected);
    }
}
