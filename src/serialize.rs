use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct EvmKeyExport {
    pvt_key: String,
    address: String,
}

impl EvmKeyExport {
    pub fn new(pvtkey: String, pubkey: String) -> EvmKeyExport {
        EvmKeyExport { pvt_key: pvtkey, address: pubkey }
    }
}
