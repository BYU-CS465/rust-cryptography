use std::error::Error;

use p256::ecdsa::{Signature, VerifyingKey, signature::Verifier};

use ecdsa_example::read_line;

fn main() -> Result<(), Box<dyn Error>> {
    // get a verifying key in hex from standard input
    let Some(hex_key) = read_line("Enter a verifying key in hex format")? else {
        return Err("no verifying key entered".into());
    };

    // decode the verifying key
    let key_bytes =
        base16ct::mixed::decode_vec(&hex_key).map_err(|_| "the verifying key must be in hex")?;
    let verifying_key = VerifyingKey::from_sec1_bytes(&key_bytes)
        .map_err(|_| "that is not a valid verifying key")?;

    // get the text to verify
    while let Some(data) = read_line("Enter data to verify, or nothing to exit")? {
        // get the signature
        let Some(hex_signature) = read_line("Enter signature")? else {
            eprintln!("No signature entered.");
            continue;
        };

        // convert from hex
        let Ok(signature) = base16ct::mixed::decode_vec(&hex_signature) else {
            eprintln!("The signature must be in hex.");
            continue;
        };
        let Ok(signature) = Signature::from_slice(&signature) else {
            eprintln!("That is not a valid signature.");
            continue;
        };

        // verify
        match verifying_key.verify(data.as_bytes(), &signature) {
            Ok(()) => println!("Signature is correct!"),
            Err(_) => eprintln!("Could not verify signature!"),
        }
    }

    Ok(())
}
