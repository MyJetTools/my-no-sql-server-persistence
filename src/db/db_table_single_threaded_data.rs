use my_no_sql_core::db::DbTable;

use crate::persist_operations::sync::PersistedTableData;

use super::PersistMarkers;

pub struct DbTableSingleThreadedData {
    pub db_table: DbTable,
    pub persist_markers: PersistMarkers,
    pub persisted_table_data: PersistedTableData,
}

impl DbTableSingleThreadedData {
    pub fn new(db_table_data: DbTable) -> Self {
        Self {
            db_table: db_table_data,
            persist_markers: PersistMarkers::new(),
            persisted_table_data: PersistedTableData::new(),
        }
    }

    pub fn restore_from_blob(db_table: DbTable) -> Self {
        let persisted_table_data = PersistedTableData::restore_from_blob(&db_table);
        Self {
            db_table,
            persist_markers: PersistMarkers::new(),
            persisted_table_data,
        }
    }
}
