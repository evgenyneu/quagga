use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// Helper function to recursively build the tree string.
fn build_tree(tree: &BTreeMap<String, Node>, prefix: String, is_last: bool, output: &mut String) {
    let connector = if is_last { "└── " } else { "├── " };
    for (i, (name, node)) in tree.iter().enumerate() {
        let is_last = i == tree.len() - 1;
        output.push_str(&format!("{}{}{}\n", prefix, connector, name));

        if let Node::Directory(ref sub_tree) = node {
            let new_prefix = if is_last {
                format!("{}    ", prefix)
            } else {
                format!("{}│   ", prefix)
            };
            build_tree(sub_tree, new_prefix, is_last, output);
        }
    }
}

/// Represents a node in the directory tree (either a directory or a file).
enum Node {
    Directory(BTreeMap<String, Node>),
    File,
}

/// Builds an ASCII tree from a list of file paths and a root directory.
pub fn file_paths_to_tree(paths: Vec<PathBuf>, root: PathBuf) -> String {
    let mut tree = BTreeMap::new();

    // Insert paths into the tree structure.
    for path in paths {
        let relative_path = path.strip_prefix(&root).unwrap_or(&path);
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
    build_tree(&tree, String::new(), true, &mut output);
    output
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_paths_to_tree() {
        let paths = vec![
            PathBuf::from("/dir1/dir2/CONTRIBUTING.md"),
            PathBuf::from("/dir1/dir2/Cargo.toml"),
            PathBuf::from("/dir1/dir2/LICENSE"),
            PathBuf::from("/dir1/dir2/README.md"),
            PathBuf::from("/dir1/dir2/TODO.md"),
            PathBuf::from("/dir1/dir2/docs/development.md"),
            PathBuf::from("/dir1/dir2/src/binary_detector.rs"),
            PathBuf::from("/dir1/dir2/src/cli.rs"),
            PathBuf::from("/dir1/dir2/src/dry_run.rs"),
            PathBuf::from("/dir1/dir2/src/file_reader.rs"),
            PathBuf::from("/dir1/dir2/src/file_walker.rs"),
            PathBuf::from("/dir1/dir2/src/lib.rs"),
            PathBuf::from("/dir1/dir2/src/main.rs"),
            PathBuf::from("/dir1/dir2/src/processor.rs"),
            PathBuf::from("/dir1/dir2/src/quagga_ignore.rs"),
            PathBuf::from("/dir1/dir2/src/template/concatenate.rs"),
            PathBuf::from("/dir1/dir2/src/template/mod.rs"),
            PathBuf::from("/dir1/dir2/src/template/template.rs"),
            PathBuf::from("/dir1/dir2/src/template/validator.rs"),
            PathBuf::from("/dir1/dir2/src/test_utils/mod.rs"),
            PathBuf::from("/dir1/dir2/src/test_utils/temp_dir.rs"),
            PathBuf::from("/dir1/dir2/src/walk_overrides.rs"),
            PathBuf::from("/dir1/dir2/templates/default.txt"),
            PathBuf::from("/dir1/dir2/tests/integration_test.rs"),
        ];

        let root = PathBuf::from("/dir1/dir2");
        let result = file_paths_to_tree(paths, root);
        let expected = r#"
.
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
}
