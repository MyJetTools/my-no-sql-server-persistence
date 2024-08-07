use std::sync::Arc;

use my_no_sql_sdk::core::db::DbTableAttributes;

use crate::{app::AppContext, my_no_sql_server_persistence_grpc::*, persist_queue::PersistEvent};

pub async fn handle_persist_events(app: &Arc<AppContext>, events: Vec<PersistGrpcEvent>) {
    for mut event in events {
        if event.clear_table {}

        if let Some(table_attributes) = event.update_table_attributes.take() {
            handle_update_table_metadata(app, event.table_name.as_str(), table_attributes.into())
                .await;
        }

        if let Some(delete_row) = event.delete_row.take() {
            handle_delete_row(app, event.table_name.as_str(), delete_row).await;
        }

        if let Some(insert_or_update) = event.insert_or_update_row.take() {
            handle_insert_or_update(app, event.table_name.as_str(), insert_or_update).await;
        }

        if let Some(delete_partition) = event.delete_partition.take() {
            handle_delete_partition(
                app,
                event.table_name.as_str(),
                delete_partition.partition_key,
            )
            .await;
        }
    }
}

async fn handle_insert_or_update(
    app: &AppContext,
    table_name: &str,
    insert_or_update: InsertOrUpdateRowGrpcModel,
) {
    let table = app.tables.get_or_create_table(table_name).await;

    let mut write_access = table.data.lock().await;

    write_access.insert_or_update(
        &insert_or_update.partition_key,
        insert_or_update.row_key.as_str(),
        insert_or_update.data,
    );

    write_access
        .persist_queue
        .enqueue(PersistEvent::PersistRow {
            partition_key: insert_or_update.partition_key,
            row_key: insert_or_update.row_key,
        });
}

async fn handle_update_table_metadata(app: &AppContext, table_name: &str, attr: DbTableAttributes) {
    let table = app.tables.get_or_create_table(table_name).await;

    let mut write_access = table.data.lock().await;
    write_access.meta_data = attr;
    write_access
        .persist_queue
        .enqueue(PersistEvent::PersistTableMetadata);
}

async fn handle_delete_partition(app: &Arc<AppContext>, table_name: &str, partition_key: String) {
    let table = app.tables.get_table(table_name).await;

    if table.is_none() {
        return;
    }

    let table = table.unwrap();

    let mut write_access = table.data.lock().await;

    write_access.partitions.remove(partition_key.as_str());

    write_access
        .persist_queue
        .enqueue(PersistEvent::PersistPartition {
            partition_key: partition_key.clone(),
        });
}

async fn handle_delete_row(
    app: &Arc<AppContext>,
    table_name: &str,
    delete_row: DeleteRowGrpcModel,
) {
    let table = app.tables.get_table(table_name).await;

    if table.is_none() {
        return;
    }

    let table = table.unwrap();

    let mut write_access = table.data.lock().await;

    let rows_inside_partition = match write_access
        .partitions
        .get_mut(delete_row.partition_key.as_str())
    {
        Some(partition) => {
            partition.rows.remove(delete_row.row_key.as_str());
            partition.rows.len()
        }
        None => 0,
    };

    if rows_inside_partition == 0 {
        write_access
            .partitions
            .remove(delete_row.partition_key.as_str());

        write_access
            .persist_queue
            .enqueue(PersistEvent::PersistPartition {
                partition_key: delete_row.partition_key,
            })
    } else {
        write_access
            .persist_queue
            .enqueue(PersistEvent::PersistRow {
                partition_key: delete_row.partition_key,
                row_key: delete_row.row_key,
            })
    }
}
