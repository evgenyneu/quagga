use arboard::Clipboard;
use std::error::Error;

pub fn output_to_clipboard(content: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut clipboard = Clipboard::new()?;
    let msg = content.join("\n").trim().to_string();
    clipboard.set_text(msg)?;
    println!("Output copied to clipboard.");
    Ok(())
}
