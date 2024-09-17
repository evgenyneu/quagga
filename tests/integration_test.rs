use assert_cmd::Command;
use quagga::test_utils::temp_dir::TempDir;

#[test]
fn test_main_success() {
    let mut cmd = Command::cargo_bin("quagga").unwrap();
    let td = TempDir::new().unwrap();
    td.mkfile_with_contents("file1.txt", "Hello");
    td.mkfile_with_contents("file2.txt", " World!");

    // Run the command
    cmd.arg(td.path());

    cmd.assert().success().stdout("Hello World!\n");
}

#[test]
fn test_main_with_nonexistent_directory() {
    let mut cmd = Command::cargo_bin("quagga").unwrap();

    // Use a non-existent directory
    let non_existent_path = "/path/to/nonexistent/directory";

    // Run the command
    cmd.arg(non_existent_path);

    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("Error"));
}
