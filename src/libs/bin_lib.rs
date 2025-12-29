pub fn encode_as_bin(encrypted: &[u8]) -> String {
    encrypted
        .iter()
        .map(|byte| format!("{:01b}", byte))
        .collect::<String>()
}