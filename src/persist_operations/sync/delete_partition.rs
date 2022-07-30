use crate::{app::AppContext, db::DbTableWrapper, persist_io::TableFile};

use super::BlobPartitionUpdateTimeResult;

pub async fn delete_partition(
    app: &AppContext,
    db_table_wrapper: &DbTableWrapper,
    partition_key: &str,
) {
    let partition_in_blob = {
        let read_access = db_table_wrapper.data.read().await;
        read_access.persisted_table_data.get(partition_key)
    };

    match partition_in_blob {
        BlobPartitionUpdateTimeResult::Ok(_) => {
            app.persist_io
                .delete_table_file(
                    db_table_wrapper.name.as_str(),
                    &TableFile::DbPartition(partition_key.to_string()),
                )
                .await;

            {
                let mut write_access = db_table_wrapper.data.write().await;
                write_access
                    .persisted_table_data
                    .delete_table_partition(partition_key);
            }
        }
        BlobPartitionUpdateTimeResult::TableNotFound => {}
        BlobPartitionUpdateTimeResult::PartitionNoFound => {}
    }
}
