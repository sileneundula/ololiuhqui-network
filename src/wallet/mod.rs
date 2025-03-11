use libslug::slugcrypt::internals::signature;
use libslug::slugcrypt::internals::encryption;
use libslug::slugcrypt::internals::signature::ed25519::ED25519PublicKey;
use libslug::slugcrypt::internals::signature::ed25519::ED25519SecretKey;
use libslug::slugcrypt::internals::signature::ed25519::ED25519Signature;
use libslug::slugcrypt::internals::signature::schnorr::SchnorrPublicKey;
use libslug::slugcrypt::internals::signature::schnorr::SchnorrSecretKey;
use libslug::slugcrypt::internals::signature::sphincs_plus::SPHINCSSecretKey;
use libslug::slugcrypt::internals::signature::sphincs_plus::SPHINCSPublicKey;

use libslug::slugcrypt::internals::encryption::ecies::{ECPublicKey, ECSecretKey};

pub type Identity = String;
pub type LocalName = String;

pub mod naive;

use std::collections::HashMap;

pub struct KeyPair {
    pub alg: u8,
    pub is_encryption: bool,

    pub pk: Vec<u8>,
    pub sk: Vec<u8>,
}

pub enum KeypairAlgorithms {
    ED25519,
    Schnorr,
    SPHINCSPLUS,
    
    ECIES,
}
/*

impl KeyPair {
    pub fn new(alg: u8, is_encryption: bool, pk: Vec<u8>, sk: Vec<u8>) -> Self {
        Self { alg, is_encryption, pk, sk }
    }
    fn to_ed25519(&self) -> (ED25519PublicKey,ED25519SecretKey) {
        let pk = ED25519PublicKey::from_bytes(&self.pk.as_bytes()).unwrap();
        let sk = ED25519SecretKey::from_bytes(&self.sk.as_bytes()).unwrap();
        (pk,sk)
    }
    fn to_schnorr(&self) -> (SchnorrPublicKey,SchnorrSecretKey) {
        let pk = SchnorrPublicKey::from_bytes(&self.pk.as_bytes()).unwrap();
        let sk = SchnorrSecretKey::from_bytes(&self.sk.as_bytes()).unwrap();
        (pk,sk)
    }
    fn to_sphincs_plus(&self) -> (SPHINCSPublicKey,SPHINCSSecretKey) {
        let pk = SPHINCSPublicKey::from_bytes(&self.pk.as_bytes()).unwrap();
        let sk = SPHINCSSecretKey::from_bytes(&self.sk.as_bytes()).unwrap();
        (pk,sk)
    }
    fn to_ecies(&self) -> (ECPublicKey,ECSecretKey) {
        let pk = ECPublicKey::from_bytes(&self.pk.as_bytes()).unwrap();
        let sk = ECSecretKey::from_bytes(&self.sk.as_bytes()).unwrap();
        (pk,sk)
    }
    pub fn sign<T: AsRef<[u8]>>(&self, msg: T) -> String {
        match self.alg {
            0 => {
                let (pk,sk) = self.to_ed25519();
                sk.sign(msg.as_ref()).unwrap().to_hex_string()
            },
            1 => {
                let (pk,sk) = self.to_schnorr();
                sk.sign_with_slugcrypt(msg.as_ref()).unwrap().to_hex_string().unwrap()
            },
            2 => {
                let (pk,sk) = self.to_sphincs_plus();
                let message = signature::sphincs_plus::;
                sk.sign(message).unwrap().to_hex_string().unwrap()
            },
            _ => panic!("Invalid algorithm")
        }
    }
    pub fn verify<T: AsRef<[u8]>>(&self, msg: T, sig: &str) -> bool {
        match self.alg {
            0 => {
                let (pk,sk) = self.to_ed25519();
                let signature = ED25519Signature::from_hex_string(sig).unwrap();
                let signature_bytes = ED25519Signature::from_bytes(&signature).unwrap();
                pk.verify(signature_bytes, msg.as_ref()).unwrap()
            },
            1 => {
                let (pk,sk) = self.to_schnorr();
                pk.verify_with_context(msg.as_ref(), b"SlugCrypt", SchnorrSignature::from_hex_string(sig).unwrap()).unwrap()
            },
            2 => {
                let (pk,sk) = self.to_sphincs_plus();
                pk.verify(msg.as_ref(), &sig.from_hex_string()).unwrap()
            },
            _ => panic!("Invalid algorithm")
        }
    }
}

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

pub enum KeypairAlgorithms {
    SIG_Ed25519,
    SIG_Schnorr,
    SIG_SPHINCS,
    
    ENC_ECIES,

}

impl SlabWallet {
    pub fn new() -> Self {
        Self
    }
    pub fn add_key(&mut self, local_name: LocalName, identity: Identity, wallet: IdentityWallet) {
        self.local_names.push(local_name);
        self.identities.insert(local_name, identity);
        //self.signer_wallets.insert(identity, IdentityWallet::new(algorithm, pk, sk));
        //self.encrypter_wallets.insert(identity, IdentityWallet::new(algorithm, pk, sk));
    }
}

impl IdentityWallet {
    pub fn new(algorithm: String, keypair_type: KeypairType, pk: String, sk: String) -> Self {
        Self { algorithm, pk, sk, keypair_type }
    }

}
    */