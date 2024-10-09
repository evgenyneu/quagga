use arboard::Clipboard;
use std::error::Error;
use std::io::{self, Write};

pub fn output_to_clipboard(content: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut clipboard = Clipboard::new()?;

    if content.len() == 1 {
        clipboard.set_text(content[0].trim().to_string())?;
        println!("Output copied to clipboard.");
    } else {
        for (index, part) in content.iter().enumerate() {
            clipboard.set_text(part.trim().to_string())?;

            if index < content.len() - 1 {
                println!(
                    "Part {} of {} copied to clipboard.",
                    index + 1,
                    content.len()
                );
                println!("Press Enter to copy the next part");
                wait_for_enter()?;
            } else {
                println!(
                    "Part {} of {} copied to clipboard.",
                    index + 1,
                    content.len()
                );
            }
        }
    }

    Ok(())
}

fn wait_for_enter() -> io::Result<()> {
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(())
}
