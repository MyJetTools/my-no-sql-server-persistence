use std::sync::{atomic::AtomicUsize, Arc};

use rust_extensions::{date_time::DateTimeAsMicroseconds, AppStates, Logger};

use crate::{
    db::DbInstance, grpc::grpc_persist_process::GrpcPersistProcesses,
    persist_io::PersistIoOperations, persist_operations::data_initializer::load_tasks::InitState,
    settings_reader::SettingsModel,
};

use super::{
    logs::{Logs, SystemProcess},
    PrometheusMetrics,
};

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct AppContext {
    pub created: DateTimeAsMicroseconds,
    pub db: DbInstance,
    pub logs: Arc<Logs>,

    pub metrics: PrometheusMetrics,

    pub process_id: String,

    pub persist_io: PersistIoOperations,
    pub init_state: InitState,
    pub settings: Arc<SettingsModel>,
    pub states: Arc<AppStates>,

    pub grpc_persist_process: GrpcPersistProcesses,

    persist_amount: AtomicUsize,
}

impl AppContext {
    pub fn new(
        logs: Arc<Logs>,
        settings: Arc<SettingsModel>,
        persist_io: PersistIoOperations,
    ) -> Self {
        AppContext {
            created: DateTimeAsMicroseconds::now(),
            init_state: InitState::new(),
            db: DbInstance::new(),
            logs,
            metrics: PrometheusMetrics::new(),
            process_id: uuid::Uuid::new_v4().to_string(),
            states: Arc::new(AppStates::create_un_initialized()),

            persist_io,
            settings,
            persist_amount: AtomicUsize::new(0),
            grpc_persist_process: GrpcPersistProcesses::new(),
        }
    }

    pub fn update_persist_amount(&self, value: usize) {
        self.persist_amount
            .store(value, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn get_persist_amount(&self) -> usize {
        self.persist_amount
            .load(std::sync::atomic::Ordering::Relaxed)
    }
}

impl Logger for AppContext {
    fn write_info(&self, process_name: String, message: String, context: Option<String>) {
        self.logs
            .add_info(None, SystemProcess::System, process_name, message, context);
    }

    fn write_error(&self, process_name: String, message: String, context: Option<String>) {
        self.logs
            .add_fatal_error(None, SystemProcess::System, process_name, message, context);
    }

    fn write_warning(&self, process_name: String, message: String, ctx: Option<String>) {
        self.logs
            .add_error(None, SystemProcess::System, process_name, message, ctx);
    }

    fn write_fatal_error(&self, process_name: String, message: String, ctx: Option<String>) {
        self.logs
            .add_error(None, SystemProcess::System, process_name, message, ctx);
    }
}
