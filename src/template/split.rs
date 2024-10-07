use crate::template::template::Template;

/// Splits the concatenated content into multiple parts based on the maximum allowed characters.
///
/// # Arguments
///
/// * `header` - The global header string.
/// * `files` - A vector of file contents as strings.
/// * `footer` - The global footer string.
/// * `template` - The Template that is used to show part header, footer, and pending text.
/// * `max_part_chars` - The maximum number of characters allowed per part.
///
/// # Returns
///
/// A vector of strings, each representing a part of the split content.
pub fn split_into_parts(
    header: String,
    files: Vec<String>,
    footer: String,
    template: Template,
    max_part_chars: usize,
) -> Vec<String> {
    // First Pass: Determine if all content fits in a single part
    if fits_in_single_part(&header, &files, &footer, max_part_chars) {
        return assemble_single_part(&header, &files, &footer);
    }

    // Multi-Pass: Split into multiple parts
    let parts = create_split_plan(&header, &files, &footer, &template, max_part_chars);
    assemble_multiple_parts(parts, &template, &header, &footer, max_part_chars)
}

/// Checks if the combined header, files, and footer fit within the max_part_chars.
///
/// # Arguments
///
/// * `header` - The global header string.
/// * `files` - A reference to a vector of file contents.
/// * `footer` - The global footer string.
/// * `max_part_chars` - The maximum number of characters allowed per part.
///
/// # Returns
///
/// `true` if all content fits in a single part, `false` otherwise.
fn fits_in_single_part(
    header: &str,
    files: &[String],
    footer: &str,
    max_part_chars: usize,
) -> bool {
    let mut total_length = files.iter().map(|f| f.len() + 1).sum::<usize>();

    if !header.is_empty() {
        total_length += header.len() + 1;
    }

    total_length += footer.len();

    total_length <= max_part_chars
}

/// Assembles all content into a single part without part headers/footers.
///
/// # Arguments
///
/// * `header` - The global header string.
/// * `files` - A reference to a vector of file contents.
/// * `footer` - The global footer string.
///
/// # Returns
///
/// A single part as a string.
fn assemble_single_part(header: &str, files: &[String], footer: &str) -> Vec<String> {
    let mut part = String::new();

    if !header.is_empty() {
        part.push_str(header);
        part.push('\n');
    }

    for file in files {
        part.push_str(file);
        part.push('\n');
    }

    part.push_str(footer);

    vec![part]
}

/// Represents the content of a single part.
/// Each part can have multiple file chunks.
#[derive(Clone)]
struct PartContent {
    /// The file chunks that make up the part.
    /// Each chunk is a string representing a content form one file,
    /// or a part of a file if the file is too large to fit in a single part.
    file_chunks: Vec<String>,
}

/// Creates a split plan determining how to divide files into parts.
///
/// # Arguments
///
/// * `header` - The global header string.
/// * `files` - A reference to a vector of file contents.
/// * `footer` - The global footer string.
/// * `template` - The Template struct.
/// * `max_part_chars` - Maximum characters per part.
///
/// # Returns
///
/// A SplitPlan struct detailing how content is divided.
fn create_split_plan(
    header: &str,
    files: &[String],
    footer: &str,
    template: &Template,
    max_part_chars: usize,
) -> Vec<PartContent> {
    let mut parts: Vec<PartContent> = Vec::new();

    let mut current_part = PartContent {
        file_chunks: Vec::new(),
    };

    let mut current_size = 0;

    // Calculate overhead coming from part header, footer, and pending text
    let part_overhead = calculate_overhead(template);

    for file in files {
        let file_length = file.len() + 1; // +1 for the newline
        if current_size + file_length + part_overhead > max_part_chars {
            // Try to add the entire file
            if file_length + part_overhead > max_part_chars {
                // File is too large, split by lines
                let line_chunks =
                    split_file_by_lines(file, max_part_chars.saturating_sub(part_overhead));

                for chunk in line_chunks {
                    if current_size + chunk.len() + part_overhead > max_part_chars {
                        if !current_part.file_chunks.is_empty() {
                            parts.push(current_part.clone());
                            current_part = PartContent {
                                file_chunks: Vec::new(),
                            };
                            current_size = 0;
                        }
                    }
                    current_part.file_chunks.push(format!("{}\n", chunk));
                    current_size += chunk.len();
                }
            } else {
                // Start a new part
                if !current_part.file_chunks.is_empty() {
                    parts.push(current_part.clone());
                    current_part = PartContent {
                        file_chunks: Vec::new(),
                    };
                    current_size = 0;
                }
                current_part.file_chunks.push(format!("{}\n", file));
                current_size += file_length;
            }
        } else {
            // Add the entire file to the current part
            current_part.file_chunks.push(format!("{}\n", file));
            current_size += file_length;
        }
    }

    // Add the last part if it has any content
    if !current_part.file_chunks.is_empty() {
        parts.push(current_part);
    }

    parts
}

