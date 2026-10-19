use std::{env, error::Error, process};

use base64::prelude::*;

// Note: we use `aead::Nonce<Aes256Gcm>` rather than `aes_gcm::Nonce`. The latter
// is parameterized by the nonce *size* (e.g. `aes_gcm::Nonce<U12>`), so naming the
// cipher, as in `Nonce::<Aes256Gcm>`, only works with the `aead` version.
use aes_gcm::{
    Aes256Gcm,
    aead::{Aead, Generate, Key, KeyInit, Nonce},
};

fn main() -> Result<(), Box<dyn Error>> {
    // collect the command line arguments
    let args: Vec<String> = env::args().collect();
    // the first argument is plaintext to encrypt
    if args.len() != 2 {
        eprintln!(
            "This program takes only one argument, which should be some plaintext to encrypt."
        );
        process::exit(1);
    }
    let plaintext = &args[1];

    // Randomly generate a 256-bit encryption key
    let key = Key::<Aes256Gcm>::generate();
    // initialize AES GCM with a 256-bit key
    let cipher = Aes256Gcm::new(&key);
    // generate a random nonce / IV -- MUST be unique per message
    let nonce = Nonce::<Aes256Gcm>::generate();
    // encrypt the plaintext
    let ciphertext = cipher.encrypt(&nonce, plaintext.as_bytes())?;

    // print out what we have
    println!("Key: {}", BASE64_STANDARD.encode(key));
    println!("Nonce: {}", BASE64_STANDARD.encode(nonce));
    println!("Plaintext: {plaintext}");
    println!("Ciphertext: {}", BASE64_STANDARD.encode(&ciphertext));

    Ok(())
}
