# Quagga

`quagga` is a command-line utility written in Rust that combines multiple text files into a single prompt suitable for Large Language Models (LLMs) like ChatGPT, Claude, or Perplexity. It automates the process of preparing prompts from coding project files, eliminating the need for manual copying and pasting.

<img src='./images/quagga.jpg' alt='Picture of Quagga'>

*The quagga is an extinct subspecies of plains zebra that lived in South Africa until it was hunted to extinction in the late 19th century. This is the only known photograph of a living quagga, taken at the London Zoo in 1870 by Frederick York. Source: [Wikimedia Commons](https://en.wikipedia.org/wiki/Quagga#/media/File:Quagga_photo.jpg).*


## Installation

Download the `quagga` binary from the [releases page](https://github.com/evgenyneu/quagga/releases) and add it to your PATH.

## Usage

```
> quagga --help
Combine text files into a single LLM prompt.

Usage: quagga [OPTIONS] [DIRECTORY]

Arguments:
  [DIRECTORY]  The root directory to search for files [default: .]

Options:
  -i, --include <PATTERN>...        Include only file paths matching the glob patterns (e.g., src/*.js)
  -x, --exclude <PATTERN>...        Ignore file paths that match the glob patterns (e.g., node_modules)
  -C, --contain <TEXT>...           Include only files that contain the specified text
  -b, --modified-before <INTERVAL>  Include only files modified before INTERVAL ago (1m, 1h, 1d, 1w, 1M, 1y)
  -a, --modified-after <INTERVAL>   Include only files modified since INTERVAL ago (1m, 1h, 1d, 1w, 1M, 1y)
  -d, --max-depth <DEPTH>           Descend only DEPTH directories deep
  -f, --max-filesize <BYTES>        Ignore files above the specified size [default: 50000]
  -s, --max-total-size <BYTES>      Show error if total is over the specified size [default: 50000]
  -g, --no-gitignore                Do not use .gitignore files (used by default)
  -B, --binary                      Do not ignore binary files (ignored by default)
  -H, --hidden                      Do not ignore hidden files (ignored by default)
  -l, --follow-links                Follow symbolic links (not followed by default)
  -t, --template <PATH>             Path to a custom template file
  -o, --output <PATH>               Output to a file
  -S, --stdout                      Output to stdout
  -c, --no-clipboard                Do not copy the output to the clipboard (copied by default)
  -D, --dry-run                     Show paths to files without combining them
  -p, --options <PATH>              Load options from a JSON file
  -v, --verbose                     Show detailed information during execution
  -h, --help                        Print help
  -V, --version                     Print version

Examples:

  Include only JavaScript, Typescript and test files, exclude 'node_modules' and 'dist' directories:
  > quagga --include '*.{js,ts}' '*.test.*' --exclude node_modules dist

  Include only files that contain the words 'todo' or 'fixthis', look in '~/code/myapp' dir:
  > quagga --contain todo fixthis -- ~/code/myapp
```


## Development

See [docs/development.md](docs/development.md) for instructions on how to set up the development environment.


## Contributing

See contributing guidelines in [CONTRIBUTING.md](CONTRIBUTING.md).


## ToDo

Brainstorm ideas for future development:

* Add more installation options (pip, brew, etc).
* Add platform-specific instructions on how to add binary to the PATH.
* More usage examples.
* Add option to show file tree.
* Set up continuous integration (CI) to run tests automatically on pushes and pull requests.

## Feedback is welcome

If you need help or notice a bug, feel free to create an issue ticket. I will be happy to help. :D


## The unlicense

This work is in [public domain](LICENSE).
