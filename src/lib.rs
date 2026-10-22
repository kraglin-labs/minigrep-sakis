
 //! # minigrep
 //!
 //! A simple, case-insensitive literal text search library.
 //!
 //! `minigrep` searches a file line by line, prints matching lines
 //! with line numbers, and highlights matches in red using ANSI
 //! escape codes.
 //!
 //! ## Example
 //!
 //! ```no_run
 //! use minigrep_sakis::greper;
 //!
 //! fn main() -> Result<(), &'static str> {
 //!     greper("rust", "example.txt")?;
 //!     Ok(())
 //! }
 //! ```
 //!
 //! ## Features
 //!
 //! - Case-insensitive literal matching.
 //! - Buffered, line-by-line file reading.
 //! - Line numbers in output.
 //! - Red highlighting for matched text.
 //! - Basic file and I/O error handling.

use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use regex::bytes::Regex;

/// Searches a file for a case-insensitive literal query and prints matching lines.
///
/// Each matching line is printed with its line number. All occurrences of the
/// query are highlighted in red using ANSI escape codes.
///
/// The query is treated as literal text rather than a regular expression.
/// For example, a query containing `.` matches a literal period.
///
/// # Arguments
///
/// * `query` - The text to search for. Matching is case-insensitive.
/// * `path` - The path to the file to search.
///
/// # Returns
///
/// Returns `Ok(())` if the file is read and the output is written successfully.
/// Returns an error message if opening the file, reading it, writing output,
/// compiling the search pattern, or flushing output fails.
///
/// # Errors
///
/// - `"Could not find the file"` if the path does not exist.
/// - `"Check the permissions for the file"` if access is denied.
/// - `"Failed for unknown reasons, contact developer"` for other file-open errors.
/// - `"Invalid regex"` if the search pattern cannot be compiled.
/// - `"Failed to read file"` if reading fails.
/// - `"Failed to write output"` if writing to standard output fails.
/// - `"Failed to flush output"` if flushing standard output fails.
///
/// # Examples
///
/// ```no_run
/// use minigrep_sakis::greper;
///
/// greper("Rust", "example.txt")?;
/// # Ok::<(), &'static str>(())
/// ```
///
/// This example searches `example.txt` for occurrences of `Rust`, regardless
/// of capitalization, and prints matching lines with highlighted matches.
///
/// # Notes
///
/// This function writes directly to standard output. It does not return the
/// matching lines. The caller should ensure that the terminal supports ANSI
/// escape codes if colored highlighting is desired.
pub fn greper(query: &str, path: &str) -> Result<(), &'static str> {
    let file = File::open(path).map_err(|e| match e.kind() {
        io::ErrorKind::NotFound => "Could not find the file",
        io::ErrorKind::PermissionDenied => "Check the permissions for the file",
        _ => "Failed for unknown reasons, contact developer",
    })?;

    let mut reader = BufReader::new(file);
    let mut line = Vec::new();

    let pattern = format!("(?i){}", regex::escape(query));
    let re = Regex::new(&pattern).map_err(|_| "Invalid regex")?;

    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut index = 0;

    loop {
        line.clear();

        let bytes = reader
            .read_until(b'\n', &mut line)
            .map_err(|_| "Failed to read file")?;

        if bytes == 0 {
            break;
        }

        index += 1;

        if line_matches(&re, &line) {
            write!(out, "Line {index}: ")
                .map_err(|_| "Failed to write output")?;

            let highlighted =
                re.replace_all(&line, b"\x1b[31m$0\x1b[0m");

            out.write_all(&highlighted)
                .map_err(|_| "Failed to write output")?;
        }
    }

    out.flush().map_err(|_| "Failed to flush output")?;

    Ok(())
}

fn line_matches(re: &Regex, line: &[u8]) -> bool {
    re.is_match(line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_matching() {
        let re = Regex::new(r"(?i)rust").unwrap();
        assert!(line_matches(&re, b"i like rust"));
        assert!(line_matches(&re, b"J'aime le Rust"));
        assert!(line_matches(&re, b"le rUst est tres rapide"));
        assert!(!line_matches(&re, b"Go is fast"));
    }

    #[test]
    fn test_query() {
        let query = "$5,00";
        let pattern = format!("(?i){}", regex::escape(query));
        let re = Regex::new(&pattern).unwrap();

        assert!(line_matches(&re, b"The pizza costs $5,00"));
        assert!(!line_matches(&re, b"The price is 5x00"));
    }
}
