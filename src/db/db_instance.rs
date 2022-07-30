use tokio::sync::RwLock;

use std::{collections::HashMap, sync::Arc};

use super::DbTableWrapper;

pub struct DbInstance {
    pub tables: RwLock<HashMap<String, Arc<DbTableWrapper>>>,
}

impl DbInstance {
    pub fn new() -> DbInstance {
        DbInstance {
            tables: RwLock::new(HashMap::new()),
        }
    }

    pub async fn add(&self, db_table_wrapper: Arc<DbTableWrapper>) {
        let mut write_access = self.tables.write().await;
        write_access.insert(db_table_wrapper.name.to_string(), db_table_wrapper);
    }

    pub async fn get_table_names(&self) -> Vec<String> {
        let read_access = self.tables.read().await;

        return read_access
            .values()
            .into_iter()
            .map(|table| table.name.clone())
            .collect();
    }

    pub async fn get_tables(&self) -> Vec<Arc<DbTableWrapper>> {
        let read_access = self.tables.read().await;

        return read_access
            .values()
            .into_iter()
            .map(|table| table.clone())
            .collect();
    }

    pub async fn get_table(&self, table_name: &str) -> Option<Arc<DbTableWrapper>> {
        let read_access = self.tables.read().await;

        let result = read_access.get(table_name)?;
        return Some(result.clone());
    }
}
