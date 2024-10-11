use crate::template::template::PartTemplate;

/// Splits the concatenated content into multiple parts based on the maximum allowed characters.
///
/// # Arguments
///
/// * `header` - The global header string.
/// * `files` - A vector of file contents as strings.
/// * `footer` - The global footer string.
/// * `part_template` - The part template containing part header, footer, and pending text.
/// * `max_part_chars` - The maximum number of characters allowed per part.
///
/// # Returns
///
/// A vector of strings, each representing a part of the split content.
pub fn split_into_parts(
    header: String,
    files: Vec<String>,
    footer: String,
    part_template: PartTemplate,
    max_part_chars: usize,
) -> Vec<String> {
    // Determine if all content fits in a single part
    if fits_in_single_part(&header, &files, &footer, max_part_chars) {
        // No need to split into parts
        return assemble_single_part(&header, &files, &footer);
    }

    // Content does not fit into one part - split into multiple parts
    let parts = create_split_plan(&header, &files, &footer, &part_template, max_part_chars);
    assemble_multiple_parts(parts, &part_template, &header, &footer)
}

/// Represents the content of a single part.
/// Each part can contain content of multiple files
#[derive(Clone)]
struct PartContent {
    /// The file content that make up the part.
    /// Each chunk is a string representing content form one file,
    /// or a section of a file if the file is too large to fit in a single part.
    file_chunks: Vec<String>,
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
    let mut total_length = files.iter().map(|f| f.chars().count() + 1).sum::<usize>();

    if !header.is_empty() {
        total_length += header.chars().count() + 1;
    }

    total_length += footer.chars().count();

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

/// Creates a split plan determining how to divide files into parts.
///
/// # Arguments
///
/// * `header` - The global header string.
/// * `files` - A reference to a vector of file contents.
/// * `footer` - The global footer string.
/// * `part_template` - The part template.
/// * `max_part_chars` - Maximum characters per part.
///
/// # Returns
///
/// A vector of `PartContent` structs containing the file content for each part.
fn create_split_plan(
    header: &str,
    files: &[String],
    footer: &str,
    part_template: &PartTemplate,
    max_part_chars: usize,
) -> Vec<PartContent> {
    let part_overhead = calculate_part_overhead(part_template);
    let mut parts = Vec::new();
    let mut current_part_size = 0;

    let mut current_part = PartContent {
        file_chunks: Vec::new(),
    };

    for (i, file) in files.iter().enumerate() {
        let file_length = calculate_file_length(header, footer, files, i, file);

        if current_part_size + file_length + part_overhead > max_part_chars {
            // File does not fit in the current part
            handle_exceeding_size(
                &mut parts,
                &mut current_part,
                &mut current_part_size,
                file,
                part_overhead,
                footer.len(),
                header.len(),
                max_part_chars,
            );
        } else {
            // File fits in the current part
            add_file_to_part(&mut current_part, file, &mut current_part_size, file_length);
        }
    }

    // Add the remaining part
    if !current_part.file_chunks.is_empty() {
        parts.push(current_part);
    }

    parts
}

/// Calculates the length of a file content including header and footer.
///
/// # Arguments
///
/// * `header` - The global header string.
/// * `footer` - The global footer string.
/// * `files` - The list of files.
/// * `index` - Current file index.
/// * `file` - Current file content.
///
/// # Returns
///
/// The total length of the file with header and footer.
fn calculate_file_length(
    header: &str,
    footer: &str,
    files: &[String],
    index: usize,
    file: &str,
) -> usize {
    let is_first = index == 0 && !header.is_empty();
    let is_last = index == files.len() - 1 && !footer.is_empty();

    let header_len = if is_first {
        header.chars().count() + 1
    } else {
        0
    };

    let footer_len = if is_last {
        footer.chars().count() + 1
    } else {
        0
    };

    header_len + file.chars().count() + 1 + footer_len // +1 for newline
}

/// Handles the scenario where adding a file exceeds the maximum part size.
/// It either splits the file or starts a new part.
///
/// # Arguments
///
/// * `parts` - The vector of parts.
/// * `current_part` - The current part being assembled.
/// * `current_size` - The current size of the part.
/// * `file` - The file content.
/// * `part_overhead` - Overhead coming from part header, footer, and pending text.
/// * `footer_len` - Length of the footer.
/// * `header_len` - Length of the header.
/// * `max_size` - Maximum allowed size.
fn handle_exceeding_size(
    parts: &mut Vec<PartContent>,
    current_part: &mut PartContent,
    current_part_size: &mut usize,
    file: &str,
    part_overhead: usize,
    footer_len: usize,
    header_len: usize,
    max_size: usize,
) {
    if file.len() + part_overhead > max_size {
        handle_large_file(
            parts,
            current_part,
            current_part_size,
            file,
            part_overhead,
            footer_len,
            header_len,
            max_size,
        );
    } else {
        start_new_part_if_needed(parts, current_part, current_part_size);
        add_file_to_part(current_part, file, current_part_size, file.len() + 1);
    }
}

// Splits file that is too large to fit in a single part into chunks at line boundaries.
// and creates parts for each chunk.
fn handle_large_file(
    parts: &mut Vec<PartContent>,
    current_part: &mut PartContent,
    current_part_size: &mut usize,
    file: &str,
    part_overhead: usize,
    footer_len: usize,
    header_len: usize,
    max_size: usize,
) {
    let max_chunk_size = max_size.saturating_sub(part_overhead + footer_len + header_len);
    let chunks = split_file_by_lines(file, max_chunk_size);

    for chunk in chunks {
        start_new_part_if_needed(parts, current_part, current_part_size);
        add_chunk_to_part(current_part, &chunk, current_part_size);
    }
}

fn start_new_part_if_needed(
    parts: &mut Vec<PartContent>,
    current_part: &mut PartContent,
    current_size: &mut usize,
) {
    if !current_part.file_chunks.is_empty() {
        parts.push(current_part.clone());
        *current_part = PartContent {
            file_chunks: Vec::new(),
        };
        *current_size = 0;
    }
}

/// Adds a file to the current part.
///
/// # Arguments
///
/// * `current_part` - The current part being assembled.
/// * `file` - The file content.
/// * `current_size` - The current size of the part.
/// * `file_length` - Length of the file to add.
fn add_file_to_part(
    current_part: &mut PartContent,
    file: &str,
    current_size: &mut usize,
    file_length: usize,
) {
    current_part.file_chunks.push(format!("{}\n", file));
    *current_size += file_length;
}

/// Adds a chunk to the current part.
///
/// # Arguments
///
/// * `current_part` - The current part being assembled.
/// * `chunk` - The file chunk.
/// * `current_size` - The current size of the part.
fn add_chunk_to_part(current_part: &mut PartContent, chunk: &str, current_size: &mut usize) {
    current_part.file_chunks.push(format!("{}\n", chunk));
    *current_size += chunk.chars().count() + 1;
}

/// Splits a content of a large file that does not fit into a single part
/// into chunks at line boundaries.
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
    let mut current_chunk_chars = 0;

