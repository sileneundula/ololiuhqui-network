use crate::wallet::naive::SigningPublicKeys;
use crate::wallet::naive::Signature;

type LinkedHash = String;

pub mod address;
pub mod services;

pub struct EhdredRegistarInit {
    id: u64,
    service_name: String,
    service_address: address::EhdredAddress,
}

pub struct EhdredLatticeInit {
    id: u64,
    common_name: String,
    
    // Lattice
    pk: SigningPublicKeys,
    sig: Signature,

    address: address::EhdredAddress,

    keypairs: Vec<SigningPublicKeys>,
}

pub struct Block {
    owner: address::EhdredAddress,

    id: u64,
    prev_hash: LinkedHash,

    

    address: address::EhdredAddress,

    // Crypto
    keys: SigningPublicKeys,

}

pub struct Transaction {
    id: u64,
    from: address::EhdredAddress,
    to: address::EhdredAddress,
    amount: u64,
    sig: Signature,
}
