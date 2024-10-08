use crate::cli::Cli;
use crate::file::file_reader::read_and_concatenate_files;
use crate::info::info::info_output;
use crate::template::read::{path_to_custom_template, read_and_parse_template};
use crate::template::template::Template;
use crate::walk::file_walker::get_all_files;
use std::error::Error;
use std::path::PathBuf;

/// The function called by `main.rs` that processes files based on provided command line options.
///
/// # Arguments
///
/// * `cli` - Command line arguments.
/// * `piped_paths` - An optional `Vec<PathBuf>` representing a list of file paths the user
///                  has piped in via stdin. When present, the program will process the
///                  files in the list instead of walking the root directory.
///
/// # Returns
///
/// A `Result` containing the output prompt content, splitted into parts, if successful,
/// or an error if any operation fails.
pub fn run(cli: &Cli, piped_paths: Option<Vec<PathBuf>>) -> Result<Vec<String>, Box<dyn Error>> {
    let output = info_output(cli, piped_paths.clone())?;

    if let Some(output) = output {
        return Ok(Vec::from([output]));
    }

    let template_path = path_to_custom_template(cli);
    let template = read_and_parse_template(template_path)?;

    if let Some(path_list) = piped_paths {
        return read_and_concatenate_files(path_list, template, cli)
            .map_err(|e| Box::new(e) as Box<dyn Error>);
    } else {
        return process_files(cli, template);
    }
}

/// Processes files starting from the given root path:
/// - Retrieves file paths by walking the root directory.
/// - Reads and concatenates their contents.
///
/// # Arguments
///
/// * `cli` - Command line arguments.
///
/// # Returns
///
/// A `Result` containing the output prompt, splitted into parts, if successful,
/// or an `io::Error` if an error occurs.
///
/// # Errors
///
/// This function will return an error if:
/// - Retrieving the list of files fails.
/// - Reading any of the files fails.
pub fn process_files(cli: &Cli, template: Template) -> Result<Vec<String>, Box<dyn Error>> {
    let mut files = get_all_files(cli)?;
    files.sort();

    read_and_concatenate_files(files, template, cli).map_err(|e| Box::new(e) as Box<dyn Error>)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::template::template::PromptTemplate;
    use crate::test_utils::temp_dir::TempDir;
    use clap::Parser;

    #[test]
    fn test_show_paths() {
        let td = TempDir::new().unwrap();
        let path1 = td.mkfile("file1.txt");
        let path2 = td.mkfile("file2.txt");

        let mut cli = Cli::parse_from(&["test", "--show-paths"]);
        cli.root = td.path_buf();

        let result = run(&cli, None);

        assert!(result.is_ok());
        let content = result.unwrap();
        assert_eq!(content.len(), 1);
        let expected = format!("{}\n{}", path1.display(), path2.display());
        assert_eq!(content[0], expected);
    }

    #[test]
    fn test_show_tree() {
        let td = TempDir::new().unwrap();
        td.mkfile("file1.txt");
        td.mkfile("file2.txt");
        td.mkdir("subdir");
        td.mkfile("subdir/file3.txt");

        let mut cli = Cli::parse_from(&["test", "--tree"]);
        cli.root = td.path_buf();

        let result = run(&cli, None);

        assert!(result.is_ok());
        let content = result.unwrap();
        assert_eq!(content.len(), 1);

        let expected = format!(
            r#"{}
├── subdir
│   └── file3.txt
├── file1.txt
└── file2.txt"#,
            td.path().display()
        );

        assert_eq!(content[0], expected);
    }

    #[test]
    fn test_process_files_success() {
        let td = TempDir::new().unwrap();
        let file1_path = td.mkfile_with_contents("file1.txt", "Hello");
        let file2_path = td.mkfile_with_contents("file3.txt", "World!");

        let mut cli = Cli::parse_from(&["test"]);
        cli.root = td.path_buf();

        let template = Template {
            prompt: PromptTemplate {
                header: "Header".to_string(),
                file: "File: <file-path>\nContent:\n<file-content>\n---".to_string(),
                footer: "Footer".to_string(),
            },
            part: Default::default(),
        };

        let result = process_files(&cli, template);

        assert!(result.is_ok());
        let content = result.unwrap();
        assert_eq!(content.len(), 1);

        let expected = format!(
            "\
Header
File: {}
Content:
Hello
---
File: {}
Content:
World!
---
Footer",
            file1_path.display(),
            file2_path.display()
        );

        assert_eq!(content[0], expected);
    }

    #[test]
    fn test_process_files_with_nonexistent_directory() {
        let mut cli = Cli::parse_from(&["test"]);
        cli.root = PathBuf::from("/path/to/nonexistent/directory");

        let result = process_files(&cli, Template::default());

        assert!(result.is_err());
    }
}
