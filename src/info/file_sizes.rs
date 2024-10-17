use crate::file::size::human_readable_size;
use std::fs;
use std::io;
use std::path::PathBuf;

struct FileWithSize {
    path: PathBuf,
    size: u64,
}

pub fn get_formatted_file_sizes(file_paths: Vec<PathBuf>) -> io::Result<String> {
    let files_with_sizes = collect_file_sizes(file_paths)?;
    let sorted_files = sort_files_by_size(files_with_sizes);
    Ok(format_files_with_sizes(sorted_files))
}

fn collect_file_sizes(file_paths: Vec<PathBuf>) -> io::Result<Vec<FileWithSize>> {
    file_paths
        .into_iter()
        .filter_map(|path| {
            fs::metadata(&path).ok().and_then(|metadata| {
                if metadata.is_file() {
                    Some(Ok(FileWithSize {
                        path,
                        size: metadata.len(),
                    }))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn sort_files_by_size(mut files: Vec<FileWithSize>) -> Vec<FileWithSize> {
    files.sort_by(|a, b| b.size.cmp(&a.size));
    files
}

fn format_files_with_sizes(files: Vec<FileWithSize>) -> String {
    files
        .into_iter()
        .map(|file| {
            format!(
                "[{}] {}",
                human_readable_size(file.size),
                file.path.display()
            )
        })
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::temp_dir::TempDir;

    #[test]
    fn test_get_formatted_file_sizes() {
        let td = TempDir::new().unwrap();
        let path1 = td.mkfile_with_contents("file1.txt", &"A".repeat(1000));
        let path2 = td.mkfile_with_contents("file2.txt", &"B".repeat(2000));
        let path3 = td.mkfile_with_contents("file3.txt", &"C".repeat(500));

        let file_paths = vec![path1.clone(), path2.clone(), path3.clone()];
        let result = get_formatted_file_sizes(file_paths).unwrap();

        let expected = format!(
            "\
[1.95 KB] {}
[1000 B] {}
[500 B] {}",
            path2.display(),
            path1.display(),
            path3.display()
        );

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_formatted_file_sizes_empty() {
        let file_paths = vec![];

        let result = get_formatted_file_sizes(file_paths).unwrap();

        assert_eq!(result, "");
    }
}
