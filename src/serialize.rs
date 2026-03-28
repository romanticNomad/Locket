use hex::encode;
use k256::ecdsa::SigningKey;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};

// ===========================================================

/// SerDe format for private key, public key, and public address.
#[derive(Serialize, Deserialize)]
pub struct EvmKeyExport {
    pvt_key: String,
    pub_key: String,
    address: String,
}

impl EvmKeyExport {
    /// Creates a new instance of `EvmKeyExport`.
    pub fn new(pvt_key: String, pub_key: String, address: String) -> EvmKeyExport {
        EvmKeyExport {
            pvt_key,
            pub_key,
            address,
        }
    }
}

// ===========================================================

/// Generates a new random Ethereum account.
///
/// Returns the `EvmKeyExport` for the generated keys.
pub fn generate_account() -> EvmKeyExport {
    // generate the pvt key and encode to hex
    let signing_key = SigningKey::random(&mut OsRng);
    let verify_key = signing_key.verifying_key();

    let private_key_bytes = signing_key.to_bytes();
    let private_key_hex = format!("0x{}", encode(private_key_bytes));

    // Uncompressed public key (0x04 || X || Y)
    let public_key = verify_key.to_encoded_point(false);
    
    let pubkey_bytes = public_key.as_bytes();
    let public_key_hex = format!("0x{}", encode(pubkey_bytes));

    // Ethereum address
    let hash = Keccak256::digest(&pubkey_bytes[1..]);
    let address_hex = format!("0x{}", encode(&hash[12..]));

    EvmKeyExport::new(private_key_hex, public_key_hex, address_hex)
}

// ===========================================================
