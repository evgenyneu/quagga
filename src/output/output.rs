use super::file::output_to_file;
use super::stdout::output_to_stdout;
use crate::cli::Cli;
use std::error::Error;

pub fn process_output(content: Vec<String>, cli: &Cli) -> Result<(), Box<dyn Error>> {
    if let Some(output_path) = &cli.output {
        output_to_file(content, output_path.clone(), false, None)?;
    } else {
        output_to_stdout(content);
    }

    Ok(())
}
