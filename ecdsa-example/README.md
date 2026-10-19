# ECDSA

This example code shows how to use elliptic curve ECDSA for signing and
verifying. The code uses the [p256](https://docs.rs/p256/latest/p256/) curve and
follows the [example code](https://docs.rs/p256/latest/p256/ecdsa/index.html)
given in the Rust documentation.

## Signing

The `sign` program generates a signing key and prints its verifying key in hex
format. It then lets you repeatedly enter any text you want to sign, and prints
a signature in hex for each one:

```bash
$ cargo run --bin sign
Here is my verifying key:
04eb01715a277cd8579619665180f0bcc02e5fc24012cceeb3028f879213a8b298899de1d66d16bdb2f8dd07809305342f2de45a63ccbe977a404036305deebe68
Enter something to sign, or nothing to exit
hello
Signature:
bcd9025bec376a948bb4d267d11946970929f46d72c605d2fff099b8d04cfcc39dabd43f675546040f9633f47b947a387dfe6605cd037720d2996cbf5939ca56
```

## Verifying

The `verify` program asks you to enter the verifying key in hex format. It then
lets you repeatedly enter any text you want, along with a signature to verify
for that text:

```bash
$ cargo run --bin verify
Enter a verifying key in hex format
04eb01715a277cd8579619665180f0bcc02e5fc24012cceeb3028f879213a8b298899de1d66d16bdb2f8dd07809305342f2de45a63ccbe977a404036305deebe68
Enter data to verify, or nothing to exit
hello
Enter signature
bcd9025bec376a948bb4d267d11946970929f46d72c605d2fff099b8d04cfcc39dabd43f675546040f9633f47b947a387dfe6605cd037720d2996cbf5939ca56
Signature is correct!
Enter data to verify, or nothing to exit
goodbye
Enter signature
bcd9025bec376a948bb4d267d11946970929f46d72c605d2fff099b8d04cfcc39dabd43f675546040f9633f47b947a387dfe6605cd037720d2996cbf5939ca56
Could not verify signature!
```

## Code notes

- `src/lib.rs` has a `read_line` helper shared by both programs.

- The verifying key is printed as an uncompressed SEC1 point: 65 bytes, starting
  with `04`, followed by the x and y coordinates of the point on the curve.

- A P-256 ECDSA signature is always 64 bytes: two 32-byte numbers, `r` and `s`.

- Signing is deterministic: the p256 crate picks the secret nonce for each
  signature from the key and the message ([RFC
  6979](https://datatracker.ietf.org/doc/html/rfc6979)), so signing the same
  text twice gives the same signature. This matters because reusing a nonce
  for two different messages would reveal the signing key. In contrast, the
  RSA-PSS signatures in the RSA example are randomized.

- This code uses the [base16ct crate](https://docs.rs/base16ct/latest/base16ct/)
  to convert keys and signatures to and from hex format. It accepts uppercase
  or lowercase hex.
