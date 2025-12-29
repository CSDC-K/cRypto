use balloon_hash::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Balloon
};
use sha2::Sha256;
use crate::libs::errors;
use crate::libs::salt_gen;


pub fn create_balloonHash(password : &str) -> Result<String, balloon_hash::password_hash::Error> {

    let salt = SaltString::generate(&mut OsRng);
    let balloon = Balloon::<Sha256>::default();
    let password_hash = balloon.hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())


} 