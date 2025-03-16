use libslug::slugcrypt::internals::signature::{ed25519::{ED25519PublicKey, ED25519SecretKey, ED25519Signature}, sphincs_plus::{SPHINCSPublicKey,SPHINCSSecretKey,SPHINCSSignature}};
use libslug::slugcrypt::internals::messages::Message;
use libslug::slugcrypt::internals::digest::blake2::SlugBlake2bHasher;
use libslug::slugcrypt::internals::digest::digest::SlugDigest;

use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize, Clone)]
pub struct SigningKeypair {
    pk_sphincs: SPHINCSPublicKey,
    sk_sphincs: SPHINCSSecretKey,

    pk_ed25519: ED25519PublicKey,
    sk_ed25519: ED25519SecretKey,

    id: String,
}

#[derive(Debug,Serialize,Deserialize, Clone)]
pub struct SigningPublicKeys {
    pk_sphincs: SPHINCSPublicKey,
    pk_ed25519: ED25519PublicKey,
    id: String,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Signature {
    sig_ed25519: ED25519Signature,
    sig_sphincs: SPHINCSSignature,
}

impl SigningKeypair {
    pub fn new() -> Self {
        let sphincs_keypair = SPHINCSSecretKey::generate();
        let ed25519_secret = ED25519SecretKey::generate();
        let ed25519_public = ed25519_secret.public_key().unwrap();

        let fp = Self::fingerprint(&sphincs_keypair.0, &ed25519_public);
        
        Self {
            pk_sphincs: sphincs_keypair.0,
            sk_sphincs: sphincs_keypair.1,

            pk_ed25519: ed25519_public,
            sk_ed25519: ed25519_secret,

            id: fp,
        }
    }
    pub fn sign<T: AsRef<[u8]>>(&self, data: T) -> Signature {
        let sig = self.sk_ed25519.sign(data.as_ref()).unwrap();
        let sig_sphincs = self.sk_sphincs.sign(Message::new(data.as_ref())).unwrap();

        return Signature {
            sig_ed25519: sig,
            sig_sphincs: sig_sphincs
        }
    }
    fn fingerprint(pk: &SPHINCSPublicKey, pk_ed25519: &ED25519PublicKey) -> String {
        let fingerprint = format!("{}{}", pk.to_hex_string().unwrap(), pk_ed25519.to_hex_string());
        let hasher = SlugBlake2bHasher::new(20);
        let digest = hasher.hash(fingerprint.as_bytes());
        let output = SlugDigest::from_bytes(&digest).unwrap();
        return output.digest().to_string();
    }
}

impl Signature {
    pub fn verify<T: AsRef<[u8]>>(self, data: T, pk: &SigningPublicKeys) -> std::result::Result<bool, Box<dyn std::error::Error>> {
        let ed25519 = pk.pk_ed25519.verify(self.sig_ed25519, data.as_ref())?;
        let sphincs = pk.pk_sphincs.verify(Message::new(data.as_ref()), self.sig_sphincs)?;

        let id = SigningKeypair::fingerprint(&pk.pk_sphincs, &pk.pk_ed25519);
        
        if id == pk.id && ed25519 == true && sphincs == true {
            return Ok(true);
        }
        else {
            return Ok(false)
        }
    }
}

impl SigningPublicKeys {
    pub fn new(pk_sphincs: SPHINCSPublicKey, pk_ed25519: ED25519PublicKey) -> Self {
        let id = SigningKeypair::fingerprint(&pk_sphincs, &pk_ed25519);
        
        Self {
            pk_sphincs,
            pk_ed25519,
            id: id,
        }
    }
}

#[test]
fn create_key() {
    let keypair = SigningKeypair::new();

    let signingkeys = SigningPublicKeys::new(keypair.pk_sphincs.clone(), keypair.pk_ed25519.clone());


    println!("Key ID: {}", &keypair.id);

    let signature = keypair.sign("Hello World");

    let is_valid = signature.verify("Hello World", &signingkeys).unwrap();

    assert!(is_valid);

    println!("Verification Successful!");

}