use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::path::PathBuf;

#[derive(Debug)]
pub enum TreeBuildError {
    RootMismatch { path: PathBuf, root: PathBuf },
}

/// Builds an ASCII tree from a list of file paths and a root directory.
pub fn file_paths_to_tree(paths: Vec<PathBuf>, root: PathBuf) -> Result<String, TreeBuildError> {
    let mut tree = BTreeMap::new();

    // Insert paths into the tree structure.
    for path in paths {
        // Check if path matches root, otherwise return an error.
        let relative_path = path
            .strip_prefix(&root)
            .map_err(|_| TreeBuildError::RootMismatch {
                path: path.clone(),
                root: root.clone(),
            })?;

        let components: Vec<_> = relative_path
            .components()
            .map(|c| c.as_os_str().to_str().unwrap().to_string())
            .collect();

        let mut current = &mut tree;
        for (i, component) in components.iter().enumerate() {
            if i == components.len() - 1 {
                current.entry(component.clone()).or_insert(Node::File);
            } else {
                current = current
                    .entry(component.clone())
                    .or_insert_with(|| Node::Directory(BTreeMap::new()))
                    .as_directory_mut();
            }
        }
    }

    // Build the ASCII tree string.
    let mut output = String::from(".\n");
    build_tree(&tree, String::new(), &mut output);
    Ok(output)
}

/// Represents a node in the directory tree (either a directory or a file).
enum Node {
    Directory(BTreeMap<String, Node>),
    File,
}

/// Helper method to turn a `Node` into a mutable `Directory`.
impl Node {
    fn as_directory_mut(&mut self) -> &mut BTreeMap<String, Node> {
        match self {
            Node::Directory(ref mut map) => map,
            _ => panic!("Tried to access a file as a directory"),
        }
    }
}

/// Custom comparator to ensure directories are listed before files, and case-insensitive sorting.
fn node_order((name1, node1): &(&String, &Node), (name2, node2): &(&String, &Node)) -> Ordering {
    match (node1, node2) {
        (Node::Directory(_), Node::File) => Ordering::Less, // Directories before files
        (Node::File, Node::Directory(_)) => Ordering::Greater, // Files after directories
        _ => name1.to_lowercase().cmp(&name2.to_lowercase()), // Case-insensitive comparison
    }
}