    for line in file_content.lines() {
        let line_chars = line.chars().count();
        let line_with_newline_chars = line_chars + 1;

        if current_chunk_chars + line_with_newline_chars > max_chunk_size {
            if !current_chunk.is_empty() {
                chunks.push(current_chunk[..current_chunk.len() - 1].to_string());
                current_chunk.clear();
                current_chunk_chars = 0;
            }
        }

        current_chunk.push_str(line);
        current_chunk.push('\n');
        current_chunk_chars += line_with_newline_chars;
    }

    if !current_chunk.is_empty() {
        chunks.push(current_chunk[..current_chunk.len() - 1].to_string());
    }

    chunks
}

/// Estimates the overhead introduced by part headers, footers, and pending texts.
///
/// # Arguments
///
/// * `part_template` - The part template.
///
/// # Returns
///
/// The total overhead in characters.
fn calculate_part_overhead(part_template: &PartTemplate) -> usize {
    let mut overhead = 0;

    // Replace placeholders with large numbers to estimate the overhead
    let part_header = part_template
        .header
        .replace("<part-number>", "999")
        .replace("<total-parts>", "999")
        .replace("<parts-remaining>", "999");

    let part_footer = part_template
        .footer
        .replace("<part-number>", "999")
        .replace("<total-parts>", "999")
        .replace("<parts-remaining>", "999");

    let part_pending = part_template
        .pending
        .replace("<part-number>", "999")
        .replace("<total-parts>", "999")
        .replace("<parts-remaining>", "999");

    overhead += part_header.chars().count() + 1; // +1 for newline
    overhead += part_footer.chars().count() + 1;

    if !part_template.pending.is_empty() {
        overhead += part_pending.chars().count() + 1;
    }

    overhead
}

