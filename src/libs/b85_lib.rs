use base85;


pub fn encode_as_b85(encrypted : &[u8]) -> String{
    base85::encode(encrypted)
}