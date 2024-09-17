// tests/integration_test.rs

use assert_cmd::Command;

#[test]
fn test_main_success() {
    let mut cmd = Command::cargo_bin("quagga").unwrap();

    // Create a temporary directory and files
    let td = tempfile::tempdir().unwrap();
    let file1 = td.path().join("file1.txt");
    let file2 = td.path().join("file2.txt");
    std::fs::write(&file1, "Hello").unwrap();
    std::fs::write(&file2, " World!").unwrap();

    // Run the command
    cmd.arg(td.path());

    cmd.assert()
        .success()
        .stdout("Hello World!\n");
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
