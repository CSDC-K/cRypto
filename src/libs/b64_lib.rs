use base64::prelude::*;

pub fn encode_as_b64(encrypted : &[u8]) -> String{
    BASE64_STANDARD.encode(encrypted)
}