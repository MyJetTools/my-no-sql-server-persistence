use std::sync::Arc;

use my_no_sql_core::db::{DbPartition, DbRow};

use crate::{
    db::DbTableWrapper, grpc::grpc_persist_process::GrpcPersistProcess,
    mynosqlserverpersistence_grpc::*,
};

pub async fn init_partition(
    db_table_wrapper: Arc<DbTableWrapper>,
    persist_process: GrpcPersistProcess,
    entities: Vec<UpdateTableEntityGrpcModel>,
) {
    let mut write_access = db_table_wrapper.data.write().await;

    let (partition_key, db_partition) = {
        let partition_key = entities.first().unwrap().partition_key.as_str();
        write_access.db_table.partitions.remove(partition_key);
        write_access
            .db_table
            .partitions
            .insert(partition_key.to_string(), DbPartition::new());

        let db_partition = write_access
            .db_table
            .partitions
            .get_mut(partition_key)
            .unwrap();

        (partition_key.to_string(), db_partition)
    };

    for grpc_model in entities {
        let db_row: DbRow = grpc_model.into();
        db_partition.insert_row(Arc::new(db_row), None);
    }

    write_access
        .persist_markers
        .data_to_persist
        .mark_partition_to_persist(partition_key.as_str(), persist_process.persist_moment);
}
