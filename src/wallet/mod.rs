use libslug::slugcrypt::internals::signature;
use libslug::slugcrypt::internals::encryption;

pub type Identity = String;
pub type LocalName = String;

use std::collections::HashMap;

pub struct SlabWallet {
    local_names: Vec<LocalName>,
    identities: HashMap<LocalName, Identity>,
    
    signer_wallets: HashMap<Identity, IdentityWallet>,
    encrypter_wallets: HashMap<Identity, IdentityWallet>,
}

pub struct IdentityWallet {
    algorithm: String,
    pk: String,
    sk: String,
    keypair_type: KeypairType,
}

pub enum KeypairType {
    Signer,
    Encrypter,
}

impl SlabWallet {
    pub fn new() -> Self {
        Self
    }
    pub fn add_key(&mut self, local_name: LocalName, identity: Identity, wallet: IdentityWallet) {
        self.local_names.push(local_name);
        self.identities.insert(local_name, identity);
        self.signer_wallets.insert(identity, IdentityWallet::new(algorithm, pk, sk));
        self.encrypter_wallets.insert(identity, IdentityWallet::new(algorithm, pk, sk));
    }
}

impl IdentityWallet {
    pub fn new(algorithm: String, keypair_type: KeypairType, pk: String, sk: String) -> Self {
        Self { algorithm, pk, sk, keypair_type }
    }

}