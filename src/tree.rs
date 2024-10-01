use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::path::PathBuf;

/// Builds an ASCII tree representation from a list of file paths:
///
/// ```text
/// .
/// ├── subdir
/// │   └── file2.txt
/// └── file1.txt
/// ```
///
/// This function takes a list of file paths and organizes them into a structured, human-readable
/// ASCII tree. Each file or directory is displayed in hierarchical order based on its relationship
/// to the provided root directory.
///
/// # Arguments
///
/// * `paths` - A vector of `PathBuf` objects representing the file paths to include in the tree.
/// * `root` - An optional `PathBuf` object representing the root directory. This argument helps
///            make the tree more compact. For example, if the root path is `/dir1/dir2` and it
///            contains the file `/dir1/dir2/file.txt`, then the top tree node will be `/dir1/dir2`:
///
/// ```text
/// /dir1/dir2
/// └── file.txt
/// ```
///
/// In this case, the path is not split into individual components (`/`, `dir1`, `dir2`),
/// which makes the tree more compact.
///
/// # Returns
///
/// A `String` containing the ASCII tree representation of the file paths.
pub fn file_paths_to_tree(paths: Vec<PathBuf>, root: Option<PathBuf>) -> String {
    let tree = build_tree_structure(&paths, &root);
    let mut output = String::new();
    build_tree(&tree, String::new(), &mut output, true);
    output
}

