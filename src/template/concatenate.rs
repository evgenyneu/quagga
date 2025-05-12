use super::split::split_into_parts;
use crate::cli::Cli;
use crate::file::file_content::FileContent;
use crate::template::tags::header_footer::process_header_footer;
use crate::template::template::Template;
use std::path::PathBuf;

/// Concatenates the contents of multiple files using the provided template.
///
/// # Arguments
///
/// * `template` - A `Template` struct containing template structure.
/// * `files` - A vector of `FileContent` structs.
///
/// # Returns
///
/// A `String` vector containing the output prompt content splitted into parts
pub fn concatenate_files(template: Template, files: Vec<FileContent>, cli: &Cli) -> Vec<String> {
    let file_paths: Vec<PathBuf> = files.iter().map(|f| f.path.clone()).collect();
    let header = process_header_footer(&template.prompt.header, &file_paths, &cli.root);
    let files = apply_file_template(&template.prompt.file, &files);
    let footer = process_header_footer(&template.prompt.footer, &file_paths, &cli.root);

    split_into_parts(
        header,
        files,
        footer,
        template.part,
        cli.max_part_size as usize,
    )
}

/// Applied the file template to each file by replacing the content and file path tags.
///
/// # Arguments
///
/// * `item_template` - A `String` representing the item template.
/// * `files` - A vector of `FileContent` structs.
///
/// # Returns
///
/// A `Vec<String>` containing the content of each file with the template applied.
pub fn apply_file_template(item_template: &str, files: &Vec<FileContent>) -> Vec<String> {
    files
        .iter()
        .map(|file| {
            item_template
                .replace("<file-path>", &file.path.display().to_string())
                .replace("<file-content>", &file.content)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::template::template::{PromptTemplate, Template};
    use clap::Parser;
    use std::path::PathBuf;

    #[test]
    fn test_concatenate_files() {
        let template = Template {
            prompt: PromptTemplate {
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

        assert_eq!(result.len(), 1);

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

        assert_eq!(result[0], expected_output);
    }

    #[test]
    fn test_concatenate_files_with_all_file_paths_tag() {
        let template = Template {
            prompt: PromptTemplate {
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

        assert_eq!(result.len(), 1);

        let expected = r#"Header with paths: file1.txt
file2.txt
File: Content1
File: Content2
Footer with paths: file1.txt
file2.txt"#;

        assert_eq!(result[0], expected);
    }

    #[test]
    fn test_apply_file_template() {
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

        let result = apply_file_template(item_template, &files);

        assert_eq!(result.len(), 2);

        let expected = "\
File: file1.txt
Content:
Hello
---";

        assert_eq!(result[0], expected);

        let expected = "\
File: file2.txt
Content:
World!
---";

        assert_eq!(result[1], expected);
    }
}
