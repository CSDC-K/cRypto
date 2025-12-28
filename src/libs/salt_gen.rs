use std::fmt::format;

use rand::distr::{Alphanumeric, SampleString};
use crate::libs::errors;

pub fn salt_gen(salt_leng : usize) -> Result<String, errors::cRyptoError> {

    if salt_leng > 999999 {
        return Err(errors::cRyptoError::SaltGenError(format!("Memory Error! -> salt len can't more than 999999")));
    }

    let salt = Alphanumeric.sample_string(&mut rand::rng(), salt_leng);
    Ok(salt)
}