use crate::libs::argon2_lib;
use crate::libs::hex_lib;
use crate::libs::errors;

pub fn build_crypted(encr_method : &str, enco_type : &str, password : &str, salt_len : usize) -> Result<String, errors::cRyptoError>{

    let enco_types : Vec<&str> = vec!["B64", "HEX"];
    let encr_methods : Vec<&str> = vec!["ARGON2", "ASCON-HASH"];

    if !enco_types.contains(&enco_type){
        return Err(errors::cRyptoError::EncoType(format!("ENCO_TYPE_ERROR! -> Not Founded Encode Type")));      // Not founded encode tpye
    }

    if !encr_methods.contains(&encr_method){
        return Err(errors::cRyptoError::EncrMethod(format!("ENCR_METHOD_ERROR! -> Not Founded Encrypt Type")));      // Not founded encryption method
    }

    let encrypted : Vec<u8> = match encr_method {
        "ARGON2" => argon2_lib::create_argon2(password, salt_len)?,
        _ => return Err(errors::cRyptoError::Unknown("MATCH OUT FROM SCOPE! // ENCR_TYPE".to_string()))
    };
    let encoded : String = match enco_type {
        "HEX" => hex_lib::encode_as_hex(&encrypted),
        _ => return Err(errors::cRyptoError::Unknown("MATCH OUT FROM SCOPE! // ENCO_TYPE".to_string()))
    };

    

    Ok(encoded)                       // Encrypted and encoded password return


}