/// Helper function to recursively build the tree string.
fn build_tree(tree: &BTreeMap<String, Node>, prefix: String, output: &mut String) {
    let mut sorted_entries: Vec<_> = tree.iter().collect();
    sorted_entries.sort_by(node_order); // Sort by custom order

    for (i, (name, node)) in sorted_entries.iter().enumerate() {
        let is_last = i == tree.len() - 1;
        let connector = if is_last { "└── " } else { "├── " };
        output.push_str(&format!("{}{}{}\n", prefix, connector, name));

        if let Node::Directory(ref sub_tree) = node {
            let new_prefix = if is_last {
                format!("{}    ", prefix)
            } else {
                format!("{}│   ", prefix)
            };
            build_tree(sub_tree, new_prefix, output);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_paths_to_tree() {
        let paths = vec![
            PathBuf::from("/dir1/dir2/Cargo.toml"),
            PathBuf::from("/dir1/dir2/CONTRIBUTING.md"),
            PathBuf::from("/dir1/dir2/src/file_reader.rs"),
            PathBuf::from("/dir1/dir2/TODO.md"),
            PathBuf::from("/dir1/dir2/README.md"),
            PathBuf::from("/dir1/dir2/docs/development.md"),
            PathBuf::from("/dir1/dir2/src/binary_detector.rs"),
            PathBuf::from("/dir1/dir2/src/cli.rs"),
            PathBuf::from("/dir1/dir2/src/dry_run.rs"),
            PathBuf::from("/dir1/dir2/src/file_walker.rs"),
            PathBuf::from("/dir1/dir2/src/lib.rs"),
            PathBuf::from("/dir1/dir2/tests/integration_test.rs"),
            PathBuf::from("/dir1/dir2/src/main.rs"),
            PathBuf::from("/dir1/dir2/src/processor.rs"),
            PathBuf::from("/dir1/dir2/src/template/template.rs"),
            PathBuf::from("/dir1/dir2/src/template/tags/mod.rs"),
            PathBuf::from("/dir1/dir2/src/quagga_ignore.rs"),
            PathBuf::from("/dir1/dir2/src/template/concatenate.rs"),
            PathBuf::from("/dir1/dir2/src/template/mod.rs"),
            PathBuf::from("/dir1/dir2/src/template/validator.rs"),
            PathBuf::from("/dir1/dir2/src/template/tags/all_file_paths.rs"),
            PathBuf::from("/dir1/dir2/src/test_utils/mod.rs"),
            PathBuf::from("/dir1/dir2/LICENSE"),
            PathBuf::from("/dir1/dir2/src/test_utils/temp_dir.rs"),
            PathBuf::from("/dir1/dir2/src/walk_overrides.rs"),
            PathBuf::from("/dir1/dir2/templates/default.txt"),
        ];

        let root = PathBuf::from("/dir1/dir2");

        let result = file_paths_to_tree(paths, root).unwrap();

        let expected = r#".
├── docs
│   └── development.md
├── src
│   ├── template
│   │   ├── tags
│   │   │   ├── all_file_paths.rs
│   │   │   └── mod.rs
│   │   ├── concatenate.rs
│   │   ├── mod.rs
│   │   ├── template.rs
│   │   └── validator.rs
│   ├── test_utils
│   │   ├── mod.rs
│   │   └── temp_dir.rs
│   ├── binary_detector.rs
│   ├── cli.rs
│   ├── dry_run.rs
│   ├── file_reader.rs
│   ├── file_walker.rs
│   ├── lib.rs
│   ├── main.rs
│   ├── processor.rs
│   ├── quagga_ignore.rs
│   └── walk_overrides.rs
├── templates
│   └── default.txt
├── tests
│   └── integration_test.rs
├── Cargo.toml
├── CONTRIBUTING.md
├── LICENSE
├── README.md
└── TODO.md
"#;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_empty_paths() {
        let paths = vec![];
        let root = PathBuf::from("/dir1");
        let result = file_paths_to_tree(paths, root).unwrap();
        assert_eq!(result, ".\n");
    }

    #[test]
    fn test_root_directory_only() {
        let paths = vec![PathBuf::from("/dir1")];
        let root = PathBuf::from("/dir1");
        let result = file_paths_to_tree(paths, root).unwrap();
        assert_eq!(result, ".\n");
    }

    #[test]
    fn test_single_file_in_root() {
        let paths = vec![PathBuf::from("/dir1/file.txt")];
        let root = PathBuf::from("/dir1");

        let result = file_paths_to_tree(paths, root).unwrap();

        let expected = r#".
└── file.txt
"#;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_deeply_nested_directory() {
        let paths = vec![PathBuf::from("/dir1/level1/level2/level3/level4/file.txt")];
        let root = PathBuf::from("/dir1");

        let result = file_paths_to_tree(paths, root).unwrap();

        let expected = r#".
└── level1
    └── level2
        └── level3
            └── level4
                └── file.txt
"#;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_same_file_name_in_different_directories() {
        let paths = vec![
            PathBuf::from("/dir1/dirA/file.txt"),
            PathBuf::from("/dir1/dirB/file.txt"),
        ];
        let root = PathBuf::from("/dir1");

        let result = file_paths_to_tree(paths, root).unwrap();

        let expected = r#".
├── dirA
│   └── file.txt
└── dirB
    └── file.txt
"#;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_mixed_case_sensitivity() {
        let paths = vec![
            PathBuf::from("/dir1/File.txt"),
            PathBuf::from("/dir1/file.txt"),
        ];

        let root = PathBuf::from("/dir1");

        let result = file_paths_to_tree(paths, root).unwrap();

        let expected = r#".
├── File.txt
└── file.txt
"#;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_special_characters_in_paths() {
        let paths = vec![
            PathBuf::from("/dir1/special@file.txt"),
            PathBuf::from("/dir1/dir with space/file.txt"),
        ];

        let root = PathBuf::from("/dir1");

        let result = file_paths_to_tree(paths, root).unwrap();

        let expected = r#".
├── dir with space
│   └── file.txt
└── special@file.txt
"#;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_start_with_dot_dir() {
        let paths = vec![PathBuf::from("./file1.txt"), PathBuf::from("./file2.txt")];
        let root = PathBuf::from(".");

        let expected = r#".
├── file1.txt
└── file2.txt
"#;
        let result = file_paths_to_tree(paths, root).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_root_directory_mismatch() {
        let paths = vec![
            PathBuf::from("/dirA/file1.txt"),
            PathBuf::from("/dirA/file2.txt"),
        ];

        let root = PathBuf::from("/dirB"); // Root is different from paths

        let result = file_paths_to_tree(paths, root);

        match result {
            Err(TreeBuildError::RootMismatch { path, root }) => {
                assert_eq!(path, PathBuf::from("/dirA/file1.txt"));
                assert_eq!(root, PathBuf::from("/dirB"));
            }
            _ => panic!("Expected RootMismatch error"),
        }
    }
}
