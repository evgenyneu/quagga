use assert_cmd::Command;
use expectrl::{spawn, Eof, WaitStatus};
use quagga::test_utils::temp_dir::TempDir;
use std::io::Read;

#[test]
fn test_main_success_run_from_terminal() {
    let td = TempDir::new().unwrap();
    let path1 = td.mkfile_with_contents("file1.txt", "Hello");
    let path2 = td.mkfile_with_contents("file2.txt", "World!");

    let quagga_bin = assert_cmd::cargo::cargo_bin("quagga");

    // Spawn the quagga binary in a PTY
    let mut p = expectrl::spawn(format!("{} {}", quagga_bin.display(), td.path().display()))
        .expect("Failed to spawn quagga binary");

    let mut output = String::new();

    p.read_to_string(&mut output)
        .expect("Failed to read output from quagga");

    let output = output.replace("\r\n", "\n");

    // Define the expected output
    let expected_output = format!(
        "\n\n-------\n{}\n-------\n\nHello\n\n-------\n{}\n-------\n\nWorld!\n",
        path1.display(),
        path2.display()
    );

    assert_eq!(output, expected_output);
    p.expect(Eof).expect("Failed to expect EOF");
}

#[test]
fn test_main_with_nonexistent_directory() {
    let mut cmd = Command::cargo_bin("quagga").unwrap();
    let non_existent_path = "/path/to/nonexistent/directory";

    cmd.arg(non_existent_path);

    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("Error"));
}

#[test]
fn test_main_with_piped_input() {
    let mut cmd = Command::cargo_bin("quagga").unwrap();
    let td = TempDir::new().unwrap();

    let file1 = td.mkfile_with_contents("file1.txt", "Hello");
    let file2 = td.mkfile_with_contents("file2.txt", "World!");
    td.mkfile_with_contents("ignore.txt", "ignore"); // This file should be ignored since its path is not piped-in

    let input = format!("{}\n{}", file1.display(), file2.display());

    cmd.write_stdin(input);

    let expected_output = format!(
        "\n\n-------\n{}\n-------\n\nHello\n\n-------\n{}\n-------\n\nWorld!\n",
        file1.display(),
        file2.display()
    );

    cmd.assert().success().stdout(expected_output);
}
