extern crate mimir_crypto;
extern crate rand;

use mimir_crypto::secp256k1::Signer;


fn main() {
    let signer: Signer = rand::random();
    println!(r#"public = "{:?}""#,signer.public());
    println!(r#"secret = "{:?}""#,signer.secret());
    println!(r#"address = "{:?}""#,signer.address());
}

