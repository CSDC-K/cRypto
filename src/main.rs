
mod libs;

use clap::{Parser};
use libs::builder;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about = "Rust Based cRypter Tool", long_about = "Basically this cli arg program using encrypt method to encrypt your password for security. (Easy Cli Tool)")]
struct Args {
    
    #[arg(short, long, default_value_t = String::from("ARGON2"), help="Encrypting Method\nValues: ARGON2, ASCON-HASH")]
    enc_method: String,
    #[arg(long, default_value_t = String::from("B64"), help="Encoding type (Byte -> String) !NOTE: Encode will note create\nproblem for your password security.")]
    encode_type: String,
    #[arg(short, long, default_value_t = String::from("cRypto"))]
    pass: String
}

fn main() {
    let args = Args::parse();

}