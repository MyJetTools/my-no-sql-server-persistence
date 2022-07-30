use super::server::GrpcServer;

use crate::mynosqlserverpersistence_grpc::my_no_sql_server_persistnce_grpc_service_server::*;
use crate::mynosqlserverpersistence_grpc::*;
use futures_core::Stream;
use rust_extensions::date_time::DateTimeAsMicroseconds;

use std::pin::Pin;

use tonic::Status;

#[tonic::async_trait]
impl MyNoSqlServerPersistnceGrpcService for GrpcServer {
    type GetTablesStream = Pin<
        Box<dyn Stream<Item = Result<TableDescriptionGrpcModel, Status>> + Send + Sync + 'static>,
    >;

    type GetTableStream =
        Pin<Box<dyn Stream<Item = Result<TableEntityGrpcModel, Status>> + Send + Sync + 'static>>;

    async fn get_tables(
        &self,
        _request: tonic::Request<()>,
    ) -> Result<tonic::Response<Self::GetTablesStream>, tonic::Status> {
        let db_tables = self.app.db.get_tables().await;

        let (tx, rx) = tokio::sync::mpsc::channel(4);

        tokio::spawn(async move {
            for db_table in db_tables {
                let attrs = db_table.get_table_attributes().await;
                let grpc_contract = TableDescriptionGrpcModel {
                    table_name: db_table.name.clone(),
                    persist: attrs.persist,
                    max_partitions_amount: match attrs.max_partitions_amount {
                        Some(amount) => amount as i32,
                        None => 0,
                    },
                };
                tx.send(Ok(grpc_contract)).await.unwrap();
            }
        });

        Ok(tonic::Response::new(Box::pin(
            tokio_stream::wrappers::ReceiverStream::new(rx),
        )))
    }

    async fn get_table(
        &self,
        request: tonic::Request<GetTableGrpcRequest>,
    ) -> Result<tonic::Response<Self::GetTableStream>, tonic::Status> {
        let request = request.into_inner();

        let db_table_wrapper =
            super::grpc_helpers::get_table(self.app.as_ref(), request.table_name.as_str()).await;

        let (tx, rx) = tokio::sync::mpsc::channel(4);

        tokio::spawn(async move {
            let db_table_snapshot = db_table_wrapper.get_table_snapshot().await;

            for (partition_key, partition) in db_table_snapshot.by_partition {
                for db_row in partition.db_rows_snapshot.db_rows {
                    let grpc_contract = TableEntityGrpcModel {
                        partition_key: partition_key.clone(),
                        row_key: db_row.row_key.clone(),
                        expires: match db_row.expires {
                            None => 0,
                            Some(expires) => expires.unix_microseconds,
                        },
                        timespan: db_row.time_stamp.clone(),
                        data: db_row.data.clone(),
                    };
                    tx.send(Ok(grpc_contract)).await.unwrap();
                }
            }
        });

        Ok(tonic::Response::new(Box::pin(
            tokio_stream::wrappers::ReceiverStream::new(rx),
        )))
    }

    async fn clean_table(
        &self,
        request: tonic::Request<CleanTableGrpcRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let request = request.into_inner();
        let db_table_wrapper =
            super::grpc_helpers::get_table(self.app.as_ref(), request.table_name.as_str()).await;

        let mut write_access = db_table_wrapper.data.write().await;

        write_access.db_table.clean_table();
        let persist_moment = DateTimeAsMicroseconds::new(request.persist_moment);
        write_access
            .persist_markers
            .data_to_persist
            .mark_table_to_persist(persist_moment);

        Ok(tonic::Response::new(()))
    }

    async fn start_persist_process(
        &self,
        request: tonic::Request<StartPersistProcessGrpcRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let request = request.into_inner();

        self.app
            .grpc_persist_process
            .add(
                request.process_id,
                request.table_name,
                DateTimeAsMicroseconds::new(request.persist_moment),
            )
            .await;

        Ok(tonic::Response::new(()))
    }

