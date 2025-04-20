use clap::Parser;
use std::path::PathBuf;

/// Combine multiple text files into a single prompt for a Large Language Model (LLM).
#[derive(Parser, Debug, PartialEq)]
#[command(
    name = "quagga",
    author = "Evgenii Neumerzhitckii <sausageskin@gmail.com>",
    version = env!("CARGO_PKG_VERSION"),
    about = "Combine text files into a single LLM prompt.",
    after_help = "\x1b[1mExamples\x1b[0m:\n  \
    Combine all Markdown files and copy the result to clipboard instead of stdout:\n  \
    >\x1b[1m quagga --include '*.md' --clipboard \x1b[0m \n\n  \
    Save output to a file instead of stdout:\n  \
    >\x1b[1m quagga --output {TIME}_prompt.txt \x1b[0m \n\n  \
    Include only JavaScript, Typescript, and test files, excluding 'node_modules' and 'dist' directories:\n  \
    >\x1b[1m quagga --include '*.{js,ts}' '*.test.*' --exclude node_modules dist \x1b[0m \n\n  \
    Use a template to customize the prompt text:\n  \
    >\x1b[1m quagga --template prompt.md --include '*.txt' \x1b[0m \n\n  \
    Include only files that contain the words 'todo' or 'fixthis', look in '~/code/myapp' directory:\n  \
    >\x1b[1m quagga --contain todo fixthis -- ~/code/myapp \x1b[0m \n\n  \
    Pipe file paths from another program:\n  \
    >\x1b[1m find . -name '*.txt' | quagga \x1b[0m \n\n  \
    Use a list of files from a text file:\n  \
    >\x1b[1m cat file_list.txt | quagga \x1b[0m"
)]
pub struct Cli {
    /// Include only file paths matching the glob patterns (e.g., src/*.js)
    #[arg(short = 'i', long, value_name = "PATTERN", num_args(1..))]
    pub include: Vec<String>,

    /// Exclude file paths that match the glob patterns (e.g., node_modules)
    #[arg(short = 'x', long, value_name = "PATTERN", num_args(1..))]
    pub exclude: Vec<String>,

    /// Include only files that contain the specified text
    #[arg(short = 'C', long, value_name = "TEXT", num_args(1..))]
    pub contain: Vec<String>,

    /// Descend only DEPTH directories deep
    #[arg(short = 'd', long, value_name = "DEPTH")]
    pub max_depth: Option<usize>,

    /// Output is split into parts of this size if it exceeds this limit
    #[arg(short = 'p', long, value_name = "CHARS", default_value_t = 100_000)]
    pub max_part_size: u64,

    /// Ignore files above the specified size
    #[arg(short = 'f', long, value_name = "BYTES", default_value_t = 300 * 1024)]
    pub max_filesize: u64,

    /// Show error if total size of files is over the specified size
    #[arg(short = 's', long, value_name = "BYTES", default_value_t = 500*1024)]
    pub max_total_size: u64,

    /// Don't use .gitignore files (used by default)
    #[arg(short = 'g', long)]
    pub no_gitignore: bool,

    /// Don't use .quagga_ignore from project and home dirs (used by default)
    #[arg(short = 'I', long)]
    pub no_quagga_ignore: bool,

    /// Include binary files (ignored by default)
    #[arg(short = 'B', long)]
    pub binary: bool,

    /// Include hidden files (ignored by default)
    #[arg(short = 'H', long)]
    pub hidden: bool,

    /// Follow symbolic links (not followed by default)
    #[arg(short = 'l', long)]
    pub follow_links: bool,

    /// Path to a custom template file
    #[arg(short = 't', long, value_name = "PATH")]
    pub template: Option<PathBuf>,

    /// Copy default template to .quagga_template in the current directory
    #[arg(short = 'm', long)]
    pub copy_template: bool,

    /// Don't use .quagga_template from project and home dirs (used by default)
    #[arg(short = 'n', long)]
    pub no_quagga_template: bool,

