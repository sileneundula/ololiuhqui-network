use libp2p::{core, identity::{self, DecodingError, KeyType}, noise, tcp, websocket, yamux, PeerId, Transport};
use serde::{Serialize,Deserialize};
use pkcs8::EncodePrivateKey;

#[derive(Debug, Clone)]
pub struct LiuhqiKeypair(identity::Keypair);

impl Serialize for LiuhqiKeypair {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        match self.to_protobuf() {
            Ok(bytes) => serializer.serialize_bytes(&bytes),
            Err(e) => Err(serde::ser::Error::custom(e)),
        }
    }
}

impl<'de> Deserialize<'de> for LiuhqiKeypair {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bytes: Vec<u8> = Deserialize::deserialize(deserializer)?;
        let keypair = identity::Keypair::from_protobuf_encoding(&bytes)
            .map_err(serde::de::Error::custom)?;
        Ok(LiuhqiKeypair(keypair))
    }
}

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
    pub fn to_protobuf(&self) -> Result<Vec<u8>, DecodingError> {
        self.0.to_protobuf_encoding()
    }
    pub fn from_protobuf(&self, bytes: &[u8]) -> Result<identity::Keypair, DecodingError> {
        identity::Keypair::from_protobuf_encoding(bytes)
    }
    pub fn keypair(&self) -> identity::Keypair {
        self.0.clone()
    }
    pub fn from_keypair(keypair: identity::Keypair) -> Self {
        Self(keypair)
    }
    pub fn keypair_type(&self) -> KeyType {
        self.0.key_type()
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

