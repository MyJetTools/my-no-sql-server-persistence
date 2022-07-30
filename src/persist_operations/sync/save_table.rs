use std::collections::HashMap;

use my_no_sql_core::db::db_snapshots::DbTableSnapshot;
use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::{app::AppContext, db::DbTableWrapper};

use super::super::sync;

pub async fn save_table(app: &AppContext, db_table_wrapper: &DbTableWrapper) {
    let (table_snapshot, in_blob) = {
        let read_access = db_table_wrapper.data.read().await;

        let table_snapshot = read_access.db_table.get_table_snapshot();

        let blob_content_snapshot = read_access.persisted_table_data.get_snapshot();

        (table_snapshot, blob_content_snapshot)
    };

    match in_blob {
        Some(in_blob) => {
            sync_with_blob(app, db_table_wrapper, in_blob, &table_snapshot).await;
        }
        None => {
            init_new_table(app, db_table_wrapper, &table_snapshot).await;
        }
    }
}

async fn init_new_table(
    app: &AppContext,
    db_table_wrapper: &DbTableWrapper,
    snapshot: &DbTableSnapshot,
) {
    for (partition_key, db_partition_snapshot) in &snapshot.by_partition {
        sync::upload_partition(
            app,
            db_table_wrapper,
            partition_key.as_str(),
            db_partition_snapshot,
        )
        .await;
    }
}

async fn sync_with_blob(
    app: &AppContext,
    db_table_wrapper: &DbTableWrapper,
    mut in_blob: HashMap<String, DateTimeAsMicroseconds>,
    snapshot: &DbTableSnapshot,
) {
    for (partition_key, partition_snapshot) in &snapshot.by_partition {
        match in_blob.remove(partition_key) {
            Some(snapshot_in_blob) => {
                if partition_snapshot.has_to_persist(snapshot_in_blob) {
                    sync::upload_partition(
                        app,
                        db_table_wrapper,
                        partition_key,
                        partition_snapshot,
                    )
                    .await;
                }
            }
            None => {
                sync::upload_partition(app, db_table_wrapper, partition_key, partition_snapshot)
                    .await;
            }
        }
    }

    for (partition_key, _) in in_blob {
        sync::delete_partition(app, db_table_wrapper, partition_key.as_str()).await;
    }
}
