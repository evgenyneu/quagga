pub fn output_to_stdout(content: Vec<String>) {
    println!("{}", content.join("\n").trim());
}
