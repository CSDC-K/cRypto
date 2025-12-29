use crate::libs::argon2_lib;        // ARGON        ENCRYPTING
use crate::libs::ascon_xof_lib;     // ASCON XOF    ENCRYPTING
use crate::libs::ascon_hash_lib;    // ASCON HASH   ENCRYPTING
use crate::libs::baloon_hash_lib;   // BALLOON HASH ENCRYPTING
use crate::libs::scrypt_lib;        // SCRYPT       ENCRYPTING
use crate::libs::cRyptit;           // CRYPTIT      ENCRYPTING
use crate::libs::hex_lib;           // HEX          ENCODING
use crate::libs::b58_lib;           // BASE58       ENCODING
use crate::libs::b64_lib;           // BASE64       ENCODING
use crate::libs::b85_lib;           // BASE85       ENCODING
use crate::libs::bin_lib;           // BINARY       ENCODING
use crate::libs::errors;            // ERRORS       ERROR TYPES

pub fn build_crypted(encr_method : &str, enco_type : &str, password : &str, salt_len : usize) -> Result<String, errors::cRyptoError>{

    let enco_types : Vec<&str> = vec!["B64", "HEX", "B85", "B58", "BIN"];
    let encr_methods : Vec<&str> = vec!["ARGON2", "ASCONHASH", "ASCONXOF", "CRYPTIT", "BALLOONHASH", "SCRYPT"];

    if !enco_types.contains(&enco_type){
        return Err(errors::cRyptoError::EncoType(format!("ENCO_TYPE_ERROR! -> Not Founded Encode Type")));      // Not founded encode tpye
    }

    if !encr_methods.contains(&encr_method){
        return Err(errors::cRyptoError::EncrMethod(format!("ENCR_METHOD_ERROR! -> Not Founded Encrypt Type")));      // Not founded encryption method
    }

    let encrypted : Vec<u8> = match encr_method {
        "ARGON2" => argon2_lib::create_argon2(password, salt_len)?,
        "ASCONXOF" => ascon_xof_lib::create_asconXof(password, salt_len)?,
        "ASCONHASH" => ascon_hash_lib::create_asconHash(password, salt_len)?,
        "CRYPTIT" => cRyptit::_func_get_cpu_info(password, salt_len)?,
        "BALLOONHASH" => {baloon_hash_lib::create_balloonHash(password)
        .map_err(|e| errors::cRyptoError::Unknown(e.to_string()))? // using map err bcz balloon_hash lib uses another error type. We have to change error type into our error type.
        .into_bytes()},
        "SCRYPT" => {scrypt_lib::create_scrypt(password)
        .map_err(|e| errors::cRyptoError::Unknown(e.to_string()))? // using map err bcz scrypt lib uses another error type. We have to change error type into our error type.
        .into_bytes()},
        _ => return Err(errors::cRyptoError::Unknown("MATCH OUT FROM SCOPE! // ENCR_TYPE".to_string()))
    };
    let encoded : String = match enco_type {
        "HEX" => hex_lib::encode_as_hex(&encrypted),
        "B58" => b58_lib::encode_as_b58(&encrypted),
        "B64" => b64_lib::encode_as_b64(&encrypted),
        "B85" => b85_lib::encode_as_b85(&encrypted),
        "BIN" => bin_lib::encode_as_bin(&encrypted),
        _ => return Err(errors::cRyptoError::Unknown("MATCH OUT FROM SCOPE! // ENCO_TYPE".to_string()))
    };

    

    Ok(encoded)                       // Encrypted and encoded password return


}

