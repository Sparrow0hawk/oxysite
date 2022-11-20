[![Rust checks](https://github.com/Sparrow0hawk/oxysite/actions/workflows/rust-check.yml/badge.svg?branch=main)](https://github.com/Sparrow0hawk/oxysite/actions/workflows/rust-check.yml)
# Oxysites

A static site generator in Rust.

## Installation

This library is not available on [crates.io](https://crates.io/) and can only be installed directly from source using [cargo](https://crates.io/crates/cargo).

### `cargo install` directly

Install this library directly as a compiled executable in your `$HOME/.cargo/bin`.

```bash
$ git clone https://github.com/Sparrow0hawk/oxysite.git

$ cargo install --path .

$ oxysite --help
Usage: oxysite <COMMAND>

Commands:
  build  Build site
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```

### Build and run within the repository

If you'd rather not build the executable and add it to your path, you can build and run `oxysite` within the repository using `cargo`.

```bash
$ git clone https://github.com/Sparrow0hawk/oxysite.git

# build as unoptimised
$ cargo build

# to build the test example
$ cargo run -- build -c test_content -p magic
```

## Usage

You can use this package to build `.html` files from `.md`. 
Nothing particularly fancy out of the box but you would use it like this:

```bash
$ oxysite build -c markdown_files -p html_output
```

This command will build a HTML file for every markdown file in the `markdown_files` directory and write them into `html_output` directory (creating such a directory if it doesn't exist).

