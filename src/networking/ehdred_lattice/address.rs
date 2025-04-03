use crate::wallet::naive::*;
use libslug::slugcrypt::internals::digest::sha3::Sha3Hasher;
use libslug::slugcrypt::internals::digest::digest::SlugDigest;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EhdredAddress(pub String);

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
}


#[test]
fn create_ehdredaddress() {
    let keypair = SigningKeypair::new();
    let spk = keypair.to_signing_public_keys();

    let address = EhdredAddress::from_keypair(spk);

    println!("Address: {}", address.as_str());

}