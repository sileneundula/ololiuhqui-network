use std::collections::HashMap;

pub mod consensus;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum StorageCommand {
    Put(String, String),
    Get(String),
}


#[derive(Debug, Default)]
pub struct KvStore {
    data: HashMap<String, String>,
}

impl KvStore {
    pub fn apply(&mut self, cmd: StorageCommand) -> Option<String> {
        match cmd {
            StorageCommand::Put(key, value) => {
                self.data.insert(key, value);
                None
            }
            StorageCommand::Get(key) => self.data.get(&key).cloned(),
        }
    }
}
