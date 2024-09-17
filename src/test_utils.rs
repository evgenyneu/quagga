use std::env;
use std::fs;
use std::io::Result;
use std::path::{Path, PathBuf};

/// A simple wrapper for creating a temporary directory that is
/// automatically deleted when it's dropped.
///
/// We use this in lieu of tempfile because tempfile brings in too many
/// dependencies.
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
            fs::create_dir_all(&path)
                .map_err(|e| format!("failed to create {}: {}", path.display(), e))?;
            return Ok(TempDir(path));
        }
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "failed to create temp dir",
        ))
    }

    /// Return the underlying path to this temporary directory.
    pub fn path(&self) -> &Path {
        &self.0
    }
}
