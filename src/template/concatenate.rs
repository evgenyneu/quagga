use crate::file_reader::FileContent;
use crate::template::tags::all_file_paths::replace_all_file_paths_tag;
use crate::template::template::TemplateParts;
use std::path::PathBuf;

/// Concatenates the contents of multiple files using the provided template parts.
///
/// # Arguments
///
/// * `template` - A `TemplateParts` struct containing the header, item, and footer.
/// * `files` - A vector of `FileContent` structs.
///
/// # Returns
///
/// A `String` containing the concatenated contents with header and footer.
pub fn concatenate_files(template: TemplateParts, files: Vec<FileContent>) -> String {
    let items = concatenate_items(&template.item, &files);
    let mut contents = String::new();
    let file_paths: Vec<PathBuf> = files.iter().map(|f| f.path.clone()).collect();

    if !template.header.is_empty() {
        let header = replace_all_file_paths_tag(&template.header, file_paths.clone());
        contents.push_str(&header);
        contents.push('\n');
    }

    contents.push_str(&items);

    if !template.footer.is_empty() {
        let footer = replace_all_file_paths_tag(&template.footer, file_paths.clone());
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
            .replace("{{FILE_PATH}}", &file.path.display().to_string())
            .replace("{{CONTENT}}", &file.content);

        contents.push_str(&item);
        contents.push('\n'); // Separate items with a newline
    }

    contents
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::template::template::TemplateParts;
    use std::path::PathBuf;

    #[test]
    fn test_concatenate_files() {
        let template = TemplateParts {
            header: "Header".to_string(),
            item: "File: {{FILE_PATH}}\nContent:\n{{CONTENT}}\n---".to_string(),
            footer: "Footer".to_string(),
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

        let result = concatenate_files(template, files);

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
    fn test_concatenate_items() {
        let item_template = "File: {{FILE_PATH}}\nContent:\n{{CONTENT}}\n---";

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
