use std::collections::BTreeMap;

use crate::persist_io::TableFile;

pub struct InitFromArchiveContent {
    pub file: TableFile,
    pub content: Vec<u8>,
}

pub struct InitFromArchiveState {
    pub to_load: BTreeMap<String, Vec<InitFromArchiveContent>>,
    pub loaded: Vec<String>,
}

impl InitFromArchiveState {
    pub fn new() -> Self {
        Self {
            to_load: BTreeMap::new(),
            loaded: Vec::new(),
        }
    }
}
