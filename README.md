# minigrep-sakis

A lightweight Rust text-search library for finding text in files, with case-insensitive matching, line numbers, and ANSI-colored highlighting.

## Features

- **Case-insensitive search** — matches `rust`, `Rust`, and `RUST`.
- **Literal queries** — special characters such as `.`, `*`, and `$` are treated as ordinary text.
- **Buffered file reading** — processes files line by line without loading the entire file into memory.
- **Line numbers** — displays the line number for each matching line.
- **Colored highlighting** — highlights matching text in red using ANSI escape codes.
- **Error handling** — reports file access, reading, and output errors.

## Installation

Add `minigrep-sakis` to your project's `Cargo.toml`:

```toml
[dependencies]
minigrep-sakis = "0.1.4"
```

Or run:

```bash
cargo add minigrep-sakis
```

## Usage

Import `greper` and provide a search query and a file path:

```rust
use minigrep_sakis::greper;

fn main() {
    if let Err(error) = greper("rust", "example.txt") {
        eprintln!("Search failed: {error}");
    }
}
```

Suppose `example.txt` contains:

```text
I am learning Rust.
Rust is fast.
I also enjoy Go.
```

The matching output will look like this in a terminal that supports ANSI colors:

```text
Line 1: I am learning Rust.
Line 2: Rust is fast.
```

The occurrences of `Rust` are highlighted in red.

## Error handling

`greper` returns `Result<(), &'static str>`.

It returns `Ok(())` when the file has been processed and output has been written successfully. It returns an error message if opening or reading the file, writing output, or flushing output fails.

A successful result does not necessarily mean a match was found. The function prints matching lines directly to standard output rather than returning them to the caller.

## How it works

1. Opens the specified file.
2. Compiles a case-insensitive, escaped search pattern.
3. Reads the file incrementally using `BufReader`.
4. Checks each line for matches.
5. Prints matching lines with line numbers and red highlighting.
6. Flushes buffered output before returning.

## Limitations

- Search results are printed directly to standard output.
- Highlighting uses ANSI escape codes, which may not render as colors in every terminal.
- The current API does not return matching lines or a count of matches.
- Case-insensitive matching is implemented using the `regex` crate's byte-oriented regex engine.

## Development

Run the tests:

```bash
cargo test
```

Build the API documentation:

```bash
cargo doc --open
```

Check the package before publishing:

```bash
cargo publish --dry-run
```

## Links

- **Source code:** https://github.com/kraglin-labs/minigrep-sakis
- **Crates.io:** https://crates.io/crates/minigrep-sakis
- **API documentation:** https://docs.rs/minigrep-sakis

## License

Licensed under the MIT License.
