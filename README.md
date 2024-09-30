# Kyber-Rust

A Rust wrapper for the Kyber post-quantum key encapsulation mechanism.

## Overview

This crate provides a safe Rust interface to the Kyber algorithm, which is a finalist in the NIST Post-Quantum Cryptography standardization process. Kyber is a key encapsulation mechanism (KEM) that is believed to be secure against attacks by quantum computers.

## Features

- Generate Kyber keypairs
- Encapsulate shared secrets
- Decapsulate shared secrets
- Easy-to-use high-level API

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
kyber-rust = "0.2.1"
```

## Usage

Here's a basic example of how to use the Kyber-Rust library:

```rust
use kyber_rust::{generate_keypair, encapsulate, decapsulate};

fn main() -> Result<(), String> {
    // Generate a keypair
    let (public_key, secret_key) = generate_keypair()?;

    // Encapsulate a shared secret
    let (ciphertext, shared_secret_enc) = encapsulate(&public_key)?;

    // Decapsulate the shared secret
    let shared_secret_dec = decapsulate(&ciphertext, &secret_key)?;

    // Verify that the shared secrets match
    assert_eq!(shared_secret_enc, shared_secret_dec);

    Ok(())
}
```

## Safety

This crate uses `unsafe` Rust to interface with the C implementation of Kyber. While efforts have been made to ensure safety, users should be aware of the potential risks associated with FFI and unsafe code.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contribution

Contributions are welcome! Please feel free to submit a Pull Request.

## Disclaimer

This implementation is for educational and research purposes only. It has not been audited for production use.