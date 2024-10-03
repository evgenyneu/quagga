mod common;
use assert_cmd::Command;
use common::{add_template, run_in_terminal};
use expectrl::{spawn, Eof};
use quagga::test_utils::temp_dir::TempDir;
use std::io::Read;

#[test]
fn test_main_success_run() {
    let td: TempDir = TempDir::new().unwrap();
    add_template(&td);
    td.mkfile_with_contents("file1.txt", "Hello");
    td.mkfile_with_contents("file2.txt", "World!");

    let output: String = run_in_terminal(td.path().display().to_string());

    let expected_output = r#"Hello
World!

"#;

    assert_eq!(output, expected_output);
}

#[test]
fn test_main_with_nonexistent_directory() {
    let quagga_bin = assert_cmd::cargo::cargo_bin("quagga");
    let non_existent_path = "/path/to/nonexistent/directory";

    // Spawn the quagga binary in a PTY with the non-existent directory as an argument
    let mut p = spawn(format!("{} {}", quagga_bin.display(), non_existent_path))
        .expect("Failed to spawn quagga binary");

    // Read the error output from the process
    let mut stderr_output = String::new();
    p.read_to_string(&mut stderr_output)
        .expect("Failed to read stderr from quagga");

    assert!(stderr_output.contains("Error"));
    p.expect(Eof).expect("Failed to expect EOF");
}

#[test]
fn test_main_with_piped_input() {
    let td = TempDir::new().unwrap();
    add_template(&td);
    let mut cmd = Command::cargo_bin("quagga").unwrap();
    cmd.arg(td.path());

    let path1 = td.mkfile_with_contents("file1.txt", "Hello");
    let path2 = td.mkfile_with_contents("file2.txt", "World!");
    td.mkfile_with_contents("ignore.txt", "ignore"); // This file should be ignored since its path is not piped-in

    let input = format!("{}\n{}", path1.display(), path2.display());

    cmd.write_stdin(input);

    let expected_output = r#"Hello
World!

"#;

    cmd.assert().success().stdout(expected_output);
}

#[test]
fn test_main_show_paths() {
    let td = TempDir::new().unwrap();
    let path1 = td.mkfile("file1.txt");
    let path2 = td.mkfile("file2.txt");

    let output: String = run_in_terminal(format!("--show-paths {}", td.path().display()));

    let expected = format!("{}\n{}\n", path1.display(), path2.display());
    assert_eq!(output, expected);
}

#[test]
fn test_main_show_tree() {
    let td = TempDir::new().unwrap();
    td.mkfile("file1.txt");
    td.mkfile("file2.txt");
    td.mkdir("subdir");
    td.mkfile("subdir/file3.txt");

    let output: String = run_in_terminal(format!("--tree {}", td.path().display()));

    let expected = format!(
        r#"{}
├── subdir
│   └── file3.txt
├── file1.txt
└── file2.txt
"#,
        td.path().display()
    );

    assert_eq!(output, expected);
}

#[test]
fn test_main_copy_template() {
    let td = TempDir::new().unwrap();
    let output: String = run_in_terminal(format!("--copy-template {}", td.path().display()));

    let expected = format!(
        "Template was copied to '{}'.\n",
        td.path().join(".quagga_template").display()
    );

    assert_eq!(output, expected);
}

#[test]
fn test_main_uses_quagga_template_by_default() {
    let td = TempDir::new().unwrap();

    // Create a custom .quagga_template in the temporary directory
    let custom_template = r#"Custom Header

{{HEADER}}
Custom Item: {{CONTENT}}
{{FOOTER}}

Custom Footer
"#;

    td.mkfile_with_contents(".quagga_template", custom_template);
    td.mkfile_with_contents("file1.txt", "Hello");
    td.mkfile_with_contents("file2.txt", "World!");

    let output: String = run_in_terminal(td.path().display().to_string());

    let expected = r#"Custom Header

Custom Item: Hello
Custom Item: World!

Custom Footer
"#;

    assert_eq!(output, expected);
}

#[test]
fn test_main_with_contain_option() {
    let td: TempDir = TempDir::new().unwrap();
    add_template(&td);

    // Create files with and without the target content
    td.mkfile_with_contents("file_with_keyword.txt", "This file contains the keyword.");
    td.mkfile_with_contents("file_without_keyword.txt", "This file does not contain it.");

    let output = run_in_terminal(format!("--contain keyword -- {}", td.path().display()));

    let expected = r#"This file contains the keyword.

"#;

    assert_eq!(output, expected);
}

#[test]
fn test_main_respect_gitignore() {
    let td = TempDir::new().unwrap();
    add_template(&td);
    td.mkfile_with_contents(".gitignore", "ignored.txt");

    // Create files that should be included
    td.mkfile_with_contents("file1.txt", "file1");
    td.mkfile_with_contents("file2.txt", "file2");

    // Create a file that should be ignored by .gitignore
    td.mkfile_with_contents("ignored.txt", "ignored");

    let output = run_in_terminal(td.path().display().to_string());

    let expected = r#"file1
file2

"#;

    assert_eq!(output, expected);
}

#[test]
fn test_main_no_gitignore() {
    let td = TempDir::new().unwrap();
    add_template(&td);
    td.mkfile_with_contents(".gitignore", "ignored.txt");
    td.mkfile_with_contents("file1.txt", "file1");
    td.mkfile_with_contents("file2.txt", "file2");
    td.mkfile_with_contents("ignored.txt", "ignored");

    // Ignore .gitignore
    let output = run_in_terminal(format!("--no-gitignore {}", td.path().display()));

    let expected = r#"file1
file2
ignored

"#;

    assert_eq!(output, expected);
}

#[test]
fn test_main_max_depth() {
    let td = TempDir::new().unwrap();
    add_template(&td);
    td.mkfile_with_contents("file0.txt", "file0");
    td.mkdir("dir1");
    td.mkfile_with_contents("dir1/file1.txt", "file1");
    td.mkdir("dir1/dir2");
    td.mkfile_with_contents("dir1/dir2/file2.txt", "file2");
    td.mkdir("dir1/dir2/dir3");
    td.mkfile_with_contents("dir1/dir2/dir3/file3.txt", "file3");

    let output = run_in_terminal(format!("--max-depth 2 {}", td.path().display()));

    let expected = r#"file1
file0

"#;

    assert_eq!(output, expected);
}

#[test]
fn test_main_max_filesize() {
    let td = TempDir::new().unwrap();
    add_template(&td);
    td.mkfile_with_contents("four_bytes.txt", "1234");
    td.mkfile_with_contents("five_bytes.txt", "12345");

    let output = run_in_terminal(format!("--max-filesize 4 {}", td.path().display()));

    let expected = r#"1234

"#;

    assert_eq!(output, expected);
}
