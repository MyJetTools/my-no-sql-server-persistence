use app::{logs::Logs, AppContext};
use background::{
    metrics_updater::MetricsUpdater, persist::PersistTimer, presist_process_gc::PersistProcessGc,
};

use rust_extensions::MyTimer;
use std::{sync::Arc, time::Duration};

mod app;
mod grpc;

mod db;

mod http;

mod background;

mod operations;
mod persist_io;
mod persist_operations;
mod settings_reader;

pub mod mynosqlserverpersistence_grpc {
    tonic::include_proto!("mynosqlserverpersistence");
}

#[tokio::main]
async fn main() {
    let settings = settings_reader::read_settings().await;

    let settings = Arc::new(settings);

    let logs = Arc::new(Logs::new());

    let persist_io = settings.get_persist_io(logs.clone());

    let app = AppContext::new(logs.clone(), settings, persist_io);

    let app = Arc::new(app);

    tokio::spawn(crate::persist_operations::data_initializer::load_tables(
        app.clone(),
    ));

    let mut timer_1s = MyTimer::new(Duration::from_secs(1));
    timer_1s.register_timer("MetricsUpdated", Arc::new(MetricsUpdater::new(app.clone())));
    timer_1s.start(app.states.clone(), app.clone());

    let mut persist_timer = MyTimer::new(Duration::from_secs(1));
    persist_timer.register_timer("Persist", Arc::new(PersistTimer::new(app.clone())));
    persist_timer.start(app.states.clone(), app.clone());

    let mut processes_gc_timer = MyTimer::new(Duration::from_secs(30));

    processes_gc_timer.register_timer(
        "ProcessesGcTimer",
        Arc::new(PersistProcessGc::new(app.clone())),
    );

    processes_gc_timer.start(app.states.clone(), app.clone());

    crate::http::start_up::setup_server(&app);

    tokio::task::spawn(crate::grpc::server::start(app.clone(), 5124));

    signal_hook::flag::register(
        signal_hook::consts::SIGTERM,
        app.states.shutting_down.clone(),
    )
    .unwrap();

    shut_down_task(app).await;
}

async fn shut_down_task(app: Arc<AppContext>) {
    let duration = Duration::from_secs(1);

    while !app.states.is_shutting_down() {
        tokio::time::sleep(duration).await;
    }

    println!("Shut down detected. Waiting for 1 second to deliver all messages");
    tokio::time::sleep(duration).await;

    crate::operations::shutdown::execute(app.as_ref()).await;
}