/// Splits a file's content into chunks at line boundaries.
///
/// # Arguments
///
/// * `file_content` - The content of the file.
/// * `max_chunk_size` - The maximum number of characters allowed per chunk.
///
/// # Returns
///
/// A vector of string chunks.
fn split_file_by_lines(file_content: &str, max_chunk_size: usize) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut current_chunk = String::new();

    for line in file_content.lines() {
        let line_with_newline = format!("{}\n", line);

        if current_chunk.len() + line_with_newline.len() > max_chunk_size {
            if !current_chunk.is_empty() {
                // add current chunk to chunks minus the last newline
                chunks.push(current_chunk[..current_chunk.len() - 1].to_string());
                current_chunk.clear();
            }
        }

        current_chunk.push_str(&line_with_newline);
    }

    if !current_chunk.is_empty() {
        // add current chunk to chunks minus the last newline
        chunks.push(current_chunk[..current_chunk.len() - 1].to_string());
    }

    chunks
}

/// Calculates the overhead introduced by part headers, footers, and pending texts.
///
/// # Arguments
///
/// * `template` - The Template struct.
///
/// # Returns
///
/// The total overhead in characters.
fn calculate_overhead(template: &Template) -> usize {
    let mut overhead = 0;

    let part_header = template
        .part
        .header
        .replace("<part-number>", "999")
        .replace("<total-parts>", "999")
        .replace("<parts-remaining>", "999");

    let part_footer = template
        .part
        .footer
        .replace("<part-number>", "999")
        .replace("<total-parts>", "999")
        .replace("<parts-remaining>", "999");

    let part_pending = template
        .part
        .pending
        .replace("<part-number>", "999")
        .replace("<total-parts>", "999")
        .replace("<parts-remaining>", "999");

    overhead += part_header.len() + 1; // +1 for newline
    overhead += part_footer.len() + 1;

    if !template.part.pending.is_empty() {
        overhead += part_pending.len() + 1;
    }

    overhead
}

/// Assembles multiple parts by inserting headers, footers, and replacing placeholders.
///
/// # Arguments
///
/// * `split_plan` - The SplitPlan struct.
/// * `template` - The Template struct.
/// * `header` - The global header string.
/// * `footer` - The global footer string.
/// * `max_part_chars` - Maximum characters per part.
///
/// # Returns
///
/// A vector of assembled parts as strings.
fn assemble_multiple_parts(
    parts: Vec<PartContent>,
    template: &Template,
    header: &str,
    footer: &str,
    max_part_chars: usize,
) -> Vec<String> {
    let mut assembled_parts: Vec<String> = Vec::new();
    let total_parts = parts.len();

    for (i, part) in parts.iter().enumerate() {
        let mut part_content = String::new();

        // Add part header
        let part_header = replace_placeholders(&template.part.header, i + 1, total_parts);

        part_content.push_str(&part_header);
        part_content.push('\n');

        // Add global header only in the first part
        if i == 0 && !header.is_empty() {
            part_content.push_str(header);
            part_content.push('\n');
        }

        // Add file chunks
        for chunk in &part.file_chunks {
            part_content.push_str(chunk);
        }

        // Add global footer only in the last part
        if i == total_parts - 1 && !footer.is_empty() {
            part_content.push_str(footer);
            part_content.push('\n');
        }

        // Add part footer
        let part_footer = replace_placeholders(&template.part.footer, i + 1, total_parts);

        part_content.push_str(&part_footer);
        part_content.push('\n');

        // Add pending text if not the last part
        if i < total_parts - 1 && !template.part.pending.is_empty() {
            let pending_text = replace_placeholders(&template.part.pending, i + 1, total_parts);

            part_content.push_str(&pending_text);
            part_content.push('\n');
        }

        assembled_parts.push(part_content.trim_end().to_string());
    }

    assembled_parts
}

