use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;

/// Writes the provided content to the specified output path. If the content has multiple parts,
/// they are written to separate files that are suffixed with `.XXX` (.001, .002, etc.).
///
/// # Arguments
///
/// * `content` - An output prompt text, splitted into parts.
/// * `path` - The base output path.
/// * `combine_parts` - When true, forces all parts to be combined in a single file.
///
/// # Returns
///
/// * `Ok(())` if the operation succeeds.
/// * `Err(io::Error)` if any file operation fails.
pub fn output_to_file(
    content: Vec<String>,
    path: PathBuf,
    combine_parts: bool,
) -> Result<(), io::Error> {
    if content.is_empty() {
        return Ok(());
    }

    if content.len() == 1 || combine_parts {
        combine_and_write(content, &path)
    } else {
        write_parts_separately(content, &path)
    }
}

/// Combines all content parts with newlines and writes to a single file.
///
/// # Arguments
///
/// * `content` - A vector of strings to combine.
/// * `path` - The output file path.
///
/// # Returns
///
/// * `Ok(())` if writing succeeds.
/// * `Err(io::Error)` if writing fails.
fn combine_and_write(content: Vec<String>, path: &PathBuf) -> Result<(), io::Error> {
    let combined_content = content.join("\n");
    create_parent_dir(path)?;
    let mut file = File::create(path)?;
    file.write_all(combined_content.as_bytes())?;
    Ok(())
}

/// Writes each part to a separate file with a `.XXX` suffix.
///
/// # Arguments
///
/// * `content` - A vector of strings, each representing a part.
/// * `base_path` - The base output file path.
///
/// # Returns
///
/// * `Ok(())` if all files are written successfully.
/// * `Err(io::Error)` if any file operation fails.
fn write_parts_separately(content: Vec<String>, base_path: &PathBuf) -> Result<(), io::Error> {
    create_parent_dir(base_path)?;

    for (index, part) in content.iter().enumerate() {
        let suffix = format!("{:03}", index + 1);
        let suffixed_path = format!("{}.{}", base_path.display(), suffix);
        let mut file = File::create(&suffixed_path)?;
        file.write_all(part.as_bytes())?;
    }

    Ok(())
}

/// Creates the parent directories of the specified path.
fn create_parent_dir(path: &PathBuf) -> Result<(), io::Error> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::temp_dir::TempDir;
    use std::fs;

    #[test]
    fn test_output_to_file_multiple_parts() {
        let td = TempDir::new().unwrap();
        let base_path = td.path().join("dir/output.txt");

        let content = vec![
            "Part 1".to_string(),
            "Part 2".to_string(),
            "Part 3".to_string(),
        ];

        let result = output_to_file(content.clone(), base_path.clone(), false);
        assert!(result.is_ok());

        let expected_path = PathBuf::from(format!("{}.001", base_path.display()));
        assert!(expected_path.exists());
        let part_content = fs::read_to_string(&expected_path).unwrap();
        assert_eq!(part_content, "Part 1");

        let expected_path = PathBuf::from(format!("{}.002", base_path.display()));
        assert!(expected_path.exists());
        let part_content = fs::read_to_string(&expected_path).unwrap();
        assert_eq!(part_content, "Part 2");

        let expected_path = PathBuf::from(format!("{}.003", base_path.display()));
        assert!(expected_path.exists());
        let part_content = fs::read_to_string(&expected_path).unwrap();
        assert_eq!(part_content, "Part 3");
    }

    #[test]
    fn test_output_to_file_one_part() {
        let td = TempDir::new().unwrap();
        let base_path = td.path().join("dir/output.txt");

        let content = vec!["Content".to_string()];

        let result = output_to_file(content.clone(), base_path.clone(), false);
        assert!(result.is_ok());

        assert!(base_path.exists());
        let part_content = fs::read_to_string(&base_path).unwrap();
        assert_eq!(part_content, "Content");
    }

    #[test]
    fn test_output_to_file_multiple_parts_force_into_one() {
        let td = TempDir::new().unwrap();
        let base_path = td.path().join("dir/output.txt");

        let content = vec![
            "Part 1".to_string(),
            "Part 2".to_string(),
            "Part 3".to_string(),
        ];

        let result = output_to_file(content.clone(), base_path.clone(), true);
        assert!(result.is_ok());

        assert!(base_path.exists());
        let part_content = fs::read_to_string(&base_path).unwrap();
        assert_eq!(part_content, "Part 1\nPart 2\nPart 3");
    }

    #[test]
    fn test_output_to_file_return_success_when_no_content() {
        let td = TempDir::new().unwrap();
        let base_path = td.path().join("dir/output.txt");

        let result = output_to_file(vec![], base_path.clone(), false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_combine_and_write_single_file() {
        let td = TempDir::new().unwrap();
        let output_path = td.path().join("subdir/combined_output.txt");

        let content = vec![
            "First part".to_string(),
            "Second part".to_string(),
            "Third part".to_string(),
        ];

        let result = combine_and_write(content.clone(), &output_path);
        assert!(result.is_ok());

        let written_content = fs::read_to_string(&output_path).unwrap();
        assert_eq!(written_content, "First part\nSecond part\nThird part");
    }

    #[test]
    fn test_write_parts_separately_multiple_files() {
        let td = TempDir::new().unwrap();
        let base_path = td.path().join("dir/output.txt");

        let content = vec![
            "Part 1".to_string(),
            "Part 2".to_string(),
            "Part 3".to_string(),
        ];

        let result = write_parts_separately(content.clone(), &base_path);
        assert!(result.is_ok());

        let expected_path = PathBuf::from(format!("{}.001", base_path.display()));
        assert!(expected_path.exists());
        let part_content = fs::read_to_string(&expected_path).unwrap();
        assert_eq!(part_content, "Part 1");

        let expected_path = PathBuf::from(format!("{}.002", base_path.display()));
        assert!(expected_path.exists());
        let part_content = fs::read_to_string(&expected_path).unwrap();
        assert_eq!(part_content, "Part 2");

        let expected_path = PathBuf::from(format!("{}.003", base_path.display()));
        assert!(expected_path.exists());
        let part_content = fs::read_to_string(&expected_path).unwrap();
        assert_eq!(part_content, "Part 3");
    }
}
