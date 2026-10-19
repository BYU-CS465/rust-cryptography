//! A helper for reading input, shared by the ECDSA example programs.

use std::io;

/// Print a prompt and read one line from standard input.
///
/// Returns `None` if the line is empty or we reached the end of the input.
pub fn read_line(prompt: &str) -> io::Result<Option<String>> {
    println!("{prompt}");
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let line = line.trim();
    if line.is_empty() {
        Ok(None)
    } else {
        Ok(Some(line.to_string()))
    }
}
