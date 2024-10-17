use crate::cli::Cli;
use crate::info::file_sizes::get_formatted_file_sizes;
use crate::info::show_paths::format_file_paths;
use crate::info::size::get_total_size;
use crate::info::tree::file_paths_to_tree;
use crate::template::copy::copy_template;
use crate::walk::file_walker::get_all_files;
use std::error::Error;
use std::path::PathBuf;

/// Generates info output for options like `--paths` or `--tree` that do
/// not involve concatenating the files.
///
/// # Arguments
///
/// * `cli` - Command line arguments.
/// * `paths` - An optional `Vec<PathBuf>` representing a list of file paths.
///             When present, the program will simply concatenate the paths and return them,
///             without traversing the root directory.
///
/// # Returns
///
/// `Ok(String)` containing the output text, or an error if something goes wrong.
pub fn info_output(
    cli: &Cli,
    paths: Option<Vec<PathBuf>>,
) -> Result<Option<String>, Box<dyn Error>> {
    if !cli.paths && !cli.tree && !cli.copy_template && !cli.size && !cli.file_sizes {
        return Ok(None);
    }

    if cli.copy_template {
        let output = copy_template(&cli.root.clone())?;
        return Ok(Some(output));
    }

    let files = get_paths(cli, paths)?;

    let mut output = Vec::new();

    if cli.tree {
        output.push(file_paths_to_tree(files.clone(), Some(cli.root.clone())));
    }

    if cli.paths {
        output.push(format_file_paths(files.clone()));
    }

    if cli.file_sizes {
        output.push(get_formatted_file_sizes(files.clone())?);
    }

    if cli.size {
        output.push(get_total_size(files.clone())?);
    }

    Ok(Some(output.join("\n\n")))
}

fn get_paths(cli: &Cli, paths: Option<Vec<PathBuf>>) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let files = if let Some(paths) = paths {
        paths
    } else {
        get_all_files(cli)?
    };

    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::temp_dir::TempDir;
    use clap::Parser;

    #[test]
    fn test_info_output_all_options() {
        let td = TempDir::new().unwrap();
        let file1 = td.mkfile_with_contents("file1.txt", "Hello");
        let file2 = td.mkfile_with_contents("file2.txt", "World");

        let mut cli = Cli::parse_from(&["test", "--paths", "--tree", "--size"]);
        cli.root = td.path_buf();

        let result = info_output(&cli, None).unwrap().unwrap();
        let parts: Vec<&str> = result.split("\n\n").collect();

        assert_eq!(parts.len(), 3);

        // Check tree output
        // ----------

        let expected = format!(
            "{}
├── file1.txt
└── file2.txt",
            cli.root.display()
        );

        assert_eq!(parts[0], expected);

        // Check paths output
        // ----------

        assert_eq!(
            parts[1],
            format!(
                "{}
{}",
                file1.display(),
                file2.display()
            )
        );

        // Check size output
        // ----------

        assert_eq!(parts[2], "10 B");
    }

    #[test]
    fn test_info_output_partial_options() {
        let td = TempDir::new().unwrap();
        td.mkfile_with_contents("file1.txt", "Hello");
        td.mkfile_with_contents("file2.txt", "World");

        let mut cli = Cli::parse_from(&["test", "--size"]);
        cli.root = td.path_buf();

        let result = info_output(&cli, None).unwrap().unwrap();
        let parts: Vec<&str> = result.split("\n\n").collect();

        assert_eq!(parts.len(), 1);

        // Check size output
        assert!(parts[0].contains("10 B"));
    }

    #[test]
    fn test_info_output_no_options() {
        let td = TempDir::new().unwrap();
        let mut cli = Cli::parse_from(&["test"]);
        cli.root = td.path_buf();

        let result = info_output(&cli, None).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_info_output_copy_template() {
        let td = TempDir::new().unwrap();
        let mut cli = Cli::parse_from(&["test", "--copy-template"]);
        cli.root = td.path_buf();

        let result = info_output(&cli, None).unwrap().unwrap();
        assert!(result.contains("Template was copied to"));
    }
}