/// Generates the output text for all parts by concatenating the headers, file contents and footers.
///
/// # Arguments
///
/// * `split_plan` - The SplitPlan struct.
/// * `part_template` - The part_template
/// * `header` - The global header string.
/// * `footer` - The global footer string.
///
/// # Returns
///
/// A vector of assembled parts as strings.
fn assemble_multiple_parts(
    parts: Vec<PartContent>,
    part_template: &PartTemplate,
    header: &str,
    footer: &str,
) -> Vec<String> {
    let mut assembled_parts: Vec<String> = Vec::new();
    let total_parts = parts.len();

    for (i, part) in parts.iter().enumerate() {
        let mut part_content = String::new();

        // Add global header only in the first part
        if i == 0 && !header.is_empty() {
            part_content.push_str(header);
            part_content.push('\n');
        }

        // Add part header
        let part_header = replace_placeholders(&part_template.header, i + 1, total_parts);
        part_content.push_str(&part_header);
        part_content.push('\n');

        // Add file chunks
        for chunk in &part.file_chunks {
            part_content.push_str(chunk);
        }

        // Add part footer
        let part_footer = replace_placeholders(&part_template.footer, i + 1, total_parts);
        part_content.push_str(&part_footer);
        part_content.push('\n');

        // Add pending text if not the last part
        if i < total_parts - 1 && !part_template.pending.is_empty() {
            let pending_text = replace_placeholders(&part_template.pending, i + 1, total_parts);

            part_content.push_str(&pending_text);
            part_content.push('\n');
        }

        // Add global footer only in the last part
        if i == total_parts - 1 && !footer.is_empty() {
            part_content.push_str(footer);
            part_content.push('\n');
        }

        assembled_parts.push(part_content.to_string());
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
    use crate::template::template::PartTemplate;

    #[test]
    fn test_split_into_parts_single_part_fit_exactly() {
        let header = "Header".to_string();
        let footer = "Footer".to_string();
        let files = vec!["File1".to_string(), "File2".to_string()];

        let part_template = PartTemplate {
            header: "== Part <part-number> OF <total-parts> ==".to_string(),
            footer: "== Part END <part-number> OF <total-parts> ==".to_string(),
            pending: "This is only a part of the code (<parts-remaining> remaining)".to_string(),
        };

        let max_part_chars = 25; // Exact size of header, files, and footer

        let parts = split_into_parts(
            header.clone(),
            files.clone(),
            footer.clone(),
            part_template,
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

        let part_template = PartTemplate {
            header: "== Part <part-number> OF <total-parts> ==".to_string(),
            footer: "== Part END <part-number> OF <total-parts> ==".to_string(),
            pending: "This is only a part of the code (<parts-remaining> remaining)".to_string(),
        };

        let max_part_chars = 24;

        let parts = split_into_parts(
            header.clone(),
            files.clone(),
            footer.clone(),
            part_template,
            max_part_chars,
        );

        assert_eq!(parts.len(), 2);

        let expected = r#"Header
== Part 1 OF 2 ==
File1
== Part END 1 OF 2 ==
This is only a part of the code (1 remaining)
"#;

        assert_eq!(parts[0], expected);

        let expected = r#"== Part 2 OF 2 ==
File2
== Part END 2 OF 2 ==
Footer
"#;

        assert_eq!(parts[1], expected);
    }

    #[test]
    fn test_split_into_parts_accounting_for_long_header() {
        let header = "Header".repeat(10);
        let footer = "Footer".to_string();

        let files = vec![
            "Line1".repeat(10),
            "Line2".repeat(10),
            "Line3".repeat(10),
            "Line4".repeat(10),
            "Line5".repeat(10),
        ];

        let part_template = PartTemplate {
            header: "== Part <part-number> OF <total-parts> ==".to_string(),
            footer: "== Part END <part-number> OF <total-parts> ==".to_string(),
            pending: "This is only a part of the code (<parts-remaining> remaining)".to_string(),
        };

        let max_part_chars = 267;

        let parts = split_into_parts(
            header.clone(),
            files.clone(),
            footer.clone(),
            part_template,
            max_part_chars,
        );

        assert_eq!(parts.len(), 2);

        let expected = r#"HeaderHeaderHeaderHeaderHeaderHeaderHeaderHeaderHeaderHeader
== Part 1 OF 2 ==
Line1Line1Line1Line1Line1Line1Line1Line1Line1Line1
Line2Line2Line2Line2Line2Line2Line2Line2Line2Line2
== Part END 1 OF 2 ==
This is only a part of the code (1 remaining)
"#;

        assert_eq!(parts[0], expected);

        let expected = r#"== Part 2 OF 2 ==
Line3Line3Line3Line3Line3Line3Line3Line3Line3Line3
Line4Line4Line4Line4Line4Line4Line4Line4Line4Line4
Line5Line5Line5Line5Line5Line5Line5Line5Line5Line5
== Part END 2 OF 2 ==
Footer
"#;

        assert_eq!(parts[1], expected);
    }

    #[test]
    fn test_split_into_parts_split_long_between_parts() {
        let header = "Header".to_string();
        let footer = "Footer".to_string();

        let files = vec!["\
Line1Line1Line1Line1Line1Line1Line1Line1Line1Line1
Line2Line2Line2Line2Line2Line2Line2Line2Line2Line2
Line3Line3Line3Line3Line3Line3Line3Line3Line3Line3
Line4Line4Line4Line4Line4Line4Line4Line4Line4Line4
Line5Line5Line5Line5Line5Line5Line5Line5Line5Line5
Line6Line6Line6Line6Line6Line6Line6Line6Line6Line6
Line7Line7Line7Line7Line7Line7Line7Line7Line7Line7
Line8Line8Line8Line8Line8Line8Line8Line8Line8Line8
Line9Line9Line9Line9Line9Line9Line9Line9Line9Line9
Line0Line0Line0Line0Line0Line0Line0Line0Line0Line0"
            .to_string()];

        let part_template = PartTemplate {
            header: "== Part <part-number> OF <total-parts> ==".to_string(),
            footer: "== Part END <part-number> OF <total-parts> ==".to_string(),
            pending: "This is only a part of the code (<parts-remaining> remaining)".to_string(),
        };

        let max_part_chars = 314;

        let parts = split_into_parts(
            header.clone(),
            files.clone(),
            footer.clone(),
            part_template,
            max_part_chars,
        );

        assert_eq!(parts.len(), 3);

        let expected = r#"Header
== Part 1 OF 3 ==
Line1Line1Line1Line1Line1Line1Line1Line1Line1Line1
Line2Line2Line2Line2Line2Line2Line2Line2Line2Line2
Line3Line3Line3Line3Line3Line3Line3Line3Line3Line3
Line4Line4Line4Line4Line4Line4Line4Line4Line4Line4
== Part END 1 OF 3 ==
This is only a part of the code (2 remaining)
"#;

        assert_eq!(parts[0], expected);

        let expected = r#"== Part 2 OF 3 ==
Line5Line5Line5Line5Line5Line5Line5Line5Line5Line5
Line6Line6Line6Line6Line6Line6Line6Line6Line6Line6
Line7Line7Line7Line7Line7Line7Line7Line7Line7Line7
Line8Line8Line8Line8Line8Line8Line8Line8Line8Line8
== Part END 2 OF 3 ==
This is only a part of the code (1 remaining)
"#;

        assert_eq!(parts[1], expected);

        let expected = r#"== Part 3 OF 3 ==
Line9Line9Line9Line9Line9Line9Line9Line9Line9Line9
Line0Line0Line0Line0Line0Line0Line0Line0Line0Line0
== Part END 3 OF 3 ==
Footer
"#;

        assert_eq!(parts[2], expected);
    }

    #[test]
    fn test_split_into_parts_long_file_coming_after_small_files() {
        let header = "Header".to_string();
        let footer = "Footer".to_string();

        let files = vec![
            "Small1".to_string(),
            "\
Line1Line1Line1Line1Line1Line1Line1Line1Line1Line1
Line2Line2Line2Line2Line2Line2Line2Line2Line2Line2
Line3Line3Line3Line3Line3Line3Line3Line3Line3Line3
Line4Line4Line4Line4Line4Line4Line4Line4Line4Line4
Line5Line5Line5Line5Line5Line5Line5Line5Line5Line5
Line6Line6Line6Line6Line6Line6Line6Line6Line6Line6
Line7Line7Line7Line7Line7Line7Line7Line7Line7Line7
Line8Line8Line8Line8Line8Line8Line8Line8Line8Line8
Line9Line9Line9Line9Line9Line9Line9Line9Line9Line9
Line0Line0Line0Line0Line0Line0Line0Line0Line0Line0"
                .to_string(),
        ];

        let part_template = PartTemplate {
            header: "== Part <part-number> OF <total-parts> ==".to_string(),
            footer: "== Part END <part-number> OF <total-parts> ==".to_string(),
            pending: "This is only a part of the code (<parts-remaining> remaining)".to_string(),
        };

        let max_part_chars = 500;

        let parts = split_into_parts(
            header.clone(),
            files.clone(),
            footer.clone(),
            part_template,
            max_part_chars,
        );

        assert_eq!(parts.len(), 3);

        let expected = r#"Header
== Part 1 OF 3 ==
Small1
== Part END 1 OF 3 ==
This is only a part of the code (2 remaining)
"#;

        // The part contains only the small file,
        // and does NOT contain chunks from the next large file
        assert_eq!(parts[0], expected);

        let expected = r#"== Part 2 OF 3 ==
Line1Line1Line1Line1Line1Line1Line1Line1Line1Line1
Line2Line2Line2Line2Line2Line2Line2Line2Line2Line2
Line3Line3Line3Line3Line3Line3Line3Line3Line3Line3
Line4Line4Line4Line4Line4Line4Line4Line4Line4Line4
Line5Line5Line5Line5Line5Line5Line5Line5Line5Line5
Line6Line6Line6Line6Line6Line6Line6Line6Line6Line6
Line7Line7Line7Line7Line7Line7Line7Line7Line7Line7
== Part END 2 OF 3 ==
This is only a part of the code (1 remaining)
"#;

        assert_eq!(parts[1], expected);

        let expected = r#"== Part 3 OF 3 ==
Line8Line8Line8Line8Line8Line8Line8Line8Line8Line8
Line9Line9Line9Line9Line9Line9Line9Line9Line9Line9
Line0Line0Line0Line0Line0Line0Line0Line0Line0Line0
== Part END 3 OF 3 ==
Footer
"#;

        assert_eq!(parts[2], expected);
    }

    #[test]
    fn test_split_into_parts_long_file_coming_after_long_header_and_footer() {
        let header = "Header".repeat(10);
        let footer = "Footer".repeat(10);

        let files = vec!["\
Line1Line1Line1Line1Line1Line1Line1Line1Line1Line1
Line2Line2Line2Line2Line2Line2Line2Line2Line2Line2
Line3Line3Line3Line3Line3Line3Line3Line3Line3Line3
Line4Line4Line4Line4Line4Line4Line4Line4Line4Line4
Line5Line5Line5Line5Line5Line5Line5Line5Line5Line5
Line6Line6Line6Line6Line6Line6Line6Line6Line6Line6
Line7Line7Line7Line7Line7Line7Line7Line7Line7Line7
Line8Line8Line8Line8Line8Line8Line8Line8Line8Line8
Line9Line9Line9Line9Line9Line9Line9Line9Line9Line9
Line0Line0Line0Line0Line0Line0Line0Line0Line0Line0"
            .to_string()];

        let part_template = PartTemplate {
            header: "== Part <part-number> OF <total-parts> ==".to_string(),
            footer: "== Part END <part-number> OF <total-parts> ==".to_string(),
            pending: "This is only a part of the code (<parts-remaining> remaining)".to_string(),
        };

        let max_part_chars = 575;

        let parts = split_into_parts(
            header.clone(),
            files.clone(),
            footer.clone(),
            part_template,
            max_part_chars,
        );

        assert_eq!(parts.len(), 2);

        let expected = r#"HeaderHeaderHeaderHeaderHeaderHeaderHeaderHeaderHeaderHeader
== Part 1 OF 2 ==
Line1Line1Line1Line1Line1Line1Line1Line1Line1Line1
Line2Line2Line2Line2Line2Line2Line2Line2Line2Line2
Line3Line3Line3Line3Line3Line3Line3Line3Line3Line3
Line4Line4Line4Line4Line4Line4Line4Line4Line4Line4
Line5Line5Line5Line5Line5Line5Line5Line5Line5Line5
Line6Line6Line6Line6Line6Line6Line6Line6Line6Line6
Line7Line7Line7Line7Line7Line7Line7Line7Line7Line7
== Part END 1 OF 2 ==
This is only a part of the code (1 remaining)
"#;

        assert_eq!(parts[0], expected);

        let expected = r#"== Part 2 OF 2 ==
Line8Line8Line8Line8Line8Line8Line8Line8Line8Line8
Line9Line9Line9Line9Line9Line9Line9Line9Line9Line9
Line0Line0Line0Line0Line0Line0Line0Line0Line0Line0
== Part END 2 OF 2 ==
FooterFooterFooterFooterFooterFooterFooterFooterFooterFooter
"#;

        assert_eq!(parts[1], expected);
    }

    #[test]
    fn test_split_into_parts_no_files() {
        let header = "Header".to_string();
        let footer = "Footer".to_string();
        let files: Vec<String> = vec![];

        let part_template = PartTemplate {
            header: "=== PART <part-number> OF <total-parts> ===".to_string(),
            footer: "=== END OF PART <part-number> ===".to_string(),
            pending: "Please wait for the next part...".to_string(),
        };

        let max_part_chars = 50;

        let parts = split_into_parts(
            header.clone(),
            files.clone(),
            footer.clone(),
            part_template,
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
