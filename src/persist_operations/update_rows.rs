use std::sync::Arc;

use my_no_sql_core::db::{DbPartition, DbRow};

use crate::{
    db::DbTableWrapper, grpc::grpc_persist_process::GrpcPersistProcess,
    mynosqlserverpersistence_grpc::*,
};

pub async fn update_rows(
    db_table_wrapper: Arc<DbTableWrapper>,
    persist_process: GrpcPersistProcess,
    entities: Vec<UpdateTableEntityGrpcModel>,
) {
    let mut write_access = db_table_wrapper.data.write().await;

    for grpc_model in entities {
        let db_row: DbRow = grpc_model.into();

        if !write_access
            .db_table
            .partitions
            .contains_key(db_row.partition_key.as_str())
        {
            write_access
                .db_table
                .partitions
                .insert(db_row.partition_key.to_string(), DbPartition::new());
        }

        let db_partition = write_access
            .db_table
            .partitions
            .get_mut(db_row.partition_key.as_str())
            .unwrap();

        let db_row = Arc::new(db_row);

        db_partition.insert_row(db_row.clone(), None);

        write_access
            .persist_markers
            .data_to_persist
            .mark_partition_to_persist(&db_row.partition_key, persist_process.persist_moment);
    }
}
