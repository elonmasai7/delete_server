use ring::rand::{SecureRandom, SystemRandom};
use ring::pbkdf2;
use ring::digest;
use std::num::NonZeroU32;

const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
static DIGEST_ALG: &'static digest::Algorithm = &digest::SHA512;
static ITERATIONS: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(100_000) };

fn generate_salt() -> [u8; 16] {
    let rng = SystemRandom::new();
    let mut salt = [0u8; 16];
    rng.fill(&mut salt).unwrap();
    salt
}

fn hash_password(password: &str, salt: &[u8]) -> [u8; CREDENTIAL_LEN] {
    let mut hash = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        DIGEST_ALG,
        ITERATIONS,
        salt,
        password.as_bytes(),
        &mut hash,
    );
    hash
}

fn verify_password(password: &str, salt: &[u8], hashed: &[u8]) -> bool {
    pbkdf2::verify(
        DIGEST_ALG,
        ITERATIONS,
        salt,
        password.as_bytes(),
        hashed,
    ).is_ok()
}

fn main() {
    // Example usage:
    let password = "SuperSecretPassword";
    
    // Generate a salt
    let salt = generate_salt();
    
    // Hash the password
    let hashed_password = hash_password(password, &salt);
    
    // Store the salt and hashed_password securely
    
    // Verify the password later
    let is_valid = verify_password("SuperSecretPassword", &salt, &hashed_password);
    assert!(is_valid);

    // Securely delete password from memory
    let mut password_to_delete = String::from(password);
    unsafe {
        let password_bytes = password_to_delete.as_mut_vec();
        for byte in password_bytes.iter_mut() {
            *byte = 0;
        }
    }

    println!("Password hashed and verified successfully.");
}
