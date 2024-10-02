# AES GCM

This example shows how to use AES GCM to encrypt data.

This program expects one argument -- some text to encrypt. It then chooses a
random 256-bit AES key and nonce for AES GCM mode and encrypts the text.

```bash
$ cargo run hello
Key: oYJGle56hySbk5XRDb+5hIXmWFquhpV+2+9Mv5ObwnU=
Nonce: Ra9SgbtJtCof7UkN
Plaintext: hello
Ciphertext: TkrXHAZBnSYdjpiQQ4k8tjW7ClMS
```
