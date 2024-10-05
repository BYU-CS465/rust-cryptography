# AES GCM

This example code shows how to use AES GCM to encrypt and decrypt data.

## Encryption

This program expects one argument -- some text to encrypt. It then chooses a
random 256-bit AES key and nonce for AES GCM mode and encrypts the text.

```bash
$ cargo run --bin encrypt hello
Key: oYJGle56hySbk5XRDb+5hIXmWFquhpV+2+9Mv5ObwnU=
Nonce: Ra9SgbtJtCof7UkN
Plaintext: hello
Ciphertext: TkrXHAZBnSYdjpiQQ4k8tjW7ClMS
```

## Decryption

This program expects three arguments:

- a key
- a nonce
- ciphertext

All arguments should be base64 encoded

```bash
$ cargo run --bin decrypt oYJGle56hySbk5XRDb+5hIXmWFquhpV+2+9Mv5ObwnU= Ra9SgbtJtCof7UkN TkrXHAZBnSYdjpiQQ4k8tjW7ClMS
Key: oYJGle56hySbk5XRDb+5hIXmWFquhpV+2+9Mv5ObwnU=
Nonce: Ra9SgbtJtCof7UkN
Ciphertext: TkrXHAZBnSYdjpiQQ4k8tjW7ClMS
Plaintext: hello
```
