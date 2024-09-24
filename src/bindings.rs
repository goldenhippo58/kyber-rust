#[link(name = "kyber")]
extern "C" {
    pub fn pqcrystals_kyber768_ref_keypair(pk: *mut u8, sk: *mut u8) -> i32;
    pub fn pqcrystals_kyber768_ref_enc(ct: *mut u8, ss: *mut u8, pk: *const u8) -> i32;
    pub fn pqcrystals_kyber768_ref_dec(ss: *mut u8, ct: *const u8, sk: *const u8) -> i32;
<<<<<<< HEAD
}
=======
}
>>>>>>> be76552 (Initial commit of Kyber-Rust library)
