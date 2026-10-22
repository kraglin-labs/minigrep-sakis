use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use regex::bytes::Regex;

pub fn greper(query: &str, path: &str) -> Result<(), &'static str> {
    let file = File::open(path).map_err(|e| match e.kind() {
        io::ErrorKind::NotFound => "Could not find the file",
        io::ErrorKind::PermissionDenied => "Check the permissions for the file",
        _ => "Failed for unknown reasons, contact developer",
    })?;

    let mut reader = BufReader::new(file);
    let mut line = Vec::new();

	// let re = Regex::new(&regex::escape(query)).unwrap();
	let pattern = format!("(?i){}", regex::escape(query));
 	let re = Regex::new(&pattern).map_err(|_| "Invalid 	regex")?;
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
