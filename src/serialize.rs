use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct EvmKeyExport {
    pvt_key: String,
    pub_key: String,
    address: String,
}

impl EvmKeyExport {
    pub fn new(pvt_key: String, pub_key: String, address: String) -> EvmKeyExport {
        EvmKeyExport { pvt_key, pub_key, address }
    }
}
