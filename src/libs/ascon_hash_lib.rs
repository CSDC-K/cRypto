use ascon_hash::{AsconHash256, Digest};
use crate::libs::salt_gen;
use crate::libs::errors;

pub fn create_asconHash(password : &str, salt_len : usize) -> Result<Vec<u8>, errors::cRyptoError>{
    let mut hasher = AsconHash256::new();

    if salt_len >= 1 {
        let salt: String = salt_gen::salt_gen(salt_len)?;
        hasher.update(salt.as_bytes());
    }

    hasher.update(password.as_bytes());
    let digest = hasher.finalize();
    Ok(digest.to_vec())
}