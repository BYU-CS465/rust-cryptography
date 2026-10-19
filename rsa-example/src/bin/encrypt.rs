use std::error::Error;

use base64::prelude::*;
use rsa::{Oaep, sha2::Sha256};

use rsa_example::{read_line, read_public_key};

fn main() -> Result<(), Box<dyn Error>> {
    // set up a random number generator
    let mut rng = rand::rng();

    // get a public key from standard input
    let public_key = read_public_key()?;

    // get the text to encrypt
    while let Some(plaintext) = read_line("Enter something to encrypt, or nothing to exit")? {
        // encrypt the plaintext using OAEP padding with SHA-256
        // (OAEP is recommended over the older PKCS#1 v1.5 padding).
        // RSA can only encrypt short messages, so this fails if the message is too long.
        let Ok(ciphertext) =
            public_key.encrypt(&mut rng, Oaep::<Sha256>::new(), plaintext.as_bytes())
        else {
            eprintln!("Encryption failed: the message is too long.");
            continue;
        };

        // print out what we have
        println!("Here is the ciphertext:");
        println!("{}", BASE64_STANDARD.encode(&ciphertext));
    }

    Ok(())
}
