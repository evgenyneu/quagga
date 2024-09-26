use std::env;
use std::fs;
use std::io::Write;
use std::io::{self, Result};
use std::path::{Path, PathBuf};

/// A simple wrapper for creating a temporary directory that is
/// automatically deleted when it's dropped.
#[derive(Debug)]
pub struct TempDir(PathBuf);

impl Drop for TempDir {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

impl TempDir {
    /// Create a new empty temporary directory under the system's configured
    /// temporary directory.
    pub fn new() -> Result<TempDir> {
        use std::sync::atomic::{AtomicUsize, Ordering};

        static TRIES: usize = 100;
        static COUNTER: AtomicUsize = AtomicUsize::new(0);

        let tmpdir = env::temp_dir();
        for _ in 0..TRIES {
            let count = COUNTER.fetch_add(1, Ordering::SeqCst);
            let path = tmpdir.join("rust-ignore").join(count.to_string());
            if path.is_dir() {
                continue;
            }
            fs::create_dir_all(&path).map_err(|e| {
                io::Error::new(
                    io::ErrorKind::Other,
                    format!("failed to create {}: {}", path.display(), e),
                )
            })?;
            return Ok(TempDir(path));
        }
        Err(io::Error::new(
            io::ErrorKind::Other,
            "failed to create temp dir after 100 tries",
        ))
    }

    /// Return the underlying path to this temporary directory.
    pub fn path(&self) -> &Path {
        &self.0
    }

    pub fn path_buf(&self) -> PathBuf {
        self.0.clone()
    }

    /// Create directory
    pub fn mkdir<P: AsRef<Path>>(&self, path: P) {
        let full_path = self.path().join(path);
        std::fs::create_dir_all(full_path).unwrap();
    }

    /// Creates a file with default content ("contents") in the temporary directory.
    ///
    /// # Arguments
    ///
    /// * `path` - The relative path within the temporary directory for the new file.
    ///
    /// # Returns
    ///
    /// A `PathBuf` representing the full path to the created file.
    pub fn mkfile<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        return self.mkfile_with_contents(path, "contents"); // Call the new method with default content
    }

    // Existing methods...

    /// Creates a file with the specified contents in the temporary directory.
    ///
    /// # Arguments
    ///
    /// * `path` - The relative path within the temporary directory for the new file.
    /// * `contents` - The content to write to the file.
    ///
    /// # Returns
    ///
    /// A `PathBuf` representing the full path to the created file.
    pub fn mkfile_with_contents<P: AsRef<Path>>(&self, path: P, contents: &str) -> PathBuf {
        let full_path = self.path().join(path);
        let mut file = fs::File::create(&full_path).unwrap();
        file.write_all(contents.as_bytes()).unwrap();
        full_path
    }

    /// Asserts that the specified path exists in the given list of files.
    pub fn assert_contains(&self, files: &Vec<PathBuf>, path: &str) {
        self.assert_contains_with_exist(files, path, true);
    }

    /// Asserts that the specified path does not exist in the given list of files.
    pub fn assert_not_contains(&self, files: &Vec<PathBuf>, path: &str) {
        self.assert_contains_with_exist(files, path, false);
    }

    /// Helper method to check existence of a path in the list of files.
    pub fn assert_contains_with_exist(&self, files: &Vec<PathBuf>, path: &str, should_exist: bool) {
        let files_str: Vec<String> = files
            .iter()
            .map(|path| path.to_string_lossy().into_owned())
            .collect();

        let path_str = self
            .path()
            .join(Path::new(path))
            .to_string_lossy()
            .into_owned();

        if should_exist {
            assert!(
                files_str.contains(&path_str),
                "Expected file {:?} to be present",
                path_str
            );
        } else {
            assert!(
                !files_str.contains(&path_str),
                "Expected file {:?} to not be present",
                path_str
            );
        }
    }
}
