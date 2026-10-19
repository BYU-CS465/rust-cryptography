use std::error::Error;

use base64::prelude::*;
use rsa::{
    pss::{Signature, VerifyingKey},
    sha2::Sha256,
    signature::Verifier,
};

use rsa_example::{read_line, read_public_key};

fn main() -> Result<(), Box<dyn Error>> {
    // get a public key from standard input
    let public_key = read_public_key()?;

    // derive the PSS verifying key, using SHA-256
    let verifying_key = VerifyingKey::<Sha256>::new(public_key);

    // get the text to verify
    while let Some(data) = read_line("Enter data to verify, or nothing to exit")? {
        // get the signature
        let Some(signature) = read_line("Enter signature")? else {
            eprintln!("No signature entered.");
            continue;
        };

        // convert from base 64
        let Ok(signature) = BASE64_STANDARD.decode(&signature) else {
            eprintln!("The signature must be base64 encoded.");
            continue;
        };
        let Ok(signature) = Signature::try_from(signature.as_slice()) else {
            eprintln!("That is not a valid signature.");
            continue;
        };

        // verify
        match verifying_key.verify(data.as_bytes(), &signature) {
            Ok(()) => println!("Signature is correct!"),
            Err(_) => eprintln!("Could not verify signature!"),
        }
    }

    Ok(())
}
