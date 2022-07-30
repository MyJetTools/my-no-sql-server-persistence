use crate::{app::AppContext, db::DbTableWrapper};

use super::{super::sync, BlobPartitionUpdateTimeResult};

pub async fn save_partition(
    app: &AppContext,
    db_table_wrapper: &DbTableWrapper,
    partition_key: &str,
) {
    let get_blob_content_cache = {
        let read_access = db_table_wrapper.data.read().await;
        read_access.persisted_table_data.get(partition_key)
    };

    let partition_snapshot = db_table_wrapper.get_partition_snapshot(partition_key).await;

    match get_blob_content_cache {
        BlobPartitionUpdateTimeResult::Ok(blob_date_time) => {
            if partition_snapshot.is_none() {
                sync::delete_partition(app, db_table_wrapper, partition_key).await;
                return;
            }

            let partition_snapshot = partition_snapshot.unwrap();

            if partition_snapshot.last_write_moment.unix_microseconds
                > blob_date_time.unix_microseconds
            {
                sync::upload_partition(app, db_table_wrapper, partition_key, &partition_snapshot)
                    .await;
            }
        }
        BlobPartitionUpdateTimeResult::TableNotFound => {
            if let Some(snapshot) = partition_snapshot {
                sync::create_table(app, db_table_wrapper).await;
                sync::upload_partition(app, db_table_wrapper, partition_key, &snapshot).await;
            }
        }
        BlobPartitionUpdateTimeResult::PartitionNoFound => {
            if let Some(snapshot) = partition_snapshot {
                sync::upload_partition(app, db_table_wrapper, partition_key, &snapshot).await;
            }
        }
    }
}
