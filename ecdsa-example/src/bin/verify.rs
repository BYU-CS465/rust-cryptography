use p256::ecdsa::Signature;
use p256::ecdsa::{signature::Verifier, VerifyingKey};
use std::io;

fn main() {
    // get a public key from standard input
    println!("Enter a public key in hex format");
    let mut hex_signing = String::new();
    match io::stdin().read_line(&mut hex_signing) {
        Ok(_n) => (),
        Err(_error) => {
            panic!("Could not read input");
        }
    };
    let hex_signing = hex_signing.trim();

    // decode the verifying key
    let mut buf = [0u8; 128];
    let verifying_key_bytes = base16ct::lower::decode(hex_signing, &mut buf).unwrap();
    let verifying_key = VerifyingKey::from_sec1_bytes(&verifying_key_bytes).unwrap();

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
        println!("length: {}", signature.len());
        // convert from hex
        let mut buf = [0u8; 128];
        let signature = base16ct::lower::decode(signature, &mut buf).unwrap();
        let signature =
            Signature::try_from(signature.as_ref()).expect("Could not convert signature");

        // verify
        match verifying_key.verify(data.as_bytes(), &signature) {
            Ok(_) => (),
            Err(e) => {
                println!("Error verifying signature: {}", e);
                return;
            }
        }

        // print
        println!("Signature is correct!");
    }
}
