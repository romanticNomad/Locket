use k256::ecdsa::SigningKey;
use rand::rngs::OsRng;
use sha3::{Digest, Keccak256};
use hex::encode;
use std::{fs, path::Path};

use crate::serialize::EvmKeyExport;

#[allow(dead_code)]
mod serialize;

fn main() -> std::io::Result<()> {
    // private key
    let signing_key = SigningKey::random(&mut OsRng);
    let verify_key = signing_key.verifying_key();

    let private_key_bytes = signing_key.to_bytes();
    let private_key_hex = format!("0x{}", encode(private_key_bytes));

    // Uncompressed public key (0x04 || X || Y)
    let public_key = verify_key.to_encoded_point(false);
    let pubkey_bytes = public_key.as_bytes();

    // Ethereum address
    let hash = Keccak256::digest(&pubkey_bytes[1..]);
    let address_hex = format!("0x{}", encode(&hash[12..]));

    // Export to JSON
    let path = "accounts.json";
    let export = serialize::EvmKeyExport::new(private_key_hex, address_hex);

    let mut entries: Vec<EvmKeyExport> = if Path::new(path).exists() {
        let data = fs::read_to_string(path)?;
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        Vec::new()
    };
    entries.push(export);

    let contents = serde_json::to_string_pretty(&entries)?;
    fs::write(path,contents)?;

    println!("evm account details written to {}", path);

    Ok(())
}
