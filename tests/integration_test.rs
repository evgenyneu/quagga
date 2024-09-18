use assert_cmd::Command;
use quagga::test_utils::temp_dir::TempDir;

#[test]
fn test_main_success() {
    let mut cmd = Command::cargo_bin("quagga").unwrap();
    let td = TempDir::new().unwrap();

    let path1 = td.mkfile_with_contents("file1.txt", "Hello");
    let path2 = td.mkfile_with_contents("file2.txt", " World!");

    // Run the command
    cmd.arg(td.path());

    let expected_output = format!(
        "\n\n-------\n{}\n-------\n\nHello\n\n-------\n{}\n-------\n\n World!\n",
        path1.display(),
        path2.display()
    );

    cmd.assert().success().stdout(expected_output);
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
