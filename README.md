# Rust Cryptography Examples

This repository contains a few examples of how to use the
[Rust Crypto](https://github.com/RustCrypto) crates for cryptography.

| Example                           | What it shows                                                  |
| --------------------------------- | -------------------------------------------------------------- |
| [aes-example](aes-example/)       | Encrypting and decrypting with AES-256-GCM                     |
| [rsa-example](rsa-example/)       | RSA encryption with OAEP and signatures with PSS (both SHA-256) |
| [ecdsa-example](ecdsa-example/)   | Signing and verifying with ECDSA on the P-256 curve            |
| [hash-example](hash-example/)     | Hashing a file with SHA-256 or SHA3-256                        |

Each example is its own Cargo project with a README that shows how to run it.
You need Rust 1.88 or newer to build all of them (run `rustup update` if
cargo says your Rust is too old).
