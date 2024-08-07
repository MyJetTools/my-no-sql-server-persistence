use my_no_sql_sdk::core::db::DbTableAttributes;
use rust_extensions::sorted_vec::*;
use tokio::sync::Mutex;

use crate::{
    serializers::TableMetadataFileContract,
    sqlite::{DbRowDto, TableRepo},
};

use super::TableCache;

pub struct TableDataHolder {
    pub data: Mutex<TableCache>,
    pub repo: TableRepo,
    pub table_name: String,
}

impl TableDataHolder {
    pub async fn new(
        sqlite_table_path: String,
        table_name: String,
        meta_data: DbTableAttributes,
    ) -> Self {
        Self {
            table_name,
            data: Mutex::new(TableCache::new(meta_data)),
            repo: TableRepo::new(sqlite_table_path).await,
        }
    }

    pub async fn restore(sqlite_table_path: String, table_name: &str) -> Self {
        let repo = TableRepo::new(sqlite_table_path).await;

        let meta_data: DbTableAttributes = match repo.get_table_attribute().await {
            Some(value) => value.into(),
            None => DbTableAttributes::create_default(),
        };

        let records = repo.get_all().await;

        let mut table_data = TableCache::new(meta_data);

        for record in records {
            table_data.insert_or_update(&record.partition_key, &record.row_key, record.content);
        }

        Self {
            table_name: table_name.to_string(),
            data: Mutex::new(table_data),
            repo,
        }
    }

    pub async fn flush_data_to_cache(&self) {
        let cache_access = self.data.lock().await;

        self.repo.update_meta_data(&cache_access.meta_data).await;
        self.repo.clear_all_records().await;

        let mut to_insert = vec![];
        for db_partition in cache_access.partitions.iter() {
            for db_row in db_partition.rows.iter() {
                to_insert.push(DbRowDto {
                    partition_key: db_partition.partition_key.clone(),
                    row_key: db_row.row_key.clone(),
                    content: db_row.content.clone(),
                });

                if to_insert.len() > 5000 {
                    println!(
                        "Bulk insert {} records into table: {}",
                        to_insert.len(),
                        self.table_name
                    );
                    self.repo.bulk_insert_or_update(to_insert.as_slice()).await;
                    to_insert.clear();
                }
            }
        }

        if to_insert.len() > 0 {
            println!(
                "Bulk insert {} records into table: {}",
                to_insert.len(),
                self.table_name
            );
            self.repo.bulk_insert_or_update(to_insert.as_slice()).await;
        }
    }
}

impl EntityWithStrKey for TableDataHolder {
    fn get_key(&self) -> &str {
        &self.table_name
    }
}
