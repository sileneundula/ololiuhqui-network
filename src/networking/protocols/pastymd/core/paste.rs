pub struct PasteMDContents {
    pub id: String,
    pub author: String,
    pub content: String,
}

impl PasteMDContents {
    pub fn new(id: String, author: String, content: String) -> Self {
        Self { id, author, content }
    }
}