pub struct StrgAddr(pub String);

impl StrgAddr {
    pub fn new(addr: String) -> Self {
        StrgAddr(addr)
    }
}