use std::error::Error;

use base64::prelude::*;
use rsa::{
    Oaep, RsaPrivateKey, RsaPublicKey,
    pkcs8::{EncodePublicKey, LineEnding},
    sha2::Sha256,
};

use rsa_example::read_line;

fn main() -> Result<(), Box<dyn Error>> {
    // get a random number generator
    let mut rng = rand::rng();
    // generate an RSA key pair
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits)?;
    let public_key = RsaPublicKey::from(&private_key);

    // print out the public key
    println!("Here is my public key:");
    println!("{}", public_key.to_public_key_pem(LineEnding::default())?);

    // wait for input of encrypted text
    while let Some(ciphertext) = read_line("Enter something to decrypt, or nothing to exit")? {
        // convert from base 64
        let Ok(ciphertext) = BASE64_STANDARD.decode(&ciphertext) else {
            eprintln!("The ciphertext must be base64 encoded.");
            continue;
        };

        // decrypt the ciphertext using OAEP padding with SHA-256
        let Ok(plaintext) = private_key.decrypt(Oaep::<Sha256>::new(), &ciphertext) else {
            eprintln!("Decryption failed.");
            continue;
        };

        // print out what we have
        match String::from_utf8(plaintext) {
            Ok(plaintext) => println!("Plaintext: {plaintext}"),
            Err(_) => eprintln!("The plaintext is not valid UTF-8 text."),
        }
    }

    Ok(())
}
