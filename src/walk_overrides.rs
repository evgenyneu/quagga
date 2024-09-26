use crate::cli::Cli;
use ignore::{overrides::Override, overrides::OverrideBuilder};
use std::error::Error;

pub fn build_overrides(cli: &Cli) -> Result<Override, Box<dyn Error>> {
    let override_builder = OverrideBuilder::new(&cli.root);
    let overrides = override_builder.build().unwrap();
    Ok(overrides)
}
