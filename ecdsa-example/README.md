# ECDSA

This example code shows how to use elliptic curve ECDSA for signing and
verifying. The code uses the [p256](https://docs.rs/p256/latest/p256/) curve and
the [example code](https://docs.rs/p256/latest/p256/ecdsa/index.html) given in
the Rust documentation.

## Signing

The `sign` program will print the verifying key in hex format, then enters a
loop to allow you to enter any text you want to sign:

```bash
$ cargo run --bin sign
```

## Verifying

The `verify` program allows you to input the verifying key in hex format, then
enters a loop to allow you to enter any text you want, along with a signature to
verify for that text:

```bash
$ cargo run --bin verify
```

## Code notes

- This code uses the [base16ct crate](https://docs.rs/base16ct/latest/base16ct/)
  to convert a key to hex format.
