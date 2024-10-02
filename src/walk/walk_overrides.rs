use crate::cli::Cli;
use ignore::{overrides::Override, overrides::OverrideBuilder};
use std::error::Error;

/// Builds an `Override` object based on the command-line include and exclude patterns.
///
/// # Arguments
///
/// * `cli` - A reference to the parsed command-line arguments.
///
/// # Returns
///
/// * `Ok(Override)` - The constructed `Override` object.
/// * `Err(Box<dyn Error>)` - If there was an error building the overrides.
///
/// # Errors
///
/// This function returns an error if any of the patterns provided are invalid.
pub fn build_overrides(cli: &Cli) -> Result<Override, Box<dyn Error>> {
    let mut builder = OverrideBuilder::new(&cli.root);
    add_include_patterns(&mut builder, &cli.include)?;
    add_exclude_patterns(&mut builder, &cli.exclude)?;
    let overrides = builder.build()?;
    Ok(overrides)
}

/// Adds include patterns to the `OverrideBuilder`.
///
/// # Arguments
///
/// * `builder` - The `OverrideBuilder` to which the patterns will be added.
/// * `includes` - A slice of include pattern strings.
///
/// # Returns
///
/// * `Ok(())` if all patterns were added successfully.
/// * `Err(Box<dyn Error>)` if any pattern is invalid.
fn add_include_patterns(
    builder: &mut OverrideBuilder,
    includes: &[String],
) -> Result<(), Box<dyn Error>> {
    for pattern in includes {
        builder.add(pattern)?;
    }
    Ok(())
}

/// Adds exclude patterns to the `OverrideBuilder`.
///
/// # Arguments
///
/// * `builder` - The `OverrideBuilder` to which the patterns will be added.
/// * `excludes` - A slice of exclude pattern strings.
///
/// # Returns
///
/// * `Ok(())` if all patterns were added successfully.
/// * `Err(Box<dyn Error>)` if any pattern is invalid.
fn add_exclude_patterns(
    builder: &mut OverrideBuilder,
    excludes: &[String],
) -> Result<(), Box<dyn Error>> {
    for pattern in excludes {
        // Prefix with '!' to negate the pattern
        let negated_pattern = format!("!{}", pattern);
        builder.add(&negated_pattern)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::Cli;
    use clap::Parser;
    use std::path::PathBuf;

    #[test]
    fn test_build_overrides_with_include_and_exclude() {
        let mut cli = Cli::parse_from(&["test"]);
        cli.include = vec!["*.rs".to_string()];
        cli.exclude = vec!["tests/*".to_string()];
        cli.root = PathBuf::from(".");

        let overrides = build_overrides(&cli).unwrap();

        assert!(overrides.matched("src/main.rs", false).is_whitelist());

        assert!(overrides
            .matched("tests/integration_test.rs", false)
            .is_ignore());
    }

    #[test]
    fn test_add_include_patterns() {
        let mut builder = OverrideBuilder::new(".");
        let includes = vec!["*.md".to_string(), "*.txt".to_string()];

        add_include_patterns(&mut builder, &includes).unwrap();

        let overrides = builder.build().unwrap();
        assert!(overrides.matched("README.md", false).is_whitelist());
        assert!(overrides.matched("notes.txt", false).is_whitelist());
        assert!(overrides.matched("main.rs", false).is_ignore());
    }

    #[test]
    fn test_add_exclude_patterns() {
        let mut builder = OverrideBuilder::new(".");
        let excludes = vec!["node_modules/*".to_string(), "target/*".to_string()];

        add_exclude_patterns(&mut builder, &excludes).unwrap();

        let overrides = builder.build().unwrap();

        assert!(overrides
            .matched("node_modules/package.json", false)
            .is_ignore());

        assert!(overrides.matched("target/app", false).is_ignore());
    }

    #[test]
    fn test_invalid_pattern() {
        let mut cli = Cli::parse_from(&["test"]);
        cli.include = vec!["**/*".to_string()];
        cli.exclude = vec!["[".to_string()]; // Invalid pattern

        let result = build_overrides(&cli);
        assert!(result.is_err());
    }
}
