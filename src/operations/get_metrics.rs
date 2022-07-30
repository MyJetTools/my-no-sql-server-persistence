use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::db::DbTableWrapper;

pub struct DbTableMetrics {
    pub table_size: usize,
    pub partitions_amount: usize,
    pub records_amount: usize,
    pub last_update_time: DateTimeAsMicroseconds,
    pub last_persist_time: Option<DateTimeAsMicroseconds>,
    pub next_persist_time: Option<DateTimeAsMicroseconds>,
    pub persist_amount: usize,
    pub last_persist_duration: Vec<usize>,
}

pub async fn get_table_metrics(db_table_wrapper: &DbTableWrapper) -> DbTableMetrics {
    let table_read_access = db_table_wrapper.data.read().await;

    DbTableMetrics {
        table_size: table_read_access.db_table.get_table_size(),
        partitions_amount: table_read_access.db_table.get_partitions_amount(),
        records_amount: table_read_access.db_table.get_rows_amount(),
        last_update_time: table_read_access.db_table.get_last_update_time(),
        last_persist_time: table_read_access.persist_markers.last_persist_time,
        next_persist_time: table_read_access
            .persist_markers
            .data_to_persist
            .get_next_persist_time(),
        persist_amount: table_read_access
            .persist_markers
            .data_to_persist
            .get_persist_amount(),
        last_persist_duration: table_read_access.persist_markers.persist_duration.clone(),
    }
}
