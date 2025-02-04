use libp2p::{core, identity, noise, tcp, websocket, yamux, PeerId, Transport};


pub struct LiuhqiKeypair(identity::Keypair);

impl LiuhqiKeypair {
    pub fn generate(algorithm: LiuhqiKeypairAlgorithms) -> Self {
        let keypair: identity::Keypair = match algorithm {
            LiuhqiKeypairAlgorithms::CURVE25519 => identity::Keypair::generate_ed25519(),
            LiuhqiKeypairAlgorithms::SECP256k1 => identity::Keypair::generate_secp256k1(),
            LiuhqiKeypairAlgorithms::ECDSA => identity::Keypair::generate_ecdsa(),
        };

        return Self(keypair)
    }
    pub fn generate_ecdsa() -> Self {
        let keypair = identity::Keypair::generate_ecdsa();

        return Self(keypair)
    }
    pub fn generate_ed25519() -> Self {
        let keypair = identity::Keypair::generate_ed25519();

        return Self(keypair)
    }
    pub fn generate_secp256k1() -> Self {
        let keypair = identity::Keypair::generate_secp256k1();

        return Self(keypair)
    }
    pub fn keypair(&self) -> identity::Keypair {
        self.0.clone()
    }
}

/// # LiuhqiKeypairAlgorithms
/// 
/// Includes:
/// - ECDSA
/// - ED25519 (Curve25519)
/// - SECP256k1
pub enum LiuhqiKeypairAlgorithms {
    ECDSA,
    CURVE25519,
    SECP256k1,
}

