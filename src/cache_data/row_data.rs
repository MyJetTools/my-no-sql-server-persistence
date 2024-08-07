use rust_extensions::sorted_vec::EntityWithStrKey;

pub struct RawData {
    pub row_key: String,
    pub content: String,
}

impl RawData {
    pub fn new(row_key: String, content: String) -> Self {
        Self { row_key, content }
    }
}

impl EntityWithStrKey for RawData {
    fn get_key(&self) -> &str {
        &self.row_key
    }
}
