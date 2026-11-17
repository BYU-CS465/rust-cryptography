use std::error::Error;

use base64::prelude::*;
use rsa::{
    PrivateKey,
    pkcs8::{EncodePublicKey, LineEnding},
    pss::SigningKey,
    sha2::Sha256,
    signature::{Keypair, RandomizedSigner, SignatureEncoding},
};

use rsa_example::read_line;

fn main() -> Result<(), Box<dyn Error>> {
    // initialize a randomness generator
    let mut rng = rand::rng();

    // generate a new RSA private key
    let bits = 2048;
    let private_key = PrivateKey::new(&mut rng, bits)?;
    // generate a PSS signing key from the private key, using SHA-256
    let signing_key = SigningKey::<Sha256>::new(private_key);
    // get the matching verifying (public) key
    let verifying_key = signing_key.verifying_key();

    // print out the public key
    println!("Here is my public key:");
    println!(
        "{}",
        verifying_key.to_public_key_pem(LineEnding::default())?
    );

    // get the text to sign
    while let Some(data) = read_line("Enter something to sign, or nothing to exit")? {
        // sign
        let signature = signing_key.sign_with_rng(&mut rng, data.as_bytes());

        // print
        println!("Signature:");
        println!("{}", BASE64_STANDARD.encode(signature.to_bytes()));
    }

    Ok(())
}
