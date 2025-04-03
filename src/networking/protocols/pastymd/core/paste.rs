use crate::wallet::naive::{SigningPublicKeys,Signature};

pub struct PasteMDContents {
    // Verification
    pub publickeys: SigningPublicKeys,
    pub signature: Signature,
    pub integrity: String,
    
    // Content
    title: String,
    content: String,
    tags: Vec<String>,
}