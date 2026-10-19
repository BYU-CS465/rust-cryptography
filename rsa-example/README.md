# RSA

This example code shows how to use RSA to encrypt, decrypt, sign, and verify
data. Encryption uses OAEP padding and signatures use PSS, both with SHA-256.

It uses a release candidate of the [rsa crate](https://docs.rs/rsa/), version
`0.10.0-rc.19`, which is pinned exactly in `Cargo.toml`.

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

- Encryption uses OAEP padding, which is recommended over the older PKCS#1
  v1.5 encryption padding.

- Signing uses `pss::BlindedSigningKey`, as the rsa crate documentation
  recommends.

## Security note

The rsa crate has a known timing side-channel,
[RUSTSEC-2023-0071](https://rustsec.org/advisories/RUSTSEC-2023-0071) (the
"Marvin Attack"): its private-key operations do not run in constant time, so an
attacker who can measure how long decryption takes could learn the private key.
No version of the crate fixes this yet. Running these examples on your own
computer is fine, but don't use this crate to decrypt or sign on a server where
attackers can time the responses.
