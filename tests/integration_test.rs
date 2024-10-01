use assert_cmd::Command;
use expectrl::{spawn, Eof};
use quagga::test_utils::temp_dir::TempDir;
use std::io::Read;

#[test]
fn test_main_success_run() {
    let td: TempDir = TempDir::new().unwrap();
    let path1 = td.mkfile_with_contents("file1.txt", "Hello");
    let path2 = td.mkfile_with_contents("file2.txt", "World!");

    let quagga_bin = assert_cmd::cargo::cargo_bin("quagga");

    // Spawn the quagga binary in a terminal
    let mut p = expectrl::spawn(format!("{} {}", quagga_bin.display(), td.path().display()))
        .expect("Failed to spawn quagga binary");

    let mut output = String::new();

    p.read_to_string(&mut output)
        .expect("Failed to read output from quagga");

    let output = output.replace("\r\n", "\n");

    let expected_output = format!(
        "\
The following is my code:

------ FILE START {} ------

Hello

------ {} FILE END ------

------ FILE START {} ------

World!

------ {} FILE END ------

All files:
{}
├── file1.txt
└── file2.txt

Reminding the important rules:
* Discuss the code changes first, don't suggest any code changes before we agreed on the approach.
* Think of an alternative/better way to do what I ask, don't simply follow my instructions.
* One small code change at a time.
* All code needs to be tested.
* Write code in such a way that so it can be used as a library, which also means it needs proper comments and documentation.
* Focus on code clarity and simplicity, even if it means writing more code (i.e. don't try to be smart or elegant D:).
* Write small functions that do one thing :D It makes the code simpler and easier to test.
* In the response text that is not the code, be very concise.

What do you think? Let's discuss ideas first without code :D
",
        path1.display(),
        path1.display(),
        path2.display(),
        path2.display(),
        td.path().display()
    );

    assert_eq!(output, expected_output);
    p.expect(Eof).expect("Failed to expect EOF");
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
    let mut cmd = Command::cargo_bin("quagga").unwrap();
    cmd.arg(td.path());

    let path1 = td.mkfile_with_contents("file1.txt", "Hello");
    let path2 = td.mkfile_with_contents("file2.txt", "World!");
    td.mkfile_with_contents("ignore.txt", "ignore"); // This file should be ignored since its path is not piped-in

    let input = format!("{}\n{}", path1.display(), path2.display());

    cmd.write_stdin(input);

    let expected_output = format!(
        "\
The following is my code:

------ FILE START {} ------

Hello

------ {} FILE END ------

------ FILE START {} ------

World!

------ {} FILE END ------

All files:
{}
├── file1.txt
└── file2.txt

Reminding the important rules:
* Discuss the code changes first, don't suggest any code changes before we agreed on the approach.
* Think of an alternative/better way to do what I ask, don't simply follow my instructions.
* One small code change at a time.
* All code needs to be tested.
* Write code in such a way that so it can be used as a library, which also means it needs proper comments and documentation.
* Focus on code clarity and simplicity, even if it means writing more code (i.e. don't try to be smart or elegant D:).
* Write small functions that do one thing :D It makes the code simpler and easier to test.
* In the response text that is not the code, be very concise.

What do you think? Let's discuss ideas first without code :D
",
        path1.display(),
        path1.display(),
        path2.display(),
        path2.display(),
        td.path().display()
    );

    cmd.assert().success().stdout(expected_output);
}

#[test]
fn test_main_show_paths() {
    let td = TempDir::new().unwrap();
    let path1 = td.mkfile("file1.txt");
    let path2 = td.mkfile("file2.txt");

    let quagga_bin = assert_cmd::cargo::cargo_bin("quagga");

    // Spawn the quagga binary in a terminal
    let mut p = expectrl::spawn(format!(
        "{} --show-paths {}",
        quagga_bin.display(),
        td.path().display()
    ))
    .expect("Failed to spawn quagga binary");

    let mut output = String::new();

    p.read_to_string(&mut output)
        .expect("Failed to read output from quagga");

    let output = output.replace("\r\n", "\n");

    let expected = format!("{}\n{}\n", path1.display(), path2.display());
    assert_eq!(output, expected);
    p.expect(Eof).expect("Failed to expect EOF");
}

#[test]
fn test_main_show_tree() {
    let td = TempDir::new().unwrap();
    td.mkfile("file1.txt");
    td.mkfile("file2.txt");
    td.mkdir("subdir");
    td.mkfile("subdir/file3.txt");

    let quagga_bin = assert_cmd::cargo::cargo_bin("quagga");

    // Spawn the quagga binary in a terminal
    let mut p = expectrl::spawn(format!(
        "{} --tree {}",
        quagga_bin.display(),
        td.path().display()
    ))
    .expect("Failed to spawn quagga binary");

    let mut output = String::new();

    p.read_to_string(&mut output)
        .expect("Failed to read output from quagga");

    let output = output.replace("\r\n", "\n");

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
    p.expect(Eof).expect("Failed to expect EOF");
}

#[test]
fn test_main_copy_template() {
    let td = TempDir::new().unwrap();
    let quagga_bin = assert_cmd::cargo::cargo_bin("quagga");

    // Spawn the quagga binary in a terminal
    let mut p = expectrl::spawn(format!(
        "{} --copy-template {}",
        quagga_bin.display(),
        td.path().display()
    ))
    .expect("Failed to spawn quagga binary");

    let mut output = String::new();

    p.read_to_string(&mut output)
        .expect("Failed to read output from quagga");

    let output = output.replace("\r\n", "\n");

    let expected = format!(
        "Template was copied to '{}'.\n",
        td.path().join(".quagga_template").display()
    );

    assert_eq!(output, expected);
    p.expect(Eof).expect("Failed to expect EOF");
}
