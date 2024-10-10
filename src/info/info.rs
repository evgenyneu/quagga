use crate::cli::Cli;
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
    if !cli.paths && !cli.tree && !cli.copy_template && !cli.size {
        return Ok(None);
    }

    if cli.copy_template {
        let output = copy_template(&cli.root.clone())?;
        return Ok(Some(output));
    }

    let files = get_paths(cli, paths)?;

    if cli.tree {
        return Ok(Some(file_paths_to_tree(files, Some(cli.root.clone()))));
    }

    if cli.paths {
        return Ok(Some(format_file_paths(files)));
    }

    if cli.size {
        return Ok(Some(get_total_size(files)?));
    }

    // Return error if we reach this point
    Err("No output generated".into())
}

fn get_paths(cli: &Cli, paths: Option<Vec<PathBuf>>) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let files = if let Some(paths) = paths {
        paths
    } else {
        get_all_files(cli)?
    };

    Ok(files)
}
