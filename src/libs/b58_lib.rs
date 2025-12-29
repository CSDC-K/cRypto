use bs58;


pub fn encode_as_b58(encrypted : &[u8]) -> String {
    let a = bs58::encode(encrypted);
    a.into_string()
}