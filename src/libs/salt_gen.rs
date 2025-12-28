use rand::distr::{Alphanumeric, SampleString};

pub fn salt_gen(salt_leng : usize) -> String{
    let salt = Alphanumeric.sample_string(&mut rand::rng(), salt_leng);
    salt
}