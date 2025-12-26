use std::collections::HashMap;

pub fn build_crypted(encr_method : &str, enco_type : &str, password : &str) -> String{

    let mut enco_hashes = HashMap::new();
    enco_hashes.insert("B64", b64_fn());
    enco_hashes.insert("HEX", hex_fn());
    
}

fn b64_fn() {}
fn hex_fn() {}

