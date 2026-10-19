use std::{error::Error, fmt, fs, path::PathBuf, process};

use clap::{Parser, ValueEnum};
use sha2::{Digest, Sha256};
use sha3::Sha3_256;

/// Generate and check hashes of a file
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The file to hash
    filename: PathBuf,

    /// The hash function to use
    #[arg(long, value_enum, default_value_t = Hasher::Sha2)]
    hash: Hasher,

    /// Check the file against this hash (in hex) instead of printing it
    #[arg(short, long)]
    check: Option<String>,
}

// deriving ValueEnum lets clap parse `--hash sha2` or `--hash sha3` and reject anything else
#[derive(Clone, Copy, ValueEnum)]
enum Hasher {
    Sha2,
    Sha3,
}

// implement fmt::Display so we can print out the enumerated type
impl fmt::Display for Hasher {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Hasher::Sha2 => write!(f, "SHA2"),
            Hasher::Sha3 => write!(f, "SHA3"),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    // get the contents of the file as bytes, so we can hash any kind of file
    let contents = fs::read(&cli.filename)
        .map_err(|e| format!("cannot read {}: {e}", cli.filename.display()))?;

    // hash the file with the chosen hash function
    let hash = match cli.hash {
        Hasher::Sha2 => Sha256::digest(&contents),
        Hasher::Sha3 => Sha3_256::digest(&contents),
    };

    // get a hex representation of the hash
    let hex_hash = base16ct::lower::encode_string(&hash);

    match cli.check {
        // if we are checking a hash, compare it to the one we computed
        // (a hash is not secret, so this does not need a constant-time comparison)
        Some(expected) => {
            if expected.eq_ignore_ascii_case(&hex_hash) {
                println!("The hash matches!");
            } else {
                eprintln!("DANGER: the hash does not match!");
                process::exit(1);
            }
        }
        // otherwise print the hash
        None => {
            println!(
                "The {} hash of {}: {}",
                cli.hash,
                cli.filename.display(),
                hex_hash
            );
        }
    }

    Ok(())
}
