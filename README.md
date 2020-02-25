# Trotline

This is just a project I'm using to learn concurrency and file IO in Rust; it isn't fit for any real-world use. If you're looking for a genuinely very useful grep alternative, try [ripgrep][ripgrep], a genuinely very impressive and generally faster search tool also written in Rust.

Trotline cretes a new thread for every file being searched then uses regex to identify and print to stdout every line containing the desired pattern. As it stands, the regex and target directory are both hardcoded, but this will be fixed before final release. It also ignores all binary files.

## Usage

In its current state, to use trotline clone this repo then from within it run:
```rust
cargo run --release
```

### Requirements

* The rust toolchain (rustup, cargo)
* A test directory (A directory in the repo named Content that contains files to be searched.)

### Notes
This is only a temporary usecase, the final version will have fill CLI support with arguments and will require a regex pattern to be run.


[ripgrep]: https://github.com/BurntSushi/ripgrep
