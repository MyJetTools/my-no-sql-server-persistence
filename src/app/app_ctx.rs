use std::sync::Arc;

use rust_extensions::{date_time::DateTimeAsMicroseconds, AppStates};
use tokio::sync::Mutex;

use crate::{
    cache_data::Tables,
    init_legacy::{InitFromArchiveState, InitState},
    persist_io::PersistIoOperations,
    settings_reader::SettingsModel,
};

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct AppContext {
    pub created: DateTimeAsMicroseconds,

    pub process_id: String,

    pub persist_io: Option<PersistIoOperations>,
    pub init_state: Mutex<InitState>,
    pub init_from_archive: Mutex<InitFromArchiveState>,
    pub settings: Arc<SettingsModel>,
    pub states: Arc<AppStates>,

    pub tables: Tables,
}

impl AppContext {
    pub fn new(settings: Arc<SettingsModel>) -> Self {
        let persist_io = settings.get_persist_io();
        AppContext {
            tables: Tables::new(settings.get_persistence_dest()),
            created: DateTimeAsMicroseconds::now(),
            init_state: Mutex::new(InitState::new()),

            process_id: uuid::Uuid::new_v4().to_string(),
            states: Arc::new(AppStates::create_un_initialized()),
            init_from_archive: Mutex::new(InitFromArchiveState::new()),

            persist_io,

            settings,
        }
    }
}
