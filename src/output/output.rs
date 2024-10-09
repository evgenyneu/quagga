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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::Cli;
    use crate::test_utils::temp_dir::TempDir;
    use clap::Parser;
    use std::fs;

    #[test]
    fn test_process_output() {
        let td = TempDir::new().unwrap();
        let output_path = td.path().join("test.txt");

        let mut cli = Cli::parse_from(&["quagga"]);
        cli.output = Some(output_path.clone());

        let content = vec!["Hello, world!".to_string()];
        let result = process_output(content.clone(), &cli);

        assert!(result.is_ok());
        assert!(output_path.exists());

        let file_content = fs::read_to_string(&output_path).unwrap();
        assert_eq!(file_content, content.join("\n"));
    }
}
