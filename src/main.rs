use k256::ecdsa::SigningKey;
use rand::rngs::OsRng;
use sha3::{Digest, Keccak256};
use hex::encode;

#[allow(dead_code)]
mod serialize;

fn main() {
    // 1. Generate private key
    let signing_key = SigningKey::random(&mut OsRng);
    let verify_key = signing_key.verifying_key();

    // 2. Private key bytes
    let private_key_bytes = signing_key.to_bytes();
    println!("Private key: 0x{}", encode(private_key_bytes));

    // 3. Uncompressed public key (0x04 || X || Y)
    let public_key = verify_key.to_encoded_point(false);
    let pubkey_bytes = public_key.as_bytes();

    // 4. Ethereum address
    let hash = Keccak256::digest(&pubkey_bytes[1..]);
    let address = &hash[12..];

    println!("Address: 0x{}", encode(address));
}
