use std::env;
use std::fs;
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

    pub fn assert_contains(&self, files: &Vec<PathBuf>, path: &Path) {
        self.assert_contains_with_exist(files, path, true);
    }

    pub fn assert_not_contains(&self, files: &Vec<PathBuf>, path: &Path) {
        self.assert_contains_with_exist(files, path, false);
    }

    pub fn assert_contains_with_exist(
        &self,
        files: &Vec<PathBuf>,
        path: &Path,
        should_exist: bool,
    ) {
        let files_str: Vec<String> = files
            .iter()
            .map(|path| path.to_string_lossy().into_owned())
            .collect();

        let path_str = self.path().join(path).to_string_lossy().into_owned();

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
