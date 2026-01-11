use k256::ecdsa::SigningKey;
use rand::rngs::OsRng;
use sha3::{Digest, Keccak256};
use hex::encode;
use std::{fs::File, io::Write};

#[allow(dead_code)]
mod serialize;

fn main() -> std::io::Result<()> {
    // Generate private key
    let signing_key = SigningKey::random(&mut OsRng);
    let verify_key = signing_key.verifying_key();

    // Pvt key bytes
    let private_key_bytes = signing_key.to_bytes();
    let private_key_hex = format!("0x{}", encode(private_key_bytes));

    // Uncompressed public key (0x04 || X || Y)
    let public_key = verify_key.to_encoded_point(false);
    let pubkey_bytes = public_key.as_bytes();

    // Ethereum address
    let hash = Keccak256::digest(&pubkey_bytes[1..]);
    let address_hex = format!("0x{}", encode(&hash[12..]));

    // Export to JSON
    let export = serialize::EvmKeyExport::new(private_key_hex, address_hex);
    let json = serde_json::to_string_pretty(&export)?;

    let mut file = File::create("account.json")?;
    file.write_all(json.as_bytes())?;

    println!("evm account details written to account.json");

    Ok(())
}