/// Build the tree structure from the paths.
///
/// # Arguments
///
/// * `paths` - A vector of `PathBuf` objects representing file paths to include in the tree.
/// * `root` - A `PathBuf` object representing the root directory.
///
/// # Returns
///
/// The tree structure of the paths as a `BTreeMap`.
fn build_tree_structure(paths: &Vec<PathBuf>, root: &Option<PathBuf>) -> BTreeMap<String, Node> {
    let mut tree = BTreeMap::new();

    // Insert paths into the tree structure.
    for path in paths {
        let mut current = &mut tree;

        let relative_path = if let Some(root) = root {
            // Check if the path can be made relative to the root
            if let Ok(stripped) = path.strip_prefix(&root) {
                // Use the full root path as the node key
                // For example, if root path is /dir1/dir2 and it contains the file /dir1/dir2/dir3/file.txt
                // then the tree node will be /dir1/dir2 (i.e. we don't need to split the path into individual components /, dir1 and dir2)
                // In this case we don't split the path into individual components /, dir1 and dir2,
                // which makes the tree more compact
                current = current
                    .entry(root.as_os_str().to_str().unwrap().to_string())
                    .or_insert_with(|| Node::Directory(BTreeMap::new()))
                    .as_directory_mut();

                stripped.to_path_buf() // If so, use the relative path
            } else {
                path.clone() // If not, use the full path
            }
        } else {
            path.to_path_buf()
        };

        let components: Vec<_> = relative_path
            .components()
            .map(|c| c.as_os_str().to_str().unwrap().to_string())
            .collect();

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

    tree
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
fn build_tree(
    tree: &BTreeMap<String, Node>,
    prefix: String,
    output: &mut String,
    is_top_level: bool,
) {
    let mut sorted_entries: Vec<_> = tree.iter().collect();
    sorted_entries.sort_by(node_order); // Sort by custom order

    for (i, (name, node)) in sorted_entries.iter().enumerate() {
        let is_last = i == tree.len() - 1;

        let connector = if is_top_level {
            ""
        } else {
            if is_last {
                "└── "
            } else {
                "├── "
            }
        };

        // let connector = if is_last { "└── " } else { "├── " };
        output.push_str(&format!("{}{}{}\n", prefix, connector, name));

        if let Node::Directory(ref sub_tree) = node {
            let new_prefix = if is_top_level {
                "".to_string()
            } else {
                if is_last {
                    format!("{}    ", prefix)
                } else {
                    format!("{}│   ", prefix)
                }
            };

            build_tree(sub_tree, new_prefix, output, false);
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
            PathBuf::from("/dir1/dir2/src/show_paths.rs"),
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

        let result = file_paths_to_tree(paths, Some(root));

        let expected = r#"/dir1/dir2
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
│   ├── file_reader.rs
│   ├── file_walker.rs
│   ├── lib.rs
│   ├── main.rs
│   ├── processor.rs
│   ├── quagga_ignore.rs
│   ├── show_paths.rs
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
        let result = file_paths_to_tree(paths, Some(root));
        assert_eq!(result, "");
    }

    #[test]
    fn test_root_directory_only() {
        let paths = vec![PathBuf::from("/dir1")];
        let root = PathBuf::from("/dir1");
        let result = file_paths_to_tree(paths, Some(root));
        assert_eq!(result, "/dir1\n");
    }

    #[test]
    fn test_single_file_in_root() {
        let paths = vec![PathBuf::from("/dir1/file.txt")];
        let root = PathBuf::from("/dir1");

        let result = file_paths_to_tree(paths, Some(root));

        let expected = r#"/dir1
└── file.txt
"#;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_deeply_nested_directory() {
        let paths = vec![PathBuf::from("/dir1/level1/level2/level3/level4/file.txt")];
        let root = PathBuf::from("/dir1");

        let result = file_paths_to_tree(paths, Some(root));

        let expected = r#"/dir1
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

        let result = file_paths_to_tree(paths, Some(root));

        let expected = r#"/dir1
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

        let result = file_paths_to_tree(paths, Some(root));

        let expected = r#"/dir1
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

        let result = file_paths_to_tree(paths, Some(root));

        let expected = r#"/dir1
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

        let result = file_paths_to_tree(paths, Some(root));

        let expected = r#".
├── file1.txt
└── file2.txt
"#;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_root_directory_mismatch() {
        let paths = vec![PathBuf::from("/file1.txt"), PathBuf::from("/file2.txt")];
        let root = PathBuf::from("dir"); // Root is different from paths

        let result = file_paths_to_tree(paths, Some(root));

        let expected = r#"/
├── file1.txt
└── file2.txt
"#;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_root_directory_mismatch_with_subdirs() {
        let paths = vec![
            PathBuf::from("dir1/dir2/file1.txt"),
            PathBuf::from("dir1/dir2/file2.txt"),
        ];
        let root = PathBuf::from("dir"); // Root is different from paths

        let result = file_paths_to_tree(paths, Some(root));

        let expected = r#"dir1
└── dir2
    ├── file1.txt
    └── file2.txt
"#;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_root_directory_matches_some_of_the_paths() {
        let paths = vec![
            PathBuf::from("/dir1/dir2/file1.txt"),
            PathBuf::from("/dir1/dir2/file2.txt"),
            PathBuf::from("/dir3/dir4/file1.txt"),
            PathBuf::from("/dir3/dir4/file2.txt"),
        ];

        let root = PathBuf::from("/dir1/dir2");

        let result = file_paths_to_tree(paths, Some(root));

        // Since the root "/dir1/dir2" dir contains the files "/dir1/dir2/file1.txt" and "/dir1/dir2/file2.txt"
        // the dir "/dir1/dir2" will be use as tree node.
        // In this case we don't split the path into individual components /, dir1 and dir2,
        // which makes the tree more compact
        let expected = r#"/
└── dir3
    └── dir4
        ├── file1.txt
        └── file2.txt
/dir1/dir2
├── file1.txt
└── file2.txt
"#;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_split_full_path_when_root_arg_is_missing() {
        let paths = vec![
            PathBuf::from("/dir1/dirA/file.txt"),
            PathBuf::from("/dir1/dirB/file.txt"),
        ];

        let result = file_paths_to_tree(paths, None);

        let expected = r#"/
└── dir1
    ├── dirA
    │   └── file.txt
    └── dirB
        └── file.txt
"#;

        assert_eq!(result, expected);
    }
}