/// Replaces `<part-number>` and `<total-parts>` placeholders in the text.
///
/// # Arguments
///
/// * `text` - The template text containing placeholders.
/// * `part_number` - The current part number.
/// * `total_parts` - The total number of parts.
///
/// # Returns
///
/// A new string with placeholders replaced.
fn replace_placeholders(text: &str, part_number: usize, total_parts: usize) -> String {
    let parts_remaining = total_parts.saturating_sub(part_number);

    text.replace("<part-number>", &part_number.to_string())
        .replace("<total-parts>", &total_parts.to_string())
        .replace("<parts-remaining>", &parts_remaining.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::template::template::{PartSection, PromptSection, Template};

    #[test]
    fn test_split_into_parts_single_part_fit_exactly() {
        let header = "Header".to_string();
        let footer = "Footer".to_string();
        let files = vec!["File1".to_string(), "File2".to_string()];

        let template = Template {
            prompt: PromptSection::default(),
            part: PartSection {
                header: "== Part <part-number> OF <total-parts> ==".to_string(),
                footer: "== Part END <part-number> OF <total-parts> ==".to_string(),
                pending: "This is only a part of the code (<parts-remaining> remaining)"
                    .to_string(),
            },
        };

        let max_part_chars = 25; // Exact size of header, files, and footer

        let parts = split_into_parts(
            header.clone(),
            files.clone(),
            footer.clone(),
            template,
            max_part_chars,
        );

        assert_eq!(parts.len(), 1);

        let expected = r#"Header
File1
File2
Footer"#;

        assert_eq!(parts[0], expected);
    }

    #[test]
    fn test_split_into_parts() {
        let header = "Header".to_string();
        let footer = "Footer".to_string();
        let files = vec!["File1".to_string(), "File2".to_string()];

        let template = Template {
            prompt: PromptSection::default(),
            part: PartSection {
                header: "== Part <part-number> OF <total-parts> ==".to_string(),
                footer: "== Part END <part-number> OF <total-parts> ==".to_string(),
                pending: "This is only a part of the code (<parts-remaining> remaining)"
                    .to_string(),
            },
        };

        let max_part_chars = 24;

        let parts = split_into_parts(
            header.clone(),
            files.clone(),
            footer.clone(),
            template,
            max_part_chars,
        );

        assert_eq!(parts.len(), 2);

        let expected = r#"== Part 1 OF 2 ==
Header
File1
== Part END 1 OF 2 ==
This is only a part of the code (1 remaining)"#;

        assert_eq!(parts[0], expected);

        let expected = r#"== Part 2 OF 2 ==
File2
Footer
== Part END 2 OF 2 =="#;

        assert_eq!(parts[1], expected);
    }

    #[test]
    fn test_split_into_parts_multiple_parts_with_placeholders() {
        let header = "Header".to_string();
        let footer = "Footer".to_string();

        let files = vec![
            "Line1".repeat(10),
            "Line2".repeat(10),
            "Line3".repeat(10),
            "Line4".repeat(10),
            "Line5".repeat(10),
        ];

        let part = PartSection {
            header: "== Part <part-number> OF <total-parts> ==".to_string(),
            footer: "== Part END <part-number> OF <total-parts> ==".to_string(),
            pending: "This is only a part of the code (<parts-remaining> remaining)".to_string(),
        };

        let template = Template {
            prompt: PromptSection::default(),
            part,
        };

        let max_part_chars = 267;

        let parts = split_into_parts(
            header.clone(),
            files.clone(),
            footer.clone(),
            template,
            max_part_chars,
        );

        assert_eq!(parts.len(), 2);

        let expected = r#"== Part 1 OF 2 ==
Header
Line1Line1Line1Line1Line1Line1Line1Line1Line1Line1
Line2Line2Line2Line2Line2Line2Line2Line2Line2Line2
Line3Line3Line3Line3Line3Line3Line3Line3Line3Line3
== Part END 1 OF 2 ==
This is only a part of the code (1 remaining)"#;

        assert_eq!(parts[0], expected);

        let expected = r#"== Part 2 OF 2 ==
Line4Line4Line4Line4Line4Line4Line4Line4Line4Line4
Line5Line5Line5Line5Line5Line5Line5Line5Line5Line5
Footer
== Part END 2 OF 2 =="#;

        assert_eq!(parts[1], expected);
    }

    #[test]
    fn test_split_into_parts_no_files() {
        let header = "Header".to_string();
        let footer = "Footer".to_string();
        let files: Vec<String> = vec![];
        let part = PartSection {
            header: "=== PART <part-number> OF <total-parts> ===".to_string(),
            footer: "=== END OF PART <part-number> ===".to_string(),
            pending: "Please wait for the next part...".to_string(),
        };
        let template = Template {
            prompt: PromptSection::default(),
            part,
        };
        let max_part_chars = 50;

        let parts = split_into_parts(
            header.clone(),
            files.clone(),
            footer.clone(),
            template,
            max_part_chars,
        );

        // Expecting a single part with header and footer only
        assert_eq!(parts.len(), 1);
        let expected = format!("Header\nFooter");
        assert_eq!(parts[0], expected);
    }

    #[test]
    fn test_split_file_by_lines_empty_content() {
        let file_content = "";
        let max_chunk_size = 10;
        let result = split_file_by_lines(file_content, max_chunk_size);
        assert!(result.is_empty());
    }

    #[test]
    fn test_split_file_by_lines_single_line_fits() {
        let file_content = "1234567890"; // 10 characters
        let max_chunk_size = 10;

        let result = split_file_by_lines(file_content, max_chunk_size);

        let expected = vec!["1234567890".to_string()];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_split_file_by_lines_single_line_exceeds() {
        let file_content = "This line is definitely longer than the maximum chunk size.";
        let max_chunk_size = 10;

        let result = split_file_by_lines(file_content, max_chunk_size);

        let expected =
            vec!["This line is definitely longer than the maximum chunk size.".to_string()];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_split_file_by_lines_multiple_lines_fit_one_chunk() {
        let file_content = "Line1
Line2
Line3";

        let max_chunk_size = 20;

        let result = split_file_by_lines(file_content, max_chunk_size);

        let expected = vec!["Line1
Line2
Line3"
            .to_string()];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_split_file_by_lines_each_line_separate_chunks() {
        let file_content = "Short
Another Short
Yet Another Short";

        let max_chunk_size = 10; // Each line plus newline exceeds 10

        let result = split_file_by_lines(file_content, max_chunk_size);

        let expected = vec![
            "Short".to_string(),
            "Another Short".to_string(),
            "Yet Another Short".to_string(),
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_split_file_first_two_lines_fit_one_chunk_exactly() {
        let file_content = "12345
67890
absde";

        let max_chunk_size = 12; // first two lines plus newline fit exactly

        let result = split_file_by_lines(file_content, max_chunk_size);

        let expected = vec![
            "12345
67890"
                .to_string(),
            "absde".to_string(),
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_split_file_by_lines_zero_max_chunk_size() {
        let file_content = "Line1\nLine2";
        let max_chunk_size = 0;
        let result = split_file_by_lines(file_content, max_chunk_size);
        let expected = vec!["Line1".to_string(), "Line2".to_string()];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_split_file_by_lines_max_chunk_smaller_than_any_line() {
        let file_content = "Short\nMedium Length\nLonger Line Than Max";
        let max_chunk_size = 5; // All lines plus newline exceed 5

        let result = split_file_by_lines(file_content, max_chunk_size);

        let expected = vec![
            "Short".to_string(),
            "Medium Length".to_string(),
            "Longer Line Than Max".to_string(),
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_split_file_by_lines_multiple_consecutive_newlines() {
        let file_content = "Line1\n\nLine3\n\n\n\n\nLine6";
        let max_chunk_size = 15;
        let result = split_file_by_lines(file_content, max_chunk_size);
        let expected = vec!["Line1\n\nLine3\n\n".to_string(), "\n\nLine6".to_string()];
        assert_eq!(result, expected);
    }

    /// Test when all lines are empty (only newlines).
    #[test]
    fn test_split_file_by_lines_all_empty_lines() {
        let file_content = "\n\n\n";
        let max_chunk_size = 2;
        let expected = vec!["\n".to_string(), "".to_string()];
        let result = split_file_by_lines(file_content, max_chunk_size);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_split_file_by_lines_mixed_line_lengths() {
        let file_content = "Short
This line is quite long and exceeds the chunk size.
Mid
Another long line that should be split properly.";

        let max_chunk_size = 30;

        let result = split_file_by_lines(file_content, max_chunk_size);

        let expected = vec![
            "Short".to_string(),
            "This line is quite long and exceeds the chunk size.".to_string(),
            "Mid".to_string(),
            "Another long line that should be split properly.".to_string(),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_split_file_by_lines_lines_exactly_max_size() {
        let file_content = "1234567890
abcdefghij
ABCDEFGHIJ";

        let max_chunk_size = 11; // Each line + newline is 11 characters

        let result = split_file_by_lines(file_content, max_chunk_size);

        let expected = vec![
            "1234567890".to_string(),
            "abcdefghij".to_string(),
            "ABCDEFGHIJ".to_string(),
        ];

        assert_eq!(result, expected);
    }
}
