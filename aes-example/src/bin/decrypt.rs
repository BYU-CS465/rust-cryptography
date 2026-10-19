use std::{env, error::Error, process};

use base64::prelude::*;

// Note: we use `aead::Nonce<Aes256Gcm>` rather than `aes_gcm::Nonce`. The latter
// is parameterized by the nonce *size* (e.g. `aes_gcm::Nonce<U12>`), so naming the
// cipher, as in `Nonce::<Aes256Gcm>`, only works with the `aead` version.
use aes_gcm::{
    Aes256Gcm,
    aead::{Aead, Key, KeyInit, Nonce},
};

fn main() -> Result<(), Box<dyn Error>> {
    // collect the command line arguments
    let args: Vec<String> = env::args().collect();
    // the arguments are:
    // (1) a key
    // (2) a nonce
    // (3) ciphertext
    if args.len() != 4 {
        eprintln!(
            "This program takes three arguments: (1) a key, (2) a nonce, (3) ciphertext. All should be base64 encoded."
        );
        process::exit(1);
    }

    // Convert the key from base 64 into a Key object
    let key_bytes = BASE64_STANDARD.decode(&args[1])?;
    let key =
        Key::<Aes256Gcm>::try_from(key_bytes.as_slice()).map_err(|_| "the key must be 256 bits")?;

    // Convert the nonce from base 64 into a Nonce object
    let nonce_bytes = BASE64_STANDARD.decode(&args[2])?;
    let nonce = Nonce::<Aes256Gcm>::try_from(nonce_bytes.as_slice())
        .map_err(|_| "the nonce must be 96 bits")?;

    // Convert the ciphertext from base 64 into a vector of bytes
    let ciphertext = BASE64_STANDARD.decode(&args[3])?;

    // initialize AES GCM with a 256-bit key
    let cipher = Aes256Gcm::new(&key);

    // decrypt the ciphertext
    // this fails if the key, nonce, or ciphertext is wrong or the ciphertext was tampered with
    let Ok(plaintext) = cipher.decrypt(&nonce, ciphertext.as_slice()) else {
        eprintln!("Decryption failed: the ciphertext could not be authenticated.");
        process::exit(1);
    };
    let plaintext = String::from_utf8(plaintext)?;

    // print out what we have
    println!("Key: {}", BASE64_STANDARD.encode(key));
    println!("Nonce: {}", BASE64_STANDARD.encode(nonce));
    println!("Ciphertext: {}", BASE64_STANDARD.encode(&ciphertext));
    println!("Plaintext: {plaintext}");

    Ok(())
}