    /// Output to a file instead of stdout
    #[arg(short = 'o', long, value_name = "PATH")]
    pub output: Option<PathBuf>,

    /// Copy the output to the clipboard instead of stdout
    #[arg(short = 'c', long)]
    pub clipboard: bool,

    /// Show paths to files without combining them
    #[arg(short = 'a', long)]
    pub paths: bool,

    /// Show sizes of files without combining them
    #[arg(short = 'Z', long)]
    pub file_sizes: bool,

    /// Show paths to files in ASCII tree format without combining them
    #[arg(short = 'T', long)]
    pub tree: bool,

    /// Show total size of files without combining them
    #[arg(short = 'z', long)]
    pub size: bool,

    /// Exclude comment lines from the output
    #[arg(long = "no-comments")]
    pub no_comments: bool,

    /// The root directory to search for files
    #[arg(value_name = "DIRECTORY", default_value = ".")]
    pub root: PathBuf,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_default_values() {
        let args = Cli::parse_from(&["quagga"]);
        assert_eq!(
            args,
            Cli {
                include: Vec::new(),
                exclude: Vec::new(),
                contain: Vec::new(),
                max_depth: None,
                no_gitignore: false,
                no_quagga_ignore: false,
                binary: false,
                hidden: false,
                follow_links: false,
                template: None,
                copy_template: false,
                no_quagga_template: false,
                output: None,
                clipboard: false,
                paths: false,
                file_sizes: false,
                tree: false,
                max_part_size: 100000,
                max_filesize: 300 * 1024,
                max_total_size: 500 * 1024,
                root: PathBuf::from("."),
                size: false,
                no_comments: false,
            }
        );
    }

    #[test]
    fn test_multiple_include_exclude() {
        let cmd = "quagga --include *.js *.rs --exclude node_modules dist";
        let args = Cli::parse_from(cmd.split_whitespace());

        assert_eq!(args.include, vec!["*.js", "*.rs"]);
        assert_eq!(args.exclude, vec!["node_modules", "dist"]);
        assert_eq!(args.root, PathBuf::from("."));
    }

    #[test]
    fn test_single_include_exclude() {
        let cmd = "quagga --include *.js --exclude node_modules";
        let args = Cli::parse_from(cmd.split_whitespace());
        assert_eq!(args.include, vec!["*.js"]);
        assert_eq!(args.exclude, vec!["node_modules"]);
    }

    #[test]
    fn test_contain() {
        let args = Cli::parse_from(vec!["quagga", "--contain", "hello world", "hi"].iter());

        assert_eq!(args.contain, vec!("hello world", "hi"));
        assert_eq!(args.root, PathBuf::from("."));
    }

    #[test]
    fn test_all_options() {
        let cmd = "quagga \
          --include *.js \
          --exclude node_modules \
          --contain hello \
          --max-depth 2 \
          --no-gitignore \
          --no-quagga-ignore \
          --binary \
          --hidden \
          --follow-links \
          --template template.txt \
          --copy-template \
          --no-quagga-template \
          --output output.txt \
          --clipboard \
          --paths \
          --tree \
          --size \
          --file-sizes \
          --max-part-size 300 \
          --max-filesize 10000 \
          --max-total-size 20000 \
          --no-comments \
          src";

        let args = Cli::parse_from(cmd.split_whitespace());

        assert_eq!(
            args,
            Cli {
                include: vec!["*.js".to_string()],
                exclude: vec!["node_modules".to_string()],
                contain: vec!("hello".to_string()),
                max_depth: Some(2),
                no_gitignore: true,
                no_quagga_ignore: true,
                binary: true,
                hidden: true,
                follow_links: true,
                template: Some(PathBuf::from("template.txt")),
                copy_template: true,
                no_quagga_template: true,
                output: Some(PathBuf::from("output.txt")),
                clipboard: true,
                paths: true,
                tree: true,
                max_part_size: 300,
                max_filesize: 10000,
                max_total_size: 20000,
                root: PathBuf::from("src"),
                size: true,
                file_sizes: true,
                no_comments: true,
            }
        );
    }
}
