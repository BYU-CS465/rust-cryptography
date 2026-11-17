//! Helpers for reading input, shared by the RSA example programs.

use std::{error::Error, io};

use rsa::{PublicKey, pkcs8::DecodePublicKey};

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

/// Read a public key in PEM format from standard input.
///
/// Reads lines up to and including the `-----END PUBLIC KEY-----` line.
pub fn read_public_key() -> Result<PublicKey, Box<dyn Error>> {
    println!("Enter a public key in PEM format");
    let mut pem = String::new();
    loop {
        let mut line = String::new();
        // read_line returns 0 at the end of the input
        if io::stdin().read_line(&mut line)? == 0 {
            break;
        }
        pem.push_str(&line);
        if line.starts_with("-----END") {
            break;
        }
    }

    // decode the public key
    let public_key = PublicKey::from_public_key_pem(&pem)
        .map_err(|e| format!("that is not a valid public key: {e}"))?;
    Ok(public_key)
}
