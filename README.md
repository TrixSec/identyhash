**Current Version**: 0.1.0
**Author**: Trix Cyrus
**Copyright**: © 2024 Trixsec Org
**Maintained**: Yes

# IdentYhash

**IdentYhash** is a simple, dependency-free hash identifier tool written in Rust. It can identify various hash types from either direct input or from a file containing multiple hashes.

## Features

- Can read hashes from a file and identify each hash type.
- Lightweight and fast, with no external dependencies.

## Installation

### Using `cargo install`

To install `IdentYhash` as a standalone tool, use the following command:

```bash
cargo install identyhash
```

This will install `IdentYhash` globally, and you can run it from anywhere on your system.

### As a Dependency

If you want to use `IdentYhash` in your Rust project, add it to your `Cargo.toml`:

```toml
[dependencies]
identyhash = "0.1.0"
```

Replace `"0.1.0"` with the latest version of the crate.

## Usage

### Command Line Usage

After installing `IdentYhash`, you can use it as follows:

1. **Identify a Single Hash**:
   ```bash
   identyhash "<hash>"
   ```
   Example:
   ```bash
   identyhash "d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2"
   ```
   Output:
   ```
   Hash: d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2 - Identified hash type: SHA-1
   ```

2. **Identify Hashes from a File**:
   ```bash
   identyhash "path/to/your/hashfile.txt"
   ```
   Example file content (`hashfile.txt`):
   ```
   d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2
   e4e4e4e4e4e4e4e4e4e4e4e4e4e4e4e4e4e4e4e4e4e4
   ```

   Output:
   ```
   Hash: d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2 - Identified hash type: SHA-1
   Hash: e4e4e4e4e4e4e4e4e4e4e4e4e4e4e4e4e4e4e4e4e4e4 - Identified hash type: SHA-256
   ```

### As a Dependency in Rust Project

If you are using `IdentYhash` as a dependency in your project, you can utilize its functions like this:

```rust
use identyhash::{identify_hash, read_hashes_from_file};

fn main() {
    let hash = "d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2";
    let hash_type = identify_hash(hash);
    println!("Hash: {} - Identified hash type: {}", hash, hash_type);

    let file_path = "path/to/your/hashfile.txt";
    if let Err(err) = read_hashes_from_file(file_path) {
        eprintln!("Error reading file: {}", err);
    }
}
```

## Supported Hash Types

Here are the supported hash types:

- MD5
- SHA-1
- bcrypt
- SHA-224
- SHA-256
- SHA-384
- SHA-512
- Cisco-IOS (Type 7)
- SHA-256 Crypt
- SHA-512 Crypt
- Blowfish
- bcrypt (Blowfish)
- Apache MD5
- DES (Unix)
- MySQL (old)
- MySQL5.x
- SHA-1 (160-bit)
- NTLM
- CRC-32
- SHA-512/224
- SHA-512/256
- Whirlpool
- MD5 (Wordpress)
- RipeMD-128
- SHA3-256
- SHA3-512

---

Developed by [TrixSec](https://github.com/TrixSec).
```