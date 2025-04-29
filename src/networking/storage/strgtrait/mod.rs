pub trait StorageKV {
    fn get(&self, key: &str) -> Option<String>;
    fn set(&mut self, key: String, value: String);
    fn remove(&mut self, key: &str);
}