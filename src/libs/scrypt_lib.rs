use scrypt::{
    Scrypt, password_hash::{
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng
    }
};

use argon2::password_hash::{self, errors};


pub fn create_scrypt(password : &str) -> Result<String, password_hash::Error>{
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Scrypt.hash_password(password.as_bytes(), &salt)?.to_string();
    let parsed_hash = PasswordHash::new(&password_hash)?.to_string();
    Ok(parsed_hash)
}