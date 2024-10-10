use arboard::Clipboard;
use std::error::Error;
use std::io::{self, Write};

pub fn output_to_clipboard(content: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut clipboard = Clipboard::new()?;

    if content.len() == 1 {
        copy_single_part(&mut clipboard, &content[0])?;
    } else {
        copy_multiple_parts(&mut clipboard, &content)?;
    }

    Ok(())
}

fn copy_single_part(clipboard: &mut Clipboard, content: &str) -> Result<(), Box<dyn Error>> {
    clipboard.set_text(content.trim().to_string())?;
    println!("Output copied to clipboard.");
    Ok(())
}

fn copy_multiple_parts(
    clipboard: &mut Clipboard,
    content: &[String],
) -> Result<(), Box<dyn Error>> {
    for (index, part) in content.iter().enumerate() {
        clipboard.set_text(part.trim().to_string())?;

        println!(
            "Part {} of {} copied to clipboard.",
            index + 1,
            content.len()
        );

        if index < content.len() - 1 {
            println!("Press Enter to copy the next part...");
            wait_for_enter()?;
        }
    }

    println!("We are done.");
    Ok(())
}

fn wait_for_enter() -> io::Result<()> {
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(())
}
