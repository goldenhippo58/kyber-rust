//! # Kyber-Rust
//!
//! A Rust wrapper for the Kyber post-quantum key encapsulation mechanism.
//!
//! This crate provides a safe Rust interface to the Kyber algorithm, which is a
//! finalist in the NIST Post-Quantum Cryptography standardization process.
//!
//! ## Example
//!
//! ```rust
//! use kyber_rust::{generate_keypair, encapsulate, decapsulate};
//!
//! // Generate a keypair
//! let (public_key, secret_key) = generate_keypair().unwrap();
//!
//! // Encapsulate a shared secret
//! let (ciphertext, shared_secret_enc) = encapsulate(&public_key).unwrap();
//!
//! // Decapsulate the shared secret
//! let shared_secret_dec = decapsulate(&ciphertext, &secret_key).unwrap();
//!
//! // Verify that the shared secrets match
//! assert_eq!(shared_secret_enc, shared_secret_dec);
//! ```

use libloading::{Library, Symbol};
use std::os::raw::c_int;
use std::sync::Once;

pub const CRYPTO_PUBLICKEYBYTES: usize = 1184;
pub const CRYPTO_SECRETKEYBYTES: usize = 2400;
pub const CRYPTO_CIPHERTEXTBYTES: usize = 1088;
pub const CRYPTO_BYTES: usize = 32;

static INIT: Once = Once::new();
static mut LIBRARY: Option<Library> = None;

fn load_library() -> &'static Library {
    INIT.call_once(|| unsafe {
        LIBRARY = Some(Library::new("kyber.dll").expect("Failed to load kyber.dll"));
    });
    unsafe { LIBRARY.as_ref().unwrap() }
}

/// Generates a Kyber keypair.
///
/// Returns a tuple containing the public key and secret key.
pub fn generate_keypair(
) -> Result<([u8; CRYPTO_PUBLICKEYBYTES], [u8; CRYPTO_SECRETKEYBYTES]), String> {
    let mut pk = [0u8; CRYPTO_PUBLICKEYBYTES];
    let mut sk = [0u8; CRYPTO_SECRETKEYBYTES];

    let result = crypto_kem_keypair(&mut pk, &mut sk);
    if result != 0 {
        return Err(format!(
            "Keypair generation failed with error code: {}",
            result
        ));
    }

    Ok((pk, sk))
}

/// Encapsulates a shared secret using a public key.
///
/// Returns a tuple containing the ciphertext and the encapsulated shared secret.
pub fn encapsulate(
    pk: &[u8; CRYPTO_PUBLICKEYBYTES],
) -> Result<([u8; CRYPTO_CIPHERTEXTBYTES], [u8; CRYPTO_BYTES]), String> {
    let mut ct = [0u8; CRYPTO_CIPHERTEXTBYTES];
    let mut ss = [0u8; CRYPTO_BYTES];

    let result = crypto_kem_enc(&mut ct, &mut ss, pk);
    if result != 0 {
        return Err(format!("Encapsulation failed with error code: {}", result));
    }

    Ok((ct, ss))
}

/// Decapsulates a shared secret using a ciphertext and a secret key.
///
/// Returns the decapsulated shared secret.
pub fn decapsulate(
    ct: &[u8; CRYPTO_CIPHERTEXTBYTES],
    sk: &[u8; CRYPTO_SECRETKEYBYTES],
) -> Result<[u8; CRYPTO_BYTES], String> {
    let mut ss = [0u8; CRYPTO_BYTES];

    let result = crypto_kem_dec(&mut ss, ct, sk);
    if result != 0 {
        return Err(format!("Decapsulation failed with error code: {}", result));
    }

    Ok(ss)
}

fn crypto_kem_keypair(
    pk: &mut [u8; CRYPTO_PUBLICKEYBYTES],
    sk: &mut [u8; CRYPTO_SECRETKEYBYTES],
) -> i32 {
    let lib = load_library();
    unsafe {
        let func: Symbol<unsafe extern "C" fn(*mut u8, *mut u8) -> c_int> = lib
            .get(b"pqcrystals_kyber768_ref_keypair")
            .expect("Failed to load keypair function");
        func(pk.as_mut_ptr(), sk.as_mut_ptr())
    }
}

fn crypto_kem_enc(
    ct: &mut [u8; CRYPTO_CIPHERTEXTBYTES],
    ss: &mut [u8; CRYPTO_BYTES],
    pk: &[u8; CRYPTO_PUBLICKEYBYTES],
) -> i32 {
    let lib = load_library();
    unsafe {
        let func: Symbol<unsafe extern "C" fn(*mut u8, *mut u8, *const u8) -> c_int> = lib
            .get(b"pqcrystals_kyber768_ref_enc")
            .expect("Failed to load enc function");
        func(ct.as_mut_ptr(), ss.as_mut_ptr(), pk.as_ptr())
    }
}

fn crypto_kem_dec(
    ss: &mut [u8; CRYPTO_BYTES],
    ct: &[u8; CRYPTO_CIPHERTEXTBYTES],
    sk: &[u8; CRYPTO_SECRETKEYBYTES],
) -> i32 {
    let lib = load_library();
    unsafe {
        let func: Symbol<unsafe extern "C" fn(*mut u8, *const u8, *const u8) -> c_int> = lib
            .get(b"pqcrystals_kyber768_ref_dec")
            .expect("Failed to load dec function");
        func(ss.as_mut_ptr(), ct.as_ptr(), sk.as_ptr())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_keypair() {
        let (pk, sk) = generate_keypair().unwrap();
        assert_eq!(pk.len(), CRYPTO_PUBLICKEYBYTES);
        assert_eq!(sk.len(), CRYPTO_SECRETKEYBYTES);
    }

    #[test]
    fn test_encapsulate() {
        let (pk, _) = generate_keypair().unwrap();
        let (ct, ss) = encapsulate(&pk).unwrap();
        assert_eq!(ct.len(), CRYPTO_CIPHERTEXTBYTES);
        assert_eq!(ss.len(), CRYPTO_BYTES);
    }

    #[test]
    fn test_decapsulate() {
        let (pk, sk) = generate_keypair().unwrap();
        let (ct, ss_enc) = encapsulate(&pk).unwrap();
        let ss_dec = decapsulate(&ct, &sk).unwrap();
        assert_eq!(ss_enc, ss_dec);
    }

    #[test]
    fn test_invalid_decapsulation() {
        let (pk1, _sk1) = generate_keypair().unwrap();
        let (_, sk2) = generate_keypair().unwrap();
        let (ct, ss_enc) = encapsulate(&pk1).unwrap();

        // Attempt to decapsulate with wrong secret key
        let ss_dec = decapsulate(&ct, &sk2).unwrap();

        // The decapsulated secret should be different from the original
        assert_ne!(ss_enc, ss_dec);
    }

    #[test]
    fn test_multiple_encapsulations() {
        let (pk, sk) = generate_keypair().unwrap();

        for _ in 0..10 {
            let (ct, ss_enc) = encapsulate(&pk).unwrap();
            let ss_dec = decapsulate(&ct, &sk).unwrap();
            assert_eq!(ss_enc, ss_dec);
        }
    }
}
