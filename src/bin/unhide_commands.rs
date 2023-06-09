use orion::aead;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let pass = std::env::var("SECRET_KEY").expect("expected SECRET_KEY env var");
    let secret_key = aead::SecretKey::from_slice(pass.as_bytes()).unwrap();
    let ciphertext = std::fs::read(".commands").expect("'.commands' file needs to exist");
    let decrypted_data = aead::open(&secret_key, &ciphertext).unwrap();

    let mut file = File::create("commands.toml").expect("could not create 'commands.toml' file");
    file.write_all(&decrypted_data[..])
        .expect("could not write to file");
}