    async fn init_table(
        &self,
        request: tonic::Request<tonic::Streaming<UpdateTableEntityGrpcModel>>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let mut request_stream = request.into_inner();

        let entities: Vec<UpdateTableEntityGrpcModel> =
            super::read_with_timeout::read_from_stream_to_vec(&mut request_stream, self.timeout)
                .await;

        if entities.len() == 0 {
            println!("Somehow we got 0 entities to init_table");
        }

        let persist_process = {
            let first = entities.get(0).unwrap();
            self.app.grpc_persist_process.get(first.process_id).await
        };

        let db_table_wrapper =
            super::grpc_helpers::get_table(&self.app, persist_process.table_name.as_str()).await;

        crate::persist_operations::init_table(db_table_wrapper, persist_process, entities).await;

        Ok(tonic::Response::new(()))
    }

    async fn delete_partition(
        &self,
        request: tonic::Request<DeletePartitionGrpcRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let request = request.into_inner();

        let db_table_wrapper =
            super::grpc_helpers::get_table(&self.app, request.table_name.as_str()).await;

        crate::persist_operations::delete_partition(
            &db_table_wrapper,
            request.partition_key.as_str(),
            DateTimeAsMicroseconds::new(request.persist_moment),
        )
        .await;

        Ok(tonic::Response::new(()))
    }

    async fn init_partition(
        &self,
        request: tonic::Request<tonic::Streaming<UpdateTableEntityGrpcModel>>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let mut request_stream = request.into_inner();

        let entities: Vec<UpdateTableEntityGrpcModel> =
            super::read_with_timeout::read_from_stream_to_vec(&mut request_stream, self.timeout)
                .await;

        if entities.len() == 0 {
            println!("Somehow we got 0 entities to init_partition");
        }

        let persist_process = {
            let first = entities.get(0).unwrap();
            self.app.grpc_persist_process.get(first.process_id).await
        };

        let db_table_wrapper =
            super::grpc_helpers::get_table(&self.app, persist_process.table_name.as_str()).await;

        crate::persist_operations::init_partition(db_table_wrapper, persist_process, entities)
            .await;

        Ok(tonic::Response::new(()))
    }

    async fn replace_rows(
        &self,
        request: tonic::Request<tonic::Streaming<UpdateTableEntityGrpcModel>>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let mut request_stream = request.into_inner();

        let entities: Vec<UpdateTableEntityGrpcModel> =
            super::read_with_timeout::read_from_stream_to_vec(&mut request_stream, self.timeout)
                .await;

        if entities.len() == 0 {
            println!("Somehow we got 0 entities to replace_rows");
        }

        let persist_process = {
            let first = entities.get(0).unwrap();
            self.app.grpc_persist_process.get(first.process_id).await
        };

        let db_table_wrapper =
            super::grpc_helpers::get_table(&self.app, persist_process.table_name.as_str()).await;

        crate::persist_operations::update_rows(db_table_wrapper, persist_process, entities).await;

        Ok(tonic::Response::new(()))
    }

    async fn delete_rows(
        &self,
        request: tonic::Request<tonic::Streaming<DeleteEntityGrpcModel>>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let mut request_stream = request.into_inner();

        let delete_entities: Vec<DeleteEntityGrpcModel> =
            super::read_with_timeout::read_from_stream_to_vec(&mut request_stream, self.timeout)
                .await;

        if delete_entities.len() == 0 {
            println!("Somehow we got 0 entities to init_partition");
        }

        let persist_process = {
            let first = delete_entities.get(0).unwrap();
            self.app.grpc_persist_process.get(first.process_id).await
        };

        let db_table_wrapper =
            super::grpc_helpers::get_table(&self.app, persist_process.table_name.as_str()).await;

        crate::persist_operations::delete_rows(
            &db_table_wrapper,
            persist_process,
            delete_entities.as_slice(),
        )
        .await;

        Ok(tonic::Response::new(()))
    }

    async fn persist_table_attrs(
        &self,
        request: tonic::Request<PersistTableAttrsRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let request = request.into_inner();

        let db_table_wrapper =
            super::grpc_helpers::get_table(&self.app, request.table_name.as_str()).await;

        let max_partitions_amount = if request.max_partitions_amount == 0 {
            None
        } else {
            Some(request.max_partitions_amount as usize)
        };

        crate::persist_operations::update_table_attrs(
            &db_table_wrapper,
            request.persist,
            DateTimeAsMicroseconds::new(request.created),
            max_partitions_amount,
        )
        .await;

        Ok(tonic::Response::new(()))
    }
}
