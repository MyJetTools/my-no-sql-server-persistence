use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::db::DbTableWrapper;

pub async fn delete_partition(
    db_table_wrapper: &DbTableWrapper,
    partition_key: &str,
    persist_moment: DateTimeAsMicroseconds,
) {
    let mut write_access = db_table_wrapper.data.write().await;
    write_access.db_table.partitions.remove(partition_key);

    write_access
        .persist_markers
        .data_to_persist
        .mark_partition_to_persist(partition_key, persist_moment)
}
