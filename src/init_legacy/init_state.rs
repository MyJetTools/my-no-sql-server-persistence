use std::{collections::BTreeMap, sync::Arc};

use super::FilesToDownloadList;

pub struct InitState {
    pub tables_to_load: Vec<String>,
    pub tables_loading: BTreeMap<String, Arc<FilesToDownloadList>>,
    pub tables_loaded: Vec<String>,
}

impl InitState {
    pub fn new() -> Self {
        Self {
            tables_to_load: Vec::new(),
            tables_loading: BTreeMap::new(),
            tables_loaded: Vec::new(),
        }
    }
}
