use hex;

pub fn encode_as_hex(encrypted : &[u8]) -> String{
    hex::encode(encrypted)
}