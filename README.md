# ClipCat

A fast and simple CLI tool to copy multiple files with a single command.

## Why this tool?

The reason for this tool is to make it easier to copy a large amount of your source code or text files to the clipboard. This can be useful when you want to give your LLM some context on what you are asking about.

## Usage

Give it a directory and it will copy all the contents of the files in that directory and subdirectories that have a valid UTF-8 encoding to the clipboard.

```bash
clipcat path/to/directory
```

## Installation

You can either download a pre-built binary or install the tool using Cargo.

### Install using Cargo

If you have Rust installed, you can build the tool from source using Cargo.

```bash
cargo install clipcat
```

### Pre-built binaries

You can visit the [releases page](https://github.com/dcodesdev/clipcat/releases) to download the latest binary for your platform.

## Building from source

If you want to build the tool from source, you can do so using Cargo.

```bash
git clone https://github.com/dcodesdev/clipcat
cd clipcat
cargo build --release
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
