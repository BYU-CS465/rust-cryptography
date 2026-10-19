# Hash

This example code shows how to use the SHA2 and SHA3 hash functions.

## Creating a hash

The `hash-example` program will print the hex format of the SHA2 (SHA-256) hash
of a file:

```bash
$ cargo run -- important.txt
The SHA2 hash of important.txt: 43494a1b3c110ad2ea3a86c3443485a10fa0381c41bcf94fcc13445dc17460d5
```

You can also print the SHA3 (SHA3-256) hash of the file:

```bash
$ cargo run -- important.txt --hash sha3
The SHA3 hash of important.txt: 376d249ff4fb483f0ff26f0cd949e89883c82674c18f4e58274b5cab2906b143
```

The `--hash` option accepts only `sha2` (the default) or `sha3`.

## Checking a hash

The program can also check the SHA2 hash of a file:

```bash
$ cargo run -- important.txt --check 43494a1b3c110ad2ea3a86c3443485a10fa0381c41bcf94fcc13445dc17460d5
The hash matches!
```

or the SHA3 hash of a file:

```bash
$ cargo run -- important.txt --hash sha3 --check 376d249ff4fb483f0ff26f0cd949e89883c82674c18f4e58274b5cab2906b143
The hash matches!
```

If the hash does not match, the program prints a warning and exits with status 1:

```bash
$ cargo run -- important.txt --hash sha3 --check 43494a1b3c110ad2ea3a86c3443485a10fa0381c41bcf94fcc13445dc17460d5
DANGER: the hash does not match!
```

Run `cargo run -- --help` to see all of the options.

## Code notes

- This code uses the [clap crate](https://docs.rs/clap/latest/clap/index.html)
  to parse arguments.

- This code uses an enumerated type to represent the hashing algorithm. It
  derives `clap::ValueEnum` so clap can parse and validate the `--hash` option,
  and implements `std::fmt::Display` so we can print out the enumerated type.

- This code reads the file as bytes, so it can hash any kind of file, not just
  text.

- This code uses the [base16ct crate](https://docs.rs/base16ct/latest/base16ct/)
  to convert a hash to hex format.
