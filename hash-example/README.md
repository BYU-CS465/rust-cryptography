# Hash

This example code shows how to use the SHA2 and SHA3 hash functions

## Creating a hash

The `hash` program will print the hex format of the SHA2 hash of a file:

```bash
$ cargo run hash important.txt
```

You can also print the SHA3 hash of the file:

```bash
$ cargo run hash important.txt --hash sha3
```

## Checking a hash

The `hash` program can also check the SHA2 hash of a file:

```bash
$ cargo run hash important.txt --check 43494a1b3c110ad2ea3a86c3443485a10fa0381c41bcf94fcc13445dc17460d5
```

or the SHA3 hash of a file:

```bash
$ cargo run hash important.txt --hash sha3 --check 376d249ff4fb483f0ff26f0cd949e89883c82674c18f4e58274b5cab2906b143
```

## Code notes

- This code uses the [clap crate](https://docs.rs/clap/latest/clap/index.html)
  to parse arguments.

- This code uses an enumerated type to represent the hashing algorithm, and
  implements `std::fmt::Display` so we can print out the enumerated type

- This code uses the [base16ct crate](https://docs.rs/base16ct/latest/base16ct/)
  to convert a hash to hex format.
