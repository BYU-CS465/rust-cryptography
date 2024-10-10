use p256::ecdsa::VerifyingKey;
use p256::ecdsa::{signature::Signer, Signature, SigningKey};
use rand_core::OsRng; // requires 'getrandom' feature
use std::io;

fn main() {
    // Generate signing key
    let signing_key = SigningKey::random(&mut OsRng);
    // Get verifying key
    let verifying_key = VerifyingKey::from(&signing_key); // Serialize with `::to_encoded_point()`

    // print signing key
    let hash_verifying = base16ct::lower::encode_string(&verifying_key.to_sec1_bytes());
    println!("Here is my verifying key:");
    println!("{}", hash_verifying);

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
        let signature: Signature = signing_key.sign(data.as_bytes());

        // print
        let hex_signature: String = base16ct::lower::encode_string(&signature.to_bytes());
        println!("Signature: ");

        println!("{}", hex_signature);
    }
}
