use crate::wallet::naive::SigningPublicKeys;
use crate::wallet::naive::Signature;

type LinkedHash = String;

pub mod address;

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
    id: u64,
    prev_hash: LinkedHash,

    owner: address::EhdredAddress,

    address: address::EhdredAddress,

    // Crypto
    keys: SigningPublicKeys,

}