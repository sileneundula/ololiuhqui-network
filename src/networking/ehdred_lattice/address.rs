use crate::wallet::naive::*;
use libslug::slugcrypt::internals::digest::sha3::Sha3Hasher;
use libslug::slugcrypt::internals::digest::digest::SlugDigest;


pub struct EhdredAddressBuilder {
    prefix: String,
    subgroup: String,
    suffix: String,
    action: String,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EhdredAddress(pub String);

pub const protocol_char: &str = "[";
pub const protocol_suffix: &str = "]";
pub const protocol_subgroup: &str = "::";


pub const prefix_address_char: &str = "_";
pub const subgroup_char: &str = ":";
pub const suffix_char: &str = "?";
pub const action_char: &str = ".";

impl EhdredAddress {
    pub fn from_keypair(keypair: SigningPublicKeys) -> Self {
        let hasher = Sha3Hasher::new(224);
        let bytes = hasher.digest(&keypair.to_vec());
        let digest = SlugDigest::from_bytes(&bytes).unwrap();
        return EhdredAddress(digest.digest().to_string())
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
    pub fn to_string(&self) -> String {
        self.0.clone()
    }
    pub fn with_prefix(&self, prefix: &str) -> String {
        let mut address = String::new();
        address.push_str(prefix);
        address.push_str()
    }
    pub fn with_subgroup(&self, subgroup: &str) -> String {
        let mut address = String::new();
        address.push_str(&self.0);
        address.push_str(subgroup_char);
        address.push_str(subgroup);
        address
    }
}

impl EhdredAddressBuilder {
    pub fn new() -> Self {
        EhdredAddressBuilder {
            protocol: String::new(), /// protocol [xxx]
            prefix: String::new(), /// prefix (3-4 chars)
            subgroup: String::new(), /// subgroup (:) of prefix
            
            suffix: String::new(), /// suffix (@) of subgroup
            action: String::new(),
        }
    }
    pub fn with_prefix(mut self, prefix: &str) -> Self {
        self.prefix = prefix.to_string();
        self
    }
    pub fn with_subgroup(mut self, subgroup: &str) -> Self {
        self.subgroup = subgroup.to_string();
        self
    }
    pub fn with_suffix(mut self, suffix: &str) -> Self {
        self.suffix = suffix.to_string();
        self
    }
    pub fn with_action(mut self, action: &str) -> Self {
        self.action = action.to_string();
        self
    }
    pub fn build(self) -> EhdredAddress {
        let mut address = String::new();
        address.push_str(&self.prefix);
        address.push_str(&self.subgroup);
        address.push_str(&self.suffix);
        address.push_str(&self.action);
        EhdredAddress(address)
    }
}


#[test]
fn create_ehdredaddress() {
    let keypair = SigningKeypair::new();
    let spk = keypair.to_signing_public_keys();

    let address = EhdredAddress::from_keypair(spk);

    println!("Address: {}", address.as_str());

}