pub struct ImxStrg {
    pub id: u64,
    pub name: String,
    pub address: String,
    pub keys: Vec<u8>,
    pub sig: Vec<u8>,
    pub keypairs: Vec<u8>,
}

pub struct ImxRequest {
    pub action: u8,

}

pub enum ImxAction {
    Request = 0,
    Response = 1,
    RegisterRequest = 2,
    
    // Register
    RegisterResponse = 3,
    RegisterError = 4,
    RegisterSuccess = 5,
    Error = 8,
}