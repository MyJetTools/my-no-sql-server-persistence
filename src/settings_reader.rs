use my_azure_storage_sdk::AzureStorageConnection;
use serde::{Deserialize, Serialize};
use std::{env, sync::Arc};
use tokio::{fs::File, io::AsyncReadExt};

use crate::persist_io::PersistIoOperations;

#[derive(Serialize, Deserialize, Debug)]
pub struct SettingsModel {
    #[serde(rename = "LegacyPersistenceDest")]
    pub legacy_persistence_dest: Option<String>,

    #[serde(rename = "LegacyZipArchive")]
    pub legacy_zip_archive: Option<String>,

    #[serde(rename = "PersistenceDest")]
    persistence_dest: String,

    #[serde(rename = "SkipBrokenPartitions")]
    pub skip_broken_partitions: bool,

    #[serde(rename = "InitThreadsAmount")]
    pub init_threads_amount: usize,
}

impl SettingsModel {
    pub fn get_persist_io(&self) -> Option<PersistIoOperations> {
        let conn_string =
            AzureStorageConnection::from_conn_string(self.legacy_persistence_dest.as_ref()?);
        Some(PersistIoOperations::new(Arc::new(conn_string)))
    }

    pub fn get_persistence_dest(&self) -> String {
        rust_extensions::file_utils::format_path(self.persistence_dest.as_str()).to_string()
    }
}

pub async fn read_settings() -> SettingsModel {
    let file_name = get_settings_filename();

    let mut file = File::open(file_name).await.unwrap();

    let mut file_content: Vec<u8> = vec![];
    file.read_to_end(&mut file_content).await.unwrap();

    let result: SettingsModel = serde_yaml::from_slice(file_content.as_slice()).unwrap();

    result
}

fn get_settings_filename() -> String {
    let path = env!("HOME");

    let file_name = ".mynosqlserver-persistence";

    if path.ends_with(std::path::MAIN_SEPARATOR) {
        return format!("{}{}", path, file_name);
    }

    return format!("{}{}{}", path, std::path::MAIN_SEPARATOR, file_name);
}
