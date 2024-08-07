use std::time::Duration;

use my_no_sql_sdk::core::db::PartitionKeyParameter;
use tokio::sync::Mutex;

use crate::persist_io::{TableFile, TableListToPopulate};

pub struct FilesToDownloadListInner {
    tables: Vec<String>,
    everything_is_loaded: bool,
}

impl FilesToDownloadListInner {
    pub fn new() -> Self {
        Self {
            tables: Vec::new(),
            everything_is_loaded: false,
        }
    }
}

pub struct FilesToDownloadList {
    inner: Mutex<FilesToDownloadListInner>,
}

impl FilesToDownloadList {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(FilesToDownloadListInner::new()),
        }
    }

    pub async fn get_next_file_to_download(&self) -> Option<TableFile> {
        loop {
            {
                let mut inner = self.inner.lock().await;

                let amount = inner.tables.len();
                if amount > 0 {
                    let file_name = inner.tables.remove(0);
                    return Some(TableFile::from_file_name(file_name.as_str()).unwrap());
                }

                if inner.everything_is_loaded {
                    return None;
                }
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }
}

#[async_trait::async_trait]
impl TableListToPopulate for FilesToDownloadList {
    async fn add_files(&self, files: Vec<String>) {
        let mut inner = self.inner.lock().await;
        inner.tables.extend(files);
    }
    async fn set_files_list_is_loaded(&self) {
        let mut inner = self.inner.lock().await;
        inner.everything_is_loaded = true;
    }
}
