use assert_cmd::Command;
use expectrl::{spawn, Eof};
use quagga::test_utils::temp_dir::TempDir;
use std::io::Read;

#[test]
fn test_main_success_run() {
    let td = TempDir::new().unwrap();
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

Reminding the important rules:
* Discuss code changes first. Don't suggest any changes until we've agreed on the approach.
* Always think of alternative or better ways to achieve the goal. Don't blindly follow instructions.
* Make one small code change at a time.
* All code must be tested and documented (no need to comment on code if its purpose is obvious).
* Prioritize clarity and simplicity, even if it means writing more code (avoid being overly clever or elegant).
* Write small, single-purpose functions to keep the code simple and easy to test.
* Be concise in any response text that is not code.
",
        path1.display(),
        path1.display(),
        path2.display(),
        path2.display()
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
    let mut cmd = Command::cargo_bin("quagga").unwrap();
    let td = TempDir::new().unwrap();

    let file1 = td.mkfile_with_contents("file1.txt", "Hello");
    let file2 = td.mkfile_with_contents("file2.txt", "World!");
    td.mkfile_with_contents("ignore.txt", "ignore"); // This file should be ignored since its path is not piped-in

    let input = format!("{}\n{}", file1.display(), file2.display());

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

Reminding the important rules:
* Discuss code changes first. Don't suggest any changes until we've agreed on the approach.
* Always think of alternative or better ways to achieve the goal. Don't blindly follow instructions.
* Make one small code change at a time.
* All code must be tested and documented (no need to comment on code if its purpose is obvious).
* Prioritize clarity and simplicity, even if it means writing more code (avoid being overly clever or elegant).
* Write small, single-purpose functions to keep the code simple and easy to test.
* Be concise in any response text that is not code.
",
      file1.display(),
      file1.display(),
      file2.display(),
      file2.display()
  );

    cmd.assert().success().stdout(expected_output);
}
