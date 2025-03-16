use libslug::slugcrypt::internals::signature::{ed25519::{ED25519PublicKey, ED25519SecretKey, ED25519Signature}, sphincs_plus::{SPHINCSPublicKey,SPHINCSSecretKey,SPHINCSSignature}};
use libslug::slugcrypt::internals::messages::Message;
use libslug::slugcrypt::internals::digest::blake2::SlugBlake2bHasher;
use libslug::slugcrypt::internals::digest::digest::SlugDigest;

pub struct SigningKeypair {
    pk_sphincs: SPHINCSPublicKey,
    sk_sphincs: SPHINCSSecretKey,

    pk_ed25519: ED25519PublicKey,
    sk_ed25519: ED25519SecretKey,

    fingerprint: String,
}

pub struct SigningPublicKeys {
    pk_sphincs: SPHINCSPublicKey,
    pk_ed25519: ED25519PublicKey,
}

pub struct Signature {
    sig_ed25519: ED25519Signature,
    sig_sphincs: SPHINCSSignature,
}

impl SigningKeypair {
    pub fn new() -> Self {
        let sphincs_keypair = SPHINCSSecretKey::generate();
        let ed25519_secret = ED25519SecretKey::generate();
        let ed25519_public = ed25519_secret.public_key().unwrap();

        
        
        Self {
            pk_sphincs: sphincs_keypair.0,
            sk_sphincs: sphincs_keypair.1,

            pk_ed25519: ed25519_public,
            sk_ed25519: ed25519_secret,
        }
    }
    pub fn sign<T: AsRef<[u8]>>(&self, data: T) -> Signature {
        let sig = self.sk_ed25519.sign(data.as_ref()).unwrap();
        let sig_sphincs = self.sk_sphincs.sign(Message::new(data.as_ref())).unwrap();

        println!("ED25519 Signature: {}", sig.to_hex_string());
        println!("SPHINCS+ Signature: {}", sig_sphincs.to_base58_string());

        return Signature {
            sig_ed25519: sig,
            sig_sphincs: sig_sphincs
        }
    }
    fn fingerprint(pk: SPHINCSPublicKey, sk: ED25519PublicKey) -> String {
        let fingerprint = format!("{}{}", pk.to_hex_string().unwrap(), sk.to_hex_string());
        let hasher = SlugBlake2bHasher::new(20);
        let digest = hasher.hash(fingerprint.as_bytes());
        let output = SlugDigest::from_bytes(&digest).unwrap();
        return output.digest().to_string();
    }
}

#[test]
fn create_key() {
    let keypair = SigningKeypair::new();
    keypair.sign("Hello World");
}