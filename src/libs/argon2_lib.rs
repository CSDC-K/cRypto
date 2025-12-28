use argon2::Argon2;
use crate::libs::salt_gen;
use crate::libs::errors;

pub fn create_argon2(password : &str, salt_len : usize) -> Result<Vec<u8>, errors::cRyptoError>{
    
    let salt = salt_gen::salt_gen(salt_len)?;

    let mut output_key_material = vec![0u8; 32]; // Can be any desired size
    Argon2::default().hash_password_into(password.as_bytes(), salt.as_bytes(), &mut output_key_material);
    Ok(output_key_material)    
}