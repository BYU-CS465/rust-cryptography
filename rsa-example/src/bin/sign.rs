use base64ct::LineEnding;
use base64ct::{Base64, Encoding};
use rsa::pkcs8::EncodePublicKey;
use rsa::pss::SigningKey;
use rsa::sha2::Sha256;
use rsa::signature::{RandomizedSigner, SignatureEncoding};
use rsa::{RsaPrivateKey, RsaPublicKey};

use std::io;

fn main() {
    // initialize a randomness generator
    let mut rng = rand::thread_rng();

    // generate a new RSA private key
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    // get public key
    let pub_key = RsaPublicKey::from(&private_key);
    // generate a signing key from the private key
    let signing_key = SigningKey::<Sha256>::new(private_key);

    let pub_pem = pub_key
        .to_public_key_pem(LineEnding::default())
        .expect("failed to convert public key");
    println!("Here is my public key:");
    println!("{}", pub_pem);

    loop {
        // get the text to sign
        println!("Enter something to sign, or nothing to exit");
        let mut data = String::new();
        match io::stdin().read_line(&mut data) {
            Ok(_n) => (),
            Err(error) => {
                println!("error: {error}");
                continue;
            }
        }

        let data = data.trim();

        if data == "" {
            return;
        }
        // sign
        let signature = signing_key.sign_with_rng(&mut rng, data.as_ref());

        // print
        println!("Signature: ");
        println!("{}", Base64::encode_string(&signature.to_bytes()));
    }
}
