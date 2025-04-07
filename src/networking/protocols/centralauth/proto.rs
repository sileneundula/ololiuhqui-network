pub struct CentralAuthProtocol;

impl CentralAuthProtocol {
    pub fn new() -> Self {
        CentralAuthProtocol
    }
}

pub struct CentralAuthService {
    pub service_name: String,
    pub service_id: u32,
    pub service_address: String,
    pub service_keys: Vec<u8>,
    pub service_signature: Vec<u8>,
    pub service_keypairs: Vec<u8>,
}