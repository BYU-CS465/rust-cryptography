use std::env;

use base64::prelude::*;

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm,
};

fn main() -> Result<(), aes_gcm::Error> {
    // collect the command line arguments
    let args: Vec<String> = env::args().collect();
    // the first argument is plaintext to encrypt
    if args.len() != 2 {
        println!(
            "This program takes only one argument, which should be some plaintext to encrypt."
        );
        return Ok(());
    }
    let plaintext = &args[1];

    // Randomly generate a 256-bit encryption key
    let key = Aes256Gcm::generate_key(OsRng);
    // initialize AES GCM with a 256-bit key
    let cipher = Aes256Gcm::new(&key);
    // generate a nonce / IV -- must be unique per message
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    // encrypt the plaintext
    let ciphertext = cipher.encrypt(&nonce, plaintext.as_ref())?;

    // print out what we have
    println!("Key: {}", BASE64_STANDARD.encode(&key));
    println!("Nonce: {}", BASE64_STANDARD.encode(&nonce));
    println!("Plaintext: {}", plaintext);
    println!("Ciphertext: {}", BASE64_STANDARD.encode(&ciphertext));

    Ok(())
}
