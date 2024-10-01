# Quagga

`quagga` is a command-line utility that combines multiple text files into a single prompt suitable for Large Language Models (LLMs) like ChatGPT. By eliminating the need to manually locate and copy/paste content, quagga streamlines prompt preparation for developers:

```bash
> quagga --include '*.js' 'README.md' --exclude 'node_modules'
```

The main focus of `quagga` is **speed**, thanks to its implementation in Rust, and [**useful defaults**](#defaults), such as respecting `.gitignore`, ignoring binary, and hidden files. It follows the Unix philosophy of doing one thing well and is designed to be used with other tools by sending the prompt to stdout and receiving file paths from stdin:

```bash
> quagga > prompt.txt
> find . -name '*.txt' | quagga
```

## Installation

Download the `quagga` binary from the [releases page](https://github.com/evgenyneu/quagga/releases) and add it to your PATH.

## Usage

### Basic command

```bash
quagga [OPTIONS] [DIRECTORY]
```

*DIRECTORY*: The root directory to search for files. Default is current directory `.`.

### Examples


#### Combine markdown files and copy to clipboard

```bash
quagga --include '*.md' --clipboard
```

Combines all Markdown files in the current directory and copies the result to the clipboard.


#### Include specific file types and exclude directories

```bash
quagga --include '*.{js,ts}' '*.test.*' --exclude node_modules dist
```

Includes JavaScript, TypeScript, and test files while excluding `node_modules` and `dist` directories.


#### Use a custom template

```bash
quagga --template prompt.json --include '*.txt'
```

Uses a template to customize the prompt text (see [Templates section](#templates) for details).

#### Include only files that contain specific text

```bash
quagga --contain todo fixthis -- ~/code/myapp
```

Includes only files that contain the words 'todo' or 'fixthis', look in the `~/code/myapp` directory. Notice the use of `--` to separate options from the directory path.


#### Pipe file paths from another program

```bash
find . -name '*.txt' | quagga
cat file_list.txt | quagga
```

Pipes file paths from another program or a text file into quagga instead of searching the directory.

#### Get the full list of options

```bash
quagga --help
```

## Templates

Quagga uses templates to format the combined output of your files. Templates allow you to define how the output is structured, including headers, footers, placeholders for file content, as well as providing instructions for an LLM.  By default, it applies a [built-in template](templates/default.txt), but you can customize this to suit your needs.

### Create a custom template

Use the `--copy-template` option to generate a default template file in the current directory:

```bash
quagga --copy-template
```

This command copies the default template to `.quagga_template` file that you can modify to customize the output format.

### Template locations

When Quagga runs, it looks for a template in the following order:

1. A custom template file specified with the `--template <PATH>` option.
1. A `.quagga_template` file in the current directory.
1. A `.quagga_template` file in your home directory.
1. If none of the above are found, Quagga uses its built-in default template.

You can disable `.quagga_template` locations by using the `--no-quagga-template` option.


## Defaults

Quagga uses the following defaults that can be overridden with command-line options:

* Respects gitignore files (disable with `--no-gitignore`):
  * Standard: `.ignore`, `.gitignore`, `.git/info/exclude`.
  * Gitignore files from parent directories are respected.
  * Global ignore file from `core.excludesFile` option in `$HOME/.gitconfig` file. If not set, then `$XDG_CONFIG_HOME/git/ignore` is used. If `$XDG_CONFIG_HOME` is not set, then `$HOME/.config/git/ignore` is used.
  * Custom gitignore files can be set with the `--gitignore-file PATH` option.
* Uses `.quagga_ignore` files from the project and home directories written in the same format as gitignore (disable with `--no-quagga-ignore`).
* Ignores binary files (enable with `--binary`).
* Ignores hidden files (enable with `--hidden`).
* Ignores files larger than 50 KB (change with `--max-filesize BYTES`).
* Symbolic links are not followed (enable with `--follow-links`).

## What's Quagga?

<img src='./images/quagga.jpg' alt='Picture of Quagga'>

*The quagga is an extinct subspecies of plains zebra that lived in South Africa until it was hunted to extinction in the late 19th century. This is the only known photograph of a living quagga, taken at the London Zoo in 1870 by Frederick York. Source: [Wikimedia Commons](https://en.wikipedia.org/wiki/Quagga#/media/File:Quagga_photo.jpg).*

## Development

See [docs/development.md](docs/development.md) for instructions on how to set up the development environment.


## Contributing

See contributing guidelines in [CONTRIBUTING.md](CONTRIBUTING.md).


## Alternative solutions

Here are some great programs from other developers that offer similar functionality:

* [simonw/files-to-prompt](https://github.com/simonw/files-to-prompt)
* [mufeedvh/code2prompt](https://github.com/mufeedvh/code2prompt)
* [banagale/FileKitty](https://github.com/banagale/FileKitty)


## Feedback is welcome

If you need help or notice a bug, feel free to create an issue ticket. We will be happy to help. :D


## License

Quagga is released under the [MIT License](LICENSE).
