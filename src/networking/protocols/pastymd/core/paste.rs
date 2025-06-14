use crate::wallet::naive::{SigningPublicKeys,Signature};

pub struct PasteMDContents {
    // Verification
    pub publickeys: SigningPublicKeys,
    pub paste_md_csprng: [u8; 64],

    pub signature: Signature,

    pub integrity: String,
    
    // Content
    title: String,
    content: String,
    tags: Vec<String>,
}

impl PasteMDContents {
    pub fn new(title: String, content: String, tags: Vec<String>, publickeys: SigningPublicKeys, signature: Signature, integrity: String, paste_md_csprng: [u8;64]) -> Self {
        PasteMDContents {
            title,
            content,
            tags,
            publickeys,
            signature,
            integrity,
            paste_md_csprng,
        }
    }

    pub fn get_title(&self) -> &String {
        &self.title
    }

    pub fn get_content(&self) -> &String {
        &self.content
    }

    pub fn get_tags(&self) -> &Vec<String> {
        &self.tags
    }
}