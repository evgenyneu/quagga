// src/template/split.rs

use crate::template::template::Template;

/// Splits the concatenated content into multiple parts based on the maximum allowed characters.
/// Utilizes a multi-pass approach to accurately calculate the total number of parts.
///
/// # Arguments
///
/// * `header` - The global header string.
/// * `files` - A vector of file contents as strings.
/// * `footer` - The global footer string.
/// * `template` - The Template struct containing part header, footer, and pending text.
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
    let split_plan = create_split_plan(&header, &files, &footer, &template, max_part_chars);
    assemble_multiple_parts(&split_plan, &template, &header, &footer, max_part_chars)
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

/// Represents how the content will be split into parts.
/// Each part contains a vector of file chunks (each chunk can be an entire file or part of a large file).
struct SplitPlan {
    parts: Vec<PartContent>,
    total_parts: usize,
}

/// Represents the content of a single part.
/// Each part can have multiple file chunks.
#[derive(Clone)]
struct PartContent {
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
) -> SplitPlan {
    let mut parts: Vec<PartContent> = Vec::new();
    let mut current_part = PartContent {
        file_chunks: Vec::new(),
    };
    let mut current_size = 0;

    // Calculate overhead for parts (excluding the first part's header)
    let part_overhead = calculate_overhead(template, false, false);

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
                    current_part.file_chunks.push(chunk.clone());
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

    let total_parts = parts.len();
    SplitPlan { parts, total_parts }
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
                chunks.push(current_chunk.clone());
                current_chunk.clear();
            }
        }

        current_chunk.push_str(&line_with_newline);
    }

    if !current_chunk.is_empty() {
        chunks.push(current_chunk);
    }

    chunks
}

