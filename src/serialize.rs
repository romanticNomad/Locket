use serde::Serialize;

#[derive(Serialize)]
pub struct EvmKeyExport {
    private_key: String,
    address: String,
}

impl EvmKeyExport {
    pub fn new(pvtkey: String, pubkey: String) -> EvmKeyExport {
        EvmKeyExport { private_key: pvtkey, address: pubkey }
    }
}
