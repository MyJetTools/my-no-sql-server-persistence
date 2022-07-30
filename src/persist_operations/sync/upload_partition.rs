use my_no_sql_core::db::db_snapshots::DbPartitionSnapshot;

use crate::{app::AppContext, db::DbTableWrapper, persist_io::TableFile};

pub async fn upload_partition(
    app: &AppContext,
    db_table_wrapper: &DbTableWrapper,
    partition_key: &str,
    snapshot: &DbPartitionSnapshot,
) {
    let content = snapshot.db_rows_snapshot.as_json_array();

    app.persist_io
        .save_table_file(
            db_table_wrapper.name.as_str(),
            &TableFile::DbPartition(partition_key.to_string()),
            content.build(),
        )
        .await;

    {
        let mut write_access = db_table_wrapper.data.write().await;
        write_access
            .persisted_table_data
            .update_table_partition_snapshot_id(partition_key, snapshot);
    }
}
