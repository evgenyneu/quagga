use expectrl::spawn;
use quagga::test_utils::temp_dir::TempDir;
use std::io::Read;

/// Constructs a command using the `quagga` binary and the provided parameters,
/// spawns it in a terminal emulator, and returns the captured output.
///
/// # Arguments
///
/// * `params` - A `String` containing the parameters to pass to the `quagga` binary.
///
/// # Returns
///
/// A `String` containing the output of the command executed in the terminal.
///
/// # Example
///
/// ```rust
/// let params = String::from("--help");
/// let output = run_in_terminal(params);
/// println!("{}", output);
/// ```
pub fn run_in_terminal(params: String) -> String {
    let quagga_bin = assert_cmd::cargo::cargo_bin("quagga");

    let cmd = format!("{} {}", quagga_bin.display(), params);

    // Spawn the command in a terminal emulator
    let mut p = spawn(cmd).expect("Failed to spawn command");

    let mut output = String::new();
    p.read_to_string(&mut output)
        .expect("Failed to read output");

    output.replace("\r\n", "\n") // Normalize line endings
}

/// Adds a custom template to the provided temporary directory.
pub fn add_template(td: &TempDir) {
    let custom_template = r#"{{HEADER}}
{{CONTENT}}
{{FOOTER}}
"#;

    td.mkfile_with_contents(".quagga_template", custom_template);
}
