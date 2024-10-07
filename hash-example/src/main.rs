use sha2::{Digest, Sha256};
use sha3::Sha3_256;
use std::fmt;
use std::fs;
use std::path::PathBuf;

use clap::Parser;

/// Generate and check hashes

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Positional: file name
    filename: Option<PathBuf>,

    /// Argument: type of hash
    #[arg(long)]
    hash: Option<String>,

    /// Argument: check a hash (default is compute)
    #[arg(short, long)]
    check: Option<String>,
}

enum Hasher {
    SHA2,
    SHA3,
}

// implement fmt::Display so we can print out the enumerated type
impl fmt::Display for Hasher {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Hasher::SHA2 => {
                write!(f, "SHA2")
            }
            Hasher::SHA3 => {
                write!(f, "SHA3")
            }
        }
    }
}

fn main() {
    let cli = Cli::parse();

    let path = match cli.filename {
        Some(path) => path,
        None => {
            println!("You must enter a filename. Try `cargo run hash --help`");
            return;
        }
    };

    // get a copy of the path so we can print it
    let output_path = path.clone();
    let pathname = output_path.to_str().expect("can't find that file");

    // get the contents of the file
    let contents = fs::read_to_string(path).expect("Cannot read the file");

    // what hash function are we using?
    let hasher = match cli.hash {
        Some(s) => {
            if s == "sha3" {
                Hasher::SHA3
            } else {
                Hasher::SHA2
            }
        }
        None => Hasher::SHA2,
    };

    let &mut hash;
    match hasher {
        Hasher::SHA2 => {
            // hash the file
            hash = Sha256::digest(contents);
        }
        Hasher::SHA3 => {
            hash = Sha3_256::digest(contents);
        }
    }

    // get a hex representation of the hash
    let hex_hash: String = base16ct::lower::encode_string(&hash);

    // if we are checking a hash
    match cli.check {
        Some(s) => {
            if s == hex_hash {
                println!("The hash matches!");
            } else {
                println!("DANGER: the hash does not match!");
            }
        }
        None => {
            println!("The {} hash of {}: {}", hasher, pathname, hex_hash);
        }
    };
}
