use std::env;

use base64::prelude::*;

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // collect the command line arguments
    let args: Vec<String> = env::args().collect();
    // the arguments are:
    // (1) a key
    // (2) a nonce
    // (3) ciphertext
    if args.len() != 4 {
        println!(
            "This program takes three arguments: (1) a key, (2) a nonce, (3) ciphertext. All should be base64 encoded"
        );
        return Ok(());
    }

    // Convert the key from base 64 into a Key object
    let key = BASE64_STANDARD.decode(&args[1])?;
    let key: &[u8] = &key;
    let key: &Key<Aes256Gcm> = key.try_into()?;

    // Convert the nonce from base 64 into a Nonce object
    let nonce = BASE64_STANDARD.decode(&args[2])?;
    let nonce = Nonce::from_slice(&nonce);

    // Convert the Ciphertext from base 64 into an array of u8
    let ciphertext = BASE64_STANDARD.decode(&args[3])?;
    let ciphertext: &[u8] = &ciphertext;

    // initialize the cipher
    // initialize AES GCM with a 256-bit key
    let cipher = Aes256Gcm::new(&key);

    // decrypt the ciphertext
    // note, we need the match here so we can convert an AES GCM error into a standard error.
    let plaintext = match cipher.decrypt(nonce, ciphertext.as_ref()) {
        Ok(result) => String::from_utf8(result)?,
        Err(_e) => "decryption failure".to_string(),
    };

    // print out what we have
    println!("Key: {}", BASE64_STANDARD.encode(&key));
    println!("Nonce: {}", BASE64_STANDARD.encode(&nonce));
    println!("Ciphertext: {}", BASE64_STANDARD.encode(&ciphertext));
    println!("Plaintext: {}", plaintext);

    Ok(())
}
