use std::collections::HashMap;

use my_no_sql_core::db::{db_snapshots::DbPartitionSnapshot, DbTable, DbTableAttributes};
use rust_extensions::date_time::DateTimeAsMicroseconds;

pub enum BlobPartitionUpdateTimeResult {
    Ok(DateTimeAsMicroseconds),
    TableNotFound,
    PartitionNoFound,
}

pub struct PersistedTableDataInner {
    pub attr: DbTableAttributes,
    pub partitions: HashMap<String, DateTimeAsMicroseconds>,
}

impl PersistedTableDataInner {
    pub fn new(attr: DbTableAttributes) -> Self {
        Self {
            attr,
            partitions: HashMap::new(),
        }
    }

    pub fn restore_from_blob(db_table: &DbTable) -> Self {
        let mut partitions = HashMap::new();

        for (partition_key, db_partition) in &db_table.partitions {
            partitions.insert(
                partition_key.to_string(),
                db_partition.get_last_write_moment(),
            );
        }

        Self {
            attr: db_table.attributes.clone(),
            partitions,
        }
    }
}

pub struct PersistedTableData {
    inner: Option<PersistedTableDataInner>,
}

impl PersistedTableData {
    pub fn new() -> Self {
        Self { inner: None }
    }

    pub fn restore_from_blob(db_table: &DbTable) -> Self {
        Self {
            inner: Some(PersistedTableDataInner::restore_from_blob(db_table)),
        }
    }

    pub fn create_table(&mut self, attr: DbTableAttributes) {
        self.inner = Some(PersistedTableDataInner {
            attr,
            partitions: HashMap::new(),
        });
    }

    pub fn get(&self, partition_key: &str) -> BlobPartitionUpdateTimeResult {
        match &self.inner {
            Some(inner) => {
                let result = inner.partitions.get(partition_key);

                if result.is_none() {
                    return BlobPartitionUpdateTimeResult::PartitionNoFound;
                }

                BlobPartitionUpdateTimeResult::Ok(*result.unwrap())
            }
            None => BlobPartitionUpdateTimeResult::TableNotFound,
        }
    }

    pub fn delete_table_partition(&mut self, partition_key: &str) {
        if let Some(table) = &mut self.inner {
            table.partitions.remove(partition_key);
        }
    }

    pub fn update_table_attributes(&mut self, attr: DbTableAttributes) {
        match &mut self.inner {
            Some(inner) => {
                inner.attr = attr;
            }
            None => {
                let inner = PersistedTableDataInner::new(attr);
                self.inner = Some(inner);
            }
        }
    }

    pub fn get_snapshot(&self) -> Option<HashMap<String, DateTimeAsMicroseconds>> {
        let inner = self.inner.as_ref()?;
        let mut result = HashMap::new();

        for (partition, value) in &inner.partitions {
            result.insert(partition.to_string(), *value);
        }

        Some(result)
    }

    pub fn update_table_partition_snapshot_id(
        &mut self,

        partition_key: &str,
        db_partition_snapshot: &DbPartitionSnapshot,
    ) {
        if let Some(table) = &mut self.inner {
            table.partitions.insert(
                partition_key.to_string(),
                db_partition_snapshot.last_write_moment,
            );
        }
    }
}
