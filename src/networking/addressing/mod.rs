pub struct LiuhqAddressing(pub String);


pub mod constants;

use constants::*;







pub struct LiuhqAddressBuilder {
    protocol: String,
    protocol_subgroup: Option<Vec<String>>,

    prefix: String, /// prefix (3-4 chars)
    subgroup: Option<Vec<String>>, /// subgroup (:) of prefix
    
    suffix: String, /// suffix (?) of subgroup
    action: String,
}

impl LiuhqAddressBuilder {
    pub fn new(protocol: String, protocol_subgroup: Option<Vec<String>>) -> Self {
        LiuhqAddressBuilder {
            protocol: String::from("liuhq"),
            protocol_subgroup: None,
            prefix: String::new(),
            subgroup: Some(vec![String::new()]),
            
            suffix: String::new(),
            action: String::new(),
        }
    }
}
