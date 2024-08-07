use rust_extensions::sorted_vec::{EntityWithStrKey, InsertOrUpdateEntry, SortedVecWithStrKey};

use super::RawData;

pub struct PartitionData {
    pub partition_key: String,
    pub rows: SortedVecWithStrKey<RawData>,
}

impl PartitionData {
    pub fn new(partition_key: String) -> Self {
        Self {
            partition_key,
            rows: SortedVecWithStrKey::new(),
        }
    }

    pub fn insert_or_update(&mut self, row_key: String, content: String) {
        match self.rows.insert_or_update(&row_key) {
            InsertOrUpdateEntry::Insert(entry) => {
                entry.insert(RawData::new(row_key, content));
            }
            InsertOrUpdateEntry::Update(entry) => {
                entry.item.content = content;
            }
        }
    }
}

impl EntityWithStrKey for PartitionData {
    fn get_key(&self) -> &str {
        &self.partition_key
    }
}
