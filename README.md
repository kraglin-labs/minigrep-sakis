
# minigrep

A simple case-insensitive text search library written in Rust.

## Features

- Case-insensitive literal search
- Line-by-line buffered file reading
- ANSI-colored match highlighting
- Error handling for file operations

## Usage

Add the crate to your Cargo.toml:

```toml
[dependencies]
minigrep_sakis = "0.1.4"
```

Then call the library:

```rust
fn main() {
    if let Err(error) = minigrep::greper("rust", "example.txt") {
        eprintln!("{error}");
    }
}
```
