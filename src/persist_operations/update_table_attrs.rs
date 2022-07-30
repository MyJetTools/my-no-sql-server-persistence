use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::db::DbTableWrapper;

pub async fn update_table_attrs(
    db_table_wrapper: &DbTableWrapper,
    persist: bool,
    created: DateTimeAsMicroseconds,
    max_partitions_amount: Option<usize>,
) {
    let mut write_access = db_table_wrapper.data.write().await;

    write_access.db_table.attributes.persist = persist;
    write_access.db_table.attributes.created = created;
    write_access.db_table.attributes.max_partitions_amount = max_partitions_amount;

    write_access
        .persist_markers
        .data_to_persist
        .mark_persist_attrs();
}
