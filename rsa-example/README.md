# RSA

This example code shows how to use RSA encrypt, decrypt, sign, and verify data.

## Encryption and decryption

The `decrypt` program will print its public key and then allow the user to
repeatedly enter cihertext to decrypt.

```bash
$ cargo run --bin decrypt
```

The `encrypt` program will allow the user to input a public key, then allow the
user to repeatedly enter plaintext to decrypt. It prints ciphertext for each
plaintext.

```bash
$ cargo run --bin encrypt
```

## Sign and verify

The `sign` program will print its public key and then allow the user to
repeatedly enter text and produce a signature over that text.

```bash
$ cargo run --bin sign
```

The `verify` program will allow the user to input a public key, then allow the
user to repeatedly enter text and a signature to verify.

```bash
$ cargo run --bin verify
```
