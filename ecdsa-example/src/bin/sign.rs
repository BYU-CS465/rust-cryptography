use std::error::Error;

use p256::{
    ecdsa::{Signature, SigningKey, signature::Signer},
    elliptic_curve::Generate,
};

use ecdsa_example::read_line;

fn main() -> Result<(), Box<dyn Error>> {
    // generate a random signing (private) key
    let signing_key = SigningKey::generate();
    // get the matching verifying (public) key
    let verifying_key = signing_key.verifying_key();

    // print the verifying key in hex, as an uncompressed SEC1 point (65 bytes)
    let verifying_key_bytes = verifying_key.to_sec1_point(false);
    println!("Here is my verifying key:");
    println!(
        "{}",
        base16ct::lower::encode_string(verifying_key_bytes.as_ref())
    );

    // get the text to sign
    while let Some(data) = read_line("Enter something to sign, or nothing to exit")? {
        // sign
        let signature: Signature = signing_key.sign(data.as_bytes());

        // print the signature in hex (64 bytes)
        println!("Signature:");
        println!("{}", base16ct::lower::encode_string(&signature.to_bytes()));
    }

    Ok(())
}
