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
    // Add .quagga_ignore from the project root
    let project_ignore = project_root.join(".quagga_ignore");

    if project_ignore.exists() {
        builder.add_ignore(project_ignore);
    }

    // Add .quagga_ignore from the home directory
    if let Some(home_dir) = home_dir() {
        let home_ignore = home_dir.join(".quagga_ignore");
        if home_ignore.exists() {
            builder.add_ignore(home_ignore);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::temp_dir::TempDir;

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
        // Collect all paths into a Vec<PathBuf>
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
}
