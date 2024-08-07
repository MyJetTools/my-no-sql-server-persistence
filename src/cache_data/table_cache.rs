use rust_extensions::sorted_vec::*;

use crate::serializers::TableMetadataFileContract;

use super::PartitionData;

pub struct TableCache {
    pub meta_data: TableMetadataFileContract,
    pub partitions: SortedVecWithStrKey<PartitionData>,
}

impl TableCache {
    pub fn new(meta_data: TableMetadataFileContract) -> Self {
        Self {
            meta_data,
            partitions: SortedVecWithStrKey::new(),
        }
    }

    pub fn insert_or_update(&mut self, partition_key: String, row_key: String, content: String) {
        match self.partitions.insert_or_update(&partition_key) {
            InsertOrUpdateEntry::Insert(entry) => {
                let mut partition_data = PartitionData::new(partition_key);
                partition_data.insert_or_update(row_key, content);
                entry.insert(partition_data);
            }
            InsertOrUpdateEntry::Update(entry) => entry.item.insert_or_update(row_key, content),
        }
    }
}
