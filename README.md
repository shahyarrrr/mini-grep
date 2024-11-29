# minigrep

A command-line search tool written in Rust, inspired by the classic `grep` command. This project is based on Chapter 12 of "The Rust Programming Language" book with additional features and modifications.

## Description

minigrep is a simple implementation of the grep utility that searches for specific text patterns in files. This version includes custom formatting for matched text (highlighting and underlining) and flexible command-line options.

minigrep is a project from the rust lang book in chapter 12, i am trying to make a bigger one from the minigrep actually should be

## Installation

Make sure you have Rust installed on your system. Then:

```bash
git clone https://github.com/yourusername/minigrep
cd minigrep
cargo build --release
```

## Usage

Basic syntax:
```bash
cargo run -- -q <QUERY> -p <FILE_PATH> [--ignore-case]
```

Arguments:
- `-q`: The text pattern to search for
- `-p`: Path to the file to search in
- `--ignore-case`: Optional flag to perform case-insensitive search

Examples:
```bash
# Search for "hello" in test.txt
cargo run -- -q hello -p test.txt

# Case-insensitive search for "HELLO" in test.txt
cargo run -- -q HELLO -p test.txt --ignore-case
```
