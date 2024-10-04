use base64ct::{Base64, Encoding};
use rand;
use rsa::pkcs8::DecodePublicKey;
use rsa::{Pkcs1v15Encrypt, RsaPublicKey};
use std::io;

fn main() {
    // set up a random number generator
    let mut rng = rand::thread_rng();

    // get a public key from standard input
    println!("Enter a public key in PEM format");
    let mut pem_file = String::new();
    loop {
        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Ok(_n) => (),
            Err(_error) => {
                panic!("Could not read input");
            }
        };
        if buffer == "\n" {
            break;
        }

        pem_file.push_str(&buffer);
    }

    // decode the public key
    println!("key: {}", pem_file);
    let pub_key = match RsaPublicKey::from_public_key_pem(&pem_file) {
        Ok(key) => key,
        Err(error) => panic!("Could not convert public key from PEM {}", error),
    };

    loop {
        // get the text to encrypt
        println!("Enter something to encrypt, or nothing to exit");
        let mut plaintext = String::new();
        match io::stdin().read_line(&mut plaintext) {
            Ok(_n) => (),
            Err(error) => {
                println!("error: {error}");
                continue;
            }
        }

        let plaintext = plaintext.trim();

        if plaintext == "" {
            return;
        }

        // encrypt the plaintext
        let ciphertext = pub_key
            .encrypt(&mut rng, Pkcs1v15Encrypt, plaintext.as_ref())
            .expect("failed to encrypt");

        // print out what we have
        println!("Here is the ciphertext:");
        println!("{}", Base64::encode_string(&ciphertext));
    }
}
