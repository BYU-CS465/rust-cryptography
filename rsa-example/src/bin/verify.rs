use base64ct::{Base64, Encoding};
use rsa::pkcs8::DecodePublicKey;
use rsa::pss::{Signature, VerifyingKey};
use rsa::sha2::Sha256;
use rsa::signature::Verifier;
use rsa::RsaPublicKey;

use std::io;

fn main() {
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

    // derive the verifying key
    let verifying_key: VerifyingKey<Sha256> = VerifyingKey::from(pub_key);

    loop {
        // get the text to verify
        println!("Enter data to verify, or nothing to exit");
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

        println!("Enter signature");

        // get signature
        let mut signature = String::new();
        match io::stdin().read_line(&mut signature) {
            Ok(_n) => (),
            Err(error) => {
                println!("error: {error}");
                continue;
            }
        }

        let signature = signature.trim();

        // convert from BASE64
        let signature = Base64::decode_vec(&signature).unwrap();
        let signature =
            Signature::try_from(signature.as_ref()).expect("Could not convert signature");

        // verify
        // TBD -- just print an error if the signature fails
        match verifying_key.verify(data.as_bytes(), &signature) {
            Ok(_) => (),
            Err(_e) => {
                println!("Could not verify signature!");
                continue;
            }
        };

        // print
        println!("Signature is correct!");
    }
}
