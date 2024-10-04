use base64ct::LineEnding;
use base64ct::{Base64, Encoding};
use rsa::pkcs8::EncodePublicKey;
use rsa::Pkcs1v15Encrypt;
use rsa::RsaPrivateKey;
use rsa::RsaPublicKey;
use std::io;

fn main() {
    // get a random number generator
    let mut rng = rand::thread_rng();
    // generate an RSA key pair
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);

    // print out the public key
    let pub_pem = pub_key
        .to_public_key_pem(LineEnding::default())
        .expect("failed to convert public key");
    println!("Here is my public key:");
    println!("{}", pub_pem);

    loop {
        // wait for input of encrypted text
        println!("Enter something to decrypt, or nothing to exit");
        let mut ciphertext = String::new();
        match io::stdin().read_line(&mut ciphertext) {
            Ok(_n) => (),
            Err(error) => {
                println!("error: {error}");
                continue;
            }
        }

        let ciphertext = ciphertext.trim();

        if ciphertext == "" {
            return;
        }

        // convert from BASE64
        let ciphertext = Base64::decode_vec(&ciphertext).unwrap();

        // decrypt the ciphertext
        let plaintext = priv_key
            .decrypt(Pkcs1v15Encrypt, ciphertext.as_ref())
            .expect("failed to decrypt");

        let plaintext =
            String::from_utf8(plaintext).expect("Could not convert plaintext to a string");

        // print out what we have
        println!("Plaintext: {}", plaintext);
    }
}
