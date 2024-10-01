use crate::cli::Cli;
use crate::file_walker::get_all_files;
use crate::show_paths::format_file_paths;
use crate::tree::tree::file_paths_to_tree;
use std::error::Error;
use std::path::PathBuf;

pub fn non_template_output(
    cli: &Cli,
    paths: Option<Vec<PathBuf>>,
) -> Result<Option<String>, Box<dyn Error>> {
    if !cli.show_paths && !cli.tree {
        return Ok(None);
    }

    let files = get_paths(cli, paths)?;

    if cli.tree {
        return Ok(Some(file_paths_to_tree(files, cli.root.clone())));
    }

    if cli.show_paths {
        return Ok(Some(format_file_paths(files)));
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