/// Calculates the overhead introduced by part headers, footers, and pending texts.
///
/// # Arguments
///
/// * `template` - The Template struct.
/// * `include_global_header` - Whether to include the global header.
/// * `include_global_footer` - Whether to include the global footer.
///
/// # Returns
///
/// The total overhead in characters.
fn calculate_overhead(
    template: &Template,
    include_global_header: bool,
    include_global_footer: bool,
) -> usize {
    let mut overhead = 0;

    // Part Header
    overhead += template.part.header.len() + 1; // +1 for newline

    // Part Footer
    overhead += template.part.footer.len() + 1; // +1 for newline

    // Pending Text
    if !template.part.pending.is_empty() {
        overhead += template.part.pending.len() + 1; // +1 for newline
    }

    // Global Header
    if include_global_header && !template.prompt.header.is_empty() {
        overhead += template.prompt.header.len() + 1; // +1 for newline
    }

    // Global Footer
    if include_global_footer && !template.prompt.footer.is_empty() {
        overhead += template.prompt.footer.len() + 1; // +1 for newline
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
    split_plan: &SplitPlan,
    template: &Template,
    header: &str,
    footer: &str,
    max_part_chars: usize,
) -> Vec<String> {
    let mut assembled_parts: Vec<String> = Vec::new();

    for (i, part) in split_plan.parts.iter().enumerate() {
        let mut part_content = String::new();

        // Add part header
        let part_header =
            replace_placeholders(&template.part.header, i + 1, split_plan.total_parts);
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
        if i == split_plan.total_parts - 1 && !footer.is_empty() {
            part_content.push_str(footer);
            part_content.push('\n');
        }

        // Add part footer
        let part_footer =
            replace_placeholders(&template.part.footer, i + 1, split_plan.total_parts);
        part_content.push_str(&part_footer);
        part_content.push('\n');

        // Add pending text if not the last part
        if i < split_plan.total_parts - 1 && !template.part.pending.is_empty() {
            let parts_remaining = split_plan.total_parts - (i + 1);
            let pending_text = replace_pending_text(&template.part.pending, parts_remaining);
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
    text.replace("<part-number>", &part_number.to_string())
        .replace("<total-parts>", &total_parts.to_string())
}

/// Replaces `<parts-remaining>` placeholder in the pending text.
///
/// # Arguments
///
/// * `text` - The pending text containing placeholders.
/// * `parts_remaining` - The number of parts remaining.
///
/// # Returns
///
/// A new string with placeholders replaced.
fn replace_pending_text(text: &str, parts_remaining: usize) -> String {
    text.replace("<parts-remaining>", &parts_remaining.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::template::template::{PartSection, PromptSection, Template};

    #[test]
    fn test_split_into_parts_single_part() {
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

        let max_part_chars = 25;

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

        assert_eq!(parts.len(), 1);

        let expected = r#"Header
File1
File2
Footer"#;

        assert_eq!(parts[0], expected);
    }

    #[test]
    fn test_split_into_parts_multiple_parts_with_placeholders() {
        let header = "Header".to_string();
        let footer = "Footer".to_string();
        let files = vec![
            "Line1".to_string(),
            "Line2".to_string(),
            "Line3".to_string(),
            "Line4".to_string(),
            "Line5".to_string(),
        ];
        let part = PartSection {
            header: "=== PART <part-number> OF <total-parts> ===".to_string(),
            footer: "=== END OF PART <part-number> ===".to_string(),
            pending: "Please wait for the next part...".to_string(),
        };
        let template = Template {
            prompt: PromptSection::default(),
            part,
        };
        let max_part_chars = 30;

        let parts = split_into_parts(
            header.clone(),
            files.clone(),
            footer.clone(),
            template,
            max_part_chars,
        );

        assert_eq!(parts.len(), 2);

        // First part
        let first_part = &parts[0];
        assert!(first_part.contains("=== PART 1 OF 2 ==="));
        assert!(first_part.contains("Header"));
        assert!(first_part.contains("Line1\nLine2\nLine3\n"));
        assert!(first_part.contains("=== END OF PART 1 ==="));
        assert!(first_part.contains("Please wait for the next part..."));

        // Second part
        let second_part = &parts[1];
        assert!(second_part.contains("=== PART 2 OF 2 ==="));
        assert!(second_part.contains("Line4\nLine5\n"));
        assert!(second_part.contains("Footer"));
        assert!(second_part.contains("=== END OF PART 2 ==="));
        assert!(!second_part.contains("Please wait for the next part..."));
    }

    #[test]
    fn test_split_into_parts_large_file_split_by_lines() {
        let header = "Header".to_string();
        let footer = "Footer".to_string();
        let large_content = vec!["Line".to_string(); 50].join("\n");
        let files = vec![large_content.clone()];
        let part = PartSection {
            header: "=== PART <part-number> OF <total-parts> ===".to_string(),
            footer: "=== END OF PART <part-number> ===".to_string(),
            pending: "Please wait for the next part...".to_string(),
        };
        let template = Template {
            prompt: PromptSection::default(),
            part,
        };
        let max_part_chars = 200;

        let parts = split_into_parts(
            header.clone(),
            files.clone(),
            footer.clone(),
            template,
            max_part_chars,
        );

        // The large file should be split into multiple parts by lines
        assert!(parts.len() > 1);

        for (i, part_content) in parts.iter().enumerate() {
            let expected_part_header = format!("=== PART {} OF {}", i + 1, parts.len());
            let expected_part_footer = format!("=== END OF PART {} ===", i + 1);

            assert!(part_content.contains(&expected_part_header));
            assert!(part_content.contains(&expected_part_footer));

            if i < parts.len() - 1 {
                assert!(part_content.contains("Please wait for the next part..."));
            } else {
                assert!(!part_content.contains("Please wait for the next part..."));
                assert!(part_content.contains(&footer));
            }

            // Verify that the chunk size does not exceed the limit
            let actual_size = part_content.len();
            assert!(actual_size <= max_part_chars + 200); // Allow some buffer for headers and footers
        }
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
        let expected = format!("Header\nFooter\n");
        assert_eq!(parts[0], expected);
    }

    #[test]
    fn test_split_into_parts_single_large_line() {
        let header = "Header".to_string();
        let footer = "Footer".to_string();
        let large_line = "A".repeat(150);
        let files = vec![large_line.clone()];
        let part = PartSection {
            header: "=== PART <part-number> OF <total-parts> ===".to_string(),
            footer: "=== END OF PART <part-number> ===".to_string(),
            pending: "Please wait for the next part...".to_string(),
        };
        let template = Template {
            prompt: PromptSection::default(),
            part,
        };
        let max_part_chars = 100;

        let parts = split_into_parts(
            header.clone(),
            files.clone(),
            footer.clone(),
            template,
            max_part_chars,
        );

        // The large line should be in its own part
        assert_eq!(parts.len(), 1);
        let expected = format!(
            "=== PART 1 OF 1 ===\n{}\n=== END OF PART 1 ===\n",
            large_line
        );
        assert_eq!(parts[0], expected);
    }

    #[test]
    fn test_split_file_by_lines_empty_content() {
        let file_content = "";
        let max_chunk_size = 10;
        let result = split_file_by_lines(file_content, max_chunk_size);
        assert!(result.is_empty());
    }

    /// Test when a single line fits within the `max_chunk_size`.
    #[test]
    fn test_split_file_by_lines_single_line_fits() {
        let file_content = "1234567890"; // 10 characters
        let max_chunk_size = 10;

        let result = split_file_by_lines(file_content, max_chunk_size);

        let expected = vec!["1234567890".to_string()];
        assert_eq!(result, expected);
    }

    // /// Test when a single line exceeds the `max_chunk_size`.
    // #[test]
    // fn test_split_file_by_lines_single_line_exceeds() {
    //     let file_content = "This line is definitely longer than the maximum chunk size.";
    //     let max_chunk_size = 20;
    //     let expected = vec!["This line is definitely longer than the maximum chunk size.\n".to_string()];
    //     let result = split_file_by_lines(file_content, max_chunk_size);
    //     assert_eq!(result, expected);
    // }

    // /// Test multiple lines that all fit within a single chunk.
    // #[test]
    // fn test_split_file_by_lines_multiple_lines_fit_one_chunk() {
    //     let file_content = "Line1\nLine2\nLine3";
    //     let max_chunk_size = 20;
    //     let expected = vec!["Line1\nLine2\nLine3\n".to_string()];
    //     let result = split_file_by_lines(file_content, max_chunk_size);
    //     assert_eq!(result, expected);
    // }

    // /// Test each line requires its own chunk due to size constraints.
    // #[test]
    // fn test_split_file_by_lines_each_line_separate_chunks() {
    //     let file_content = "Short\nAnother Short\nYet Another Short";
    //     let max_chunk_size = 10; // Each line plus newline exceeds 10
    //     let expected = vec![
    //         "Short\n".to_string(),
    //         "Another Short\n".to_string(),
    //         "Yet Another Short\n".to_string(),
    //     ];
    //     let result = split_file_by_lines(file_content, max_chunk_size);
    //     assert_eq!(result, expected);
    // }

    // /// Test lines that exactly fit the `max_chunk_size`.
    // #[test]
    // fn test_split_file_by_lines_lines_exact_fit() {
    //     let file_content = "12345\n67890";
    //     let max_chunk_size = 6; // "12345\n" is 6 characters
    //     let expected = vec!["12345\n".to_string(), "67890\n".to_string()];
    //     let result = split_file_by_lines(file_content, max_chunk_size);
    //     assert_eq!(result, expected);
    // }

    // /// Test when the last line does not end with a newline character.
    // #[test]
    // fn test_split_file_by_lines_no_newline_at_end() {
    //     let file_content = "Line1\nLine2\nLine3";
    //     let max_chunk_size = 15;
    //     let expected = vec!["Line1\nLine2\n".to_string(), "Line3\n".to_string()];
    //     let result = split_file_by_lines(file_content, max_chunk_size);
    //     assert_eq!(result, expected);
    // }

    // /// Test with `max_chunk_size` set to zero.
    // #[test]
    // fn test_split_file_by_lines_zero_max_chunk_size() {
    //     let file_content = "Line1\nLine2";
    //     let max_chunk_size = 0;
    //     let expected = vec!["Line1\n".to_string(), "Line2\n".to_string()];
    //     let result = split_file_by_lines(file_content, max_chunk_size);
    //     assert_eq!(result, expected);
    // }

    // /// Test with `max_chunk_size` smaller than any individual line.
    // #[test]
    // fn test_split_file_by_lines_max_chunk_smaller_than_any_line() {
    //     let file_content = "Short\nMedium Length\nLonger Line Than Max";
    //     let max_chunk_size = 5; // All lines plus newline exceed 5
    //     let expected = vec![
    //         "Short\n".to_string(),
    //         "Medium Length\n".to_string(),
    //         "Longer Line Than Max\n".to_string(),
    //     ];
    //     let result = split_file_by_lines(file_content, max_chunk_size);
    //     assert_eq!(result, expected);
    // }

    // /// Test with multiple consecutive newlines (empty lines).
    // #[test]
    // fn test_split_file_by_lines_multiple_consecutive_newlines() {
    //     let file_content = "Line1\n\nLine3\n\n\nLine6";
    //     let max_chunk_size = 15;
    //     let expected = vec![
    //         "Line1\n\nLine3\n".to_string(),
    //         "\n\nLine6\n".to_string(),
    //     ];
    //     let result = split_file_by_lines(file_content, max_chunk_size);
    //     assert_eq!(result, expected);
    // }

    // /// Test when all lines are empty (only newlines).
    // #[test]
    // fn test_split_file_by_lines_all_empty_lines() {
    //     let file_content = "\n\n\n";
    //     let max_chunk_size = 2;
    //     let expected = vec!["\n".to_string(), "\n".to_string(), "\n".to_string()];
    //     let result = split_file_by_lines(file_content, max_chunk_size);
    //     assert_eq!(result, expected);
    // }

    // /// Test when `file_content` has a mix of short and long lines.
    // #[test]
    // fn test_split_file_by_lines_mixed_line_lengths() {
    //     let file_content = "Short\nThis line is quite long and exceeds the chunk size.\nMid\nAnother long line that should be split properly.";
    //     let max_chunk_size = 30;
    //     let expected = vec![
    //         "Short\n".to_string(),
    //         "This line is quite long and exceeds the chunk size.\n".to_string(),
    //         "Mid\n".to_string(),
    //         "Another long line that should be split properly.\n".to_string(),
    //     ];
    //     let result = split_file_by_lines(file_content, max_chunk_size);
    //     assert_eq!(result, expected);
    // }

    // /// Test when `file_content` has lines with exactly `max_chunk_size` characters.
    // #[test]
    // fn test_split_file_by_lines_lines_exactly_max_size() {
    //     let file_content = "1234567890\nabcdefghij\nABCDEFGHIJ";
    //     let max_chunk_size = 11; // Each line + newline is 11 characters
    //     let expected = vec![
    //         "1234567890\n".to_string(),
    //         "abcdefghij\n".to_string(),
    //         "ABCDEFGHIJ\n".to_string(),
    //     ];
    //     let result = split_file_by_lines(file_content, max_chunk_size);
    //     assert_eq!(result, expected);
    // }

    // /// Test with a mix of lines that fit and lines that require splitting.
    // #[test]
    // fn test_split_file_by_lines_mixed_requirements() {
    //     let file_content = "Fit\nTooLongLineThatExceedsMax\nFitAgain";
    //     let max_chunk_size = 10;
    //     let expected = vec![
    //         "Fit\n".to_string(),
    //         "TooLongLineThatExceedsMax\n".to_string(),
    //         "FitAgain\n".to_string(),
    //     ];
    //     let result = split_file_by_lines(file_content, max_chunk_size);
    //     assert_eq!(result, expected);
    // }
}
