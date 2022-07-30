use crate::app::AppContext;
use crate::mynosqlserverpersistence_grpc::my_no_sql_server_persistnce_grpc_service_server::*;
use anyhow::*;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tonic::transport::Server;

#[derive(Clone)]
pub struct GrpcServer {
    pub app: Arc<AppContext>,
    pub timeout: Duration,
}

impl GrpcServer {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self {
            app,
            timeout: Duration::from_secs(5),
        }
    }
}

pub async fn start(app: Arc<AppContext>, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let service = GrpcServer::new(app);

    println!("Listening to {:?} as grpc endpoint", addr);
    Server::builder()
        .add_service(MyNoSqlServerPersistnceGrpcServiceServer::new(service))
        .serve(addr)
        .await
        .context("Server error")
}
