# AES GCM

This example code shows how to use AES GCM to encrypt and decrypt data.

## Encryption

This program expects one argument -- some text to encrypt. It then chooses a
random 256-bit AES key and nonce for AES GCM mode and encrypts the text.

```bash
$ cargo run --bin encrypt hello
Key: Wad9dtmU024PZ7u8neQRSKrPl2h072c8Zd4+VNUJY0M=
Nonce: 4iiGDwVSKjhXXVL6
Plaintext: hello
Ciphertext: Sb8/uTr/AmYQLvH+GLJ2BBInz4Ld
```

## Decryption

This program expects three arguments:

- a key
- a nonce
- ciphertext

All arguments should be base64 encoded.

```bash
$ cargo run --bin decrypt Wad9dtmU024PZ7u8neQRSKrPl2h072c8Zd4+VNUJY0M= 4iiGDwVSKjhXXVL6 Sb8/uTr/AmYQLvH+GLJ2BBInz4Ld
Key: Wad9dtmU024PZ7u8neQRSKrPl2h072c8Zd4+VNUJY0M=
Nonce: 4iiGDwVSKjhXXVL6
Ciphertext: Sb8/uTr/AmYQLvH+GLJ2BBInz4Ld
Plaintext: hello
```

If the key, nonce, or ciphertext is wrong, or the ciphertext has been
modified, AES GCM detects this and decryption fails:

```bash
$ cargo run --bin decrypt Wad9dtmU024PZ7u8neQRSKrPl2h072c8Zd4+VNUJY0M= 4iiGDwVSKjhXXVL6 Xb8/uTr/AmYQLvH+GLJ2BBInz4Ld
Decryption failed: the ciphertext could not be authenticated.
```
