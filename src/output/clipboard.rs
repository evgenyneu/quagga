extern crate copypasta;

use copypasta::{ClipboardContext, ClipboardProvider};
use std::error::Error;

pub fn output_to_clipboard(content: Vec<String>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut ctx = ClipboardContext::new().map_err(|e| e as Box<dyn Error + Send + Sync>)?;
    let msg = content.join("\n").trim().to_string();
    ctx.set_contents(msg).unwrap();
    println!("Output copied to clipboard.");
    Ok(())
}
