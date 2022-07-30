use rust_extensions::lazy::LazyVec;

use crate::{
    db::DbTableWrapper, grpc::grpc_persist_process::GrpcPersistProcess,
    mynosqlserverpersistence_grpc::*,
};

pub async fn delete_rows(
    db_table_wrapper: &DbTableWrapper,
    persist_process: GrpcPersistProcess,
    rows_to_delete: &[DeleteEntityGrpcModel],
) {
    let mut write_access = db_table_wrapper.data.write().await;

    let mut empty_partitions = LazyVec::new();

    for row_to_delete in rows_to_delete {
        let db_partition = write_access
            .db_table
            .partitions
            .get_mut(row_to_delete.partition_key.as_str());

        if db_partition.is_none() {
            continue;
        }

        let partition_is_empty = {
            let db_partition = db_partition.unwrap();
            db_partition.remove_row(row_to_delete.row_key.as_str(), None);
            db_partition.is_empty()
        };

        write_access
            .persist_markers
            .data_to_persist
            .mark_partition_to_persist(
                row_to_delete.partition_key.as_str(),
                persist_process.persist_moment,
            );

        if partition_is_empty {
            empty_partitions.add(row_to_delete.partition_key.to_string());
        }
    }

    if let Some(partitions_to_persist) = empty_partitions.get_result() {
        for partition_key in partitions_to_persist {
            write_access
                .db_table
                .partitions
                .remove(partition_key.as_str());
        }
    }
}
