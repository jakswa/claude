use orion::aead;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let pass = std::env::var("SECRET_KEY").expect("expected SECRET_KEY env var");
    let secret_key = aead::SecretKey::from_slice(pass.as_bytes()).unwrap();
    let input = std::fs::read_to_string("policies.toml").expect("policies.toml needs to exist");
    let ciphertext = aead::seal(&secret_key, input.as_bytes()).unwrap();

    let mut file = File::create("policies").expect("could not create 'policies' file");
    file.write_all(&ciphertext[..])
        .expect("could not write to file");
}
