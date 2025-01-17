use std::env;
use std::path::Path;
use identyhash::{identify_hash, read_hashes_from_file};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <hash or file path>", args[0]);
        return;
    }

    let input = &args[1];

    if Path::new(input).exists() {
        if let Err(err) = read_hashes_from_file(input) {
            eprintln!("Error reading file: {}", err);
        }
    } else {
        let hash_type = identify_hash(input.trim());
        println!("Hash: {} - Identified hash type: {}", input, hash_type);
    }
}