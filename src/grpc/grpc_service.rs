use super::server::GrpcServer;

use crate::my_no_sql_server_persistence_grpc::my_no_sql_server_persistence_grpc_service_server::*;
use crate::my_no_sql_server_persistence_grpc::*;
use futures_core::Stream;
use rust_extensions::date_time::DateTimeAsMicroseconds;

use std::pin::Pin;

use tonic::Status;

use my_grpc_extensions::server::generate_server_stream;

#[tonic::async_trait]
impl MyNoSqlServerPersistenceGrpcService for GrpcServer {
    generate_server_stream!(stream_name:"GetTablesStream", item_name:"TableDescriptionGrpcModel");
    async fn get_tables(
        &self,
        _request: tonic::Request<()>,
    ) -> Result<tonic::Response<Self::GetTablesStream>, tonic::Status> {
        todo!("Implement");
    }

    async fn persist_events(
        &self,
        _request: tonic::Request<tonic::Streaming<PersistEvent>>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        todo!("Implement");
    }

    async fn ping(&self, _: tonic::Request<()>) -> Result<tonic::Response<()>, tonic::Status> {
        Ok(tonic::Response::new(()))
    }
}
