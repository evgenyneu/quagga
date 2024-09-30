use crate::cli::Cli;
use crate::file_walker::get_all_files;
use crate::tree::tree::file_paths_to_tree as paths_to_tree;
use std::error::Error;
use std::path::PathBuf;

/// Builds an ASCII tree representation from a list of file paths for the --tree command line option:
///
///
/// ├── subdir
/// │   └── file2.txt
/// └── file1.txt
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
/// `Ok(String)` containing the concatenated file paths, or an error if something goes wrong.
pub fn file_paths_to_tree(
    cli: &Cli,
    paths: Option<Vec<PathBuf>>,
) -> Result<String, Box<dyn Error>> {
    let files = if let Some(paths) = paths {
        paths
    } else {
        get_all_files(cli)?
    };

    Ok(paths_to_tree(files, cli.root.clone()))
}
