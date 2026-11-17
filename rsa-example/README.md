# RSA

This example code shows how to use RSA to encrypt, decrypt, sign, and verify
data. Encryption uses OAEP padding and signatures use PSS, both with SHA-256.

It uses version 0.10 of the [rsa crate](https://docs.rs/rsa/), which needs
Rust 1.87 or newer.

## Encryption and decryption

Run these two programs in separate terminals.

The `decrypt` program generates an RSA key pair and prints its public key. It
then lets the user repeatedly enter base64 ciphertext to decrypt.

```bash
$ cargo run --bin decrypt
Here is my public key:
-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA7lGgv8VRUUH6K7H45gBq
...
CQIDAQAB
-----END PUBLIC KEY-----

Enter something to decrypt, or nothing to exit
G084PMliofhYaaenLH/JPHXDnXIeCXksvOIoUFkDrouxnaLf9YLOldHDkjKpPQha...
Plaintext: hello
```

The `encrypt` program asks the user to paste in a public key (everything from
`-----BEGIN PUBLIC KEY-----` through `-----END PUBLIC KEY-----`). It then lets
the user repeatedly enter plaintext to encrypt, and prints the ciphertext for
each one.

```bash
$ cargo run --bin encrypt
Enter a public key in PEM format
-----BEGIN PUBLIC KEY-----
...
-----END PUBLIC KEY-----
Enter something to encrypt, or nothing to exit
hello
Here is the ciphertext:
G084PMliofhYaaenLH/JPHXDnXIeCXksvOIoUFkDrouxnaLf9YLOldHDkjKpPQha...
```

RSA can only encrypt a short message: with a 2048-bit key and OAEP with
SHA-256, at most 190 bytes. In practice, RSA is used to encrypt a symmetric key
(such as an AES key), which then encrypts the data.

## Sign and verify

The `sign` program generates an RSA key pair and prints its public key. It
then lets the user repeatedly enter text and prints a signature over that text.

```bash
$ cargo run --bin sign
Here is my public key:
-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAziczOL/rTB7UzAdnr7ec
...
MwIDAQAB
-----END PUBLIC KEY-----

Enter something to sign, or nothing to exit
hello
Signature:
Ku8hC0exq3ZOI5XeL3sOmh8wSS8JABDq7TrU4PxPRVGTLysrjcIjPpI7bwxEpRNg...
```

The `verify` program asks the user to paste in a public key. It then lets the
user repeatedly enter text and a signature to verify.

```bash
$ cargo run --bin verify
Enter a public key in PEM format
-----BEGIN PUBLIC KEY-----
...
-----END PUBLIC KEY-----
Enter data to verify, or nothing to exit
hello
Enter signature
Ku8hC0exq3ZOI5XeL3sOmh8wSS8JABDq7TrU4PxPRVGTLysrjcIjPpI7bwxEpRNg...
Signature is correct!
```

(Keys, ciphertext, and signatures are shortened with `...` above.)

## Code notes

- `src/lib.rs` has two helpers shared by all four programs: `read_line` reads
  one line of input, and `read_public_key` reads a PEM public key.

- The key types are `rsa::PrivateKey` and `rsa::PublicKey`. (Older versions of
  the crate, and older examples online, call them `RsaPrivateKey` and
  `RsaPublicKey`.)

- Encryption uses OAEP padding with SHA-256 (`Oaep::<Sha256>`), and signing
  uses PSS with SHA-256 (`pss::SigningKey<Sha256>` and
  `pss::VerifyingKey<Sha256>`). These are the currently recommended schemes.
  The older PKCS#1 v1.5 padding is still available, but only if you enable the
  crate's `pkcs1v15` feature, because it is easy to use insecurely.

- `pss::SigningKey` uses blinding by default, which protects the private key
  against timing attacks. (Older versions had a separate `BlindedSigningKey`.)

## Security note

Versions of the rsa crate before 0.10 had a timing side-channel,
[RUSTSEC-2023-0071](https://rustsec.org/advisories/RUSTSEC-2023-0071) (the
"Marvin Attack"): their private-key operations did not run in constant time, so
an attacker who could measure how long decryption took could learn the private
key. Version 0.10 fixes this by using constant-time arithmetic, which is one
reason to use 0.10 or newer.
