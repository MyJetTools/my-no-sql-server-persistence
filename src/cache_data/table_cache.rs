use my_no_sql_sdk::core::db::DbTableAttributes;
use rust_extensions::sorted_vec::*;

use crate::persist_queue::PersistQueue;

use super::PartitionData;

pub struct TableCache {
    pub meta_data: DbTableAttributes,
    pub partitions: SortedVecWithStrKey<PartitionData>,
    pub persist_queue: PersistQueue,
}

impl TableCache {
    pub fn new(meta_data: DbTableAttributes) -> Self {
        Self {
            meta_data,
            partitions: SortedVecWithStrKey::new(),
            persist_queue: PersistQueue::new(),
        }
    }

    pub fn insert_or_update(&mut self, partition_key: &str, row_key: &str, content: String) {
        match self.partitions.insert_or_update(partition_key) {
            InsertOrUpdateEntry::Insert(entry) => {
                let mut partition_data = PartitionData::new(partition_key.to_string());
                partition_data.insert_or_update(row_key, content);
                entry.insert(partition_data);
            }
            InsertOrUpdateEntry::Update(entry) => entry.item.insert_or_update(row_key, content),
        }
    }
}
