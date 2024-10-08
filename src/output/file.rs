use chrono::{DateTime, Local, Utc};
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;

/// Writes the provided content to the specified output path. If the content has multiple parts,
/// they are written to separate files that are suffixed with `.XXX` (e.g. file.txt.001, file.txt.002).
///
/// # Arguments
///
/// * `content` - An output prompt text, splitted into parts.
/// * `path` - The base output path. Can contain `{TIME}` and `{TIME_UTC}` tags that will be replaced with current timestamp in the format `YYYY-mm-DD_HH-MM-SS`.
/// * `combine_parts` - When true, forces all parts to be combined in a single file.
/// * `fixed_time` - An optional timestamp to use instead of current time.
///                  Used to replace the {TIME} and {TIME_UTC} tags in path with current time.
///
/// # Returns
///
/// * `Ok(())` if the operation succeeds.
/// * `Err(io::Error)` if any file operation fails.
pub fn output_to_file(
    content: Vec<String>,
    path: PathBuf,
    combine_parts: bool,
    fixed_time: Option<DateTime<Local>>,
) -> Result<(), io::Error> {
    let path = replace_time_tags(&path, fixed_time)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

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
    file.write_all(combined_content.trim().as_bytes())?;
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
        file.write_all(part.trim().as_bytes())?;
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

/// Replaces `{TIME}` and `{TIME_UTC}` in the provided path with the current timestamp.
///
/// `{TIME}` is replaced with the local time in the format `YYYY-mm-DD_HH-MM-SS`.
/// `{TIME_UTC}` is replaced with the UTC time in the same format.
///
/// # Arguments
///
/// * `path` - The original path potentially containing `{TIME}` or `{TIME_UTC}`.
/// * `fixed_time` - An optional timestamp to use instead of current time.
///
/// # Returns
///
/// * `Ok(PathBuf)` with tags replaced.
/// * `Err(Box<dyn Error>)` if any error occurs.
pub fn replace_time_tags(
    path: &PathBuf,
    fixed_time: Option<DateTime<Local>>,
) -> Result<PathBuf, Box<dyn Error + Send + Sync>> {
    let path_str = path.to_string_lossy();
    let time_format = "%Y-%m-%d_%H-%M-%S";

    let replaced = path_str
        .replace(
            "{TIME}",
            &get_current_timestamp_local(time_format, fixed_time),
        )
        .replace(
            "{TIME_UTC}",
            &get_current_timestamp_utc(time_format, fixed_time),
        );

    Ok(PathBuf::from(replaced))
}

/// Gets the current local timestamp in the specified format.
///
/// # Arguments
///
/// * `time_format` - The format string for the timestamp.
/// * `fixed_time` - An optional timestamp to use instead of current time.
///
/// # Returns
///
/// A formatted timestamp string.
fn get_current_timestamp_local(time_format: &str, fixed_time: Option<DateTime<Local>>) -> String {
    match fixed_time {
        Some(t) => t.format(time_format).to_string(),
        None => Local::now().format(time_format).to_string(),
    }
}

/// Gets the current UTC timestamp in the specified format.
///
/// # Arguments
///
/// * `time_format` - The format string for the timestamp.
/// * `fixed_time` - An optional timestamp to use instead of current time.
///
/// # Returns
///
/// A formatted UTC timestamp string.
fn get_current_timestamp_utc(time_format: &str, fixed_time: Option<DateTime<Local>>) -> String {
    match fixed_time {
        Some(t) => t.with_timezone(&Utc).format(time_format).to_string(),
        None => Utc::now().format(time_format).to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::temp_dir::TempDir;
    use chrono::{DateTime, LocalResult, TimeZone, Utc};
    use regex::Regex;
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

        let result = output_to_file(content.clone(), base_path.clone(), false, None);
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

        let result = output_to_file(content.clone(), base_path.clone(), false, None);
        assert!(result.is_ok());

        assert!(base_path.exists());
        let part_content = fs::read_to_string(&base_path).unwrap();
        assert_eq!(part_content, "Content");
    }

    #[test]
    fn test_output_to_file_replace_time_in_path() {
        let td = TempDir::new().unwrap();
        let base_path = td.path().join("dir/{TIME_UTC}_output.txt");

        let content = vec!["Content".to_string()];
        let fixed_time = create_fixed_datetime(1_700_000_000);

        let result = output_to_file(content.clone(), base_path.clone(), false, Some(fixed_time));
        assert!(result.is_ok());

        let path_with_time = PathBuf::from(format!(
            "{}/dir/2023-11-14_22-13-20_output.txt",
            td.path().display()
        ));

        assert!(path_with_time.exists());
        let part_content = fs::read_to_string(&path_with_time).unwrap();
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

        let result = output_to_file(content.clone(), base_path.clone(), true, None);
        assert!(result.is_ok());

        assert!(base_path.exists());
        let part_content = fs::read_to_string(&base_path).unwrap();
        assert_eq!(part_content, "Part 1\nPart 2\nPart 3");
    }

    #[test]
    fn test_output_to_file_return_success_when_no_content() {
        let td = TempDir::new().unwrap();
        let base_path = td.path().join("dir/output.txt");

        let result = output_to_file(vec![], base_path.clone(), false, None);
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

    /// Helper function to create a fixed DateTime<Local> from a timestamp.
    fn create_fixed_datetime(timestamp: i64) -> DateTime<Local> {
        let fixed_utc: DateTime<Utc> = match Utc.timestamp_opt(timestamp, 0) {
            LocalResult::Single(dt) => dt,
            LocalResult::None => panic!("Invalid timestamp provided"),
            LocalResult::Ambiguous(_, _) => panic!("Ambiguous timestamp provided"),
        };

        // Convert to DateTime<Local>
        fixed_utc.with_timezone(&Local)
    }

    #[test]
    fn test_replace_time_tags_local() {
        let path = PathBuf::from("dir/{TIME}_output.txt");

        let result = replace_time_tags(&path, None).unwrap();
        let path_str = result.to_string_lossy();

        // since we don't know local time, just check the format
        let re = Regex::new(r"^dir/\d{4}-\d{2}-\d{2}_\d{2}-\d{2}-\d{2}_output\.txt$").unwrap();
        assert!(re.is_match(&path_str));
    }

    #[test]
    fn test_replace_time_tags_utc() {
        let path = PathBuf::from("dir/{TIME_UTC}_output.txt");
        let fixed_time = create_fixed_datetime(1_700_000_000);

        let result = replace_time_tags(&path, Some(fixed_time)).unwrap();

        let expected = PathBuf::from("dir/2023-11-14_22-13-20_output.txt");

        assert_eq!(result, expected);
    }
}
