use std::{sync::Arc, time::Duration};

use my_no_sql_core::db::{
    db_snapshots::{DbPartitionSnapshot, DbTableSnapshot},
    DbTable, DbTableAttributes,
};
use rust_extensions::date_time::DateTimeAsMicroseconds;
use tokio::sync::RwLock;

use super::{data_to_persist::PersistResult, DbTableSingleThreadedData};

pub struct DbTableWrapper {
    pub name: String,
    pub data: RwLock<DbTableSingleThreadedData>,
}

impl DbTableWrapper {
    pub fn new(db_table: DbTable) -> Arc<Self> {
        let result = Self {
            name: db_table.name.clone(),
            data: RwLock::new(DbTableSingleThreadedData::new(db_table)),
        };

        Arc::new(result)
    }

    pub fn restore_from_blob(db_table: DbTable) -> Arc<Self> {
        let result = Self {
            name: db_table.name.clone(),
            data: RwLock::new(DbTableSingleThreadedData::restore_from_blob(db_table)),
        };

        Arc::new(result)
    }

    pub async fn get_partition_snapshot(&self, partition_key: &str) -> Option<DbPartitionSnapshot> {
        let read_access = self.data.read().await;
        let db_partition = read_access.db_table.get_partition(partition_key)?;
        let result: DbPartitionSnapshot = db_partition.into();
        result.into()
    }

    pub async fn get_table_snapshot(&self) -> DbTableSnapshot {
        let read_access = self.data.read().await;
        read_access.db_table.get_table_snapshot()
    }

    pub async fn get_table_attributes(&self) -> DbTableAttributes {
        let read_access = self.data.read().await;
        read_access.db_table.attributes.clone()
    }

    pub async fn get_job_to_persist(
        &self,
        now: DateTimeAsMicroseconds,
        is_shutting_down: bool,
    ) -> Option<PersistResult> {
        let mut write_access = self.data.write().await;
        write_access
            .persist_markers
            .data_to_persist
            .get_what_to_persist(now, is_shutting_down)
    }

    pub async fn set_persisted(&self, duration: Duration) {
        let mut write_access = self.data.write().await;
        write_access.persist_markers.add_persist_duration(duration);
    }
}
