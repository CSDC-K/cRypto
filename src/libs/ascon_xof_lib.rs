use ascon_hash::{AsconXof128, ExtendableOutput, Update, XofReader};
use crate::libs::errors;
use crate::libs::salt_gen;

pub fn create_asconXof(password : &str, salt_len : usize) -> Result<Vec<u8>, errors::cRyptoError>{
    let mut xof = AsconXof128::default();
    if salt_len >= 1 {
        let salt: String = salt_gen::salt_gen(salt_len)?;
        xof.update(salt.as_bytes());
    }

    
    xof.update(password.as_bytes());
    let mut reader = xof.finalize_xof();
    let mut dst = [0u8; 5];
    reader.read(&mut dst);
    Ok(dst.to_vec())
}
