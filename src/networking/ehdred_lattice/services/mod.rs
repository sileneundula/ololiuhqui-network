pub struct EhdredLatticeService {
    // Define the fields for the EhdredLatticeService struct here
    service_name: String,
    service_id: u32,
    service_address: String,
    service_keys: Vec<u8>,
    service_signature: Vec<u8>,
    service_keypairs: Vec<u8>,
}

pub enum EhdredServices {
    Registar = 0,
    
}