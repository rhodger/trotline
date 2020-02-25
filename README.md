# Trotline

This is just a project I'm using to learn concurrency and file IO in Rust; it isn't fit for any real-world use. If you're looking for a genuinely very useful grep alternative, try [ripgrep][ripgrep], a genuinely very impressive and generally faster search tool also written in Rust.

Trotline cretes a new thread for every file being searched then uses regex to identify and print to stdout every line containing the desired pattern. As it stands, the regex and target directory are both hardcoded, but this will be fixed before final release. It also ignores all binary files.

## Installation

To install, ensure you have the requirements listed below, then run:
```bash
cargo install --git "https://github.com/SuedeGently/trotline.git"
```
then `trotline --version` to check its installed correctly.

### Requirements

* The rust toolchain (rustup, cargo)
* A test directory (A directory in the repo named Content that contains files to be searched.)

## Usage

```
    trotline <pattern> [directory]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <pattern>      regex search pattern
    <directory>    target directory
```

Where `pattern` can be any valid regex string. If no directory is specified, the current working directory will be used instead.


[ripgrep]: https://github.com/BurntSushi/ripgrep
