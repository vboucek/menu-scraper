use anyhow::anyhow;
use argon2::password_hash::rand_core::OsRng;

use argon2::{Argon2, password_hash, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::SaltString;

pub fn hash_password(password: &str) -> Result<String, password_hash::errors::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default().hash_password(password.as_ref(), &salt)?;
    Ok(hash.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> Result<(), anyhow::Error> {
    let parsed_hash = PasswordHash::new(hash).map_err(|_| anyhow!("Chyba při kontrole hesla."))?;
    Argon2::default().verify_password(password.as_ref(), &parsed_hash).map_err(|_| anyhow!("Chybný email nebo heslo."))
}
