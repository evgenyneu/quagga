use crate::cli::Cli;
use crate::file::file_reader::FileContent;
use crate::template::parser::template::Template;
use crate::template::tags::header_footer::process_header_footer;
use std::path::PathBuf;

/// Concatenates the contents of multiple files using the provided template parts.
///
/// # Arguments
///
/// * `template` - A `Template` struct containing template structure.
/// * `files` - A vector of `FileContent` structs.
///
/// # Returns
///
/// A `String` containing the concatenated contents with header and footer.
pub fn concatenate_files(template: Template, files: Vec<FileContent>, cli: &Cli) -> String {
    let items: String = concatenate_items(&template.prompt.file, &files);
    let mut contents = String::new();
    let file_paths: Vec<PathBuf> = files.iter().map(|f| f.path.clone()).collect();

    if !template.prompt.header.is_empty() {
        let header = process_header_footer(&template.prompt.header, &file_paths, &cli.root);
        contents.push_str(&header);
        contents.push('\n');
    }

    contents.push_str(&items);

    if !template.prompt.footer.is_empty() {
        let footer = process_header_footer(&template.prompt.footer, &file_paths, &cli.root);
        contents.push_str(&footer);
    }

    contents
}

/// Concatenates the items by applying the item template to each `FileContent`.
///
/// # Arguments
///
/// * `item_template` - A `String` representing the item template.
/// * `files` - A vector of `FileContent` structs.
///
/// # Returns
///
/// A `String` containing all items concatenated after applying the item template.
pub fn concatenate_items(item_template: &str, files: &Vec<FileContent>) -> String {
    let mut contents = String::new();

    for file in files {
        let item = item_template
            .replace("<file-path>", &file.path.display().to_string())
            .replace("<file-content>", &file.content);

        contents.push_str(&item);
        contents.push('\n'); // Separate items with a newline
    }

    contents
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::template::parser::template::{PromptSection, Template};
    use clap::Parser;
    use std::path::PathBuf;

    #[test]
    fn test_concatenate_files() {
        let template = Template {
            prompt: PromptSection {
                header: "Header".to_string(),
                file: "File: <file-path>\nContent:\n<file-content>\n---".to_string(),
                footer: "Footer".to_string(),
            },
            part: Default::default(),
        };

        let file1 = FileContent {
            path: PathBuf::from("file1.txt"),
            content: "Hello".to_string(),
        };

        let file2 = FileContent {
            path: PathBuf::from("file2.txt"),
            content: "World!".to_string(),
        };

        let files = vec![file1, file2];
        let cli = Cli::parse_from(&["test"]);

        let result = concatenate_files(template, files, &cli);

        let expected_output = "\
Header
File: file1.txt
Content:
Hello
---
File: file2.txt
Content:
World!
---
Footer";

        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_concatenate_files_with_all_file_paths_tag() {
        let template = Template {
            prompt: PromptSection {
                header: "Header with paths: <all-file-paths>".to_string(),
                file: "File: <file-content>".to_string(),
                footer: "Footer with paths: <all-file-paths>".to_string(),
            },
            part: Default::default(),
        };

        let files = vec![
            FileContent {
                path: PathBuf::from("file1.txt"),
                content: "Content1".to_string(),
            },
            FileContent {
                path: PathBuf::from("file2.txt"),
                content: "Content2".to_string(),
            },
        ];

        let cli = Cli::parse_from(&["test"]);

        let result = concatenate_files(template, files, &cli);
        let expected = r#"Header with paths: file1.txt
file2.txt
File: Content1
File: Content2
Footer with paths: file1.txt
file2.txt"#;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_concatenate_items() {
        let item_template = "File: <file-path>\nContent:\n<file-content>\n---";

        let file1 = FileContent {
            path: PathBuf::from("file1.txt"),
            content: "Hello".to_string(),
        };

        let file2 = FileContent {
            path: PathBuf::from("file2.txt"),
            content: "World!".to_string(),
        };

        let files = vec![file1, file2];

        let result = concatenate_items(item_template, &files);

        let expected_output = "\
File: file1.txt
Content:
Hello
---
File: file2.txt
Content:
World!
---
";

        assert_eq!(result, expected_output);
    }
}
