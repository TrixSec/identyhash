use std::fs::File;
use std::io::{self, BufRead};

pub fn identify_hash(hash: &str) -> &str {
    match hash.len() {
        32 => {
            if hash.chars().all(|c| c.is_digit(16)) {
                "MD5"
            } else {
                "Unknown (length 32)"
            }
        }
        40 => {
            if hash.chars().all(|c| c.is_digit(16)) {
                "SHA-1"
            } else {
                "Unknown (length 40)"
            }
        }
        56 => {
            if hash.starts_with("$2a$") || hash.starts_with("$2b$") || hash.starts_with("$2y$") {
                "bcrypt"
            } else {
                "SHA-224"
            }
        }
        64 => {
            if hash.chars().all(|c| c.is_digit(16)) {
                "SHA-256"
            } else {
                "Unknown (length 64)"
            }
        }
        96 => {
            if hash.chars().all(|c| c.is_digit(16)) {
                "SHA-384"
            } else {
                "Unknown (length 96)"
            }
        }
        128 => "SHA-512",
        37 => "Cisco-IOS (Type 7)",
        68 => {
            if hash.starts_with("$5$") {
                "SHA-256 Crypt"
            } else {
                "Unknown (length 68)"
            }
        }
        86 => {
            if hash.starts_with("$6$") {
                "SHA-512 Crypt"
            } else {
                "Unknown (length 86)"
            }
        }
        22 => {
            if hash.chars().all(|c| c.is_alphanumeric()) {
                "Blowfish"
            } else {
                "Unknown (length 22)"
            }
        }
        29 => {
            if hash.starts_with("$2a$") || hash.starts_with("$2b$") || hash.starts_with("$2y$") {
                "bcrypt (Blowfish)"
            } else {
                "Unknown (length 29)"
            }
        }
        59 => {
            if hash.starts_with("$apr1$") {
                "Apache MD5"
            } else {
                "Unknown (length 59)"
            }
        }
        13 => "DES (Unix)",
        16 => "MySQL (old)",
        41 => {
            if hash.starts_with("*") {
                "MySQL5.x"
            } else {
                "SHA-1 (160-bit)"
            }
        }
        20 => {
            if hash.chars().all(|c| c.is_alphanumeric()) {
                "NTLM"
            } else {
                "Unknown (length 20)"
            }
        }
        24 => "CRC-32",
        48 => "SHA-384",
        112 => "SHA-512/256",
        256 => "Whirlpool",
        34 => "MD5(Wordpress)",
        39 => "RipeMD-128",
        80 => "SHA3-256",
        104 => "SHA3-512",
        _ => "Unknown",
    }
}

pub fn read_hashes_from_file(file_path: &str) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        if let Ok(hash) = line {
            let hash_type = identify_hash(hash.trim());
            println!("Hash: {} - Identified hash type: {}", hash, hash_type);
        }
    }
    Ok(())
}
