mod common;
use assert_cmd::Command;
use common::{add_template, run_in_terminal};
use expectrl::{spawn, Eof};
use quagga::test_utils::temp_dir::TempDir;
use std::fs;
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
    let custom_template = r#"
<template>
  <prompt>
    <header>Custom Header</header>
    <file>Custom Item: <file-content></file>
    <footer>Custom Footer</footer>
  </prompt>

  <part>
    <header>Part start</header>
    <footer>Part end</footer>
    <pending>If part pending</pending>
  </part>
</template>
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

#[test]
fn test_main_max_total_size_exceeds_maximum() {
    let td = TempDir::new().unwrap();
    add_template(&td);
    td.mkfile_with_contents("four_bytes.txt", "123456"); // 6 bytes

    let output = run_in_terminal(format!("--max-total-size 5 {}", td.path().display()));

    assert!(output.contains("exceeds the maximum"));
}

#[test]
fn test_main_split_into_parts() {
    let td = TempDir::new().unwrap();

    // Create a custom .quagga_template in the temporary directory
    let custom_template = r#"
<template>
  <prompt>
    <header>Custom Header</header>
    <file>Custom Item: <file-content></file>
    <footer>Custom Footer</footer>
  </prompt>

  <part>
    <header>
      == Part start
    </header>
    <footer>
      == Part end
    </footer>
    <pending>Wait for more parts please</pending>
  </part>
</template>
"#;

    td.mkfile_with_contents(".quagga_template", custom_template);
    td.mkfile_with_contents("file1.txt", &"Hello".repeat(10));
    td.mkfile_with_contents("file2.txt", &"World!".repeat(10));

    let output: String = run_in_terminal(format!("--max-part-size 164 {}", td.path().display()));

    let expected = r#"Custom Header

== Part start

Custom Item: HelloHelloHelloHelloHelloHelloHelloHelloHelloHello

== Part end

Wait for more parts please


== Part start

Custom Item: World!World!World!World!World!World!World!World!World!World!

== Part end

Custom Footer
"#;

    assert_eq!(output, expected);
}

#[test]
fn test_main_output_to_file() {
    let td: TempDir = TempDir::new().unwrap();
    add_template(&td);
    td.mkfile_with_contents("file1.txt", "Hello");
    td.mkfile_with_contents("file2.txt", "World!");
    let output_path = td.path().join("output.txt");

    let output: String = run_in_terminal(format!(
        "--output {} {}",
        output_path.display(),
        td.path().display()
    ));

    assert_eq!(output, "");

    let written_content = fs::read_to_string(&output_path).unwrap();

    let expected = "Hello
World!";

    assert_eq!(written_content, expected);
}

#[test]
fn test_main_output_to_file_multiple_parts() {
    let td = TempDir::new().unwrap();
    let output_path = td.path().join("output.txt");

    // Create a custom .quagga_template in the temporary directory
    let custom_template = r#"
<template>
  <prompt>
    <header>Custom Header</header>
    <file>Custom Item: <file-content></file>
    <footer>Custom Footer</footer>
  </prompt>

  <part>
    <header>
      == Part start
    </header>
    <footer>
      == Part end
    </footer>
    <pending>Wait for more parts please</pending>
  </part>
</template>
"#;

    td.mkfile_with_contents(".quagga_template", custom_template);
    td.mkfile_with_contents("file1.txt", &"Hello".repeat(10));
    td.mkfile_with_contents("file2.txt", &"World!".repeat(10));

    let output: String = run_in_terminal(format!(
        "--output {} --max-part-size 164 {}",
        output_path.display(),
        td.path().display()
    ));

    assert_eq!(output, "");

    // Check part one
    // --------

    let path1 = td.path().join("output.txt.001");
    let written_content = fs::read_to_string(&path1).unwrap();

    let expected = "Custom Header

== Part start

Custom Item: HelloHelloHelloHelloHelloHelloHelloHelloHelloHello

== Part end

Wait for more parts please";

    assert_eq!(written_content, expected);

    // Check part two
    // --------

    let path2 = td.path().join("output.txt.002");
    let written_content = fs::read_to_string(&path2).unwrap();

    let expected = "== Part start

Custom Item: World!World!World!World!World!World!World!World!World!World!

== Part end

Custom Footer";

    assert_eq!(written_content, expected);
}

#[test]
fn test_main_output_to_clipboard_single_part() {
    let td: TempDir = TempDir::new().unwrap();
    add_template(&td);
    td.mkfile_with_contents("file.txt", "Hello");

    let output: String = run_in_terminal(format!("--clipboard {}", td.path().display()));

    assert_eq!(output.trim(), "Output copied to clipboard.");
}
