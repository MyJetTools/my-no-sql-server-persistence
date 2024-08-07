use super::server::GrpcServer;

use crate::my_no_sql_server_persistence_grpc::my_no_sql_server_persistence_grpc_service_server::*;
use crate::my_no_sql_server_persistence_grpc::*;
use futures_core::Stream;
use quick_xml::events;
use rust_extensions::date_time::DateTimeAsMicroseconds;

use std::pin::Pin;
use std::time::Duration;

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
        request: tonic::Request<tonic::Streaming<PersistGrpcEvent>>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let stream = request.into_inner();

        let events = my_grpc_extensions::read_grpc_stream::as_vec(stream, Duration::from_secs(10))
            .await
            .unwrap();

        if let Some(events) = events {
            crate::flows::handle_persist_events(&self.app, events).await;
        }

        Ok(tonic::Response::new(()))
    }

    async fn ping(&self, _: tonic::Request<()>) -> Result<tonic::Response<()>, tonic::Status> {
        Ok(tonic::Response::new(()))
    }
}
