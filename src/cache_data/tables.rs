use std::sync::Arc;

use my_no_sql_sdk::core::db::DbTableAttributes;
use rust_extensions::sorted_vec::*;
use tokio::sync::Mutex;

use crate::serializers::DbRowFromFile;

use super::TableDataHolder;

pub struct Tables {
    data: Mutex<SortedVecOfArcWithStrKey<TableDataHolder>>,
    sqlite_table_path: String,
}

impl Tables {
    pub fn new(sqlite_table_path: String) -> Self {
        Self {
            sqlite_table_path,
            data: Mutex::new(SortedVecOfArcWithStrKey::new()),
        }
    }

    pub async fn get_table(&self, table_name: &str) -> Option<Arc<TableDataHolder>> {
        let write_access = self.data.lock().await;
        write_access.get(table_name).cloned()
    }

    pub async fn get_or_create_table(&self, table_name: &str) -> Arc<TableDataHolder> {
        let mut write_access = self.data.lock().await;

        if let Some(result) = write_access.get(table_name) {
            return result.clone();
        }

        let table_data = TableDataHolder::new(
            compile_sql_table_file_name(self.sqlite_table_path.as_str(), table_name),
            table_name.to_string(),
            DbTableAttributes::create_default(),
        )
        .await;

        let table_data = Arc::new(table_data);
        write_access.insert_or_replace(table_data.clone());

        table_data
    }

    pub async fn get_tables(&self) -> Vec<Arc<TableDataHolder>> {
        let write_access = self.data.lock().await;
        write_access.to_vec_cloned()
    }

    pub async fn restore_table(&self, table_name: &str, meta_data: DbTableAttributes) {
        match self.data.lock().await.get_or_create(table_name) {
            rust_extensions::sorted_vec::GetOrCreateEntry::Get(table_data) => {
                let mut table_data = table_data.data.lock().await;
                table_data.meta_data = meta_data;
            }
            rust_extensions::sorted_vec::GetOrCreateEntry::Create(entry) => {
                let table_data = TableDataHolder::new(
                    compile_sql_table_file_name(self.sqlite_table_path.as_str(), table_name),
                    table_name.to_string(),
                    meta_data,
                )
                .await;

                let table_data = Arc::new(table_data);
                entry.insert(table_data.clone());
            }
        }
    }

    pub async fn restore_table_from_sqlite(&self, table_name: &str, file_path: String) {
        println!("Restoring table: {}", table_name);
        let table_holder = TableDataHolder::restore(file_path, table_name).await;

        let mut write_access = self.data.lock().await;

        write_access.insert_or_replace(table_holder.into());
    }

    pub async fn restore_records(&self, table_name: &str, db_rows: Vec<DbRowFromFile>) {
        match self.data.lock().await.get_or_create(table_name) {
            rust_extensions::sorted_vec::GetOrCreateEntry::Get(table_data) => {
                let mut table_data = table_data.data.lock().await;

                for db_row in db_rows {
                    table_data.insert_or_update(
                        &db_row.partition_key,
                        &db_row.row_key,
                        db_row.content,
                    );
                }
            }
            rust_extensions::sorted_vec::GetOrCreateEntry::Create(entry) => {
                let table_data = TableDataHolder::new(
                    compile_sql_table_file_name(self.sqlite_table_path.as_str(), table_name),
                    table_name.to_string(),
                    DbTableAttributes::create_default(),
                )
                .await;

                let table_data = Arc::new(table_data);
                entry.insert(table_data.clone());

                if db_rows.len() > 0 {
                    let mut table_data = table_data.data.lock().await;

                    for db_row in db_rows {
                        table_data.insert_or_update(
                            &db_row.partition_key,
                            &db_row.row_key,
                            db_row.content,
                        );
                    }
                }
            }
        }
    }
}

fn compile_sql_table_file_name(table_path: &str, table_name: &str) -> String {
    let mut result = table_path.to_string();
    if !result.ends_with(std::path::MAIN_SEPARATOR) {
        result.push(std::path::MAIN_SEPARATOR);
    }
    result.push_str(table_name);
    result.push_str(".sqlite");
    result
}
