use home::home_dir;
use ignore::WalkBuilder;
use std::path::PathBuf;

/// Adds .quagga_ignore files from the project root and home directories to the WalkBuilder.
///
/// # Arguments
///
/// * `builder` - The WalkBuilder to which ignore files will be added.
/// * `project_root` - PathBuf of the project root directory.
///
/// # Returns
///
/// * Result<(), Box<dyn std::error::Error>> - Ok if the files were successfully added, Err otherwise.
pub fn add_quagga_ignore_files(
    builder: &mut WalkBuilder,
    project_root: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    add_home_ignore_file(builder)?;
    add_project_ignore_file(builder, project_root)?;
    Ok(())
}

fn add_home_ignore_file(builder: &mut WalkBuilder) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(home_dir) = home_dir() {
        let home_ignore = home_dir.join(".quagga_ignore");

        if home_ignore.exists() {
            builder.add_ignore(home_ignore);
        }
    }

    Ok(())
}

fn add_project_ignore_file(
    builder: &mut WalkBuilder,
    project_root: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let project_ignore = project_root.join(".quagga_ignore");

    if project_ignore.exists() {
        builder.add_ignore(project_ignore);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::temp_dir::TempDir;
    use std::env;

    #[test]
    fn test_use_quagga_ignore_files_from_project_dir() {
        let td = TempDir::new().unwrap();
        td.mkfile("file.md");
        td.mkfile("file.txt");
        td.mkdir("subdir");
        td.mkfile("subdir/file.txt");
        td.mkfile("subdir/file.md");

        td.mkfile_with_contents(".quagga_ignore", "*.md");

        // Test in the context of the temporary project root
        let mut builder = WalkBuilder::new(td.path());
        add_quagga_ignore_files(&mut builder, td.path_buf()).unwrap();

        let walker = builder.build();

        let paths: Vec<PathBuf> = walker
            .filter_map(|entry| entry.ok().map(|e| e.path().to_path_buf()))
            .collect();

        assert_eq!(paths.len(), 4); // two directories and two *.txt file

        // Present files
        td.assert_contains(&paths, "file.txt");
        td.assert_contains(&paths, "subdir/file.txt");

        // Ignored files
        td.assert_not_contains(&paths, "file.md");
        td.assert_not_contains(&paths, "subdir/file.md");
    }

    #[test]
    fn test_use_quagga_ignore_files_from_home_dir() {
        // Project directory
        let td = TempDir::new().unwrap();
        td.mkfile("file.md");
        td.mkfile("file.txt");

        // Home directory
        let home_td = TempDir::new().unwrap();
        home_td.mkfile_with_contents(".quagga_ignore", "*.md");
        env::set_var("HOME", home_td.path());

        let mut builder = WalkBuilder::new(td.path());

        add_quagga_ignore_files(&mut builder, td.path_buf()).unwrap();

        let walker = builder.build();

        let paths: Vec<PathBuf> = walker
            .filter_map(|entry| entry.ok().map(|e| e.path().to_path_buf()))
            .collect();

        assert_eq!(paths.len(), 2); // Only one *.txt file and the project directory itself

        // Present file
        td.assert_contains(&paths, "file.txt");

        // Ignored file
        td.assert_not_contains(&paths, "file.md");

        // Clean up: unset the HOME environment variable
        env::remove_var("HOME");
    }

    #[test]
    fn test_project_ignore_takes_precedence_over_home_ignore() {
        // Project directory
        let td = TempDir::new().unwrap();
        td.mkfile_with_contents("README.md", "markdown content");
        td.mkfile_with_contents("file.md", "other markdown content");
        td.mkfile_with_contents("file.txt", "text content");

        // Home directory
        let home_td = TempDir::new().unwrap(); // Temporary home directory
        env::set_var("HOME", home_td.path());

        // Home: ignore all .md files
        home_td.mkfile_with_contents(".quagga_ignore", "*.md");

        // Project: whitelist README.md
        td.mkfile_with_contents(".quagga_ignore", "!README.md");

        let mut builder = WalkBuilder::new(td.path());
        add_quagga_ignore_files(&mut builder, td.path_buf()).unwrap();

        let walker = builder.build();

        let paths: Vec<PathBuf> = walker
            .filter_map(|entry| entry.ok().map(|e| e.path().to_path_buf()))
            .collect();

        assert_eq!(paths.len(), 3); // Two files plus the project directory

        // Ensure README.md is white-listed (from project .quagga_ignore)
        td.assert_contains(&paths, "README.md");

        // Ensure other .md files are still ignored (from home .quagga_ignore)
        td.assert_not_contains(&paths, "file.md");

        // Ensure .txt files are included as expected
        td.assert_contains(&paths, "file.txt");

        // Clean up: unset the HOME environment variable
        env::remove_var("HOME");
    }
}
