use std::path::PathBuf;

/// Represents the content of a file along with its path.
///
/// # Fields
///
/// * `path` - The file path.
/// * `content` - The contents of the file as a `String`.
#[derive(Debug)]
pub struct FileContent {
    pub path: PathBuf,
    pub content: String,
}
