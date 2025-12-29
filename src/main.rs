
mod libs;

use clap::{Parser};
use libs::builder;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about = "Rust Based cRypter Tool")]
struct Args {
    #[arg(short, long, default_value_t = String::from("ARGON2"), value_name = "METHOD", help="Encrypting Method\nValues: ARGON2, ASCONHASH, ASCONXOF, CRYPTIT, BALLOONHASH")]
    enc_method: String,
    #[arg(short = 'n',long, value_name = "TYPE", default_value_t = String::from("HEX"), help="Encoding type (Byte -> String) !NOTE: Encode will note create\nproblem for your password security.")]
    encode_type: String,
    #[arg(short, long,value_name = "SALT LENGTH", default_value_t = 16, help="Random Salt Lenght.")]
    salt_len : usize,
    #[arg(short, long,value_name = "PASSWORD", default_value_t = String::from("cRypto"))]
    pass: String
}

fn main() {
    let args = Args::parse();
    match builder::build_crypted(&args.enc_method, &args.encode_type, &args.pass, args.salt_len) {
        Ok(encr) => println!("{}", encr),
        Err(errcode) => println!("!{}", errcode)
    }

}